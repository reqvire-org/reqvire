//! Shared local Git plumbing. Offline reads never acquire remote objects.
use crate::Error;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{LazyLock, Mutex},
};

static ROOT_CACHE: LazyLock<Mutex<BTreeMap<PathBuf, String>>> =
    LazyLock::new(|| Mutex::new(BTreeMap::new()));
pub fn clear_root_cache() {
    ROOT_CACHE.lock().expect("root cache poisoned").clear();
}
/// Preserve the existing per-directory Git-root cache across model reads.
pub fn find_root(directory: &Path) -> Result<String, Error> {
    let mut cache = ROOT_CACHE.lock().expect("root cache poisoned");
    if let Some(root) = cache.get(directory) {
        return Ok(root.clone());
    }
    let root = local_text(directory, &["rev-parse", "--show-toplevel"])?;
    cache.insert(directory.to_owned(), root.clone());
    drop(cache);
    Ok(root)
}

/// Core Git execution for local metadata and immutable objects.
///
/// Callers that
/// need bounded writes/cancellation supply their own runner to GitReader.
/// Neither lazy object fetching nor any Git transport is permitted here.
pub fn run_local(directory: &Path, args: &[&str], input: Option<&[u8]>) -> Result<Vec<u8>, Error> {
    let mut command = Command::new("git");
    command
        .current_dir(directory)
        .args(["-c", "protocol.allow=never"])
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_NO_LAZY_FETCH", "1")
        .env("GIT_ALLOW_PROTOCOL", "")
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        });
    let output =
        crate::process::run_binary(command, input, std::time::Duration::from_secs(30), None)
            .map_err(|failure| failure.error)?;
    if !output.success {
        return Err(git_error(format!(
            "Git in {}: {}",
            directory.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(output.stdout)
}

pub fn local_bytes(directory: &Path, args: &[&str]) -> Result<Vec<u8>, Error> {
    run_local(directory, args, None)
}

pub fn local_text(directory: &Path, args: &[&str]) -> Result<String, Error> {
    String::from_utf8(local_bytes(directory, args)?)
        .map(|text| text.trim_end_matches(['\r', '\n']).to_owned())
        .map_err(|e| git_error(e.to_string()))
}

/// Read-only Git plumbing with the caller's execution policy. MCP supplies its
/// bounded, binary-safe runner; repository parsing stays in this shared layer.
pub struct GitReader<'a, F> {
    root: &'a Path,
    run: F,
}

pub struct TreeFile {
    pub bytes: std::sync::Arc<Vec<u8>>,
    pub executable: bool,
    pub symlink: bool,
}

impl<'a, F, E> GitReader<'a, F>
where
    F: FnMut(&[&str], Option<&[u8]>) -> Result<Vec<u8>, E>,
    E: From<Error>,
{
    pub const fn new(root: &'a Path, run: F) -> Self {
        Self { root, run }
    }

    pub fn operation_in_progress(&mut self) -> Result<bool, E> {
        let bytes = (self.run)(&["rev-parse", "--absolute-git-dir"], None)?;
        let directory = PathBuf::from(git_utf8(&bytes)?.trim_end_matches(['\n', '\r']));
        Ok([
            "MERGE_HEAD",
            "CHERRY_PICK_HEAD",
            "REVERT_HEAD",
            "rebase-merge",
            "rebase-apply",
            "sequencer",
        ]
        .iter()
        .any(|marker| directory.join(marker).exists()))
    }

    pub fn changed_paths(&mut self, old: &str, new: &str) -> Result<BTreeSet<PathBuf>, E> {
        let bytes = (self.run)(
            &[
                "diff",
                "--no-ext-diff",
                "--no-renames",
                "--name-only",
                "-z",
                old,
                new,
                "--",
            ],
            None,
        )?;
        bytes
            .split(|b| *b == 0)
            .filter(|p| !p.is_empty())
            .map(|p| git_utf8(p).map(|p| self.root.join(p)).map_err(Into::into))
            .collect()
    }

    /// One tree read and one deduplicated blob batch, regardless of file count.
    /// Missing paths represent deletions. Nonregular entries require a caller
    /// with explicit symlink/submodule support rather than a byte-file snapshot.
    pub fn regular_files_at_commit(
        &mut self,
        commit: &str,
        paths: &BTreeSet<PathBuf>,
    ) -> Result<BTreeMap<PathBuf, TreeFile>, E> {
        if paths.is_empty() {
            return Ok(BTreeMap::new());
        }
        self.selected_files_at_commit(commit, Some(paths), false)
    }

    /// Read a complete tree for isolated historical reconstruction. Symlink
    /// bytes are declarations; the caller must resolve them inside this tree.
    pub fn files_at_commit(&mut self, commit: &str) -> Result<BTreeMap<PathBuf, TreeFile>, E> {
        self.selected_files_at_commit(commit, None, true)
    }

    fn selected_files_at_commit(
        &mut self,
        commit: &str,
        paths: Option<&BTreeSet<PathBuf>>,
        with_symlinks: bool,
    ) -> Result<BTreeMap<PathBuf, TreeFile>, E> {
        let tree = (self.run)(&["ls-tree", "-r", "-z", "--full-tree", commit], None)?;
        let mut entries = BTreeMap::new();
        for entry in tree.split(|b| *b == 0).filter(|e| !e.is_empty()) {
            let (metadata, path) = git_utf8(entry)?
                .split_once('\t')
                .ok_or_else(|| git_error("Invalid Git tree entry"))?;
            let mut metadata = metadata.split_whitespace();
            let mode = metadata
                .next()
                .ok_or_else(|| git_error("Missing tree mode"))?;
            let kind = metadata
                .next()
                .ok_or_else(|| git_error("Missing tree object kind"))?;
            let oid = metadata
                .next()
                .ok_or_else(|| git_error("Missing tree object ID"))?;
            let path = self.root.join(path);
            if paths.is_none_or(|paths| paths.contains(&path)) {
                if paths.is_none() && kind == "commit" && mode == "160000" {
                    continue; // Undeclared submodules are outside the composition.
                }
                if kind != "blob"
                    || !(matches!(mode, "100644" | "100755") || (with_symlinks && mode == "120000"))
                {
                    return Err(git_error(format!(
                        "Changed nonregular artifact {} requires full admission",
                        path.display()
                    ))
                    .into());
                }
                entries.insert(path, (mode == "100755", mode == "120000", oid.to_owned()));
            }
        }
        let oids: BTreeSet<_> = entries.values().map(|(_, _, oid)| oid.clone()).collect();
        let mut blobs = BTreeMap::new();
        if !oids.is_empty() {
            let input = oids.iter().map(|s| format!("{s}\n")).collect::<String>();
            let bytes = (self.run)(&["cat-file", "--batch"], Some(input.as_bytes()))?;
            let mut cursor = bytes.as_slice();
            for oid in oids {
                let end = cursor
                    .iter()
                    .position(|b| *b == b'\n')
                    .ok_or_else(|| git_error("Missing blob header"))?;
                let mut fields = git_utf8(&cursor[..end])?.split_whitespace();
                if fields.next() != Some(oid.as_str()) || fields.next() != Some("blob") {
                    return Err(git_error("Invalid blob batch reply").into());
                }
                let length: usize = fields
                    .next()
                    .ok_or_else(|| git_error("Missing blob length"))?
                    .parse()
                    .map_err(|_| git_error("Invalid blob length"))?;
                cursor = &cursor[end + 1..];
                if cursor.len() <= length || cursor[length] != b'\n' {
                    return Err(git_error("Incomplete blob reply").into());
                }
                blobs.insert(oid, std::sync::Arc::new(cursor[..length].to_vec()));
                cursor = &cursor[length + 1..];
            }
        }
        Ok(entries
            .into_iter()
            .map(|(path, (executable, symlink, oid))| {
                (
                    path,
                    TreeFile {
                        bytes: std::sync::Arc::clone(&blobs[&oid]),
                        executable,
                        symlink,
                    },
                )
            })
            .collect())
    }
}

fn git_error(message: impl Into<String>) -> Error {
    Error::GitCommandError(message.into())
}
fn git_utf8(bytes: &[u8]) -> Result<&str, Error> {
    std::str::from_utf8(bytes).map_err(|e| git_error(e.to_string()))
}
