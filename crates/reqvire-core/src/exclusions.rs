//! Exclusion matchers retain their immutable inputs, independently of the
//! compiled matcher's per-thread regex caches.

use globset::{Glob, GlobSet};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct ExclusionSet {
    matcher: GlobSet,
    globs: Vec<Glob>,
    additional: Vec<Glob>,
    owned: Vec<(std::path::PathBuf, GlobSet, Vec<Glob>)>,
}

impl ExclusionSet {
    pub fn is_match(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        self.matcher.is_match(path)
            || self
                .owned
                .iter()
                .filter(|(root, _, _)| path.starts_with(root))
                .max_by_key(|(root, _, _)| root.components().count())
                .is_some_and(|(root, matcher, _)| {
                    path.strip_prefix(root).is_ok_and(|p| matcher.is_match(p))
                })
    }
    pub fn is_global_match(&self, path: impl AsRef<Path>) -> bool {
        self.matcher.is_match(path)
    }

    /// Reload workspace policy, preserving only explicitly supplied extra rules.
    pub fn refreshed(&self) -> Self {
        if crate::mutation_io::active() {
            return self.clone();
        }
        let mut current = crate::config::get_excluded_filename_patterns_glob_set();
        current.globs.extend(self.additional.iter().cloned());
        current.additional = self.additional.clone();
        current.matcher = compile(&current.globs).expect("previously compiled exclusions");
        current
    }

    pub(crate) fn workspace_policy(mut self) -> Self {
        self.additional.clear();
        self
    }

    pub(crate) fn with_owners(mut self, owners: Vec<(std::path::PathBuf, Vec<Glob>)>) -> Self {
        self.owned = owners
            .into_iter()
            .map(|(path, globs)| {
                (
                    path,
                    compile(&globs).expect("validated owner exclusions"),
                    globs,
                )
            })
            .collect();
        self
    }

    /// Regex source encodes the effective matching options (case sensitivity,
    /// separator handling, escaping, etc.). Neither runtime state nor rule order
    /// participates in the identity of this any-match set.
    pub(crate) fn identity(&self) -> Vec<(String, String)> {
        let mut identity: Vec<_> = self
            .globs
            .iter()
            .map(|glob| (glob.glob().to_owned(), glob.regex().to_owned()))
            .collect();
        for (path, _, globs) in &self.owned {
            identity.extend(globs.iter().map(|glob| {
                (
                    format!(
                        "owner:{}",
                        serde_json::to_string(&(path, glob.glob())).expect("owner policy identity")
                    ),
                    glob.regex().to_owned(),
                )
            }));
        }
        identity.sort();
        identity.dedup();
        identity
    }
}

#[derive(Default)]
pub struct ExclusionSetBuilder {
    globs: Vec<Glob>,
}

impl ExclusionSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, glob: Glob) -> &mut Self {
        self.globs.push(glob);
        self
    }

    pub fn build(&self) -> Result<ExclusionSet, globset::Error> {
        Ok(ExclusionSet {
            matcher: compile(&self.globs)?,
            globs: self.globs.clone(),
            additional: self.globs.clone(),
            owned: Vec::new(),
        })
    }
}

fn compile(globs: &[Glob]) -> Result<GlobSet, globset::Error> {
    let mut builder = globset::GlobSetBuilder::new();
    for glob in globs {
        builder.add(glob.clone());
    }
    builder.build()
}
