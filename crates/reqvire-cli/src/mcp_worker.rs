//! Private pipe protocol. Each worker has one fixed cwd, core cache and owned model.
use crate::mcp_session::MutationSession;
use reqvire::{error::ReqvireError, explorer_runtime::ExplorerRuntimeAssets};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

/// Runtime serialization is reused for clones of the core cache's completed build.
/// Resource previews also depend on bytes outside model validation's dependency set.
struct ReadRuntime {
    index: Arc<reqvire::semantic_contract::SemanticIndex>,
    resources: BTreeMap<PathBuf, Option<String>>,
}
fn preview_input(path: &std::path::Path) -> Option<String> {
    let metadata = std::fs::metadata(path).ok()?;
    if !metadata.is_file() || metadata.len() > 512 * 1024 {
        return None;
    }
    std::fs::read_to_string(path)
        .ok()
        .map(|s| reqvire::utils::hash_content(&s))
}
fn read_runtime(
    options: &WorkerOptions,
    cache: &mut Option<ReadRuntime>,
) -> Result<Value, ReqvireError> {
    let exclusions = crate::config::get_excluded_filename_patterns_glob_set();
    let model = reqvire::model_cache::load_cached_model(
        &exclusions,
        reqvire::ModelBuildOptions {
            lenient: false,
            with_size_estimates: options.with_size_estimates,
        },
    )?;
    let index = &model
        .semantic_store
        .as_ref()
        .ok_or_else(|| error("Missing validated semantic state"))?
        .index;
    if cache.as_ref().is_some_and(|old| {
        Arc::ptr_eq(&old.index, index)
            && old
                .resources
                .iter()
                .all(|(p, value)| preview_input(p) == *value)
    }) {
        return Ok(json!({"runtime_unchanged":true}));
    }
    let assets = reqvire::explorer_runtime::build_runtime_assets(&model)?;
    let mut deserializer = serde_json::Deserializer::from_str(&assets.project_store_json);
    deserializer.disable_recursion_limit();
    let mut store = Value::deserialize(&mut deserializer)?;
    let root = std::env::current_dir()?;
    let branch = crate::mcp_process::git(&root, &["symbolic-ref", "--short", "HEAD"])
        .unwrap_or_else(|_| "Detached HEAD".into());
    store["project"]["worktree_id"] = json!(options.worktree_id);
    store["project"]["branch"] = json!(branch);
    let resources = store["resources"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|row| {
            row["file_path"].as_str().map(|path| {
                (
                    root.join(path),
                    row["source_text"]
                        .as_str()
                        .map(reqvire::utils::hash_content),
                )
            })
        })
        .collect();
    *cache = Some(ReadRuntime {
        index: Arc::clone(index),
        resources,
    });
    Ok(
        json!({"runtime":RuntimeData { project_store_json:serde_json::to_string(&store)?, ontologies_ttl:assets.ontologies_ttl }}),
    )
}
fn read_status() -> Value {
    let root = std::env::current_dir().unwrap_or_default();
    json!({"workspace_root":root, "branch":crate::mcp_process::git(&root, &["symbolic-ref","--short","HEAD"]).unwrap_or_else(|_| "Detached HEAD".into()),
        "head":crate::mcp_process::git(&root, &["rev-parse","HEAD"]).ok(),"available":true,"owned":false})
}

#[derive(Serialize, Deserialize)]
pub struct WorkerOptions {
    pub worktree_id: String,
    pub enable_commits: bool,
    pub with_size_estimates: bool,
    pub explorer: bool,
    #[serde(default)]
    pub read_only: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RuntimeData {
    pub project_store_json: String,
    pub ontologies_ttl: String,
}
impl RuntimeData {
    pub fn assets(&self) -> ExplorerRuntimeAssets {
        ExplorerRuntimeAssets {
            project_store_js: format!(
                "window.reqvireProjectStore = {};\n",
                self.project_store_json
            ),
            project_store_json: self.project_store_json.clone(),
            ontologies_ttl: self.ontologies_ttl.clone(),
        }
    }
}
fn error(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}
fn runtime(session: &MutationSession, id: &str) -> Result<RuntimeData, ReqvireError> {
    let assets = session.with_snapshot(reqvire::explorer_runtime::build_runtime_assets)?;
    let mut deserializer = serde_json::Deserializer::from_str(&assets.project_store_json);
    deserializer.disable_recursion_limit();
    let mut store = Value::deserialize(&mut deserializer)?;
    let status = session.status();
    store["project"]["worktree_id"] = json!(id);
    store["project"]["branch"] = status["branch"].clone();
    Ok(RuntimeData {
        project_store_json: serde_json::to_string(&store)?,
        ontologies_ttl: assets.ontologies_ttl,
    })
}
fn runtime_result(session: &MutationSession, options: &WorkerOptions, response: &mut Value) {
    if options.explorer {
        match runtime(session, &options.worktree_id) {
            Ok(data) => response["runtime"] = json!(data),
            Err(failure) => response["runtime_error"] = json!(failure.to_string()),
        }
    }
}

/// Not a public MCP transport: the manager sends one JSON request at a time over inherited pipes.
pub fn run() -> Result<(), ReqvireError> {
    reqvire::utils::enable_quiet_mode();
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let options: WorkerOptions = serde_json::from_str(
        &lines
            .next()
            .ok_or_else(|| error("Missing worker initialization"))??,
    )?;
    let exclusions = crate::config::get_excluded_filename_patterns_glob_set();
    if options.read_only {
        let mut cache = None;
        let mut ready = match read_runtime(&options, &mut cache) {
            Ok(value) => value,
            Err(failure) => {
                emit(&json!({"error":failure.to_string()}))?;
                return Ok(());
            }
        };
        ready["status"] = read_status();
        let branch = ready["status"]["branch"].clone();
        emit(&ready)?;
        for line in lines {
            let result = serde_json::from_str::<Value>(&line?)
                .map_err(|e| error(e.to_string()))
                .and_then(|request| match request["operation"].as_str() {
                    Some("load" | "runtime") => {
                        if read_status()["branch"] != branch {
                            return Err(error(
                                "Selected worktree branch changed outside the server",
                            ));
                        }
                        let result = read_runtime(&options, &mut cache)?;
                        if read_status()["branch"] != branch {
                            cache = None;
                            return Err(error("Selected worktree branch changed during loading"));
                        }
                        Ok(result)
                    }
                    Some("status") => Ok(json!({})),
                    _ => Err(error("Read-only worker rejects mutation operations")),
                });
            let mut response = match result {
                Ok(value) => value,
                Err(failure) => {
                    json!({"error":failure.to_string(),"runtime_error":failure.to_string()})
                }
            };
            response["status"] = read_status();
            emit(&response)?;
        }
        return Ok(());
    }
    let mut session = match MutationSession::start(
        &exclusions,
        options.with_size_estimates,
        options.enable_commits,
    ) {
        Ok(session) => session,
        Err(failure) => {
            emit(&json!({"error":failure.to_string()}))?;
            return Ok(());
        }
    };
    let mut ready = json!({"status":session.status()});
    runtime_result(&session, &options, &mut ready);
    emit(&ready)?;
    for line in lines {
        let request: Value = match serde_json::from_str(&line?) {
            Ok(value) => value,
            Err(failure) => {
                emit(&json!({"error":failure.to_string()}))?;
                continue;
            }
        };
        let operation = request["operation"].as_str().unwrap_or("");
        let mut response = match operation {
            "status" => json!({}),
            "check_clean" => match session
                .check_head()
                .and_then(|_| crate::mcp_session::require_clean(&std::env::current_dir()?))
            {
                Ok(()) => json!({}),
                Err(failure) => json!({"error":failure.to_string()}),
            },
            "commit" => match session.explicit_commit(request["message"].as_str().unwrap_or("")) {
                Ok(value) => json!({"result":value}),
                Err(failure) => json!({"error":failure.to_string()}),
            },
            "runtime" => {
                let mut value = json!({});
                runtime_result(&session, &options, &mut value);
                value
            }
            "rpc" => {
                let method = request["method"].as_str().unwrap_or("");
                let params = request["params"].clone();
                let tool = params["name"].as_str().unwrap_or(method).to_string();
                let persists = method == "tools/call"
                    && crate::mcp::request_refreshes_runtime_after_write(&params);
                let (result, changed) = session.execute(persists, &tool, || {
                    let response = crate::mcp::handle_rpc_value(
                        json!({"id":1,"method":method,"params":params}),
                        true,
                        options.with_size_estimates,
                        &exclusions,
                    )
                    .ok_or_else(|| {
                        rmcp::ErrorData::internal_error("Missing worker result", None)
                    })?;
                    if let Some(failure) = response.get("error") {
                        Err(serde_json::from_value(failure.clone()).unwrap_or_else(|_| {
                            rmcp::ErrorData::internal_error(
                                "Worker operation failed",
                                Some(failure.clone()),
                            )
                        }))
                    } else {
                        Ok(response["result"].clone())
                    }
                });
                let mut value = match result {
                    Ok(value) => json!({"result":value}),
                    Err(failure) => json!({"rpc_error":failure}),
                };
                if changed {
                    runtime_result(&session, &options, &mut value);
                }
                value
            }
            _ => json!({"error":"Unknown private worker operation"}),
        };
        response["status"] = session.status();
        emit(&response)?;
    }
    Ok(())
}
fn emit(value: &Value) -> Result<(), ReqvireError> {
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    serde_json::to_writer(&mut writer, value)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}
