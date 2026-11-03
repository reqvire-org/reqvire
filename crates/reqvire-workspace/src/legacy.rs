//! Explicit Repo XML conversion and legacy baseline reading. No Repo process.
use crate::{
    git::local_text as git_text,
    manifest::{full_commit, WorkspaceComposition, WorkspaceProject},
    paths::canonicalize_existing,
    Error,
};
use quick_xml::{
    events::{BytesStart, Event},
    Reader, Writer,
};
use std::{
    collections::BTreeMap,
    path::{Component, Path, PathBuf},
};
fn failure(message: impl Into<String>) -> Error {
    Error::ProcessError(message.into())
}
#[cfg(test)]
fn selected_manifest(root: &Path) -> Result<Option<PathBuf>, Error> {
    selected_manifest_at(root, None)
}
#[cfg(test)]
fn selected_manifest_at(root: &Path, selection: Option<&Path>) -> Result<Option<PathBuf>, Error> {
    if let Some(p) = selection {
        return Ok(Some(relative_path(root, p)?));
    }
    if root.join(".repo/manifest.xml").exists() {
        Ok(Some(".repo/manifest.xml".into()))
    } else {
        Ok(None)
    }
}
fn relative_path(root: &Path, path: &Path) -> Result<PathBuf, Error> {
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

fn source_path(root: &Path, path: &Path) -> Result<PathBuf, Error> {
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

fn overlay_entries(root: &Path) -> Result<Vec<PathBuf>, Error> {
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

#[cfg(all(test, unix))]
fn observe_selector(
    root: &Path,
    path: &Path,
    inputs: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> Result<(), Error> {
    let entry = root.join(path);
    // Guard symlink target identity through selectors, and source content at
    // its root path. Duplicate alias bytes would mistake our pin update for drift.
    if !std::fs::symlink_metadata(&entry)?.file_type().is_symlink() {
        inputs.insert(
            path.to_owned(),
            std::fs::read_to_string(&entry)?.into_bytes(),
        );
    }
    Ok(())
}

type Attributes = BTreeMap<String, String>;
fn attributes(tag: &BytesStart<'_>) -> Result<Attributes, Error> {
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
fn attr<'a>(attrs: &'a Attributes, name: &str) -> Result<&'a str, Error> {
    attrs
        .get(name)
        .map(String::as_str)
        .ok_or_else(|| failure(format!("Missing manifest {name}")))
}
pub(crate) fn tags(xml: &str) -> Result<Vec<(String, Attributes)>, Error> {
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
) -> Result<PathBuf, Error> {
    let relative = source_path(root, path)?;
    if visiting.contains(&relative) {
        return Err(failure(format!(
            "Recursive manifest include {}",
            relative.display()
        )));
    }
    let source = root.join(&relative);
    if canonicalize_existing(&source)
        .map_err(|e| failure(format!("Manifest source {}: {e}", relative.display())))?
        != canonicalize_existing(root)?.join(&relative)
    {
        return Err(failure(format!(
            "Manifest symlink is not allowed: {}",
            source.display()
        )));
    }
    let text = std::fs::read_to_string(&source)
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

pub fn source_manifest(root: &Path, selection: &Path) -> Result<PathBuf, Error> {
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
        let target = canonicalize_existing(&entry)?;
        let physical = canonicalize_existing(root)?;
        let owner = if overlay || !target.starts_with(physical.join(".repo/manifests")) {
            physical
        } else {
            physical.join(".repo/manifests")
        };
        return target
            .strip_prefix(owner)
            .map_err(|_| failure("Repo entry does not select its manifest source"))
            .and_then(|path| source_path(root, path));
    }
    let xml = std::fs::read_to_string(&entry)?;
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

pub fn check_selection(view: &WorkspaceComposition, root: &Path) -> Result<(), Error> {
    let accepted: Vec<_> = view
        .selectors
        .keys()
        .filter(|entry| entry.starts_with(".repo/local_manifests"))
        .cloned()
        .collect();
    if overlay_entries(root)? != accepted {
        return Err(failure("Repo local manifest selection changed externally"));
    }
    for (entry, source) in &view.selectors {
        if source_manifest(root, entry)? != *source {
            return Err(failure(format!(
                "Manifest source selection {} changed externally",
                entry.display()
            )));
        }
    }
    Ok(())
}

fn fetch_url(root: &Path, fetch: &str, name: &str) -> Result<String, Error> {
    if fetch.contains("://") || (fetch.contains(':') && !fetch.starts_with('.')) {
        return Ok(format!("{}/{}", fetch.trim_end_matches('/'), name));
    }
    let base = git_text(root, &["remote", "get-url", "origin"])
        .unwrap_or_else(|_| root.to_string_lossy().into_owned());
    let raw = format!("{base}/{fetch}/{name}");
    let (prefix, path) = raw
        .split_once("://")
        .map(|(a, b)| (format!("{a}://"), b))
        .unwrap_or((String::new(), raw.as_str()));
    let mut parts = Vec::new();
    for part in path.split('/') {
        match part {
            "." | "" => {}
            ".." => {
                parts.pop();
            }
            p => parts.push(p),
        }
    }
    Ok(format!("{prefix}{}", parts.join("/")))
}

/// Resolve the selected versioned XML source and includes into a flat native inventory.
pub fn resolve(root: &Path, selection: &Path) -> Result<WorkspaceComposition, Error> {
    let root = canonicalize_existing(root)?;
    if let Some(overlay) = overlay_entries(&root)?.first() {
        return Err(failure(format!("Unsupported Repo local overlay {}; version its effective declarations in the selected XML source before migration",overlay.display())));
    }
    let manifest = source_manifest(&root, selection)?;
    let temporary = crate::paths::temporary_directory()?;
    let mut inputs = BTreeMap::new();
    stage_manifest(
        &root,
        &manifest,
        temporary.path(),
        &mut inputs,
        &mut Vec::new(),
    )?;
    let mut ordered = Vec::new();
    fn expand(
        path: &Path,
        inputs: &BTreeMap<PathBuf, Vec<u8>>,
        out: &mut Vec<(String, Attributes)>,
    ) -> Result<(), Error> {
        let bytes = inputs
            .get(path)
            .ok_or_else(|| failure(format!("Missing manifest input {}", path.display())))?;
        let text = std::str::from_utf8(bytes).map_err(|e| failure(e.to_string()))?;
        for (tag, attrs) in tags(text)? {
            if tag == "include" {
                expand(Path::new(attr(&attrs, "name")?), inputs, out)?;
            } else {
                out.push((tag, attrs));
            }
        }
        Ok(())
    }
    expand(&manifest, &inputs, &mut ordered)?;
    let mut remotes: BTreeMap<String, Attributes> = BTreeMap::new();
    let mut defaults = Attributes::new();
    for (tag, attrs) in &ordered {
        match tag.as_str() {
            "remote" => {
                remotes.insert(attr(attrs, "name")?.to_owned(), attrs.clone());
            }
            "default" => {
                if !defaults.is_empty() {
                    return Err(failure("Ambiguous XML defaults"));
                }
                defaults = attrs.clone();
            }
            _ => {}
        }
    }
    let mut projects: Vec<WorkspaceProject> = Vec::new();
    for (tag, attrs) in ordered {
        match tag.as_str() {
            "manifest" | "remote" | "default" => {}
            "project" => {
                let name = attr(&attrs, "name")?.to_owned();
                let path = relative_path(&root, Path::new(attrs.get("path").unwrap_or(&name)))?;
                crate::manifest::relative_path(&root, &path)?;
                crate::manifest::contained(&root, &path)?;
                let remote_name = attrs
                    .get("remote")
                    .or_else(|| defaults.get("remote"))
                    .ok_or_else(|| failure("Missing XML remote"))?;
                let remote = remotes
                    .get(remote_name)
                    .ok_or_else(|| failure("Unknown XML remote"))?;
                let revision = attrs
                    .get("revision")
                    .or_else(|| remote.get("revision"))
                    .or_else(|| defaults.get("revision"))
                    .ok_or_else(|| failure("Missing XML project revision"))?
                    .clone();
                let reference = attrs
                    .get("upstream")
                    .cloned()
                    .or_else(|| (!full_commit(&revision)).then(|| revision.clone()));
                let url = fetch_url(&root, attr(remote, "fetch")?, &name)?;
                projects.push(WorkspaceProject {
                    name,
                    path,
                    commit: if full_commit(&revision) {
                        revision.clone()
                    } else {
                        String::new()
                    },
                    revision,
                    tracking_ref: reference,
                    remote: remote.get("alias").unwrap_or(remote_name).clone(),
                    fetch_url: url,
                });
            }
            "remove-project" => {
                let name = attr(&attrs, "name")?;
                projects.retain(|p| p.name != name);
            }
            "extend-project" => {
                let name = attr(&attrs, "name")?;
                let mut count = 0;
                for p in projects.iter_mut().filter(|p| {
                    p.name == name
                        && attrs
                            .get("path")
                            .is_none_or(|path| p.path == Path::new(path))
                }) {
                    count += 1;
                    if let Some(rev) = attrs.get("revision") {
                        p.revision = rev.clone();
                        p.commit = if full_commit(rev) {
                            rev.clone()
                        } else {
                            String::new()
                        };
                        p.tracking_ref = attrs
                            .get("upstream")
                            .cloned()
                            .or_else(|| (!full_commit(rev)).then(|| rev.clone()));
                    }
                    if let Some(dest) = attrs.get("dest-path") {
                        p.path = crate::manifest::relative_path(&root, Path::new(dest))?;
                    }
                    if let Some(remote) = attrs.get("remote") {
                        let info = remotes
                            .get(remote)
                            .ok_or_else(|| failure("Unknown XML remote"))?;
                        p.fetch_url = fetch_url(&root, attr(info, "fetch")?, &p.name)?;
                        p.remote = info.get("alias").unwrap_or(remote).clone();
                    }
                }
                if count == 0 {
                    return Err(failure(format!("Unknown extended project {name}")));
                }
            }
            other => return Err(failure(format!("Unsupported Repo XML element {other}"))),
        }
    }
    projects.sort_by(|a, b| a.path.cmp(&b.path));
    for pair in projects.windows(2) {
        if pair[1].path.starts_with(&pair[0].path) {
            return Err(failure("Duplicate or overlapping XML project paths"));
        }
    }
    Ok(WorkspaceComposition {
        selection: selection.to_owned(),
        manifest,
        repositories_dir: ".".into(),
        projects,
        inputs,
        ..WorkspaceComposition::default()
    })
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use super::observe_selector;
    use super::{selected_manifest, selected_manifest_at, stage_manifest, WorkspaceComposition};
    use std::{collections::BTreeMap, path::Path};

    #[test]
    fn default_selection_identifies_root_source_without_reading_cached_xml() {
        let temporary = crate::paths::temporary_directory().expect("temporary workspace");
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
            WorkspaceComposition::source_manifest(root, &selected).expect("source selection"),
            Path::new("manifests/default.xml")
        );
    }

    #[test]
    fn override_selects_an_alternate_repo_manifest_location() {
        let temporary = crate::paths::temporary_directory().expect("temporary workspace");
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
            WorkspaceComposition::source_manifest(root, &selected).expect("source selection"),
            Path::new("alternate.xml")
        );
    }

    #[test]
    fn native_entries_reject_ambiguous_sources_and_path_traversal() {
        let temporary = crate::paths::temporary_directory().expect("temporary workspace");
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
                WorkspaceComposition::source_manifest(root, Path::new(".repo/manifest.xml"))
                    .is_err(),
                "accepted ambiguous or escaping entry: {entry}"
            );
        }
    }

    #[test]
    fn includes_use_root_sources_and_missing_inputs_cannot_fall_back_to_cache() {
        let temporary = crate::paths::temporary_directory().expect("temporary workspace");
        let root = temporary.path();
        let staged = crate::paths::temporary_directory().expect("staging directory");
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
        let temporary = crate::paths::temporary_directory().expect("temporary workspace");
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
            WorkspaceComposition::source_manifest(root, &selected).expect("root source selection"),
            Path::new("default.xml")
        );
    }

    #[cfg(unix)]
    #[test]
    fn retargeting_an_entry_is_rejected_even_when_cached_files_have_identical_bytes() {
        let temporary = crate::paths::temporary_directory().expect("temporary workspace");
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
        let workspace = WorkspaceComposition {
            selection: entry.into(),
            manifest: "first.xml".into(),
            projects: Vec::new(),
            source: String::new(),
            inputs: BTreeMap::new(),
            central_commit: String::new(),
            central_repository: String::new(),
            selectors: BTreeMap::from([(entry.into(), "first.xml".into())]),
            ..WorkspaceComposition::default()
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
        let temporary = crate::paths::temporary_directory().expect("temporary workspace");
        let physical = temporary.path().join("physical");
        std::fs::create_dir(&physical).expect("physical workspace");
        let alias = temporary.path().join("alias");
        std::os::unix::fs::symlink(&physical, &alias).expect("workspace prefix alias");
        std::fs::write(physical.join("default.xml"), "<manifest/>\n").expect("root source");
        let staged = crate::paths::temporary_directory().expect("staging directory");
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
