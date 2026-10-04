//! Direct observations of the existing post-write hook and workspace write gate.
//! HTTP behavior is covered by test-cache-integration/check_correctness.py.
use super::*;
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn isolated test process");
    let deadline = Instant::now() + Duration::from_secs(20);
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
}

macro_rules! case {
    ($name:ident, $body:block) => {
        #[test]
        fn $name() { isolated(stringify!($name), async $body); }
    };
}

fn server(fail_refresh: bool) -> (ReqvireMcpServer, Arc<AtomicUsize>, Arc<AtomicBool>) {
    server_with_commits(fail_refresh, true)
}

fn server_with_commits(
    fail_refresh: bool,
    enable_commits: bool,
) -> (ReqvireMcpServer, Arc<AtomicUsize>, Arc<AtomicBool>) {
    let calls = Arc::new(AtomicUsize::new(0));
    // Simulate an already outstanding refresh diagnostic. Any hook invocation
    // would clear it on success, so we can observe an incorrect gate directly.
    let diagnostic = Arc::new(AtomicBool::new(true));
    let hook_calls = Arc::clone(&calls);
    let hook_diagnostic = Arc::clone(&diagnostic);
    let hook: PostWriteHook = Arc::new(move |_model| {
        hook_calls.fetch_add(1, Ordering::SeqCst);
        hook_diagnostic.store(fail_refresh, Ordering::SeqCst);
        Box::pin(async move {
            if fail_refresh {
                Err(ReqvireError::ProcessError(
                    "injected runtime failure".into(),
                ))
            } else {
                Ok(())
            }
        })
    });
    (
        ReqvireMcpServer::new_with_write_lock(
            true,
            enable_commits,
            false,
            &reqvire::exclusions::ExclusionSetBuilder::new()
                .build()
                .expect("build test configuration"),
            Arc::new(Mutex::new(())),
            Some(hook),
        )
        .expect("start clean mutation session"),
        calls,
        diagnostic,
    )
}
fn add(content: &str, dry_run: bool) -> Value {
    json!({"name": "reqvire.add_element", "arguments": {
        "file": "Model.md", "content": content.split_once("# Elements\n\n").expect("expected fixture to contain an Elements header").1, "dry_run": dry_run,
    }})
}

case!(rejected_core_mutation_does_not_invoke_refresh_hook, {
    let (server, calls, diagnostic) = server(false);
    let result = server
        .call_handler("tools/call", add(MODEL, false), true)
        .await
        .expect("execute test MCP request");
    assert_eq!(
        result["isError"], true,
        "must exercise tool rejection inside JSON-RPC success"
    );
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("read generated test file"),
        MODEL
    );
    assert_eq!(
        calls.load(Ordering::SeqCst),
        0,
        "rejected tool invoked the post-write hook"
    );
    assert!(
        diagnostic.load(Ordering::SeqCst),
        "rejected tool cleared the existing diagnostic"
    );
});

case!(rejected_tool_error_is_not_replaced_by_refresh_error, {
    let (server, calls, _) = server(true);
    let result = server
        .call_handler("tools/call", add(MODEL, false), true)
        .await;
    assert!(
        result.is_ok(),
        "rejected mutation was misreported as succeeded-but-refresh-failed: {result:?}"
    );
    assert_eq!(result.expect("add: expected success")["isError"], true);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
});

case!(
    rejected_core_mutation_preserves_previous_refresh_diagnostic,
    {
        let (server, _, diagnostic) = server(false);
        let result = server
            .call_handler("tools/call", add(MODEL, false), true)
            .await
            .expect("execute test MCP request");
        assert_eq!(result["isError"], true);
        assert!(
            diagnostic.load(Ordering::SeqCst),
            "rejected tool cleared the existing diagnostic"
        );
    }
);

case!(preview_preserves_sources_and_refresh_diagnostic, {
    let (server, calls, diagnostic) = server(false);
    let result = server
        .call_handler("tools/call", add(OTHER, true), true)
        .await
        .expect("execute test MCP request");
    assert!(!result["isError"].as_bool().unwrap_or(false));
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("read generated test file"),
        MODEL
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(diagnostic.load(Ordering::SeqCst));
});

case!(protocol_error_does_not_invoke_refresh_hook, {
    let (server, calls, diagnostic) = server(false);
    let result = server
        .call_handler(
            "tools/call",
            json!({"name": "reqvire.add_element", "arguments": {}}),
            true,
        )
        .await;
    assert!(result.is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(diagnostic.load(Ordering::SeqCst));
});

case!(
    persisted_success_invokes_hook_once_and_refreshes_semantic_reads,
    {
        let (server, calls, diagnostic) = server(false);
        let result = server
            .call_handler("tools/call", add(OTHER, false), true)
            .await
            .expect("execute test MCP request");
        assert!(!result["isError"].as_bool().unwrap_or(false));
        assert!(std::fs::read_to_string("Model.md")
            .expect("read generated test file")
            .contains("Other Subject"));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(!diagnostic.load(Ordering::SeqCst));
        let read = server.call_handler("tools/call", json!({"name": "reqvire.semantic.sparql", "arguments": {
        "query": "ASK { ?s <https://www.reqvire.org/ontology#elementName> \"Other Subject\" }"
    }}), false).await.expect("execute test MCP request");
        assert_eq!(read["structuredContent"]["boolean"], true);
    }
);

case!(
    refresh_failure_keeps_persisted_success_distinct_from_rejection,
    {
        let (server, calls, diagnostic) = server(true);
        let result = server
            .call_handler("tools/call", add(OTHER, false), true)
            .await;
        assert!(result
            .expect_err("expected the operation to fail")
            .message
            .contains("mutation succeeded but Explorer runtime refresh failed"));
        assert!(std::fs::read_to_string("Model.md")
            .expect("read generated test file")
            .contains("Other Subject"));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(diagnostic.load(Ordering::SeqCst));
    }
);

case!(read_during_write_gate_cannot_publish_partial_persistence, {
    let server = ReqvireMcpServer::new_with_write_lock(
        false,
        false,
        false,
        &reqvire::exclusions::ExclusionSetBuilder::new()
            .build()
            .expect("ownership test assertion"),
        Arc::new(Mutex::new(())),
        None,
    )
    .expect("ownership test assertion");
    let request = json!({"name": "reqvire.search", "arguments": {}});
    server
        .call_handler("tools/call", request.clone(), false)
        .await
        .expect("execute test MCP request");
    let gate = server.write_lock.lock().await;
    // Controlled intermediate state of a write holding the same gate used by MCP.
    std::fs::write(
        "Model.md",
        MODEL.replace("Cache Subject", "Partial Subject"),
    )
    .expect("write test fixture");
    let mut read = Box::pin(server.call_handler("tools/call", request.clone(), false));
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
        .call_handler("tools/call", request, false)
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
        true,
        false,
        false,
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
    ReqvireMcpServer::new_with_write_lock(
        true,
        false,
        false,
        &reqvire::exclusions::ExclusionSetBuilder::new()
            .build()
            .expect("ownership test assertion"),
        Arc::new(Mutex::new(())),
        None,
    )
}
case!(mutation_commits_are_disabled_by_default, {
    let (server, calls, _) = server_with_commits(false, false);
    let head = git_test(&["rev-parse", "HEAD"]);
    std::fs::write("unrelated.txt", "staged human work").expect("write unrelated file");
    git_test(&["add", "unrelated.txt"]);
    let index = std::fs::read(".git/index").expect("index bytes");
    // Commit-disabled writes do not need or acquire the Git index lock.
    std::fs::write(".git/index.lock", "external lock").expect("lock index");
    std::fs::write("Model.md", "external invalid edit").expect("external model edit");
    for (count, content) in [
        OTHER.to_string(),
        OTHER.replace("Other Subject", "Third Subject"),
    ]
    .iter()
    .enumerate()
    {
        let result = server
            .call_handler("tools/call", add(content, false), true)
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
        assert_eq!(
            calls.load(Ordering::SeqCst),
            count + 1,
            "persistence must refresh Explorer without a commit"
        );
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
        }}), false)
        .await
        .expect("read accepted semantic snapshot");
    assert_eq!(read["structuredContent"]["boolean"], true);
    assert_eq!(
        calls.load(Ordering::SeqCst),
        2,
        "reads must not refresh Explorer"
    );
    std::fs::remove_file(".git/index.lock").expect("unlock index");
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
        let (server, _, _) = server(false);
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
                false,
            )
            .await
            .expect("ownership test assertion");
        assert!(read.to_string().contains("Cache Subject"));
        let result = server
            .call_handler("tools/call", add(OTHER, false), true)
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
    let (server, calls, _) = server_with_commits(false, enable_commits);
    let before = git_test(&["rev-parse", "HEAD"]);
    let index = std::fs::read(".git/index").expect("read index");
    let preview = server
        .call_handler("tools/call", add(OTHER, true), true)
        .await
        .expect("ownership test assertion");
    assert!(!preview["isError"].as_bool().unwrap_or(false));
    let rejected = server
        .call_handler("tools/call", add(MODEL, false), true)
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
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(git_test(&["rev-parse", "HEAD"]), before);
    let request = json!({"name":"reqvire.format","arguments":{"fix":true}});
    let first = server
        .call_handler("tools/call", request.clone(), true)
        .await
        .expect("ownership test assertion");
    assert!(!first["isError"].as_bool().unwrap_or(false), "{first}");
    let formatted = git_test(&["rev-parse", "HEAD"]);
    let hooks = calls.load(Ordering::SeqCst);
    let formatted_index = std::fs::read(".git/index").expect("formatted index");
    let formatted_model = std::fs::read("Model.md").expect("formatted model");
    let second = server
        .call_handler("tools/call", request, true)
        .await
        .expect("ownership test assertion");
    assert!(!second["isError"].as_bool().unwrap_or(false), "{second}");
    assert_eq!(git_test(&["rev-parse", "HEAD"]), formatted);
    assert_eq!(calls.load(Ordering::SeqCst), hooks);
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
case!(default_refresh_failure_retains_persisted_snapshot, {
    let (server, calls, _) = server_with_commits(true, false);
    let head = git_test(&["rev-parse", "HEAD"]);
    let error = server
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .expect_err("injected refresh failure");
    assert!(error
        .message
        .contains("mutation succeeded but Explorer runtime refresh failed"));
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
            false,
        )
        .await
        .expect("accepted snapshot after refresh failure");
    assert!(read.to_string().contains("Other Subject"));
    assert!(std::fs::read_to_string("Model.md")
        .expect("persisted model")
        .contains("Other Subject"));
});
case!(external_head_change_stops_mutations, {
    let (server, _, _) = server(false);
    git_test(&["commit", "--allow-empty", "-qm", "external commit"]);
    let result = server
        .call_handler("tools/call", add(OTHER, false), true)
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
    let (server, calls, _) = server(false);
    let head = git_test(&["rev-parse", "HEAD"]);
    std::fs::write(".git/index.lock", "another git operation").expect("ownership test assertion");
    let result = server
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .expect("ownership test assertion");
    assert_eq!(result["isError"], true, "{result}");
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("ownership test assertion"),
        MODEL
    );
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    std::fs::remove_file(".git/index.lock").expect("ownership test assertion");
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
            false,
        )
        .await
        .expect("ownership test assertion");
    assert!(!read.to_string().contains("Other Subject"));
});
#[cfg(unix)]
case!(git_publication_failure_rolls_back_persisted_files, {
    use std::os::unix::fs::PermissionsExt;
    let (server, _, _) = server(false);
    let head = git_test(&["rev-parse", "HEAD"]);
    let hook = ".git/hooks/reference-transaction";
    std::fs::write(hook, "#!/bin/sh\n[ \"$1\" != prepared ]\n").expect("ownership test assertion");
    std::fs::set_permissions(hook, std::fs::Permissions::from_mode(0o755))
        .expect("ownership test assertion");
    let result = server
        .call_handler("tools/call", add(OTHER, false), true)
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
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .expect("ownership test assertion");
    assert!(
        result["structuredContent"]["commit"].is_string(),
        "{result}"
    );
});
case!(owned_folder_move_commits_files_and_rebuilds_reads, {
    let (server, _, _) = server(false);
    let moved = server.call_handler("tools/call", json!({"name":"reqvire.move_file","arguments":{"source_file":"Model.md","target_file":"nested/Moved.md"}}), true).await.expect("ownership test assertion");
    assert!(moved["structuredContent"]["commit"].is_string(), "{moved}");
    assert!(!std::path::Path::new("Model.md").exists());
    let moved = server.call_handler("tools/call", json!({"name":"reqvire.move_folder","arguments":{"source_folder":"nested","target_folder":"renamed"}}), true).await.expect("ownership test assertion");
    assert!(moved["structuredContent"]["commit"].is_string(), "{moved}");
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
            false,
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
        let (server, _, _) = server(false);
        git_test(&["switch", "-qc", "external-branch"]);
        assert!(attempt_server()
            .err()
            .expect("ownership test assertion")
            .to_string()
            .contains("owned"));
        let result = server
            .call_handler("tools/call", add(OTHER, false), true)
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
        let (server, calls, _) = server(false);
        let head = git_test(&["rev-parse", "HEAD"]);
        for dry_run in [true, false] {
            let result = server.call_handler("tools/call", json!({"name":"reqvire.remove_asset","arguments":{"file_path":"external.ttl","dry_run":dry_run}}), true).await.expect("ownership test assertion");
            assert_eq!(result["isError"], true, "{result}");
            assert_eq!(
                std::fs::read_to_string("external.ttl").expect("ownership test assertion"),
                external
            );
            assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
        }
        assert_eq!(calls.load(Ordering::SeqCst), 0);
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
    let (server, _, _) = server(false);
    let result = server.call_handler("tools/call", json!({"name":"reqvire.move_asset","arguments":{"old_path":"evidence.txt","new_path":"moved.sh"}}), true).await.expect("ownership test assertion");
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
            false,
        )
        .await
        .expect("ownership test assertion");
    assert!(read.to_string().contains("moved.sh"), "{read}");
});

case!(new_external_destination_is_not_overwritten, {
    let (server, _, _) = server(false);
    std::fs::write("Taken.md", "human work after startup").expect("write external file");
    let head = git_test(&["rev-parse", "HEAD"]);
    let result = server.call_handler("tools/call", json!({"name":"reqvire.move_file","arguments":{"source_file":"Model.md","target_file":"Taken.md"}}), true).await.expect("tool response");
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
