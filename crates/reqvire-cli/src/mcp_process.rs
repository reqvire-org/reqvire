//! Protocol adapter for the shared workspace process runner.
use reqvire::error::ReqvireError;
pub use reqvire_workspace::process::{BinaryOutput, Output};
use std::{path::Path, process::Command, time::Duration};
pub struct Failure {
    pub error: ReqvireError,
    pub outcome_unknown: bool,
}
pub fn run(
    root: &Path,
    program: &str,
    args: &[&str],
    input: Option<&[u8]>,
    timeout: Duration,
) -> Result<Output, ReqvireError> {
    reqvire_workspace::process::run(root, program, args, input, timeout).map_err(Into::into)
}
#[cfg(test)]
fn run_command(
    command: Command,
    input: Option<&[u8]>,
    timeout: Duration,
) -> Result<Output, ReqvireError> {
    reqvire_workspace::process::run_command(command, input, timeout).map_err(Into::into)
}
pub fn run_binary(
    command: Command,
    input: Option<&[u8]>,
    timeout: Duration,
    output_limit: Option<usize>,
) -> Result<BinaryOutput, Failure> {
    reqvire_workspace::process::run_binary(command, input, timeout, output_limit).map_err(|f| {
        Failure {
            error: f.error.into(),
            outcome_unknown: f.outcome_unknown,
        }
    })
}
pub fn git(root: &Path, args: &[&str]) -> Result<String, ReqvireError> {
    reqvire_workspace::process::git(root, args).map_err(Into::into)
}

#[cfg(all(test, unix))]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::fs;

    fn python(root: &Path, script: &str) -> Command {
        let mut command = Command::new("python3");
        command.current_dir(root).args(["-c", script]);
        command
    }

    #[test]
    fn session_git_drains_large_streams_while_sending_binary_stdin() {
        let dir = tempfile::tempdir().unwrap();
        crate::mcp_session::git(dir.path(), &["init", "-q"], None, None).unwrap();
        let script = dir.path().join("duplex.py");
        fs::write(&script, "import sys, signal\nsignal.alarm(2)\nsys.stdout.buffer.write(b'x' * 1048576)\nsys.stdout.flush()\nsys.stderr.buffer.write(b'e' * 1048576)\nsys.stderr.flush()\ndata = sys.stdin.buffer.read()\nsys.stdout.buffer.write(b'\\0\\xff' + data)\n").unwrap();
        let alias = format!("alias.duplex=!python3 {}", script.display());
        let input = [0, 255, 128, 10].repeat(262144);
        let output =
            crate::mcp_session::git(dir.path(), &["-c", &alias, "duplex"], Some(&input), None)
                .unwrap();
        let mut expected = vec![b'x'; 1048576];
        expected.extend_from_slice(&[0, 255]);
        expected.extend_from_slice(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn incomplete_stdin_cannot_be_reported_as_success() {
        let dir = tempfile::tempdir().unwrap();
        let command = python(dir.path(), "import os, time\nos.close(0)\nprint('apparent success', flush=True)\ntime.sleep(0.1)\n");
        let result = run_command(command, Some(&vec![b'x'; 1048576]), Duration::from_secs(2));
        let failure = result
            .err()
            .expect("incomplete stdin must be an error")
            .to_string();
        assert!(failure.contains("stdin"), "{failure}");
        assert!(failure.contains("outcome may be unknown"), "{failure}");
    }

    fn inherited_pipe_is_bounded_and_descendant_stops(parent_exits: bool) {
        let dir = tempfile::tempdir().unwrap();
        let script = format!("import subprocess, sys, time\nsubprocess.Popen([sys.executable, '-c', \"import time; from pathlib import Path; time.sleep(1); Path('descendant-effect').write_text('unexpected')\"])\n{}\n", if parent_exits { "sys.exit(0)" } else { "time.sleep(5)" });
        let result = run_command(
            python(dir.path(), &script),
            None,
            Duration::from_millis(250),
        );
        let failure = result
            .err()
            .expect("inherited pipe must time out")
            .to_string();
        assert!(failure.contains("timed out"), "{failure}");
        assert!(failure.contains("outcome may be unknown"), "{failure}");
        std::thread::sleep(Duration::from_millis(1100));
        assert!(
            !dir.path().join("descendant-effect").exists(),
            "owned descendant outlived timeout"
        );
    }

    #[test]
    fn timeout_stops_a_live_parent_and_its_pipe_holding_descendant() {
        inherited_pipe_is_bounded_and_descendant_stops(false);
    }

    #[test]
    fn timeout_stops_a_pipe_holding_descendant_after_parent_exit() {
        inherited_pipe_is_bounded_and_descendant_stops(true);
    }

    #[test]
    fn output_limit_is_detected_before_waiting_for_process_exit() {
        let dir = tempfile::tempdir().unwrap();
        let command = python(dir.path(), "import sys, time, signal\nsignal.alarm(3)\nsys.stdout.buffer.write(b'x' * (17 * 1024 * 1024))\nsys.stdout.flush()\ntime.sleep(2)\n");
        let failure = run_command(command, None, Duration::from_secs(1))
            .err()
            .unwrap()
            .to_string();
        assert!(failure.contains("output exceeded limit"), "{failure}");
        assert!(failure.contains("outcome may be unknown"), "{failure}");
    }

    #[test]
    fn timeout_does_not_erase_or_retry_a_completed_side_effect() {
        let dir = tempfile::tempdir().unwrap();
        let command = python(dir.path(), "from pathlib import Path\nimport time\nwith Path('attempts').open('a') as f: f.write('one\\n')\ntime.sleep(5)\n");
        let failure = run_command(command, None, Duration::from_millis(250))
            .err()
            .unwrap()
            .to_string();
        assert!(failure.contains("outcome may be unknown"), "{failure}");
        assert_eq!(
            fs::read_to_string(dir.path().join("attempts")).unwrap(),
            "one\n"
        );
    }

    #[test]
    fn session_git_preserves_large_blobs_literal_paths_and_alternate_indexes() {
        let dir = tempfile::tempdir().unwrap();
        let git = |args: &[&str], input: Option<&[u8]>, index: Option<&Path>| {
            crate::mcp_session::git(dir.path(), args, input, index).unwrap()
        };
        git(&["init", "-q"], None, None);
        let path = "literal[ab]*\nname.bin";
        fs::write(dir.path().join(path), "original").unwrap();
        fs::write(dir.path().join("literalab-extra\nname.bin"), "unrelated").unwrap();
        git(&["add", "."], None, None);
        let old_index = fs::read(dir.path().join(".git/index")).unwrap();
        assert_eq!(
            git(&["ls-files", "-z", "--", path], None, None),
            format!("{path}\0").as_bytes()
        );
        let bytes = [0, 255, 128, 10].repeat(17 * 1024 * 1024 / 4);
        let oid =
            String::from_utf8(git(&["hash-object", "-w", "--stdin"], Some(&bytes), None)).unwrap();
        assert_eq!(git(&["cat-file", "blob", oid.trim()], None, None), bytes);
        let private_index = dir.path().join("private-index");
        git(&["read-tree", "--empty"], None, Some(&private_index));
        let entry = format!("100644 {}\t{path}\0", oid.trim());
        git(
            &["update-index", "-z", "--index-info"],
            Some(entry.as_bytes()),
            Some(&private_index),
        );
        assert_eq!(
            git(&["ls-files", "-z"], None, Some(&private_index)),
            format!("{path}\0").as_bytes()
        );
        assert_eq!(fs::read(dir.path().join(".git/index")).unwrap(), old_index);
        assert_eq!(fs::read(dir.path().join(path)).unwrap(), b"original");
        assert_eq!(
            fs::read(dir.path().join("literalab-extra\nname.bin")).unwrap(),
            b"unrelated"
        );
        // Exercise the shared Git tree/blob reader through the same bounded
        // transport, retaining binary bytes and literal NUL-delimited names.
        fs::write(dir.path().join(path), &bytes).unwrap();
        git(&["add", "--", path], None, None);
        git(
            &[
                "-c",
                "user.name=Git Test",
                "-c",
                "user.email=git@example.invalid",
                "commit",
                "-qm",
                "baseline",
            ],
            None,
            None,
        );
        let head = String::from_utf8(git(&["rev-parse", "HEAD"], None, None)).unwrap();
        let paths = [dir.path().join(path), dir.path().join("deleted.bin")].into();
        let mut reader = reqvire::git_commands::GitReader::new(dir.path(), |args, input| {
            crate::mcp_session::git(dir.path(), args, input, None)
        });
        let files = reader.regular_files_at_commit(head.trim(), &paths).unwrap();
        assert_eq!(files[&dir.path().join(path)].bytes.as_slice(), bytes);
        assert!(!files.contains_key(&dir.path().join("deleted.bin")));
        assert!(!reader.operation_in_progress().unwrap());
    }

    #[test]
    fn output_caps_apply_to_each_stream_without_exposing_data() {
        let dir = tempfile::tempdir().unwrap();
        for stream in ["stdout", "stderr"] {
            let script = format!("import sys, time\nsys.{stream}.buffer.write(b'sensitive-data' * 4)\nsys.{stream}.flush()\ntime.sleep(2)\n");
            let failure = run_binary(
                python(dir.path(), &script),
                None,
                Duration::from_secs(1),
                Some(32),
            )
            .err()
            .unwrap();
            assert!(failure.outcome_unknown);
            let text = failure.error.to_string();
            assert!(text.contains("output exceeded limit"), "{text}");
            assert!(!text.contains("sensitive-data"), "{text}");
        }
        let output = run_binary(
            python(
                dir.path(),
                "import sys\nsys.stdout.write('x' * 32)\nsys.stderr.write('y' * 32)\n",
            ),
            None,
            Duration::from_secs(1),
            Some(32),
        )
        .map_err(|failure| failure.error)
        .unwrap();
        assert!(output.success);
        assert_eq!(output.stdout, vec![b'x'; 32]);
        assert_eq!(output.stderr, vec![b'y'; 32]);
    }

    #[test]
    fn text_adapter_retains_nonzero_stderr_and_closes_empty_stdin() {
        let dir = tempfile::tempdir().unwrap();
        let output = run_command(
            python(
                dir.path(),
                "import os, sys\nos.close(0)\nsys.stderr.write('rejected input')\nsys.exit(7)\n",
            ),
            Some(&vec![0; 1048576]),
            Duration::from_secs(1),
        )
        .unwrap();
        assert!(!output.success);
        assert_eq!(output.stderr, "rejected input");
        let output = run_command(python(dir.path(), "import sys\nprint(len(sys.stdin.buffer.read()))\nsys.stderr.buffer.write(b'\\xff')\n"), None, Duration::from_secs(1)).unwrap();
        assert!(output.success);
        assert_eq!(output.stdout, "0\n");
        assert_eq!(output.stderr, "\u{fffd}");
    }

    #[test]
    fn noninteractive_settings_preserve_configured_transport() {
        let dir = tempfile::tempdir().unwrap();
        let mut command = python(dir.path(), "import os\nassert os.environ['GIT_TERMINAL_PROMPT'] == '0'\nassert os.environ['GH_PROMPT_DISABLED'] == '1'\nassert os.environ['GH_PAGER'] == 'cat'\nassert os.environ['GIT_SSH_COMMAND'] == 'configured transport'\n");
        command.env("GIT_SSH_COMMAND", "configured transport");
        assert!(
            run_command(command, None, Duration::from_secs(1))
                .unwrap()
                .success
        );
    }

    #[test]
    fn spawn_failure_is_distinct_from_an_interrupted_operation() {
        let dir = tempfile::tempdir().unwrap();
        let failure = run_binary(
            Command::new(dir.path().join("missing-executable")),
            None,
            Duration::from_secs(1),
            None,
        )
        .err()
        .unwrap();
        assert!(!failure.outcome_unknown);
        let command = python(dir.path(), "from pathlib import Path\nimport os, signal\nPath('effect').write_text('one')\nos.kill(os.getpid(), signal.SIGKILL)\n");
        let failure = run_binary(command, None, Duration::from_secs(1), None)
            .err()
            .unwrap();
        assert!(failure.outcome_unknown);
        assert_eq!(
            fs::read_to_string(dir.path().join("effect")).unwrap(),
            "one"
        );
    }
}
