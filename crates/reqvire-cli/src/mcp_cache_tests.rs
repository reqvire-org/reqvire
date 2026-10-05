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
            Arc::new(RwLock::new(())),
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

fn shared_snapshot_bookkeeping(commits: bool) {
    use crate::mcp_session::MutationSession;
    use reqvire::mutation_io;
    fn prepared(
        session: &mut MutationSession,
        persists: bool,
        reject: bool,
        edit: impl FnOnce(),
    ) -> (Value, bool) {
        let (result, changed) = session.execute(persists, "reqvire.add_element", || {
            edit();
            Ok(if reject {
                json!({"isError":true})
            } else {
                json!({"structuredContent":{}})
            })
        });
        (result.unwrap(), changed)
    }
    fn pending(session: &MutationSession) -> Value {
        session.status()["pending_changes"].clone()
    }

    std::fs::write("asset.txt", "asset").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::write("mode-source.txt", MODEL).unwrap();
        std::fs::write("mode-plain.txt", MODEL).unwrap();
        std::fs::set_permissions("mode-source.txt", std::fs::Permissions::from_mode(0o755))
            .unwrap();
    }
    git_test(&["add", "."]);
    git_test(&["commit", "-qm", "snapshot assets"]);
    let initial_head = git_test(&["rev-parse", "HEAD"]);
    let (server, _, _) = server_with_commits(false, commits);
    let mut session = server.session.as_ref().unwrap().lock().unwrap();
    let first = session.model();
    let again = session.model();
    assert!(Arc::ptr_eq(&first, &again));
    std::fs::write("unrelated.txt", "keep staged").unwrap();
    git_test(&["add", "unrelated.txt"]);
    let unrelated = git_test(&["ls-files", "--stage", "unrelated.txt"]);
    // status() may refresh Git's stat cache; tracked entries must stay intact.
    let index = git_test(&["ls-files", "--stage"]);
    let changed_source = MODEL.replace("alpha", "bravo");
    let (result, changed) = prepared(&mut session, true, false, || {
        mutation_io::write("Model.md", &changed_source).unwrap();
    });
    assert_ne!(result["isError"], true, "{result}");
    assert!(changed);
    let accepted = session.model();
    assert!(!Arc::ptr_eq(&first, &accepted));
    assert!(first
        .graph_registry
        .get_element_by_name("Cache Subject")
        .unwrap()
        .content
        .contains("alpha"));
    assert!(accepted
        .graph_registry
        .get_element_by_name("Cache Subject")
        .unwrap()
        .content
        .contains("bravo"));
    let expected = if commits {
        json!([])
    } else {
        json!(["Model.md"])
    };
    assert_eq!(pending(&session), expected);
    for (persists, reject) in [(false, false), (true, true)] {
        let (_, published) = prepared(&mut session, persists, reject, || {
            mutation_io::write("Model.md", MODEL).unwrap();
        });
        assert!(!published);
        assert!(Arc::ptr_eq(&session.model(), &accepted));
        assert_eq!(pending(&session), expected);
        assert_eq!(std::fs::read_to_string("Model.md").unwrap(), changed_source);
    }
    let (failure, changed) = prepared(&mut session, true, false, || {
        mutation_io::write(
            "Model.md",
            "# Elements\n\n### Invalid\n\n#### Metadata\n  * type: unsupported-type\n",
        )
        .unwrap();
    });
    assert_eq!(failure["isError"], true, "{failure}");
    assert!(!changed);
    assert!(Arc::ptr_eq(&session.model(), &accepted));
    assert_eq!(pending(&session), expected);

    // Revert accepted bytes without importing an intervening external edit.
    std::fs::write("Model.md", "outside accepted snapshot").unwrap();
    let (_, changed) = prepared(&mut session, true, false, || {
        assert_eq!(
            mutation_io::read_to_string("Model.md").unwrap(),
            changed_source
        );
        mutation_io::write("Model.md", MODEL).unwrap();
    });
    assert!(changed);
    assert_eq!(pending(&session), json!([]));
    assert!(accepted
        .graph_registry
        .get_element_by_name("Cache Subject")
        .unwrap()
        .content
        .contains("bravo"));
    #[cfg(unix)]
    {
        let (_, changed) = prepared(&mut session, true, false, || {
            mutation_io::copy("mode-source.txt", "Model.md").unwrap();
        });
        assert!(
            changed,
            "same bytes with a changed executable mode is a prepared change"
        );
        assert_eq!(
            pending(&session),
            if commits {
                json!([])
            } else {
                json!(["Model.md"])
            }
        );
        let (_, changed) = prepared(&mut session, true, false, || {
            mutation_io::copy("mode-plain.txt", "Model.md").unwrap();
        });
        assert!(changed);
        assert_eq!(pending(&session), json!([]));
    }
    let (_, changed) = prepared(&mut session, true, false, || {
        mutation_io::rename("asset.txt", "moved.txt").unwrap();
    });
    assert!(changed);
    assert_eq!(
        pending(&session),
        if commits {
            json!([])
        } else {
            json!(["asset.txt", "moved.txt"])
        }
    );
    let (_, changed) = prepared(&mut session, true, false, || {
        mutation_io::rename("moved.txt", "asset.txt").unwrap();
    });
    assert!(changed);
    assert_eq!(pending(&session), json!([]));
    let (_, changed) = prepared(&mut session, true, false, || {
        mutation_io::remove_file("asset.txt").unwrap();
    });
    assert!(changed);
    assert_eq!(
        pending(&session),
        if commits {
            json!([])
        } else {
            json!(["asset.txt"])
        }
    );
    let (_, changed) = prepared(&mut session, true, false, || {
        mutation_io::write("asset.txt", b"asset").unwrap();
    });
    assert!(changed);
    assert_eq!(pending(&session), json!([]));
    if !commits {
        assert_eq!(git_test(&["rev-parse", "HEAD"]), initial_head);
        assert_eq!(git_test(&["ls-files", "--stage"]), index);
    }
    let (_, changed) = prepared(&mut session, true, false, || {
        mutation_io::write("Model.md", &changed_source).unwrap();
    });
    assert!(changed);
    let result = session.explicit_commit("commit accepted changes").unwrap();
    assert_eq!(
        result["outcome"],
        if commits { "no_op" } else { "completed" }
    );
    assert_eq!(pending(&session), json!([]));
    assert_eq!(
        git_test(&["ls-files", "--stage", "unrelated.txt"]),
        unrelated
    );
    assert_eq!(git_test(&["ls-tree", "HEAD", "unrelated.txt"]), "");
}

case!(shared_snapshot_bookkeeping_without_commits, {
    shared_snapshot_bookkeeping(false);
});
case!(shared_snapshot_bookkeeping_with_commits, {
    shared_snapshot_bookkeeping(true);
});

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
        Arc::new(RwLock::new(())),
        None,
    )
    .expect("ownership test assertion");
    let request = json!({"name": "reqvire.search", "arguments": {}});
    server
        .call_handler("tools/call", request.clone(), false)
        .await
        .expect("execute test MCP request");
    let gate = server.write_lock.write().await;
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
        "origin",
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
        Arc::new(RwLock::new(())),
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

#[cfg(unix)]
fn native_mode(path: &str) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .expect("permission regression")
        .permissions()
        .mode()
        & 0o7777
}
#[cfg(unix)]
fn fixture_mode(path: &str, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
        .expect("permission regression");
}
#[cfg(unix)]
fn evidence_fixture() {
    std::fs::write(
        "Evidence.md",
        include_str!("../../../tests/test-cache-integration/fixtures/evidence.md.txt"),
    )
    .expect("permission regression");
    std::fs::write("evidence.txt", "#!/bin/sh\nexit 0\n").expect("permission regression");
    fixture_mode("evidence.txt", 0o755);
    git_test(&["config", "core.filemode", "true"]);
    git_test(&["add", "."]);
    git_test(&["commit", "-qm", "evidence"]);
}
#[cfg(unix)]
fn reject_reference_publication() {
    let hook = ".git/hooks/reference-transaction";
    std::fs::write(hook, "#!/bin/sh\n[ \"$1\" != prepared ]\n").expect("permission regression");
    fixture_mode(hook, 0o755);
}
#[cfg(unix)]
async fn permission_content_edits(commits: bool, mode: u32, deny: bool) {
    use crate::mcp_session::permission_test_io as io;
    fixture_mode("Model.md", mode);
    git_test(&["config", "core.filemode", "true"]);
    git_test(&["add", "Model.md"]);
    git_test(&["commit", "--allow-empty", "-qm", "native mode"]);
    let head = git_test(&["rev-parse", "HEAD"]);
    let index = git_test(&["ls-files", "--stage"]);
    let (server, _, _) = server_with_commits(false, commits);
    io::configure(if deny { &["*"] } else { &[] }, &[]);
    let result = server
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .expect("permission regression");
    assert_ne!(result["isError"], true, "{result}");
    assert_eq!(
        native_mode("Model.md"),
        mode,
        "ordinary edits preserve the complete native mode"
    );
    assert!(
        io::calls().is_empty(),
        "unnecessary chmod: {:?}",
        io::calls()
    );
    let new = server.call_handler("tools/call", json!({"name":"reqvire.add_element","arguments":{
        "file":"New.md", "content":"### New Root\n\nNew capability.\n\n#### Metadata\n  * type: capability\n"
    }}), true).await.expect("permission regression");
    assert_ne!(new["isError"], true, "{new}");
    assert_eq!(native_mode("New.md") & 0o111, 0);
    assert!(io::calls().is_empty());
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
            false,
        )
        .await
        .expect("permission regression");
    assert!(read.to_string().contains("Other Subject"));
    if commits {
        assert!(result["structuredContent"]["commit"].is_string());
        assert_eq!(git_test(&["status", "--porcelain"]), "");
    } else {
        assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
        assert_eq!(git_test(&["ls-files", "--stage"]), index);
    }
}
#[cfg(unix)]
case!(permission_edits_without_commits_skip_denied_chmod, {
    permission_content_edits(false, 0o640, true).await;
});
#[cfg(unix)]
case!(permission_edits_with_commits_skip_denied_chmod, {
    permission_content_edits(true, 0o640, true).await;
});
#[cfg(unix)]
case!(permission_edits_without_commits_preserve_executable_mode, {
    permission_content_edits(false, 0o750, false).await;
});
#[cfg(unix)]
case!(permission_edits_with_commits_preserve_executable_mode, {
    permission_content_edits(true, 0o750, false).await;
});
#[cfg(unix)]
case!(permission_edits_preserve_group_execute_bit, {
    permission_content_edits(true, 0o650, true).await;
});

#[cfg(unix)]
async fn required_permission_failure(commits: bool, ineffective: bool, after_delete: bool) {
    use crate::mcp_session::permission_test_io as io;
    evidence_fixture();
    let destination = if after_delete {
        "z-moved.sh"
    } else {
        "a-moved.sh"
    };
    let destinations = [destination];
    let before = std::fs::read("Evidence.md").expect("permission regression");
    let head = git_test(&["rev-parse", "HEAD"]);
    let index = git_test(&["ls-files", "--stage"]);
    let (server, _, _) = server_with_commits(false, commits);
    let revision = server
        .session
        .as_ref()
        .expect("permission regression")
        .lock()
        .expect("permission regression")
        .status()["model_revision"]
        .clone();
    io::configure(
        if ineffective { &[] } else { &destinations },
        if ineffective { &destinations } else { &[] },
    );
    let result = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.move_asset","arguments":{
                "old_path":"evidence.txt","new_path":destination
            }}),
            true,
        )
        .await
        .expect("permission regression");
    assert_eq!(
        result["isError"], true,
        "required executable mode must not be ignored: {result}"
    );
    assert!(
        result.to_string().contains(destination) && result.to_string().contains("permission"),
        "{result}"
    );
    assert_eq!(
        std::fs::read("Evidence.md").expect("permission regression"),
        before
    );
    assert!(!std::path::Path::new(destination).exists());
    assert_eq!(native_mode("evidence.txt"), 0o755);
    if after_delete {
        assert!(
            io::calls().iter().any(|p| p.ends_with("evidence.txt")),
            "deleted executable must regain its native mode"
        );
    } else {
        assert!(
            !io::writes().iter().any(|p| p.ends_with("evidence.txt")),
            "unattempted source must not be rewritten during rollback"
        );
    }
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(git_test(&["ls-files", "--stage"]), index);
    assert_eq!(git_test(&["status", "--porcelain"]), "");
    assert_eq!(
        server
            .session
            .as_ref()
            .expect("permission regression")
            .lock()
            .expect("permission regression")
            .status()["model_revision"],
        revision
    );
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
            false,
        )
        .await
        .expect("permission regression");
    assert_ne!(
        read["isError"], true,
        "verified rollback must leave the session usable: {read}"
    );
    io::configure(&[], &[]);
    let retry = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.move_asset","arguments":{
                "old_path":"evidence.txt","new_path":destination
            }}),
            true,
        )
        .await
        .expect("permission regression");
    assert_ne!(retry["isError"], true, "{retry}");
    assert!(native_mode(destination) & 0o111 != 0);
}
#[cfg(unix)]
case!(permission_required_denial_without_commits_recovers, {
    required_permission_failure(false, false, false).await;
});
#[cfg(unix)]
case!(permission_required_denial_with_commits_recovers, {
    required_permission_failure(true, false, false).await;
});
#[cfg(unix)]
case!(permission_ineffective_change_is_rejected, {
    required_permission_failure(false, true, false).await;
});

#[cfg(unix)]
case!(permission_deleted_asset_recovers_without_commits, {
    required_permission_failure(false, false, true).await;
});
#[cfg(unix)]
case!(permission_deleted_asset_recovers_with_commits, {
    required_permission_failure(true, false, true).await;
});

#[cfg(unix)]
case!(permission_rollback_skips_matching_native_mode, {
    use crate::mcp_session::permission_test_io as io;
    fixture_mode("Model.md", 0o640);
    let head = git_test(&["rev-parse", "HEAD"]);
    let index = git_test(&["ls-files", "--stage"]);
    let (server, _, _) = server(false);
    reject_reference_publication();
    io::configure(&["*"], &[]);
    let result = server
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .expect("permission regression");
    assert_eq!(result["isError"], true, "{result}");
    assert!(
        result.to_string().contains("update-ref"),
        "must reach Git publication before the injected failure: {result}"
    );
    assert_eq!(native_mode("Model.md"), 0o640);
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("permission regression"),
        MODEL
    );
    assert!(io::calls().is_empty());
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(git_test(&["ls-files", "--stage"]), index);
    std::fs::remove_file(".git/hooks/reference-transaction").expect("permission regression");
    let retry = server
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .expect("permission regression");
    assert_ne!(retry["isError"], true, "{retry}");
});

#[cfg(unix)]
case!(permission_required_restore_failure_still_disables_writes, {
    use crate::mcp_session::permission_test_io as io;
    evidence_fixture();
    let (server, _, _) = server(false);
    let revision = server
        .session
        .as_ref()
        .expect("permission regression")
        .lock()
        .expect("permission regression")
        .status()["model_revision"]
        .clone();
    reject_reference_publication();
    io::configure(&["evidence.txt"], &[]);
    let result = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.move_asset","arguments":{
                "old_path":"evidence.txt","new_path":"z-moved.sh"
            }}),
            true,
        )
        .await
        .expect("permission regression");
    assert_eq!(result["isError"], true, "{result}");
    assert!(
        result.to_string().contains("recovery failed")
            && result.to_string().contains("evidence.txt"),
        "{result}"
    );
    let status = server
        .session
        .as_ref()
        .expect("permission regression")
        .lock()
        .expect("permission regression")
        .status();
    assert_eq!(status["model_revision"], revision);
    assert_eq!(status["available"], true);
    assert_eq!(status["writes_available"], false);
    io::configure(&[], &[]);
    let retry = server
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .expect("permission regression");
    assert_eq!(retry["isError"], true);
    assert!(retry.to_string().contains("requires recovery"));
});

#[cfg(unix)]
async fn ignored_filemode_keeps_logical_mode(commits: bool) {
    use crate::mcp_session::permission_test_io as io;
    evidence_fixture();
    git_test(&["config", "core.filemode", "false"]);
    fixture_mode("Model.md", 0o777); // typical non-authoritative bind-mount mode
    fixture_mode("evidence.txt", 0o644); // Git still tracks the executable asset
    let (server, _, _) = server_with_commits(false, commits);
    io::configure(&["*"], &[]);
    let result = server
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .expect("permission regression");
    assert_ne!(result["isError"], true, "{result}");
    let moved = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.move_asset","arguments":{
                "old_path":"evidence.txt","new_path":"moved.sh"
            }}),
            true,
        )
        .await
        .expect("permission regression");
    assert_ne!(moved["isError"], true, "{moved}");
    if !commits {
        server
            .session
            .as_ref()
            .expect("permission regression")
            .lock()
            .expect("permission regression")
            .explicit_commit("accepted changes")
            .expect("permission regression");
    }
    assert!(io::calls().is_empty());
    assert_eq!(native_mode("Model.md"), 0o777);
    assert!(git_test(&["ls-tree", "HEAD", "--", "Model.md"]).starts_with("100644"));
    assert!(git_test(&["ls-tree", "HEAD", "--", "moved.sh"]).starts_with("100755"));
    assert_eq!(git_test(&["status", "--porcelain"]), "");
}
#[cfg(unix)]
case!(permission_ignored_filemode_without_commits, {
    ignored_filemode_keeps_logical_mode(false).await;
});
#[cfg(unix)]
case!(permission_ignored_filemode_with_commits, {
    ignored_filemode_keeps_logical_mode(true).await;
});

#[cfg(unix)]
async fn recovery_snapshot_reads(commits: bool) {
    use crate::mcp_session::permission_test_io as io;
    evidence_fixture();
    std::fs::write(
        "Ontology.md",
        include_str!("../../../tests/test-cache-integration/fixtures/ontology.md.txt"),
    )
    .expect("ontology fixture");
    std::fs::write(
        "external.ttl",
        include_str!("../../../tests/test-cache-integration/fixtures/external.ttl"),
    )
    .expect("external fixture");
    git_test(&["add", "."]);
    git_test(&["commit", "-qm", "semantic inputs"]);
    let (server, refreshes, _) = server_with_commits(false, commits);
    let before = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
            false,
        )
        .await
        .expect("baseline");
    let revision = server
        .session
        .as_ref()
        .expect("session")
        .lock()
        .expect("lock")
        .status()["model_revision"]
        .clone();
    let head = git_test(&["rev-parse", "HEAD"]);
    // Source deletion succeeds, destination chmod fails, source mode restoration fails.
    io::configure(&["z-moved.sh", "evidence.txt"], &[]);
    let failed = server.call_handler("tools/call", json!({"name":"reqvire.move_asset","arguments":{"old_path":"evidence.txt","new_path":"z-moved.sh"}}), true).await.expect("failure envelope");
    assert!(failed.to_string().contains("recovery failed"), "{failed}");
    io::configure(&[], &[]);
    let status = server
        .session
        .as_ref()
        .expect("session")
        .lock()
        .expect("lock")
        .status();
    assert_eq!(
        status["available"], true,
        "last accepted snapshot remains usable: {status}"
    );
    assert_eq!(status["writes_available"], false);
    assert_eq!(status["recovery_required"], true);
    assert!(status["diagnostic"]
        .as_str()
        .is_some_and(|s| s.contains("evidence.txt")));
    std::fs::write("Model.md", "external invalid model").expect("external edit");
    std::fs::write("external.ttl", "external invalid turtle").expect("external edit");
    let read = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
            false,
        )
        .await
        .expect("snapshot read");
    assert_ne!(read["isError"], true, "{read}");
    assert_eq!(
        read["structuredContent"]["files"],
        before["structuredContent"]["files"]
    );
    let meta = &read["_meta"]["reqvire/context"];
    assert_eq!(meta["recovery_required"], true);
    assert_eq!(meta["model_revision"], revision);
    assert_eq!(meta["head"], head);
    for name in [
        "reqvire.workspace_status",
        "reqvire.model_revision",
        "reqvire.coverage",
        "reqvire.traces",
        "reqvire.lint",
        "reqvire.resources",
        "reqvire.semantic.export",
    ] {
        let result = server
            .call_handler("tools/call", json!({"name":name,"arguments":{}}), false)
            .await
            .expect("snapshot read");
        assert_ne!(result["isError"], true, "{name}: {result}");
        assert_eq!(
            result["_meta"]["reqvire/context"]["model_revision"],
            revision
        );
    }
    // First SPARQL initialization occurs after failure and external dependency corruption.
    let query = server.call_handler("tools/call", json!({"name":"reqvire.semantic.sparql","arguments":{
        "include_external":true,"query":"ASK { <https://example.test/external#ExternalResource> <http://www.w3.org/2000/01/rdf-schema#label> \"Label alpha\" }"
    }}), false).await.expect("lazy snapshot query");
    assert_eq!(query["structuredContent"]["boolean"], true, "{query}");
    let resource = server
        .call_handler(
            "resources/read",
            json!({"uri":"reqvire://workspace/status"}),
            false,
        )
        .await
        .expect("resource read");
    assert_ne!(resource["isError"], true, "{resource}");
    assert_eq!(
        resource["_meta"]["reqvire/context"]["recovery_required"],
        true
    );
    // Exercise the RMCP conversion and actual HTTP response serialization, not
    // just call_handler: ReadResourceResult can discard extension metadata.
    {
        use axum::{body::{to_bytes, Body}, http::Request};
        use tower::ServiceExt;
        let app = mount_server(axum::Router::new(), server.clone(), &HttpAccess::new(&[], &[]));
        let request = Request::builder().method("POST").uri("/mcp")
            .header("host", "127.0.0.1:8081")
            .header("content-type", "application/json")
            .header("accept", "application/json, text/event-stream")
            .header("mcp-protocol-version", "2025-11-25")
            .body(Body::from(json!({"jsonrpc":"2.0", "id":42,
                "method":"resources/read", "params":{"uri":"reqvire://workspace/status"}}).to_string()))
            .expect("HTTP request");
        let response = app.oneshot(request).await.expect("HTTP router");
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let bytes = to_bytes(response.into_body(), usize::MAX).await.expect("HTTP body");
        let wire: Value = serde_json::from_slice(&bytes).expect("JSON-RPC response");
        assert_eq!(wire["result"]["contents"], resource["contents"]);
        assert_eq!(wire["result"]["_meta"], resource["_meta"], "metadata lost on HTTP wire: {wire}");
    }
    for params in [
        add(OTHER, false),
        add(OTHER, true),
        json!({"name":"reqvire.format","arguments":{"fix":false}}),
        json!({"name":"reqvire.change_impact","arguments":{}}),
    ] {
        let rejected = server
            .call_handler("tools/call", params, true)
            .await
            .expect("rejection envelope");
        assert_eq!(rejected["isError"], true, "{rejected}");
        assert!(rejected.to_string().contains("recovery"));
    }
    assert!(server
        .session
        .as_ref()
        .expect("session")
        .lock()
        .expect("lock")
        .explicit_commit("forbidden")
        .is_err());
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("disk"),
        "external invalid model"
    );
    assert_eq!(refreshes.load(Ordering::SeqCst), 0);
    assert!(server
        .call_handler("tools/list", json!({}), false)
        .await
        .expect("discovery")["tools"]
        .is_array());
    std::fs::rename(".git", ".saved-git").expect("make Git observations unavailable");
    let status = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.workspace_status","arguments":{}}),
            false,
        )
        .await
        .expect("status without live Git");
    assert_ne!(status["isError"], true, "{status}");
    assert!(
        status["structuredContent"]["git"]["dirty"].is_null(),
        "unavailable Git must not report clean: {status}"
    );
    assert_eq!(
        status["_meta"]["reqvire/context"]["model_revision"],
        revision
    );
    std::fs::rename(".saved-git", ".git").expect("restore Git fixture");
}
#[cfg(unix)]
case!(recovery_preserves_snapshot_reads_without_commits, {
    recovery_snapshot_reads(false).await;
});
#[cfg(unix)]
case!(recovery_preserves_snapshot_reads_with_commits, {
    recovery_snapshot_reads(true).await;
});

fn read_only_server() -> ReqvireMcpServer {
    let mut server = ReqvireMcpServer::new_with_write_lock(
        false,
        false,
        false,
        &reqvire::exclusions::ExclusionSetBuilder::new()
            .build()
            .unwrap(),
        Arc::new(RwLock::new(())),
        None,
    )
    .unwrap();
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
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let wire: Value = serde_json::from_slice(&bytes).unwrap();
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
        let hook_barrier = barrier.clone();
        let hook_started = started.clone();
        server.before_dispatch = Some(Arc::new(move |_, _| {
            entered_tx.send(std::thread::current().id()).unwrap();
            hook_started.notify_one();
            let (lock, ready) = &*hook_barrier;
            let (released, timeout) = ready
                .wait_timeout_while(lock.lock().unwrap(), Duration::from_secs(5), |released| {
                    !*released
                })
                .unwrap();
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
            *lock.lock().unwrap() = true;
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
            let wire = result.unwrap();
            assert_ne!(wire["result"]["isError"], true, "{wire}");
            assert!(wire["result"]["structuredContent"]
                .to_string()
                .contains("Cache Subject"));
        }
        ping.await.unwrap();
        let (overlap, live, _receiver) = coordinator.join().unwrap();
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
        values.push(value.unwrap()["result"].clone());
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

    std::fs::write("Model.md", MODEL.replace("alpha", "omega")).unwrap();
    let params = json!({"name":"reqvire.read_element","arguments":{"name":"Cache Subject"}});
    let fresh = read_wave(app.clone(), "tools/call", params.clone()).await;
    assert!(fresh.iter().all(|v| v["structuredContent"]["content"]
        .as_str()
        .unwrap()
        .contains("omega")));
    std::fs::write(
        "Model.md",
        format!("{MODEL}\n### Invalid\n\n#### Metadata\n  * type: unsupported-type\n"),
    )
    .unwrap();
    let failed = read_wave(app.clone(), "tools/call", params.clone()).await;
    assert!(failed.iter().all(|v| v["isError"] == true), "{failed:?}");
    std::fs::write("Model.md", MODEL).unwrap();
    let recovered = read_wave(app, "tools/call", params).await;
    assert!(recovered.iter().all(|v| v["structuredContent"]["content"]
        .as_str()
        .unwrap()
        .contains("alpha")));
});

case!(read_only_capacity_cancellation_and_writer_priority, {
    use std::sync::{Condvar, Mutex as StdMutex};
    let mut server = read_only_server();
    server.read_capacity = Arc::new(Semaphore::new(2));
    let barrier = Arc::new((StdMutex::new(false), Condvar::new()));
    let (entered_tx, mut entered_rx) = tokio::sync::mpsc::unbounded_channel();
    let hook_barrier = barrier.clone();
    server.before_dispatch = Some(Arc::new(move |_, _| {
        let _ = entered_tx.send(());
        let (lock, ready) = &*hook_barrier;
        let (released, _) = ready
            .wait_timeout_while(lock.lock().unwrap(), Duration::from_secs(5), |released| {
                !*released
            })
            .unwrap();
        assert!(*released, "test must release dispatch barrier");
    }));
    let request = json!({"name":"reqvire.search","arguments":{}});
    let mut readers = Vec::new();
    for _ in 0..2 {
        let server = server.clone();
        let request = request.clone();
        readers.push(tokio::spawn(async move {
            server.call_handler("tools/call", request, false).await
        }));
    }
    for _ in 0..2 {
        tokio::time::timeout(Duration::from_secs(2), entered_rx.recv())
            .await
            .unwrap()
            .unwrap();
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
    let cancelled = readers.pop().unwrap();
    cancelled.abort();
    assert!(cancelled.await.unwrap_err().is_cancelled());
    assert_eq!(
        server.read_capacity.available_permits(),
        0,
        "running cancellation released capacity early"
    );
    let mut writer = Box::pin(server.write_lock.clone().write_owned());
    assert!(
        writer
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending(),
        "running cancellation released shared workspace gate early"
    );
    {
        let (lock, ready) = &*barrier;
        *lock.lock().unwrap() = true;
        ready.notify_all();
    }
    assert_ne!(
        readers.pop().unwrap().await.unwrap().unwrap()["isError"],
        true
    );
    tokio::time::timeout(Duration::from_secs(2), async {
        while server.read_capacity.available_permits() != 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    let mut late_read = Box::pin(server.call_handler("tools/call", request.clone(), false));
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
    std::fs::write("Model.md", "partially persisted invalid model").unwrap();
    let mut after = Box::pin(server.call_handler("tools/call", request, false));
    assert!(after
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
        .is_pending());
    std::fs::write("Model.md", MODEL.replace("Cache Subject", "Final Subject")).unwrap();
    reqvire::model_cache::invalidate();
    drop(gate);
    let result = after.await.unwrap();
    assert_ne!(result["isError"], true, "{result}");
    assert!(result.to_string().contains("Final Subject"));
    assert_eq!(server.read_capacity.available_permits(), 2);
});

case!(read_only_failure_and_panic_release_dispatch_guards, {
    let mut server = read_only_server();
    server.read_capacity = Arc::new(Semaphore::new(1));
    let denied = server
        .call_handler("tools/call", add(OTHER, false), true)
        .await
        .unwrap_err();
    assert_eq!(denied.code, ErrorCode(-32602));
    assert_eq!(server.read_capacity.available_permits(), 1);
    assert_eq!(std::fs::read_to_string("Model.md").unwrap(), MODEL);
    let missing = server
        .call_handler(
            "tools/call",
            json!({"name":"reqvire.read_element","arguments":{"name":"Missing"}}),
            false,
        )
        .await
        .unwrap();
    assert_eq!(missing["isError"], true);
    assert_eq!(server.read_capacity.available_permits(), 1);
    assert!(server
        .call_handler("unknown", json!({}), false)
        .await
        .is_err());
    assert_eq!(server.read_capacity.available_permits(), 1);
    server.before_dispatch = Some(Arc::new(|_, _| panic!("injected blocking read panic")));
    let failure = server
        .call_handler("tools/list", json!({}), false)
        .await
        .unwrap_err();
    assert!(failure.message.contains("MCP read failed"), "{failure}");
    assert_eq!(server.read_capacity.available_permits(), 1);
    assert!(server.write_lock.try_write().is_ok());
    server.before_dispatch = None;
    let result = server
        .call_handler("tools/list", json!({}), false)
        .await
        .unwrap();
    assert!(result["tools"].is_array());
});

async fn owned_http_selector_error_envelopes(commits: bool) {
    let executable = std::env::var_os("REQVIRE_TEST_BIN")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join(format!("reqvire{}", std::env::consts::EXE_SUFFIX))
        });
    let worktrees = crate::mcp_worktrees::Worktrees::start(
        &std::env::current_dir().unwrap(),
        &executable,
        commits,
        false,
        true,
        false,
        "origin",
    )
    .unwrap();
    let original = worktrees.browser_context(None).unwrap().metadata();
    let exclusions = reqvire::exclusions::ExclusionSetBuilder::new()
        .build()
        .unwrap();
    // This is the same HTTP mount used by standalone MCP and embedded Serve.
    let app = mount_worktrees(
        axum::Router::new(),
        worktrees,
        &exclusions,
        &HttpAccess::new(&[], &[]),
    )
    .unwrap();
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
    let root = std::env::current_dir().unwrap();
    let child_root = std::path::PathBuf::from(child["workspace_root"].as_str().unwrap());
    let git_state = |path: &std::path::Path| {
        [vec!["rev-parse", "HEAD"], vec!["status", "--porcelain=v1"]].map(|args| {
            let output = Command::new("git")
                .current_dir(path)
                .args(args)
                .output()
                .unwrap();
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
                .unwrap()
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
                .unwrap()
                .contains(id.as_str().unwrap()));
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
    assert_eq!(std::fs::read_to_string("Model.md").unwrap(), MODEL);
    assert_eq!(
        std::fs::read_to_string(child_root.join("Model.md")).unwrap(),
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
        child["worktree_id"].as_str().unwrap(),
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
    let barrier = tempfile::tempdir().unwrap();
    let real_git = Command::new("sh")
        .args(["-c", "command -v git"])
        .output()
        .unwrap();
    let real_git = String::from_utf8(real_git.stdout)
        .unwrap()
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
    end=time.monotonic()+10
    while not (base/'release').exists():
        if time.monotonic()>end: sys.exit(99)
        time.sleep(0.005)
os.execv({git},[{git},*sys.argv[1:]])
"#,
            base = json!(barrier.path()),
            git = json!(real_git)
        ),
    )
    .unwrap();
    std::fs::set_permissions(&shim, std::fs::Permissions::from_mode(0o755)).unwrap();
    // Each case runs in its own process, so this cannot change another test's PATH.
    std::env::set_var(
        "PATH",
        format!(
            "{}:{}",
            barrier.path().display(),
            std::env::var("PATH").unwrap()
        ),
    );
    let executable = std::env::var_os("REQVIRE_TEST_BIN")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("reqvire")
        });
    let worktrees = crate::mcp_worktrees::Worktrees::start(
        &std::env::current_dir().unwrap(),
        &executable,
        commits,
        false,
        true,
        false,
        "origin",
    )
    .unwrap();
    let original = worktrees.browser_context(None).unwrap();
    let before = original.metadata();
    let mut server = read_only_server();
    server.enable_mutations = true;
    server.worktrees = Some(worktrees.clone());
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
        std::fs::write(barrier.path().join("armed"), "").unwrap();
    };
    let release = || std::fs::write(barrier.path().join("release"), "").unwrap();
    async fn entered(base: &std::path::Path, stage: &str) {
        tokio::time::timeout(Duration::from_secs(3), async {
            while !base.join("entered").exists() {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .unwrap_or_else(|_| panic!("worker did not enter held read: {stage}"));
        // The held Git child still waits, but later publication may observe Git.
        std::fs::remove_file(base.join("armed")).unwrap();
    }
    async fn response(job: tokio::task::JoinHandle<Value>) -> Value {
        tokio::time::timeout(Duration::from_secs(3), job)
            .await
            .expect("worker response timed out")
            .unwrap()
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
    let written = response(tokio::spawn(http_rpc(
        app.clone(),
        3,
        "tools/call",
        add(OTHER, false),
    )))
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
    let runtime = original.runtime().unwrap();
    release();
    let old = response(old).await;
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
    assert!(Arc::ptr_eq(&runtime, &original.runtime().unwrap()));

    // The asynchronous caller can disappear; the actual dispatched read still
    // owns admission until it finishes. Control operations retain their lane.
    arm();
    let cancelled_server = server.clone();
    let mut cancelled = tokio::spawn(async move {
        cancelled_server
            .call_handler(
                "tools/call",
                json!({"name":"reqvire.workspace_status"}),
                false,
            )
            .await
    });
    tokio::select! {
        early = &mut cancelled => panic!("Cancelled read returned before barrier: {early:?}"),
        _ = entered(barrier.path(), "cancellation") => {}
    }
    cancelled.abort();
    let _ = cancelled.await;
    assert_eq!(server.read_capacity.available_permits(), 0);
    let preview = response(tokio::spawn(http_rpc(app.clone(), 4, "tools/call",
        json!({"name":"reqvire.remove_element", "arguments":{"element_name":"Other Subject","dry_run":true}})))).await;
    assert!(
        preview.get("error").is_none() && preview["result"]["isError"] != true,
        "{preview}"
    );
    release();
    tokio::time::timeout(Duration::from_secs(3), async {
        while server.read_capacity.available_permits() != 1 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
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
    let secondary = worktrees.browser_context(Some(id)).unwrap();
    arm();
    let pending = tokio::spawn(http_rpc(
        app.clone(),
        7,
        "tools/call",
        json!({"name":"reqvire.workspace_status","arguments":{"worktree_id":id}}),
    ));
    entered(barrier.path(), "removal").await;
    let removed = response(tokio::spawn(http_rpc(
        app.clone(),
        8,
        "tools/call",
        json!({"name":"reqvire.worktree.remove","arguments":{"worktree_id":id}}),
    )))
    .await;
    assert!(
        removed.get("error").is_none() && removed["result"]["isError"] != true,
        "{removed}"
    );
    let pending = response(pending).await;
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
        .unwrap()
        .success());
    let reopened = response(tokio::spawn(http_rpc(
        app.clone(),
        10,
        "tools/call",
        json!({"name":"reqvire.worktree.open", "arguments":{"branch":before["branch"]}}),
    )))
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
    let pending = response(pending).await;
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
