//! Manifest selection and repository provenance shared by every model consumer.
//!
//! Repo owns manifest composition. The default entry or an alternate file
//! location selects the inputs. This boundary never syncs or fetches.
use crate::{error::ReqvireError, git_commands::local_text as git_text, model_inputs};
use quick_xml::{
    events::{BytesStart, Event},
    Reader, Writer,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    sync::OnceLock,
};

static MANIFEST: OnceLock<Option<PathBuf>> = OnceLock::new();

pub fn set_manifest_selection(path: Option<PathBuf>) -> Result<(), ReqvireError> {
    MANIFEST
        .set(path)
        .map_err(|_| failure("Manifest selection is already configured"))
}

fn failure(message: impl Into<String>) -> ReqvireError {
    ReqvireError::ProcessError(message.into())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoProject {
    pub name: String,
    pub path: PathBuf,
    pub revision: String,
    #[serde(default)]
    pub commit: String,
    pub remote: String,
    pub fetch_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoWorkspace {
    pub selection: PathBuf,
    pub manifest: PathBuf,
    pub projects: Vec<RepoProject>,
    pub resolved_xml: String,
    pub inputs: BTreeMap<PathBuf, Vec<u8>>,
    /// Administrative selectors mapped to tracked root source paths.
    pub selectors: BTreeMap<PathBuf, PathBuf>,
    pub central_commit: String,
    pub central_repository: String,
}

pub fn selected_manifest(root: &Path) -> Result<Option<PathBuf>, ReqvireError> {
    selected_manifest_at(
        root,
        MANIFEST.get().and_then(|selection| selection.as_deref()),
    )
}

fn selected_manifest_at(
    root: &Path,
    selection: Option<&Path>,
) -> Result<Option<PathBuf>, ReqvireError> {
    if let Some(path) = selection {
        return Ok(Some(relative_path(root, path)?));
    }
    let entry = root.join(".repo/manifest.xml");
    if !entry.exists() {
        if root.join(".repo").exists() {
            return Err(failure(format!(
                "Repo manifest {} is unavailable",
                entry.display()
            )));
        }
        return Ok(None);
    }
    Ok(Some(PathBuf::from(".repo/manifest.xml")))
}

fn relative_path(root: &Path, path: &Path) -> Result<PathBuf, ReqvireError> {
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
    Ok(relative.to_owned())
}

fn source_path(root: &Path, path: &Path) -> Result<PathBuf, ReqvireError> {
    let path = relative_path(root, path)?;
    if path
        .components()
        .any(|part| part.as_os_str() == ".repo" || part.as_os_str() == ".git")
    {
        return Err(failure(format!(
            "Manifest source {} is repository administration, not a tracked root source",
            path.display()
        )));
    }
    Ok(path)
}

fn overlay_entries(root: &Path) -> Result<Vec<PathBuf>, ReqvireError> {
    let directory = root.join(".repo/local_manifests");
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut entries = std::fs::read_dir(directory)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == "xml"))
        .map(|path| relative_path(root, &path))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    Ok(entries)
}

fn observe_selector(
    root: &Path,
    path: &Path,
    inputs: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> Result<(), ReqvireError> {
    let entry = root.join(path);
    // Guard symlink target identity through selectors, and source content at
    // its root path. Duplicate alias bytes would mistake our pin update for drift.
    if !std::fs::symlink_metadata(&entry)?.file_type().is_symlink() {
        inputs.insert(
            path.to_owned(),
            model_inputs::read_to_string(&entry)?.into_bytes(),
        );
    }
    Ok(())
}

type Attributes = BTreeMap<String, String>;
fn attributes(tag: &BytesStart<'_>) -> Result<Attributes, ReqvireError> {
    tag.attributes()
        .map(|a| {
            let a = a.map_err(|e| failure(e.to_string()))?;
            Ok((
                String::from_utf8_lossy(a.key.as_ref()).into_owned(),
                a.unescape_value()
                    .map_err(|e| failure(e.to_string()))?
                    .into_owned(),
            ))
        })
        .collect()
}
fn attr<'a>(attrs: &'a Attributes, name: &str) -> Result<&'a str, ReqvireError> {
    attrs
        .get(name)
        .map(String::as_str)
        .ok_or_else(|| failure(format!("Missing manifest {name}")))
}
fn tags(xml: &str) -> Result<Vec<(String, Attributes)>, ReqvireError> {
    let mut result = Vec::new();
    let mut reader = Reader::from_str(xml);
    loop {
        match reader.read_event().map_err(|e| failure(e.to_string()))? {
            Event::Start(tag) | Event::Empty(tag) => result.push((
                String::from_utf8_lossy(tag.name().as_ref()).into_owned(),
                attributes(&tag)?,
            )),
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(result)
}

/// Stage only authoritative manifest inputs. Repo resolves their relative
/// includes against this isolated source root, never its cached checkout.
fn stage_manifest(
    root: &Path,
    path: &Path,
    temporary: &Path,
    inputs: &mut BTreeMap<PathBuf, Vec<u8>>,
    visiting: &mut Vec<PathBuf>,
) -> Result<PathBuf, ReqvireError> {
    let relative = source_path(root, path)?;
    if visiting.contains(&relative) {
        return Err(failure(format!(
            "Recursive manifest include {}",
            relative.display()
        )));
    }
    let source = root.join(&relative);
    if source
        .canonicalize()
        .map_err(|e| failure(format!("Manifest source {}: {e}", relative.display())))?
        != root.canonicalize()?.join(&relative)
    {
        return Err(failure(format!(
            "Manifest symlink is not allowed: {}",
            source.display()
        )));
    }
    let text = model_inputs::read_to_string(&source)
        .map_err(|e| failure(format!("Manifest source {}: {e}", relative.display())))?;
    inputs.insert(relative.clone(), text.as_bytes().to_vec());
    visiting.push(relative.clone());
    let destination = temporary.join(&relative);
    let mut reader = Reader::from_str(&text);
    let mut writer = Writer::new(Vec::new());
    loop {
        let event = reader.read_event().map_err(|e| failure(e.to_string()))?;
        if matches!(event, Event::Eof) {
            break;
        }
        let rewritten = match &event {
            Event::Start(tag) | Event::Empty(tag) if tag.name().as_ref() == b"include" => {
                let mut attrs = attributes(tag)?;
                let name = attr(&attrs, "name")?.to_owned();
                let included = source_path(root, Path::new(&name))?;
                stage_manifest(root, &included, temporary, inputs, visiting)?;
                attrs.insert("name".into(), included.to_string_lossy().into_owned());
                let mut tag = BytesStart::new("include");
                for (key, value) in &attrs {
                    tag.push_attribute((key.as_str(), value.as_str()));
                }
                if matches!(event, Event::Empty(_)) {
                    Event::Empty(tag)
                } else {
                    Event::Start(tag)
                }
            }
            _ => event,
        };
        writer
            .write_event(rewritten)
            .map_err(|e| failure(e.to_string()))?;
    }
    visiting.pop();
    std::fs::create_dir_all(
        destination
            .parent()
            .ok_or_else(|| failure("Manifest has no parent"))?,
    )?;
    std::fs::write(&destination, writer.into_inner())?;
    Ok(destination)
}

impl RepoWorkspace {
    /// Repo's entry selects a filename, not a second revision authority. The
    /// corresponding tracked file in the root branch is the authored source.
    pub fn source_manifest(root: &Path, selection: &Path) -> Result<PathBuf, ReqvireError> {
        if !selection.starts_with(".repo") {
            return source_path(root, selection);
        }
        if let Ok(path) = selection.strip_prefix(".repo/manifests") {
            return source_path(root, path);
        }
        let overlay = selection.starts_with(".repo/local_manifests");
        if selection != Path::new(".repo/manifest.xml") && !overlay {
            return Err(failure(format!(
                "Unsupported Repo entry {}",
                selection.display()
            )));
        }
        let entry = root.join(selection);
        if std::fs::symlink_metadata(&entry)?.file_type().is_symlink() {
            let target = entry.canonicalize()?;
            let owner = if overlay {
                root.canonicalize()?
            } else {
                root.canonicalize()?.join(".repo/manifests")
            };
            return target
                .strip_prefix(owner)
                .map_err(|_| failure("Repo entry does not select its manifest source"))
                .and_then(|path| source_path(root, path));
        }
        let xml = model_inputs::read_to_string(&entry)?;
        let includes: Vec<_> = tags(&xml)?
            .into_iter()
            .filter(|(name, _)| name == "include")
            .collect();
        if includes.len() != 1
            || tags(&xml)?
                .iter()
                .any(|(name, _)| name != "manifest" && name != "include")
        {
            return Err(failure(
                "Repo entry must identify one authoritative source manifest",
            ));
        }
        source_path(root, Path::new(attr(&includes[0].1, "name")?))
    }

    pub fn check_selection(&self, root: &Path) -> Result<(), ReqvireError> {
        let accepted: Vec<_> = self
            .selectors
            .keys()
            .filter(|entry| entry.starts_with(".repo/local_manifests"))
            .cloned()
            .collect();
        if overlay_entries(root)? != accepted {
            return Err(failure("Repo local manifest selection changed externally"));
        }
        for (entry, source) in &self.selectors {
            if Self::source_manifest(root, entry)? != *source {
                return Err(failure(format!(
                    "Manifest source selection {} changed externally",
                    entry.display()
                )));
            }
        }
        Ok(())
    }

    /// Reconstruct disposable selectors only. Authored XML stays in root Git.
    pub fn write_selectors(&self, root: &Path) -> Result<(), ReqvireError> {
        let mut entries = self.selectors.clone();
        entries.insert(PathBuf::from(".repo/manifest.xml"), self.manifest.clone());
        for (entry, source) in entries
            .iter()
            .filter(|(entry, _)| !entry.starts_with(".repo/manifests"))
        {
            let path = root.join(entry);
            std::fs::create_dir_all(
                path.parent()
                    .ok_or_else(|| failure("Manifest entry has no parent"))?,
            )?;
            std::fs::write(
                path,
                format!(
                    "<manifest><include name=\"{}\"/></manifest>\n",
                    quick_xml::escape::escape(source.to_string_lossy())
                ),
            )?;
        }
        Ok(())
    }

    pub fn resolve(root: &Path, manifest: PathBuf) -> Result<Self, ReqvireError> {
        Self::resolve_with_repo(root, manifest, &root.join(".repo/repo"))
    }

    /// Resolve isolated historical inputs through the same initialized Repo
    /// implementation, without copying or changing the client's administration.
    pub(crate) fn resolve_with_repo(
        root: &Path,
        manifest: PathBuf,
        repo_code: &Path,
    ) -> Result<Self, ReqvireError> {
        // Normalize the workspace prefix once, including macOS temporary
        // directories whose /var spelling resolves under /private/var.
        let root = &root.canonicalize()?;
        let load = || {
            if !repo_code.join("main.py").is_file() {
                return Err(failure("Initialized local Repo metadata is unavailable; explicitly initialize the Repo client before selecting its manifest"));
            }
            let temporary = tempfile::tempdir()?;
            let staged_sources = temporary.path().join("sources");
            let mut inputs = BTreeMap::new();
            let mut selectors = BTreeMap::new();
            let native = manifest.starts_with(".repo");
            let source_manifest = Self::source_manifest(root, &manifest)?;
            let mut selected = stage_manifest(
                root,
                &source_manifest,
                &staged_sources,
                &mut inputs,
                &mut Vec::new(),
            )?;
            if native {
                // Protect source-selection state, but never import cached XML.
                if !manifest.starts_with(".repo/manifests") {
                    observe_selector(root, &manifest, &mut inputs)?;
                }
                selectors.insert(manifest.clone(), source_manifest.clone());
            }
            let entries = overlay_entries(root)?;
            if !entries.is_empty() {
                let mut includes = vec![selected];
                for entry in entries {
                    if entry == manifest {
                        continue;
                    }
                    let source = Self::source_manifest(root, &entry).map_err(|e| {
                        failure(format!(
                            "Manifest input {} has no unambiguous versioned root source: {e}",
                            entry.display()
                        ))
                    })?;
                    includes.push(stage_manifest(
                        root,
                        &source,
                        &staged_sources,
                        &mut inputs,
                        &mut Vec::new(),
                    )?);
                    observe_selector(root, &entry, &mut inputs)?;
                    selectors.insert(entry, source);
                }
                selected = temporary.path().join("combined.xml");
                let mut xml = String::from("<manifest>");
                for path in includes {
                    xml.push_str(&format!(
                        "<include name=\"{}\"/>",
                        quick_xml::escape::escape(
                            path.strip_prefix(&staged_sources)
                                .expect("staged manifest source")
                                .to_string_lossy()
                        )
                    ));
                }
                xml.push_str("</manifest>\n");
                std::fs::write(&selected, xml)?;
            }
            // Use the initialized Repo client's resolved Project objects, rather
            // than reimplementing URL joins, aliases or revision precedence.
            let out = Command::new("python3")
                .current_dir(root)
                .env("GIT_TERMINAL_PROMPT", "0")
                .env("PYTHONDONTWRITEBYTECODE", "1")
                .stdin(Stdio::null())
                .args(["-c", REPO_RESOLUTION])
                .arg(root.join(".repo"))
                .arg(&selected)
                .arg(root.join(&manifest))
                .arg(repo_code)
                .arg(&staged_sources)
                .output()?;
            if !out.status.success() {
                return Err(failure(format!(
                    "Repo resolution failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                )));
            }
            let output: ResolvedManifest = serde_json::from_slice(&out.stdout)
                .map_err(|e| failure(format!("Invalid Repo resolution: {e}")))?;
            for (path, bytes) in &inputs {
                if model_inputs::read_to_string(&root.join(path))?.as_bytes() != bytes {
                    return Err(failure(format!(
                        "Manifest changed during resolution: {}",
                        path.display()
                    )));
                }
            }
            let config = PathBuf::from(".repo/manifests.git/config");
            inputs.insert(
                config.clone(),
                model_inputs::read_to_string(&root.join(config))?.into_bytes(),
            );
            let resolved_xml = output.xml;
            if tags(&resolved_xml)?
                .iter()
                .any(|(name, _)| name == "submanifest")
            {
                return Err(failure("Repo submanifests are not supported; declare the component projects in the root source manifest"));
            }
            let mut projects: Vec<RepoProject> = Vec::new();
            for mut project in output.projects {
                let path = relative_path(root, &project.path)?;
                if path
                    .components()
                    .any(|c| matches!(c, Component::Normal(n) if n == ".repo" || n == ".git"))
                {
                    return Err(failure(format!("Reserved project path {}", path.display())));
                }
                if projects
                    .iter()
                    .any(|p| path.starts_with(&p.path) || p.path.starts_with(&path))
                {
                    return Err(failure(format!(
                        "Duplicate or overlapping project path {}",
                        path.display()
                    )));
                }
                project.path = path;
                projects.push(project);
            }
            projects.sort_by(|a, b| a.path.cmp(&b.path));
            Ok(Self {
                selection: manifest.clone(),
                manifest: source_manifest,
                projects,
                resolved_xml,
                inputs,
                selectors,
                central_commit: String::new(),
                central_repository: String::new(),
            })
        };
        load().map_err(|e| failure(format!("Manifest {}: {e}", manifest.display())))
    }

    pub fn load(root: &Path, manifest: PathBuf) -> Result<Self, ReqvireError> {
        let root = &root.canonicalize()?;
        let mut resolved = Self::resolve(root, manifest)?;
        resolved.central_commit = git_text(root, &["rev-parse", "--verify", "HEAD"])?;
        resolved.central_repository =
            git_text(root, &["remote", "get-url", "origin"]).unwrap_or_default();
        {
            let identity = git_text(
                &root.join(".repo/manifests"),
                &["remote", "get-url", "origin"],
            )?;
            if resolved.central_repository.is_empty()
                || normalize_url(&identity) != normalize_url(&resolved.central_repository)
            {
                return Err(failure(format!("Manifest {}: Repo manifest repository identity differs from its owning workspace root", resolved.manifest.display())));
            }
        }
        for path in resolved.manifest_input_paths() {
            let source = root.join(&path);
            let owner = PathBuf::from(git_text(
                source.parent().unwrap_or(root),
                &["rev-parse", "--show-toplevel"],
            )?);
            let relative = source
                .strip_prefix(&owner)
                .map_err(|e| failure(e.to_string()))?;
            let tracked = git_text(
                &owner,
                &[
                    "ls-files",
                    "--error-unmatch",
                    "--",
                    &relative.to_string_lossy(),
                ],
            )
            .is_ok();
            if owner != *root || !tracked {
                return Err(failure(format!(
                    "Manifest source {} must be tracked in the owning root repository",
                    path.display()
                )));
            }
        }
        for project in &mut resolved.projects {
            let mut check = || {
                let checkout = root.join(&project.path);
                if checkout.canonicalize()? != checkout || !checkout.join(".git").exists() {
                    return Err(failure(
                        "Project is missing, escapes its path or is not a checkout",
                    ));
                }
                project.remote = matching_remote(&checkout, project)?;
                let commit = resolve_revision(&checkout, project)?;
                if git_text(&checkout, &["rev-parse", "HEAD"])? != commit {
                    return Err(failure("Project HEAD differs from the selected revision; explicitly sync the checkout first"));
                }
                Ok(commit)
            };
            project.commit = check().map_err(|e| {
                failure(format!(
                    "Manifest {}, project {} ({}): {e}",
                    resolved.manifest.display(),
                    project.name,
                    project.path.display()
                ))
            })?;
        }
        Ok(resolved)
    }

    pub(crate) fn manifest_input_paths(&self) -> Vec<PathBuf> {
        self.inputs
            .keys()
            .filter(|path| !path.starts_with(".repo"))
            .cloned()
            .collect()
    }

    /// Canonical Repo export preserves the resolved composition. Change only
    /// project revision attributes, including unchanged projects' exact commits.
    pub fn pinned_xml(&self) -> Result<Vec<u8>, ReqvireError> {
        let mut reader = Reader::from_str(&self.resolved_xml);
        let mut writer = Writer::new(Vec::new());
        loop {
            let event = reader.read_event().map_err(|e| failure(e.to_string()))?;
            if matches!(event, Event::Eof) {
                break;
            }
            match &event {
                Event::Start(tag) | Event::Empty(tag) if tag.name().as_ref() == b"project" => {
                    let mut attrs = attributes(tag)?;
                    let name = attr(&attrs, "name")?;
                    let path = attrs.get("path").map(String::as_str).unwrap_or(name);
                    let project = self
                        .projects
                        .iter()
                        .find(|p| p.name == name && p.path == Path::new(path))
                        .ok_or_else(|| failure("Resolved project disappeared"))?;
                    attrs.insert("revision".into(), project.commit.clone());
                    let mut updated = BytesStart::new("project");
                    for (key, value) in &attrs {
                        updated.push_attribute((key.as_str(), value.as_str()));
                    }
                    writer.write_event(if matches!(event, Event::Empty(_)) {
                        Event::Empty(updated)
                    } else {
                        Event::Start(updated)
                    })?;
                }
                _ => writer.write_event(event)?,
            }
        }
        Ok(writer.into_inner())
    }

    /// Update native project and revision-overlay declarations in place, keeping
    /// Repo's include and local-manifest composition instead of flattening it.
    pub fn pinned_inputs(&self) -> Result<BTreeMap<PathBuf, Vec<u8>>, ReqvireError> {
        let mut changed = BTreeMap::new();
        for (path, bytes) in &self.inputs {
            if path.starts_with(".repo") {
                continue;
            }
            let mut reader = Reader::from_reader(bytes.as_slice());
            let mut writer = Writer::new(Vec::new());
            loop {
                let event = reader.read_event().map_err(|e| failure(e.to_string()))?;
                if matches!(event, Event::Eof) {
                    break;
                }
                let event = match &event {
                    Event::Start(tag) | Event::Empty(tag)
                        if matches!(tag.name().as_ref(), b"project" | b"extend-project") =>
                    {
                        let mut attrs = attributes(tag)?;
                        let name = attr(&attrs, "name")?;
                        let candidates: Vec<_> =
                            self.projects.iter().filter(|p| p.name == name).collect();
                        let selected = attrs.get("dest-path").or_else(|| attrs.get("path"));
                        let project = match candidates.as_slice() {
                            [] => None, // A later remove-project can remove this declaration.
                            [project] => Some(*project),
                            _ => selected.and_then(|path| {
                                candidates
                                    .iter()
                                    .copied()
                                    .find(|p| p.path == Path::new(path))
                            }),
                        };
                        if !candidates.is_empty() && project.is_none() {
                            return Err(failure(format!(
                                "Cannot identify project pin in {}: {name}",
                                path.display()
                            )));
                        }
                        if let Some(project) = project.filter(|project| {
                            (tag.name().as_ref() == b"project" || attrs.contains_key("revision"))
                                && attrs.get("revision") != Some(&project.commit)
                        }) {
                            attrs.insert("revision".into(), project.commit.clone());
                            let tag_name =
                                String::from_utf8_lossy(tag.name().as_ref()).into_owned();
                            let mut replacement = BytesStart::new(tag_name);
                            for (key, value) in &attrs {
                                replacement.push_attribute((key.as_str(), value.as_str()));
                            }
                            if matches!(event, Event::Empty(_)) {
                                Event::Empty(replacement)
                            } else {
                                Event::Start(replacement)
                            }
                        } else {
                            event
                        }
                    }
                    _ => event,
                };
                writer
                    .write_event(event)
                    .map_err(|e| failure(e.to_string()))?;
            }
            let updated = writer.into_inner();
            if updated != *bytes {
                changed.insert(path.clone(), updated);
            }
        }
        Ok(changed)
    }
}

pub fn resolve_revision(checkout: &Path, project: &RepoProject) -> Result<String, ReqvireError> {
    let candidates = [
        project.revision.clone(),
        format!(
            "refs/remotes/{}/{}",
            project.remote,
            project.revision.trim_start_matches("refs/heads/")
        ),
    ];
    for name in candidates {
        if let Ok(commit) = git_text(
            checkout,
            &[
                "rev-parse",
                "--verify",
                "--end-of-options",
                &format!("{name}^0"),
            ],
        ) {
            return Ok(commit);
        }
    }
    Err(failure(format!(
        "Selected project revision {} is unavailable locally",
        project.revision
    )))
}

/// Canonical repository identity across Git's common SSH/HTTPS spellings.
/// Preserve explicit ports and path case; discard only transport/user syntax.
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
    let host = host.rsplit('@').next().unwrap_or(host).to_ascii_lowercase();
    format!("{host}/{}", path.trim_start_matches('/'))
}

fn matching_remote(checkout: &Path, project: &RepoProject) -> Result<String, ReqvireError> {
    let remotes = git_text(checkout, &["remote"])?;
    let mut matches = Vec::new();
    for remote in remotes.lines() {
        let identity = git_text(checkout, &["remote", "get-url", remote])?;
        if normalize_url(&identity) == normalize_url(&project.fetch_url) {
            if remote == project.remote {
                return Ok(remote.to_owned());
            }
            matches.push(remote);
        } else if remote == project.remote {
            return Err(failure(
                "Project Git remote does not match its manifest identity",
            ));
        }
    }
    match matches.as_slice() {
        [remote] => Ok((*remote).to_owned()),
        _ => Err(failure(
            "Project Git remote identity is missing or ambiguous",
        )),
    }
}

#[derive(Deserialize)]
struct ResolvedManifest {
    xml: String,
    projects: Vec<RepoProject>,
}

// Repo supplies both the composed manifest and effective per-project metadata.
// Resolve the selected input files without any sync/fetch operation.
const REPO_RESOLUTION: &str = r#"
import json, sys
sys.path.insert(0, sys.argv[4])
from manifest_xml import RepoClient
client = RepoClient(sys.argv[1])
client.manifestProject.worktree = sys.argv[5]
client.Override(sys.argv[2], load_local_manifests=False)
xml = client.ToXml(filter_groups='all').toxml()
print(json.dumps({
    'xml': xml,
    'projects': [dict(name=p.name, path=p.relpath, revision=p.revisionExpr,
                      remote=p.remote.name, fetch_url=p.remote.url)
                 for p in client.projects],
}))
"#;

#[cfg(test)]
mod tests {
    use super::{
        observe_selector, selected_manifest, selected_manifest_at, stage_manifest, RepoWorkspace,
    };
    use std::{collections::BTreeMap, path::Path};

    #[test]
    fn default_selection_identifies_root_source_without_reading_cached_xml() {
        let temporary = tempfile::tempdir().expect("temporary workspace");
        let root = temporary.path();
        std::fs::create_dir_all(root.join(".repo/manifests")).expect("manifest directory");
        std::fs::create_dir_all(root.join("manifests")).expect("source directory");
        let entry = "<manifest><include name=\"manifests/default.xml\"/></manifest>\n";
        std::fs::write(root.join(".repo/manifest.xml"), entry).expect("selection wrapper");
        // Cache content is deliberately invalid. It cannot select model inputs.
        std::fs::write(root.join(".repo/manifests/default.xml"), "invalid cache")
            .expect("cached input");
        std::fs::write(root.join("manifests/default.xml"), "<manifest/>\n")
            .expect("authoritative source");

        let selected = selected_manifest(root)
            .expect("manifest selection")
            .expect("selected path");
        assert_eq!(selected, Path::new(".repo/manifest.xml"));
        assert_eq!(
            RepoWorkspace::source_manifest(root, &selected).expect("source selection"),
            Path::new("manifests/default.xml")
        );
    }

    #[test]
    fn override_selects_an_alternate_repo_manifest_location() {
        let temporary = tempfile::tempdir().expect("temporary workspace");
        let root = temporary.path();
        std::fs::create_dir_all(root.join(".repo/manifests")).expect("manifest directory");
        let alternate = std::path::Path::new(".repo/manifests/alternate.xml");
        std::fs::write(root.join(alternate), "invalid cached content").expect("alternate cache");
        std::fs::write(root.join("alternate.xml"), "<manifest/>\n").expect("alternate source");

        let selected = selected_manifest_at(root, Some(alternate))
            .expect("manifest selection")
            .expect("selected path");
        assert_eq!(selected, alternate);
        assert_eq!(
            RepoWorkspace::source_manifest(root, &selected).expect("source selection"),
            Path::new("alternate.xml")
        );
    }

    #[test]
    fn native_entries_reject_ambiguous_sources_and_path_traversal() {
        let temporary = tempfile::tempdir().expect("temporary workspace");
        let root = temporary.path();
        std::fs::create_dir_all(root.join(".repo")).expect("Repo metadata");
        for entry in [
            "<manifest><include name=\"../outside.xml\"/></manifest>",
            "<manifest><include name=\"/outside.xml\"/></manifest>",
            "<manifest><include name=\".repo/manifests/default.xml\"/></manifest>",
            "<manifest><include name=\"a.xml\"/><include name=\"b.xml\"/></manifest>",
            "<manifest><project name=\"component\"/></manifest>",
        ] {
            std::fs::write(root.join(".repo/manifest.xml"), entry).expect("entry fixture");
            assert!(
                RepoWorkspace::source_manifest(root, Path::new(".repo/manifest.xml")).is_err(),
                "accepted ambiguous or escaping entry: {entry}"
            );
        }
    }

    #[test]
    fn includes_use_root_sources_and_missing_inputs_cannot_fall_back_to_cache() {
        let temporary = tempfile::tempdir().expect("temporary workspace");
        let root = temporary.path();
        let staged = tempfile::tempdir().expect("staging directory");
        std::fs::create_dir_all(root.join("manifests")).expect("source directory");
        std::fs::create_dir_all(root.join(".repo/manifests/manifests")).expect("cache directory");
        let manifest = "<manifest><include name=\"manifests/projects.xml\"/></manifest>\n";
        std::fs::write(root.join("manifests/default.xml"), manifest).expect("selecting source");
        let projects = "<manifest><project name=\"current\" revision=\"source-pin\"/></manifest>\n";
        std::fs::write(root.join("manifests/projects.xml"), projects).expect("included source");
        std::fs::write(
            root.join(".repo/manifests/manifests/projects.xml"),
            "cached input",
        )
        .expect("stale cache");
        let mut inputs = BTreeMap::new();
        let staged_manifest = stage_manifest(
            root,
            Path::new("manifests/default.xml"),
            staged.path(),
            &mut inputs,
            &mut Vec::new(),
        )
        .expect("stage root sources");
        let staged_xml = std::fs::read_to_string(staged_manifest).expect("staged source");
        assert!(
            staged_xml.contains("name=\"manifests/projects.xml\""),
            "nested Repo includes must stay relative to the isolated source root"
        );
        assert!(
            !staged_xml.contains(&staged.path().to_string_lossy().to_string()),
            "absolute nested include paths are rejected by Repo"
        );
        assert_eq!(inputs.len(), 2);
        assert_eq!(
            inputs[Path::new("manifests/projects.xml")],
            projects.as_bytes()
        );
        assert_eq!(
            std::fs::read_to_string(root.join("manifests/default.xml")).expect("unchanged source"),
            manifest
        );
        std::fs::remove_file(root.join("manifests/projects.xml"))
            .expect("remove authoritative include");
        let missing = stage_manifest(
            root,
            Path::new("manifests/default.xml"),
            staged.path(),
            &mut BTreeMap::new(),
            &mut Vec::new(),
        )
        .expect_err("cached input cannot replace a missing source");
        assert!(missing.to_string().contains("manifests/projects.xml"));
        std::fs::write(root.join("manifests/projects.xml"), manifest).expect("recursive include");
        let recursive = stage_manifest(
            root,
            Path::new("manifests/default.xml"),
            staged.path(),
            &mut BTreeMap::new(),
            &mut Vec::new(),
        )
        .expect_err("recursive include");
        assert!(recursive.to_string().contains("Recursive manifest include"));
        std::fs::write(
            root.join("manifests/default.xml"),
            "<manifest><include name=\".repo/manifests/manifests/projects.xml\"/></manifest>",
        )
        .expect("cached source include");
        let cached = stage_manifest(
            root,
            Path::new("manifests/default.xml"),
            staged.path(),
            &mut BTreeMap::new(),
            &mut Vec::new(),
        )
        .expect_err("authored includes cannot import Repo administration");
        assert!(cached.to_string().contains("administration"));
    }

    #[cfg(unix)]
    #[test]
    fn native_repo_entry_symlink_selects_root_source_not_cache_bytes() {
        let temporary = tempfile::tempdir().expect("temporary workspace");
        let root = temporary.path();
        std::fs::create_dir_all(root.join(".repo/manifests")).expect("manifest directory");
        std::fs::write(root.join(".repo/manifests/default.xml"), "stale cached XML")
            .expect("cached input");
        std::fs::write(root.join("default.xml"), "<manifest/>\n").expect("root source");
        std::os::unix::fs::symlink("manifests/default.xml", root.join(".repo/manifest.xml"))
            .expect("native selection symlink");

        let selected = selected_manifest(root)
            .expect("manifest selection")
            .expect("selected path");
        assert_eq!(
            RepoWorkspace::source_manifest(root, &selected).expect("root source selection"),
            Path::new("default.xml")
        );
    }

    #[cfg(unix)]
    #[test]
    fn retargeting_an_entry_is_rejected_even_when_cached_files_have_identical_bytes() {
        let temporary = tempfile::tempdir().expect("temporary workspace");
        let root = temporary.path();
        std::fs::create_dir_all(root.join(".repo/manifests")).expect("cache directory");
        for path in ["first.xml", "second.xml"] {
            std::fs::write(
                root.join(".repo/manifests").join(path),
                "identical cached bytes",
            )
            .expect("cached input");
        }
        let entry = Path::new(".repo/manifest.xml");
        std::os::unix::fs::symlink("manifests/first.xml", root.join(entry))
            .expect("initial selector");
        let mut inputs = BTreeMap::new();
        observe_selector(root, entry, &mut inputs)
            .expect("observe selection without cached pin bytes");
        assert!(inputs.is_empty());
        let workspace = RepoWorkspace {
            selection: entry.into(),
            manifest: "first.xml".into(),
            projects: Vec::new(),
            resolved_xml: String::new(),
            inputs: BTreeMap::new(),
            central_commit: String::new(),
            central_repository: String::new(),
            selectors: BTreeMap::from([(entry.into(), "first.xml".into())]),
        };
        workspace
            .check_selection(root)
            .expect("unchanged selection");
        std::fs::write(
            root.join(".repo/manifests/first.xml"),
            "regenerated cache bytes",
        )
        .expect("cache refresh");
        workspace
            .check_selection(root)
            .expect("cache content is not source-selection authority");
        std::fs::remove_file(root.join(entry)).expect("replace entry");
        std::os::unix::fs::symlink("manifests/second.xml", root.join(entry))
            .expect("retarget selector");
        assert!(workspace
            .check_selection(root)
            .expect_err("source selection drift")
            .to_string()
            .contains("changed externally"));
    }

    #[cfg(unix)]
    #[test]
    fn manifest_guards_allow_an_aliased_workspace_prefix_and_reject_source_symlinks() {
        let temporary = tempfile::tempdir().expect("temporary workspace");
        let physical = temporary.path().join("physical");
        std::fs::create_dir(&physical).expect("physical workspace");
        let alias = temporary.path().join("alias");
        std::os::unix::fs::symlink(&physical, &alias).expect("workspace prefix alias");
        std::fs::write(physical.join("default.xml"), "<manifest/>\n").expect("root source");
        let staged = tempfile::tempdir().expect("staging directory");
        let mut inputs = BTreeMap::new();
        stage_manifest(
            &alias,
            Path::new("default.xml"),
            staged.path(),
            &mut inputs,
            &mut Vec::new(),
        )
        .expect("a symlink above the workspace is harmless, as with macOS /var");
        assert_eq!(inputs[Path::new("default.xml")], b"<manifest/>\n");
        std::os::unix::fs::symlink("../physical/default.xml", alias.join("linked.xml"))
            .expect("source symlink");
        let rejected = stage_manifest(
            &alias,
            Path::new("linked.xml"),
            staged.path(),
            &mut inputs,
            &mut Vec::new(),
        )
        .expect_err("a source symlink remains prohibited");
        assert!(rejected.to_string().contains("symlink"));
        let missing = stage_manifest(
            &alias,
            Path::new("missing.xml"),
            staged.path(),
            &mut inputs,
            &mut Vec::new(),
        )
        .expect_err("missing source");
        assert!(missing.to_string().contains("missing.xml"));
    }
}
