//! MutationSession unit tests: captured state, rollback and injected I/O failures.
//! HTTP routing and runtime publication are verified through production workers.
use super::*;
use serde_json::json;
use std::process::Command;
use std::time::{Duration, Instant};

const MODEL: &str = include_str!("../../../tests/test-cache-integration/fixtures/model.md.txt");
const OTHER: &str = include_str!("../../../tests/test-cache-integration/fixtures/other.md.txt");

fn isolated(name: &str, test: impl FnOnce()) {
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
        test();
        return;
    }
    let mut child = Command::new(std::env::current_exe().expect("locate test executable"))
        .args([
            "--exact",
            &format!("mcp_session::tests::{name}"),
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
        fn $name() {
            isolated(stringify!($name), || $body);
        }
    };
}

struct SessionFixture {
    session: std::sync::Mutex<MutationSession>,
    exclusions: reqvire::exclusions::ExclusionSet,
}
fn fixture(commits: bool) -> SessionFixture {
    let exclusions = reqvire::exclusions::ExclusionSetBuilder::new()
        .build()
        .expect("test exclusions");
    SessionFixture {
        session: std::sync::Mutex::new(
            MutationSession::start(&exclusions, false, commits).expect("unit session"),
        ),
        exclusions,
    }
}
impl SessionFixture {
    fn dispatch(&self, method: &str, params: Value) -> Result<Value, rmcp::ErrorData> {
        let persists =
            method == "tools/call" && crate::mcp::request_refreshes_runtime_after_write(&params);
        let tool = params["name"].as_str().unwrap_or(method).to_owned();
        self.session
            .lock()
            .expect("unit session lock")
            .execute(persists, &tool, || {
                crate::mcp_worker::dispatch_rpc(
                    &json!({"method":method,"params":params}),
                    false,
                    &self.exclusions,
                )
            })
            .0
    }
}
fn add(content: &str, dry_run: bool) -> Value {
    json!({"name": "reqvire.add_element", "arguments": {
        "file": "Model.md", "content": content.split_once("# Elements\n\n").expect("expected fixture to contain an Elements header").1, "dry_run": dry_run,
    }})
}

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
        (
            result.expect("test fixture operation should succeed"),
            changed,
        )
    }
    fn pending(session: &MutationSession) -> Value {
        session.status()["pending_changes"].clone()
    }

    std::fs::write("asset.txt", "asset").expect("write test fixture");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::write("mode-source.txt", MODEL).expect("write test fixture");
        std::fs::write("mode-plain.txt", MODEL).expect("write test fixture");
        std::fs::set_permissions("mode-source.txt", std::fs::Permissions::from_mode(0o755))
            .expect("test fixture operation should succeed");
    }
    git_test(&["add", "."]);
    git_test(&["commit", "-qm", "snapshot assets"]);
    let initial_head = git_test(&["rev-parse", "HEAD"]);
    let server = fixture(commits);
    let mut session = server
        .session
        .lock()
        .expect("test lock should not be poisoned");
    let first = session.model();
    let again = session.model();
    assert!(Arc::ptr_eq(&first, &again));
    std::fs::write("unrelated.txt", "keep staged").expect("write test fixture");
    git_test(&["add", "unrelated.txt"]);
    let unrelated = git_test(&["ls-files", "--stage", "unrelated.txt"]);
    // status() may refresh Git's stat cache; tracked entries must stay intact.
    let index = git_test(&["ls-files", "--stage"]);
    let changed_source = MODEL.replace("alpha", "bravo");
    let (result, changed) = prepared(&mut session, true, false, || {
        mutation_io::write("Model.md", &changed_source)
            .expect("test fixture operation should succeed");
    });
    assert_ne!(result["isError"], true, "{result}");
    assert!(changed);
    let accepted = session.model();
    assert!(!Arc::ptr_eq(&first, &accepted));
    assert!(first
        .graph_registry
        .get_element_by_name("Cache Subject")
        .expect("test fixture operation should succeed")
        .content
        .contains("alpha"));
    assert!(accepted
        .graph_registry
        .get_element_by_name("Cache Subject")
        .expect("test fixture operation should succeed")
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
            mutation_io::write("Model.md", MODEL).expect("test fixture operation should succeed");
        });
        assert!(!published);
        assert!(Arc::ptr_eq(&session.model(), &accepted));
        assert_eq!(pending(&session), expected);
        assert_eq!(
            std::fs::read_to_string("Model.md").expect("read test fixture"),
            changed_source
        );
    }
    let (failure, changed) = prepared(&mut session, true, false, || {
        mutation_io::write(
            "Model.md",
            "# Elements\n\n### Invalid\n\n#### Metadata\n  * type: unsupported-type\n",
        )
        .expect("test fixture operation should succeed");
    });
    assert_eq!(failure["isError"], true, "{failure}");
    assert!(!changed);
    assert!(Arc::ptr_eq(&session.model(), &accepted));
    assert_eq!(pending(&session), expected);

    // Revert accepted bytes without importing an intervening external edit.
    std::fs::write("Model.md", "outside accepted snapshot").expect("write test fixture");
    let (_, changed) = prepared(&mut session, true, false, || {
        assert_eq!(
            mutation_io::read_to_string("Model.md").expect("test fixture operation should succeed"),
            changed_source
        );
        mutation_io::write("Model.md", MODEL).expect("test fixture operation should succeed");
    });
    assert!(changed);
    assert_eq!(pending(&session), json!([]));
    assert!(accepted
        .graph_registry
        .get_element_by_name("Cache Subject")
        .expect("test fixture operation should succeed")
        .content
        .contains("bravo"));
    #[cfg(unix)]
    {
        let (_, changed) = prepared(&mut session, true, false, || {
            mutation_io::copy("mode-source.txt", "Model.md")
                .expect("test fixture operation should succeed");
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
            mutation_io::copy("mode-plain.txt", "Model.md")
                .expect("test fixture operation should succeed");
        });
        assert!(changed);
        assert_eq!(pending(&session), json!([]));
    }
    let (_, changed) = prepared(&mut session, true, false, || {
        mutation_io::rename("asset.txt", "moved.txt")
            .expect("test fixture operation should succeed");
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
        mutation_io::rename("moved.txt", "asset.txt")
            .expect("test fixture operation should succeed");
    });
    assert!(changed);
    assert_eq!(pending(&session), json!([]));
    let (_, changed) = prepared(&mut session, true, false, || {
        mutation_io::remove_file("asset.txt").expect("test fixture operation should succeed");
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
        mutation_io::write("asset.txt", b"asset").expect("test fixture operation should succeed");
    });
    assert!(changed);
    assert_eq!(pending(&session), json!([]));
    if !commits {
        assert_eq!(git_test(&["rev-parse", "HEAD"]), initial_head);
        assert_eq!(git_test(&["ls-files", "--stage"]), index);
    }
    let (_, changed) = prepared(&mut session, true, false, || {
        mutation_io::write("Model.md", &changed_source)
            .expect("test fixture operation should succeed");
    });
    assert!(changed);
    let result = session
        .explicit_commit("commit accepted changes")
        .expect("test fixture operation should succeed");
    assert_eq!(
        result["outcome"],
        if commits { "no_op" } else { "completed" }
    );
    assert_eq!(pending(&session), json!([]));
    drop(session);
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

#[cfg(unix)]
#[allow(clippy::unwrap_used, clippy::significant_drop_tightening)]
fn reconcile_published_fixture_commit(head: &str, paths: &[&str]) {
    // Operator repair is selective: finish the verified commit's index
    // publication, without another commit, retry, or whole-index reset.
    std::fs::remove_file(".git/hooks/reference-transaction").unwrap();
    std::fs::remove_file("ref-effects").unwrap(); // fixture-only hook evidence
    let mut args = vec!["restore", "--staged", "--source", head, "--"];
    args.extend_from_slice(paths);
    git_test(&args);
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(git_test(&["status", "--porcelain"]), "");
}

#[cfg(unix)]
case!(
    explicit_commit_timeout_with_external_files_retains_snapshot_reads,
    {
        use std::os::unix::fs::PermissionsExt;
        let server = fixture(false);
        let mut session = server
            .session
            .lock()
            .expect("test lock should not be poisoned");
        let before = session.status();
        let (result, changed) = session.execute(true, "reqvire.add_element", || {
            reqvire::mutation_io::write("asset.bin", [0, 255, 128])
                .expect("test fixture operation should succeed");
            Ok(json!({"structuredContent":{}}))
        });
        assert_ne!(
            result.expect("test fixture operation should succeed")["isError"],
            true
        );
        assert!(changed);
        let accepted = session.model();
        let pending_before = session.status()["pending_changes"].clone();
        let index_before = std::fs::read(".git/index").expect("read test fixture");
        std::fs::write(".git/hooks/reference-transaction", "#!/usr/bin/env python3\nimport sys, time\nfrom pathlib import Path\nif sys.argv[1] == 'committed':\n    with Path('ref-effects').open('a') as f: f.write('one\\n')\n    time.sleep(1)\n").expect("write test fixture");
        std::fs::set_permissions(
            ".git/hooks/reference-transaction",
            std::fs::Permissions::from_mode(0o755),
        )
        .expect("test fixture operation should succeed");
        let failure = crate::mcp_session::with_git_timeout(Duration::from_millis(250), || {
            session.explicit_commit("ambiguous commit")
        })
        .expect_err("test fixture operation should fail")
        .to_string();
        assert!(failure.contains("outcome may be unknown"), "{failure}");
        assert_ne!(git_test(&["rev-parse", "HEAD"]), before["head"]);
        let status = session.status();
        assert_eq!(status["head"], before["head"]);
        assert_eq!(status["available"], true);
        assert_eq!(status["writes_available"], false);
        assert_eq!(status["recovery_required"], true);
        assert!(Arc::ptr_eq(&accepted, &session.model()));
        assert_eq!(status["pending_changes"], pending_before);
        assert_eq!(
            std::fs::read(".git/index").expect("read test fixture"),
            index_before
        );
        assert_eq!(
            std::fs::read("asset.bin").expect("read test fixture"),
            [0, 255, 128]
        );
        assert!(!std::path::Path::new(".git/index.lock").exists());
        assert!(session.capture_read("reqvire.search").is_ok());
        assert!(session
            .explicit_commit("must not retry")
            .expect_err("test fixture operation should fail")
            .to_string()
            .contains("recovery"));
        assert_eq!(
            std::fs::read_to_string("ref-effects").expect("read test fixture"),
            "one\n"
        );
        let published_head = git_test(&["rev-parse", "HEAD"]);
        assert!(failure.contains(&format!("attempted commit {published_head}")));
        assert!(failure.contains(&format!(
            "accepted HEAD {}",
            before["head"]
                .as_str()
                .expect("expected a string in the test response")
        )));
        assert_eq!(git_test(&["rev-parse", "HEAD^"]), before["head"]);
        assert_eq!(
            crate::mcp_session::git(
                std::path::Path::new("."),
                &["show", "HEAD:asset.bin"],
                None,
                None
            )
            .expect("fixture Git command should succeed"),
            [0, 255, 128]
        );
        drop(session);
        drop(server);
        reconcile_published_fixture_commit(&published_head, &["asset.bin"]);
        let reopened = fixture(false);
        let repaired = reopened
            .session
            .lock()
            .expect("test lock should not be poisoned");
        assert_eq!(repaired.status()["head"], published_head);
        assert_eq!(repaired.status()["recovery_required"], false);
        assert_eq!(repaired.status()["writes_available"], true);
        assert_eq!(repaired.status()["pending_changes"], json!([]));
        let read = repaired
            .capture_read("reqvire.read_element")
            .expect("test fixture operation should succeed");
        drop(repaired);
        assert_eq!(
            read.execute(|| Ok(json!(reqvire::mutation_io::read("asset.bin")
                .expect("test fixture operation should succeed"))))
                .expect("test fixture operation should succeed"),
            json!([0, 255, 128])
        );
    }
);

#[cfg(unix)]
case!(
    automatic_commit_timeout_with_external_files_requires_recovery,
    {
        use std::os::unix::fs::PermissionsExt;
        let server = fixture(true);
        let mut session = server
            .session
            .lock()
            .expect("test lock should not be poisoned");
        let before = session.status();
        let accepted = session.model();
        let index_before = std::fs::read(".git/index").expect("read test fixture");
        std::fs::write(".git/hooks/reference-transaction", "#!/usr/bin/env python3\nimport sys, time\nfrom pathlib import Path\nif sys.argv[1] == 'committed':\n    with Path('ref-effects').open('a') as f: f.write('one\\n')\n    time.sleep(1)\n").expect("write test fixture");
        std::fs::set_permissions(
            ".git/hooks/reference-transaction",
            std::fs::Permissions::from_mode(0o755),
        )
        .expect("test fixture operation should succeed");
        let changed_source = MODEL.replace("alpha", "candidate");
        let (result, changed) =
            crate::mcp_session::with_git_timeout(Duration::from_millis(250), || {
                session.execute(true, "reqvire.add_element", || {
                    reqvire::mutation_io::write("Model.md", changed_source.as_bytes())
                        .expect("test fixture operation should succeed");
                    Ok(json!({"structuredContent":{}}))
                })
            });
        let failure = result.expect("test fixture operation should succeed");
        assert_eq!(failure["isError"], true, "{failure}");
        assert!(!changed);
        assert!(
            failure.to_string().contains("outcome may be unknown"),
            "{failure}"
        );
        assert_ne!(git_test(&["rev-parse", "HEAD"]), before["head"]);
        let status = session.status();
        assert_eq!(status["head"], before["head"]);
        assert_eq!(status["available"], true);
        assert_eq!(status["writes_available"], false);
        assert_eq!(status["recovery_required"], true);
        assert_eq!(status["model_revision"], before["model_revision"]);
        assert_eq!(status["pending_changes"], before["pending_changes"]);
        assert!(Arc::ptr_eq(&accepted, &session.model()));
        assert_eq!(
            std::fs::read(".git/index").expect("read test fixture"),
            index_before
        );
        assert!(!std::path::Path::new(".git/index.lock").exists());
        assert_eq!(
            std::fs::read_to_string("Model.md").expect("read test fixture"),
            changed_source
        );
        let snapshot = session
            .capture_read("reqvire.read_element")
            .expect("test fixture operation should succeed");
        let captured = snapshot
            .execute(|| {
                Ok(json!(reqvire::mutation_io::read_to_string("Model.md")
                    .expect("test fixture operation should succeed")))
            })
            .expect("test fixture operation should succeed");
        assert_eq!(captured, MODEL);
        assert!(session
            .explicit_commit("must not retry")
            .expect_err("test fixture operation should fail")
            .to_string()
            .contains("recovery"));
        assert_eq!(
            std::fs::read_to_string("ref-effects").expect("read test fixture"),
            "one\n"
        );
        let published_head = git_test(&["rev-parse", "HEAD"]);
        assert!(failure
            .to_string()
            .contains(&format!("attempted commit {published_head}")));
        assert!(failure.to_string().contains(&format!(
            "accepted HEAD {}",
            before["head"]
                .as_str()
                .expect("expected a string in the test response")
        )));
        assert_eq!(git_test(&["rev-parse", "HEAD^"]), before["head"]);
        assert_eq!(
            crate::mcp_session::git(
                std::path::Path::new("."),
                &["show", "HEAD:Model.md"],
                None,
                None
            )
            .expect("fixture Git command should succeed"),
            changed_source.as_bytes()
        );
        drop(session);
        drop(server);
        reconcile_published_fixture_commit(&published_head, &["Model.md"]);
        let reopened = fixture(true);
        let repaired = reopened
            .session
            .lock()
            .expect("test lock should not be poisoned");
        assert_eq!(repaired.status()["head"], published_head);
        assert_eq!(repaired.status()["recovery_required"], false);
        assert_eq!(repaired.status()["writes_available"], true);
        let read = repaired
            .capture_read("reqvire.read_element")
            .expect("test fixture operation should succeed");
        drop(repaired);
        assert_eq!(
            read.execute(|| Ok(json!(reqvire::mutation_io::read_to_string("Model.md")
                .expect("test fixture operation should succeed"))))
                .expect("test fixture operation should succeed"),
            changed_source
        );
    }
);

#[cfg(unix)]
fn reconcile_unknown_commit(commits: bool, blocked: bool) {
    use crate::mcp_session::{git, with_git_timeout};
    use std::os::unix::fs::PermissionsExt;
    let root = std::path::Path::new(".");
    std::fs::write("obsolete.bin", [1, 2]).expect("write deleted fixture asset");
    std::fs::write("unrelated.bin", [3, 4]).expect("write unrelated fixture asset");
    // Git tracks the owner-execute bit. Preserve other native execute bits
    // without interpreting them as a different committed executable mode.
    std::fs::set_permissions("unrelated.bin", std::fs::Permissions::from_mode(0o654))
        .expect("group-executable fixture permissions");
    git_test(&["add", "obsolete.bin", "unrelated.bin"]);
    git_test(&["commit", "-qm", "asset baseline"]);
    let server = fixture(commits);
    let mut session = server.session.lock().expect("session lock");
    let captured = session
        .capture_read("reqvire.read_element")
        .expect("capture baseline read");
    let initial = session.status();
    // Stage unrelated bytes without changing accepted physical content. This
    // pre-existing index entry must survive reconciliation exactly.
    let unrelated_oid = String::from_utf8(
        git(
            root,
            &["hash-object", "-w", "--stdin"],
            Some(b"staged only"),
            None,
        )
        .expect("hash staged fixture"),
    )
    .expect("Git object ID");
    git_test(&[
        "update-index",
        "--cacheinfo",
        &format!("100644,{},unrelated.bin", unrelated_oid.trim()),
    ]);
    let unrelated = git_test(&["ls-files", "--stage", "unrelated.bin"]);
    let index_before = std::fs::read(".git/index").expect("index before publication");
    let changed_source = MODEL.replace("Cache Subject", "Recovered Subject");
    let asset = "odd\n[asset]*.bin";
    let publish = |session: &mut crate::mcp_session::MutationSession| {
        session.execute(true, "reqvire.add_element", || {
            reqvire::mutation_io::write("Model.md", &changed_source)
                .expect("prepare candidate source");
            reqvire::mutation_io::write(asset, [0, 255, 128]).expect("prepare binary asset");
            reqvire::mutation_io::remove_file("obsolete.bin").expect("prepare asset deletion");
            Ok(json!({"structuredContent":{}}))
        })
    };
    if !commits {
        let (result, changed) = publish(&mut session);
        assert_ne!(result.expect("accepted mutation")["isError"], true);
        assert!(changed);
    }
    let before = session.status();
    let accepted = session.model();
    std::fs::write(".git/hooks/reference-transaction", "#!/usr/bin/env python3\nimport sys,time\nfrom pathlib import Path\nif sys.argv[1]=='committed':\n    with Path('.git/ref-effects').open('a') as f: f.write('one\\n')\n    time.sleep(1)\n").expect("install post-publication stall");
    std::fs::set_permissions(
        ".git/hooks/reference-transaction",
        std::fs::Permissions::from_mode(0o755),
    )
    .expect("executable hook");
    if blocked {
        // An unrelated file prevents automatic adoption. Resolve this evidence
        // afterward to exercise the explicit recovery tool's verification.
        std::fs::write("reconciliation-blocker", "external work")
            .expect("block automatic reconciliation");
    }
    with_git_timeout(Duration::from_millis(250), || {
        if commits {
            let (result, changed) = publish(&mut session);
            let result = result.expect("tool result");
            assert_eq!(result["isError"] == true, blocked);
            assert_eq!(changed, !blocked);
            if !blocked {
                assert_eq!(result["structuredContent"]["reconciled"], true);
            }
        } else {
            let result = session.explicit_commit("recorded attempt");
            if blocked {
                result.expect_err("external files prevent automatic reconciliation");
            } else {
                assert_eq!(
                    result.expect("automatically resolved commit")["reconciled"],
                    true
                );
            }
        }
    });
    let commit = git_test(&["rev-parse", "HEAD"]);
    if blocked {
        std::fs::remove_file("reconciliation-blocker").expect("resolve external evidence");
        let status = session.status();
        assert_eq!(status["recovery_attempt"]["attempted_commit"], commit);
        assert_eq!(status["head"], before["head"]);
        assert!(Arc::ptr_eq(&accepted, &session.model()));
        assert_eq!(
            std::fs::read(".git/index").expect("unchanged index"),
            index_before
        );
        assert!(session
            .reconcile_commit(&format!("{commit}^"), false)
            .is_err());
        // External edits of affected and unrelated entries must never be erased.
        std::fs::write("Model.md", b"external edit").expect("inject physical mismatch");
        assert!(session.reconcile_commit(&commit, false).is_err());
        assert_eq!(
            std::fs::read("Model.md").expect("external edit retained"),
            b"external edit"
        );
        std::fs::write("Model.md", &changed_source).expect("restore intended candidate");
        let external_oid = git_test(&["hash-object", "-w", "Model.md"]);
        git_test(&[
            "update-index",
            "--cacheinfo",
            &format!("100644,{external_oid},unrelated.bin"),
        ]);
        let externally_staged = std::fs::read(".git/index").expect("external index state");
        assert!(session.reconcile_commit(&commit, false).is_err());
        assert_eq!(
            std::fs::read(".git/index").expect("external staging retained"),
            externally_staged
        );
        std::fs::write(".git/index", &index_before).expect("restore recorded index fixture");
        git_test(&[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("100644,{external_oid},{asset}"),
        ]);
        let affected_edit = std::fs::read(".git/index").expect("affected external index state");
        assert!(session.reconcile_commit(&commit, false).is_err());
        assert_eq!(
            std::fs::read(".git/index").expect("affected staging retained"),
            affected_edit
        );
        std::fs::write(".git/index", &index_before).expect("restore recorded index fixture");
        // A moved checkout, new untracked source, symlink, or changed executable
        // mode is an operator decision, not permission to adopt physical files.
        let head_file = std::fs::read(".git/HEAD").expect("record symbolic HEAD");
        std::fs::write(".git/HEAD", "ref: refs/heads/other\n").expect("inject checkout mismatch");
        assert!(session.reconcile_commit(&commit, false).is_err());
        std::fs::write(".git/HEAD", head_file).expect("restore owned checkout fixture");
        let branch_path = std::path::Path::new(".git").join(git_test(&["symbolic-ref", "HEAD"]));
        std::fs::write(
            &branch_path,
            format!("{}\n", before["head"].as_str().expect("accepted HEAD")),
        )
        .expect("inject moved branch tip");
        assert!(session.reconcile_commit(&commit, false).is_err());
        std::fs::write(&branch_path, format!("{commit}\n")).expect("restore recorded tip fixture");
        std::fs::write(".git/MERGE_HEAD", format!("{commit}\n")).expect("inject merge state");
        assert!(session.reconcile_commit(&commit, false).is_err());
        std::fs::remove_file(".git/MERGE_HEAD").expect("resolve merge fixture");
        std::fs::write("unexpected.md", "invalid model").expect("inject untracked source");
        assert!(session.reconcile_commit(&commit, false).is_err());
        std::fs::remove_file("unexpected.md").expect("resolve untracked fixture");
        std::fs::set_permissions(asset, std::fs::Permissions::from_mode(0o755))
            .expect("inject mode mismatch");
        assert!(session.reconcile_commit(&commit, false).is_err());
        std::fs::set_permissions(asset, std::fs::Permissions::from_mode(0o644))
            .expect("restore candidate mode");
        std::fs::remove_file(asset).expect("replace candidate with symlink");
        std::os::unix::fs::symlink("unrelated.bin", asset).expect("inject symlink mismatch");
        assert!(session.reconcile_commit(&commit, false).is_err());
        std::fs::remove_file(asset).expect("resolve symlink fixture");
        std::fs::write(asset, [0, 255, 128]).expect("restore candidate asset");
        std::fs::write(".git/index.lock", b"other writer").expect("simulate occupied index");
        assert!(session.reconcile_commit(&commit, false).is_err());
        assert_eq!(
            std::fs::read(".git/index.lock").expect("external lock retained"),
            b"other writer"
        );
        std::fs::remove_file(".git/index.lock").expect("release fixture lock");
        let ready = session
            .reconcile_commit(&commit, true)
            .expect("verified preview");
        assert_eq!(ready["outcome"], "ready");
        assert_eq!(
            std::fs::read(".git/index").expect("preview index unchanged"),
            index_before
        );
        assert_eq!(session.status()["writes_available"], false);
        assert_eq!(session.status()["head"], before["head"]);
        let result = session
            .reconcile_commit(&commit, false)
            .expect("apply verified attempt");
        assert_eq!(result["outcome"], "completed");
    } else if !commits {
        assert!(Arc::ptr_eq(&accepted, &session.model()));
        assert_eq!(session.status()["model_revision"], before["model_revision"]);
    }
    assert_eq!(session.status()["head"], commit);
    assert_eq!(session.status()["writes_available"], true);
    assert_eq!(session.status()["recovery_required"], false);
    assert_eq!(session.status()["pending_changes"], json!([]));
    assert_eq!(
        git_test(&["ls-files", "--stage", "unrelated.bin"]),
        unrelated
    );
    assert_eq!(
        std::fs::metadata("unrelated.bin")
            .expect("preserved native permissions")
            .permissions()
            .mode()
            & 0o777,
        0o654
    );
    assert_eq!(
        git_test(&["diff", "--name-only", "--cached"]),
        "unrelated.bin"
    );
    assert_eq!(git_test(&["rev-parse", "HEAD^"]), initial["head"]);
    assert_eq!(
        std::fs::read_to_string(".git/ref-effects").expect("hook effect count"),
        "one\n"
    );
    assert!(!std::path::Path::new(".git/index.lock").exists());
    assert!(!std::path::Path::new("obsolete.bin").exists());
    assert!(
        session.reconcile_commit(&commit, false).is_err(),
        "no repeated reconciliation"
    );
    let read = session
        .capture_read("reqvire.read_element")
        .expect("capture reconciled read");
    drop(session);
    assert_eq!(
        captured
            .execute(|| Ok(json!(
                reqvire::mutation_io::read_to_string("Model.md").expect("old captured source")
            )))
            .expect("old read result"),
        json!(MODEL)
    );
    assert_eq!(
        read.execute(|| Ok(json!(
            reqvire::mutation_io::read_to_string("Model.md").expect("new captured source")
        )))
        .expect("new read result"),
        json!(changed_source)
    );
    assert_eq!(
        read.execute(|| Ok(json!(
            reqvire::mutation_io::read(asset).expect("captured binary asset")
        )))
        .expect("binary read result"),
        json!([0, 255, 128])
    );
}

#[cfg(unix)]
case!(automatic_commit_reconciliation_is_verified_and_selective, {
    reconcile_unknown_commit(true, true);
});
#[cfg(unix)]
case!(explicit_commit_reconciliation_is_verified_and_selective, {
    reconcile_unknown_commit(false, true);
});

#[cfg(unix)]
case!(automatic_commit_timeout_reconciles_selective_index, {
    reconcile_unknown_commit(true, false);
});
#[cfg(unix)]
case!(explicit_commit_timeout_reconciles_selective_index, {
    reconcile_unknown_commit(false, false);
});

#[cfg(unix)]
case!(
    automatic_commit_timeout_before_ref_publication_rolls_back,
    {
        use std::os::unix::fs::PermissionsExt;
        let server = fixture(true);
        let mut session = server.session.lock().expect("session lock");
        let before = session.status();
        let accepted = session.model();
        let index = std::fs::read(".git/index").expect("record index");
        std::fs::write(".git/hooks/reference-transaction", "#!/usr/bin/env python3\nimport sys,time\nfrom pathlib import Path\nif sys.argv[1]=='prepared':\n    with Path('.git/ref-effects').open('a') as f: f.write('one\\n')\n    time.sleep(1)\n").expect("install pre-publication stall");
        std::fs::set_permissions(
            ".git/hooks/reference-transaction",
            std::fs::Permissions::from_mode(0o755),
        )
        .expect("hook executable");
        let (result, changed) =
            crate::mcp_session::with_git_timeout(Duration::from_millis(250), || {
                session.execute(true, "reqvire.add_element", || {
                    reqvire::mutation_io::write(
                        "Model.md",
                        MODEL.replace("Cache Subject", "Unpublished Subject"),
                    )
                    .expect("prepare candidate");
                    Ok(json!({"structuredContent":{}}))
                })
            });
        assert_eq!(result.expect("confirmed failure envelope")["isError"], true);
        assert!(!changed);
        let status = session.status();
        assert_eq!(status["recovery_attempt"], Value::Null);
        assert_eq!(status["recovery_required"], false);
        assert_eq!(status["writes_available"], true);
        assert_eq!(status["head"], before["head"]);
        assert_eq!(git_test(&["rev-parse", "HEAD"]), before["head"]);
        assert_eq!(std::fs::read(".git/index").expect("retained index"), index);
        assert_eq!(status["model_revision"], before["model_revision"]);
        assert_eq!(status["pending_changes"], before["pending_changes"]);
        assert!(Arc::ptr_eq(&accepted, &session.model()));
        assert_eq!(
            session
                .explicit_commit("nothing to commit")
                .expect("writable session")["outcome"],
            "no_op"
        );
        assert_eq!(
            std::fs::read_to_string("Model.md").expect("restored source"),
            MODEL
        );
        assert_eq!(
            std::fs::read_to_string(".git/ref-effects").expect("single hook effect"),
            "one\n"
        );
        assert!(!std::path::Path::new(".git/index.lock").exists());
        drop(session);
    }
);

#[cfg(unix)]
case!(
    explicit_commit_timeout_before_ref_publication_keeps_pending_edits,
    {
        use std::os::unix::fs::PermissionsExt;
        let server = fixture(false);
        let mut session = server.session.lock().expect("session lock");
        let source = MODEL.replace("Cache Subject", "Pending Subject");
        let (result, changed) = session.execute(true, "reqvire.add_element", || {
            reqvire::mutation_io::write("Model.md", &source).expect("prepare pending source");
            Ok(json!({"structuredContent":{}}))
        });
        assert_ne!(result.expect("accepted mutation")["isError"], true);
        assert!(changed);
        let before = session.status();
        let accepted = session.model();
        let index = std::fs::read(".git/index").expect("record index");
        std::fs::write(".git/hooks/reference-transaction", "#!/usr/bin/env python3\nimport sys,time\nfrom pathlib import Path\nif sys.argv[1]=='prepared':\n    with Path('.git/ref-effects').open('a') as f: f.write('one\\n')\n    time.sleep(1)\n").expect("install pre-publication stall");
        std::fs::set_permissions(
            ".git/hooks/reference-transaction",
            std::fs::Permissions::from_mode(0o755),
        )
        .expect("hook executable");
        let failure = crate::mcp_session::with_git_timeout(Duration::from_millis(250), || {
            session.explicit_commit("unpublished attempt")
        })
        .expect_err("commit not published");
        assert!(failure.to_string().contains("not published"), "{failure}");
        let status = session.status();
        assert_eq!(status["recovery_attempt"], Value::Null);
        assert_eq!(status["recovery_required"], false);
        assert_eq!(status["writes_available"], true);
        assert_eq!(status["head"], before["head"]);
        assert_eq!(status["model_revision"], before["model_revision"]);
        assert_eq!(status["pending_changes"], before["pending_changes"]);
        assert_eq!(git_test(&["rev-parse", "HEAD"]), before["head"]);
        assert_eq!(std::fs::read(".git/index").expect("retained index"), index);
        assert_eq!(
            std::fs::read_to_string("Model.md").expect("pending source"),
            source
        );
        assert!(Arc::ptr_eq(&accepted, &session.model()));
        drop(session);
        assert_eq!(
            std::fs::read_to_string(".git/ref-effects").expect("single hook effect"),
            "one\n"
        );
        assert!(!std::path::Path::new(".git/index.lock").exists());
    }
);

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
fn permission_content_edits(commits: bool, mode: u32, deny: bool) {
    use crate::mcp_session::permission_test_io as io;
    fixture_mode("Model.md", mode);
    git_test(&["config", "core.filemode", "true"]);
    git_test(&["add", "Model.md"]);
    git_test(&["commit", "--allow-empty", "-qm", "native mode"]);
    let head = git_test(&["rev-parse", "HEAD"]);
    let index = git_test(&["ls-files", "--stage"]);
    let server = fixture(commits);
    io::configure(if deny { &["*"] } else { &[] }, &[]);
    let result = server
        .dispatch("tools/call", add(OTHER, false))
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
    let new = server.dispatch("tools/call", json!({"name":"reqvire.add_element","arguments":{
        "file":"New.md", "content":"### New Root\n\nNew capability.\n\n#### Metadata\n  * type: capability\n"
    }})).expect("permission regression");
    assert_ne!(new["isError"], true, "{new}");
    assert_eq!(native_mode("New.md") & 0o111, 0);
    assert!(io::calls().is_empty());
    let read = server
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
        )
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
    permission_content_edits(false, 0o640, true);
});
#[cfg(unix)]
case!(permission_edits_with_commits_skip_denied_chmod, {
    permission_content_edits(true, 0o640, true);
});
#[cfg(unix)]
case!(permission_edits_without_commits_preserve_executable_mode, {
    permission_content_edits(false, 0o750, false);
});
#[cfg(unix)]
case!(permission_edits_with_commits_preserve_executable_mode, {
    permission_content_edits(true, 0o750, false);
});
#[cfg(unix)]
case!(permission_edits_preserve_group_execute_bit, {
    permission_content_edits(true, 0o650, true);
});

#[cfg(unix)]
fn required_permission_failure(commits: bool, ineffective: bool, after_delete: bool) {
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
    let server = fixture(commits);
    let revision = server
        .session
        .lock()
        .expect("permission regression")
        .status()["model_revision"]
        .clone();
    io::configure(
        if ineffective { &[] } else { &destinations },
        if ineffective { &destinations } else { &[] },
    );
    let result = server
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.move_asset","arguments":{
                "old_path":"evidence.txt","new_path":destination
            }}),
        )
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
            .lock()
            .expect("permission regression")
            .status()["model_revision"],
        revision
    );
    let read = server
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
        )
        .expect("permission regression");
    assert_ne!(
        read["isError"], true,
        "verified rollback must leave the session usable: {read}"
    );
    io::configure(&[], &[]);
    let retry = server
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.move_asset","arguments":{
                "old_path":"evidence.txt","new_path":destination
            }}),
        )
        .expect("permission regression");
    assert_ne!(retry["isError"], true, "{retry}");
    assert!(native_mode(destination) & 0o111 != 0);
}
#[cfg(unix)]
case!(permission_required_denial_without_commits_recovers, {
    required_permission_failure(false, false, false);
});
#[cfg(unix)]
case!(permission_required_denial_with_commits_recovers, {
    required_permission_failure(true, false, false);
});
#[cfg(unix)]
case!(permission_ineffective_change_is_rejected, {
    required_permission_failure(false, true, false);
});

#[cfg(unix)]
case!(permission_deleted_asset_recovers_without_commits, {
    required_permission_failure(false, false, true);
});
#[cfg(unix)]
case!(permission_deleted_asset_recovers_with_commits, {
    required_permission_failure(true, false, true);
});

#[cfg(unix)]
case!(permission_rollback_skips_matching_native_mode, {
    use crate::mcp_session::permission_test_io as io;
    fixture_mode("Model.md", 0o640);
    let head = git_test(&["rev-parse", "HEAD"]);
    let index = git_test(&["ls-files", "--stage"]);
    let server = fixture(true);
    reject_reference_publication();
    io::configure(&["*"], &[]);
    let result = server
        .dispatch("tools/call", add(OTHER, false))
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
        .dispatch("tools/call", add(OTHER, false))
        .expect("permission regression");
    assert_ne!(retry["isError"], true, "{retry}");
});

#[cfg(unix)]
case!(permission_required_restore_failure_still_disables_writes, {
    use crate::mcp_session::permission_test_io as io;
    evidence_fixture();
    let server = fixture(true);
    let revision = server
        .session
        .lock()
        .expect("permission regression")
        .status()["model_revision"]
        .clone();
    reject_reference_publication();
    io::configure(&["evidence.txt"], &[]);
    let result = server
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.move_asset","arguments":{
                "old_path":"evidence.txt","new_path":"z-moved.sh"
            }}),
        )
        .expect("permission regression");
    assert_eq!(result["isError"], true, "{result}");
    assert!(
        result.to_string().contains("recovery failed")
            && result.to_string().contains("evidence.txt"),
        "{result}"
    );
    let status = server
        .session
        .lock()
        .expect("permission regression")
        .status();
    assert_eq!(status["model_revision"], revision);
    assert_eq!(status["available"], true);
    assert_eq!(status["writes_available"], false);
    io::configure(&[], &[]);
    let retry = server
        .dispatch("tools/call", add(OTHER, false))
        .expect("permission regression");
    assert_eq!(retry["isError"], true);
    assert!(retry.to_string().contains("requires recovery"));
});

#[cfg(unix)]
fn ignored_filemode_keeps_logical_mode(commits: bool) {
    use crate::mcp_session::permission_test_io as io;
    evidence_fixture();
    git_test(&["config", "core.filemode", "false"]);
    fixture_mode("Model.md", 0o777); // typical non-authoritative bind-mount mode
    fixture_mode("evidence.txt", 0o644); // Git still tracks the executable asset
    let server = fixture(commits);
    io::configure(&["*"], &[]);
    let result = server
        .dispatch("tools/call", add(OTHER, false))
        .expect("permission regression");
    assert_ne!(result["isError"], true, "{result}");
    let moved = server
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.move_asset","arguments":{
                "old_path":"evidence.txt","new_path":"moved.sh"
            }}),
        )
        .expect("permission regression");
    assert_ne!(moved["isError"], true, "{moved}");
    if !commits {
        server
            .session
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
    ignored_filemode_keeps_logical_mode(false);
});
#[cfg(unix)]
case!(permission_ignored_filemode_with_commits, {
    ignored_filemode_keeps_logical_mode(true);
});

#[cfg(unix)]
fn recovery_snapshot_reads(commits: bool) {
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
    let server = fixture(commits);
    let before = server
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
        )
        .expect("baseline");
    let revision = server.session.lock().expect("lock").status()["model_revision"].clone();
    let head = git_test(&["rev-parse", "HEAD"]);
    // Source deletion succeeds, destination chmod fails, source mode restoration fails.
    io::configure(&["z-moved.sh", "evidence.txt"], &[]);
    let failed = server.dispatch("tools/call", json!({"name":"reqvire.move_asset","arguments":{"old_path":"evidence.txt","new_path":"z-moved.sh"}})).expect("failure envelope");
    assert!(failed.to_string().contains("recovery failed"), "{failed}");
    io::configure(&[], &[]);
    let status = server.session.lock().expect("lock").status();
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
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.search","arguments":{}}),
        )
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
            .dispatch("tools/call", json!({"name":name,"arguments":{}}))
            .expect("snapshot read");
        assert_ne!(result["isError"], true, "{name}: {result}");
        assert_eq!(
            result["_meta"]["reqvire/context"]["model_revision"],
            revision
        );
    }
    // First SPARQL initialization occurs after failure and external dependency corruption.
    let query = server.dispatch("tools/call", json!({"name":"reqvire.semantic.sparql","arguments":{
        "include_external":true,"query":"ASK { <https://example.test/external#ExternalResource> <http://www.w3.org/2000/01/rdf-schema#label> \"Label alpha\" }"
    }})).expect("lazy snapshot query");
    assert_eq!(query["structuredContent"]["boolean"], true, "{query}");
    let resource = server
        .dispatch(
            "resources/read",
            json!({"uri":"reqvire://workspace/status"}),
        )
        .expect("resource read");
    assert_ne!(resource["isError"], true, "{resource}");
    assert_eq!(
        resource["_meta"]["reqvire/context"]["recovery_required"],
        true
    );
    for params in [
        add(OTHER, false),
        add(OTHER, true),
        json!({"name":"reqvire.format","arguments":{"fix":false}}),
        json!({"name":"reqvire.change_impact","arguments":{}}),
    ] {
        let rejected = server
            .dispatch("tools/call", params)
            .expect("rejection envelope");
        assert_eq!(rejected["isError"], true, "{rejected}");
        assert!(rejected.to_string().contains("recovery"));
    }
    assert!(server
        .session
        .lock()
        .expect("lock")
        .explicit_commit("forbidden")
        .is_err());
    assert_eq!(git_test(&["rev-parse", "HEAD"]), head);
    assert_eq!(
        std::fs::read_to_string("Model.md").expect("disk"),
        "external invalid model"
    );
    assert!(server.dispatch("tools/list", json!({})).expect("discovery")["tools"].is_array());
    std::fs::rename(".git", ".saved-git").expect("make Git observations unavailable");
    let status = server
        .dispatch(
            "tools/call",
            json!({"name":"reqvire.workspace_status","arguments":{}}),
        )
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
    recovery_snapshot_reads(false);
});
#[cfg(unix)]
case!(recovery_preserves_snapshot_reads_with_commits, {
    recovery_snapshot_reads(true);
});
