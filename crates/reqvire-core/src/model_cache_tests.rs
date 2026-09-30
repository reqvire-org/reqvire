//! Deterministic cache contract regressions. Checkpoints compile only in unit
//! tests; the shipped CLI and the HTTP e2e tests have no hooks or behavior changes.
//! Each case runs in its own test process because CWD and MODEL_CACHE are global.

use super::*;
use crate::exclusions::ExclusionSetBuilder as GlobSetBuilder;
use globset::{Glob, GlobBuilder};
use oxigraph::sparql::{QueryResults, SparqlEvaluator};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Barrier, Condvar};
use std::thread;
use std::time::{Duration, Instant};

const MODEL: &str = include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt");
const ONTOLOGY: &str =
    include_str!("../../../tests/test-cache-integration/fixtures/ontology.md.txt");
const EXTERNAL: &str = include_str!("../../../tests/test-cache-integration/fixtures/external.ttl");
const PAGE: &str = include_str!("../../../tests/test-cache-integration/fixtures/page.md.txt");
const BROKEN: &str = include_str!("../../../tests/test-cache-integration/fixtures/broken.md.txt");
const REFERENCE: &str =
    include_str!("../../../tests/test-cache-integration/fixtures/reference.md.txt");
const EVIDENCE: &str =
    include_str!("../../../tests/test-cache-integration/fixtures/evidence.md.txt");
const STRICT: ModelBuildOptions = ModelBuildOptions {
    lenient: false,
    with_size_estimates: false,
};
const DEADLINE: Duration = Duration::from_secs(10);
type Observer = Arc<dyn Fn(&str) + Send + Sync>;
static OBSERVER: Mutex<Option<Observer>> = Mutex::new(None);
static BUILDS: AtomicUsize = AtomicUsize::new(0);

pub(super) fn checkpoint(phase: &str) {
    if phase == "build-start" {
        BUILDS.fetch_add(1, Ordering::SeqCst);
    }
    let observer = OBSERVER
        .lock()
        .expect("test mutex should not be poisoned")
        .clone();
    if let Some(observer) = observer {
        observer(phase);
    }
}

fn isolated(name: &str, test: impl FnOnce()) {
    if std::env::var("REQVIRE_CACHE_TEST_CHILD").as_deref() == Ok(name) {
        test();
        return;
    }
    let mut child = Command::new(std::env::current_exe().expect("locate test executable"))
        .args([
            "--exact",
            &format!("model_cache::tests::{name}"),
            "--nocapture",
        ])
        .env("REQVIRE_CACHE_TEST_CHILD", name)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn isolated test process");
    let deadline = Instant::now() + Duration::from_secs(30);
    while child.try_wait().expect("check child test status").is_none() {
        if Instant::now() >= deadline {
            child.kill().expect("terminate timed-out child test");
            let output = child.wait_with_output().expect("collect child test output");
            panic!(
                "cache test exceeded deadline: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().expect("collect child test output");
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

macro_rules! case {
    ($name:ident, $body:block) => {
        #[test]
        fn $name() {
            isolated(stringify!($name), || $body);
        }
    };
}

struct Workspace {
    _dir: tempfile::TempDir,
    original: PathBuf,
}
impl Workspace {
    fn new() -> Self {
        let original = std::env::current_dir().expect("read test working directory");
        let dir = tempfile::tempdir().expect("create temporary test workspace");
        std::env::set_current_dir(dir.path()).expect("change test working directory");
        git(&["init", "-q"]);
        git(&["config", "user.email", "cache-test@example.test"]);
        git(&["config", "user.name", "Cache Test"]);
        git(&[
            "remote",
            "add",
            "origin",
            "https://github.com/example/cache-test.git",
        ]);
        std::fs::write("Model.md", MODEL).expect("write test fixture");
        std::fs::write("dirty.txt", "Keep metadata dirty throughout each case.\n")
            .expect("write test fixture");
        invalidate();
        BUILDS.store(0, Ordering::SeqCst);
        Self {
            _dir: dir,
            original,
        }
    }
}
impl Drop for Workspace {
    fn drop(&mut self) {
        *OBSERVER.lock().expect("test mutex should not be poisoned") = None;
        invalidate();
        std::env::set_current_dir(&self.original).expect("change test working directory");
    }
}
fn git(args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("run test subprocess");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
fn patterns(values: &[&str]) -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for value in values {
        builder.add(Glob::new(value).expect("compile test exclusion pattern"));
    }
    builder.build().expect("build test configuration")
}
fn load() -> Result<ModelManager, ReqvireError> {
    load_cached_model(&patterns(&[]), STRICT)
}
fn builds() -> usize {
    BUILDS.load(Ordering::SeqCst)
}
fn content(model: &ModelManager) -> String {
    model
        .graph_registry
        .get_element_by_name("Cache Subject")
        .expect("expected test element to exist")
        .content
        .clone()
}
fn labels(model: &ModelManager) -> Vec<String> {
    let query = "SELECT ?label WHERE { <https://example.test/external#ExternalResource> <http://www.w3.org/2000/01/rdf-schema#label> ?label }";
    let result = SparqlEvaluator::new()
        .parse_query(query)
        .expect("parse test SPARQL query")
        .on_store(
            model
                .semantic_store
                .as_ref()
                .expect("expected semantic store in test model")
                .store(true, true),
        )
        .execute()
        .expect("execute test SPARQL query");
    match result {
        QueryResults::Solutions(rows) => rows
            .map(|row| {
                row.expect("read SPARQL solution row")
                    .get("label")
                    .expect("expected requested field in test response")
                    .to_string()
            })
            .collect(),
        _ => panic!("expected SELECT solutions"),
    }
}

// Pause exactly once on the candidate worker, never while holding the cache lock.
fn pause(phase: &'static str) -> (mpsc::Receiver<()>, mpsc::SyncSender<()>) {
    let (reached_tx, reached_rx) = mpsc::sync_channel(1);
    let (release_tx, release_rx) = mpsc::sync_channel(1);
    let release_rx = Mutex::new(release_rx);
    let used = AtomicBool::new(false);
    *OBSERVER.lock().expect("test mutex should not be poisoned") = Some(Arc::new(move |event| {
        if event == phase
            && thread::current().name() == Some("candidate")
            && !used.swap(true, Ordering::SeqCst)
        {
            reached_tx
                .send(())
                .expect("send test synchronization event");
            release_rx
                .lock()
                .expect("test mutex should not be poisoned")
                .recv_timeout(DEADLINE)
                .expect("candidate release deadline");
        }
    }));
    (reached_rx, release_tx)
}
fn candidate() -> thread::JoinHandle<Result<ModelManager, ReqvireError>> {
    thread::Builder::new()
        .name("candidate".into())
        .spawn(load)
        .expect("candidate: expected success")
}

case!(equivalent_regex_matcher_reconstruction_reuses_build, {
    let _workspace = Workspace::new();
    load_cached_model(&patterns(&["**/*[0-9]*.md"]), STRICT).expect("load valid test model");
    load_cached_model(&patterns(&["**/*[0-9]*.md"]), STRICT).expect("load valid test model");
    assert_eq!(builds(), 1, "equivalent policy must not rebuild");
});

case!(regex_matcher_worker_changes_do_not_change_identity, {
    let _workspace = Workspace::new();
    let matcher = Arc::new(patterns(&["**/*[0-9]*.md"]));
    load_cached_model(&matcher, STRICT).expect("load valid test model");
    for _ in 0..3 {
        let worker_matcher = Arc::clone(&matcher);
        thread::spawn(move || {
            load_cached_model(&worker_matcher, STRICT).expect("load valid test model")
        })
        .join()
        .expect("worker thread should complete without panicking");
    }
    assert_eq!(
        builds(),
        1,
        "fresh worker threads must reuse unchanged inputs"
    );
});

case!(rule_order_and_duplicates_preserve_effective_identity, {
    let _workspace = Workspace::new();
    load_cached_model(&patterns(&["unused-a.md", "unused-b.md"]), STRICT)
        .expect("load valid test model");
    load_cached_model(
        &patterns(&["unused-b.md", "unused-a.md", "unused-a.md"]),
        STRICT,
    )
    .expect("load valid test model");
    assert_eq!(
        builds(),
        1,
        "any-match order and duplicates do not change policy"
    );
});

case!(different_same_count_rules_and_options_do_not_alias, {
    let _workspace = Workspace::new();
    load_cached_model(&patterns(&["unused-a.md"]), STRICT).expect("load valid test model");
    load_cached_model(&patterns(&["unused-b.md"]), STRICT).expect("load valid test model");
    assert_eq!(builds(), 2);
    for case_insensitive in [false, true] {
        let matcher = GlobSetBuilder::new()
            .add(
                GlobBuilder::new("unused-c.md")
                    .case_insensitive(case_insensitive)
                    .build()
                    .expect("build test configuration"),
            )
            .build()
            .expect("build test configuration");
        load_cached_model(&matcher, STRICT).expect("load valid test model");
    }
    assert_eq!(
        builds(),
        4,
        "matching options matter even with identical selected files"
    );
});

case!(strict_lenient_and_size_modes_remain_separate, {
    let _workspace = Workspace::new();
    assert!(load()
        .expect("load valid test model")
        .graph_registry
        .get_element_by_name("Cache Subject")
        .expect("expected test element to exist")
        .size_estimate
        .is_none());
    let sized = load_cached_model(
        &patterns(&[]),
        ModelBuildOptions {
            with_size_estimates: true,
            ..STRICT
        },
    )
    .expect("load valid test model");
    assert!(sized
        .graph_registry
        .get_element_by_name("Cache Subject")
        .expect("expected test element to exist")
        .size_estimate
        .is_some());
    std::fs::write("Broken.md", BROKEN).expect("write test fixture");
    load_cached_model(
        &patterns(&[]),
        ModelBuildOptions {
            lenient: true,
            ..STRICT
        },
    )
    .expect("load valid test model");
    assert!(load().is_err(), "lenient model cannot satisfy strict load");
    std::fs::remove_file("Broken.md").expect("remove test fixture");
    assert!(load().is_ok());
});

case!(workspace_and_git_context_do_not_alias, {
    let first = Workspace::new();
    load().expect("load valid test model");
    git(&["add", "."]);
    git(&["commit", "-qm", "initial"]);
    load().expect("load valid test model");
    assert_eq!(builds(), 2, "HEAD/dirty change must change identity");
    let second = tempfile::tempdir().expect("create temporary test workspace");
    std::env::set_current_dir(second.path()).expect("change test working directory");
    git(&["init", "-q"]);
    std::fs::write("Model.md", MODEL).expect("write test fixture");
    load().expect("load valid test model");
    assert_eq!(
        builds(),
        3,
        "a different effective workspace cannot reuse the first"
    );
    std::env::set_current_dir(first._dir.path()).expect("change test working directory");
});

case!(historical_build_does_not_consume_or_populate_live_cache, {
    let _workspace = Workspace::new();
    git(&["add", "."]);
    git(&["commit", "-qm", "initial"]);
    std::fs::write("Model.md", MODEL.replace("alpha", "omega")).expect("write test fixture");
    let current = load().expect("load valid test model");
    let before = builds();
    let mut historical = ModelManager::new();
    historical
        .parse_and_validate_with_options(Some("HEAD"), &patterns(&[]), STRICT)
        .expect("candidate: expected success");
    assert!(content(&historical).contains("alpha"));
    assert!(content(&current).contains("omega"));
    crate::operations::change_impact_report(&current.graph_registry, "HEAD", &patterns(&[]))
        .expect("candidate: expected success");
    assert!(content(&load().expect("load valid test model")).contains("omega"));
    assert_eq!(
        builds(),
        before,
        "history construction must neither consume nor evict the live entry"
    );
});

fn concurrent_builds(invalid: bool) {
    let _workspace = Workspace::new();
    if invalid {
        std::fs::write("Broken.md", BROKEN).expect("write test fixture");
    }
    let captured = Arc::new(Barrier::new(4));
    let release = Arc::new((Mutex::new(false), Condvar::new()));
    let (arrived_tx, arrived_rx) = mpsc::channel();
    let observer_release = Arc::clone(&release);
    *OBSERVER.lock().expect("test mutex should not be poisoned") = Some(Arc::new(move |phase| {
        if phase == "captured" {
            captured.wait();
        }
        // A single-flight implementation should report "waiting" when a request
        // joins an existing build. This makes arrival observable without sleeps.
        if phase == "build-start" || phase == "waiting" {
            arrived_tx
                .send(phase.to_owned())
                .expect("send test synchronization event");
            if phase == "build-start" {
                let (lock, cv) = &*observer_release;
                let (released, timeout) = cv
                    .wait_timeout_while(
                        lock.lock().expect("test mutex should not be poisoned"),
                        DEADLINE,
                        |ready| !*ready,
                    )
                    .expect("test mutex should not be poisoned");
                assert!(
                    *released && !timeout.timed_out(),
                    "shared build release deadline"
                );
            }
        }
    }));
    // Start every contender before waiting for arrivals or joining any worker.
    let workers: [_; 4] = std::array::from_fn(|_| thread::spawn(load));
    let arrivals: Vec<_> = (0..4)
        .map(|_| {
            arrived_rx
                .recv_timeout(DEADLINE)
                .expect("every contender must register")
        })
        .collect();
    let (lock, cv) = &*release;
    *lock.lock().expect("test mutex should not be poisoned") = true;
    cv.notify_all();
    let results: Vec<_> = workers
        .into_iter()
        .map(|worker| {
            worker
                .join()
                .expect("worker thread should complete without panicking")
        })
        .collect();
    *OBSERVER.lock().expect("test mutex should not be poisoned") = None;
    let count = builds();
    assert!(results.iter().all(|result| result.is_err() == invalid));
    if invalid {
        std::fs::remove_file("Broken.md").expect("remove test fixture");
    }
    assert!(load().is_ok(), "subsequent load must recover");
    assert_eq!(count, 1, "concurrent builds: {arrivals:?}");
}
case!(concurrent_cold_reads_share_one_completed_build, {
    concurrent_builds(false);
});
case!(concurrent_failed_reads_share_failure_and_recover, {
    concurrent_builds(true);
});

case!(completed_build_between_lookup_and_registration_is_reused, {
    let _workspace = Workspace::new();
    let (reached, release) = pause("lookup-miss");
    let worker = candidate();
    reached
        .recv_timeout(DEADLINE)
        .expect("receive test synchronization event before deadline");
    load().expect("load valid test model");
    release.send(()).expect("send test synchronization event");
    assert!(content(
        &worker
            .join()
            .expect("worker thread should complete without panicking")
            .expect("candidate model build should succeed")
    )
    .contains("alpha"));
    assert_eq!(builds(), 1, "late contender started a duplicate build");
});

case!(source_change_after_capture_cannot_mislabel_candidate, {
    let _workspace = Workspace::new();
    let (reached, release) = pause("captured");
    let worker = candidate();
    reached
        .recv_timeout(DEADLINE)
        .expect("receive test synchronization event before deadline");
    std::fs::write("Model.md", MODEL.replace("alpha", "omega")).expect("write test fixture");
    release.send(()).expect("send test synchronization event");
    assert!(content(
        &worker
            .join()
            .expect("worker thread should complete without panicking")
            .expect("candidate model build should succeed")
    )
    .contains("omega"));
    let before = builds();
    assert!(content(&load().expect("load valid test model")).contains("omega"));
    assert_eq!(
        builds(),
        before,
        "published key must describe the consumed source"
    );
});

case!(completed_candidate_rechecks_sources_before_publication, {
    let _workspace = Workspace::new();
    let (reached, release) = pause("built");
    let worker = candidate();
    reached
        .recv_timeout(DEADLINE)
        .expect("receive test synchronization event before deadline");
    std::fs::write("Model.md", MODEL.replace("alpha", "omega")).expect("write test fixture");
    release.send(()).expect("send test synchronization event");
    let result = worker
        .join()
        .expect("worker thread should complete without panicking");
    assert!(
        result.is_err()
            || content(&result.expect("concurrent builds: expected success")).contains("omega"),
        "superseded build returned stale success"
    );
});

case!(
    completed_candidate_rechecks_dependencies_before_publication,
    {
        let _workspace = Workspace::new();
        std::fs::write("Model.md", ONTOLOGY).expect("write test fixture");
        std::fs::write("external.ttl", EXTERNAL).expect("write test fixture");
        let (reached, release) = pause("built");
        let worker = candidate();
        reached
            .recv_timeout(DEADLINE)
            .expect("receive test synchronization event before deadline");
        std::fs::write("external.ttl", EXTERNAL.replace("alpha", "omega"))
            .expect("write test fixture");
        release.send(()).expect("send test synchronization event");
        let result = worker
            .join()
            .expect("worker thread should complete without panicking");
        assert!(
            result.is_err()
                || labels(&result.expect("concurrent builds: expected success"))
                    == ["\"Label omega\""],
            "superseded semantic store returned stale success"
        );
    }
);

case!(
    completed_candidate_rechecks_exclusions_before_publication,
    {
        let _workspace = Workspace::new();
        let (reached, release) = pause("built");
        let worker = candidate();
        reached
            .recv_timeout(DEADLINE)
            .expect("receive test synchronization event before deadline");
        std::fs::write(".reqvireignore", "Model.md\n").expect("write test fixture");
        release.send(()).expect("send test synchronization event");
        let result = worker
            .join()
            .expect("worker thread should complete without panicking");
        assert!(
            result.is_err()
                || result
                    .expect("concurrent builds: expected success")
                    .graph_registry
                    .nodes
                    .is_empty(),
            "superseded exclusion policy returned stale success"
        );
    }
);

case!(invalidated_old_build_cannot_replace_new_publication, {
    let _workspace = Workspace::new();
    let (reached, release) = pause("built");
    let worker = candidate();
    reached
        .recv_timeout(DEADLINE)
        .expect("receive test synchronization event before deadline");
    let matcher = patterns(&[]);
    crate::tool_interface::ReqvireToolRegistry::new(true, &matcher).call_tool("reqvire.add_element", &serde_json::json!({
        "file": "Model.md", "content": MODEL.replace("alpha", "omega").split_once("# Elements\n\n").expect("expected fixture to contain an Elements header").1,
        "override_existing": true, "dry_run": false,
    })).expect("expected fixture to contain an Elements header");
    assert!(content(&load().expect("load valid test model")).contains("omega"));
    release.send(()).expect("send test synchronization event");
    let old_result = worker
        .join()
        .expect("worker thread should complete without panicking");
    let before = builds();
    let current = load().expect("load valid test model");
    assert!(content(&current).contains("omega"));
    let retained_new_entry = builds() == before;
    let no_stale_result = old_result.is_err()
        || content(&old_result.expect("concurrent builds: expected success")).contains("omega");
    assert!(
        retained_new_entry && no_stale_result,
        "new cache entry retained: {retained_new_entry}; stale result rejected: {no_stale_result}"
    );
});

case!(continuous_source_changes_end_in_bounded_error, {
    let _workspace = Workspace::new();
    let attempts = Arc::new(AtomicUsize::new(0));
    let observed = Arc::clone(&attempts);
    *OBSERVER.lock().expect("test mutex should not be poisoned") = Some(Arc::new(move |phase| {
        if phase == "built" {
            let attempt = observed.fetch_add(1, Ordering::SeqCst);
            // The child-process deadline bounds this case without prescribing
            // a particular production retry count.
            std::fs::write(
                "Model.md",
                MODEL.replace("alpha", if attempt & 1 == 0 { "omega" } else { "alpha" }),
            )
            .expect("write test fixture");
        }
    }));
    let result = load();
    *OBSERVER.lock().expect("test mutex should not be poisoned") = None;
    assert!(
        load().is_ok(),
        "stopping external changes must permit recovery"
    );
    assert!(attempts.load(Ordering::SeqCst) > 0);
    assert!(
        result.is_err(),
        "continuous changes returned a superseded model as current"
    );
});

case!(
    captured_graph_pages_and_sparql_stay_consistent_after_write,
    {
        let _workspace = Workspace::new();
        std::fs::write("Ontology.md", ONTOLOGY).expect("write test fixture");
        std::fs::write("external.ttl", EXTERNAL).expect("write test fixture");
        std::fs::write("Page.md", PAGE).expect("write test fixture");
        let captured = load().expect("load valid test model");
        std::fs::write("external.ttl", EXTERNAL.replace("alpha", "omega"))
            .expect("write test fixture");
        std::fs::write("Page.md", PAGE.replace("alpha", "omega")).expect("write test fixture");
        let matcher = patterns(&[]);
        crate::tool_interface::ReqvireToolRegistry::new(true, &matcher).call_tool("reqvire.add_element", &serde_json::json!({
        "file": "Model.md", "content": MODEL.replace("alpha", "omega").split_once("# Elements\n\n").expect("expected fixture to contain an Elements header").1,
        "override_existing": true, "dry_run": false,
    })).expect("expected fixture to contain an Elements header");
        let current = load().expect("load valid test model");
        assert!(content(&captured).contains("alpha"));
        assert!(captured.graph_registry.pages["Page.md"]
            .frontmatter_content
            .contains("alpha"));
        assert_eq!(labels(&captured), ["\"Label alpha\""]);
        assert!(content(&current).contains("omega"));
        assert!(current.graph_registry.pages["Page.md"]
            .frontmatter_content
            .contains("omega"));
        assert_eq!(labels(&current), ["\"Label omega\""]);
        let captured_assets = crate::explorer_runtime::build_runtime_assets(&captured)
            .expect("build Explorer assets from test model");
        let current_assets = crate::explorer_runtime::build_runtime_assets(&current)
            .expect("build Explorer assets from test model");
        assert!(captured_assets.project_store_json.contains("Label alpha"));
        assert!(!captured_assets.project_store_json.contains("Label omega"));
        assert!(current_assets.project_store_json.contains("Label omega"));
    }
);

case!(
    evidence_worktree_eligibility_changes_revalidate_and_recover,
    {
        let _workspace = Workspace::new();
        let backup = tempfile::tempdir().expect("create temporary test workspace");
        // A workspace containing two independent eligible worktrees, not itself Git.
        std::fs::rename(".git", backup.path().join("workspace-git")).expect("rename test fixture");
        for directory in ["model", "evidence"] {
            std::fs::create_dir(directory).expect("create test directory");
            git(&["-C", directory, "init", "-q"]);
        }
        std::fs::write(
            "model/Model.md",
            EVIDENCE.replace("(evidence.txt)", "(../evidence/evidence.txt)"),
        )
        .expect("write test fixture");
        std::fs::write("evidence/evidence.txt", "Existing evidence.\n")
            .expect("write test fixture");
        let original = load().expect("load valid test model");
        assert!(original
            .graph_registry
            .get_element_by_name("Evidence Requirement")
            .is_some());
        std::fs::rename("evidence/.git", backup.path().join("evidence-git"))
            .expect("rename test fixture");
        let mut authoritative = ModelManager::new();
        assert!(authoritative
            .parse_and_validate_with_options(None, &patterns(&[]), STRICT)
            .is_err());
        assert!(
            load().is_err(),
            "cached evidence lost workspace eligibility without revalidation"
        );
        std::fs::rename(backup.path().join("evidence-git"), "evidence/.git")
            .expect("rename test fixture");
        assert!(load().is_ok(), "restoring eligibility must permit recovery");
    }
);

// Inject EISDIR at the next persistence target only after observing a real,
// completed first write. This uses an existing log event, needs no production
// persistence hook, and also works when the test runner has elevated privileges.
struct PartialWriteFault {
    fired: AtomicBool,
    blocked_file: Mutex<Option<String>>,
}
static PARTIAL_WRITE_FAULT: PartialWriteFault = PartialWriteFault {
    fired: AtomicBool::new(false),
    blocked_file: Mutex::new(None),
};
impl log::Log for PartialWriteFault {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &log::Record<'_>) {
        let message = record.args().to_string();
        if !message.starts_with("Flushed ") || self.fired.swap(true, Ordering::SeqCst) {
            return;
        }
        let (written, blocked) = if message.ends_with("Model.md") {
            ("Model.md", "Reference.md")
        } else {
            ("Reference.md", "Model.md")
        };
        assert!(
            std::fs::read_to_string(written)
                .expect("read generated test file")
                .contains("Changed Subject"),
            "fault must occur after actual changed bytes were persisted"
        );
        std::fs::rename(blocked, format!("{blocked}.backup")).expect("rename test fixture");
        std::fs::create_dir(blocked).expect("create test directory");
        *self
            .blocked_file
            .lock()
            .expect("test mutex should not be poisoned") = Some(blocked.into());
    }
    fn flush(&self) {}
}

case!(
    failure_after_partial_persistence_invalidates_cached_state,
    {
        let _workspace = Workspace::new();
        std::fs::write("Reference.md", REFERENCE).expect("write test fixture");
        load().expect("load valid test model");
        log::set_logger(&PARTIAL_WRITE_FAULT).expect("flush: expected success");
        log::set_max_level(log::LevelFilter::Debug);
        let matcher = patterns(&[]);
        let result = crate::tool_interface::ReqvireToolRegistry::new(true, &matcher).call_tool(
            "reqvire.rename_element",
            &serde_json::json!({
                "element_name": "Cache Subject", "new_name": "Changed Subject", "dry_run": false,
            }),
        );
        assert!(
            PARTIAL_WRITE_FAULT.fired.load(Ordering::SeqCst),
            "no completed first write was observed"
        );
        assert!(
            matches!(&result, Err(ReqvireError::IoError(_))),
            "persistence error must remain visible: {result:?}"
        );
        let invalidated = MODEL_CACHE
            .lock()
            .expect("test mutex should not be poisoned")
            .entry
            .is_none();
        let blocked = PARTIAL_WRITE_FAULT
            .blocked_file
            .lock()
            .expect("test mutex should not be poisoned")
            .take()
            .expect("fault injection should record the blocked file");
        std::fs::remove_dir(&blocked).expect("remove test directory");
        std::fs::rename(format!("{blocked}.backup"), &blocked).expect("rename test fixture");
        let mut authoritative = ModelManager::new();
        let expected_error = authoritative
            .parse_and_validate_with_options(None, &matcher, STRICT)
            .is_err();
        assert_eq!(
            load().is_err(),
            expected_error,
            "partial inputs must follow authoritative validation"
        );
        std::fs::write("Model.md", MODEL).expect("write test fixture");
        std::fs::write("Reference.md", REFERENCE).expect("write test fixture");
        assert!(
            load().is_ok(),
            "repair must recover after a partial persistence failure"
        );
        assert!(
            invalidated,
            "failed mutation persisted bytes but retained its old cache entry"
        );
    }
);
