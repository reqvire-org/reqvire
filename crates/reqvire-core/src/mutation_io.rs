//! Request-local file preparation for an authoritative MCP session. Outside a
//! session these helpers retain the ordinary CLI filesystem behaviour.
use crate::ModelManager;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone)]
pub struct SnapshotFiles {
    pub root: PathBuf,
    pub files: Arc<BTreeMap<PathBuf, Arc<Vec<u8>>>>,
    pub executable: Arc<BTreeSet<PathBuf>>,
}
struct PreparedFile {
    bytes: Arc<Vec<u8>>,
    executable: bool,
}
struct Context {
    files: SnapshotFiles,
    overlay: BTreeMap<PathBuf, Option<PreparedFile>>,
    changed: BTreeSet<PathBuf>,
    model: Option<Arc<ModelManager>>,
    capture: bool,
}
thread_local! {
    static CONTEXT: RefCell<Option<Context>> = const { RefCell::new(None) };
}
/// A synchronous request scope; never retain this across an await.
pub struct SnapshotGuard;
impl SnapshotFiles {
    pub fn enter(&self, model: Option<Arc<ModelManager>>) -> SnapshotGuard {
        CONTEXT.with(|slot| {
            assert!(slot.borrow().is_none(), "nested mutation file snapshot");
            *slot.borrow_mut() = Some(Context {
                files: self.clone(),
                overlay: BTreeMap::new(),
                changed: BTreeSet::new(),
                model,
                capture: false,
            });
        });
        SnapshotGuard
    }
}
impl SnapshotGuard {
    pub fn capture_dependencies(&self) {
        CONTEXT.with(|s| s.borrow_mut().as_mut().expect("active snapshot").capture = true);
    }
    /// Consume preparation once; ordinary reads never copy the accepted maps.
    pub fn prepared(self) -> Option<(SnapshotFiles, BTreeSet<PathBuf>)> {
        CONTEXT.with(|slot| {
            let mut context = slot.borrow_mut().take().expect("active snapshot");
            if context.overlay.is_empty() {
                return None;
            }
            let files = Arc::make_mut(&mut context.files.files);
            let executable = Arc::make_mut(&mut context.files.executable);
            for (path, entry) in context.overlay {
                match entry {
                    Some(entry) => {
                        if entry.executable {
                            executable.insert(path.clone());
                        } else {
                            executable.remove(&path);
                        }
                        files.insert(path, entry.bytes);
                    }
                    None => {
                        files.remove(&path);
                        executable.remove(&path);
                    }
                }
            }
            Some((context.files, context.changed))
        })
    }
}
impl Drop for SnapshotGuard {
    fn drop(&mut self) {
        CONTEXT.with(|slot| *slot.borrow_mut() = None);
    }
}
pub fn active() -> bool {
    CONTEXT.with(|slot| slot.borrow().is_some())
}
pub fn model() -> Option<Arc<ModelManager>> {
    CONTEXT.with(|slot| {
        slot.borrow()
            .as_ref()
            .and_then(|c| c.model.as_ref().map(Arc::clone))
    })
}
impl Context {
    fn file(&self, path: &Path) -> Option<&Arc<Vec<u8>>> {
        self.overlay.get(path).map_or_else(
            || self.files.files.get(path),
            |entry| entry.as_ref().map(|entry| &entry.bytes),
        )
    }
    fn executable(&self, path: &Path) -> bool {
        self.overlay.get(path).map_or_else(
            || self.files.executable.contains(path),
            |entry| entry.as_ref().is_some_and(|entry| entry.executable),
        )
    }
    fn paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.files
            .files
            .keys()
            .filter(|p| !self.overlay.contains_key(*p))
            .chain(
                self.overlay
                    .iter()
                    .filter_map(|(p, entry)| entry.as_ref().map(|_| p)),
            )
    }
}
fn absolute(path: &Path) -> PathBuf {
    crate::workspace::absolute_logical_path(path)
}
fn not_found(path: &Path) -> io::Error {
    io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "{} is absent from the accepted MCP snapshot",
            path.display()
        ),
    )
}
fn writable(context: &Context, path: &Path) -> io::Result<()> {
    if !path.starts_with(&context.files.root) || path.components().any(|c| c.as_os_str() == ".git")
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "MCP mutation path is outside its owned worktree",
        ));
    }
    Ok(())
}
pub fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let path = absolute(path.as_ref());
    CONTEXT.with(|slot| {
        let mut slot = slot.borrow_mut();
        match slot.as_mut() {
            Some(c) => {
                if let Some(bytes) = c.file(&path) {
                    return Ok((**bytes).clone());
                }
                if c.capture && !c.overlay.contains_key(&path) {
                    let bytes = std::fs::read(&path)?;
                    c.overlay.insert(
                        path,
                        Some(PreparedFile {
                            bytes: Arc::new(bytes.clone()),
                            executable: false,
                        }),
                    );
                    return Ok(bytes);
                }
                Err(not_found(&path))
            }
            None => std::fs::read(path),
        }
    })
}
pub fn read_to_string(path: impl AsRef<Path>) -> io::Result<String> {
    String::from_utf8(read(path)?).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}
pub fn is_file(path: impl AsRef<Path>) -> bool {
    let path = absolute(path.as_ref());
    CONTEXT.with(|slot| {
        slot.borrow().as_ref().map_or_else(
            || path.is_file(),
            |c| {
                c.file(&path).is_some()
                    || (c.capture && !c.overlay.contains_key(&path) && path.is_file())
            },
        )
    })
}
pub fn is_dir(path: impl AsRef<Path>) -> bool {
    let path = absolute(path.as_ref());
    CONTEXT.with(|slot| {
        slot.borrow().as_ref().map_or_else(
            || path.is_dir(),
            |c| c.paths().any(|p| p != &path && p.starts_with(&path)),
        )
    })
}
pub fn exists(path: impl AsRef<Path>) -> bool {
    is_file(&path) || is_dir(path)
}
pub fn paths_under(path: &Path) -> Option<Vec<PathBuf>> {
    let path = absolute(path);
    CONTEXT.with(|slot| {
        slot.borrow().as_ref().map(|c| {
            c.paths()
                .filter(|p| p.starts_with(&path))
                .cloned()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect()
        })
    })
}
pub fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let path = absolute(path.as_ref());
    CONTEXT.with(|slot| {
        let mut slot = slot.borrow_mut();
        match slot.as_mut() {
            Some(c) => {
                writable(c, &path)?;
                let executable = c.executable(&path);
                c.overlay.insert(
                    path.clone(),
                    Some(PreparedFile {
                        bytes: Arc::new(contents.as_ref().to_vec()),
                        executable,
                    }),
                );
                c.changed.insert(path);
                Ok(())
            }
            None => std::fs::write(path, contents),
        }
    })
}
pub fn remove_file(path: impl AsRef<Path>) -> io::Result<()> {
    let path = absolute(path.as_ref());
    CONTEXT.with(|slot| {
        let mut slot = slot.borrow_mut();
        match slot.as_mut() {
            Some(c) => {
                writable(c, &path)?;
                c.file(&path).ok_or_else(|| not_found(&path))?;
                c.overlay.insert(path.clone(), None);
                c.changed.insert(path);
                Ok(())
            }
            None => std::fs::remove_file(path),
        }
    })
}
pub fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<()> {
    if !active() {
        return std::fs::rename(from, to);
    }
    let bytes = read(&from)?;
    write(&to, bytes)?;
    copy_mode(from.as_ref(), to.as_ref());
    if absolute(from.as_ref()) == absolute(to.as_ref()) {
        return Ok(());
    }
    remove_file(from)
}
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<u64> {
    if !active() {
        return std::fs::copy(from, to);
    }
    let bytes = read(&from)?;
    let len = bytes.len() as u64;
    write(&to, bytes)?;
    copy_mode(from.as_ref(), to.as_ref());
    Ok(len)
}
pub fn create_dir_all(path: impl AsRef<Path>) -> io::Result<()> {
    if active() {
        Ok(())
    } else {
        std::fs::create_dir_all(path)
    }
}
pub fn remove_dir_all(path: impl AsRef<Path>) -> io::Result<()> {
    if let Some(paths) = paths_under(path.as_ref()) {
        for path in paths {
            remove_file(path)?;
        }
        Ok(())
    } else {
        std::fs::remove_dir_all(path)
    }
}

fn copy_mode(from: &Path, to: &Path) {
    CONTEXT.with(|slot| {
        if let Some(context) = slot.borrow_mut().as_mut() {
            let executable = context.executable(&absolute(from));
            if let Some(Some(entry)) = context.overlay.get_mut(&absolute(to)) {
                entry.executable = executable;
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (tempfile::TempDir, SnapshotFiles) {
        let directory = tempfile::tempdir().expect("test fixture operation should succeed");
        let root = directory.path().to_path_buf();
        let files = SnapshotFiles {
            root: root.clone(),
            files: Arc::new(BTreeMap::from([(
                root.join("a.txt"),
                Arc::new(b"accepted".to_vec()),
            )])),
            executable: Arc::new(BTreeSet::from([root.join("a.txt")])),
        };
        (directory, files)
    }

    #[test]
    fn read_scope_shares_accepted_file_map_entries() {
        let (_directory, files) = fixture();
        let path = files.root.join("a.txt");
        let original = files
            .files
            .get(&path)
            .expect("test fixture operation should succeed");
        let _scope = files.enter(None);
        CONTEXT.with(|slot| {
            let slot = slot.borrow();
            let entry = slot
                .as_ref()
                .expect("test fixture operation should succeed")
                .files
                .files
                .get(&path)
                .expect("test fixture operation should succeed");
            assert!(
                std::ptr::eq(original, entry),
                "read entry must not clone the file map"
            );
        });
    }

    #[test]
    fn read_scope_shares_accepted_model() {
        let (_directory, files) = fixture();
        let accepted = Arc::new(ModelManager::new());
        let _scope = files.enter(Some(Arc::clone(&accepted)));
        let loaded = model().expect("test fixture operation should succeed");
        assert!(
            std::ptr::eq(&accepted.graph_registry, &loaded.graph_registry),
            "read scope must not clone the accepted registry"
        );
    }

    #[test]
    fn overlays_preserve_base_and_resolve_files_directories_and_modes() {
        let (_directory, files) = fixture();
        let a = files.root.join("a.txt");
        let moved = files.root.join("nested/moved.txt");
        let copied = files.root.join("nested/copied.txt");
        std::fs::write(&a, b"external edit").expect("write test fixture");
        let scope = files.enter(None);
        assert_eq!(
            read(&a).expect("test fixture operation should succeed"),
            b"accepted"
        );
        write(&a, b"candidate").expect("test fixture operation should succeed");
        assert_eq!(
            read(&a).expect("test fixture operation should succeed"),
            b"candidate"
        );
        rename(&a, &moved).expect("test fixture operation should succeed");
        assert!(!is_file(&a));
        assert!(
            read(&a).is_err(),
            "a tombstone must not read the external file"
        );
        assert!(is_dir(files.root.join("nested")));
        copy(&moved, &copied).expect("test fixture operation should succeed");
        assert_eq!(
            paths_under(&files.root).expect("test fixture operation should succeed"),
            vec![copied.clone(), moved.clone()]
        );
        remove_file(&moved).expect("test fixture operation should succeed");
        assert!(remove_file(&moved).is_err());
        let (prepared, changed) = scope
            .prepared()
            .expect("test fixture operation should succeed");
        assert_eq!(changed, BTreeSet::from([a.clone(), copied.clone(), moved]));
        assert!(!prepared.files.contains_key(&a));
        assert!(!prepared.executable.contains(&a));
        assert_eq!(prepared.files[&copied].as_slice(), b"candidate");
        assert!(prepared.executable.contains(&copied));
        assert_eq!(files.files[&a].as_slice(), b"accepted");
        assert!(files.executable.contains(&a));
        assert_eq!(
            std::fs::read(&a).expect("read test fixture"),
            b"external edit"
        );
        assert!(!copied.exists());
        let scope = prepared.enter(None);
        remove_dir_all(files.root.join("nested")).expect("test fixture operation should succeed");
        assert!(!is_dir(files.root.join("nested")));
        assert!(paths_under(&files.root)
            .expect("test fixture operation should succeed")
            .is_empty());
        drop(scope);
        assert!(
            prepared.files.contains_key(&copied),
            "discarded preparation changed the base"
        );
    }

    #[test]
    fn read_scopes_and_discarded_preparation_do_not_materialize_maps() {
        let (_directory, files) = fixture();
        let a = files.root.join("a.txt");
        let scope = files.enter(None);
        assert_eq!(
            read(&a).expect("test fixture operation should succeed"),
            b"accepted"
        );
        assert!(scope.prepared().is_none());
        {
            let _scope = files.enter(None);
            write(&a, b"discard me").expect("test fixture operation should succeed");
            let denied = write(
                files
                    .root
                    .parent()
                    .expect("fixture path has a parent")
                    .join("outside"),
                b"denied",
            );
            assert_eq!(
                denied
                    .expect_err("test fixture operation should fail")
                    .kind(),
                io::ErrorKind::PermissionDenied
            );
            assert!(write(files.root.join(".git/config"), b"denied").is_err());
        }
        let scope = files.enter(None);
        assert_eq!(
            read(&a).expect("test fixture operation should succeed"),
            b"accepted"
        );
        assert!(scope.prepared().is_none());
    }

    #[test]
    fn only_explicit_startup_capture_reads_uncaptured_dependencies() {
        let (_directory, files) = fixture();
        let dependency = files.root.join("external.ttl");
        std::fs::write(&dependency, b"captured RDF").expect("write test fixture");
        {
            let _scope = files.enter(None);
            assert!(read(&dependency).is_err());
            assert!(!is_file(&dependency));
        }
        let capture = files.enter(None);
        capture.capture_dependencies();
        assert!(is_file(&dependency));
        assert_eq!(
            read(&dependency).expect("test fixture operation should succeed"),
            b"captured RDF"
        );
        let (captured, changed) = capture
            .prepared()
            .expect("test fixture operation should succeed");
        assert!(changed.is_empty(), "dependency capture is not a mutation");
        assert!(!files.files.contains_key(&dependency));
        std::fs::write(&dependency, b"external change").expect("write test fixture");
        let scope = captured.enter(None);
        assert_eq!(
            read(&dependency).expect("test fixture operation should succeed"),
            b"captured RDF"
        );
        assert!(scope.prepared().is_none());
    }
}
