//! Physical filesystem identity shared by workspace and Git-root consumers.
use std::{
    io,
    path::{Path, PathBuf},
};

/// Resolve an existing path to the filesystem's canonical absolute spelling.
///
/// The standard library selects the OS implementation at compile time. This
/// handles ancestor aliases such as macOS /var and Windows path prefixes
/// without hand-written platform substitutions.
///
/// Do not use physical identity to rewrite logical model identifiers or paths
/// for files that do not exist yet. Containment checks must still compare the
/// result with the declared logical destination to reject component/source
/// symlinks.
pub fn canonicalize_existing(path: &Path) -> io::Result<PathBuf> {
    std::fs::canonicalize(path)
}

/// Create a temporary directory whose path already has physical identity.
///
/// Normalize the OS-selected temporary parent before creation so TempDir keeps
/// ownership and cleanup while fixtures never inherit an ancestor alias.
pub fn temporary_directory() -> io::Result<tempfile::TempDir> {
    tempfile::Builder::new().tempdir_in(canonicalize_existing(&std::env::temp_dir())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temporary_directory_has_physical_identity_and_cleans_up() {
        let temporary = temporary_directory().expect("temporary directory");
        let path = temporary.path().to_path_buf();
        assert_eq!(path, std::fs::canonicalize(&path).expect("physical path"));
        drop(temporary);
        assert!(!path.exists());
    }

    #[test]
    fn existing_paths_have_one_identity_and_missing_paths_fail() {
        let temporary = tempfile::tempdir().expect("temporary directory");
        let child = temporary.path().join("child");
        std::fs::create_dir(&child).expect("child directory");
        let expected = canonicalize_existing(temporary.path()).expect("physical directory");
        assert_eq!(
            canonicalize_existing(&child.join("..")).expect("parent"),
            expected
        );
        assert_eq!(
            canonicalize_existing(&expected).expect("idempotent resolution"),
            expected
        );
        assert_eq!(
            canonicalize_existing(&child.join("missing"))
                .expect_err("missing path")
                .kind(),
            io::ErrorKind::NotFound
        );
    }

    #[cfg(unix)]
    #[test]
    fn ancestor_aliases_resolve_to_the_same_physical_directory() {
        use std::os::unix::fs::symlink;
        let temporary = tempfile::tempdir().expect("temporary directory");
        let physical = canonicalize_existing(temporary.path()).expect("physical directory");
        std::fs::create_dir_all(physical.join("real/nested")).expect("nested directory");
        symlink(physical.join("real"), physical.join("alias")).expect("ancestor alias");
        assert_eq!(
            canonicalize_existing(&physical.join("alias/nested")).expect("alias"),
            physical.join("real/nested")
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_regular_and_verbatim_paths_have_the_same_identity() {
        use std::{
            ffi::OsString,
            path::{Component, Prefix},
        };
        let temporary = tempfile::tempdir().expect("temporary directory");
        let physical = canonicalize_existing(temporary.path()).expect("physical directory");
        let mut components = physical.components();
        let Some(Component::Prefix(prefix)) = components.next() else {
            panic!("Windows prefix missing");
        };
        let mut regular = match prefix.kind() {
            Prefix::VerbatimDisk(letter) => PathBuf::from(format!("{}:", char::from(letter))),
            Prefix::VerbatimUNC(server, share) => {
                let mut prefix = OsString::from(r"\\");
                prefix.push(server);
                prefix.push(r"\");
                prefix.push(share);
                PathBuf::from(prefix)
            }
            other => panic!("Expected a canonical Windows prefix, got {other:?}"),
        };
        for component in components {
            regular.push(component.as_os_str());
        }
        assert_eq!(
            canonicalize_existing(&regular).expect("regular spelling"),
            physical
        );
    }
}
