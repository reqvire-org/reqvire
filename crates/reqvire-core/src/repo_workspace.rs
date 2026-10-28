//! Manifest selection and repository provenance shared by every model consumer.
//!
//! Repo owns manifest composition. The default entry or an alternate file
//! location selects the inputs. This boundary never syncs or fetches.
use crate::{error::ReqvireError, model_inputs};
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

pub fn git_text(root: &Path, args: &[&str]) -> Result<String, ReqvireError> {
    let out = Command::new("git")
        .current_dir(root)
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::null())
        .args(args)
        .output()?;
    if !out.status.success() {
        return Err(failure(format!(
            "Git in {}: {}",
            root.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    String::from_utf8(out.stdout)
        .map(|v| v.trim_end_matches(['\r', '\n']).to_owned())
        .map_err(|e| failure(e.to_string()))
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

/// Stage only manifest inputs. Absolute include destinations keep Repo's own
/// include/group/extend semantics while preventing its remote manifest checkout
/// from replacing inputs selected by this Central worktree.
fn stage_manifest(
    root: &Path,
    path: &Path,
    temporary: &Path,
    inputs: &mut BTreeMap<PathBuf, Vec<u8>>,
    visiting: &mut Vec<PathBuf>,
) -> Result<PathBuf, ReqvireError> {
    let relative = relative_path(root, path)?;
    if visiting.contains(&relative) {
        return Err(failure(format!(
            "Recursive manifest include {}",
            relative.display()
        )));
    }
    let source = root.join(&relative);
    if source.canonicalize()? != source {
        return Err(failure(format!(
            "Manifest symlink is not allowed: {}",
            source.display()
        )));
    }
    let text = model_inputs::read_to_string(&source)?;
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
                let included = stage_manifest(root, Path::new(&name), temporary, inputs, visiting)?;
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
    pub fn uses_repo_managed_inputs(&self) -> bool {
        self.selection.starts_with(".repo")
    }

    pub fn resolve(root: &Path, manifest: PathBuf) -> Result<Self, ReqvireError> {
        let load = || {
            if !root.join(".repo/repo/main.py").is_file() {
                return Err(failure("Initialized local Repo metadata is unavailable; explicitly initialize the Repo client before selecting its manifest"));
            }
            let temporary = tempfile::tempdir()?;
            let mut inputs = BTreeMap::new();
            let native = manifest.starts_with(".repo");
            let selected = if native {
                PathBuf::from("--active-repo-manifest")
            } else {
                stage_manifest(
                    root,
                    &manifest,
                    temporary.path(),
                    &mut inputs,
                    &mut Vec::new(),
                )?
            };
            // Use the initialized Repo client's resolved Project objects, rather
            // than reimplementing URL joins, aliases or revision precedence.
            let out = Command::new("python3")
                .current_dir(root)
                .env("GIT_TERMINAL_PROMPT", "0")
                .stdin(Stdio::null())
                .args(["-c", REPO_RESOLUTION])
                .arg(root.join(".repo"))
                .arg(&selected)
                .arg(root.join(&manifest))
                .output()?;
            if !out.status.success() {
                return Err(failure(format!(
                    "Repo resolution failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                )));
            }
            let output: ResolvedManifest = serde_json::from_slice(&out.stdout)
                .map_err(|e| failure(format!("Invalid Repo resolution: {e}")))?;
            for (path, bytes) in output.inputs {
                let path = relative_path(root, &path)?;
                let source = root.join(&path);
                if source.canonicalize()?.strip_prefix(root).is_err() {
                    return Err(failure(format!(
                        "Repo manifest escapes workspace: {}",
                        path.display()
                    )));
                }
                if model_inputs::read_to_string(&source)?.as_bytes() != bytes {
                    return Err(failure(format!(
                        "Repo manifest changed during resolution: {}",
                        path.display()
                    )));
                }
                inputs.insert(path, bytes);
            }
            let resolved_xml = output.xml;
            if tags(&resolved_xml)?
                .iter()
                .any(|(name, _)| name == "submanifest")
            {
                return Err(failure("Repo submanifests are not supported; declare the component projects in Central's manifest"));
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
                manifest: output.manifest.unwrap_or_else(|| manifest.clone()),
                projects,
                resolved_xml,
                inputs,
            })
        };
        load().map_err(|e| failure(format!("Manifest {}: {e}", manifest.display())))
    }

    pub fn load(root: &Path, manifest: PathBuf) -> Result<Self, ReqvireError> {
        let mut resolved = Self::resolve(root, manifest)?;
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
        if !self.uses_repo_managed_inputs() {
            return Ok(BTreeMap::from([(
                self.manifest.clone(),
                self.pinned_xml()?,
            )]));
        }
        let mut changed = BTreeMap::new();
        for (path, bytes) in &self.inputs {
            if path.starts_with(".repo/manifests.git") {
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
                        if let Some(project) = project.filter(|_| {
                            tag.name().as_ref() == b"project" || attrs.contains_key("revision")
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
    #[serde(default)]
    inputs: BTreeMap<PathBuf, Vec<u8>>,
    #[serde(default)]
    manifest: Option<PathBuf>,
}

// Repo supplies both the composed manifest and effective per-project metadata.
// Resolve the selected input files without any sync/fetch operation.
const REPO_RESOLUTION: &str = r#"
import json, os, sys
from xml.dom import minidom
sys.path.insert(0, os.path.join(sys.argv[1], 'repo'))
from manifest_xml import RepoClient
client = RepoClient(sys.argv[1])
native = sys.argv[2] == '--active-repo-manifest'
inputs = {}
manifest = None
root = os.path.realpath(os.path.dirname(sys.argv[1]))
def capture(path):
    source = os.path.realpath(path)
    if os.path.commonpath([root, source]) != root:
        raise ValueError('Repo manifest input escapes workspace: ' + source)
    with open(source, 'rb') as stream:
        inputs[os.path.relpath(source, root)] = list(stream.read())
    return source
if native:
    parse = client._ParseManifestXml
    def parse_with_inputs(path, *args, **kwargs):
        capture(path)
        return parse(path, *args, **kwargs)
    client._ParseManifestXml = parse_with_inputs
    selected = sys.argv[3]
    entry = capture(selected)
    document = minidom.parse(selected)
    nodes = [n for n in document.documentElement.childNodes if n.nodeType == n.ELEMENT_NODE]
    if len(nodes) == 1 and nodes[0].tagName == 'include':
        entry = capture(os.path.join(client.manifestProject.worktree, nodes[0].getAttribute('name')))
    manifest = os.path.relpath(entry, root)
    config = os.path.join(sys.argv[1], 'manifests.git', 'config')
    if os.path.isfile(config):
        capture(config)
    if os.path.abspath(selected) != os.path.abspath(client.manifestFile):
        client.Override(selected, load_local_manifests=True)
else:
    client.Override(sys.argv[2], load_local_manifests=False)
xml = client.ToXml(filter_groups='all').toxml()
print(json.dumps({
    'xml': xml, 'inputs': inputs, 'manifest': manifest,
    'projects': [dict(name=p.name, path=p.relpath, revision=p.revisionExpr,
                      remote=p.remote.name, fetch_url=p.remote.url)
                 for p in client.projects],
}))
"#;

#[cfg(test)]
mod tests {
    use super::{selected_manifest, selected_manifest_at};

    #[test]
    fn default_selection_loads_repo_entry_without_a_root_manifest() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        std::fs::create_dir_all(root.join(".repo/manifests")).unwrap();
        let entry = "<manifest><include name=\"default.xml\"/></manifest>\n";
        std::fs::write(root.join(".repo/manifest.xml"), entry).unwrap();
        std::fs::write(root.join(".repo/manifests/default.xml"), "<manifest/>\n").unwrap();

        let selected = selected_manifest(root).unwrap().unwrap();
        let loaded = std::fs::read_to_string(root.join(selected)).unwrap();
        assert_eq!(loaded, entry);
    }

    #[test]
    fn override_selects_an_alternate_repo_manifest_location() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        std::fs::create_dir_all(root.join(".repo/manifests")).unwrap();
        let alternate = std::path::Path::new(".repo/manifests/alternate.xml");
        let manifest = "<manifest/>\n";
        std::fs::write(root.join(alternate), manifest).unwrap();

        let selected = selected_manifest_at(root, Some(alternate))
            .unwrap()
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(root.join(selected)).unwrap(),
            manifest
        );
    }

    #[cfg(unix)]
    #[test]
    fn default_selection_loads_the_legacy_repo_entry_symlink() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        std::fs::create_dir_all(root.join(".repo/manifests")).unwrap();
        let manifest = "<manifest/>\n";
        std::fs::write(root.join(".repo/manifests/default.xml"), manifest).unwrap();
        std::os::unix::fs::symlink("manifests/default.xml", root.join(".repo/manifest.xml"))
            .unwrap();

        let selected = selected_manifest(root).unwrap().unwrap();
        assert_eq!(
            std::fs::read_to_string(root.join(selected)).unwrap(),
            manifest
        );
    }
}
