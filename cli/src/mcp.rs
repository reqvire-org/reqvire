use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{header, HeaderMap, Response, StatusCode};
use axum::routing::post;
use axum::Router;
use globset::GlobSet;
use reqvire::error::ReqvireError;
use reqvire::tool_interface::{
    request_requires_write_tool, resource_definitions as shared_resource_definitions,
    tool_definitions as shared_tool_definitions,
    validate_startup_with_options as shared_validate_startup_with_options, ReqvireToolRegistry,
    MCP_PROTOCOL_VERSION as SHARED_MCP_PROTOCOL_VERSION,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use tokio::sync::Mutex;

const MCP_PROTOCOL_VERSION: &str = SHARED_MCP_PROTOCOL_VERSION;

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
    with_size_estimates: bool,
    excluded_filename_patterns: Arc<GlobSet>,
    write_lock: Arc<Mutex<()>>,
}

pub fn serve_stdio(
    enable_mutations: bool,
    with_size_estimates: bool,
    excluded_filename_patterns: &GlobSet,
) -> Result<(), ReqvireError> {
    shared_validate_startup_with_options(excluded_filename_patterns, with_size_estimates)?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.map_err(ReqvireError::IoError)?;
        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Value>(&line) {
            Ok(raw) => handle_rpc_value(
                raw,
                enable_mutations,
                with_size_estimates,
                excluded_filename_patterns,
            ),
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
    with_size_estimates: bool,
    excluded_filename_patterns: &GlobSet,
    host: &str,
    port: u16,
) -> Result<(), ReqvireError> {
    shared_validate_startup_with_options(excluded_filename_patterns, with_size_estimates)?;

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        ReqvireError::ProcessError(format!("Failed to start MCP HTTP server: {}", e))
    })?;

    let state = HttpMcpState {
        enable_mutations,
        with_size_estimates,
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
            state.with_size_estimates,
            state.excluded_filename_patterns.as_ref(),
        )
    } else {
        handle_rpc_value(
            raw,
            state.enable_mutations,
            state.with_size_estimates,
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

    request_requires_write_tool(tool_name, params.get("arguments"))
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
    with_size_estimates: bool,
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
            json!({ "tools": shared_tool_definitions(enable_mutations) }),
        )),
        "tools/call" => Some(handle_tool_call(
            id,
            request.params,
            enable_mutations,
            with_size_estimates,
            excluded_filename_patterns,
        )),
        "resources/list" => Some(rpc_result(
            id,
            json!({ "resources": shared_resource_definitions() }),
        )),
        "resources/read" => Some(handle_resource_read(
            id,
            request.params,
            enable_mutations,
            with_size_estimates,
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
    with_size_estimates: bool,
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

    let registry = ReqvireToolRegistry::new_with_options(
        enable_mutations,
        with_size_estimates,
        excluded_filename_patterns,
    );

    if registry.is_mutation_tool(&params.name) && !enable_mutations {
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

    if !registry.tool_exists(&params.name) {
        return rpc_error(
            id,
            -32602,
            "Tool not found",
            Some(json!({ "tool": params.name })),
        );
    }

    if let Err(message) = registry.validate_tool_arguments(&params.name, &params.arguments) {
        return rpc_error(
            id,
            -32602,
            "Invalid tool arguments",
            Some(json!({ "tool": params.name, "message": message })),
        );
    }

    let outcome = registry.call_tool(&params.name, &params.arguments);

    match outcome {
        Ok(value) => rpc_result(id, tool_success(value)),
        Err(err) => rpc_result(id, tool_error(&params.name, err)),
    }
}

fn handle_resource_read(
    id: Value,
    params: Value,
    enable_mutations: bool,
    with_size_estimates: bool,
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

    let registry = ReqvireToolRegistry::new_with_options(
        enable_mutations,
        with_size_estimates,
        excluded_filename_patterns,
    );
    let value = registry.read_resource(uri);

    match value {
        Ok(value) => rpc_result(id, value),
        Err(err) => rpc_result(id, tool_error("resources/read", err)),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tool_list_does_not_advertise_mutations() {
        let names: Vec<String> = shared_tool_definitions(false)
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
        let names: Vec<String> = shared_tool_definitions(true)
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
