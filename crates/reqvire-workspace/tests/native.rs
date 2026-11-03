#![allow(clippy::unwrap_used)]

use reqvire_workspace::{
    baseline, git::local_text, locks, manifest::MANIFEST, service::git, Operation, Workspace,
    WorkspaceComposition,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

struct Fixture {
    _temporary: tempfile::TempDir,
    root: PathBuf,
    remote: PathBuf,
    a: String,
    b: String,
}

#[test]
fn native_configuration_rejects_splitting_without_loading_a_partial_inventory() {
    let f = Fixture::new(true);
    let original = String::from_utf8(f.bytes()).unwrap();
    let head = command(&f.component(), &["rev-parse", "HEAD"]);
    for key in ["include", "includes", "fragments", "overlays"] {
        let invalid = format!("{key} = [\"missing.toml\"]\n{original}");
        fs::write(f.root.join(MANIFEST), &invalid).unwrap();
        let error = Workspace::open(&f.root).unwrap_err().to_string();
        assert!(error.contains("Unsupported workspace key"), "{error}");
        assert_eq!(f.bytes(), invalid.as_bytes());
        assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), head);
    }
}
fn command(root: &Path, args: &[&str]) -> String {
    git(root, args).unwrap()
}
fn init(path: &Path) {
    fs::create_dir_all(path).unwrap();
    command(path, &["init", "-q", "-b", "main"]);
    command(path, &["config", "user.name", "Workspace Test"]);
    command(path, &["config", "user.email", "workspace@example.invalid"]);
}
fn commit(path: &Path, message: &str) -> String {
    command(path, &["add", "-A"]);
    command(
        path,
        &["-c", "commit.gpgSign=false", "commit", "-qm", message],
    );
    command(path, &["rev-parse", "HEAD"])
}
impl Fixture {
    fn new(prepared: bool) -> Self {
        let temporary = reqvire_workspace::paths::temporary_directory().unwrap();
        let directory = temporary.path().to_path_buf();
        let root = directory.join("root");
        let seed = directory.join("seed");
        let remote = directory.join("remote.git");
        init(&seed);
        fs::write(seed.join("code.txt"), "A\n").unwrap();
        let a = commit(&seed, "A");
        fs::write(seed.join("code.txt"), "B\n").unwrap();
        let b = commit(&seed, "B");
        command(
            &directory,
            &[
                "clone",
                "-q",
                "--bare",
                seed.to_str().unwrap(),
                remote.to_str().unwrap(),
            ],
        );
        init(&root);
        fs::write(root.join(".gitignore"), "/external/site/\n/.repo/\n").unwrap();
        fs::write(root.join(MANIFEST),format!("# retain this comment\nversion = 1\nrepositories_dir = \"external\"\n\n[[repositories]]\npath = \"site\"\nurl = \"file://{}\"\nref = \"refs/heads/main\"\ncommit = \"{a}\" # retain pin comment\n",remote.display())).unwrap();
        commit(&root, "workspace");
        let fixture = Self {
            _temporary: temporary,
            root,
            remote,
            a,
            b,
        };
        if prepared {
            fixture.run(Operation::Sync { latest: false });
        }
        fixture
    }
    fn component(&self) -> PathBuf {
        self.root.join("external/site")
    }
    fn bytes(&self) -> Vec<u8> {
        fs::read(self.root.join(MANIFEST)).unwrap()
    }
    fn run(&self, operation: Operation) -> reqvire_workspace::Report {
        Workspace::open(&self.root)
            .unwrap()
            .prepare(operation, false, &[])
            .unwrap()
            .accept()
    }
    fn local_commit(&self) -> String {
        fs::write(
            self.component().join("local.txt"),
            "committed outside MCP\n",
        )
        .unwrap();
        command(
            &self.component(),
            &["config", "user.name", "Workspace Test"],
        );
        command(
            &self.component(),
            &["config", "user.email", "workspace@example.invalid"],
        );
        commit(&self.component(), "external")
    }
}

#[test]
fn pinned_sync_and_explicit_latest_preserve_tracking_ref_and_source() {
    let f = Fixture::new(false);
    let source = f.bytes();
    let root_head = command(&f.root, &["rev-parse", "HEAD"]);
    f.run(Operation::Sync { latest: false });
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.a);
    assert_eq!(source, f.bytes());
    assert_eq!(
        command(&f.component(), &["symbolic-ref", "--short", "HEAD"]),
        "main"
    );
    f.run(Operation::Sync { latest: true });
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.b);
    assert_eq!(
        String::from_utf8(source).unwrap().replace(&f.a, &f.b),
        String::from_utf8(f.bytes()).unwrap()
    );
    assert_eq!(command(&f.root, &["rev-parse", "HEAD"]), root_head);
}
#[test]
fn detached_owner_sync_uses_detached_components_and_preserves_tracking_ref() {
    let f = Fixture::new(false);
    let original = f.bytes();
    let root_head = command(&f.root, &["rev-parse", "HEAD"]);
    command(&f.root, &["checkout", "--quiet", "--detach", "HEAD"]);
    f.run(Operation::Sync { latest: false });
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.a);
    assert!(local_text(&f.component(), &["symbolic-ref", "--quiet", "HEAD"]).is_err());
    assert_eq!(f.bytes(), original);
    f.run(Operation::Sync { latest: true });
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.b);
    assert!(local_text(&f.component(), &["symbolic-ref", "--quiet", "HEAD"]).is_err());
    assert_eq!(
        String::from_utf8(f.bytes()).unwrap(),
        String::from_utf8(original).unwrap().replace(&f.a, &f.b)
    );
    assert_eq!(command(&f.root, &["rev-parse", "HEAD"]), root_head);
}

#[test]
fn freeze_preview_apply_and_repeat_record_commits_without_changing_refs() {
    let f = Fixture::new(true);
    let head = f.local_commit();
    let before = f.bytes();
    let preview = Workspace::open(&f.root)
        .unwrap()
        .prepare(Operation::Freeze, true, &[])
        .unwrap()
        .accept();
    assert!(preview.changed);
    assert_eq!(f.bytes(), before);
    f.run(Operation::Freeze);
    assert_eq!(
        String::from_utf8(before).unwrap().replace(&f.a, &head),
        String::from_utf8(f.bytes()).unwrap()
    );
    assert!(!f.run(Operation::Freeze).changed);
}
#[test]
fn dirty_and_diverged_sync_preserve_all_user_work() {
    let f = Fixture::new(true);
    let source = f.bytes();
    fs::write(f.component().join("code.txt"), "pending\n").unwrap();
    assert!(Workspace::open(&f.root)
        .unwrap()
        .prepare(Operation::Sync { latest: true }, false, &[])
        .err()
        .unwrap()
        .to_string()
        .contains("Dirty"));
    assert_eq!(f.bytes(), source);
    assert_eq!(
        fs::read_to_string(f.component().join("code.txt")).unwrap(),
        "pending\n"
    );
    fs::write(f.component().join("code.txt"), "A\n").unwrap();
    let c = f.local_commit();
    assert!(Workspace::open(&f.root)
        .unwrap()
        .prepare(Operation::Sync { latest: true }, false, &[])
        .is_err());
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), c);
    assert_eq!(source, f.bytes());
}
#[test]
fn latest_without_ref_rejects_before_advancing_any_pin() {
    let f = Fixture::new(true);
    let source = String::from_utf8(f.bytes())
        .unwrap()
        .replace("ref = \"refs/heads/main\"\n", "");
    fs::write(f.root.join(MANIFEST), &source).unwrap();
    assert!(Workspace::open(&f.root)
        .unwrap()
        .prepare(Operation::Sync { latest: true }, false, &[])
        .err()
        .unwrap()
        .to_string()
        .contains("tracking ref"));
    assert_eq!(f.bytes(), source.as_bytes());
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.a);
}
#[test]
fn rejected_candidate_restores_manifest_branch_tip_and_checkout() {
    let f = Fixture::new(true);
    let source = f.bytes();
    {
        let _unaccepted = Workspace::open(&f.root)
            .unwrap()
            .prepare(Operation::Sync { latest: true }, false, &[])
            .unwrap();
        assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.b);
    }
    assert_eq!(f.bytes(), source);
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.a);
    assert_eq!(command(&f.component(), &["status", "--porcelain"]), "");
}
#[test]
fn standalone_preparation_cannot_bypass_an_mcp_worktree_owner() {
    let f = Fixture::new(true);
    let path = f.root.join(
        local_text(
            &f.root,
            &["rev-parse", "--git-path", "reqvire-mcp-worktree.lock"],
        )
        .unwrap(),
    );
    let _owner = locks::lock_file(&path).unwrap();
    let source = f.bytes();
    assert!(Workspace::open(&f.root)
        .unwrap()
        .prepare(Operation::Freeze, false, &[])
        .err()
        .unwrap()
        .to_string()
        .contains("owned or busy"));
    assert_eq!(f.bytes(), source);
}
#[test]
fn removal_preserves_checkout_and_dirty_content_even_for_final_dependency() {
    let f = Fixture::new(true);
    fs::write(f.component().join("pending.txt"), "retain\n").unwrap();
    f.run(Operation::Remove {
        path: "site".into(),
    });
    let status = Workspace::open(&f.root).unwrap().status().unwrap();
    assert!(status["projects"].as_array().unwrap().is_empty());
    assert_eq!(
        fs::read_to_string(f.component().join("pending.txt")).unwrap(),
        "retain\n"
    );
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.a);
}
#[test]
fn schema_and_path_errors_never_fall_back_to_single_repository() {
    let f = Fixture::new(false);
    let source = String::from_utf8(f.bytes()).unwrap();
    for invalid in [
        source.replace("version = 1", "version = 99"),
        source.replace("path = \"site\"", "path = \"../escape\""),
        source.replace(
            "repositories_dir = \"external\"",
            "repositories_dir = \".git\"",
        ),
        source.replace("path = \"site\"", "path = \"/absolute\""),
    ] {
        fs::write(f.root.join(MANIFEST), &invalid).unwrap();
        assert!(Workspace::open(&f.root).is_err());
        assert_eq!(fs::read_to_string(f.root.join(MANIFEST)).unwrap(), invalid);
    }
}
#[test]
fn discovery_stops_at_component_git_boundary_and_accepts_subdirectory_root() {
    let f = Fixture::new(true);
    let selected = f.root.join("engineering");
    fs::create_dir_all(selected.join("client")).unwrap();
    fs::write(
        selected.join(MANIFEST),
        "version = 1\nrepositories_dir = \"vendor\"\n",
    )
    .unwrap();
    assert_eq!(
        reqvire_workspace::service::effective_root(&selected.join("client")).unwrap(),
        selected
    );
    assert_eq!(
        reqvire_workspace::service::effective_root(&f.component()).unwrap(),
        f.component()
    );
}
#[cfg(unix)]
#[test]
fn baseline_reads_accept_ancestor_aliases_for_root_and_subdirectory_workspaces() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new(true);
    let nested = f.root.join("engineering");
    fs::create_dir_all(&nested).unwrap();
    fs::write(
        nested.join(MANIFEST),
        "version = 1\nrepositories_dir = \"vendor\"\n",
    )
    .unwrap();
    fs::write(nested.join("evidence.txt"), "nested source\n").unwrap();
    let head = commit(&f.root, "Nested workspace");
    let prefix = f.root.parent().unwrap().join("prefix-alias");
    symlink(f.root.parent().unwrap(), &prefix).unwrap();
    for relative in ["root", "root/engineering"] {
        let canonical = f.root.parent().unwrap().join(relative);
        let alias = prefix.join(relative);
        assert_eq!(Workspace::open(&alias).unwrap().root, canonical);
        assert_eq!(
            PathBuf::from(reqvire_workspace::git::find_root(&alias).unwrap()),
            f.root
        );
        assert_eq!(
            reqvire_workspace::service::effective_root(&alias).unwrap(),
            canonical
        );
        let current = WorkspaceComposition::load(&canonical, MANIFEST.into()).unwrap();
        let expected = baseline::read(&canonical, &current, "HEAD").unwrap();
        let observed = baseline::read(&alias, &current, "HEAD").unwrap();
        assert_eq!(observed.composition, expected.composition);
        assert_eq!(observed.files, expected.files);
        assert!(observed
            .files
            .keys()
            .all(|path| path.starts_with(&canonical)));
        if relative.ends_with("engineering") {
            assert!(observed.files.contains_key(&canonical.join("evidence.txt")));
            assert!(!observed.files.contains_key(&f.root.join(MANIFEST)));
        }
    }
    assert_eq!(command(&f.root, &["rev-parse", "HEAD"]), head);
    assert_eq!(command(&f.root, &["status", "--porcelain"]), "");
    assert_eq!(command(&f.component(), &["rev-parse", "HEAD"]), f.a);
}

#[cfg(unix)]
#[test]
fn portable_prefix_alias_is_accepted_but_component_symlinks_are_rejected() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new(false);
    let alias = f.root.with_file_name("alias");
    symlink(&f.root, &alias).unwrap();
    assert_eq!(Workspace::open(&alias).unwrap().root, f.root);
    fs::create_dir_all(f.root.join("external")).unwrap();
    symlink(f._temporary.path().join("seed"), f.component()).unwrap();
    assert!(Workspace::open(&f.root).is_err());
}
#[test]
fn xml_to_native_migration_preserves_linked_git_directory_and_supports_old_baseline() {
    let f = Fixture::new(true);
    let component = f.component();
    // Model Repo's ordinary gitdir-file layout with Git itself, without Repo/Python.
    let storage = f.root.join(".repo/component.git");
    fs::create_dir_all(storage.parent().unwrap()).unwrap();
    fs::rename(component.join(".git"), &storage).unwrap();
    fs::write(
        component.join(".git"),
        format!("gitdir: {}\n", storage.display()),
    )
    .unwrap();
    let link = fs::read(component.join(".git")).unwrap();
    command(&f.root, &["rm", "-q", MANIFEST]);
    fs::create_dir_all(f.root.join("manifests")).unwrap();
    let xml=format!("<manifest><remote name=\"local\" fetch=\"file://{}/\"/><default remote=\"local\" revision=\"refs/heads/main\"/><include name=\"manifests/projects.xml\"/></manifest>\n",f.remote.parent().unwrap().display());
    fs::write(f.root.join("manifests/default.xml"), xml).unwrap();
    fs::write(f.root.join("manifests/projects.xml"),format!("<manifest><project name=\"remote.git\" path=\"external/site\" revision=\"{}\" upstream=\"refs/heads/main\"/></manifest>\n",f.a)).unwrap();
    let base = commit(&f.root, "XML composition");
    f.run(Operation::Migrate {
        source: Some("manifests/default.xml".into()),
    });
    assert_eq!(fs::read(component.join(".git")).unwrap(), link);
    assert!(storage.exists());
    assert_eq!(command(&component, &["rev-parse", "HEAD"]), f.a);
    commit(&f.root, "Native composition");
    let current = WorkspaceComposition::load(&f.root, MANIFEST.into()).unwrap();
    let old = baseline::read(&f.root, &current, &base).unwrap();
    assert_eq!(old.composition.projects[0].commit, f.a);
    assert!(old.files.contains_key(&component.join("code.txt")));
    assert_eq!(command(&component, &["rev-parse", "HEAD"]), f.a);
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let prefix = f.root.parent().unwrap().join("prefix-alias");
        symlink(f.root.parent().unwrap(), &prefix).unwrap();
        let aliased = baseline::read(&prefix.join("root"), &current, &base).unwrap();
        assert_eq!(aliased.composition, old.composition);
        assert_eq!(aliased.files, old.files);
        assert_eq!(fs::read(component.join(".git")).unwrap(), link);
    }
    let repeated = f.run(Operation::Migrate {
        source: Some("manifests/default.xml".into()),
    });
    assert!(!repeated.changed);
}
