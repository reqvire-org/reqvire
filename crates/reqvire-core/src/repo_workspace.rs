//! Compatibility imports for consumers; workspace ownership lives in its crate.
pub use reqvire_workspace::manifest::{
    normalize_url, resolve_revision, selected_manifest, set_manifest_selection,
    WorkspaceComposition as RepoWorkspace, WorkspaceProject as RepoProject,
};
