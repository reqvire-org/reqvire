//! Records the actual filesystem observations consumed by a synchronous model
//! build. The parser and semantic loader use the same reads with or without an
//! observation guard; historical and uncached builds do not create cache entries.

use std::cell::RefCell;
use std::collections::BTreeMap;
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
    pub fn is_current(&self) -> bool {
        !self.changed
            && self.values.iter().all(|((path, _), previous)| {
                let current = match previous {
                    Observation::Exists(_) => Observation::Exists(path.exists()),
                    Observation::Content(_) => content_observation(&std::fs::read_to_string(path)),
                };
                current == *previous
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
    let result = std::fs::read_to_string(path);
    record(path, content_observation(&result));
    result
}

pub fn exists(path: &Path) -> bool {
    let exists = path.exists();
    record(path, Observation::Exists(exists));
    exists
}
