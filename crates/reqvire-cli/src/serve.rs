use crate::live_store::{ChunkRequest, LiveStore};
use crate::mcp;
use reqvire::exclusions::ExclusionSet as GlobSet;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::body::Body;
use axum::extract::State;
use axum::http::{header, HeaderMap, Method, Response, StatusCode, Uri};
use axum::routing::{any, get, post};
use axum::Json;
use axum::Router;
use percent_encoding::percent_decode_str;
use reqvire::error::ReqvireError;
use reqvire::explorer_runtime::{
    build_runtime_assets, embedded_asset, index_html, is_workspace_asset_path,
    ExplorerRuntimeAssets,
};
use reqvire::{model_cache, ModelBuildOptions};

struct RuntimeSnapshot {
    assets: ExplorerRuntimeAssets,
    live: LiveStore,
}

impl RuntimeSnapshot {
    fn new(assets: ExplorerRuntimeAssets) -> Result<Self, ReqvireError> {
        let live = LiveStore::new(&assets)?;
        Ok(Self { assets, live })
    }
}

struct RuntimeState {
    snapshot: Arc<RuntimeSnapshot>,
    refresh_error: Option<String>,
}

#[derive(Clone)]
pub struct ServeState {
    excluded_filename_patterns: Arc<GlobSet>,
    runtime_assets: Arc<Mutex<RuntimeState>>,
    live_refresh: bool,
    write_lock: Arc<Mutex<()>>,
}

/// Starts an HTTP server for the embedded Explorer SPA and generated runtime data.
pub async fn serve_explorer(
    assets: ExplorerRuntimeAssets,
    host: &str,
    port: u16,
    enable_mcp: bool,
    mcp_enable_mutations: bool,
    mcp_enable_commits: bool,
    excluded_filename_patterns: &GlobSet,
    http_access: &crate::mcp_http::HttpAccess,
) -> Result<(), ReqvireError> {
    let listener = tokio::net::TcpListener::bind((crate::mcp_http::listener_hostname(host), port))
        .await
        .map_err(|e| ReqvireError::ProcessError(format!("Failed to start server: {}", e)))?;

    let port = listener
        .local_addr()
        .map_err(|e| ReqvireError::ProcessError(e.to_string()))?
        .port();
    let state = ServeState {
        excluded_filename_patterns: Arc::new(excluded_filename_patterns.clone()),
        runtime_assets: Arc::new(Mutex::new(RuntimeState {
            snapshot: Arc::new(RuntimeSnapshot::new(assets)?),
            refresh_error: None,
        })),
        live_refresh: enable_mcp && mcp_enable_mutations,
        write_lock: Arc::new(Mutex::new(())),
    };
    let mut app = explorer_routes(state.live_refresh);

    if enable_mcp {
        let http_access = http_access
            .for_listener(host, port)
            .map_err(ReqvireError::ProcessError)?;
        let refresh_state = state.clone();
        let post_write_hook: mcp::PostWriteHook = Arc::new(move |model| {
            let refresh_state = refresh_state.clone();
            Box::pin(async move { refresh_runtime_assets(&refresh_state, model).await })
                as Pin<Box<dyn std::future::Future<Output = Result<(), ReqvireError>> + Send>>
        });
        app = mcp::mount_service_with_post_write_hook(
            app,
            mcp_enable_mutations,
            mcp_enable_commits,
            false,
            excluded_filename_patterns,
            Arc::clone(&state.write_lock),
            state.live_refresh.then_some(post_write_hook),
            &http_access,
        )?;
    }
    let app = app.with_state(state);

    let url = format!("http://{}", crate::mcp_http::endpoint_authority(host, port));
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

fn explorer_routes(live_refresh: bool) -> Router<ServeState> {
    let app = Router::new()
        .route("/", any(serve_static))
        .fallback(serve_static);
    if live_refresh {
        app.route("/api/project-store", get(serve_live_store))
            .route("/api/project-store/manifest", get(serve_manifest))
            .route("/api/project-store/chunks", post(serve_chunks))
    } else {
        app
    }
}

async fn serve_static(State(state): State<ServeState>, method: Method, uri: Uri) -> Response<Body> {
    let request_path = match resolve_request_path(uri.path()) {
        Ok(path) => path,
        Err(status) => return response_with_status(status),
    };

    if request_path == "api" || request_path.starts_with("api/") {
        return response_with_status(StatusCode::NOT_FOUND);
    }
    if method != Method::GET && method != Method::HEAD {
        return response_with_status(StatusCode::METHOD_NOT_ALLOWED);
    }

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
    if method == Method::HEAD {
        return no_store_response(match kind {
            RuntimeAssetKind::ProjectStore => "application/javascript",
            RuntimeAssetKind::Ontologies => "text/turtle; charset=utf-8",
        });
    }

    let snapshot = Arc::clone(&state.runtime_assets.lock().await.snapshot);

    match kind {
        RuntimeAssetKind::ProjectStore => runtime_bytes_response(
            "application/javascript",
            if state.live_refresh {
                format!(
                    "{}window.reqvireLiveRefresh = {{\"revision\":\"{}\",\"manifest\":{}}};\n",
                    snapshot.assets.project_store_js,
                    snapshot.live.revision,
                    snapshot.live.manifest_json
                )
            } else {
                snapshot.assets.project_store_js.clone()
            }
            .into_bytes(),
        ),
        RuntimeAssetKind::Ontologies => runtime_bytes_response(
            "text/turtle; charset=utf-8",
            snapshot.assets.ontologies_ttl.clone().into_bytes(),
        ),
    }
}

/// Called only after an embedded MCP write, while the MCP workspace write gate
/// is held. Browser requests read the published snapshot without model I/O.
async fn refresh_runtime_assets(
    state: &ServeState,
    accepted: Option<reqvire::ModelManager>,
) -> Result<(), ReqvireError> {
    let exclusions = Arc::clone(&state.excluded_filename_patterns);
    let result = tokio::task::spawn_blocking(move || {
        let model = if let Some(model) = accepted {
            model
        } else {
            model_cache::load_cached_model(
                exclusions.as_ref(),
                ModelBuildOptions {
                    lenient: false,
                    with_size_estimates: false,
                },
            )?
        };
        let assets = build_runtime_assets(&model)?;
        RuntimeSnapshot::new(assets).map(Arc::new)
    })
    .await
    .map_err(|error| ReqvireError::ProcessError(format!("Runtime refresh task failed: {error}")))
    .and_then(|result| result);

    let mut published = state.runtime_assets.lock().await;
    match result {
        Ok(snapshot) => {
            published.snapshot = snapshot;
            published.refresh_error = None;
            drop(published);
            Ok(())
        }
        Err(error) => {
            published.refresh_error = Some(error.to_string());
            drop(published);
            Err(error)
        }
    }
}

async fn serve_live_store(
    State(state): State<ServeState>,
    method: Method,
    headers: HeaderMap,
) -> Response<Body> {
    live_response(state, method, headers, false).await
}

async fn serve_manifest(
    State(state): State<ServeState>,
    method: Method,
    headers: HeaderMap,
) -> Response<Body> {
    live_response(state, method, headers, true).await
}

async fn live_response(
    state: ServeState,
    method: Method,
    headers: HeaderMap,
    manifest: bool,
) -> Response<Body> {
    let (snapshot, error) = {
        let published = state.runtime_assets.lock().await;
        (
            Arc::clone(&published.snapshot),
            published.refresh_error.clone(),
        )
    };
    let revision = &snapshot.live.revision;
    let etag = format!("\"{revision}\"");
    let (status, body) = error.map_or_else(
        || {
            if headers
                .get(header::IF_NONE_MATCH)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    value.split(',').any(|candidate| {
                        let candidate = candidate
                            .trim()
                            .strip_prefix("W/")
                            .unwrap_or_else(|| candidate.trim());
                        candidate == etag || candidate == "*"
                    })
                })
            {
                (StatusCode::NOT_MODIFIED, String::new())
            } else if method == Method::HEAD {
                (StatusCode::OK, String::new())
            } else if manifest {
                (StatusCode::OK, snapshot.live.manifest_json.clone())
            } else {
                (
                    StatusCode::OK,
                    format!(
                        "{{\"revision\":\"{revision}\",\"store\":{}}}",
                        snapshot.assets.project_store_json
                    ),
                )
            }
        },
        |error| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                serde_json::json!({ "revision": revision, "error": error }).to_string(),
            )
        },
    );
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

async fn serve_chunks(
    State(state): State<ServeState>,
    Json(request): Json<ChunkRequest>,
) -> Response<Body> {
    let snapshot = Arc::clone(&state.runtime_assets.lock().await.snapshot);
    match snapshot.live.chunks_json(&request) {
        Ok(body) => runtime_bytes_response("application/json", body.into_bytes()),
        Err(status) => Response::builder()
            .status(status)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CACHE_CONTROL, "no-store")
            .body(Body::from(
                serde_json::json!({
                    "error": if status == StatusCode::CONFLICT {
                        "The published snapshot changed; request the latest manifest."
                    } else {
                        "Invalid or unavailable chunk request."
                    },
                })
                .to_string(),
            ))
            .unwrap_or_else(|_| response_with_status(StatusCode::INTERNAL_SERVER_ERROR)),
    }
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

    std::fs::read(&absolute_path).map_or_else(
        |_| Some(response_with_status(StatusCode::NOT_FOUND)),
        |content| {
            Some(bytes_response(
                method,
                content_type_for_path(request_path),
                content,
            ))
        },
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::Request;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    fn snapshot(name: &str) -> Arc<RuntimeSnapshot> {
        let json = json!({"elements": [{"id": "aster", "name": name}]}).to_string();
        Arc::new(
            RuntimeSnapshot::new(ExplorerRuntimeAssets {
                project_store_js: format!("window.reqvireProjectStore = {json};\n"),
                project_store_json: json,
                ontologies_ttl: "@prefix : <urn:test:> .".into(),
            })
            .expect("build test runtime snapshot"),
        )
    }

    fn state(live_refresh: bool) -> ServeState {
        ServeState {
            excluded_filename_patterns: Arc::new(
                reqvire::exclusions::ExclusionSetBuilder::new()
                    .build()
                    .expect("build test configuration"),
            ),
            runtime_assets: Arc::new(Mutex::new(RuntimeState {
                snapshot: snapshot("initial"),
                refresh_error: None,
            })),
            live_refresh,
            write_lock: Arc::new(Mutex::new(())),
        }
    }

    async fn body(response: Response<Body>) -> String {
        String::from_utf8(
            to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("read response body")
                .to_vec(),
        )
        .expect("expected UTF-8 response body")
    }

    #[tokio::test]
    async fn manifest_headers_conditional_variants_and_head_are_consistent() {
        let state = state(true);
        let expected = Arc::clone(&state.runtime_assets.lock().await.snapshot);
        let app = explorer_routes(true).with_state(state);
        let etag = format!("\"{}\"", expected.live.revision);
        for (condition, status) in [
            (etag.clone(), 304),
            (format!("W/{etag}"), 304),
            (format!("\"old\", {etag}"), 304),
            ("*".into(), 304),
            ("\"old\"".into(), 200),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/api/project-store/manifest")
                        .header(header::IF_NONE_MATCH, condition)
                        .body(Body::empty())
                        .expect("build valid test HTTP request"),
                )
                .await
                .unwrap();
            assert_eq!(response.status().as_u16(), status);
            assert_eq!(response.headers()[header::ETAG], etag);
            assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
            let text = body(response).await;
            assert_eq!(
                text,
                if status == 304 {
                    ""
                } else {
                    &expected.live.manifest_json
                }
            );
        }
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::HEAD)
                    .uri("/api/project-store/manifest")
                    .body(Body::empty())
                    .expect("build valid test HTTP request"),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[header::ETAG], etag);
        assert!(body(response).await.is_empty());
    }

    #[tokio::test]
    async fn revision_reads_do_not_wait_for_the_mcp_write_gate() {
        let state = state(true);
        let _write_guard = state.write_lock.lock().await;
        let app = explorer_routes(true).with_state(state.clone());
        let response = tokio::time::timeout(
            std::time::Duration::from_millis(500),
            app.oneshot(
                Request::builder()
                    .uri("/api/project-store/manifest")
                    .body(Body::empty())
                    .expect("build valid test HTTP request"),
            ),
        )
        .await
        .expect("manifest read should finish while the model gate is held")
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn disabled_serving_has_no_manifest_chunks_or_seed_advertisement() {
        let state = state(false);
        let app = explorer_routes(false).with_state(state);
        for (url, method) in [
            ("/api/project-store", Method::GET),
            ("/api/project-store/manifest", Method::GET),
            ("/api/project-store/chunks", Method::POST),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(method)
                        .uri(url)
                        .body(Body::empty())
                        .expect("build valid test HTTP request"),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/assets/project-store.js")
                    .body(Body::empty())
                    .expect("build valid test HTTP request"),
            )
            .await
            .unwrap();
        assert!(!body(response).await.contains("reqvireLiveRefresh"));
    }

    #[tokio::test]
    async fn superseded_manifest_conflicts_but_captured_snapshot_remains_immutable() {
        let state = state(true);
        let captured = Arc::clone(&state.runtime_assets.lock().await.snapshot);
        let manifest: Value =
            serde_json::from_str(&captured.live.manifest_json).expect("parse generated JSON");
        let hash = manifest["sections"]["elements"]["hashes"][0]
            .as_str()
            .expect("expected a JSON string")
            .to_string();
        let request = ChunkRequest {
            revision: captured.live.revision.clone(),
            hashes: vec![hash.clone()],
        };
        state.runtime_assets.lock().await.snapshot = snapshot("updated");
        let captured_response: Value = serde_json::from_str(
            &captured
                .live
                .chunks_json(&request)
                .expect("retrieve requested test chunks"),
        )
        .expect("parse generated JSON");
        assert!(captured_response["chunks"][&hash]
            .as_str()
            .expect("expected a JSON string")
            .contains("initial"));
        let app = explorer_routes(true).with_state(state);
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/project-store/chunks")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({"revision": request.revision, "hashes": request.hashes}).to_string(),
                    ))
                    .expect("build valid test HTTP request"),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
        assert!(body(response).await.contains("latest manifest"));
    }

    #[tokio::test]
    async fn refresh_diagnostics_preserve_assets_and_clear_without_advancing_revision() {
        let state = state(true);
        let revision = state
            .runtime_assets
            .lock()
            .await
            .snapshot
            .live
            .revision
            .clone();
        state.runtime_assets.lock().await.refresh_error = Some("Failed runtime generation".into());
        let app = explorer_routes(true).with_state(state.clone());
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/project-store/manifest")
                    .header(header::IF_NONE_MATCH, format!("\"{revision}\""))
                    .body(Body::empty())
                    .expect("build valid test HTTP request"),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let diagnostic: Value =
            serde_json::from_str(&body(response).await).expect("parse generated JSON");
        assert_eq!(diagnostic["revision"], revision);
        let seed = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/assets/project-store.js")
                    .body(Body::empty())
                    .expect("build valid test HTTP request"),
            )
            .await
            .unwrap();
        assert!(body(seed).await.contains("initial"));
        state.runtime_assets.lock().await.refresh_error = None;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/project-store/manifest")
                    .header(header::IF_NONE_MATCH, format!("\"{revision}\""))
                    .body(Body::empty())
                    .expect("build valid test HTTP request"),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
    }
}
