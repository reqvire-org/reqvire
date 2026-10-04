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
    pub files: BTreeMap<PathBuf, Arc<Vec<u8>>>,
    pub executable: BTreeSet<PathBuf>,
}
struct Context {
    files: SnapshotFiles,
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
    pub fn prepared(&self) -> (SnapshotFiles, BTreeSet<PathBuf>) {
        CONTEXT.with(|slot| {
            let slot = slot.borrow();
            let context = slot.as_ref().expect("active snapshot");
            (context.files.clone(), context.changed.clone())
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
pub fn model() -> Option<ModelManager> {
    CONTEXT.with(|slot| {
        slot.borrow()
            .as_ref()
            .and_then(|c| c.model.as_ref().map(|m| (**m).clone()))
    })
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
                if let Some(bytes) = c.files.files.get(&path) {
                    return Ok((**bytes).clone());
                }
                if c.capture {
                    let bytes = std::fs::read(&path)?;
                    c.files.files.insert(path, Arc::new(bytes.clone()));
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
            |c| c.files.files.contains_key(&path) || (c.capture && path.is_file()),
        )
    })
}
pub fn is_dir(path: impl AsRef<Path>) -> bool {
    let path = absolute(path.as_ref());
    CONTEXT.with(|slot| {
        slot.borrow().as_ref().map_or_else(
            || path.is_dir(),
            |c| {
                c.files
                    .files
                    .keys()
                    .any(|p| p != &path && p.starts_with(&path))
            },
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
            c.files
                .files
                .keys()
                .filter(|p| p.starts_with(&path))
                .cloned()
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
                c.files
                    .files
                    .insert(path.clone(), Arc::new(contents.as_ref().to_vec()));
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
                c.files
                    .files
                    .remove(&path)
                    .ok_or_else(|| not_found(&path))?;
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
    copy_mode(from.as_ref(), to.as_ref());
    write(&to, bytes)?;
    remove_file(from)
}
pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<u64> {
    if !active() {
        return std::fs::copy(from, to);
    }
    let bytes = read(&from)?;
    let len = bytes.len() as u64;
    copy_mode(from.as_ref(), to.as_ref());
    write(to, bytes)?;
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
            if context.files.executable.contains(&absolute(from)) {
                context.files.executable.insert(absolute(to));
            } else {
                context.files.executable.remove(&absolute(to));
            }
        }
    });
}
