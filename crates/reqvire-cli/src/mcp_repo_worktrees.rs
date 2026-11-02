//! MCP error adapter; branch-set ownership is in the workspace crate.
use reqvire::{error::ReqvireError, repo_workspace::RepoWorkspace};
use std::path::Path;
pub fn prepare(
    source: &Path,
    target: &Path,
    common: &Path,
    manifest: &Path,
    creating: bool,
) -> Result<(), ReqvireError> {
    reqvire_workspace::branches::prepare(source, target, common, manifest, creating)
        .map_err(Into::into)
}
pub fn retain(root: &Path, common: &Path, manifest: &Path) -> Result<RepoWorkspace, ReqvireError> {
    reqvire_workspace::branches::retain(root, common, manifest).map_err(Into::into)
}
pub fn remove_checkouts(root: &Path, view: &RepoWorkspace) -> Result<(), ReqvireError> {
    reqvire_workspace::branches::remove_checkouts(root, view).map_err(Into::into)
}
