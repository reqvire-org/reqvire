//! Dependency-first publication plans. Git/GitHub single-repository adapters
//! supply execution; this layer owns composition ordering and retained results.
use crate::Error;
use serde_json::{json, Value};
use std::{collections::BTreeMap, fmt::Display, path::PathBuf};

pub fn changed(project: &Value, base: &Value) -> bool {
    if base.is_null() {
        return true;
    }
    let identity = crate::manifest::normalize_url(
        project["repository"]
            .as_str()
            .or_else(|| project["fetch_url"].as_str())
            .unwrap_or(""),
    );
    let matched = base["projects"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|entry| {
            crate::manifest::normalize_url(entry["repository"].as_str().unwrap_or("")) == identity
                && entry["path"] == project["path"]
        });
    matched.is_none_or(|entry| entry["commit"] != project["commit"])
}

pub fn distinct_refs(
    owners: impl IntoIterator<Item = (PathBuf, String, String)>,
) -> Result<(), Error> {
    let mut destinations = BTreeMap::new();
    for (path, repository, branch) in owners {
        if let Some(previous) = destinations.insert((repository, branch.clone()), path.clone()) {
            if previous != path {
                return Err(Error::ProcessError(format!("Repository owners {} and {} target the same published branch {branch}; prepare distinct authoring branches before publication",if previous.as_os_str().is_empty(){"workspace root".into()}else{previous.display().to_string()},path.display())));
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub enum Kind {
    Push,
    PullRequests,
}

/// Stop before the root on a failed or uncertain dependency. Confirmed earlier
/// outcomes remain in the response for idempotent adapter retries.
pub fn publish<E: Display>(
    paths: &[PathBuf],
    kind: Kind,
    mut dependency: impl FnMut(&std::path::Path) -> Result<Value, E>,
    root: impl FnOnce(&[Value]) -> Result<Value, E>,
) -> Value {
    let field = match kind {
        Kind::Push => "projects",
        Kind::PullRequests => "pull_requests",
    };
    let mut completed = Vec::new();
    for path in paths {
        match dependency(path) {
            Ok(mut result) => {
                result["path"] = json!(path);
                if result.get("pr_number").is_some() {
                    result["number"] = result["pr_number"].clone();
                    result["url"] = result["pr_url"].clone();
                }
                let unknown = result["outcome"] == "unknown";
                let failed = result["outcome"] == "failed";
                completed.push(result);
                if unknown || failed {
                    let message = match kind {
                        Kind::Push => {
                            "Dependency push could not be confirmed; root was not published"
                        }
                        Kind::PullRequests => {
                            "Dependency PR creation could not be confirmed; root PR was not created"
                        }
                    };
                    return json!({"outcome":if unknown{"unknown"}else{"failed"},field:completed,"error":message});
                }
            }
            Err(error) => {
                if matches!(kind, Kind::Push) {
                    completed.push(json!({"path":path,"outcome":"failed"}));
                }
                return json!({"outcome":"failed",field:completed,"error":error.to_string()});
            }
        }
    }
    let mut result = match root(&completed) {
        Ok(result) => result,
        Err(error) => return json!({"outcome":"failed",field:completed,"error":error.to_string()}),
    };
    if matches!(kind, Kind::PullRequests) {
        let mut entry = result.clone();
        entry["path"] = json!("");
        entry["number"] = entry["pr_number"].clone();
        entry["url"] = entry["pr_url"].clone();
        completed.push(entry);
    }
    result[field] = json!(completed);
    result
}

pub fn linked_body(body: &str, prs: &[Value]) -> String {
    let mut result = body.to_owned();
    if !prs.is_empty() {
        result.push_str("\n\n## Coordinated pull requests\n\n");
        for pr in prs {
            result.push_str(&format!(
                "- [{}]({}) — commit `{}`\n",
                pr["path"].as_str().unwrap_or(""),
                pr["pr_url"].as_str().unwrap_or(""),
                pr["commit"].as_str().unwrap_or("")
            ));
        }
    }
    result
}
