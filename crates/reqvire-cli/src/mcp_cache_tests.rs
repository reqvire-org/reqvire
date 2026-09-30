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
    let calls = Arc::new(AtomicUsize::new(0));
    // Simulate an already outstanding refresh diagnostic. Any hook invocation
    // would clear it on success, so we can observe an incorrect gate directly.
    let diagnostic = Arc::new(AtomicBool::new(true));
    let hook_calls = Arc::clone(&calls);
    let hook_diagnostic = Arc::clone(&diagnostic);
    let hook: PostWriteHook = Arc::new(move || {
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
            false,
            &reqvire::exclusions::ExclusionSetBuilder::new()
                .build()
                .expect("build test configuration"),
            Arc::new(Mutex::new(())),
            Some(hook),
        ),
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
    let (server, _, _) = server(false);
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
