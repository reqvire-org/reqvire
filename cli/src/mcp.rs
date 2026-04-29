use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{header, HeaderMap, Response, StatusCode};
use axum::routing::post;
use axum::Router;
use globset::GlobSet;
use reqvire::change_impact;
use reqvire::containment::ContainmentHierarchy;
use reqvire::crud;
use reqvire::diff::render_crud_json;
use reqvire::error::ReqvireError;
use reqvire::format::{format_files, render_diff_json};
use reqvire::git_commands;
use reqvire::lint;
use reqvire::report_collect;
use reqvire::report_coverage;
use reqvire::report_model;
use reqvire::report_resources;
use reqvire::report_submodels;
use reqvire::search;
use reqvire::verification_trace;
use reqvire::ModelManager;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

const MCP_PROTOCOL_VERSION: &str = "2025-11-25";
const TOOL_CONTRACT_VERSION: &str = "1";

#[derive(Debug, Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Deserialize)]
struct ToolCallParams {
    name: String,
    #[serde(default)]
    arguments: Value,
}

#[derive(Clone)]
struct HttpMcpState {
    enable_mutations: bool,
    excluded_filename_patterns: Arc<GlobSet>,
    write_lock: Arc<Mutex<()>>,
}

pub fn serve_stdio(
    enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
) -> Result<(), ReqvireError> {
    validate_startup(excluded_filename_patterns)?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.map_err(ReqvireError::IoError)?;
        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Value>(&line) {
            Ok(raw) => handle_rpc_value(raw, enable_mutations, excluded_filename_patterns),
            Err(err) => Some(rpc_error(
                Value::Null,
                -32700,
                "Parse error",
                Some(json!({ "message": err.to_string() })),
            )),
        };

        if let Some(response) = response {
            writeln!(stdout, "{}", response).map_err(ReqvireError::IoError)?;
            stdout.flush().map_err(ReqvireError::IoError)?;
        }
    }

    Ok(())
}

pub async fn serve_http(
    enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
    host: &str,
    port: u16,
) -> Result<(), ReqvireError> {
    validate_startup(excluded_filename_patterns)?;

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        ReqvireError::ProcessError(format!("Failed to start MCP HTTP server: {}", e))
    })?;

    let state = HttpMcpState {
        enable_mutations,
        excluded_filename_patterns: Arc::new(excluded_filename_patterns.clone()),
        write_lock: Arc::new(Mutex::new(())),
    };

    let app = Router::new()
        .route("/mcp", post(handle_http_rpc).get(http_method_not_allowed))
        .with_state(state);

    eprintln!("MCP HTTP server listening at http://{}/mcp", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .map_err(|e| ReqvireError::ProcessError(format!("MCP HTTP server error: {}", e)))?;

    Ok(())
}

async fn handle_http_rpc(
    State(state): State<HttpMcpState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response<Body> {
    if !origin_is_allowed(&headers) {
        return text_response(
            StatusCode::FORBIDDEN,
            "Forbidden: non-loopback Origin is not allowed",
        );
    }

    let raw = match serde_json::from_slice::<Value>(&body) {
        Ok(raw) => raw,
        Err(err) => {
            return json_response(
                StatusCode::OK,
                rpc_error(
                    Value::Null,
                    -32700,
                    "Parse error",
                    Some(json!({ "message": err.to_string() })),
                ),
            );
        }
    };

    let response = if request_requires_write(&raw) {
        let _guard = state.write_lock.lock().await;
        handle_rpc_value(
            raw,
            state.enable_mutations,
            state.excluded_filename_patterns.as_ref(),
        )
    } else {
        handle_rpc_value(
            raw,
            state.enable_mutations,
            state.excluded_filename_patterns.as_ref(),
        )
    };

    match response {
        Some(value) => json_response(StatusCode::OK, value),
        None => text_response(StatusCode::ACCEPTED, ""),
    }
}

async fn http_method_not_allowed() -> Response<Body> {
    text_response(StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
}

fn request_requires_write(raw: &Value) -> bool {
    if raw.get("method").and_then(Value::as_str) != Some("tools/call") {
        return false;
    }

    let params = raw.get("params").unwrap_or(&Value::Null);
    let tool_name = match params.get("name").and_then(Value::as_str) {
        Some(name) => name,
        None => return false,
    };

    is_mutation_tool(tool_name)
        || (tool_name == "reqvire.format"
            && params
                .get("arguments")
                .and_then(|args| args.get("fix"))
                .and_then(Value::as_bool)
                .unwrap_or(false))
}

fn origin_is_allowed(headers: &HeaderMap) -> bool {
    let Some(origin) = headers.get(header::ORIGIN) else {
        return true;
    };

    let Ok(origin) = origin.to_str() else {
        return false;
    };

    loopback_origin(origin)
}

fn loopback_origin(origin: &str) -> bool {
    let lower = origin.to_ascii_lowercase();
    if lower == "null" || lower.starts_with("file:") {
        return false;
    }

    let Some(authority_and_path) = lower
        .strip_prefix("http://")
        .or_else(|| lower.strip_prefix("https://"))
    else {
        return false;
    };

    let authority = authority_and_path
        .split('/')
        .next()
        .unwrap_or(authority_and_path);
    let host = if authority.starts_with('[') {
        authority
            .split(']')
            .next()
            .map(|value| format!("{}]", value))
            .unwrap_or_default()
    } else {
        authority.split(':').next().unwrap_or(authority).to_string()
    };

    host == "localhost" || host.starts_with("127.") || host == "[::1]" || host == "::1"
}

fn json_response(status: StatusCode, value: Value) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(value.to_string()))
        .unwrap_or_else(|_| {
            text_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        })
}

fn text_response(status: StatusCode, text: &str) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(text.to_string()))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn handle_rpc_value(
    raw: Value,
    enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
) -> Option<Value> {
    let id = raw.get("id").cloned().unwrap_or(Value::Null);
    let request = match serde_json::from_value::<RpcRequest>(raw) {
        Ok(request) => request,
        Err(err) => {
            return Some(rpc_error(
                id,
                -32600,
                "Invalid Request",
                Some(json!({ "message": err.to_string() })),
            ));
        }
    };

    if request.id.is_none() {
        return None;
    }

    let id = request.id.unwrap_or(Value::Null);
    match request.method.as_str() {
        "initialize" => initialize(id, request.params),
        "tools/list" => Some(rpc_result(
            id,
            json!({ "tools": tool_definitions(enable_mutations) }),
        )),
        "tools/call" => Some(handle_tool_call(
            id,
            request.params,
            enable_mutations,
            excluded_filename_patterns,
        )),
        "resources/list" => Some(rpc_result(
            id,
            json!({ "resources": resource_definitions() }),
        )),
        "resources/read" => Some(handle_resource_read(
            id,
            request.params,
            enable_mutations,
            excluded_filename_patterns,
        )),
        "resources/templates/list" => Some(rpc_result(id, json!({ "resourceTemplates": [] }))),
        _ => Some(rpc_error(
            id,
            -32601,
            "Method not found",
            Some(json!({ "method": request.method })),
        )),
    }
}

fn initialize(id: Value, params: Value) -> Option<Value> {
    if let Some(client_version) = params.get("protocolVersion").and_then(Value::as_str) {
        if client_version != MCP_PROTOCOL_VERSION {
            return Some(rpc_error(
                id,
                -32000,
                "Unsupported MCP protocol version",
                Some(json!({
                    "expected": MCP_PROTOCOL_VERSION,
                    "received": client_version
                })),
            ));
        }
    }

    Some(rpc_result(
        id,
        json!({
            "protocolVersion": MCP_PROTOCOL_VERSION,
            "capabilities": {
                "tools": {},
                "resources": {}
            },
            "serverInfo": {
                "name": "reqvire",
                "version": env!("CARGO_PKG_VERSION")
            }
        }),
    ))
}

fn handle_tool_call(
    id: Value,
    params: Value,
    enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
) -> Value {
    let params = match serde_json::from_value::<ToolCallParams>(params) {
        Ok(params) => params,
        Err(err) => {
            return rpc_error(
                id,
                -32602,
                "Invalid params",
                Some(json!({ "message": err.to_string() })),
            );
        }
    };

    if is_mutation_tool(&params.name) && !enable_mutations {
        return rpc_error(
            id,
            -32602,
            "Tool not found",
            Some(json!({
                "tool": params.name,
                "reason": "mutation tools are not advertised unless --enable-mutations is set"
            })),
        );
    }

    if !tool_exists(&params.name, enable_mutations) {
        return rpc_error(
            id,
            -32602,
            "Tool not found",
            Some(json!({ "tool": params.name })),
        );
    }

    if let Err(message) = validate_tool_arguments(&params.name, &params.arguments, enable_mutations)
    {
        return rpc_error(
            id,
            -32602,
            "Invalid tool arguments",
            Some(json!({ "tool": params.name, "message": message })),
        );
    }

    let outcome = dispatch_tool(
        &params.name,
        &params.arguments,
        enable_mutations,
        excluded_filename_patterns,
    );

    match outcome {
        Ok(value) => rpc_result(id, tool_success(value)),
        Err(err) => rpc_result(id, tool_error(&params.name, err)),
    }
}

fn handle_resource_read(
    id: Value,
    params: Value,
    enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
) -> Value {
    let uri = match params.get("uri").and_then(Value::as_str) {
        Some(uri) => uri,
        None => {
            return rpc_error(
                id,
                -32602,
                "Invalid params",
                Some(json!({ "message": "resources/read requires string field 'uri'" })),
            );
        }
    };

    let value = match uri {
        "reqvire://workspace/status" => {
            workspace_status(excluded_filename_patterns).map(|v| resource_contents(uri, v))
        }
        "reqvire://workspace/model-revision" => {
            model_revision(excluded_filename_patterns).map(|v| resource_contents(uri, v))
        }
        "reqvire://tools/contract" => Ok(resource_contents(
            uri,
            json!({
                "mcp_protocol_version": MCP_PROTOCOL_VERSION,
                "tool_contract_version": TOOL_CONTRACT_VERSION,
                "mutation_tools_enabled": enable_mutations,
                "tools": tool_definitions(enable_mutations)
            }),
        )),
        _ => Err(ReqvireError::ProcessError(format!(
            "Unknown MCP resource URI '{}'",
            uri
        ))),
    };

    match value {
        Ok(value) => rpc_result(id, value),
        Err(err) => rpc_result(id, tool_error("resources/read", err)),
    }
}

fn dispatch_tool(
    name: &str,
    args: &Value,
    enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    match name {
        "reqvire.workspace_status" => workspace_status(excluded_filename_patterns),
        "reqvire.tool_contract" => Ok(json!({
            "mcp_protocol_version": MCP_PROTOCOL_VERSION,
            "tool_contract_version": TOOL_CONTRACT_VERSION,
            "mutation_tools_enabled": enable_mutations,
            "tools": tool_definitions(enable_mutations)
        })),
        "reqvire.model_revision" => model_revision(excluded_filename_patterns),
        "reqvire.read_element" => read_element(args, excluded_filename_patterns),
        "reqvire.search" => search_tool(args, excluded_filename_patterns),
        "reqvire.model" => model_tool(args, excluded_filename_patterns),
        "reqvire.containment" => containment_tool(args, excluded_filename_patterns),
        "reqvire.collect" => collect_tool(args, excluded_filename_patterns),
        "reqvire.submodels" => submodels_tool(args, excluded_filename_patterns),
        "reqvire.lint" => lint_tool(args, excluded_filename_patterns),
        "reqvire.coverage" => coverage_tool(excluded_filename_patterns),
        "reqvire.traces" => traces_tool(args, excluded_filename_patterns),
        "reqvire.resources" => resources_tool(excluded_filename_patterns),
        "reqvire.change_impact" => change_impact_tool(args, excluded_filename_patterns),
        "reqvire.format" => format_tool(args, enable_mutations, excluded_filename_patterns),
        "reqvire.add_element" => add_element_tool(args, excluded_filename_patterns),
        "reqvire.remove_element" => remove_element_tool(args, excluded_filename_patterns),
        "reqvire.move_element" => move_element_tool(args, excluded_filename_patterns),
        "reqvire.rename_element" => rename_element_tool(args, excluded_filename_patterns),
        "reqvire.merge_elements" => merge_elements_tool(args, excluded_filename_patterns),
        "reqvire.move_file" => move_file_tool(args, excluded_filename_patterns),
        "reqvire.link" => link_tool(args, excluded_filename_patterns),
        "reqvire.unlink" => unlink_tool(args, excluded_filename_patterns),
        "reqvire.relink" => relink_tool(args, excluded_filename_patterns),
        "reqvire.move_asset" => move_asset_tool(args, excluded_filename_patterns),
        "reqvire.remove_asset" => remove_asset_tool(args, excluded_filename_patterns),
        _ => Err(ReqvireError::ProcessError(format!(
            "Unknown MCP tool '{}'",
            name
        ))),
    }
}

fn workspace_status(excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let mut files = BTreeSet::new();
    for element in model.graph_registry.get_all_elements() {
        files.insert(element.file_path.clone());
    }
    for file in model.graph_registry.pages.keys() {
        files.insert(file.clone());
    }

    Ok(json!({
        "workspace_root": current_dir_string(),
        "git": git_state(),
        "reqvire_version": env!("CARGO_PKG_VERSION"),
        "mcp_protocol_version": MCP_PROTOCOL_VERSION,
        "tool_contract_version": TOOL_CONTRACT_VERSION,
        "model": {
            "valid": true,
            "fingerprint": model_fingerprint(&model),
            "element_count": model.graph_registry.nodes.len(),
            "file_count": files.len()
        }
    }))
}

fn model_revision(excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    Ok(json!({
        "workspace_root": current_dir_string(),
        "git": git_state(),
        "reqvire_version": env!("CARGO_PKG_VERSION"),
        "mcp_protocol_version": MCP_PROTOCOL_VERSION,
        "tool_contract_version": TOOL_CONTRACT_VERSION,
        "model_fingerprint": model_fingerprint(&model)
    }))
}

fn read_element(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let identifier = string_arg(args, "identifier");
    let name = string_arg(args, "name");
    if identifier.is_none() && name.is_none() {
        return Err(ReqvireError::ProcessError(
            "read_element requires 'identifier' or 'name'".to_string(),
        ));
    }

    let model = load_model(excluded_filename_patterns)?;
    let element = if let Some(identifier) = identifier {
        model.graph_registry.get_element(&identifier)
    } else {
        model.graph_registry.get_element_by_name(&name.unwrap())
    }
    .ok_or_else(|| ReqvireError::ElementNotFound("Element not found".to_string()))?;

    Ok(serde_json::to_value(element)
        .map_err(|e| ReqvireError::SerializationError(e.to_string()))?)
}

fn search_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let filters = search::SearchFilters::new(
        string_arg(args, "filter_file").as_deref(),
        string_arg(args, "filter_name").as_deref(),
        string_arg(args, "filter_type").as_deref(),
        string_arg(args, "filter_content").as_deref(),
        string_arg(args, "filter_page_content").as_deref(),
        string_arg(args, "have_relations").as_deref(),
        string_arg(args, "not_have_relations").as_deref(),
        bool_arg(args, "has_attachments", false),
        string_arg(args, "filter_attachment").as_deref(),
    )?;
    parse_json_string(search::generate_search_report(
        &model.graph_registry,
        &filters,
        true,
        bool_arg(args, "short", false),
    )?)
}

fn model_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let filter_type = string_arg(args, "filter_type");
    let type_filter: Option<Vec<&str>> = filter_type
        .as_deref()
        .map(|s| s.split(',').map(|t| t.trim()).collect());
    parse_json_string(report_model::generate_model_report(
        &model.graph_registry,
        string_arg(args, "from").as_deref(),
        bool_arg(args, "reverse", false),
        type_filter,
        true,
        "LR",
    )?)
}

fn containment_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let hierarchy =
        ContainmentHierarchy::build(&model.graph_registry, bool_arg(args, "short", false))?;
    Ok(serde_json::to_value(hierarchy)
        .map_err(|e| ReqvireError::SerializationError(e.to_string()))?)
}

fn collect_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let direction = match string_arg(args, "direction")
        .unwrap_or_else(|| "UPSTREAM".to_string())
        .to_uppercase()
        .as_str()
    {
        "UPSTREAM" => report_collect::CollectDirection::Upstream,
        "DOWNSTREAM" => report_collect::CollectDirection::Downstream,
        other => {
            return Err(ReqvireError::ProcessError(format!(
                "Invalid direction '{}'. Valid values: UPSTREAM, DOWNSTREAM",
                other
            )));
        }
    };
    let git_root = git_commands::get_git_root_dir()?;
    parse_json_string(report_collect::generate_collect_report(
        &model.graph_registry,
        &required_string_arg(args, "element_name")?,
        &git_root,
        true,
        direction,
    )?)
}

fn submodels_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let report = report_submodels::generate_submodels_report(
        &model.graph_registry,
        string_arg(args, "from").as_deref(),
    )?;
    parse_json_string(report.to_json_string())
}

fn lint_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model_lenient(excluded_filename_patterns)?;
    let report = lint::analyze_model(&model.graph_registry);
    parse_json_string(report.to_json_string(
        bool_arg(args, "fixable", false),
        bool_arg(args, "auditable", false),
    ))
}

fn coverage_tool(excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let report = report_coverage::generate_coverage_report(&model.graph_registry);
    parse_json_string(report.to_json_string())
}

fn traces_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let generator = verification_trace::VerificationTraceGenerator::new(
        &model.graph_registry,
        bool_arg(args, "links_with_blobs", false),
        string_arg(args, "from_folder"),
    );
    let mut report = generator.generate();
    if args.get("filter_id").is_some()
        || args.get("filter_name").is_some()
        || args.get("filter_type").is_some()
    {
        report = verification_trace::apply_filters(
            report,
            string_arg(args, "filter_id").as_deref(),
            string_arg(args, "filter_name").as_deref(),
            string_arg(args, "filter_type").as_deref(),
        )?;
    }
    Ok(
        serde_json::to_value(report)
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))?,
    )
}

fn resources_tool(excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let model = load_model(excluded_filename_patterns)?;
    let report = report_resources::generate_resources_report(&model.graph_registry);
    parse_json_string(report.to_json_string())
}

fn change_impact_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let git_commit = string_arg(args, "git_commit").unwrap_or_else(|| "HEAD".to_string());
    let model = load_model(excluded_filename_patterns)?;
    let mut reference_model = ModelManager::new();
    match reference_model.parse_and_validate_with_mode(
        Some(&git_commit),
        excluded_filename_patterns,
        false,
    ) {
        Ok(_) => {}
        Err(ReqvireError::ValidationError(_)) => {
            reference_model.parse_and_validate_with_mode(
                Some(&git_commit),
                excluded_filename_patterns,
                true,
            )?;
        }
        Err(e) => return Err(e),
    }

    let base_url = git_commands::get_repository_base_url()?;
    let current_commit = git_commands::get_commit_hash()?;
    let report = change_impact::compute_change_impact(
        &model.graph_registry,
        &reference_model.graph_registry,
    )
    .map_err(|e| ReqvireError::ProcessError(format!("{:?}", e)))?;
    parse_json_string(report.to_json_string(&base_url, &current_commit, &git_commit))
}

fn format_tool(
    args: &Value,
    enable_mutations: bool,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let fix = bool_arg(args, "fix", false);
    if fix && !enable_mutations {
        return Err(ReqvireError::ProcessError(
            "format with fix=true requires --enable-mutations".to_string(),
        ));
    }
    let model = load_model(excluded_filename_patterns)?;
    let result = format_files(
        &model.graph_registry,
        !fix,
        bool_arg(args, "with_full_relations", false),
    )?;
    parse_json_string(render_diff_json(&result))
}

fn add_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::add_element(
        &mut model,
        &required_string_arg(args, "content")?,
        &required_string_arg(args, "file")?,
        excluded_filename_patterns,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
        bool_arg(args, "override_existing", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn remove_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::remove_element(
        &mut model,
        &element_id,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn move_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::move_element(
        &mut model,
        &element_id,
        &required_string_arg(args, "file")?,
        excluded_filename_patterns,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn rename_element_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let element_id = model
        .graph_registry
        .find_element_by_name(&required_string_arg(args, "element_name")?)?;
    let result = crud::rename_element(
        &mut model,
        &element_id,
        &required_string_arg(args, "new_name")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn merge_elements_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::merge_elements(
        &mut model,
        &required_string_arg(args, "target")?,
        &string_array_arg(args, "sources")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn move_file_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::move_file(
        &mut model,
        &required_string_arg(args, "source_file")?,
        &required_string_arg(args, "target_file")?,
        &current_dir_path(),
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
        bool_arg(args, "squash", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn link_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let source = required_string_arg(args, "source")?;
    let relation_type = required_string_arg(args, "relation_type")?;
    let target = required_string_arg(args, "target")?;
    let git_root = git_commands::get_git_root_dir()?;
    let result = if relation_type == "attaching" {
        if reqvire::utils::is_external_url(&target) {
            return Err(ReqvireError::ProcessError(
                "External URLs cannot be attached. Use a relation type such as trace instead."
                    .to_string(),
            ));
        }
        crud::attach_element_identifier(
            &mut model,
            &source,
            &target,
            &git_root,
            bool_arg(args, "dry_run", false),
        )?
    } else {
        crud::link(
            &mut model,
            &source,
            &relation_type,
            &target,
            &git_root,
            bool_arg(args, "dry_run", false),
        )?
    };
    parse_json_string(render_crud_json(&result))
}

fn unlink_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::unlink(
        &mut model,
        &required_string_arg(args, "source")?,
        &required_string_arg(args, "target")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn relink_tool(args: &Value, excluded_filename_patterns: &GlobSet) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::relink(
        &mut model,
        &required_string_arg(args, "source")?,
        &required_string_arg(args, "relation_type")?,
        &required_string_arg(args, "from_target")?,
        &required_string_arg(args, "to_target")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn move_asset_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::mv_asset(
        &mut model,
        &required_string_arg(args, "old_path")?,
        &required_string_arg(args, "new_path")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn remove_asset_tool(
    args: &Value,
    excluded_filename_patterns: &GlobSet,
) -> Result<Value, ReqvireError> {
    let mut model = load_model(excluded_filename_patterns)?;
    let result = crud::rm_asset(
        &mut model,
        &required_string_arg(args, "file_path")?,
        &git_commands::get_git_root_dir()?,
        bool_arg(args, "dry_run", false),
    )?;
    parse_json_string(render_crud_json(&result))
}

fn load_model(excluded_filename_patterns: &GlobSet) -> Result<ModelManager, ReqvireError> {
    let mut model = ModelManager::new();
    model.parse_and_validate(None, excluded_filename_patterns)?;
    Ok(model)
}

fn validate_startup(excluded_filename_patterns: &GlobSet) -> Result<(), ReqvireError> {
    load_model(excluded_filename_patterns)
        .map(|_| ())
        .map_err(|err| match err {
            ReqvireError::ValidationError(errors) => {
                let mut messages: Vec<String> = errors.iter().map(ToString::to_string).collect();
                messages.sort();
                ReqvireError::ProcessError(format!(
                    "MCP startup validation failed for Reqvire {} using MCP protocol {}:\n{}",
                    env!("CARGO_PKG_VERSION"),
                    MCP_PROTOCOL_VERSION,
                    messages
                        .iter()
                        .enumerate()
                        .map(|(idx, msg)| format!("{}. {}", idx + 1, msg))
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
            }
            other => ReqvireError::ProcessError(format!(
                "MCP startup failed for Reqvire {} using MCP protocol {}: {}",
                env!("CARGO_PKG_VERSION"),
                MCP_PROTOCOL_VERSION,
                other
            )),
        })
}

fn load_model_lenient(excluded_filename_patterns: &GlobSet) -> Result<ModelManager, ReqvireError> {
    let mut model = ModelManager::new();
    model.parse_and_validate_with_mode(None, excluded_filename_patterns, true)?;
    Ok(model)
}

fn parse_json_string(json_str: String) -> Result<Value, ReqvireError> {
    serde_json::from_str(&json_str).map_err(|e| ReqvireError::SerializationError(e.to_string()))
}

fn required_string_arg(args: &Value, name: &str) -> Result<String, ReqvireError> {
    string_arg(args, name).ok_or_else(|| {
        ReqvireError::ProcessError(format!("Missing required string argument '{}'", name))
    })
}

fn string_arg(args: &Value, name: &str) -> Option<String> {
    args.get(name)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn bool_arg(args: &Value, name: &str, default: bool) -> bool {
    args.get(name).and_then(Value::as_bool).unwrap_or(default)
}

fn string_array_arg(args: &Value, name: &str) -> Result<Vec<String>, ReqvireError> {
    let values = args.get(name).and_then(Value::as_array).ok_or_else(|| {
        ReqvireError::ProcessError(format!("Missing required string array argument '{}'", name))
    })?;
    values
        .iter()
        .map(|value| {
            value.as_str().map(ToString::to_string).ok_or_else(|| {
                ReqvireError::ProcessError(format!("Argument '{}' must contain only strings", name))
            })
        })
        .collect()
}

fn tool_success(value: Value) -> Value {
    let text = serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string());
    json!({
        "content": [{ "type": "text", "text": text }],
        "structuredContent": value
    })
}

fn tool_error(tool_name: &str, err: ReqvireError) -> Value {
    let error = reqvire_error(tool_name, err);
    let text = error
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("Reqvire tool execution failed")
        .to_string();
    json!({
        "isError": true,
        "content": [{ "type": "text", "text": text }],
        "structuredContent": { "error": error }
    })
}

fn reqvire_error(tool_name: &str, err: ReqvireError) -> Value {
    let (code, related_errors) = match &err {
        ReqvireError::ValidationError(errors) => (
            "validation_failed",
            Some(errors.iter().map(ToString::to_string).collect::<Vec<_>>()),
        ),
        ReqvireError::DuplicateElement(_) => ("duplicate_element", None),
        ReqvireError::ElementNotFound(_) | ReqvireError::MissingElement(_) => {
            ("element_not_found", None)
        }
        ReqvireError::UnsupportedRelationType(_) => ("invalid_relation_type", None),
        ReqvireError::IncompatibleElementTypes(_) => ("invalid_element_type_for_relation", None),
        ReqvireError::InvalidAttachmentScope(_) | ReqvireError::InvalidAttachmentTarget(_) => {
            ("attachment_contract_violation", None)
        }
        ReqvireError::InvalidOperation(message)
            if message.contains("Single-root hierarchy ownership violation") =>
        {
            ("single_root_ownership_violation", None)
        }
        ReqvireError::IoError(_) => ("filesystem_error", None),
        ReqvireError::GitCommandError(_) => ("git_error", None),
        ReqvireError::SerializationError(_) => ("serialization_error", None),
        _ => ("reqvire_error", None),
    };

    json!({
        "code": code,
        "message": err.to_string(),
        "tool": tool_name,
        "recoverability": recoverability_hint(code),
        "related_errors": related_errors
    })
}

fn recoverability_hint(code: &str) -> &'static str {
    match code {
        "element_not_found" => "Check element name or identifier and retry.",
        "validation_failed" => "Run reqvire validate and resolve model errors before retrying.",
        "duplicate_element" => "Rename or remove duplicate elements before retrying.",
        "attachment_contract_violation" => {
            "Use a valid refinement attachment that respects submodel attachment contracts."
        }
        "single_root_ownership_violation" => {
            "Adjust hierarchy so each requirement branch has a single root owner."
        }
        _ => "Inspect the message and retry with corrected arguments.",
    }
}

fn rpc_result(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    })
}

fn rpc_error(id: Value, code: i64, message: &str, data: Option<Value>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message,
            "data": data
        }
    })
}

fn resource_contents(uri: &str, value: Value) -> Value {
    let text = serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string());
    json!({
        "contents": [{
            "uri": uri,
            "mimeType": "application/json",
            "text": text
        }]
    })
}

fn tool_exists(name: &str, enable_mutations: bool) -> bool {
    read_tool_names().contains(&name)
        || conditional_tool_names().contains(&name)
        || (enable_mutations && mutation_tool_names().contains(&name))
}

fn is_mutation_tool(name: &str) -> bool {
    mutation_tool_names().contains(&name)
}

fn read_tool_names() -> Vec<&'static str> {
    vec![
        "reqvire.workspace_status",
        "reqvire.tool_contract",
        "reqvire.model_revision",
        "reqvire.read_element",
        "reqvire.search",
        "reqvire.model",
        "reqvire.containment",
        "reqvire.collect",
        "reqvire.submodels",
        "reqvire.lint",
        "reqvire.coverage",
        "reqvire.traces",
        "reqvire.resources",
        "reqvire.change_impact",
    ]
}

fn conditional_tool_names() -> Vec<&'static str> {
    vec!["reqvire.format"]
}

fn mutation_tool_names() -> Vec<&'static str> {
    vec![
        "reqvire.add_element",
        "reqvire.remove_element",
        "reqvire.move_element",
        "reqvire.rename_element",
        "reqvire.merge_elements",
        "reqvire.move_file",
        "reqvire.link",
        "reqvire.unlink",
        "reqvire.relink",
        "reqvire.move_asset",
        "reqvire.remove_asset",
    ]
}

fn tool_definitions(enable_mutations: bool) -> Vec<Value> {
    let mut tools = vec![
        read_tool(
            "reqvire.workspace_status",
            "Report workspace, git, and model status.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.tool_contract",
            "Return the Reqvire MCP tool contract.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.model_revision",
            "Report the current workspace and model revision.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.read_element",
            "Read one authoritative model element by identifier or name.",
            object_schema(vec![
                ("identifier", json!({ "type": "string" })),
                ("name", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.search",
            "Search and filter model elements.",
            object_schema(vec![
                ("short", json!({ "type": "boolean" })),
                ("filter_file", json!({ "type": "string" })),
                ("filter_name", json!({ "type": "string" })),
                ("filter_type", json!({ "type": "string" })),
                ("filter_content", json!({ "type": "string" })),
                ("filter_page_content", json!({ "type": "string" })),
                ("have_relations", json!({ "type": "string" })),
                ("not_have_relations", json!({ "type": "string" })),
                ("has_attachments", json!({ "type": "boolean" })),
                ("filter_attachment", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.model",
            "Generate model-centric structure.",
            object_schema(vec![
                ("from", json!({ "type": "string" })),
                ("reverse", json!({ "type": "boolean" })),
                ("filter_type", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.containment",
            "Generate folder/file/element containment hierarchy.",
            object_schema(vec![("short", json!({ "type": "boolean" }))]),
        ),
        read_tool(
            "reqvire.collect",
            "Collect requirement context upstream or downstream.",
            required_object_schema(
                vec![
                    ("element_name", json!({ "type": "string" })),
                    (
                        "direction",
                        json!({ "type": "string", "enum": ["UPSTREAM", "DOWNSTREAM"] }),
                    ),
                ],
                vec!["element_name"],
            ),
        ),
        read_tool(
            "reqvire.submodels",
            "Analyze independent requirement submodels.",
            object_schema(vec![("from", json!({ "type": "string" }))]),
        ),
        read_tool(
            "reqvire.lint",
            "Analyze model quality without applying fixes.",
            object_schema(vec![
                ("fixable", json!({ "type": "boolean" })),
                ("auditable", json!({ "type": "boolean" })),
            ]),
        ),
        read_tool(
            "reqvire.coverage",
            "Generate verification and implementation coverage.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.traces",
            "Generate verification traces.",
            object_schema(vec![
                ("from_folder", json!({ "type": "string" })),
                ("links_with_blobs", json!({ "type": "boolean" })),
                ("filter_id", json!({ "type": "string" })),
                ("filter_name", json!({ "type": "string" })),
                ("filter_type", json!({ "type": "string" })),
            ]),
        ),
        read_tool(
            "reqvire.resources",
            "Report files referenced by the model.",
            object_schema(vec![]),
        ),
        read_tool(
            "reqvire.change_impact",
            "Analyze change impact against a git commit.",
            object_schema(vec![(
                "git_commit",
                json!({ "type": "string", "default": "HEAD" }),
            )]),
        ),
    ];

    if enable_mutations {
        tools.push(conditional_tool(
            "reqvire.format",
            "Preview formatting, or apply formatting when mutation mode is enabled and fix=true.",
            object_schema(vec![
                ("fix", json!({ "type": "boolean", "default": false })),
                (
                    "with_full_relations",
                    json!({ "type": "boolean", "default": false }),
                ),
            ]),
        ));
    } else {
        tools.push(read_tool(
            "reqvire.format",
            "Preview formatting without applying changes.",
            object_schema(vec![
                (
                    "fix",
                    json!({ "type": "boolean", "enum": [false], "default": false }),
                ),
                (
                    "with_full_relations",
                    json!({ "type": "boolean", "default": false }),
                ),
            ]),
        ));
    }

    if enable_mutations {
        tools.extend(vec![
            mutation_tool(
                "reqvire.add_element",
                "Add a new element from Markdown content.",
                required_object_schema(
                    vec![
                        ("file", json!({ "type": "string" })),
                        ("content", json!({ "type": "string" })),
                        (
                            "override_existing",
                            json!({ "type": "boolean", "default": false }),
                        ),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["file", "content"],
                ),
            ),
            mutation_tool(
                "reqvire.remove_element",
                "Remove an element.",
                required_object_schema(
                    vec![
                        ("element_name", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["element_name"],
                ),
            ),
            mutation_tool(
                "reqvire.move_element",
                "Move an element to another file.",
                required_object_schema(
                    vec![
                        ("element_name", json!({ "type": "string" })),
                        ("file", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["element_name", "file"],
                ),
            ),
            mutation_tool(
                "reqvire.rename_element",
                "Rename an element.",
                required_object_schema(
                    vec![
                        ("element_name", json!({ "type": "string" })),
                        ("new_name", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["element_name", "new_name"],
                ),
            ),
            mutation_tool(
                "reqvire.merge_elements",
                "Merge source elements into a target element.",
                required_object_schema(
                    vec![
                        ("target", json!({ "type": "string" })),
                        (
                            "sources",
                            json!({ "type": "array", "items": { "type": "string" } }),
                        ),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["target", "sources"],
                ),
            ),
            mutation_tool(
                "reqvire.move_file",
                "Move a model file and its elements.",
                required_object_schema(
                    vec![
                        ("source_file", json!({ "type": "string" })),
                        ("target_file", json!({ "type": "string" })),
                        ("squash", json!({ "type": "boolean", "default": false })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source_file", "target_file"],
                ),
            ),
            mutation_tool(
                "reqvire.link",
                "Add a relation or attachment.",
                required_object_schema(
                    vec![
                        ("source", json!({ "type": "string" })),
                        ("relation_type", json!({ "type": "string" })),
                        ("target", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source", "relation_type", "target"],
                ),
            ),
            mutation_tool(
                "reqvire.unlink",
                "Remove a relation or attachment.",
                required_object_schema(
                    vec![
                        ("source", json!({ "type": "string" })),
                        ("target", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source", "target"],
                ),
            ),
            mutation_tool(
                "reqvire.relink",
                "Replace an existing relation target.",
                required_object_schema(
                    vec![
                        ("source", json!({ "type": "string" })),
                        ("relation_type", json!({ "type": "string" })),
                        ("from_target", json!({ "type": "string" })),
                        ("to_target", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["source", "relation_type", "from_target", "to_target"],
                ),
            ),
            mutation_tool(
                "reqvire.move_asset",
                "Move an asset and update references.",
                required_object_schema(
                    vec![
                        ("old_path", json!({ "type": "string" })),
                        ("new_path", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["old_path", "new_path"],
                ),
            ),
            mutation_tool(
                "reqvire.remove_asset",
                "Remove an asset and update references.",
                required_object_schema(
                    vec![
                        ("file_path", json!({ "type": "string" })),
                        ("dry_run", json!({ "type": "boolean", "default": false })),
                    ],
                    vec!["file_path"],
                ),
            ),
        ]);
    }

    tools
}

fn read_tool(name: &str, description: &str, input_schema: Value) -> Value {
    tool(name, description, input_schema, true, false)
}

fn conditional_tool(name: &str, description: &str, input_schema: Value) -> Value {
    tool(name, description, input_schema, false, false)
}

fn mutation_tool(name: &str, description: &str, input_schema: Value) -> Value {
    tool(name, description, input_schema, false, true)
}

fn tool(
    name: &str,
    description: &str,
    input_schema: Value,
    read_only: bool,
    destructive: bool,
) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
        "outputSchema": generic_output_schema(),
        "annotations": {
            "title": name,
            "readOnlyHint": read_only,
            "destructiveHint": destructive,
            "openWorldHint": false
        }
    })
}

fn object_schema(properties: Vec<(&str, Value)>) -> Value {
    required_object_schema(properties, Vec::new())
}

fn required_object_schema(properties: Vec<(&str, Value)>, required: Vec<&str>) -> Value {
    let mut map = serde_json::Map::new();
    for (name, schema) in properties {
        map.insert(name.to_string(), schema);
    }
    json!({
        "type": "object",
        "properties": map,
        "required": required,
        "additionalProperties": false
    })
}

fn generic_output_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": true
    })
}

fn resource_definitions() -> Vec<Value> {
    vec![
        json!({
            "uri": "reqvire://workspace/status",
            "name": "Reqvire workspace status",
            "mimeType": "application/json",
            "description": "Workspace, git, and model status."
        }),
        json!({
            "uri": "reqvire://workspace/model-revision",
            "name": "Reqvire model revision",
            "mimeType": "application/json",
            "description": "Current workspace revision metadata."
        }),
        json!({
            "uri": "reqvire://tools/contract",
            "name": "Reqvire MCP tool contract",
            "mimeType": "application/json",
            "description": "Tool definitions and Reqvire MCP contract metadata."
        }),
    ]
}

fn validate_tool_arguments(
    tool_name: &str,
    arguments: &Value,
    enable_mutations: bool,
) -> Result<(), String> {
    let tools = tool_definitions(enable_mutations);
    let tool = tools
        .iter()
        .find(|tool| tool.get("name").and_then(Value::as_str) == Some(tool_name))
        .ok_or_else(|| format!("Unknown tool '{}'", tool_name))?;
    let schema = tool
        .get("inputSchema")
        .ok_or_else(|| format!("Tool '{}' has no inputSchema", tool_name))?;
    validate_object_schema(arguments, schema)
}

fn validate_object_schema(arguments: &Value, schema: &Value) -> Result<(), String> {
    let object = arguments
        .as_object()
        .ok_or_else(|| "Tool arguments must be a JSON object".to_string())?;
    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| "inputSchema.properties must be an object".to_string())?;

    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        for item in required {
            let name = item
                .as_str()
                .ok_or_else(|| "inputSchema.required must contain strings".to_string())?;
            if !object.contains_key(name) {
                return Err(format!("Missing required argument '{}'", name));
            }
        }
    }

    if schema
        .get("additionalProperties")
        .and_then(Value::as_bool)
        .is_some_and(|allowed| !allowed)
    {
        for name in object.keys() {
            if !properties.contains_key(name) {
                return Err(format!("Unknown argument '{}'", name));
            }
        }
    }

    for (name, value) in object {
        if let Some(property_schema) = properties.get(name) {
            validate_property_type(name, value, property_schema)?;
            validate_property_enum(name, value, property_schema)?;
        }
    }

    Ok(())
}

fn validate_property_type(name: &str, value: &Value, schema: &Value) -> Result<(), String> {
    match schema.get("type").and_then(Value::as_str) {
        Some("string") if !value.is_string() => {
            Err(format!("Argument '{}' must be a string", name))
        }
        Some("boolean") if !value.is_boolean() => {
            Err(format!("Argument '{}' must be a boolean", name))
        }
        Some("array") => {
            let values = value
                .as_array()
                .ok_or_else(|| format!("Argument '{}' must be an array", name))?;
            if schema
                .get("items")
                .and_then(|items| items.get("type"))
                .and_then(Value::as_str)
                == Some("string")
                && values.iter().any(|item| !item.is_string())
            {
                return Err(format!("Argument '{}' must contain only strings", name));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_property_enum(name: &str, value: &Value, schema: &Value) -> Result<(), String> {
    if let Some(allowed) = schema.get("enum").and_then(Value::as_array) {
        if !allowed.iter().any(|allowed_value| allowed_value == value) {
            return Err(format!(
                "Argument '{}' has unsupported value '{}'",
                name, value
            ));
        }
    }
    Ok(())
}

fn git_state() -> Value {
    let head = git_output(["rev-parse", "HEAD"]);
    let status = git_output(["status", "--porcelain"]);
    json!({
        "head": head,
        "dirty": status.as_ref().is_some_and(|s| !s.trim().is_empty())
    })
}

fn git_output<const N: usize>(args: [&str; N]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn current_dir_path() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf())
}

fn current_dir_string() -> String {
    current_dir_path().to_string_lossy().to_string()
}

fn model_fingerprint(model: &ModelManager) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut page_paths: Vec<&String> = model.graph_registry.pages.keys().collect();
    page_paths.sort();
    for path in page_paths {
        path.hash(&mut hasher);
        if let Some(page) = model.graph_registry.pages.get(path) {
            page.frontmatter_content.hash(&mut hasher);
        }
    }

    for element in model.graph_registry.get_all_elements() {
        element.identifier.hash(&mut hasher);
        element.name.hash(&mut hasher);
        element.element_type.as_str().hash(&mut hasher);
        element.content.hash(&mut hasher);
        element.file_path.hash(&mut hasher);
        for relation in &element.relations {
            relation.relation_type.name.hash(&mut hasher);
            relation.target.link.as_str().hash(&mut hasher);
        }
        for attachment in &element.attachments {
            attachment.target.as_str().hash(&mut hasher);
        }
    }

    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tool_list_does_not_advertise_mutations() {
        let names: Vec<String> = tool_definitions(false)
            .into_iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str).map(str::to_string))
            .collect();

        assert!(names.contains(&"reqvire.search".to_string()));
        assert!(names.contains(&"reqvire.format".to_string()));
        assert!(!names.contains(&"reqvire.add_element".to_string()));
        assert!(!names.contains(&"reqvire.link".to_string()));
    }

    #[test]
    fn mutation_tool_list_is_flag_gated() {
        let names: Vec<String> = tool_definitions(true)
            .into_iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str).map(str::to_string))
            .collect();

        assert!(names.contains(&"reqvire.add_element".to_string()));
        assert!(names.contains(&"reqvire.link".to_string()));
    }

    #[test]
    fn initialize_response_uses_standard_capabilities() {
        let response =
            initialize(json!(1), json!({ "protocolVersion": MCP_PROTOCOL_VERSION })).unwrap();
        let result = response.get("result").unwrap();

        assert_eq!(result.get("protocolVersion").unwrap(), MCP_PROTOCOL_VERSION);
        assert!(result.get("capabilities").unwrap().get("tools").is_some());
        assert!(result
            .get("capabilities")
            .unwrap()
            .get("resources")
            .is_some());
        assert!(result.get("serverInfo").is_some());
    }

    #[test]
    fn invalid_tool_arguments_are_protocol_errors() {
        let response = handle_rpc_value(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "tools/call",
                "params": {
                    "name": "reqvire.collect",
                    "arguments": {}
                }
            }),
            false,
            &globset::GlobSetBuilder::new().build().unwrap(),
        )
        .unwrap();

        assert_eq!(response.get("error").unwrap().get("code").unwrap(), -32602);
        assert!(response
            .get("error")
            .unwrap()
            .get("data")
            .unwrap()
            .get("message")
            .unwrap()
            .as_str()
            .unwrap()
            .contains("element_name"));
    }
}
