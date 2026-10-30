//! Records filesystem observations consumed by a synchronous model build.
//!
//! The parser and semantic loader use the same reads with or without an
//! observation guard; historical and uncached builds do not create cache entries.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Observation {
    Content(Result<String, (std::io::ErrorKind, String)>),
    Exists(bool),
}

#[derive(Clone, Debug, Default)]
pub struct Inputs {
    // A path can be consulted for both existence and content.
    values: BTreeMap<(PathBuf, bool), Observation>,
    changed: bool,
    semantic_sources: BTreeMap<PathBuf, BTreeSet<PathBuf>>,
    semantic_candidates: BTreeMap<PathBuf, BTreeSet<PathBuf>>,
    reference_targets: BTreeMap<PathBuf, BTreeSet<PathBuf>>,
}

thread_local! {
    static OBSERVATIONS: RefCell<Option<Inputs>> = const { RefCell::new(None) };
}

pub struct InputObservationGuard;
impl InputObservationGuard {
    pub fn start() -> Self {
        OBSERVATIONS.with(|slot| {
            assert!(slot.borrow().is_none(), "nested model input observation");
            *slot.borrow_mut() = Some(Inputs::default());
        });
        Self
    }

    pub fn finish(self) -> Inputs {
        OBSERVATIONS.with(|slot| {
            slot.borrow_mut()
                .take()
                .expect("active model input observation")
        })
    }
}
impl Drop for InputObservationGuard {
    fn drop(&mut self) {
        OBSERVATIONS.with(|slot| *slot.borrow_mut() = None);
    }
}

impl Inputs {
    pub fn semantic_sources(&self) -> impl Iterator<Item = (&PathBuf, &BTreeSet<PathBuf>)> {
        self.semantic_sources.iter()
    }
    pub fn semantic_candidates(&self) -> impl Iterator<Item = (&PathBuf, &BTreeSet<PathBuf>)> {
        self.semantic_candidates.iter()
    }
    pub fn reference_targets(&self) -> impl Iterator<Item = (&PathBuf, &BTreeSet<PathBuf>)> {
        self.reference_targets.iter()
    }
    pub fn content_is_current(&self, path: &Path) -> bool {
        self.values.get(&(path.to_path_buf(), true))
            == Some(&content_observation(&std::fs::read_to_string(path)))
    }
    pub fn existence_is_current(&self, path: &Path) -> bool {
        self.values.get(&(path.to_path_buf(), false)) == Some(&Observation::Exists(path.exists()))
    }
    pub fn is_current(&self) -> bool {
        !self.changed
            && self.values.keys().all(|(path, content)| {
                if *content {
                    self.content_is_current(path)
                } else {
                    self.existence_is_current(path)
                }
            })
    }
}

fn content_observation(result: &std::io::Result<String>) -> Observation {
    Observation::Content(match result {
        Ok(content) => Ok(crate::utils::hash_content(content)),
        Err(error) => Err((error.kind(), error.to_string())),
    })
}

fn record(path: &Path, value: Observation) {
    OBSERVATIONS.with(|slot| {
        if let Some(inputs) = slot.borrow_mut().as_mut() {
            let key = (
                crate::workspace::absolute_logical_path(path),
                matches!(value, Observation::Content(_)),
            );
            if let Some(previous) = inputs.values.insert(key, value.clone()) {
                inputs.changed |= previous != value;
            }
        }
    });
}

pub fn read_to_string(path: &Path) -> std::io::Result<String> {
    let result = crate::mutation_io::read_to_string(path);
    record(path, content_observation(&result));
    result
}

pub fn exists(path: &Path) -> bool {
    let exists = crate::mutation_io::exists(path);
    record(path, Observation::Exists(exists));
    exists
}

pub fn read_semantic_source(path: &Path, owner: &Path) -> std::io::Result<String> {
    let result = read_to_string(path);
    OBSERVATIONS.with(|slot| {
        if let Some(inputs) = slot.borrow_mut().as_mut() {
            inputs
                .semantic_sources
                .entry(crate::workspace::absolute_logical_path(path))
                .or_default()
                .insert(crate::workspace::absolute_logical_path(owner));
        }
    });
    result
}

pub fn semantic_candidate_exists(path: &Path, owner: &Path) -> bool {
    let result = exists(path);
    OBSERVATIONS.with(|slot| {
        if let Some(inputs) = slot.borrow_mut().as_mut() {
            inputs
                .semantic_candidates
                .entry(crate::workspace::absolute_logical_path(path))
                .or_default()
                .insert(crate::workspace::absolute_logical_path(owner));
        }
    });
    result
}

pub fn reference_target_exists(path: &Path, owner: &Path) -> bool {
    let result = exists(path);
    OBSERVATIONS.with(|slot| {
        if let Some(inputs) = slot.borrow_mut().as_mut() {
            inputs
                .reference_targets
                .entry(crate::workspace::absolute_logical_path(path))
                .or_default()
                .insert(crate::workspace::absolute_logical_path(owner));
        }
    });
    result
}
