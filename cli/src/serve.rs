use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::{header, Method, Response, StatusCode, Uri};
use axum::routing::any;
use axum::Router;
use percent_encoding::percent_decode_str;
use reqvire::error::ReqvireError;
use tokio::sync::Mutex;

#[derive(Clone)]
pub(crate) struct ServeState {
    export_root: Arc<PathBuf>,
    write_lock: Arc<Mutex<()>>,
}

impl ServeState {
    fn new(export_root: &Path) -> Self {
        Self {
            export_root: Arc::new(export_root.to_path_buf()),
            write_lock: Arc::new(Mutex::new(())),
        }
    }

    async fn run_mutation<F, T>(&self, mutation: F) -> Result<T, ReqvireError>
    where
        F: FnOnce() -> Result<T, ReqvireError> + Send + 'static,
        T: Send + 'static,
    {
        let _guard = self.write_lock.lock().await;
        tokio::task::spawn_blocking(mutation)
            .await
            .map_err(|e| ReqvireError::ProcessError(format!("Mutation task failed: {}", e)))?
    }
}

#[allow(dead_code)]
pub(crate) async fn run_serialized_mutation<F, T>(
    state: &ServeState,
    mutation: F,
) -> Result<T, ReqvireError>
where
    F: FnOnce() -> Result<T, ReqvireError> + Send + 'static,
    T: Send + 'static,
{
    state.run_mutation(mutation).await
}

/// Starts an HTTP server serving static files from the given directory
pub async fn serve_directory(directory: &Path, host: &str, port: u16) -> Result<(), ReqvireError> {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| ReqvireError::ProcessError(format!("Failed to start server: {}", e)))?;

    let state = ServeState::new(directory);
    let app = Router::new()
        .route("/", any(serve_static))
        .route("/{*path}", any(serve_static))
        .with_state(state);

    let url = format!("http://{}:{}", host, port);
    println!(
        "🌐 Server running at: \x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
        url, url
    );
    println!();
    println!("📖 Instructions:");
    println!("  • Open the link above in your browser");
    println!("  • Press Ctrl-C to stop server");
    println!();

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .map_err(|e| ReqvireError::ProcessError(format!("Server error: {}", e)))?;

    Ok(())
}

async fn serve_static(State(state): State<ServeState>, method: Method, uri: Uri) -> Response<Body> {
    if method != Method::GET && method != Method::HEAD {
        return response_with_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    let file_path = match resolve_secure_path(&state.export_root, uri.path()) {
        Ok(path) => path,
        Err(status) => return response_with_status(status),
    };

    let content_type = content_type_for_path(&file_path);

    if method == Method::HEAD {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::empty())
            .unwrap_or_else(|_| response_with_status(StatusCode::INTERNAL_SERVER_ERROR));
    }

    let content = match tokio::fs::read(&file_path).await {
        Ok(bytes) => bytes,
        Err(_) => return response_with_status(StatusCode::NOT_FOUND),
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(content))
        .unwrap_or_else(|_| response_with_status(StatusCode::INTERNAL_SERVER_ERROR))
}

fn resolve_secure_path(root: &Path, raw_request_path: &str) -> Result<PathBuf, StatusCode> {
    let root_canonical =
        std::fs::canonicalize(root).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let decoded = percent_decode_str(raw_request_path)
        .decode_utf8()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut relative = PathBuf::new();
    for segment in decoded.trim_start_matches('/').split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." || segment.contains('\\') || segment.contains('\0') {
            return Err(StatusCode::NOT_FOUND);
        }
        relative.push(segment);
    }

    if relative.as_os_str().is_empty() {
        relative.push("index.html");
    }

    let mut candidate = root.join(relative);
    if candidate.is_dir() {
        candidate = candidate.join("index.html");
    }

    let canonical_candidate =
        std::fs::canonicalize(&candidate).map_err(|_| StatusCode::NOT_FOUND)?;
    if !canonical_candidate.starts_with(&root_canonical) || !canonical_candidate.is_file() {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(canonical_candidate)
}

fn response_with_status(status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::from(match status {
            StatusCode::NOT_FOUND => "404 Not Found",
            StatusCode::METHOD_NOT_ALLOWED => "405 Method Not Allowed",
            _ => "Internal Server Error",
        }))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn content_type_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("ttl") | Some("turtle") => "text/turtle; charset=utf-8",
        Some("jsonld") => "application/ld+json",
        _ => "text/plain",
    }
}
