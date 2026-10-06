use crate::live_store::ChunkRequest;
use crate::mcp;
use reqvire::exclusions::ExclusionSet as GlobSet;
use std::path::Path;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::{header, HeaderMap, Method, Response, StatusCode, Uri};
use axum::routing::{any, get, post};
use axum::Json;
use axum::Router;
use percent_encoding::percent_decode_str;
use reqvire::error::ReqvireError;
use reqvire::explorer_runtime::{
    build_runtime_data, embedded_asset, index_html, is_workspace_asset_path,
};
use reqvire::ModelBuildOptions;

use crate::mcp_worktrees::PublishedRuntime as RuntimeSnapshot;

#[derive(Clone)]
enum RuntimeSource {
    Worktrees(Arc<crate::mcp_worktrees::Worktrees>),
    Local(Arc<RuntimeSnapshot>),
}

#[derive(Clone)]
pub struct ServeState {
    runtime: RuntimeSource,
    live_refresh: bool,
}

impl ServeState {
    const fn worktrees(&self) -> Option<&Arc<crate::mcp_worktrees::Worktrees>> {
        match &self.runtime {
            RuntimeSource::Worktrees(worktrees) => Some(worktrees),
            RuntimeSource::Local(_) => None,
        }
    }

    const fn has_worktrees(&self) -> bool {
        self.worktrees().is_some()
    }

    fn local_runtime(&self) -> Result<&Arc<RuntimeSnapshot>, ReqvireError> {
        match &self.runtime {
            RuntimeSource::Local(runtime) => Ok(runtime),
            RuntimeSource::Worktrees(_) => Err(ReqvireError::ProcessError(
                "Worktree runtimes are published by their workers".into(),
            )),
        }
    }
}

/// Select one runtime owner before constructing any model. The local path is
/// only for an effective workspace outside a Git worktree (e.g. an aggregate).
fn prepare_runtime(
    root: &Path,
    executable: &Path,
    enable_mcp: bool,
    options: mcp::McpOptions<'_>,
    excluded_filename_patterns: &GlobSet,
) -> Result<RuntimeSource, ReqvireError> {
    if enable_mcp && options.enable_mutations {
        Ok(RuntimeSource::Worktrees(
            crate::mcp_worktrees::Worktrees::start(
                root,
                executable,
                options.enable_commits,
                false,
                true,
                options.enable_github,
                options.github_remote,
            )?,
        ))
    } else if crate::mcp_process::git(root, &["rev-parse", "--git-common-dir"]).is_ok() {
        Ok(RuntimeSource::Worktrees(
            crate::mcp_worktrees::Worktrees::start_read_only(root, executable)?,
        ))
    } else {
        let mut model = reqvire::ModelManager::new();
        model.parse_and_validate_with_options(
            None,
            excluded_filename_patterns,
            ModelBuildOptions {
                lenient: false,
                with_size_estimates: false,
            },
        )?;
        let snapshot = RuntimeSnapshot::new(build_runtime_data(&model, None)?)?;
        Ok(RuntimeSource::Local(Arc::new(snapshot)))
    }
}

/// Starts an HTTP server for the embedded Explorer SPA and generated runtime data.
pub async fn serve_explorer(
    host: &str,
    port: u16,
    enable_mcp: bool,
    options: mcp::McpOptions<'_>,
    excluded_filename_patterns: &GlobSet,
    http_access: &crate::mcp_http::HttpAccess,
) -> Result<(), ReqvireError> {
    let runtime = prepare_runtime(
        &std::env::current_dir()?,
        &std::env::current_exe()?,
        enable_mcp,
        options,
        excluded_filename_patterns,
    )?;
    let listener = tokio::net::TcpListener::bind((crate::mcp_http::listener_hostname(host), port))
        .await
        .map_err(|e| ReqvireError::ProcessError(format!("Failed to start server: {}", e)))?;

    let port = listener
        .local_addr()
        .map_err(|e| ReqvireError::ProcessError(e.to_string()))?
        .port();
    let state = ServeState {
        runtime,
        live_refresh: enable_mcp && options.enable_mutations,
    };
    let mut app = explorer_routes();

    if enable_mcp {
        let http_access = http_access
            .for_listener(host, port)
            .map_err(ReqvireError::ProcessError)?;
        app = if let Some(worktrees) = state.worktrees().filter(|_| options.enable_mutations) {
            mcp::mount_worktrees(
                app,
                Arc::clone(worktrees),
                excluded_filename_patterns,
                &http_access,
            )?
        } else {
            mcp::mount_read_only(
                app,
                options.with_size_estimates,
                excluded_filename_patterns,
                Arc::new(tokio::sync::RwLock::new(())),
                &http_access,
            )
        };
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

fn explorer_routes() -> Router<ServeState> {
    Router::new()
        .route("/", any(serve_static))
        .route("/api/worktrees", get(serve_worktrees))
        .route("/api/worktrees/load", post(load_worktree))
        .route("/api/project-store", get(serve_live_store))
        .route("/api/project-store/manifest", get(serve_manifest))
        .route("/api/project-store/chunks", post(serve_chunks))
        .fallback(serve_static)
}

async fn serve_worktrees(State(state): State<ServeState>) -> Response<Body> {
    match state.runtime {
        RuntimeSource::Worktrees(worktrees) => {
            (tokio::task::spawn_blocking(move || worktrees.browser_inventory()).await).map_or_else(
                |_| response_with_status(StatusCode::SERVICE_UNAVAILABLE),
                |value| {
                    let failed = value.get("error").is_some();
                    let mut response =
                        runtime_bytes_response("application/json", value.to_string().into_bytes());
                    if failed {
                        *response.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
                    }
                    response
                },
            )
        }
        RuntimeSource::Local(_) => response_with_status(StatusCode::NOT_FOUND),
    }
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BranchLoad {
    worktree_id: String,
}
async fn load_worktree(
    State(state): State<ServeState>,
    Json(selection): Json<BranchLoad>,
) -> Response<Body> {
    let RuntimeSource::Worktrees(worktrees) = state.runtime else {
        return response_with_status(StatusCode::NOT_FOUND);
    };
    let id = selection.worktree_id;
    let requested = id.clone();
    let result = tokio::task::spawn_blocking(move || {
        worktrees
            .load_browser_context(Some(&id))
            .map(|c| c.metadata())
    })
    .await;
    let value = match result {
        Ok(Ok(value)) => {
            return runtime_bytes_response("application/json", value.to_string().into_bytes())
        }
        Ok(Err(e)) => e.to_string(),
        Err(e) => e.to_string(),
    };
    Response::builder()
        .status(StatusCode::SERVICE_UNAVAILABLE)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "no-store")
        .body(Body::from(
            serde_json::json!({"worktree_id":requested,"error":value}).to_string(),
        ))
        .expect("response uses a valid status and static headers")
}
async fn selected_context(
    state: &ServeState,
    uri: &Uri,
    load: bool,
) -> Result<Arc<crate::mcp_worktrees::Context>, ReqvireError> {
    let id = crate::mcp_worktrees::parse_selector(uri.query())?;
    let initial_read = uri.path() == "/api/project-store";
    let worktrees = Arc::clone(
        state
            .worktrees()
            .ok_or_else(|| ReqvireError::ProcessError("Branch selection is unavailable".into()))?,
    );
    tokio::task::spawn_blocking(move || {
        if load {
            worktrees.load_browser_context(id.as_deref())
        } else if initial_read {
            worktrees.browser_context(id.as_deref())
        } else {
            worktrees.published_browser_context(id.as_deref())
        }
    })
    .await
    .map_err(|e| ReqvireError::ProcessError(e.to_string()))?
}
async fn snapshot_for(state: &ServeState, uri: &Uri) -> Result<Arc<RuntimeSnapshot>, ReqvireError> {
    let id = crate::mcp_worktrees::parse_selector(uri.query())?;
    if state.has_worktrees() {
        return selected_context(state, uri, uri.path() == "/assets/project-store.js")
            .await?
            .runtime();
    }
    if id.is_some() {
        return Err(ReqvireError::ProcessError(
            "Worktree selection is unavailable on this server".into(),
        ));
    }
    Ok(Arc::clone(state.local_runtime()?))
}
fn context_error(uri: &Uri, error: ReqvireError) -> Response<Body> {
    Response::builder().status(StatusCode::SERVICE_UNAVAILABLE)
        .header(header::CONTENT_TYPE,"application/json").header(header::CACHE_CONTROL,"no-store")
        .body(Body::from(serde_json::json!({"error":error.to_string(),"worktree_id":crate::mcp_worktrees::parse_selector(uri.query()).ok().flatten()}).to_string())).expect("response uses a valid status and static headers")
}
fn shell_response(method: Method, uri: &Uri, worktrees: bool) -> Response<Body> {
    if !worktrees {
        return static_response(method, "text/html; charset=utf-8", index_html());
    }
    let mut html = String::from_utf8_lossy(index_html()).into_owned();
    if let Ok(Some(id)) = crate::mcp_worktrees::parse_selector(uri.query()) {
        let id = percent_encoding::utf8_percent_encode(&id, percent_encoding::NON_ALPHANUMERIC);
        html = html.replace(
            "assets/project-store.js",
            &format!("assets/project-store.js?worktree_id={id}"),
        );
    }
    html = html.replace(
        "<head>",
        "<head><script>window.reqvireWorktreeRouting = true;</script>",
    );
    bytes_response(method, "text/html; charset=utf-8", html.into_bytes())
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
        return runtime_asset_response(state, method, &uri, RuntimeAssetKind::ProjectStore).await;
    }

    if request_path == "ontologies.ttl" {
        return runtime_asset_response(state, method, &uri, RuntimeAssetKind::Ontologies).await;
    }

    // Handle the shell before the compiled-asset lookup, which also contains index.html.
    // Expired context IDs must still get a shell with explicit recovery choices.
    if request_path == "index.html" {
        return shell_response(method, &uri, state.has_worktrees());
    }

    if let Some(content) = embedded_asset(&request_path) {
        return static_response(method, content_type_for_path(&request_path), content);
    }

    let root =
        if state.has_worktrees() {
            let selected =
                selected_context(&state, &uri, false).await.and_then(|c| {
                    let status = c.metadata();
                    if status["recovery_required"] == true {
                        return Err(ReqvireError::ProcessError(format!(
                        "Worktree {} requires recovery; local file downloads are unavailable: {}",
                        c.id, status["diagnostic"].as_str().unwrap_or("incomplete restoration")
                    )));
                    }
                    Ok(c.root.clone())
                });
            match selected {
                Ok(root) => Some(root),
                Err(e) => return context_error(&uri, e),
            }
        } else {
            None
        };
    if let Some(response) =
        workspace_file_response_at(method.clone(), &request_path, root.as_deref())
    {
        return response;
    }

    if request_path.starts_with("assets/") {
        return response_with_status(StatusCode::NOT_FOUND);
    }

    shell_response(method, &uri, state.has_worktrees())
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
    uri: &Uri,
    kind: RuntimeAssetKind,
) -> Response<Body> {
    let snapshot = match snapshot_for(&state, uri).await {
        Ok(snapshot) => snapshot,
        Err(e) => return context_error(uri, e),
    };
    if method == Method::HEAD {
        return no_store_response(match kind {
            RuntimeAssetKind::ProjectStore => "application/javascript",
            RuntimeAssetKind::Ontologies => "text/turtle; charset=utf-8",
        });
    }

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

async fn serve_live_store(
    State(state): State<ServeState>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
) -> Response<Body> {
    live_response(state, method, headers, &uri, false).await
}

async fn serve_manifest(
    State(state): State<ServeState>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
) -> Response<Body> {
    live_response(state, method, headers, &uri, true).await
}

async fn live_response(
    state: ServeState,
    method: Method,
    headers: HeaderMap,
    uri: &Uri,
    manifest: bool,
) -> Response<Body> {
    let selected = match snapshot_for(&state, uri).await {
        Ok(snapshot) => snapshot,
        Err(e) => return context_error(uri, e),
    };
    let snapshot = selected;
    let recovery_required = if state.has_worktrees() {
        selected_context(&state, uri, false)
            .await
            .is_ok_and(|c| c.metadata()["recovery_required"] == true)
    } else {
        false
    };
    let revision = &snapshot.live.revision;
    let etag = format!("\"{revision}\"");
    let (status, body) = if headers
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
        }) {
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
    };
    let mut response = Response::builder();
    if recovery_required {
        response = response.header("X-Reqvire-Recovery-Required", "true");
    }
    response
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
    uri: Uri,
    Json(request): Json<ChunkRequest>,
) -> Response<Body> {
    let snapshot = match snapshot_for(&state, &uri).await {
        Ok(snapshot) => snapshot,
        Err(e) => return context_error(&uri, e),
    };
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

fn workspace_file_response_at(
    method: Method,
    request_path: &str,
    root: Option<&Path>,
) -> Option<Response<Body>> {
    let path = Path::new(request_path);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        || !is_workspace_asset_path(path)
    {
        return None;
    }

    let root = root.unwrap_or_else(|| Path::new("."));
    let absolute_path = root.join(path);
    let eligible = reqvire::workspace::WorkspaceScope::discover_from(root.to_path_buf())
        .map(|scope| scope.is_eligible_path(&absolute_path))
        .unwrap_or(false);
    if !eligible {
        return None;
    }
    if path.components().any(|p| p.as_os_str() == ".git")
        || !absolute_path
            .canonicalize()
            .ok()
            .zip(root.canonicalize().ok())
            .is_some_and(|(path, root)| path.starts_with(root))
    {
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
        Arc::new(
            RuntimeSnapshot::new(reqvire::explorer_runtime::ExplorerRuntimeData {
                project_store: json!({"elements": [{"id": "aster", "name": name}]}),
                ontologies_ttl: "@prefix : <urn:test:> .".into(),
            })
            .expect("build test runtime snapshot"),
        )
    }

    fn state(live_refresh: bool) -> ServeState {
        ServeState {
            runtime: RuntimeSource::Local(snapshot("initial")),
            live_refresh,
        }
    }

    fn worker_state() -> (
        tempfile::TempDir,
        ServeState,
        Arc<crate::mcp_worktrees::Context>,
    ) {
        let directory = tempfile::tempdir().expect("worker fixture directory");
        let root = directory.path();
        for args in [
            vec!["init", "-qb", "main"],
            vec!["config", "user.name", "Serve Test"],
            vec!["config", "user.email", "serve@example.invalid"],
        ] {
            assert!(std::process::Command::new("git")
                .current_dir(root)
                .args(args)
                .status()
                .expect("fixture Git")
                .success());
        }
        std::fs::write(
            root.join("Model.md"),
            include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt"),
        )
        .expect("worker source");
        for args in [vec!["add", "."], vec!["commit", "-qm", "baseline"]] {
            assert!(std::process::Command::new("git")
                .current_dir(root)
                .args(args)
                .status()
                .expect("fixture Git")
                .success());
        }
        let executable = std::env::var_os("REQVIRE_TEST_BIN")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::env::current_exe()
                    .expect("test executable")
                    .parent()
                    .expect("deps directory")
                    .parent()
                    .expect("target directory")
                    .join(format!("reqvire{}", std::env::consts::EXE_SUFFIX))
            });
        let manager = crate::mcp_worktrees::Worktrees::start(
            root,
            &executable,
            false,
            false,
            true,
            false,
            "origin",
        )
        .expect("production worktree worker");
        let context = manager
            .published_browser_context(None)
            .expect("original context");
        (
            directory,
            ServeState {
                runtime: RuntimeSource::Worktrees(manager),
                live_refresh: true,
            },
            context,
        )
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
    async fn startup_uses_only_worker_runtime_with_consistent_initial_routes() {
        let executable = std::env::var_os("REQVIRE_TEST_BIN")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::env::current_exe()
                    .expect("test fixture operation should succeed")
                    .parent()
                    .expect("fixture path has a parent")
                    .parent()
                    .expect("fixture path has a parent")
                    .join("reqvire")
            });
        for (mcp, mutations, commits) in [
            (false, false, false),
            (true, false, false),
            (true, true, false),
            (true, true, true),
        ] {
            let temp = tempfile::tempdir().expect("test fixture operation should succeed");
            let root = temp.path().join("repo");
            std::fs::create_dir(&root).expect("test fixture operation should succeed");
            let git = |args: &[&str]| {
                crate::mcp_process::git(&root, args).expect("fixture Git command should succeed")
            };
            git(&["init", "-qb", "main"]);
            git(&["config", "user.name", "Startup Test"]);
            git(&["config", "user.email", "startup@example.invalid"]);
            std::fs::write(
                root.join("Model.md"),
                include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt"),
            )
            .expect("write test fixture");
            git(&["add", "."]);
            git(&["commit", "-qm", "baseline"]);
            let mut state = state(mutations);
            state.runtime = prepare_runtime(
                &root,
                &executable,
                mcp,
                mcp::McpOptions {
                    enable_mutations: mutations,
                    enable_commits: commits,
                    enable_github: false,
                    github_remote: "origin",
                    with_size_estimates: false,
                },
                &reqvire::exclusions::ExclusionSetBuilder::new()
                    .build()
                    .expect("startup exclusions"),
            )
            .expect("test fixture operation should succeed");
            assert!(
                state.local_runtime().is_err(),
                "worker-backed startup cannot retain a fallback snapshot"
            );
            let manager = Arc::clone(
                state
                    .worktrees()
                    .expect("original context is worker-backed"),
            );
            let context = manager
                .published_browser_context(None)
                .expect("test fixture operation should succeed");
            let accepted = context
                .runtime()
                .expect("test fixture operation should succeed");
            let expected: Value = serde_json::from_str(&accepted.assets.project_store_json)
                .expect("test fixture operation should succeed");
            assert_eq!(expected["project"]["worktree_id"], manager.original);
            assert_eq!(expected["project"]["branch"], "main");
            let app = explorer_routes().with_state(state);
            let get = |path: &str| {
                app.clone().oneshot(
                    Request::builder()
                        .uri(path)
                        .body(Body::empty())
                        .expect("build test HTTP request"),
                )
            };
            let store = get("/api/project-store").await.unwrap();
            assert_eq!(store.status(), StatusCode::OK);
            let store: Value = serde_json::from_str(&body(store).await)
                .expect("test fixture operation should succeed");
            assert_eq!(store["store"], expected);
            assert_eq!(store["revision"], accepted.live.revision);
            let manifest = get("/api/project-store/manifest").await.unwrap();
            assert_eq!(manifest.status(), StatusCode::OK);
            let manifest = body(manifest).await;
            assert_eq!(manifest, accepted.live.manifest_json);
            let manifest: Value =
                serde_json::from_str(&manifest).expect("test fixture operation should succeed");
            let hashes: Vec<_> = manifest["sections"]
                .as_object()
                .expect("expected an object in the test response")
                .values()
                .flat_map(|section| {
                    section["hashes"]
                        .as_array()
                        .cloned()
                        .unwrap_or_else(|| vec![section["hash"].clone()])
                })
                .collect();
            let request = ChunkRequest {
                revision: accepted.live.revision.clone(),
                hashes: hashes
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .expect("expected a string in the test response")
                            .to_owned()
                    })
                    .collect(),
            };
            let chunks = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/api/project-store/chunks")
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(
                            json!({"revision":request.revision,"hashes":hashes}).to_string(),
                        ))
                        .expect("build test HTTP request"),
                )
                .await
                .unwrap();
            assert_eq!(chunks.status(), StatusCode::OK);
            assert_eq!(
                body(chunks).await,
                accepted
                    .live
                    .chunks_json(&request)
                    .expect("test fixture operation should succeed")
            );
            let seed = get("/assets/project-store.js").await.unwrap();
            assert_eq!(seed.status(), StatusCode::OK);
            let seed = body(seed).await;
            assert!(seed.starts_with(&accepted.assets.project_store_js));
            assert_eq!(seed.contains("reqvireLiveRefresh"), mutations);
            let ontology = get("/ontologies.ttl").await.unwrap();
            assert_eq!(ontology.status(), StatusCode::OK);
            assert_eq!(body(ontology).await, accepted.assets.ontologies_ttl);
            assert!(
                Arc::ptr_eq(
                    &accepted,
                    &context
                        .runtime()
                        .expect("test fixture operation should succeed")
                ),
                "unchanged initial reads must reuse the published snapshot"
            );
            assert!(git(&["status", "--porcelain"]).is_empty());
        }
    }

    #[tokio::test]
    async fn read_only_worktree_routes_do_not_require_mcp_or_ownership() {
        let temp = tempfile::tempdir().expect("test fixture operation should succeed");
        let root = temp.path().join("repo");
        let second = temp.path().join("second");
        let invalid = temp.path().join("invalid");
        std::fs::create_dir(&root).expect("test fixture operation should succeed");
        let git = |args: &[&str]| {
            crate::mcp_process::git(&root, args).expect("fixture Git command should succeed")
        };
        git(&["init", "-qb", "main"]);
        git(&["config", "user.name", "Read Test"]);
        git(&["config", "user.email", "read@example.invalid"]);
        let model = include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt");
        std::fs::write(root.join("Model.md"), model).expect("write test fixture");
        std::fs::write(root.join("shared.txt"), "original bytes").expect("write test fixture");
        git(&["add", "."]);
        git(&["commit", "-qm", "baseline"]);
        git(&[
            "worktree",
            "add",
            "-qb",
            "second",
            second.to_str().expect("fixture path is UTF-8"),
        ]);
        git(&[
            "worktree",
            "add",
            "-qb",
            "invalid",
            invalid.to_str().expect("fixture path is UTF-8"),
        ]);
        std::fs::write(
            second.join("Model.md"),
            model.replace("Cache Subject", "Other Subject"),
        )
        .expect("write test fixture");
        std::fs::write(second.join("shared.txt"), "second bytes").expect("write test fixture");
        std::fs::write(
            invalid.join("Model.md"),
            model.replace("capability", "invalid-type"),
        )
        .expect("write test fixture");
        std::fs::write(
            root.join("Model.md"),
            model.replace("Cache Subject", "Dirty Subject"),
        )
        .expect("write test fixture");
        let executable = std::env::var_os("REQVIRE_TEST_BIN")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::env::current_exe()
                    .expect("test fixture operation should succeed")
                    .parent()
                    .expect("fixture path has a parent")
                    .parent()
                    .expect("fixture path has a parent")
                    .join(format!("reqvire{}", std::env::consts::EXE_SUFFIX))
            });
        let contexts = crate::mcp_worktrees::Worktrees::start_read_only(&root, &executable)
            .expect("test fixture operation should succeed");
        let inventory = contexts.browser_inventory();
        let rows = inventory["worktrees"]
            .as_array()
            .expect("expected an array in the test response");
        assert_eq!(rows.len(), 3);
        assert!(rows.iter().all(|row| row["owned"] == false));
        let unavailable = rows
            .iter()
            .find(|row| row["branch"] == "invalid")
            .expect("test fixture operation should succeed");
        assert_eq!(
            unavailable["state"], "unloaded",
            "inventory must not validate other models"
        );
        let mut state = state(false);
        state.runtime = RuntimeSource::Worktrees(contexts);
        let app = explorer_routes().with_state(state);
        for (branch, name, asset) in [
            ("main", "Dirty Subject", "original bytes"),
            ("second", "Other Subject", "second bytes"),
        ] {
            let id = rows
                .iter()
                .find(|row| row["branch"] == branch)
                .expect("test fixture operation should succeed")["worktree_id"]
                .as_str()
                .expect("expected a string in the test response");
            let get = |path: String| {
                app.clone().oneshot(
                    Request::builder()
                        .uri(path)
                        .body(Body::empty())
                        .expect("build test HTTP request"),
                )
            };
            let response = get(format!("/api/project-store?worktree_id={id}"))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let store: Value = serde_json::from_str(&body(response).await)
                .expect("test fixture operation should succeed");
            assert_eq!(store["store"]["project"]["branch"], branch);
            assert!(store["store"]["elements"]
                .as_array()
                .expect("expected an array in the test response")
                .iter()
                .any(|e| e["name"] == name));
            let seed = body(
                get(format!("/assets/project-store.js?worktree_id={id}"))
                    .await
                    .unwrap(),
            )
            .await;
            assert!(seed.contains(id));
            assert!(!seed.contains("reqvireLiveRefresh"));
            assert_eq!(
                body(get(format!("/shared.txt?worktree_id={id}")).await.unwrap()).await,
                asset
            );
            assert_eq!(
                get(format!("/api/project-store/manifest?worktree_id={id}"))
                    .await
                    .unwrap()
                    .status(),
                StatusCode::OK
            );
        }
        std::fs::write(
            second.join("Model.md"),
            model.replace("Cache Subject", "Fresh Subject"),
        )
        .expect("write test fixture");
        let id = rows
            .iter()
            .find(|row| row["branch"] == "second")
            .expect("test fixture operation should succeed")["worktree_id"]
            .as_str()
            .expect("expected a string in the test response");
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/api/project-store?worktree_id={id}"))
                    .body(Body::empty())
                    .expect("build test HTTP request"),
            )
            .await
            .unwrap();
        assert!(body(response).await.contains("Other Subject"));
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/worktrees/load")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({"worktree_id":id}).to_string()))
                    .expect("build test HTTP request"),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/api/project-store?worktree_id={id}"))
                    .body(Body::empty())
                    .expect("build test HTTP request"),
            )
            .await
            .unwrap();
        assert!(body(response).await.contains("Fresh Subject"));
        for id in [
            "unknown",
            unavailable["worktree_id"]
                .as_str()
                .expect("expected a string in the test response"),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(format!("/api/project-store?worktree_id={id}"))
                        .body(Body::empty())
                        .expect("build test HTTP request"),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        }
        let shell = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .expect("build test HTTP request"),
            )
            .await
            .unwrap();
        assert!(body(shell)
            .await
            .contains("window.reqvireWorktreeRouting = true"));
        assert_eq!(git(&["branch", "--show-current"]), "main");
        assert!(!std::fs::read_dir(root.join(".git"))
            .expect("read test fixture")
            .any(|e| e
                .expect("test fixture operation should succeed")
                .file_name()
                .to_string_lossy()
                .starts_with("reqvire-mcp-worktree")));
    }

    #[cfg(unix)]
    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn recovery_routes_keep_snapshot_but_reject_disk_assets() {
        use std::os::unix::fs::PermissionsExt;
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let git = |args: &[&str]| crate::mcp_process::git(root, args).unwrap();
        git(&["init", "-qb", "main"]);
        git(&["config", "user.name", "Recovery Test"]);
        git(&["config", "user.email", "recovery@example.invalid"]);
        std::fs::write(
            root.join("Model.md"),
            include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt"),
        )
        .unwrap();
        std::fs::write(root.join("evidence.txt"), "original evidence").unwrap();
        git(&["add", "."]);
        git(&["commit", "-qm", "baseline"]);
        let executable = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(format!("reqvire{}", std::env::consts::EXE_SUFFIX));
        let manager = crate::mcp_worktrees::Worktrees::start(
            root,
            &executable,
            true,
            false,
            true,
            false,
            "origin",
        )
        .unwrap();
        let mut state = state(true);
        state.runtime = RuntimeSource::Worktrees(Arc::clone(&manager));
        let app = explorer_routes().with_state(state);
        let before = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/project-store/manifest")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let etag = before.headers()["etag"].clone();
        let manifest = body(before).await;
        let hook = root.join(".git/hooks/reference-transaction");
        std::fs::write(
            &hook,
            r#"#!/bin/sh
if [ "$1" = prepared ]; then rm -f Model.md; mkdir Model.md; exit 1; fi
"#,
        )
        .unwrap();
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).unwrap();
        let failed = manager.handle("tools/call", json!({"name":"reqvire.add_element","arguments":{"file":"Model.md","content":include_str!("../../../tests/test-cache-integration/fixtures/other.md.txt").split_once("# Elements\n\n").unwrap().1}})).unwrap();
        assert!(failed.to_string().contains("recovery failed"));
        for conditional in [false, true] {
            let mut request = Request::builder().uri("/api/project-store/manifest");
            if conditional {
                request = request.header("If-None-Match", &etag);
            }
            let response = app
                .clone()
                .oneshot(request.body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(
                response.status(),
                if conditional {
                    StatusCode::NOT_MODIFIED
                } else {
                    StatusCode::OK
                }
            );
            assert_eq!(response.headers()["etag"], etag);
            assert_eq!(
                response
                    .headers()
                    .get("X-Reqvire-Recovery-Required")
                    .and_then(|h| h.to_str().ok()),
                Some("true")
            );
            if !conditional {
                assert_eq!(body(response).await, manifest);
            }
        }
        for path in [
            "/api/project-store",
            "/assets/project-store.js",
            "/ontologies.ttl",
        ] {
            let response = app
                .clone()
                .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert!(!body(response).await.contains("Other Subject"));
        }
        let parsed: Value = serde_json::from_str(&manifest).unwrap();
        let hash = parsed["sections"]
            .as_object()
            .unwrap()
            .values()
            .find_map(|section| section["hash"].as_str())
            .unwrap();
        let revision = etag.to_str().unwrap().trim_matches('"');
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/project-store/chunks")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        json!({"revision": revision, "hashes": [hash]}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let chunks: Value = serde_json::from_str(&body(response).await).unwrap();
        assert_eq!(chunks["revision"], revision);
        assert!(chunks["chunks"][hash].is_string());
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/evidence.txt")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(body(response).await.contains("recovery"));
        std::fs::remove_file(hook).unwrap();
    }

    #[tokio::test]
    async fn worktree_routes_publish_only_the_selected_snapshot_and_asset_boundary() {
        let temp = tempfile::tempdir().expect("test fixture operation should succeed");
        let root = temp.path().join("repo");
        std::fs::create_dir(&root).expect("test fixture operation should succeed");
        let git = |args: &[&str]| {
            crate::mcp_process::git(&root, args).expect("fixture Git command should succeed")
        };
        git(&["init", "-qb", "main"]);
        git(&["config", "user.name", "Runtime Test"]);
        git(&["config", "user.email", "runtime@example.invalid"]);
        std::fs::write(
            root.join("Model.md"),
            include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt"),
        )
        .expect("write test fixture");
        std::fs::write(root.join("shared.txt"), "original bytes").expect("write test fixture");
        git(&["add", "."]);
        git(&["commit", "-qm", "baseline"]);
        let executable = std::env::var_os("REQVIRE_TEST_BIN")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::env::current_exe()
                    .expect("test fixture operation should succeed")
                    .parent()
                    .expect("fixture path has a parent")
                    .parent()
                    .expect("fixture path has a parent")
                    .join(format!("reqvire{}", std::env::consts::EXE_SUFFIX))
            });
        let manager = crate::mcp_worktrees::Worktrees::start(
            &root,
            &executable,
            false,
            false,
            true,
            false,
            "origin",
        )
        .expect("test fixture operation should succeed");
        let created=manager.handle("tools/call",json!({"name":"reqvire.worktree.create","arguments":{"branch":"second","base_ref":"main"}})).expect("test fixture operation should succeed");
        assert_ne!(created["isError"], true, "{created}");
        let id = created["structuredContent"]["worktree_id"]
            .as_str()
            .expect("expected a string in the test response");
        let other_root = std::path::PathBuf::from(
            created["structuredContent"]["workspace_root"]
                .as_str()
                .expect("expected a string in the test response"),
        );
        std::fs::write(other_root.join("shared.txt"), "second bytes").expect("write test fixture");
        let mut state = state(true);
        state.runtime = RuntimeSource::Worktrees(Arc::clone(&manager));
        let app = explorer_routes().with_state(state);
        for (selected, branch, asset) in [
            (manager.original.as_str(), "main", "original bytes"),
            (id, "second", "second bytes"),
        ] {
            let get = |path: String| {
                app.clone().oneshot(
                    Request::builder()
                        .uri(path)
                        .body(Body::empty())
                        .expect("build test HTTP request"),
                )
            };
            let store = get(format!("/api/project-store?worktree_id={selected}"))
                .await
                .unwrap();
            assert_eq!(store.status(), StatusCode::OK);
            let store: Value = serde_json::from_str(&body(store).await)
                .expect("test fixture operation should succeed");
            assert_eq!(store["store"]["project"]["branch"], branch);
            assert_eq!(store["store"]["project"]["worktree_id"], selected);
            assert_eq!(
                body(
                    get(format!("/shared.txt?worktree_id={selected}"))
                        .await
                        .unwrap()
                )
                .await,
                asset
            );
            let manifest = get(format!(
                "/api/project-store/manifest?worktree_id={selected}"
            ))
            .await
            .unwrap();
            assert_eq!(manifest.status(), StatusCode::OK);
            let seed = body(
                get(format!("/assets/project-store.js?worktree_id={selected}"))
                    .await
                    .unwrap(),
            )
            .await;
            assert!(seed.contains(selected));
            assert_eq!(
                get(format!("/ontologies.ttl?worktree_id={selected}"))
                    .await
                    .unwrap()
                    .status(),
                StatusCode::OK
            );
        }
        for path in [
            "/api/project-store",
            "/api/project-store/manifest",
            "/assets/project-store.js",
            "/ontologies.ttl",
            "/shared.txt",
        ] {
            for method in [Method::GET, Method::HEAD] {
                let response = app
                    .clone()
                    .oneshot(
                        Request::builder()
                            .method(method)
                            .uri(format!("{path}?worktree_id=missing"))
                            .body(Body::empty())
                            .expect("build test HTTP request"),
                    )
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE, "{path}");
            }
        }
        let original = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/project-store")
                    .body(Body::empty())
                    .expect("build test HTTP request"),
            )
            .await
            .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&body(original).await)
                .expect("test fixture operation should succeed")["store"]["project"]["branch"],
            "main"
        );
        let inventory = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/worktrees")
                    .body(Body::empty())
                    .expect("build test HTTP request"),
            )
            .await
            .unwrap();
        assert!(body(inventory).await.contains(id));
        let dirty_root = root
            .parent()
            .expect("fixture path has a parent")
            .join("dirty-selection");
        git(&[
            "worktree",
            "add",
            "-qb",
            "dirty-selection",
            dirty_root.to_str().expect("fixture path is UTF-8"),
        ]);
        std::fs::write(dirty_root.join("untracked.txt"), "must not discard")
            .expect("write test fixture");
        let inventory = manager.browser_inventory();
        let target = inventory["worktrees"]
            .as_array()
            .expect("expected an array in the test response")
            .iter()
            .find(|r| r["branch"] == "dirty-selection")
            .expect("expected an array in the test response")["worktree_id"]
            .as_str()
            .expect("expected a string in the test response");
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/worktrees/load")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({"worktree_id":target}).to_string()))
                    .expect("build test HTTP request"),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(body(response).await.contains("clean"));
        assert_eq!(
            std::fs::read_to_string(dirty_root.join("untracked.txt")).expect("read test fixture"),
            "must not discard"
        );
        std::fs::remove_file(dirty_root.join("untracked.txt")).expect("remove test fixture");
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/worktrees/load")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({"worktree_id":target}).to_string()))
                    .expect("build test HTTP request"),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(root.join("shared.txt"), other_root.join("escape.txt"))
                .expect("test fixture operation should succeed");
            let response = app
                .oneshot(
                    Request::builder()
                        .uri(format!("/escape.txt?worktree_id={id}"))
                        .body(Body::empty())
                        .expect("build test HTTP request"),
                )
                .await
                .unwrap();
            assert_ne!(body(response).await, "original bytes");
        }
    }

    #[tokio::test]
    async fn manifest_headers_conditional_variants_and_head_are_consistent() {
        let state = state(true);
        let expected = Arc::clone(state.local_runtime().expect("local snapshot"));
        let app = explorer_routes().with_state(state);
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
    async fn revision_reads_do_not_wait_for_the_worker_control_gate() {
        let (_directory, state, context) = worker_state();
        let (held, acquired) = std::sync::mpsc::channel();
        let (release, released) = std::sync::mpsc::channel();
        let holding = std::thread::spawn(move || {
            let _gate = context.test_control_gate();
            held.send(()).expect("gate held");
            released
                .recv_timeout(std::time::Duration::from_secs(3))
                .expect("release gate");
        });
        acquired
            .recv_timeout(std::time::Duration::from_secs(3))
            .expect("control gate acquired");
        let app = explorer_routes().with_state(state);
        let response = tokio::time::timeout(
            std::time::Duration::from_millis(500),
            app.oneshot(
                Request::builder()
                    .uri("/api/project-store/manifest")
                    .body(Body::empty())
                    .expect("manifest request"),
            ),
        )
        .await;
        release.send(()).expect("release worker gate");
        holding.join().expect("control gate thread");
        assert_eq!(
            response
                .expect("manifest reads published data without the worker gate")
                .expect("HTTP response")
                .status(),
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn read_only_serving_exposes_snapshots_without_live_refresh_advertisement() {
        let state = state(false);
        let app = explorer_routes().with_state(state);
        for (url, method, expected) in [
            ("/api/project-store", Method::GET, StatusCode::OK),
            ("/api/project-store/manifest", Method::GET, StatusCode::OK),
            (
                "/api/project-store/chunks",
                Method::POST,
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ),
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
            assert_eq!(response.status(), expected);
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
        let (_directory, state, context) = worker_state();
        let captured = context.runtime().expect("captured worker runtime");
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
        let manager = state.worktrees().expect("worker-backed routes");
        let content = include_str!("../../../tests/test-cache-integration/fixtures/other.md.txt")
            .split_once("# Elements\n\n")
            .expect("fixture header")
            .1;
        let result = manager.handle("tools/call", json!({"name":"reqvire.add_element","arguments":{"file":"Model.md","content":content}})).expect("production mutation");
        assert_ne!(result["isError"], true, "{result}");
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
            .contains("Cache Subject"));
        let app = explorer_routes().with_state(state);
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
    async fn worker_runtime_diagnostics_preserve_accepted_model_and_recover_at_same_revision() {
        let (_directory, state, context) = worker_state();
        let captured = context.runtime().expect("initial worker runtime");
        let revision = captured.live.revision.clone();
        context
            .publish_test_response(
                json!({"runtime":{"project_store":[],"ontologies_ttl":"invalid"}}),
            )
            .expect("reject malformed runtime publication");
        let app = explorer_routes().with_state(state.clone());
        for uri in ["/api/project-store/manifest", "/assets/project-store.js"] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(uri)
                        .header(header::IF_NONE_MATCH, format!("\"{revision}\""))
                        .body(Body::empty())
                        .expect("runtime request"),
                )
                .await
                .expect("HTTP response");
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert!(body(response)
                .await
                .contains("Explorer store must be an object"));
        }
        let read = state
            .worktrees()
            .expect("worker routes")
            .handle(
                "tools/call",
                json!({"name":"reqvire.search","arguments":{}}),
            )
            .expect("accepted model read");
        assert_ne!(read["isError"], true, "{read}");
        assert!(read.to_string().contains("Cache Subject"));
        assert!(captured.assets.project_store_json.contains("Cache Subject"));
        context
            .test_refresh_runtime()
            .expect("regenerate accepted worker runtime");
        assert_eq!(
            context.runtime().expect("recovered runtime").live.revision,
            revision
        );
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/project-store/manifest")
                    .header(header::IF_NONE_MATCH, format!("\"{revision}\""))
                    .body(Body::empty())
                    .expect("manifest request"),
            )
            .await
            .expect("HTTP response");
        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
    }
}
