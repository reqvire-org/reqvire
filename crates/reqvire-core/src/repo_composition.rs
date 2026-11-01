//! Versioned root-to-manifest revision associations. XML remains in its owner.
use crate::{
    error::ReqvireError,
    git_commands::{local_bytes as git_bytes, local_text as git_text, run_local, GitReader},
    repo_workspace::{normalize_url, RepoWorkspace},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path, PathBuf},
};

pub const RECORD_PATH: &str = ".reqvire-composition.json";

pub fn owner_cache_key(repository: &str, owner: &Path) -> String {
    crate::utils::hash_content(&if repository.is_empty() {
        format!("local:{}", owner.display())
    } else {
        normalize_url(repository)
    })
}

fn failure(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(format!("Workspace composition: {}", message.into()))
}

fn relative(path: &Path, empty: bool) -> Result<(), ReqvireError> {
    if (!empty && path.as_os_str().is_empty())
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(failure(format!("invalid relative path {}", path.display())));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestSelection {
    pub path: PathBuf,
    pub owner: PathBuf,
    pub source: PathBuf,
    pub repository: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompositionRecord {
    pub schema_version: u32,
    pub selection: PathBuf,
    pub manifest: PathBuf,
    pub sources: Vec<ManifestSelection>,
}

impl CompositionRecord {
    pub fn validate(&self) -> Result<(), ReqvireError> {
        if self.schema_version != 1 || self.sources.is_empty() {
            return Err(failure(
                "unsupported schema or empty manifest source inventory",
            ));
        }
        relative(&self.selection, false)?;
        relative(&self.manifest, false)?;
        let mut paths = BTreeSet::new();
        let mut owners = BTreeMap::new();
        for source in &self.sources {
            relative(&source.path, false)?;
            relative(&source.owner, true)?;
            relative(&source.source, false)?;
            if source.owner.join(&source.source) != source.path
                || !paths.insert(source.path.clone())
                || source.path.components().any(|c| c.as_os_str() == ".git")
            {
                return Err(failure(format!(
                    "invalid or duplicate manifest ownership for {}",
                    source.path.display()
                )));
            }
            match (&source.commit, source.owner.as_os_str().is_empty()) {
                (None, true) => {}
                (Some(commit), false)
                    if matches!(commit.len(), 40 | 64)
                        && commit.bytes().all(|byte| byte.is_ascii_hexdigit()) => {}
                _ => {
                    return Err(failure(format!(
                        "{} requires an exact non-root owner commit, or the containing root tree",
                        source.path.display()
                    )))
                }
            }
            if let Some(previous) =
                owners.insert(&source.owner, (&source.repository, &source.commit))
            {
                if previous != (&source.repository, &source.commit) {
                    return Err(failure(format!(
                        "inconsistent revision identity for owner {}",
                        source.owner.display()
                    )));
                }
            }
        }
        if !paths.contains(&self.manifest) {
            return Err(failure(format!(
                "selected manifest {} is absent from the source inventory",
                self.manifest.display()
            )));
        }
        Ok(())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ReqvireError> {
        let record: Self = serde_json::from_slice(bytes)
            .map_err(|e| failure(format!("invalid {RECORD_PATH}: {e}")))?;
        record.validate()?;
        Ok(record)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, ReqvireError> {
        self.validate()?;
        let mut record = self.clone();
        record.sources.sort_by(|a, b| a.path.cmp(&b.path));
        let mut bytes = serde_json::to_vec_pretty(&record)?;
        bytes.push(b'\n');
        Ok(bytes)
    }

    /// Capture accepted ownership, rather than re-resolving a live composition.
    pub fn capture(workspace: &RepoWorkspace) -> Result<Self, ReqvireError> {
        let sources = workspace
            .manifest_sources
            .iter()
            .map(|(path, source)| {
                if !source.tracked {
                    return Err(failure(format!(
                        "manifest source {} is unversioned; commit it in its actual owner before checkpointing",
                        path.display()
                    )));
                }
                Ok(ManifestSelection {
                    path: path.clone(),
                    source: path
                        .strip_prefix(&source.root)
                        .map_err(|e| failure(e.to_string()))?
                        .to_owned(),
                    owner: source.root.clone(),
                    repository: source.repository.clone(),
                    commit: (!source.root.as_os_str().is_empty()).then(|| source.commit.clone()),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let record = Self {
            schema_version: 1,
            selection: workspace.selection.clone(),
            manifest: workspace.manifest.clone(),
            sources,
        };
        record.validate()?;
        Ok(record)
    }

    pub fn at_commit(
        root: &Path,
        commit: &str,
        selection: &Path,
    ) -> Result<Option<Self>, ReqvireError> {
        let present = git_text(root, &["ls-tree", "--name-only", commit, "--", RECORD_PATH])?;
        if present.is_empty() {
            return Ok(None);
        }
        let record = Self::from_bytes(&git_bytes(
            root,
            &["show", &format!("{commit}:{RECORD_PATH}")],
        )?)?;
        Ok((record.selection == selection).then_some(record))
    }

    pub fn matches_workspace(&self, workspace: &RepoWorkspace) -> Result<(), ReqvireError> {
        let mut selected = self.clone();
        let mut observed = Self::capture(workspace)?;
        for source in selected
            .sources
            .iter_mut()
            .chain(observed.sources.iter_mut())
        {
            source.repository = normalize_url(&source.repository);
        }
        selected.sources.sort_by(|a, b| a.path.cmp(&b.path));
        observed.sources.sort_by(|a, b| a.path.cmp(&b.path));
        if selected != observed {
            return Err(failure("committed composition association differs from the prepared manifest-owner revisions; update pins and checkpoint their owner revisions before admission"));
        }
        Ok(())
    }

    /// Find immutable local objects by recorded identity; never use a remote as
    /// object storage or import a live XML file as historical content.
    pub fn object_source(
        &self,
        root: &Path,
        current: &RepoWorkspace,
        source: &ManifestSelection,
    ) -> Result<PathBuf, ReqvireError> {
        if source.owner.as_os_str().is_empty() {
            return Ok(root.to_owned());
        }
        let commit = source.commit.as_deref().expect("validated manifest commit");
        let mut candidates = vec![root.join(&source.owner)];
        if !source.repository.is_empty() {
            candidates.extend(
                current
                    .manifest_sources
                    .values()
                    .map(|s| root.join(&s.root)),
            );
            candidates.push(root.to_owned());
        }
        if let Ok(common) = git_text(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        ) {
            candidates.push(
                PathBuf::from(common)
                    .join("reqvire-manifest-objects")
                    .join(owner_cache_key(&source.repository, &source.owner)),
            );
        }
        let mut seen = BTreeSet::new();
        for candidate in candidates {
            if !candidate.is_dir() || !seen.insert(candidate.clone()) {
                continue;
            }
            if !source.repository.is_empty() {
                let identity =
                    git_text(&candidate, &["remote", "get-url", "origin"]).unwrap_or_default();
                if normalize_url(&identity) != normalize_url(&source.repository) {
                    continue;
                }
            }
            if git_text(
                &candidate,
                &[
                    "rev-parse",
                    "--verify",
                    "--end-of-options",
                    &format!("{commit}^0"),
                ],
            )
            .is_ok_and(|resolved| resolved.eq_ignore_ascii_case(commit))
            {
                return Ok(candidate);
            }
        }
        Err(failure(format!(
            "repo_impact_manifest_history_unavailable: {} owner {} commit {} is unavailable locally; restore its recorded objects/composition association; no fetch was attempted",
            source.path.display(), source.owner.display(), commit
        )))
    }

    pub fn historical_inputs(
        &self,
        root: &Path,
        root_commit: &str,
        current: &RepoWorkspace,
    ) -> Result<BTreeMap<PathBuf, Vec<u8>>, ReqvireError> {
        self.validate()?;
        let mut inputs = BTreeMap::new();
        let mut owners: BTreeMap<PathBuf, Vec<&ManifestSelection>> = BTreeMap::new();
        for source in &self.sources {
            owners.entry(source.owner.clone()).or_default().push(source);
        }
        for selections in owners.values() {
            let first = selections[0];
            let objects = self.object_source(root, current, first)?;
            let commit = first.commit.as_deref().unwrap_or(root_commit);
            let paths = selections
                .iter()
                .map(|source| objects.join(&source.source))
                .collect();
            let files = GitReader::new(&objects, |args, input| run_local(&objects, args, input))
                .regular_files_at_commit(commit, &paths)
                .map_err(|e| {
                    failure(format!(
                        "manifest owner {} at {commit}: {e}",
                        first.owner.display()
                    ))
                })?;
            for source in selections {
                let file = files.get(&objects.join(&source.source)).ok_or_else(|| {
                    failure(format!(
                        "manifest {} is absent at {commit}",
                        source.path.display()
                    ))
                })?;
                inputs.insert(source.path.clone(), file.bytes.as_ref().clone());
            }
        }
        Ok(inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record() -> CompositionRecord {
        CompositionRecord {
            schema_version: 1,
            selection: ".repo/manifest.xml".into(),
            manifest: ".repo/manifests/default.xml".into(),
            sources: vec![ManifestSelection {
                path: ".repo/manifests/default.xml".into(),
                owner: ".repo/manifests".into(),
                source: "default.xml".into(),
                repository: "https://github.com/test/manifests".into(),
                commit: Some("a".repeat(40)),
            }],
        }
    }

    #[test]
    fn association_round_trip_contains_references_without_xml_or_self_commit() {
        let record = record();
        let bytes = record.to_bytes().expect("serialize association");
        assert_eq!(
            CompositionRecord::from_bytes(&bytes).expect("parse association"),
            record
        );
        assert!(!String::from_utf8(bytes)
            .expect("UTF-8 JSON")
            .contains("<manifest"));
    }

    #[test]
    fn rejects_escape_duplicate_owner_mismatch_and_mutable_revision() {
        for variant in 0..5 {
            let mut record = record();
            match variant {
                0 => record.sources[0].source = "../default.xml".into(),
                1 => record.sources.push(record.sources[0].clone()),
                2 => record.sources[0].owner = "repositories/foreign".into(),
                3 => record.sources[0].commit = Some("main".into()),
                _ => record.schema_version = 2,
            }
            assert!(
                record.validate().is_err(),
                "accepted invalid variant {variant}"
            );
        }
    }

    #[test]
    fn root_owned_source_uses_containing_tree_without_circular_commit() {
        let mut record = record();
        record.manifest = "manifest.xml".into();
        record.selection = record.manifest.clone();
        record.sources[0].path = record.manifest.clone();
        record.sources[0].source = record.manifest.clone();
        record.sources[0].owner = PathBuf::new();
        record.sources[0].commit = None;
        assert!(record.validate().is_ok());
        record.sources[0].commit = Some("a".repeat(40));
        assert!(record.validate().is_err());
    }
}
