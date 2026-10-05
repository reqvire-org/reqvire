//! Runtime assets for the served Explorer SPA.
//!
//! The compiled Explorer bundle is embedded by `build.rs`. Runtime model data is
//! generated in memory from the already-validated registry and served beside the
//! embedded bundle.

use crate::error::ReqvireError;
use crate::html::store::{build_project_store, project_store_javascript};
use crate::model::ModelManager;
use serde::Serialize;
use serde_json::Value;

include!(concat!(env!("OUT_DIR"), "/explorer_bundle_manifest.rs"));

#[derive(Clone)]
pub struct ExplorerRuntimeAssets {
    pub project_store_js: String,
    pub project_store_json: String,
    pub ontologies_ttl: String,
}

/// Canonical structured runtime. Local generation and worker publication use
/// this value for assets and chunks without reparsing generated JSON text.
pub struct ExplorerRuntimeData {
    pub project_store: Value,
    pub ontologies_ttl: String,
}

/// Serving identity is applied to the typed store before any serialization.
pub struct ExplorerRuntimeContext<'a> {
    pub worktree_id: &'a str,
    pub branch: &'a str,
}

impl ExplorerRuntimeAssets {
    pub fn from_store(
        store: &impl Serialize,
        ontologies_ttl: String,
    ) -> Result<Self, ReqvireError> {
        let project_store_json = serde_json::to_string(store)?;
        let project_store_js = project_store_javascript(&project_store_json);
        Ok(Self {
            project_store_js,
            project_store_json,
            ontologies_ttl,
        })
    }
}

impl ExplorerRuntimeData {
    pub fn into_assets(self) -> Result<ExplorerRuntimeAssets, ReqvireError> {
        ExplorerRuntimeAssets::from_store(&self.project_store, self.ontologies_ttl)
    }
}

pub fn build_runtime_data(
    model: &ModelManager,
    context: Option<ExplorerRuntimeContext<'_>>,
) -> Result<ExplorerRuntimeData, ReqvireError> {
    log::debug!("Building Explorer runtime assets from validated model");
    // Use the semantic state accepted with this graph. Re-reading external
    // ontology files here could mix two different source observations.
    let semantic_store = model.semantic_store.as_ref().ok_or_else(|| {
        ReqvireError::ProcessError("Explorer assets require a completed model build".to_owned())
    })?;
    let mut project_store = build_project_store(&model.graph_registry, semantic_store);
    if let Some(context) = context {
        project_store.project.worktree_id = Some(context.worktree_id.to_owned());
        project_store.project.branch = Some(context.branch.to_owned());
    }
    let ontologies_ttl = semantic_store.index().to_turtle_string()?;

    Ok(ExplorerRuntimeData {
        project_store: serde_json::to_value(project_store)?,
        ontologies_ttl,
    })
}

pub fn build_runtime_assets(model: &ModelManager) -> Result<ExplorerRuntimeAssets, ReqvireError> {
    build_runtime_data(model, None)?.into_assets()
}

/// Writes the full Explorer SPA + generated runtime data to `output_dir`.
///
/// Produces a self-contained static site ready for GitHub Pages or any static host:
///   <output_dir>/index.html           (and all other bundle assets)
///   <output_dir>/assets/project-store.js
///   <output_dir>/ontologies.ttl
pub fn export_to_dir(
    assets: &ExplorerRuntimeAssets,
    output_dir: &std::path::Path,
) -> Result<(), ReqvireError> {
    use std::fs;

    for (rel_path, bytes) in EMBEDDED_BUNDLE {
        let dest = output_dir.join(rel_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ReqvireError::ProcessError(format!(
                    "Failed to create directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }
        fs::write(&dest, bytes).map_err(|e| {
            ReqvireError::ProcessError(format!("Failed to write {}: {}", dest.display(), e))
        })?;
    }

    let store_js_path = output_dir.join("assets").join("project-store.js");
    fs::create_dir_all(store_js_path.parent().expect("unexpected error"))
        .map_err(|e| ReqvireError::ProcessError(format!("Failed to create assets dir: {}", e)))?;
    fs::write(&store_js_path, assets.project_store_js.as_bytes()).map_err(|e| {
        ReqvireError::ProcessError(format!("Failed to write project-store.js: {}", e))
    })?;

    let ttl_path = output_dir.join("ontologies.ttl");
    fs::write(&ttl_path, assets.ontologies_ttl.as_bytes()).map_err(|e| {
        ReqvireError::ProcessError(format!("Failed to write ontologies.ttl: {}", e))
    })?;

    copy_workspace_assets(output_dir)?;

    Ok(())
}

fn copy_workspace_assets(output_dir: &std::path::Path) -> Result<(), ReqvireError> {
    let output_dir = output_dir
        .canonicalize()
        .unwrap_or_else(|_| output_dir.to_path_buf());
    let scope = crate::workspace::WorkspaceScope::discover()?;
    for scan_root in scope.scan_roots() {
        copy_workspace_assets_from_dir(&scope, &scan_root, &output_dir)?;
    }
    Ok(())
}

fn copy_workspace_assets_from_dir(
    scope: &crate::workspace::WorkspaceScope,
    dir: &std::path::Path,
    output_dir: &std::path::Path,
) -> Result<(), ReqvireError> {
    use std::fs;

    let canonical_dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    if canonical_dir == output_dir || canonical_dir.starts_with(output_dir) {
        return Ok(());
    }

    for entry in fs::read_dir(dir).map_err(|e| {
        ReqvireError::ProcessError(format!("Failed to read directory {}: {}", dir.display(), e))
    })? {
        let entry = entry.map_err(|e| {
            ReqvireError::ProcessError(format!("Failed to read directory entry: {}", e))
        })?;
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if path.is_dir() {
            if should_skip_workspace_asset_dir(&name) {
                continue;
            }
            copy_workspace_assets_from_dir(scope, &path, output_dir)?;
            continue;
        }

        if !path.is_file() || !scope.is_eligible_path(&path) || !is_workspace_asset_path(&path) {
            continue;
        }

        let rel = path.strip_prefix(&scope.root).map_err(|e| {
            ReqvireError::ProcessError(format!(
                "Failed to relativize asset path {}: {}",
                path.display(),
                e
            ))
        })?;
        let dest = output_dir.join(rel);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ReqvireError::ProcessError(format!(
                    "Failed to create asset directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }
        fs::copy(&path, &dest).map_err(|e| {
            ReqvireError::ProcessError(format!(
                "Failed to copy workspace asset {} to {}: {}",
                path.display(),
                dest.display(),
                e
            ))
        })?;
    }

    Ok(())
}

fn should_skip_workspace_asset_dir(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | ".index"
            | ".playwright-cli"
            | ".playwright-mcp"
            | "node_modules"
            | "target"
            | "tmp"
            | "tmpspec"
    )
}

pub fn is_workspace_asset_path(path: &std::path::Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase()),
        Some(ext)
            if matches!(
                ext.as_str(),
                "png"
                    | "jpg"
                    | "jpeg"
                    | "gif"
                    | "webp"
                    | "svg"
                    | "pdf"
                    | "txt"
                    | "csv"
                    | "json"
                    | "jsonld"
                    | "ttl"
                    | "turtle"
            )
    )
}

pub fn embedded_asset(path: &str) -> Option<&'static [u8]> {
    let normalized = normalize_asset_path(path);
    EMBEDDED_BUNDLE
        .iter()
        .find_map(|(asset_path, bytes)| (*asset_path == normalized).then_some(*bytes))
}

pub fn index_html() -> &'static [u8] {
    embedded_asset("index.html").expect("embedded Explorer bundle must include index.html")
}

fn normalize_asset_path(path: &str) -> String {
    let trimmed = path.trim_start_matches('/');
    if trimmed.is_empty() {
        "index.html".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn seed_and_json_share_one_serialization_and_preserve_script_sensitive_content() {
        struct Counted<'a> {
            visits: &'a Cell<usize>,
            store: Value,
        }
        impl Serialize for Counted<'_> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.visits.set(self.visits.get() + 1);
                self.store.serialize(serializer)
            }
        }
        let visits = Cell::new(0);
        let counted = Counted {
            visits: &visits,
            store: serde_json::json!({
                "elements":[{"name":"Lärche 測定 </script>&\u{2028}\u{2029}", "content":"quotes: \" \\"}],
                "numbers":[-0.0, 1.0, 1e-20, 9223372036854775807_i64], "empty":[], "null":null,
            }),
        };
        let assets = ExplorerRuntimeAssets::from_store(&counted, "ontology 測定".into()).unwrap();
        assert_eq!(
            visits.get(),
            1,
            "seed and full JSON must serialize the store once"
        );
        assert_eq!(
            serde_json::from_str::<Value>(&assets.project_store_json).unwrap(),
            counted.store
        );
        let seed = assets
            .project_store_js
            .strip_prefix("window.reqvireProjectStore = ")
            .unwrap()
            .strip_suffix(";\n")
            .unwrap();
        assert!(!seed.contains(['<', '>', '&', '\u{2028}', '\u{2029}']));
        assert!(seed.contains("\\u003c/script\\u003e\\u0026\\u2028\\u2029"));
        assert_eq!(serde_json::from_str::<Value>(seed).unwrap(), counted.store);
        assert_eq!(assets.ontologies_ttl, "ontology 測定");
    }
}
