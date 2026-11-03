//! Native manifests and immutable composition provenance. Loading is offline.
use crate::paths::canonicalize_existing;
use crate::{git::local_text as git_text, Error};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Component, Path, PathBuf},
    sync::OnceLock,
};
use toml_edit::{DocumentMut, Item};

pub const MANIFEST: &str = "reqvire-workspace.toml";
static SELECTION: OnceLock<Option<PathBuf>> = OnceLock::new();
fn failure(message: impl Into<String>) -> Error {
    Error::ProcessError(message.into())
}

pub fn set_manifest_selection(path: Option<PathBuf>) -> Result<(), Error> {
    SELECTION
        .set(path)
        .map_err(|_| failure("Manifest selection is already configured"))
}
pub fn selected_manifest(root: &Path) -> Result<Option<PathBuf>, Error> {
    selected_manifest_at(root, SELECTION.get().and_then(|p| p.as_deref()))
}
pub fn selected_manifest_at(
    root: &Path,
    selection: Option<&Path>,
) -> Result<Option<PathBuf>, Error> {
    if let Some(path) = selection {
        return Ok(Some(relative_path(root, path)?));
    }
    match std::fs::symlink_metadata(root.join(MANIFEST)) {
        Ok(_) => Ok(Some(MANIFEST.into())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceProject {
    pub name: String,
    pub path: PathBuf,
    /// Recorded exact selection; an unresolved setup declaration uses its ref.
    pub revision: String,
    #[serde(default)]
    pub commit: String,
    #[serde(rename = "ref", default)]
    pub tracking_ref: Option<String>,
    pub remote: String,
    pub fetch_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkspaceComposition {
    pub selection: PathBuf,
    pub manifest: PathBuf,
    pub repositories_dir: PathBuf,
    pub projects: Vec<WorkspaceProject>,
    pub source: String,
    pub inputs: BTreeMap<PathBuf, Vec<u8>>,
    /// Legacy selectors exist only while converting or reading an old baseline.
    pub selectors: BTreeMap<PathBuf, PathBuf>,
    pub central_commit: String,
    pub central_repository: String,
}

pub fn relative_path(root: &Path, path: &Path) -> Result<PathBuf, Error> {
    let relative = path.strip_prefix(root).unwrap_or(path);
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(failure(format!(
            "Invalid manifest/project path {}",
            path.display()
        )));
    }
    if relative
        .components()
        .any(|c| c.as_os_str() == ".git" || c.as_os_str() == ".repo")
    {
        return Err(failure(format!(
            "Reserved repository administration path {}",
            path.display()
        )));
    }
    Ok(relative.to_owned())
}

pub fn contained(root: &Path, path: &Path) -> Result<(), Error> {
    let physical = canonicalize_existing(root)?;
    let relative = relative_path(root, path)?;
    let mut check = root.join(relative);
    loop {
        match std::fs::symlink_metadata(&check) {
            Ok(_) => {
                let actual = canonicalize_existing(&check)?;
                if !actual.starts_with(&physical)
                    || actual
                        != physical.join(
                            check
                                .strip_prefix(root)
                                .map_err(|e| failure(e.to_string()))?,
                        )
                {
                    return Err(failure(format!(
                        "Checkout/source symlink is not allowed: {}",
                        check.display()
                    )));
                }
                return Ok(());
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if !check.pop() {
                    return Err(e.into());
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
}

pub fn document(bytes: &str) -> Result<DocumentMut, Error> {
    bytes
        .parse::<DocumentMut>()
        .map_err(|e| failure(format!("Invalid workspace TOML: {e}")))
}
fn field<'a>(item: &'a Item, key: &str) -> Result<&'a str, Error> {
    item.get(key)
        .and_then(Item::as_str)
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| failure(format!("Missing or invalid workspace {key}")))
}
pub fn full_commit(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|b| b.is_ascii_hexdigit())
}

impl WorkspaceComposition {
    pub fn resolve(root: &Path, selection: PathBuf) -> Result<Self, Error> {
        let root = canonicalize_existing(root)?;
        let selection = relative_path(&root, &selection)?;
        contained(&root, &selection)?;
        let source = std::fs::read_to_string(root.join(&selection))?;
        let doc = document(&source)?;
        for (key, _) in doc.iter() {
            if !matches!(key, "version" | "repositories_dir" | "repositories") {
                return Err(failure(format!("Unsupported workspace key {key}: all repository declarations must be in one native manifest")));
            }
        }
        if doc.get("version").and_then(Item::as_integer) != Some(1) {
            return Err(failure(format!(
                "Manifest {}: unsupported or missing workspace version (expected 1)",
                selection.display()
            )));
        }
        let parent = doc
            .get("repositories_dir")
            .and_then(Item::as_str)
            .ok_or_else(|| failure("Missing workspace repositories_dir"))?;
        let repositories_dir = if parent == "." {
            PathBuf::from(".")
        } else {
            relative_path(&root, Path::new(parent))?
        };
        if parent != "." {
            contained(&root, &repositories_dir)?;
        }
        let mut projects: Vec<WorkspaceProject> = Vec::new();
        let inputs = BTreeMap::from([(selection.clone(), source.as_bytes().to_vec())]);
        let entries = match doc.get("repositories") {
            None => Vec::new(),
            Some(Item::ArrayOfTables(tables)) => tables.iter().cloned().map(Item::Table).collect(),
            Some(Item::Value(toml_edit::Value::Array(a))) if a.is_empty() => Vec::new(),
            _ => return Err(failure("Workspace repositories must be an array of tables")),
        };
        for entry in entries {
            let path = relative_path(&root, Path::new(field(&entry, "path")?))?;
            let path = if parent == "." {
                path
            } else {
                repositories_dir.join(path)
            };
            contained(&root, &path)?;
            if projects
                .iter()
                .any(|p| path.starts_with(&p.path) || p.path.starts_with(&path))
            {
                return Err(failure(format!(
                    "Duplicate or overlapping project path {}",
                    path.display()
                )));
            }
            let url = field(&entry, "url")?.to_owned();
            let commit = entry
                .get("commit")
                .and_then(Item::as_str)
                .unwrap_or_default()
                .to_owned();
            let reference = entry.get("ref").and_then(Item::as_str).map(str::to_owned);
            if !commit.is_empty() && !full_commit(&commit) {
                return Err(failure(format!(
                    "Project {} commit {commit} must be a full commit ID",
                    path.display()
                )));
            }
            if commit.is_empty() && reference.as_deref().is_none_or(|r| r.trim().is_empty()) {
                return Err(failure(format!(
                    "Project {} needs commit or ref",
                    path.display()
                )));
            }
            if reference
                .as_deref()
                .is_some_and(|r| r.is_empty() || r.starts_with('-') || r.contains(['\n', '\0']))
            {
                return Err(failure("Invalid project tracking ref"));
            }
            let name = entry
                .get("name")
                .and_then(Item::as_str)
                .unwrap_or(field(&entry, "path")?)
                .to_owned();
            projects.push(WorkspaceProject {
                name,
                path,
                revision: if commit.is_empty() {
                    reference.clone().unwrap_or_default()
                } else {
                    commit.clone()
                },
                commit,
                tracking_ref: reference,
                remote: "origin".into(),
                fetch_url: url,
            });
        }
        projects.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(Self {
            selection: selection.clone(),
            manifest: selection,
            repositories_dir,
            projects,
            source,
            inputs,
            ..Self::default()
        })
    }

    pub fn load(root: &Path, selection: PathBuf) -> Result<Self, Error> {
        Self::load_inner(root, selection.clone())
            .map_err(|e| failure(format!("Workspace manifest {}: {e}", selection.display())))
    }
    fn load_inner(root: &Path, selection: PathBuf) -> Result<Self, Error> {
        let root = canonicalize_existing(root)?;
        let mut view = Self::resolve(&root, selection)?;
        let owner = canonicalize_existing(&PathBuf::from(git_text(
            &root,
            &["rev-parse", "--show-toplevel"],
        )?))?;
        let source = root.join(&view.manifest);
        let source_owner = canonicalize_existing(&PathBuf::from(git_text(
            source.parent().expect("source parent"),
            &["rev-parse", "--show-toplevel"],
        )?))?;
        if source_owner != owner {
            return Err(failure("Native manifest belongs to another repository"));
        }
        view.central_commit = git_text(&root, &["rev-parse", "--verify", "HEAD"])?;
        view.central_repository =
            git_text(&root, &["remote", "get-url", "origin"]).unwrap_or_default();
        for project in &mut view.projects {
            if !full_commit(&project.commit) {
                return Err(failure(format!(
                    "Project {} requires an exact pin; run workspace sync first",
                    project.path.display()
                )));
            }
            let checkout = root.join(&project.path);
            validate_checkout(&checkout, project)?;
            project.remote = matching_remote(&checkout, project)?;
            let commit = resolve_revision(&checkout, project)?;
            if git_text(&checkout, &["rev-parse", "HEAD"])? != commit {
                return Err(failure(format!(
                    "Project {} HEAD differs from selected pin; explicitly sync or freeze first",
                    project.path.display()
                )));
            }
            project.commit = commit;
        }
        Ok(view)
    }

    pub fn manifest_input_paths(&self) -> Vec<PathBuf> {
        self.inputs
            .keys()
            .filter(|p| !p.starts_with(".repo"))
            .cloned()
            .collect()
    }
    pub fn source_manifest(root: &Path, selection: &Path) -> Result<PathBuf, Error> {
        crate::legacy::source_manifest(root, selection)
    }
    pub fn check_selection(&self, root: &Path) -> Result<(), Error> {
        if self.manifest.extension().is_some_and(|e| e == "toml") {
            contained(root, &self.manifest)
        } else {
            crate::legacy::check_selection(self, root)
        }
    }
    pub fn pinned_manifest(&self) -> Result<Vec<u8>, Error> {
        let mut doc = document(&self.source)?;
        if let Some(tables) = doc
            .get_mut("repositories")
            .and_then(Item::as_array_of_tables_mut)
        {
            for table in tables.iter_mut() {
                let relative = table
                    .get("path")
                    .and_then(Item::as_str)
                    .ok_or_else(|| failure("Project path disappeared"))?;
                let path = if self.repositories_dir == Path::new(".") {
                    PathBuf::from(relative)
                } else {
                    self.repositories_dir.join(relative)
                };
                let project = self
                    .projects
                    .iter()
                    .find(|p| p.path == path)
                    .ok_or_else(|| failure("Project disappeared"))?;
                if table.get("commit").and_then(Item::as_str) != Some(&project.commit) {
                    let decor = table
                        .get("commit")
                        .and_then(Item::as_value)
                        .map(|v| v.decor().clone());
                    table["commit"] = toml_edit::value(&project.commit);
                    if let Some(decor) = decor {
                        *table["commit"]
                            .as_value_mut()
                            .expect("commit value")
                            .decor_mut() = decor;
                    }
                }
            }
        }
        Ok(doc.to_string().into_bytes())
    }
}

pub fn validate_checkout(checkout: &Path, project: &WorkspaceProject) -> Result<(), Error> {
    let physical = canonicalize_existing(checkout)
        .map_err(|e| failure(format!("Missing project {}: {e}", project.path.display())))?;
    let owner = canonicalize_existing(&PathBuf::from(git_text(
        checkout,
        &["rev-parse", "--show-toplevel"],
    )?))?;
    if physical != owner || !checkout.join(".git").exists() {
        return Err(failure(format!(
            "Foreign or missing checkout {}",
            checkout.display()
        )));
    }
    matching_remote(checkout, project)?;
    Ok(())
}

pub fn resolve_revision(checkout: &Path, project: &WorkspaceProject) -> Result<String, Error> {
    let reference = if project.commit.is_empty() {
        &project.revision
    } else {
        &project.commit
    };
    git_text(
        checkout,
        &[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{reference}^{{commit}}"),
        ],
    )
}

pub fn normalize_url(value: &str) -> String {
    let value = value.trim_end_matches('/').trim_end_matches(".git");
    if let Some(path) = value.strip_prefix("file://") {
        return path.to_owned();
    }
    let endpoint = if let Some((scheme, rest)) = value.split_once("://") {
        if !matches!(scheme, "http" | "https" | "ssh" | "git") {
            return value.to_owned();
        }
        rest.to_owned()
    } else if let Some((host, path)) = value.split_once(':') {
        if host.contains('/') || host.is_empty() {
            return value.to_owned();
        }
        format!("{host}/{path}")
    } else {
        return value.to_owned();
    };
    let (host, path) = endpoint.split_once('/').unwrap_or((&endpoint, ""));
    format!(
        "{}/{}",
        host.rsplit('@').next().unwrap_or(host).to_ascii_lowercase(),
        path.trim_start_matches('/')
    )
}
pub fn matching_remote(checkout: &Path, project: &WorkspaceProject) -> Result<String, Error> {
    let remotes = git_text(checkout, &["remote"])?;
    let mut matching = Vec::new();
    for remote in remotes.lines() {
        let url = git_text(checkout, &["remote", "get-url", remote])?;
        if normalize_url(&url) == normalize_url(&project.fetch_url) {
            if remote == project.remote {
                return Ok(remote.to_owned());
            }
            matching.push(remote.to_owned());
        } else if remote == project.remote {
            return Err(failure(
                "Project Git remote does not match its manifest identity",
            ));
        }
    }
    match matching.as_slice() {
        [remote] => Ok(remote.clone()),
        _ => Err(failure(
            "Project Git remote identity is missing or ambiguous",
        )),
    }
}
