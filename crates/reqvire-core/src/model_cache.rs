//! Shared current-workspace model cache.
//!
//! Source identity includes matching policy, workspace context, Markdown bytes and dependencies
//! consumed by validation. It is distinct from public semantic model revisions.

use crate::error::ReqvireError;
use crate::exclusions::ExclusionSet as GlobSet;
use crate::model::{ModelBuildOptions, ModelManager};
use crate::model_inputs::{InputObservationGuard, Inputs};
use crate::{tool_interface, utils};
use log::debug;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex};

struct CachedModel {
    key: CacheKey,
    inputs: Inputs,
    model: Arc<ModelManager>,
}

#[derive(Clone, Eq, PartialEq)]
struct CacheKey {
    options: ModelBuildOptions,
    workspace_root: PathBuf,
    git_worktrees: Vec<GitWorktreeFingerprint>,
    excluded_patterns: Vec<(String, String)>,
    reqvire_version: &'static str,
    tool_contract_version: &'static str,
    files: BTreeMap<PathBuf, String>,
}

#[derive(Clone, Eq, PartialEq)]
struct GitWorktreeFingerprint {
    workspace_relative_root: PathBuf,
    head: Option<String>,
    dirty: bool,
}

struct CacheState {
    entry: Option<Arc<CachedModel>>,
    generation: u64,
    flights: Vec<Arc<Flight>>,
}

struct Flight {
    key: CacheKey,
    generation: u64,
    outcome: Mutex<Option<Outcome>>,
    ready: Condvar,
}

#[derive(Clone)]
enum Outcome {
    Complete(Arc<Result<Arc<ModelManager>, ReqvireError>>),
    Retry,
}

// Ensure waiters are released even if a builder unwinds before completing.
struct FlightGuard(Arc<Flight>);
impl Drop for FlightGuard {
    fn drop(&mut self) {
        let mut state = MODEL_CACHE.lock().expect("model cache mutex poisoned");
        let mut outcome = self.0.outcome.lock().expect("model flight mutex poisoned");
        if outcome.is_none() {
            *outcome = Some(Outcome::Retry);
        }
        drop(outcome);
        state.flights.retain(|other| !Arc::ptr_eq(other, &self.0));
        drop(state);
        self.0.ready.notify_all();
    }
}

static MODEL_CACHE: Mutex<CacheState> = Mutex::new(CacheState {
    entry: None,
    generation: 0,
    flights: Vec::new(),
});
const MAX_BUILD_ATTEMPTS: usize = 3;

fn capture(
    supplied: &GlobSet,
    options: ModelBuildOptions,
) -> Result<(GlobSet, CacheKey), ReqvireError> {
    let exclusions = supplied.refreshed();
    let scope = crate::workspace::WorkspaceScope::discover()?;
    let mut files = BTreeMap::new();
    for path in utils::scan_markdown_files(None, &exclusions)? {
        let content = crate::model_inputs::read_to_string(&path)?;
        files.insert(
            utils::get_relative_path(&path)?,
            crate::utils::hash_content(&content),
        );
    }
    let key = CacheKey {
        options,
        workspace_root: scope.root.clone(),
        git_worktrees: git_worktree_fingerprints(&scope),
        excluded_patterns: exclusions.identity(),
        reqvire_version: env!("CARGO_PKG_VERSION"),
        tool_contract_version: tool_interface::TOOL_CONTRACT_VERSION,
        files,
    };
    Ok((exclusions, key))
}

/// Load a complete graph, pages and semantic store.
///
/// Identical concurrent misses share one result, including validation failures. Historical builds bypass
/// this cache. External changes are checked on each load, without a watcher.
pub fn load_cached_model(
    excluded_filename_patterns: &GlobSet,
    options: ModelBuildOptions,
) -> Result<Arc<ModelManager>, ReqvireError> {
    if let Some(model) = crate::mutation_io::model() { return Ok(model); }
    for _ in 0..MAX_BUILD_ATTEMPTS {
        let generation = MODEL_CACHE
            .lock()
            .expect("model cache mutex poisoned")
            .generation;
        let observation_guard = InputObservationGuard::start();
        let (exclusions, key) = capture(excluded_filename_patterns, options)?;
        #[cfg(test)]
        tests::checkpoint("captured");

        let cached = MODEL_CACHE
            .lock()
            .expect("model cache mutex poisoned")
            .entry
            .clone();
        if let Some(cached) = &cached {
            if cached.key == key && cached.inputs.is_current() {
                let observed = observation_guard.finish();
                if observed.is_current()
                    && MODEL_CACHE
                        .lock()
                        .expect("model cache mutex poisoned")
                        .generation
                        == generation
                {
                    debug!(
                        "model cache hit ({} files, lenient={}, size_estimates={})",
                        key.files.len(),
                        options.lenient,
                        options.with_size_estimates
                    );
                    return Ok(cached.model.clone());
                }
                continue;
            }
        }

        #[cfg(test)]
        tests::checkpoint("lookup-miss");
        let (flight, builder) = {
            let mut state = MODEL_CACHE.lock().expect("model cache mutex poisoned");
            if state.generation != generation {
                continue;
            }
            // A build may have completed while this caller checked dependency
            // freshness outside the lock. Reconsider that entry before starting
            // another build for the same observation.
            if state.entry.as_ref().is_some_and(|entry| {
                entry.key == key && !cached.as_ref().is_some_and(|old| Arc::ptr_eq(old, entry))
            }) {
                continue;
            }
            let existing = state
                .flights
                .iter()
                .find(|flight| flight.generation == generation && flight.key == key)
                .cloned();
            let registration = existing.map_or_else(
                || {
                    let flight = Arc::new(Flight {
                        key: key.clone(),
                        generation,
                        outcome: Mutex::new(None),
                        ready: Condvar::new(),
                    });
                    state.flights.push(Arc::clone(&flight));
                    (flight, true)
                },
                |flight| (flight, false),
            );
            drop(state);
            registration
        };
        if !builder {
            drop(observation_guard);
            #[cfg(test)]
            tests::checkpoint("waiting");
            let mut outcome = flight.outcome.lock().expect("model flight mutex poisoned");
            while outcome.is_none() {
                outcome = flight
                    .ready
                    .wait(outcome)
                    .expect("model flight mutex poisoned");
            }
            let completed = outcome.as_ref().expect("completed model flight").clone();
            drop(outcome);
            match completed {
                Outcome::Complete(result) => return result.as_ref().clone(),
                Outcome::Retry => continue,
            }
        }

        let _flight_guard = FlightGuard(Arc::clone(&flight));
        debug!(
            "model cache miss ({} files, lenient={}, size_estimates={})",
            key.files.len(),
            options.lenient,
            options.with_size_estimates
        );
        #[cfg(test)]
        tests::checkpoint("build-start");
        let mut model = ModelManager::new();
        let result = model
            .parse_and_validate_with_options(None, &exclusions, options)
            .map(|_| Arc::new(model));
        #[cfg(test)]
        tests::checkpoint("built");
        let inputs = observation_guard.finish();
        // Compare both the inventory/policy/context and every value consumed by
        // the build. Repeated inconsistent reads also reject the candidate (ABA).
        let current = capture(excluded_filename_patterns, options)
            .is_ok_and(|(_, after)| after == key)
            && inputs.is_current();
        let mut state = MODEL_CACHE.lock().expect("model cache mutex poisoned");
        let outcome = if current && state.generation == generation {
            if let Ok(model) = &result {
                state.entry = Some(Arc::new(CachedModel {
                    key,
                    inputs,
                    model: model.clone(),
                }));
            }
            Outcome::Complete(Arc::new(result))
        } else {
            Outcome::Retry
        };
        *flight.outcome.lock().expect("model flight mutex poisoned") = Some(outcome.clone());
        state.flights.retain(|other| !Arc::ptr_eq(other, &flight));
        flight.ready.notify_all();
        drop(state);
        match outcome {
            Outcome::Complete(result) => return result.as_ref().clone(),
            Outcome::Retry => continue,
        }
    }
    Err(ReqvireError::ProcessError(
        "Model inputs changed during construction; retry once the workspace is stable".to_owned(),
    ))
}

fn git_worktree_fingerprints(
    scope: &crate::workspace::WorkspaceScope,
) -> Vec<GitWorktreeFingerprint> {
    scope
        .git_worktrees
        .iter()
        .map(|root| {
            let workspace_relative_root = root
                .strip_prefix(&scope.root)
                .ok()
                .map(PathBuf::from)
                .filter(|path| !path.as_os_str().is_empty())
                .unwrap_or_else(|| PathBuf::from("."));
            let head = git_output_in_dir(root, ["rev-parse", "HEAD"]);
            let dirty = git_output_in_dir(root, ["status", "--porcelain"])
                .as_ref()
                .is_some_and(|status| !status.trim().is_empty());
            GitWorktreeFingerprint {
                workspace_relative_root,
                head,
                dirty,
            }
        })
        .collect()
}

fn git_output_in_dir<const N: usize>(
    directory: &std::path::Path,
    args: [&str; N],
) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(directory)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Supersede in-flight reads before a controlled write acquires its working
/// model. An already completed snapshot remains reusable until persistence.
pub(crate) fn begin_write() {
    let mut state = MODEL_CACHE.lock().expect("model cache mutex poisoned");
    supersede(&mut state);
}

/// Invalidation is a publication barrier as well as eviction: an older build
/// cannot put its result back after a persisted or partially persisted write.
pub fn invalidate() {
    let mut state = MODEL_CACHE.lock().expect("model cache mutex poisoned");
    supersede(&mut state);
    state.entry = None;
}

fn supersede(state: &mut CacheState) {
    state.generation += 1;
    for flight in state.flights.drain(..) {
        *flight.outcome.lock().expect("model flight mutex poisoned") = Some(Outcome::Retry);
        flight.ready.notify_all();
    }
}

#[cfg(test)]
#[path = "model_cache_tests.rs"]
pub(crate) mod tests;
