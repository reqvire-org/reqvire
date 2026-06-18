use globset::GlobSet;
use reqvire::error::ReqvireError;
use reqvire::mcp_prompts::{prompt_definitions_json, prompt_get_result_json};
use reqvire::tool_interface::{
    request_requires_write_tool, resource_definitions as shared_resource_definitions,
    tool_definitions as shared_tool_definitions,
    validate_startup_with_options as shared_validate_startup_with_options, ReqvireToolRegistry,
};
use rmcp::{
    handler::server::ServerHandler,
    model::{
        CallToolRequestParams, CallToolResult, ErrorCode, GetPromptRequestParams, GetPromptResult,
        Implementation, ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult,
        ListToolsResult, PaginatedRequestParams, Prompt, ReadResourceRequestParams,
        ReadResourceResult, ServerCapabilities, ServerInfo, Tool,
    },
    service::{MaybeSendFuture, RequestContext, RoleServer},
    transport::streamable_http_server::{
        session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
    },
    ErrorData as McpError,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

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
struct ReqvireMcpServer {
    enable_mutations: bool,
    with_size_estimates: bool,
    excluded_filename_patterns: Arc<GlobSet>,
    write_lock: Arc<Mutex<()>>,
}

impl ReqvireMcpServer {
    fn new(
        enable_mutations: bool,
        with_size_estimates: bool,
        excluded_filename_patterns: &GlobSet,
    ) -> Self {
        Self {
            enable_mutations,
            with_size_estimates,
            excluded_filename_patterns: Arc::new(excluded_filename_patterns.clone()),
            write_lock: Arc::new(Mutex::new(())),
        }
    }

    async fn call_handler(
        &self,
        method: &str,
        params: Value,
        serialize: bool,
    ) -> Result<Value, McpError> {
        if serialize {
            let _guard = self.write_lock.lock().await;
            return self.call_handler_unlocked(method, params);
        }
        self.call_handler_unlocked(method, params)
    }

    fn call_handler_unlocked(&self, method: &str, params: Value) -> Result<Value, McpError> {
        let response = handle_rpc_value(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params
            }),
            self.enable_mutations,
            self.with_size_estimates,
            self.excluded_filename_patterns.as_ref(),
        )
        .ok_or_else(|| McpError::internal_error("MCP request produced no response", None))?;
        if let Some(error) = response.get("error") {
            let code = error.get("code").and_then(Value::as_i64).unwrap_or(-32603) as i32;
            let message = error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("MCP request failed")
                .to_string();
            let data = error.get("data").cloned();
            return Err(McpError::new(ErrorCode(code), message, data));
        }
        response
            .get("result")
            .cloned()
            .ok_or_else(|| McpError::internal_error("MCP response missing result", None))
    }
}

impl ServerHandler for ReqvireMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
        )
        .with_server_info(Implementation::new("reqvire", env!("CARGO_PKG_VERSION")))
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, McpError>> + MaybeSendFuture + '_ {
        async move {
            let result = self.call_handler("tools/list", json!({}), false).await?;
            let tools = serde_json::from_value::<Vec<Tool>>(
                result.get("tools").cloned().unwrap_or_else(|| json!([])),
            )
            .map_err(|error| McpError::internal_error(error.to_string(), None))?;
            Ok(ListToolsResult::with_all_items(tools))
        }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, McpError>> + MaybeSendFuture + '_ {
        async move {
            let arguments = request
                .arguments
                .map(Value::Object)
                .unwrap_or_else(|| json!({}));
            let serialize = request_requires_write_tool(&request.name, Some(&arguments));
            let result = self
                .call_handler(
                    "tools/call",
                    json!({
                        "name": request.name.as_ref(),
                        "arguments": arguments
                    }),
                    serialize,
                )
                .await?;
            serde_json::from_value::<CallToolResult>(result)
                .map_err(|error| McpError::internal_error(error.to_string(), None))
        }
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListResourcesResult, McpError>> + MaybeSendFuture + '_ {
        async move {
            let result = self
                .call_handler("resources/list", json!({}), false)
                .await?;
            serde_json::from_value::<ListResourcesResult>(result)
                .map_err(|error| McpError::internal_error(error.to_string(), None))
        }
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ReadResourceResult, McpError>> + MaybeSendFuture + '_ {
        async move {
            let result = self
                .call_handler("resources/read", json!({ "uri": request.uri }), false)
                .await?;
            serde_json::from_value::<ReadResourceResult>(result)
                .map_err(|error| McpError::internal_error(error.to_string(), None))
        }
    }

    fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListResourceTemplatesResult, McpError>> + MaybeSendFuture + '_
    {
        std::future::ready(Ok(ListResourceTemplatesResult::with_all_items(vec![])))
    }

    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListPromptsResult, McpError>> + MaybeSendFuture + '_ {
        async move {
            let result = self.call_handler("prompts/list", json!({}), false).await?;
            let prompts = serde_json::from_value::<Vec<Prompt>>(
                result.get("prompts").cloned().unwrap_or_else(|| json!([])),
            )
            .map_err(|error| McpError::internal_error(error.to_string(), None))?;
            Ok(ListPromptsResult {
                prompts,
                next_cursor: None,
                meta: None,
            })
        }
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<GetPromptResult, McpError>> + MaybeSendFuture + '_ {
        async move {
            let arguments = request
                .arguments
                .map(Value::Object)
                .unwrap_or_else(|| json!({}));
            let result = self
                .call_handler(
                    "prompts/get",
                    json!({
                        "name": request.name,
                        "arguments": arguments
                    }),
                    false,
                )
                .await?;
            serde_json::from_value::<GetPromptResult>(result)
                .map_err(|error| McpError::internal_error(error.to_string(), None))
        }
    }
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

    let server = ReqvireMcpServer::new(
        enable_mutations,
        with_size_estimates,
        excluded_filename_patterns,
    );
    let service: StreamableHttpService<ReqvireMcpServer, LocalSessionManager> =
        StreamableHttpService::new(
            move || Ok(server.clone()),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig::default()
                .with_allowed_origins(loopback_allowed_origins())
                .with_stateful_mode(false)
                .with_json_response(true),
        );
    let app = axum::Router::new().nest_service("/mcp", service);

    eprintln!("MCP HTTP server listening at http://{}/mcp", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .map_err(|e| ReqvireError::ProcessError(format!("MCP HTTP server error: {}", e)))?;

    Ok(())
}

fn loopback_allowed_origins() -> [&'static str; 6] {
    [
        "http://localhost",
        "https://localhost",
        "http://127.0.0.1",
        "https://127.0.0.1",
        "http://[::1]",
        "https://[::1]",
    ]
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
        "prompts/list" => Some(rpc_result(
            id,
            json!({ "prompts": prompt_definitions_json() }),
        )),
        "prompts/get" => Some(handle_prompt_get(id, request.params)),
        _ => Some(rpc_error(
            id,
            -32601,
            "Method not found",
            Some(json!({ "method": request.method })),
        )),
    }
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

fn handle_prompt_get(id: Value, params: Value) -> Value {
    let name = match params.get("name").and_then(Value::as_str) {
        Some(name) => name,
        None => {
            return rpc_error(
                id,
                -32602,
                "Invalid params",
                Some(json!({ "message": "prompts/get requires string field 'name'" })),
            );
        }
    };
    let arguments = params.get("arguments").and_then(Value::as_object);

    match prompt_get_result_json(name, arguments) {
        Ok(value) => rpc_result(id, value),
        Err(err) => rpc_error(
            id,
            -32602,
            "Prompt not found",
            Some(json!({ "message": err.to_string(), "prompt": name })),
        ),
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
        ReqvireError::InvalidReusedContractContextScope(_)
        | ReqvireError::InvalidReusedContractContextTarget(_) => {
            ("reused_contract_context_contract_violation", None)
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
        "reused_contract_context_contract_violation" => {
            "Use a valid contract reused_contract_context that respects submodel reused_contract_context contracts."
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
