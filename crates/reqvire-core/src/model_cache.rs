//! In-memory cache for the parsed [`ModelManager`], keyed by a fingerprint of
//! the scanned markdown files and the active build options.
//!
//! Every MCP tool dispatch and many CLI commands call a `load_model*` variant
//! that re-parses the whole workspace from disk. This module memoizes the
//! result so that repeated calls within an unchanged workspace return a clone
//! of the cached model instead of re-reading and re-validating every file.
//!
//! The fingerprint is the set of `(relative_path, size, content hash)` for
//! every `.md` file the scanner would consider, plus the [`ModelBuildOptions`].
//! If any file changes, is added, or is removed, the fingerprint changes and the
//! model is rebuilt. This mirrors the recommendation in `CodeReview.md` (item
//! 2).

use crate::error::ReqvireError;
use crate::model::{ModelBuildOptions, ModelManager};
use crate::{tool_interface, utils};
use globset::GlobSet;
use log::debug;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

/// The cached model together with the fingerprint that produced it.
struct CachedModel {
    key: CacheKey,
    model: ModelManager,
}

/// Cache key: build options plus a sorted map of every scanned file path to a
/// content fingerprint. Two keys are equal only when the same set of files with
/// the same content is observed and the build options match.
#[derive(Clone, Eq, PartialEq)]
struct CacheKey {
    options: ModelBuildOptions,
    workspace_root: PathBuf,
    git_worktrees: Vec<GitWorktreeFingerprint>,
    excluded_patterns: String,
    reqvire_version: &'static str,
    tool_contract_version: &'static str,
    files: BTreeMap<PathBuf, FileFingerprint>,
}

#[derive(Clone, Eq, PartialEq)]
struct GitWorktreeFingerprint {
    workspace_relative_root: PathBuf,
    head: Option<String>,
    dirty: bool,
}

#[derive(Clone, Eq, PartialEq)]
struct FileFingerprint {
    len: u64,
    content_hash: String,
}

static MODEL_CACHE: Mutex<Option<CachedModel>> = Mutex::new(None);

/// Builds (or reuses) a [`ModelManager`] for the current workspace.
///
/// On a cache hit the stored model is cloned and returned, avoiding a full
/// re-parse. On a miss the workspace is parsed and validated with the supplied
/// `options`, the result is cached, and a clone is returned.
///
/// This cache only covers the current working tree. Git-commit scans still use
/// [`crate::model::ModelManager::parse_and_validate`] directly.
pub fn load_cached_model(
    excluded_filename_patterns: &GlobSet,
    options: ModelBuildOptions,
) -> Result<ModelManager, ReqvireError> {
    let workspace_scope = crate::workspace::WorkspaceScope::discover()?;
    // Compute the fingerprint by scanning the same files the parser would.
    let files = utils::scan_markdown_files(None, excluded_filename_patterns)?;
    let mut fingerprint = BTreeMap::new();
    for path in &files {
        let content = std::fs::read_to_string(path)?;
        let relative_path = utils::get_relative_path(path)?;
        fingerprint.insert(
            relative_path,
            FileFingerprint {
                len: content.len() as u64,
                content_hash: utils::hash_content(&content),
            },
        );
    }
    let key = CacheKey {
        options,
        workspace_root: workspace_scope.root.clone(),
        git_worktrees: git_worktree_fingerprints(&workspace_scope),
        excluded_patterns: format!("{:?}", excluded_filename_patterns),
        reqvire_version: env!("CARGO_PKG_VERSION"),
        tool_contract_version: tool_interface::TOOL_CONTRACT_VERSION,
        files: fingerprint,
    };

    // Cache hit: return a clone without re-parsing.
    {
        let cache = MODEL_CACHE.lock().expect("model cache mutex poisoned");
        if let Some(cached) = cache.as_ref() {
            if cached.key == key {
                debug!(
                    "model cache hit ({} files, lenient={}, size_estimates={})",
                    cached.key.files.len(),
                    options.lenient,
                    options.with_size_estimates
                );
                return Ok(cached.model.clone());
            }
        }
    }

    debug!(
        "model cache miss ({} files, lenient={}, size_estimates={})",
        key.files.len(),
        options.lenient,
        options.with_size_estimates,
    );

    // Cache miss: rebuild outside the lock to avoid holding it during I/O.
    let mut model = ModelManager::new();
    model.parse_and_validate_with_options(None, excluded_filename_patterns, options)?;
    let entry = CachedModel {
        key,
        model: model.clone(),
    };
    let mut cache = MODEL_CACHE.lock().expect("model cache mutex poisoned");
    *cache = Some(entry);
    Ok(model)
}

fn git_worktree_fingerprints(
    scope: &crate::workspace::WorkspaceScope,
) -> Vec<GitWorktreeFingerprint> {
    scope
        .git_worktrees
        .iter()
        .map(|root| {
            let workspace_relative_root = root
                .strip_prefix(&scope.root)
                .ok()
                .map(PathBuf::from)
                .filter(|path| !path.as_os_str().is_empty())
                .unwrap_or_else(|| PathBuf::from("."));
            let head = git_output_in_dir(root, ["rev-parse", "HEAD"]);
            let dirty = git_output_in_dir(root, ["status", "--porcelain"])
                .as_ref()
                .is_some_and(|status| !status.trim().is_empty());
            GitWorktreeFingerprint {
                workspace_relative_root,
                head,
                dirty,
            }
        })
        .collect()
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

/// Clears any cached model, forcing the next `load_cached_model` call to
/// rebuild. Useful when an external mutation may have invalidated the cache
/// outside of the mtime fingerprint (for example after a CRUD write).
pub fn invalidate() {
    let mut cache = MODEL_CACHE.lock().expect("model cache mutex poisoned");
    *cache = None;
}
