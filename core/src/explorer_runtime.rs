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
    let ontologies_ttl = semantic_index.to_turtle_string();

    Ok(ExplorerRuntimeAssets {
        project_store_js,
        ontologies_ttl,
    })
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
