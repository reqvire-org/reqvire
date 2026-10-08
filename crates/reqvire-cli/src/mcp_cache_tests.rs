//! Production worker publication, admission, and read-only dispatch regressions.
//! HTTP behavior is covered by test-cache-integration/check_correctness.py.
use super::*;
use std::process::Command;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

const MODEL: &str = include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt");
const OTHER: &str = include_str!("../../../tests/test-cache-integration/fixtures/other.md.txt");

fn isolated(name: &str, test: impl Future<Output = ()>) {
    if std::env::var("REQVIRE_MCP_CACHE_TEST_CHILD").as_deref() == Ok(name) {
        let directory = tempfile::tempdir().expect("create temporary test workspace");
        std::env::set_current_dir(directory.path()).expect("change test working directory");
        assert!(Command::new("git")
            .args(["init", "-q"])
            .status()
            .expect("run test subprocess")
            .success());
        std::fs::write("Model.md", MODEL).expect("write test fixture");
        for args in [
            vec!["config", "user.email", "mcp-test@example.invalid"],
            vec!["config", "user.name", "MCP Test"],
            vec!["add", "Model.md"],
            vec!["commit", "-qm", "baseline"],
        ] {
            assert!(Command::new("git")
                .args(args)
                .status()
                .expect("fixture git")
                .success());
        }
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("build test configuration")
            .block_on(test);
        return;
    }
    let mut child = Command::new(std::env::current_exe().expect("locate test executable"))
        .args([
            "--exact",
            &format!("mcp::cache_correctness_tests::{name}"),
            "--nocapture",
        ])
        .env("REQVIRE_MCP_CACHE_TEST_CHILD", name)
        .env("RUST_LOG", "error,reqvire::explorer_runtime=debug")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn isolated test process");
    let deadline = Instant::now() + Duration::from_secs(120);
    while child.try_wait().expect("check child test status").is_none() {
        if Instant::now() >= deadline {
            child.kill().expect("terminate timed-out child test");
            panic!("MCP regression exceeded deadline");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().expect("collect child test output");
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if name.starts_with("worker_publication_") {
        assert_runtime_build_counts(&String::from_utf8_lossy(&output.stderr));
    }
}

fn assert_runtime_build_counts(log: &str) {
    let phases = [
        ("startup", Some(1)),
        ("rejected", Some(0)),
        ("accepted", Some(1)),
        ("reads", Some(0)),
        ("failed_publication", Some(0)),
        ("recovery", Some(1)),
        ("prepare_noop", None),
        ("noop", Some(0)),
        ("complete", Some(0)),
    ];
    let mut counts = vec![0; phases.len()];
    let mut seen = 0;
    for line in log.lines() {
        if let Some(phase) = line.strip_prefix("@@runtime ") {
            assert_eq!(phase, phases[seen].0, "unexpected phase order: {log}");
            seen += 1;
        } else if line.contains("Building Explorer runtime assets from validated model") {
            assert!(seen > 0, "runtime build outside a measured phase: {log}");
            counts[seen - 1] += 1;
        }
    }
    assert_eq!(seen, phases.len(), "missing measured phases: {log}");
    for ((phase, expected), count) in phases.into_iter().zip(counts) {
        if let Some(expected) = expected {
            assert_eq!(count, expected, "runtime builds in {phase}: {log}");
        }
    }
}

macro_rules! case {
    ($name:ident, $body:block) => {
        #[test]
        fn $name() { isolated(stringify!($name), async $body); }
    };
}

fn worker_executable() -> std::path::PathBuf {
    std::env::var_os("REQVIRE_TEST_BIN")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .expect("test executable")
                .parent()
                .expect("deps directory")
                .parent()
                .expect("target directory")
                .join(format!("reqvire{}", std::env::consts::EXE_SUFFIX))
        })
}
fn worker_server(commits: bool) -> Result<ReqvireMcpServer, ReqvireError> {
    let worktrees = crate::mcp_worktrees::Worktrees::start(
        &std::env::current_dir()?,
        &worker_executable(),
        commits,
        false,
        true,
        false,
        "origin",
    )?;
    let exclusions = reqvire::exclusions::ExclusionSetBuilder::new().build()?;
    Ok(server_with_worktrees(Some(worktrees), false, &exclusions))
}
fn context(server: &ReqvireMcpServer) -> Arc<crate::mcp_worktrees::Context> {
    server
        .worktrees
        .as_ref()
        .expect("worker-backed server")
        .published_browser_context(None)
        .expect("original context")
}
fn add(content: &str, dry_run: bool) -> Value {
    json!({"name": "reqvire.add_element", "arguments": {
        "file": "Model.md", "content": content.split_once("# Elements\n\n").expect("expected fixture to contain an Elements header").1, "dry_run": dry_run,
    }})
}

async fn worker_publication_contract(commits: bool) {
    eprintln!("@@runtime startup");
    let server = worker_server(commits).expect("production worker server");
    let context = context(&server);
    let before = context.metadata();
    let runtime = context.runtime().expect("initial runtime");
    let index = std::fs::read(".git/index").expect("initial index");
    eprintln!("@@runtime rejected");
    context
        .publish_test_response(json!({"runtime_error":"previous runtime diagnostic"}))
        .expect("publish diagnostic");
    let rejected = server
        .call_handler("tools/call", add(MODEL, false))
        .await
        .expect("core rejection envelope");
    assert_eq!(rejected["isError"], true, "{rejected}");
    assert!(!rejected.to_string().contains("mutation succeeded"));
    let invalid = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.add_element","arguments":{}}),
        )
        .await
        .expect_err("schema rejection");
    assert_eq!(invalid.code, ErrorCode::INVALID_PARAMS);
    let preview = server
        .call_handler("tools/call", add(OTHER, true))
        .await
        .expect("preview");
    assert_ne!(preview["isError"], true, "{preview}");
    assert_eq!(
        context.metadata()["explorer_diagnostic"],
        "previous runtime diagnostic"
    );
    assert_eq!(context.metadata()["head"], before["head"]);
    assert_eq!(
        context.metadata()["model_revision"],
        before["model_revision"]
    );
    assert_eq!(std::fs::read(".git/index").expect("preview index"), index);
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("preview source"),
        MODEL
    );
    eprintln!("@@runtime accepted");
    let accepted = server
        .call_handler("tools/call", add(OTHER, false))
        .await
        .expect("accepted mutation");
    assert_ne!(accepted["isError"], true, "{accepted}");
    assert_eq!(accepted["structuredContent"]["commit"].is_string(), commits);
    let published = context.runtime().expect("new worker runtime");
    assert!(!Arc::ptr_eq(&runtime, &published));
    assert!(published
        .assets
        .project_store_json
        .contains("Other Subject"));
    assert_eq!(context.metadata()["explorer_diagnostic"], Value::Null);
    eprintln!("@@runtime reads");
    let read = server.call_handler("tools/call",json!({"name":"reqvire.semantic.sparql","arguments":{"query":"ASK { ?s <https://www.reqvire.org/ontology#elementName> \"Other Subject\" }"}})).await.expect("new semantic snapshot");
    assert_eq!(read["structuredContent"]["boolean"], true);
    assert!(Arc::ptr_eq(
        &published,
        &context.runtime().expect("read retains runtime")
    ));
    eprintln!("@@runtime failed_publication");
    // An invalid parent publication must not replace accepted model inputs or
    // turn the successful mutation into the legacy hook's protocol exception.
    let accepted_status = context.metadata();
    context
        .publish_test_response(json!({"runtime":{"project_store":[],"ontologies_ttl":"invalid"}}))
        .expect("reject malformed runtime");
    assert!(context.runtime().is_err());
    assert_eq!(context.metadata()["head"], accepted_status["head"]);
    assert_eq!(
        context.metadata()["pending_changes"],
        accepted_status["pending_changes"]
    );
    let app = mount_server(
        axum::Router::new(),
        server.clone(),
        &HttpAccess::new(&[], &[]),
    );
    let resource = http_rpc(
        app.clone(),
        1,
        "resources/read",
        json!({"uri":"reqvire://workspace/status"}),
    )
    .await;
    assert_eq!(
        resource["result"]["_meta"]["reqvire/context"]["model_revision"],
        accepted_status["model_revision"]
    );
    assert!(!resource["result"]["_meta"]["reqvire/context"]["explorer_diagnostic"].is_null());
    let search = http_rpc(
        app,
        2,
        "tools/call",
        json!({"name":"reqvire.search","arguments":{}}),
    )
    .await;
    assert_ne!(search["result"]["isError"], true, "{search}");
    assert!(search.to_string().contains("Other Subject"));
    assert_eq!(
        search["result"]["_meta"]["reqvire/context"]["model_revision"],
        accepted_status["model_revision"]
    );
    assert!(!search["result"]["_meta"]["reqvire/context"]["explorer_diagnostic"].is_null());
    assert_eq!(
        context.metadata()["model_revision"],
        accepted_status["model_revision"]
    );
    eprintln!("@@runtime recovery");
    context
        .test_refresh_runtime()
        .expect("refresh accepted worker inputs");
    assert_eq!(
        context.runtime().expect("recovered runtime").live.revision,
        published.live.revision
    );
    assert_eq!(context.metadata()["explorer_diagnostic"], Value::Null);
    eprintln!("@@runtime prepare_noop");
    let format = json!({"name":"reqvire.format","arguments":{"fix":true}});
    let prepared = server
        .call_handler("tools/call", format.clone())
        .await
        .expect("normalize fixture");
    assert_ne!(prepared["isError"], true, "{prepared}");
    eprintln!("@@runtime noop");
    let formatted = context.runtime().expect("formatted runtime");
    let formatted_status = context.metadata();
    let formatted_index = std::fs::read(".git/index").expect("formatted index");
    let noop = server
        .call_handler("tools/call", format.clone())
        .await
        .expect("repeat format");
    assert_ne!(noop["isError"], true, "{noop}");
    assert!(noop["structuredContent"].get("commit").is_none(), "{noop}");
    assert!(Arc::ptr_eq(
        &formatted,
        &context.runtime().expect("no-op runtime")
    ));
    assert_eq!(context.metadata()["head"], formatted_status["head"]);
    assert_eq!(
        context.metadata()["model_revision"],
        formatted_status["model_revision"]
    );
    assert_eq!(
        std::fs::read(".git/index").expect("no-op index"),
        formatted_index
    );
    context
        .publish_test_response(json!({"runtime_error":"no-op must retain this diagnostic"}))
        .expect("publish no-op diagnostic");
    let noop_with_diagnostic = server
        .call_handler("tools/call", format)
        .await
        .expect("no-op with diagnostic");
    assert_ne!(
        noop_with_diagnostic["isError"], true,
        "{noop_with_diagnostic}"
    );
    assert!(
        noop_with_diagnostic["structuredContent"]
            .get("commit")
            .is_none(),
        "{noop_with_diagnostic}"
    );
    assert_eq!(
        context.metadata()["explorer_diagnostic"],
        "no-op must retain this diagnostic"
    );
    assert!(context.runtime().is_err());
    assert_eq!(context.metadata()["head"], formatted_status["head"]);
    assert_eq!(
        context.metadata()["model_revision"],
        formatted_status["model_revision"]
    );
    assert_eq!(
        std::fs::read(".git/index").expect("diagnostic no-op index"),
        formatted_index
    );
    eprintln!("@@runtime complete");
}
case!(
    worker_publication_without_commits_preserves_rejections_and_diagnostics,
    {
        worker_publication_contract(false).await;
    }
);
case!(
    worker_publication_with_commits_preserves_rejections_and_diagnostics,
    {
        worker_publication_contract(true).await;
    }
);

case!(read_only_aggregate_workspace_keeps_model_reads_available, {
    std::fs::create_dir("nested").expect("aggregate child directory");
    std::fs::rename(".git", "nested/.git").expect("move child repository");
    std::fs::rename("Model.md", "nested/Model.md").expect("move child model");
    let index = std::fs::read("nested/.git/index").expect("child index");
    let server = read_only_server();
    let search = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
        )
        .await
        .expect("aggregate search");
    assert_ne!(search["isError"], true, "{search}");
    assert!(search.to_string().contains("Cache Subject"), "{search}");
    assert!(search.to_string().contains("nested/Model.md"), "{search}");
    let catalog = server
        .call_handler("tools/list", json!({}))
        .await
        .expect("read-only catalog");
    assert!(
        !catalog.to_string().contains("reqvire.add_element"),
        "{catalog}"
    );
    let mutation = server
        .call_handler("tools/call", add(OTHER, false))
        .await
        .expect_err("read-only mutation rejection");
    assert_eq!(mutation.code, ErrorCode::INVALID_PARAMS);
    assert_eq!(
        std::fs::read("nested/.git/index").expect("unchanged child index"),
        index
    );
    assert_eq!(
        std::fs::read_to_string("nested/Model.md").expect("unchanged child source"),
        MODEL
    );
});

case!(read_during_write_gate_cannot_publish_partial_persistence, {
    let server = ReqvireMcpServer::read_only(
        false,
        &reqvire::exclusions::ExclusionSetBuilder::new()
            .build()
            .expect("ownership test assertion"),
        Arc::new(RwLock::new(())),
    );
    let request = json!({"name": "reqvire.search", "arguments": {}});
    server
        .call_handler("tools/call", request.clone())
        .await
        .expect("execute test MCP request");
    let gate = server.write_lock.write().await;
    // Controlled intermediate state of a write holding the same gate used by MCP.
    std::fs::write(
        "Model.md",
        MODEL.replace("Cache Subject", "Partial Subject"),
    )
    .expect("write test fixture");
    let mut read = Box::pin(server.call_handler("tools/call", request.clone()));
    let first_poll = read.as_mut().poll(&mut Context::from_waker(Waker::noop()));
    let pending = first_poll.is_pending();
    let safe = match first_poll {
        Poll::Pending => true,
        Poll::Ready(result) => {
            let result = result.expect("add: expected success");
            !result.to_string().contains("Partial Subject")
                && result.to_string().contains("Cache Subject")
        }
    };
    std::fs::write("Model.md", MODEL.replace("Cache Subject", "Final Subject"))
        .expect("write test fixture");
    reqvire::model_cache::invalidate();
    drop(gate);
    // Complete the queued read before issuing another one: Tokio's fair mutex
    // otherwise reserves the gate for this unpolled future indefinitely.
    if pending {
        assert!(read
            .await
            .expect("add: expected success")
            .to_string()
            .contains("Final Subject"));
    }
    let after = server
        .call_handler("tools/call", request)
        .await
        .expect("execute test MCP request");
    assert!(after.to_string().contains("Final Subject"));
    assert!(
        safe,
        "read published intermediate disk state while workspace write gate was held"
    );
});

case!(mutation_startup_rejects_dirty_worktree_before_listening, {
    std::fs::write("Model.md", format!("{MODEL}\n")).expect("ownership test assertion");
    let error = serve_http(
        McpOptions {
            enable_mutations: true,
            enable_commits: false,
            enable_github: false,
            github_remote: "origin",
            with_size_estimates: false,
        },
        &reqvire::exclusions::ExclusionSetBuilder::new()
            .build()
            .expect("ownership test assertion"),
        "127.0.0.1",
        0,
        &HttpAccess::default(),
    )
    .await
    .expect_err("reject dirty mutation startup");
    assert!(
        error.to_string().contains("clean"),
        "expected dirty-worktree startup rejection, got: {error}"
    );
});

fn git_test(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("run fixture git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("ownership test assertion")
        .trim()
        .to_string()
}
fn attempt_server() -> Result<ReqvireMcpServer, ReqvireError> {
    worker_server(false)
}
case!(mutation_commits_are_disabled_by_default, {
    let server = worker_server(false).expect("production worker server");
    let context = context(&server);
    let mut runtime = context.runtime().expect("initial runtime");
    let head = git_test(&["rev-parse", "HEAD"]);
    std::fs::write("unrelated.txt", "staged human work").expect("write unrelated file");
    git_test(&["add", "unrelated.txt"]);
    let index = std::fs::read(".git/index").expect("index bytes");
    // Commit-disabled writes do not need or acquire the Git index lock.
    std::fs::write(".git/index.lock", "external lock").expect("lock index");
    std::fs::write("Model.md", "external invalid edit").expect("external model edit");
    for content in [
        OTHER.to_string(),
        OTHER.replace("Other Subject", "Third Subject"),
    ]
    .iter()
    {
        let result = server
            .call_handler("tools/call", add(content, false))
            .await
            .expect("execute mutation");
        assert!(!result["isError"].as_bool().unwrap_or(false), "{result}");
        assert_eq!(
            git_test(&["rev-parse", "HEAD"]),
            head,
            "default mutation must not create a commit"
        );
        assert_eq!(
            std::fs::read(".git/index").expect("index bytes"),
            index,
            "default mutation must not stage changes"
        );
        assert!(
            result["structuredContent"].get("commit").is_none(),
            "{result}"
        );
        let next = context.runtime().expect("published runtime");
        assert!(
            !Arc::ptr_eq(&runtime, &next),
            "successful persistence publishes runtime"
        );
        runtime = next;
    }
    let persisted = std::fs::read_to_string("Model.md").expect("persisted model");
    assert!(
        persisted.contains("Cache Subject")
            && persisted.contains("Other Subject")
            && persisted.contains("Third Subject")
    );
    assert!(!persisted.contains("external invalid edit"));
    let read = server
        .call_handler("tools/call", json!({"name":"reqvire.semantic.sparql","arguments":{
            "query":"ASK { ?s <https://www.reqvire.org/ontology#elementName> \"Third Subject\" }"
        }}))
        .await
        .expect("read accepted semantic snapshot");
    assert_eq!(read["structuredContent"]["boolean"], true);
    assert!(Arc::ptr_eq(
        &runtime,
        &context.runtime().expect("unchanged read runtime")
    ));
    std::fs::remove_file(".git/index.lock").expect("unlock index");
    drop(context);
    drop(server);
    assert!(attempt_server()
        .err()
        .expect("reject dirty restart")
        .to_string()
        .contains("clean"));
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("model unchanged on restart"),
        persisted
    );
    git_test(&["add", "Model.md"]);
    git_test(&["commit", "-qm", "user commits saved changes"]);
    let committed = git_test(&["rev-parse", "HEAD"]);
    let _resumed = attempt_server().expect("resume after user commit");
    assert_eq!(git_test(&["rev-parse", "HEAD"]), committed);
});
case!(ownership_rejects_second_writer_and_releases_on_stop, {
    let head = git_test(&["rev-parse", "HEAD"]);
    let first = attempt_server().expect("ownership test assertion");
    assert!(attempt_server()
        .err()
        .expect("ownership test assertion")
        .to_string()
        .contains("owned"));
    drop(first);
    let _resumed = attempt_server().expect("ownership test assertion");
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
});
case!(ownership_requires_clean_named_committed_branch, {
    std::fs::write("untracked.txt", "outside").expect("ownership test assertion");
    assert!(attempt_server()
        .err()
        .expect("ownership test assertion")
        .to_string()
        .contains("clean"));
    git_test(&["add", "untracked.txt"]);
    assert!(attempt_server()
        .err()
        .expect("ownership test assertion")
        .to_string()
        .contains("clean"));
    git_test(&["reset", "-q", "HEAD", "--", "untracked.txt"]);
    std::fs::remove_file("untracked.txt").expect("ownership test assertion");
    git_test(&["checkout", "--detach", "-q"]);
    assert!(attempt_server()
        .err()
        .expect("ownership test assertion")
        .to_string()
        .contains("named branch"));
    git_test(&["switch", "--orphan", "unborn"]);
    assert!(attempt_server()
        .err()
        .expect("reject branch without a commit")
        .to_string()
        .contains("existing commit"));
});
case!(
    owned_snapshot_ignores_external_edits_and_commits_only_prepared_files,
    {
        let server = worker_server(true).expect("production worker server");
        let before = git_test(&["rev-list", "--count", "HEAD"])
            .parse::<usize>()
            .expect("ownership test assertion");
        std::fs::write("Model.md", "external invalid edit").expect("ownership test assertion");
        std::fs::write("unrelated.txt", "staged human work").expect("ownership test assertion");
        git_test(&["add", "unrelated.txt"]);
        let read = server
            .call_handler(
                "tools/call",
                json!({"name":"reqvire.search","arguments":{}}),
            )
            .await
            .expect("ownership test assertion");
        assert!(read.to_string().contains("Cache Subject"));
        let result = server
            .call_handler("tools/call", add(OTHER, false))
            .await
            .expect("ownership test assertion");
        assert!(!result["isError"].as_bool().unwrap_or(false), "{result}");
        assert_eq!(
            result["structuredContent"]["commit"],
            git_test(&["rev-parse", "HEAD"])
        );
        assert_eq!(
            git_test(&["rev-list", "--count", "HEAD"])
                .parse::<usize>()
                .expect("ownership test assertion"),
            before + 1
        );
        assert_eq!(
            git_test(&["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"]),
            "Model.md"
        );
        assert_eq!(
            git_test(&["diff", "--cached", "--name-only"]),
            "unrelated.txt"
        );
        assert!(!git_test(&["show", "HEAD:Model.md"]).contains("external invalid edit"));
        assert!(git_test(&["show", "HEAD:Model.md"]).contains("Other Subject"));
    }
);
async fn check_preview_rejection_and_noop(enable_commits: bool) {
    let server = worker_server(enable_commits).expect("production worker server");
    let context = context(&server);
    let runtime = context.runtime().expect("initial runtime");
    let before = git_test(&["rev-parse", "HEAD"]);
    let index = std::fs::read(".git/index").expect("read index");
    let preview = server
        .call_handler("tools/call", add(OTHER, true))
        .await
        .expect("ownership test assertion");
    assert!(!preview["isError"].as_bool().unwrap_or(false));
    let rejected = server
        .call_handler("tools/call", add(MODEL, false))
        .await
        .expect("ownership test assertion");
    assert_eq!(rejected["isError"], true);
    assert!(preview["structuredContent"].get("commit").is_none());
    assert!(rejected["structuredContent"].get("commit").is_none());
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("unchanged model"),
        MODEL
    );
    assert_eq!(std::fs::read(".git/index").expect("unchanged index"), index);
    assert!(Arc::ptr_eq(
        &runtime,
        &context.runtime().expect("unchanged runtime")
    ));
    assert_eq!(git_test(&["rev-parse", "HEAD"]), before);
    let request = json!({"name":"reqvire.format","arguments":{"fix":true}});
    let first = server
        .call_handler("tools/call", request.clone())
        .await
        .expect("ownership test assertion");
    assert!(!first["isError"].as_bool().unwrap_or(false), "{first}");
    let formatted = git_test(&["rev-parse", "HEAD"]);
    let formatted_runtime = context.runtime().expect("formatted runtime");
    let formatted_index = std::fs::read(".git/index").expect("formatted index");
    let formatted_model = std::fs::read("Model.md").expect("formatted model");
    let second = server
        .call_handler("tools/call", request)
        .await
        .expect("ownership test assertion");
    assert!(!second["isError"].as_bool().unwrap_or(false), "{second}");
    assert_eq!(git_test(&["rev-parse", "HEAD"]), formatted);
    assert!(Arc::ptr_eq(
        &formatted_runtime,
        &context.runtime().expect("no-op runtime")
    ));
    assert!(second["structuredContent"].get("commit").is_none());
    assert_eq!(
        std::fs::read(".git/index").expect("unchanged index"),
        formatted_index
    );
    assert_eq!(
        std::fs::read("Model.md").expect("unchanged model"),
        formatted_model
    );
}
case!(owned_preview_rejection_and_noop_never_commit, {
    check_preview_rejection_and_noop(true).await;
});
case!(default_preview_rejection_and_noop_do_not_publish, {
    check_preview_rejection_and_noop(false).await;
});
case!(external_head_change_stops_mutations, {
    let server = worker_server(true).expect("production worker server");
    git_test(&["commit", "--allow-empty", "-qm", "external commit"]);
    let result = server
        .call_handler("tools/call", add(OTHER, false))
        .await
        .expect("ownership test assertion");
    assert_eq!(result["isError"], true, "{result}");
    assert!(result.to_string().contains("HEAD changed"));
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("ownership test assertion"),
        MODEL
    );
});
case!(git_index_failure_keeps_files_head_and_snapshot, {
    let server = worker_server(true).expect("production worker server");
    let context = context(&server);
    let runtime = context.runtime().expect("initial runtime");
    let head = git_test(&["rev-parse", "HEAD"]);
    std::fs::write(".git/index.lock", "another git operation").expect("ownership test assertion");
    let result = server
        .call_handler("tools/call", add(OTHER, false))
        .await
        .expect("ownership test assertion");
    assert_eq!(result["isError"], true, "{result}");
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("ownership test assertion"),
        MODEL
    );
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert!(Arc::ptr_eq(
        &runtime,
        &context.runtime().expect("unchanged runtime")
    ));
    std::fs::remove_file(".git/index.lock").expect("ownership test assertion");
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
        )
        .await
        .expect("ownership test assertion");
    assert!(!read.to_string().contains("Other Subject"));
});
#[cfg(unix)]
case!(git_publication_failure_rolls_back_persisted_files, {
    use std::os::unix::fs::PermissionsExt;
    let server = worker_server(true).expect("production worker server");
    let head = git_test(&["rev-parse", "HEAD"]);
    let hook = ".git/hooks/reference-transaction";
    std::fs::write(hook, "#!/bin/sh\n[ \"$1\" != prepared ]\n").expect("ownership test assertion");
    std::fs::set_permissions(hook, std::fs::Permissions::from_mode(0o755))
        .expect("ownership test assertion");
    let result = server
        .call_handler("tools/call", add(OTHER, false))
        .await
        .expect("ownership test assertion");
    assert_eq!(result["isError"], true, "{result}");
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("ownership test assertion"),
        MODEL
    );
    assert_eq!(git_test(&["status", "--porcelain"]), "");
    std::fs::remove_file(hook).expect("ownership test assertion");
    let result = server
        .call_handler("tools/call", add(OTHER, false))
        .await
        .expect("ownership test assertion");
    assert!(
        result["structuredContent"]["commit"].is_string(),
        "{result}"
    );
});
case!(owned_folder_move_commits_files_and_rebuilds_reads, {
    let server = worker_server(true).expect("production worker server");
    let moved = server.call_handler("tools/call", json!({"name":"reqvire.move_file","arguments":{"source_file":"Model.md","target_file":"nested/Moved.md"}})).await.expect("ownership test assertion");
    assert!(moved["structuredContent"]["commit"].is_string(), "{moved}");
    assert!(!std::path::Path::new("Model.md").exists());
    let moved = server.call_handler("tools/call", json!({"name":"reqvire.move_folder","arguments":{"source_folder":"nested","target_folder":"renamed"}})).await.expect("ownership test assertion");
    assert!(moved["structuredContent"]["commit"].is_string(), "{moved}");
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
        )
        .await
        .expect("ownership test assertion");
    assert!(read.to_string().contains("renamed/Moved.md"), "{read}");
    assert_eq!(git_test(&["status", "--porcelain"]), "");
});

case!(invalid_startup_releases_ownership_for_repair, {
    std::fs::write(
        "Model.md",
        include_str!("../../../tests/test-cache-integration/fixtures/broken.md.txt"),
    )
    .expect("ownership test assertion");
    git_test(&["add", "Model.md"]);
    git_test(&["commit", "-qm", "invalid model"]);
    assert!(attempt_server().is_err());
    std::fs::write("Model.md", MODEL).expect("ownership test assertion");
    git_test(&["add", "Model.md"]);
    git_test(&["commit", "-qm", "repair"]);
    let _server = attempt_server().expect("ownership test assertion");
});
case!(
    branch_switch_stops_owner_and_cannot_acquire_same_worktree,
    {
        let server = worker_server(true).expect("production worker server");
        git_test(&["switch", "-qc", "external-branch"]);
        assert!(attempt_server()
            .err()
            .expect("ownership test assertion")
            .to_string()
            .contains("owned"));
        let result = server
            .call_handler("tools/call", add(OTHER, false))
            .await
            .expect("ownership test assertion");
        assert_eq!(result["isError"], true, "{result}");
        assert_eq!(
            std::fs::read_to_string("Model.md").expect("ownership test assertion"),
            MODEL
        );
    }
);

case!(
    asset_candidate_validation_precedes_persistence_and_preview,
    {
        std::fs::write(
            "Ontology.md",
            include_str!("../../../tests/test-cache-integration/fixtures/ontology.md.txt"),
        )
        .expect("ownership test assertion");
        let external = include_str!("../../../tests/test-cache-integration/fixtures/external.ttl");
        std::fs::write("external.ttl", external).expect("ownership test assertion");
        git_test(&["add", "."]);
        git_test(&["commit", "-qm", "ontology fixture"]);
        let server = worker_server(true).expect("production worker server");
        let context = context(&server);
        let runtime = context.runtime().expect("initial runtime");
        let head = git_test(&["rev-parse", "HEAD"]);
        for dry_run in [true, false] {
            let result = server.call_handler("tools/call", json!({"name":"reqvire.remove_asset","arguments":{"file_path":"external.ttl","dry_run":dry_run}})).await.expect("ownership test assertion");
            assert_eq!(result["isError"], true, "{result}");
            assert_eq!(
                std::fs::read_to_string("external.ttl").expect("ownership test assertion"),
                external
            );
            assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
        }
        assert!(Arc::ptr_eq(
            &runtime,
            &context.runtime().expect("unchanged runtime")
        ));
    }
);
#[cfg(unix)]
case!(asset_move_preserves_executable_mode_and_updates_snapshot, {
    use std::os::unix::fs::PermissionsExt;
    std::fs::write(
        "Evidence.md",
        include_str!("../../../tests/test-cache-integration/fixtures/evidence.md.txt"),
    )
    .expect("ownership test assertion");
    std::fs::write("evidence.txt", "#!/bin/sh\nexit 0\n").expect("ownership test assertion");
    std::fs::set_permissions("evidence.txt", std::fs::Permissions::from_mode(0o755))
        .expect("ownership test assertion");
    git_test(&["add", "."]);
    git_test(&["commit", "-qm", "evidence fixture"]);
    let server = worker_server(true).expect("production worker server");
    let result = server.call_handler("tools/call", json!({"name":"reqvire.move_asset","arguments":{"old_path":"evidence.txt","new_path":"moved.sh"}})).await.expect("ownership test assertion");
    assert!(
        result["structuredContent"]["commit"].is_string(),
        "{result}"
    );
    assert!(git_test(&["ls-tree", "HEAD", "--", "moved.sh"]).starts_with("100755"));
    assert_eq!(git_test(&["status", "--porcelain"]), "");
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.read_element","arguments":{"name":"Evidence Requirement"}}),
        )
        .await
        .expect("ownership test assertion");
    assert!(read.to_string().contains("moved.sh"), "{read}");
});

case!(new_external_destination_is_not_overwritten, {
    let server = worker_server(true).expect("production worker server");
    std::fs::write("Taken.md", "human work after startup").expect("write external file");
    let head = git_test(&["rev-parse", "HEAD"]);
    let result = server.call_handler("tools/call", json!({"name":"reqvire.move_file","arguments":{"source_file":"Model.md","target_file":"Taken.md"}})).await.expect("tool response");
    assert_eq!(result["isError"], true, "{result}");
    assert_eq!(
        std::fs::read_to_string("Taken.md").expect("external file"),
        "human work after startup"
    );
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("model file"),
        MODEL
    );
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
});

fn read_only_server() -> ReqvireMcpServer {
    let mut server = ReqvireMcpServer::read_only(
        false,
        &reqvire::exclusions::ExclusionSetBuilder::new()
            .build()
            .expect("test fixture operation should succeed"),
        Arc::new(RwLock::new(())),
    );
    // Deterministic five-client tests also run on single-CPU CI hosts.
    server.read_capacity = Arc::new(Semaphore::new(5));
    server
}

async fn http_rpc(app: axum::Router, id: usize, method: &str, params: Value) -> Value {
    use axum::{
        body::{to_bytes, Body},
        http::Request,
    };
    use tower::ServiceExt;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("host", "127.0.0.1:8081")
                .header("content-type", "application/json")
                .header("accept", "application/json, text/event-stream")
                .header("mcp-protocol-version", "2025-11-25")
                .body(Body::from(
                    json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}).to_string(),
                ))
                .expect("build test HTTP request"),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("test fixture operation should succeed");
    let wire: Value =
        serde_json::from_slice(&bytes).expect("test fixture operation should succeed");
    assert_eq!(wire["id"], id, "response correlation: {wire}");
    wire
}

case!(
    read_only_http_requests_overlap_without_blocking_transport,
    {
        use std::sync::{mpsc, Condvar, Mutex as StdMutex};
        let mut server = read_only_server();
        let barrier = Arc::new((StdMutex::new(false), Condvar::new()));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (ping_tx, ping_rx) = mpsc::channel();
        let started = Arc::new(tokio::sync::Notify::new());
        let hook_barrier = Arc::clone(&barrier);
        let hook_started = Arc::clone(&started);
        server.before_dispatch = Some(Arc::new(move |_, _| {
            entered_tx
                .send(std::thread::current().id())
                .expect("send test worker message");
            hook_started.notify_one();
            let (lock, ready) = &*hook_barrier;
            let (released, timeout) = ready
                .wait_timeout_while(
                    lock.lock().expect("test lock should not be poisoned"),
                    Duration::from_secs(5),
                    |released| !*released,
                )
                .expect("test lock should not be poisoned");
            assert!(
                *released && !timeout.timed_out(),
                "dispatch barrier did not release"
            );
        }));
        // A separate coordinator releases even the old implementation's blocked
        // async thread, allowing the regression to fail rather than hang forever.
        let coordinator = std::thread::spawn(move || {
            let mut threads = std::collections::HashSet::new();
            for _ in 0..5 {
                match entered_rx.recv_timeout(Duration::from_secs(2)) {
                    Ok(thread) => {
                        threads.insert(thread);
                    }
                    Err(_) => break,
                }
            }
            let live = ping_rx.recv_timeout(Duration::from_millis(500)).is_ok();
            let (lock, ready) = &*barrier;
            *lock.lock().expect("test lock should not be poisoned") = true;
            ready.notify_all();
            // Keep the receiver alive until every handler has left its hook.
            (threads.len(), live, entered_rx)
        });
        let app = mount_server(axum::Router::new(), server, &HttpAccess::new(&[], &[]));
        let mut clients = tokio::task::JoinSet::new();
        for id in 1..=5 {
            let app = app.clone();
            clients.spawn(async move {
                http_rpc(
                    app,
                    id,
                    "tools/call",
                    json!({"name":"reqvire.search","arguments":{}}),
                )
                .await
            });
        }
        let ping = tokio::spawn(async move {
            started.notified().await;
            let wire = http_rpc(app, 99, "ping", json!({})).await;
            assert!(wire.get("result").is_some(), "{wire}");
            let _ = ping_tx.send(());
        });
        while let Some(result) = clients.join_next().await {
            let wire = result.expect("test fixture operation should succeed");
            assert_ne!(wire["result"]["isError"], true, "{wire}");
            assert!(wire["result"]["structuredContent"]
                .to_string()
                .contains("Cache Subject"));
        }
        ping.await.expect("test fixture operation should succeed");
        let (overlap, live, _receiver) = coordinator.join().expect("test worker should complete");
        assert_eq!(
            overlap, 5,
            "five reads must enter dispatch before any is released"
        );
        assert!(
            live,
            "a stalled synchronous read blocked HTTP ping on the async runtime"
        );
    }
);

async fn read_wave(app: axum::Router, method: &str, params: Value) -> Vec<Value> {
    let mut clients = tokio::task::JoinSet::new();
    for id in 1..=5 {
        let app = app.clone();
        let params = params.clone();
        let method = method.to_string();
        clients.spawn(async move { http_rpc(app, id, &method, params).await });
    }
    let mut values = Vec::new();
    while let Some(value) = clients.join_next().await {
        values.push(value.expect("test fixture operation should succeed")["result"].clone());
    }
    values
}

case!(read_only_parallel_payloads_and_source_refresh, {
    let server = read_only_server();
    let app = mount_server(axum::Router::new(), server, &HttpAccess::new(&[], &[]));
    let tools = [
        ("reqvire.workspace_status", json!({})),
        ("reqvire.search", json!({})),
        ("reqvire.read_element", json!({"name":"Cache Subject"})),
        ("reqvire.coverage", json!({})),
        ("reqvire.lint", json!({})),
        ("reqvire.traces", json!({})),
        (
            "reqvire.semantic.sparql",
            json!({"query":"ASK { ?s ?p ?o }"}),
        ),
    ];
    let mut timings = Vec::new();
    for (name, arguments) in tools {
        let params = json!({"name":name,"arguments":arguments});
        let oracle =
            http_rpc(app.clone(), 99, "tools/call", params.clone()).await["result"].clone();
        assert_ne!(oracle["isError"], true, "{name}: {oracle}");
        let mut serial = Vec::new();
        for id in 100..105 {
            let start = Instant::now();
            let wire = http_rpc(app.clone(), id, "tools/call", params.clone()).await;
            serial.push(start.elapsed().as_secs_f64());
            assert_eq!(wire["result"], oracle);
        }
        let start = Instant::now();
        let values = read_wave(app.clone(), "tools/call", params).await;
        timings.push(json!({"tool":name,"serial_seconds":serial,"five_client_wall_seconds":start.elapsed().as_secs_f64()}));
        assert!(
            values.iter().all(|v| *v == oracle),
            "{name} concurrent payload mismatch"
        );
    }
    eprintln!("read-only timings: {}", json!(timings));
    let params = json!({"uri":"reqvire://workspace/status"});
    let oracle =
        http_rpc(app.clone(), 99, "resources/read", params.clone()).await["result"].clone();
    assert!(oracle["contents"].is_array(), "{oracle}");
    assert!(read_wave(app.clone(), "resources/read", params)
        .await
        .iter()
        .all(|v| *v == oracle));

    std::fs::write("Model.md", MODEL.replace("alpha", "omega")).expect("write test fixture");
    let params = json!({"name":"reqvire.read_element","arguments":{"name":"Cache Subject"}});
    let fresh = read_wave(app.clone(), "tools/call", params.clone()).await;
    assert!(fresh.iter().all(|v| v["structuredContent"]["content"]
        .as_str()
        .expect("expected a string in the test response")
        .contains("omega")));
    std::fs::write(
        "Model.md",
        format!("{MODEL}\n### Invalid\n\n#### Metadata\n  * type: unsupported-type\n"),
    )
    .expect("write test fixture");
    let failed = read_wave(app.clone(), "tools/call", params.clone()).await;
    assert!(failed.iter().all(|v| v["isError"] == true), "{failed:?}");
    std::fs::write("Model.md", MODEL).expect("write test fixture");
    let recovered = read_wave(app, "tools/call", params).await;
    assert!(recovered.iter().all(|v| v["structuredContent"]["content"]
        .as_str()
        .expect("expected a string in the test response")
        .contains("alpha")));
});

case!(read_only_capacity_cancellation_and_writer_priority, {
    use std::sync::{Condvar, Mutex as StdMutex};
    let mut server = read_only_server();
    server.read_capacity = Arc::new(Semaphore::new(2));
    let barrier = Arc::new((StdMutex::new(false), Condvar::new()));
    let (entered_tx, mut entered_rx) = tokio::sync::mpsc::unbounded_channel();
    let hook_barrier = Arc::clone(&barrier);
    server.before_dispatch = Some(Arc::new(move |_, _| {
        let _ = entered_tx.send(());
        let (lock, ready) = &*hook_barrier;
        let (released, _) = ready
            .wait_timeout_while(
                lock.lock().expect("test lock should not be poisoned"),
                Duration::from_secs(5),
                |released| !*released,
            )
            .expect("test lock should not be poisoned");
        assert!(*released, "test must release dispatch barrier");
    }));
    let request = json!({"name":"reqvire.search","arguments":{}});
    let mut readers = Vec::new();
    for _ in 0..2 {
        let server = server.clone();
        let request = request.clone();
        readers.push(tokio::spawn(async move {
            server.call_handler("tools/call", request).await
        }));
    }
    for _ in 0..2 {
        tokio::time::timeout(Duration::from_secs(2), entered_rx.recv())
            .await
            .expect("receive test worker response")
            .expect("receive test worker response");
    }
    let app = mount_server(
        axum::Router::new(),
        server.clone(),
        &HttpAccess::new(&[], &[]),
    );
    let rejected = http_rpc(app, 3, "tools/call", request.clone()).await;
    assert_eq!(rejected["error"]["code"], -32000, "{rejected}");
    assert_eq!(rejected["error"]["data"]["retryable"], true);
    assert!(entered_rx.try_recv().is_err(), "overload reached dispatch");
    let cancelled = readers
        .pop()
        .expect("test fixture operation should succeed");
    cancelled.abort();
    assert!(cancelled
        .await
        .expect_err("test fixture operation should fail")
        .is_cancelled());
    assert_eq!(
        server.read_capacity.available_permits(),
        0,
        "running cancellation released capacity early"
    );
    let mut writer = Box::pin(Arc::clone(&server.write_lock).write_owned());
    assert!(
        writer
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending(),
        "running cancellation released shared workspace gate early"
    );
    {
        let (lock, ready) = &*barrier;
        *lock.lock().expect("test lock should not be poisoned") = true;
        ready.notify_all();
    }
    assert_ne!(
        readers
            .pop()
            .expect("test fixture operation should succeed")
            .await
            .expect("test fixture operation should succeed")
            .expect("test fixture operation should succeed")["isError"],
        true
    );
    tokio::time::timeout(Duration::from_secs(2), async {
        while server.read_capacity.available_permits() != 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("test fixture operation should succeed");
    let mut late_read = Box::pin(server.call_handler("tools/call", request.clone()));
    assert!(
        late_read
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending(),
        "later reader overtook queued writer"
    );
    assert_eq!(server.read_capacity.available_permits(), 1);
    drop(late_read);
    assert_eq!(
        server.read_capacity.available_permits(),
        2,
        "undispatched cancellation leaked capacity"
    );
    let gate = writer.await;
    assert!(
        entered_rx.try_recv().is_err(),
        "read dispatched through exclusive gate"
    );
    std::fs::write("Model.md", "partially persisted invalid model").expect("write test fixture");
    let mut after = Box::pin(server.call_handler("tools/call", request));
    assert!(after
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
        .is_pending());
    std::fs::write("Model.md", MODEL.replace("Cache Subject", "Final Subject"))
        .expect("write test fixture");
    reqvire::model_cache::invalidate();
    drop(gate);
    let result = after.await.expect("test fixture operation should succeed");
    assert_ne!(result["isError"], true, "{result}");
    assert!(result.to_string().contains("Final Subject"));
    assert_eq!(server.read_capacity.available_permits(), 2);
});

case!(read_only_failure_and_panic_release_dispatch_guards, {
    let mut server = read_only_server();
    server.read_capacity = Arc::new(Semaphore::new(1));
    let denied = server
        .call_handler("tools/call", add(OTHER, false))
        .await
        .expect_err("test fixture operation should fail");
    assert_eq!(denied.code, ErrorCode(-32602));
    assert_eq!(server.read_capacity.available_permits(), 1);
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("read test fixture"),
        MODEL
    );
    let missing = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.read_element","arguments":{"name":"Missing"}}),
        )
        .await
        .expect("test fixture operation should succeed");
    assert_eq!(missing["isError"], true);
    assert_eq!(server.read_capacity.available_permits(), 1);
    assert!(server.call_handler("unknown", json!({})).await.is_err());
    assert_eq!(server.read_capacity.available_permits(), 1);
    server.before_dispatch = Some(Arc::new(|_, _| panic!("injected blocking read panic")));
    let failure = server
        .call_handler("tools/list", json!({}))
        .await
        .expect_err("test fixture operation should fail");
    assert!(failure.message.contains("MCP read failed"), "{failure}");
    assert_eq!(server.read_capacity.available_permits(), 1);
    assert!(server.write_lock.try_write().is_ok());
    server.before_dispatch = None;
    let result = server
        .call_handler("tools/list", json!({}))
        .await
        .expect("test fixture operation should succeed");
    assert!(result["tools"].is_array());
});

async fn owned_http_selector_error_envelopes(commits: bool) {
    let executable = std::env::var_os("REQVIRE_TEST_BIN")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .expect("test fixture operation should succeed")
                .parent()
                .expect("fixture path has a parent")
                .parent()
                .expect("fixture path has a parent")
                .join(format!("reqvire{}", std::env::consts::EXE_SUFFIX))
        });
    let worktrees = crate::mcp_worktrees::Worktrees::start(
        &std::env::current_dir().expect("test fixture operation should succeed"),
        &executable,
        commits,
        false,
        true,
        false,
        "origin",
    )
    .expect("test fixture operation should succeed");
    let original = worktrees
        .browser_context(None)
        .expect("test fixture operation should succeed")
        .metadata();
    let exclusions = reqvire::exclusions::ExclusionSetBuilder::new()
        .build()
        .expect("test fixture operation should succeed");
    // This is the same HTTP mount used by standalone MCP and embedded Serve.
    let app = mount_worktrees(
        axum::Router::new(),
        worktrees,
        &exclusions,
        &HttpAccess::new(&[], &[]),
    )
    .expect("test fixture operation should succeed");
    let single = http_rpc(
        app.clone(),
        1,
        "tools/call",
        json!({"name":"reqvire.search","arguments":{}}),
    )
    .await;
    assert_eq!(
        single["result"]["_meta"]["reqvire/context"]["worktree_id"],
        original["worktree_id"]
    );
    let created = http_rpc(app.clone(), 2, "tools/call", json!({"name":"reqvire.worktree.create","arguments":{"branch":"selector-child","base_ref":"HEAD"}})).await;
    assert!(created.get("error").is_none(), "{created}");
    assert_ne!(created["result"]["isError"], true, "{created}");
    let child = created["result"]["structuredContent"].clone();
    let root = std::env::current_dir().expect("test fixture operation should succeed");
    let child_root = std::path::PathBuf::from(
        child["workspace_root"]
            .as_str()
            .expect("expected a string in the test response"),
    );
    let git_state = |path: &std::path::Path| {
        [vec!["rev-parse", "HEAD"], vec!["status", "--porcelain=v1"]].map(|args| {
            let output = Command::new("git")
                .current_dir(path)
                .args(args)
                .output()
                .expect("test fixture operation should succeed");
            assert!(output.status.success());
            output.stdout
        })
    };
    let before = [git_state(&root), git_state(&child_root)];
    let assert_tool_error = |wire: &Value, tool: &str, diagnostic: &str| {
        assert!(
            wire.get("error").is_none(),
            "tool error changed into protocol error: {wire}"
        );
        let result = &wire["result"];
        assert_eq!(result["isError"], true, "{wire}");
        assert_eq!(result["structuredContent"]["error"]["tool"], tool, "{wire}");
        assert!(
            result["structuredContent"]["error"]["message"]
                .as_str()
                .expect("expected a string in the test response")
                .contains(diagnostic),
            "{wire}"
        );
        assert!(
            result.get("_meta").is_none(),
            "unresolved selection must not gain a context: {wire}"
        );
        assert!(result["structuredContent"].get("context").is_none());
    };
    for (name, args) in [
        ("reqvire.search", json!({})),
        ("reqvire.workspace_status", json!({})),
        ("reqvire.semantic.sparql", json!({"query":"ASK {}"})),
        ("reqvire.format", json!({"fix":false})),
        (
            "reqvire.add_element",
            add(OTHER, false)["arguments"].clone(),
        ),
    ] {
        let omitted = http_rpc(
            app.clone(),
            3,
            "tools/call",
            json!({"name":name,"arguments":args}),
        )
        .await;
        assert_tool_error(&omitted, name, "worktree_id is required");
        for id in [&original["worktree_id"], &child["worktree_id"]] {
            assert!(omitted["result"]["structuredContent"]["error"]["message"]
                .as_str()
                .expect("expected a string in the test response")
                .contains(id.as_str().expect("expected a string in the test response")));
        }
        let mut unknown_args = args.clone();
        unknown_args["worktree_id"] = json!("unknown-context");
        let unknown = http_rpc(
            app.clone(),
            4,
            "tools/call",
            json!({"name":name,"arguments":unknown_args}),
        )
        .await;
        assert_tool_error(
            &unknown,
            name,
            "Unknown or removed worktree_id: unknown-context",
        );
    }
    for selector in [json!(42), json!(""), Value::Null] {
        let invalid = http_rpc(
            app.clone(),
            5,
            "tools/call",
            json!({"name":"reqvire.search","arguments":{"worktree_id":selector}}),
        )
        .await;
        assert_eq!(invalid["error"]["code"], -32602, "{invalid}");
        assert!(invalid.get("result").is_none());
    }
    for (method, params) in [
        (
            "resources/read",
            json!({"uri":"reqvire://workspace/status"}),
        ),
        (
            "resources/read",
            json!({"uri":"reqvire://workspace/status?worktree_id=unknown-context"}),
        ),
        (
            "prompts/get",
            json!({"name":"reqvire.workflow.explore_model","arguments":{}}),
        ),
        (
            "prompts/get",
            json!({"name":"reqvire.workflow.explore_model","arguments":{"worktree_id":"unknown-context"}}),
        ),
    ] {
        let rejected = http_rpc(app.clone(), 6, method, params).await;
        assert_eq!(rejected["error"]["code"], -32602, "{rejected}");
        assert!(rejected.get("result").is_none());
    }
    for context in [&original, &child] {
        let read = http_rpc(
            app.clone(),
            7,
            "tools/call",
            json!({"name":"reqvire.search","arguments":{"worktree_id":context["worktree_id"]}}),
        )
        .await;
        assert_ne!(read["result"]["isError"], true, "{read}");
        for field in ["worktree_id", "head", "model_revision"] {
            assert_eq!(
                read["result"]["_meta"]["reqvire/context"][field], context[field],
                "{read}"
            );
        }
    }
    assert_eq!([git_state(&root), git_state(&child_root)], before);
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("read test fixture"),
        MODEL
    );
    assert_eq!(
        std::fs::read_to_string(child_root.join("Model.md")).expect("read test fixture"),
        MODEL
    );
    let removed = http_rpc(
        app.clone(),
        8,
        "tools/call",
        json!({"name":"reqvire.worktree.remove","arguments":{"worktree_id":child["worktree_id"]}}),
    )
    .await;
    assert_ne!(removed["result"]["isError"], true, "{removed}");
    let stale = http_rpc(
        app.clone(),
        9,
        "tools/call",
        json!({"name":"reqvire.search","arguments":{"worktree_id":child["worktree_id"]}}),
    )
    .await;
    assert_tool_error(
        &stale,
        "reqvire.search",
        child["worktree_id"]
            .as_str()
            .expect("expected a string in the test response"),
    );
    let remaining = http_rpc(
        app,
        10,
        "tools/call",
        json!({"name":"reqvire.search","arguments":{}}),
    )
    .await;
    assert_eq!(
        remaining["result"]["_meta"]["reqvire/context"]["worktree_id"],
        original["worktree_id"]
    );
}

case!(owned_http_selector_errors_without_commits, {
    owned_http_selector_error_envelopes(false).await;
});
case!(owned_http_selector_errors_with_commits, {
    owned_http_selector_error_envelopes(true).await;
});

#[cfg(unix)]
async fn owned_http_reads_overlap_writes(commits: bool) {
    use std::os::unix::fs::PermissionsExt;
    // Worker startup/reopening includes Git processes and runtime construction.
    // These are correctness checks, so allow slower CI hosts while keeping the
    // read barrier held longer than any individual response deadline.
    const RESPONSE_TIMEOUT: Duration = Duration::from_secs(15);
    let barrier = tempfile::tempdir().expect("test fixture operation should succeed");
    let real_git = Command::new("sh")
        .args(["-c", "command -v git"])
        .output()
        .expect("test fixture operation should succeed");
    let real_git = String::from_utf8(real_git.stdout)
        .expect("test fixture operation should succeed")
        .trim()
        .to_owned();
    let shim = barrier.path().join("git");
    std::fs::write(
        &shim,
        format!(
            r#"#!/usr/bin/env python3
import os,sys,time
from pathlib import Path
base=Path({base})
if sys.argv[1:]==['status','--porcelain'] and (base/'armed').exists():
    (base/'entered').touch()
    end=time.monotonic()+60
    while not (base/'release').exists():
        if time.monotonic()>end: sys.exit(99)
        time.sleep(0.005)
os.execv({git},[{git},*sys.argv[1:]])
"#,
            base = json!(barrier.path()),
            git = json!(real_git)
        ),
    )
    .expect("write test fixture");
    std::fs::set_permissions(&shim, std::fs::Permissions::from_mode(0o755))
        .expect("test fixture operation should succeed");
    // Each case runs in its own process, so this cannot change another test's PATH.
    std::env::set_var(
        "PATH",
        format!(
            "{}:{}",
            barrier.path().display(),
            std::env::var("PATH").expect("test fixture operation should succeed")
        ),
    );
    let executable = std::env::var_os("REQVIRE_TEST_BIN")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .expect("test fixture operation should succeed")
                .parent()
                .expect("fixture path has a parent")
                .parent()
                .expect("fixture path has a parent")
                .join("reqvire")
        });
    let worktrees = crate::mcp_worktrees::Worktrees::start(
        &std::env::current_dir().expect("test fixture operation should succeed"),
        &executable,
        commits,
        false,
        true,
        false,
        "origin",
    )
    .expect("test fixture operation should succeed");
    let original = worktrees
        .browser_context(None)
        .expect("test fixture operation should succeed");
    let before = original.metadata();
    let mut server = read_only_server();
    server.worktrees = Some(Arc::clone(&worktrees));
    server.read_capacity = Arc::new(Semaphore::new(1));
    server.control_capacity = Arc::new(Semaphore::new(1));
    let app = mount_server(
        axum::Router::new(),
        server.clone(),
        &HttpAccess::new(&[], &[]),
    );
    let arm = || {
        let _ = std::fs::remove_file(barrier.path().join("release"));
        let _ = std::fs::remove_file(barrier.path().join("entered"));
        std::fs::write(barrier.path().join("armed"), "").expect("write test fixture");
    };
    let release =
        || std::fs::write(barrier.path().join("release"), "").expect("write test fixture");
    async fn entered(base: &std::path::Path, stage: &str) {
        tokio::time::timeout(RESPONSE_TIMEOUT, async {
            while !base.join("entered").exists() {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .unwrap_or_else(|_| panic!("worker did not enter held read: {stage}"));
        // The held Git child still waits, but later publication may observe Git.
        std::fs::remove_file(base.join("armed")).expect("remove test fixture");
    }
    async fn response(stage: &str, job: tokio::task::JoinHandle<Value>) -> Value {
        tokio::time::timeout(RESPONSE_TIMEOUT, job)
            .await
            .unwrap_or_else(|_| panic!("worker response timed out: {stage}"))
            .expect("test fixture operation should succeed")
    }
    arm();
    let mut old = tokio::spawn(http_rpc(
        app.clone(),
        1,
        "tools/call",
        json!({"name":"reqvire.workspace_status"}),
    ));
    tokio::select! {
        early = &mut old => panic!("Read returned before barrier: {early:?}"),
        _ = entered(barrier.path(), "initial") => {}
    }
    let excess = http_rpc(
        app.clone(),
        2,
        "tools/call",
        json!({"name":"reqvire.search"}),
    )
    .await;
    assert_eq!(excess["error"]["code"], -32000, "{excess}");
    assert_eq!(excess["error"]["data"]["retryable"], true);
    let written = response(
        "mutation while read is held",
        tokio::spawn(http_rpc(app.clone(), 3, "tools/call", add(OTHER, false))),
    )
    .await;
    assert!(
        written.get("error").is_none() && written["result"]["isError"] != true,
        "{written}"
    );
    assert!(
        !old.is_finished(),
        "the write must finish while the old read is held"
    );
    let accepted = original.metadata();
    assert_ne!(accepted["model_revision"], before["model_revision"]);
    let runtime = original
        .runtime()
        .expect("test fixture operation should succeed");
    release();
    let old = response("released snapshot read", old).await;
    let metadata = &old["result"]["_meta"]["reqvire/context"];
    for field in ["model_revision", "head", "pending_changes"] {
        assert_eq!(metadata[field], before[field], "captured {field}: {old}");
    }
    assert_eq!(
        old["result"]["structuredContent"]["model"]["fingerprint"],
        before["model_revision"]
    );
    assert_eq!(
        metadata["writes_available"], true,
        "an internal commit is not an external HEAD change"
    );
    assert_eq!(
        original.metadata()["model_revision"],
        accepted["model_revision"]
    );
    assert!(Arc::ptr_eq(
        &runtime,
        &original
            .runtime()
            .expect("test fixture operation should succeed")
    ));

    // The asynchronous caller can disappear; the actual dispatched read still
    // owns admission until it finishes. Control operations retain their lane.
    arm();
    let cancelled_server = server.clone();
    let mut cancelled = tokio::spawn(async move {
        cancelled_server
            .call_handler("tools/call", json!({"name":"reqvire.workspace_status"}))
            .await
    });
    tokio::select! {
        early = &mut cancelled => panic!("Cancelled read returned before barrier: {early:?}"),
        _ = entered(barrier.path(), "cancellation") => {}
    }
    cancelled.abort();
    let _ = cancelled.await;
    assert_eq!(server.read_capacity.available_permits(), 0);
    let preview = response("preview while cancelled read is held", tokio::spawn(http_rpc(app.clone(), 4, "tools/call",
        json!({"name":"reqvire.remove_element", "arguments":{"element_name":"Other Subject","dry_run":true}})))).await;
    assert!(
        preview.get("error").is_none() && preview["result"]["isError"] != true,
        "{preview}"
    );
    release();
    tokio::time::timeout(RESPONSE_TIMEOUT, async {
        while server.read_capacity.available_permits() != 1 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("cancelled read did not release admission after barrier release");
    assert_eq!(
        original.metadata()["model_revision"],
        accepted["model_revision"]
    );

    // Removal must stop a worker even though an in-flight read owns a transport
    // handle, release ownership, and return a contextual error to its caller.
    let commit = http_rpc(
        app.clone(),
        5,
        "tools/call",
        json!({"name":"reqvire.git.commit","arguments":{"message":"Prepare branch"}}),
    )
    .await;
    assert_ne!(commit["result"]["isError"], true, "{commit}");
    let branch = http_rpc(app.clone(), 6, "tools/call", json!({"name":"reqvire.worktree.create","arguments":{"branch":"secondary","base_ref":"HEAD"}})).await;
    let id = branch["result"]["structuredContent"]["worktree_id"]
        .as_str()
        .expect("branch created");
    let secondary = worktrees
        .browser_context(Some(id))
        .expect("test fixture operation should succeed");
    arm();
    let pending = tokio::spawn(http_rpc(
        app.clone(),
        7,
        "tools/call",
        json!({"name":"reqvire.workspace_status","arguments":{"worktree_id":id}}),
    ));
    entered(barrier.path(), "removal").await;
    let removed = response(
        "removal while read is held",
        tokio::spawn(http_rpc(
            app.clone(),
            8,
            "tools/call",
            json!({"name":"reqvire.worktree.remove","arguments":{"worktree_id":id}}),
        )),
    )
    .await;
    assert!(
        removed.get("error").is_none() && removed["result"]["isError"] != true,
        "{removed}"
    );
    let pending = response("removed worker read", pending).await;
    release();
    assert_eq!(pending["result"]["isError"], true, "{pending}");
    assert_eq!(
        pending["result"]["_meta"]["reqvire/context"]["worktree_id"],
        id
    );
    assert!(!secondary.root.exists());
    assert!(secondary.runtime().is_err());
    assert_eq!(original.metadata()["available"], true);

    arm();
    let pending = tokio::spawn(http_rpc(
        app.clone(),
        9,
        "tools/call",
        json!({"name":"reqvire.workspace_status"}),
    ));
    entered(barrier.path(), "reopening").await;
    assert!(Command::new("git")
        .args(["commit", "--allow-empty", "-qm", "External HEAD change"])
        .status()
        .expect("test fixture operation should succeed")
        .success());
    let reopened = response(
        "reopening while read is held",
        tokio::spawn(http_rpc(
            app.clone(),
            10,
            "tools/call",
            json!({"name":"reqvire.worktree.open", "arguments":{"branch":before["branch"]}}),
        )),
    )
    .await;
    release();
    assert!(
        reopened.get("error").is_none() && reopened["result"]["isError"] != true,
        "{reopened}"
    );
    let reopened = &reopened["result"]["structuredContent"];
    assert_ne!(reopened["worktree_id"], before["worktree_id"]);
    assert_ne!(reopened["head"], accepted["head"]);
    assert_eq!(reopened["available"], true);
    let pending = response("replaced worker read", pending).await;
    assert_eq!(pending["result"]["isError"], true, "{pending}");
    assert_eq!(
        pending["result"]["_meta"]["reqvire/context"]["worktree_id"],
        before["worktree_id"]
    );
    assert!(original.runtime().is_err());
}

#[cfg(unix)]
case!(owned_http_concurrent_reads_without_commits, {
    owned_http_reads_overlap_writes(false).await;
});
#[cfg(unix)]
case!(owned_http_concurrent_reads_with_commits, {
    owned_http_reads_overlap_writes(true).await;
});
