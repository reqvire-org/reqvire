use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::{header, Method, Response, StatusCode, Uri};
use axum::routing::any;
use axum::Router;
use percent_encoding::percent_decode_str;
use reqvire::error::ReqvireError;
use reqvire::explorer_runtime::{embedded_asset, index_html, ExplorerRuntimeAssets};

#[derive(Clone)]
pub(crate) struct ServeState {
    project_store_js: Arc<String>,
    ontologies_ttl: Arc<String>,
}

/// Starts an HTTP server for the embedded Explorer SPA and generated runtime data.
pub async fn serve_explorer(
    assets: ExplorerRuntimeAssets,
    host: &str,
    port: u16,
) -> Result<(), ReqvireError> {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| ReqvireError::ProcessError(format!("Failed to start server: {}", e)))?;

    let state = ServeState {
        project_store_js: Arc::new(assets.project_store_js),
        ontologies_ttl: Arc::new(assets.ontologies_ttl),
    };
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

    let request_path = match resolve_request_path(uri.path()) {
        Ok(path) => path,
        Err(status) => return response_with_status(status),
    };

    if request_path == "assets/project-store.js" {
        return bytes_response(
            method,
            "application/javascript",
            state.project_store_js.as_bytes().to_vec(),
        );
    }

    if request_path == "ontologies.ttl" {
        return bytes_response(
            method,
            "text/turtle; charset=utf-8",
            state.ontologies_ttl.as_bytes().to_vec(),
        );
    }

    if let Some(content) = embedded_asset(&request_path) {
        return static_response(method, content_type_for_path(&request_path), content);
    }

    if let Some(response) = workspace_file_response(method.clone(), &request_path) {
        return response;
    }

    if request_path.starts_with("assets/") {
        return response_with_status(StatusCode::NOT_FOUND);
    }

    static_response(method, "text/html; charset=utf-8", index_html())
}

fn static_response(
    method: Method,
    content_type: &'static str,
    content: &'static [u8],
) -> Response<Body> {
    if method == Method::HEAD {
        return empty_response(content_type);
    }
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(content))
        .unwrap_or_else(|_| response_with_status(StatusCode::INTERNAL_SERVER_ERROR))
}

fn bytes_response(method: Method, content_type: &'static str, content: Vec<u8>) -> Response<Body> {
    if method == Method::HEAD {
        return empty_response(content_type);
    }
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::from(content))
        .unwrap_or_else(|_| response_with_status(StatusCode::INTERNAL_SERVER_ERROR))
}

fn empty_response(content_type: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .body(Body::empty())
        .unwrap_or_else(|_| response_with_status(StatusCode::INTERNAL_SERVER_ERROR))
}

fn workspace_file_response(method: Method, request_path: &str) -> Option<Response<Body>> {
    let path = Path::new(request_path);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        || !is_workspace_asset_path(path)
    {
        return None;
    }

    let absolute_path = PathBuf::from(".").join(path);
    if !absolute_path.is_file() {
        return None;
    }

    match std::fs::read(&absolute_path) {
        Ok(content) => Some(bytes_response(
            method,
            content_type_for_path(request_path),
            content,
        )),
        Err(_) => Some(response_with_status(StatusCode::NOT_FOUND)),
    }
}

fn is_workspace_asset_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_ascii_lowercase()),
        Some(ext)
            if matches!(
                ext.as_str(),
                "png"
                    | "jpg"
                    | "jpeg"
                    | "gif"
                    | "webp"
                    | "svg"
                    | "pdf"
                    | "txt"
                    | "csv"
                    | "json"
                    | "jsonld"
                    | "ttl"
                    | "turtle"
            )
    )
}

fn resolve_request_path(raw_request_path: &str) -> Result<String, StatusCode> {
    let decoded = percent_decode_str(raw_request_path)
        .decode_utf8()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut parts = Vec::new();
    for segment in decoded.trim_start_matches('/').split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." || segment.contains('\\') || segment.contains('\0') {
            return Err(StatusCode::NOT_FOUND);
        }
        parts.push(segment);
    }

    if parts.is_empty() {
        Ok("index.html".to_string())
    } else {
        Ok(parts.join("/"))
    }
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

fn content_type_for_path(path: &str) -> &'static str {
    match Path::new(path).extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain; charset=utf-8",
        Some("csv") => "text/csv; charset=utf-8",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("ttl") | Some("turtle") => "text/turtle; charset=utf-8",
        Some("jsonld") => "application/ld+json",
        _ => "text/plain",
    }
}
