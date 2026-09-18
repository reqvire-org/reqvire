use crate::mcp;
use globset::GlobSet;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, Method, Response, StatusCode, Uri};
use axum::routing::{any, get};
use axum::Router;
use percent_encoding::percent_decode_str;
use reqvire::error::ReqvireError;
use reqvire::explorer_runtime::{
    build_runtime_assets, embedded_asset, index_html, is_workspace_asset_path,
    ExplorerRuntimeAssets,
};
use reqvire::{model_cache, ModelBuildOptions};
use serde::Deserialize;

const REFRESH_CHECK_INTERVAL: Duration = Duration::from_secs(1);

struct RuntimeSnapshot {
    assets: ExplorerRuntimeAssets,
    revision: String,
    fingerprint: Option<model_cache::ModelFingerprint>,
    last_checked: Option<Instant>,
    refresh_error: Option<String>,
}

fn runtime_revision(assets: &ExplorerRuntimeAssets) -> String {
    reqvire::utils::hash_content(&format!(
        "{}\0{}",
        assets.project_store_json, assets.ontologies_ttl
    ))
}

#[derive(Clone)]
pub(crate) struct ServeState {
    excluded_filename_patterns: Arc<GlobSet>,
    runtime_assets: Arc<Mutex<RuntimeSnapshot>>,
    refresh_lock: Arc<Mutex<()>>,
    write_lock: Arc<Mutex<()>>,
}

/// Starts an HTTP server for the embedded Explorer SPA and generated runtime data.
pub async fn serve_explorer(
    assets: ExplorerRuntimeAssets,
    host: &str,
    port: u16,
    enable_mcp: bool,
    mcp_enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
) -> Result<(), ReqvireError> {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| ReqvireError::ProcessError(format!("Failed to start server: {}", e)))?;

    let state = ServeState {
        excluded_filename_patterns: Arc::new(excluded_filename_patterns.clone()),
        runtime_assets: Arc::new(Mutex::new(RuntimeSnapshot {
            revision: runtime_revision(&assets),
            assets,
            fingerprint: None,
            last_checked: None,
            refresh_error: None,
        })),
        refresh_lock: Arc::new(Mutex::new(())),
        write_lock: Arc::new(Mutex::new(())),
    };
    let mut app = Router::new()
        .route("/api/project-store", get(serve_live_store))
        .route("/", any(serve_static))
        .fallback(serve_static);

    if enable_mcp {
        let refresh_state = state.clone();
        let post_write_hook: mcp::PostWriteHook = Arc::new(move || {
            let refresh_state = refresh_state.clone();
            Box::pin(async move { refresh_runtime_assets(&refresh_state, true).await })
                as Pin<Box<dyn std::future::Future<Output = Result<(), ReqvireError>> + Send>>
        });
        app = mcp::mount_service_with_post_write_hook(
            app,
            mcp_enable_mutations,
            false,
            excluded_filename_patterns,
            Arc::clone(&state.write_lock),
            Some(post_write_hook),
        );
    }
    let app = app.with_state(state);

    let url = format!("http://{}:{}", host, port);
    println!(
        "🌐 Server running at: \x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
        url, url
    );
    println!();
    println!("📖 Instructions:");
    println!("  • Open the link above in your browser");
    if enable_mcp {
        println!("  • MCP endpoint: {}/mcp", url);
    }
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
        return runtime_asset_response(state, method, RuntimeAssetKind::ProjectStore).await;
    }

    if request_path == "ontologies.ttl" {
        return runtime_asset_response(state, method, RuntimeAssetKind::Ontologies).await;
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

    if request_path == "api" || request_path.starts_with("api/") {
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

enum RuntimeAssetKind {
    ProjectStore,
    Ontologies,
}

async fn runtime_asset_response(
    state: ServeState,
    method: Method,
    kind: RuntimeAssetKind,
) -> Response<Body> {
    {
        // The MCP post-write hook already holds this gate. Ordinary runtime
        // requests acquire it here to avoid reading partially written sources.
        let _write_guard = state.write_lock.lock().await;
        // A failed external edit keeps the last valid static snapshot available;
        // the live API reports the failure to mounted Explorer clients.
        let _ = refresh_runtime_assets(&state, false).await;
    }
    if method == Method::HEAD {
        return no_store_response(match kind {
            RuntimeAssetKind::ProjectStore => "application/javascript",
            RuntimeAssetKind::Ontologies => "text/turtle; charset=utf-8",
        });
    }

    let snapshot = state.runtime_assets.lock().await;

    match kind {
        RuntimeAssetKind::ProjectStore => runtime_bytes_response(
            "application/javascript",
            format!(
                "{}window.reqvireLiveRefresh = {{\"revision\":\"{}\"}};\n",
                snapshot.assets.project_store_js, snapshot.revision
            )
            .into_bytes(),
        ),
        RuntimeAssetKind::Ontologies => runtime_bytes_response(
            "text/turtle; charset=utf-8",
            snapshot.assets.ontologies_ttl.clone().into_bytes(),
        ),
    }
}

async fn refresh_runtime_assets(state: &ServeState, force: bool) -> Result<(), ReqvireError> {
    let _refresh_guard = state.refresh_lock.lock().await;
    let fingerprint = {
        let snapshot = state.runtime_assets.lock().await;
        if !force
            && snapshot
                .last_checked
                .is_some_and(|checked| checked.elapsed() < REFRESH_CHECK_INTERVAL)
        {
            return match &snapshot.refresh_error {
                Some(error) => Err(ReqvireError::ProcessError(error.clone())),
                None => Ok(()),
            };
        }
        snapshot.fingerprint.clone()
    };
    let exclusions = Arc::clone(&state.excluded_filename_patterns);
    let result = tokio::task::spawn_blocking(move || {
        let changed = model_cache::load_cached_model_if_changed(
            exclusions.as_ref(),
            ModelBuildOptions {
                lenient: false,
                with_size_estimates: false,
            },
            fingerprint.as_ref(),
        )?;
        changed
            .map(|(fingerprint, model)| {
                build_runtime_assets(&model.graph_registry).map(|assets| (fingerprint, assets))
            })
            .transpose()
    })
    .await
    .map_err(|error| ReqvireError::ProcessError(format!("Runtime refresh task failed: {error}")))
    .and_then(|result| result);

    let mut snapshot = state.runtime_assets.lock().await;
    snapshot.last_checked = Some(Instant::now());
    match result {
        Ok(changed) => {
            if let Some((fingerprint, assets)) = changed {
                snapshot.revision = runtime_revision(&assets);
                snapshot.assets = assets;
                snapshot.fingerprint = Some(fingerprint);
            }
            snapshot.refresh_error = None;
            Ok(())
        }
        Err(error) => {
            snapshot.refresh_error = Some(error.to_string());
            Err(error)
        }
    }
}

#[derive(Default, Deserialize)]
struct LiveStoreQuery {
    #[serde(default)]
    refresh: bool,
}

async fn serve_live_store(
    State(state): State<ServeState>,
    Query(query): Query<LiveStoreQuery>,
    method: Method,
    headers: HeaderMap,
) -> Response<Body> {
    {
        let _write_guard = state.write_lock.lock().await;
        let _ = refresh_runtime_assets(&state, query.refresh).await;
    }
    let snapshot = state.runtime_assets.lock().await;
    let etag = format!("\"{}\"", snapshot.revision);
    let (status, body) = if let Some(error) = &snapshot.refresh_error {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            serde_json::json!({
                "revision": snapshot.revision, "error": error,
            })
            .to_string(),
        )
    } else if headers
        .get(header::IF_NONE_MATCH)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            value.split(',').any(|candidate| {
                let candidate = candidate
                    .trim()
                    .strip_prefix("W/")
                    .unwrap_or(candidate.trim());
                candidate == etag || candidate == "*"
            })
        })
    {
        (StatusCode::NOT_MODIFIED, String::new())
    } else {
        (
            StatusCode::OK,
            format!(
                "{{\"revision\":\"{}\",\"store\":{}}}",
                snapshot.revision, snapshot.assets.project_store_json
            ),
        )
    };
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "no-store")
        .header(header::ETAG, etag)
        .body(if method == Method::HEAD {
            Body::empty()
        } else {
            Body::from(body)
        })
        .unwrap_or_else(|_| response_with_status(StatusCode::INTERNAL_SERVER_ERROR))
}

fn runtime_bytes_response(content_type: &'static str, content: Vec<u8>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "no-store")
        .body(Body::from(content))
        .unwrap_or_else(|_| response_with_status(StatusCode::INTERNAL_SERVER_ERROR))
}

fn no_store_response(content_type: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "no-store")
        .body(Body::empty())
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
    let eligible = reqvire::workspace::WorkspaceScope::discover()
        .map(|scope| scope.is_eligible_path(&absolute_path))
        .unwrap_or(false);
    if !eligible {
        return None;
    }
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
