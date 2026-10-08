//! Argument-parsing and model-loading helpers shared across MCP tool
//! dispatch and registry entry points.

use super::*;
use std::sync::Arc;

pub fn load_model(excluded_filename_patterns: &GlobSet) -> Result<Arc<ModelManager>, ReqvireError> {
    load_model_with_options(excluded_filename_patterns, false)
}

pub fn load_model_with_options(
    excluded_filename_patterns: &GlobSet,
    with_size_estimates: bool,
) -> Result<Arc<ModelManager>, ReqvireError> {
    crate::model_cache::load_cached_model(
        excluded_filename_patterns,
        ModelBuildOptions {
            lenient: false,
            with_size_estimates,
        },
    )
}

/// Mutable candidates never share graph/page state with accepted read handles.
pub(super) fn load_mutation_model(exclusions: &GlobSet) -> Result<ModelManager, ReqvireError> {
    Ok(load_model(exclusions)?.as_ref().clone())
}

pub fn parse_json_string(json_str: String) -> Result<Value, ReqvireError> {
    serde_json::from_str(&json_str).map_err(ReqvireError::from)
}

pub fn required_string_arg(args: &Value, name: &str) -> Result<String, ReqvireError> {
    string_arg(args, name).ok_or_else(|| {
        ReqvireError::ProcessError(format!("Missing required string argument '{}'", name))
    })
}

pub fn string_arg(args: &Value, name: &str) -> Option<String> {
    args.get(name)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

pub fn bool_arg(args: &Value, name: &str, default: bool) -> bool {
    args.get(name).and_then(Value::as_bool).unwrap_or(default)
}

pub fn usize_arg(args: &Value, name: &str, default: usize) -> usize {
    args.get(name)
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(default)
}

pub fn string_array_arg(args: &Value, name: &str) -> Result<Vec<String>, ReqvireError> {
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

pub fn git_state() -> Value {
    let head = git_output(["rev-parse", "HEAD"]);
    let status = git_output(["status", "--porcelain"]);
    json!({
        "head": head,
        "dirty": status.as_ref().map(|s| !s.trim().is_empty())
    })
}

pub fn eligible_git_worktrees_state() -> Value {
    let Ok(scope) = crate::workspace::WorkspaceScope::discover() else {
        return json!([]);
    };

    let worktrees = scope
        .git_worktrees
        .iter()
        .map(|root| {
            let workspace_relative_root = root
                .strip_prefix(&scope.root)
                .ok()
                .map(|path| {
                    if path.as_os_str().is_empty() {
                        ".".to_string()
                    } else {
                        path.to_string_lossy().to_string()
                    }
                })
                .unwrap_or_else(|| root.to_string_lossy().to_string());
            let head = git_output_in_dir(root, ["rev-parse", "HEAD"]);
            let status = git_output_in_dir(root, ["status", "--porcelain"]);
            json!({
                "root": root.to_string_lossy().to_string(),
                "workspace_relative_root": workspace_relative_root,
                "head": head,
                "dirty": status.as_ref().map(|s| !s.trim().is_empty())
            })
        })
        .collect::<Vec<_>>();

    json!(worktrees)
}

pub fn git_output<const N: usize>(args: [&str; N]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_output_in_dir<const N: usize>(
    directory: &std::path::Path,
    args: [&str; N],
) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(directory)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn current_dir_path() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn current_dir_string() -> String {
    current_dir_path().to_string_lossy().to_string()
}

pub fn model_fingerprint(model: &ModelManager) -> Result<String, ReqvireError> {
    crate::model_revision::fingerprint(&model.graph_registry.get_all_elements())
}
