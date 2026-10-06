//! Private pipe protocol. Each worker has one fixed cwd, core cache and owned model.
use crate::mcp_session::{GitCheckout, MutationSession};
use reqvire::{
    error::ReqvireError,
    explorer_runtime::{build_runtime_data, ExplorerRuntimeContext, ExplorerRuntimeData},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

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
    branch: &str,
) -> Result<Value, ReqvireError> {
    let exclusions = crate::config::get_excluded_filename_patterns_glob_set();
    let model = reqvire::model_cache::load_cached_model(
        &exclusions,
        reqvire::ModelBuildOptions {
            lenient: false,
            with_size_estimates: options.with_size_estimates,
        },
    )?;
    let index = model
        .semantic_store
        .as_ref()
        .ok_or_else(|| error("Missing validated semantic state"))?
        .index();
    if cache.as_ref().is_some_and(|old| {
        Arc::ptr_eq(&old.index, index)
            && old
                .resources
                .iter()
                .all(|(p, value)| preview_input(p) == *value)
    }) {
        return Ok(json!({"runtime_unchanged":true}));
    }
    let data = build_runtime_data(
        &model,
        Some(ExplorerRuntimeContext {
            worktree_id: &options.worktree_id,
            branch,
        }),
    )?;
    let root = std::env::current_dir()?;
    let resources = data.project_store["resources"]
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
    let mut result = json!({});
    result["runtime"] = runtime_value(data);
    Ok(result)
}
fn read_status(observed: Result<GitCheckout, ReqvireError>) -> Value {
    let root = std::env::current_dir().unwrap_or_default();
    match observed {
        Ok(observed) => observed.read_status(&root),
        Err(failure) => json!({"workspace_root":root, "branch":"Detached HEAD", "head":null,
            "available":false, "owned":false, "diagnostic":failure.to_string()}),
    }
}

#[derive(Serialize, Deserialize)]
pub struct WorkerOptions {
    pub worktree_id: String,
    pub enable_commits: bool,
    pub with_size_estimates: bool,
    pub explorer: bool,
    #[serde(default)]
    pub read_only: bool,
    #[serde(default = "available_parallelism")]
    pub read_parallelism: usize,
}
pub fn available_parallelism() -> usize {
    std::thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
}
pub fn parallel_read(method: &str, params: &Value) -> bool {
    match method {
        "tools/call" => params["name"].as_str().is_some_and(|name| {
            name != "reqvire.tool_contract" && crate::mcp_session::snapshot_read(name)
        }),
        "resources/read" | "prompts/get" => true,
        _ => false,
    }
}

fn runtime_value(data: ExplorerRuntimeData) -> Value {
    // Move the canonical value into the protocol envelope. Serializing data
    // through json! here would copy the entire store into another Value.
    Value::Object(
        [
            ("project_store".into(), data.project_store),
            ("ontologies_ttl".into(), Value::String(data.ontologies_ttl)),
        ]
        .into_iter()
        .collect(),
    )
}
fn error(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}
// Admission errors cross a pipe as text. Keep individual validation diagnostics
// now that the worker is also the initial Serve validation boundary.
fn startup_error(failure: &ReqvireError) -> String {
    let mut messages: Vec<_> = failure
        .diagnostics()
        .into_iter()
        .map(|d| d.message)
        .collect();
    messages.sort();
    if messages.is_empty() {
        failure.to_string()
    } else {
        format!("{}:\n{}", failure, messages.join("\n"))
    }
}
fn runtime(session: &MutationSession, id: &str) -> Result<ExplorerRuntimeData, ReqvireError> {
    session.with_snapshot(|model| {
        build_runtime_data(
            model,
            Some(ExplorerRuntimeContext {
                worktree_id: id,
                branch: session.branch_name(),
            }),
        )
    })
}
fn runtime_result(session: &MutationSession, options: &WorkerOptions, response: &mut Value) {
    if options.explorer {
        match runtime(session, &options.worktree_id) {
            Ok(data) => response["runtime"] = runtime_value(data),
            Err(failure) => response["runtime_error"] = json!(failure.to_string()),
        }
    }
}

/// Private correlated pipe protocol; only audited snapshot reads may overlap.
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
        let root = std::env::current_dir()?;
        let branch = GitCheckout::read(&root)?.branch_name().to_owned();
        let mut cache = None;
        let mut ready = match read_runtime(&options, &mut cache, &branch) {
            Ok(value) => value,
            Err(failure) => {
                emit(&json!({"error":startup_error(&failure)}))?;
                return Ok(());
            }
        };
        let observed = GitCheckout::read(&root)?;
        if observed.branch_name() != branch {
            emit(&json!({"error":"Selected worktree branch changed during loading"}))?;
            return Ok(());
        }
        ready["status"] = read_status(Ok(observed));
        emit(&ready)?;
        for line in lines.by_ref() {
            let mut final_observation = None;
            let parsed = serde_json::from_str::<Value>(&line?);
            let id = parsed
                .as_ref()
                .ok()
                .map(|request| request["request_id"].clone());
            let result = parsed
                .map_err(|e| error(e.to_string()))
                .and_then(|request| match request["operation"].as_str() {
                    Some("load" | "runtime") => {
                        let before = GitCheckout::read(&root)?;
                        if before.branch_name() != branch {
                            final_observation = Some(Ok(before));
                            return Err(error(
                                "Selected worktree branch changed outside the server",
                            ));
                        }
                        let result = read_runtime(&options, &mut cache, &branch);
                        // Loading (including failure) is a transition: never
                        // reuse the entry observation for final status.
                        let after = GitCheckout::read(&root);
                        let same_branch = after.as_ref().is_ok_and(|s| s.branch_name() == branch);
                        final_observation = Some(after);
                        if !same_branch {
                            cache = None;
                            return Err(error("Selected worktree branch changed during loading"));
                        }
                        result
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
            response["status"] =
                read_status(final_observation.unwrap_or_else(|| GitCheckout::read(&root)));
            response["request_id"] = id.unwrap_or(Value::Null);
            emit(&response)?;
        }
        drop(lines);
        return Ok(());
    }
    let session = match MutationSession::start(
        &exclusions,
        options.with_size_estimates,
        options.enable_commits,
    ) {
        Ok(session) => session,
        Err(failure) => {
            emit(&json!({"error":startup_error(&failure)}))?;
            return Ok(());
        }
    };
    let mut ready = json!({});
    runtime_result(&session, &options, &mut ready);
    ready["status"] = session.status();
    emit(&ready)?;
    let session = Arc::new(tokio::sync::RwLock::new(session));
    let sequence = Arc::new(AtomicU64::new(0));
    let capacity = Arc::new(tokio::sync::Semaphore::new(options.read_parallelism.max(1)));
    let mut readers: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for line in lines.by_ref() {
        let request: Value = match serde_json::from_str(&line?) {
            Ok(value) => value,
            Err(failure) => {
                emit(&json!({"error":failure.to_string()}))?;
                continue;
            }
        };
        readers.retain(|reader| !reader.is_finished());
        let operation = request["operation"].as_str().unwrap_or("");
        if operation == "rpc"
            && parallel_read(request["method"].as_str().unwrap_or(""), &request["params"])
        {
            let permit = match Arc::clone(&capacity).try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    emit(
                        &json!({"request_id":request["request_id"], "rpc_error":crate::mcp::capacity_error()}),
                    )?;
                    continue;
                }
            };
            let session = Arc::clone(&session);
            let sequence = Arc::clone(&sequence);
            let exclusions = exclusions.clone();
            let estimates = options.with_size_estimates;
            readers.push(std::thread::spawn(move || {
                let _permit = permit;
                let method = request["method"].as_str().unwrap_or("");
                let tool = if method == "tools/call" {
                    request["params"]["name"].as_str().unwrap_or("")
                } else {
                    method
                };
                let snapshot = session.blocking_read().capture_read(tool);
                let result = match &snapshot {
                    Ok(snapshot) => std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        snapshot.execute(|| dispatch_rpc(&request, estimates, &exclusions))
                    }))
                    .unwrap_or_else(|_| {
                        Err(rmcp::ErrorData::internal_error(
                            "Worker snapshot read panicked",
                            None,
                        ))
                    }),
                    Err(failure) if method == "tools/call" => {
                        Ok(crate::mcp::tool_error(tool, error(failure.to_string())))
                    }
                    Err(failure) => Err(rmcp::ErrorData::invalid_params(failure.to_string(), None)),
                };
                let mut response = rpc_result(result);
                let current = session.blocking_read();
                let status = current.status();
                response["read_status"] = snapshot.as_ref().map_or_else(
                    |_| status.clone(),
                    |snapshot| snapshot.status(status.clone()),
                );
                response["status"] = status;
                response["sequence"] = json!(sequence.fetch_add(1, Ordering::SeqCst) + 1);
                response["request_id"] = request["request_id"].clone();
                drop(current);
                let _ = emit(&response);
            }));
            continue;
        }
        let mut session = session.blocking_write();
        let mut response = match operation {
            "status" => json!({}),
            "check_writable" => match session.check_head() {
                Ok(()) => json!({}),
                Err(failure) => json!({"error":failure.to_string()}),
            },
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
            "reconcile" => {
                let accepted_model = session.model();
                match session.reconcile_commit(
                    request["attempted_commit"].as_str().unwrap_or(""),
                    request["dry_run"].as_bool().unwrap_or(true),
                ) {
                    Ok(value) => {
                        let applied = value["outcome"] == "completed"
                            && !Arc::ptr_eq(&accepted_model, &session.model());
                        let mut response = json!({"result":value});
                        if applied {
                            runtime_result(&session, &options, &mut response);
                        }
                        response
                    }
                    Err(failure) => json!({"error":failure.to_string()}),
                }
            }
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
                    dispatch_rpc(&request, options.with_size_estimates, &exclusions)
                });
                let mut value = rpc_result(result);
                if changed {
                    runtime_result(&session, &options, &mut value);
                }
                value
            }
            _ => json!({"error":"Unknown private worker operation"}),
        };
        response["status"] = session.status();
        response["sequence"] = json!(sequence.fetch_add(1, Ordering::SeqCst) + 1);
        response["request_id"] = request["request_id"].clone();
        drop(session);
        emit(&response)?;
    }
    drop(lines);
    for reader in readers {
        let _ = reader.join();
    }
    Ok(())
}
pub fn dispatch_rpc(
    request: &Value,
    estimates: bool,
    exclusions: &reqvire::exclusions::ExclusionSet,
) -> Result<Value, rmcp::ErrorData> {
    let response = crate::mcp::handle_rpc_value(
        json!({"id":1,"method":request["method"],"params":request["params"]}),
        true,
        estimates,
        exclusions,
    )
    .ok_or_else(|| rmcp::ErrorData::internal_error("Missing worker result", None))?;
    response.get("error").map_or_else(
        || Ok(response["result"].clone()),
        |failure| {
            Err(serde_json::from_value(failure.clone()).unwrap_or_else(|_| {
                rmcp::ErrorData::internal_error("Worker operation failed", Some(failure.clone()))
            }))
        },
    )
}
fn rpc_result(result: Result<Value, rmcp::ErrorData>) -> Value {
    match result {
        Ok(value) => json!({"result":value}),
        Err(failure) => json!({"rpc_error":failure}),
    }
}
fn emit(value: &Value) -> Result<(), ReqvireError> {
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    serde_json::to_writer(&mut writer, value)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worker_envelope_moves_store_and_ontology_without_cloning() {
        let name = "owned model content".repeat(32);
        let name_address = name.as_ptr();
        let ontology = "owned ontology content".repeat(32);
        let ontology_address = ontology.as_ptr();
        let store = Value::Object(std::iter::once(("name".into(), Value::String(name))).collect());
        let wire = runtime_value(ExplorerRuntimeData {
            project_store: store,
            ontologies_ttl: ontology,
        });
        assert_eq!(
            wire["project_store"]["name"]
                .as_str()
                .expect("expected a string in the test response")
                .as_ptr(),
            name_address
        );
        assert_eq!(
            wire["ontologies_ttl"]
                .as_str()
                .expect("expected a string in the test response")
                .as_ptr(),
            ontology_address
        );
    }
}
