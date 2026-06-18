//! Runtime assets for the served Explorer SPA.
//!
//! The compiled Explorer bundle is embedded by `build.rs`. Runtime model data is
//! generated in memory from the already-validated registry and served beside the
//! embedded bundle.

use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::html::store::{build_project_store, project_store_javascript};
use crate::semantic_contract::build_semantic_index;

include!(concat!(env!("OUT_DIR"), "/explorer_bundle_manifest.rs"));

pub struct ExplorerRuntimeAssets {
    pub project_store_js: String,
    pub ontologies_ttl: String,
}

pub fn build_runtime_assets(
    registry: &GraphRegistry,
) -> Result<ExplorerRuntimeAssets, ReqvireError> {
    let semantic_index = build_semantic_index(registry);
    let project_store = build_project_store(registry, &semantic_index);
    let project_store_js = project_store_javascript(&project_store)?;
    let ontologies_ttl = semantic_index.to_turtle_string()?;

    Ok(ExplorerRuntimeAssets {
        project_store_js,
        ontologies_ttl,
    })
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
    fs::create_dir_all(store_js_path.parent().unwrap())
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
    use std::path::Path;

    let output_dir = output_dir
        .canonicalize()
        .unwrap_or_else(|_| output_dir.to_path_buf());
    copy_workspace_assets_from_dir(Path::new("."), Path::new("."), &output_dir)
}

fn copy_workspace_assets_from_dir(
    root: &std::path::Path,
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
            copy_workspace_assets_from_dir(root, &path, output_dir)?;
            continue;
        }

        if !path.is_file() || !is_workspace_asset_path(&path) {
            continue;
        }

        let rel = path.strip_prefix(root).map_err(|e| {
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

fn is_workspace_asset_path(path: &std::path::Path) -> bool {
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
