//! Bounded, noninteractive subprocesses with arguments kept separate from data.
use reqvire::error::ReqvireError;
use std::{
    io::{self, Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};
pub struct Output {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}
pub fn run(
    root: &Path,
    program: &str,
    args: &[&str],
    input: Option<&[u8]>,
    timeout: Duration,
) -> Result<Output, ReqvireError> {
    let mut command = Command::new(program);
    command.current_dir(root).args(args);
    run_command(command, input, timeout)
}

fn run_command(
    command: Command,
    input: Option<&[u8]>,
    timeout: Duration,
) -> Result<Output, ReqvireError> {
    let output = run_binary(command, input, timeout, Some(16 * 1024 * 1024))
        .map_err(|failure| failure.error)?;
    Ok(Output {
        success: output.success,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

pub struct BinaryOutput {
    pub success: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub struct Failure {
    pub error: ReqvireError,
    pub outcome_unknown: bool,
}

impl Failure {
    fn started(reason: impl std::fmt::Display) -> Self {
        Self {
            error: ReqvireError::ProcessError(format!(
                "{reason}; operation outcome may be unknown"
            )),
            outcome_unknown: true,
        }
    }
}

enum Event {
    Stdout(io::Result<Vec<u8>>),
    Stderr(io::Result<Vec<u8>>),
    Stdin(io::Result<()>),
}

/// Byte transport shared by text publication commands and local Git plumbing.
/// Local Git retains uncapped binary output; publication retains its existing
/// per-stream cap. The deadline includes stdin, process exit, and pipe EOF.
pub fn run_binary(
    mut command: Command,
    input: Option<&[u8]>,
    timeout: Duration,
    output_limit: Option<usize>,
) -> Result<BinaryOutput, Failure> {
    let deadline = Instant::now() + timeout;
    command
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GH_PROMPT_DISABLED", "1")
        .env("GH_PAGER", "cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // Preserve configured transports; the default SSH invocation must not open a prompt.
    let configured_ssh = command
        .get_envs()
        .any(|(key, value)| (key == "GIT_SSH_COMMAND" || key == "GIT_SSH") && value.is_some());
    if !configured_ssh
        && std::env::var_os("GIT_SSH_COMMAND").is_none()
        && std::env::var_os("GIT_SSH").is_none()
    {
        command.env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes");
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // Assign the job before user code can create an unowned descendant.
        command.creation_flags(windows_sys::Win32::System::Threading::CREATE_SUSPENDED);
    }
    #[cfg(windows)]
    let job = WindowsJob::new().map_err(|error| Failure {
        error: error.into(),
        outcome_unknown: false,
    })?;
    let child = command.spawn().map_err(|error| Failure {
        error: ReqvireError::ProcessError(format!(
            "Could not start required Git/GitHub executable: {error}"
        )),
        outcome_unknown: false,
    })?;
    let mut process = Process {
        #[cfg(unix)]
        group: rustix::process::Pid::from_child(&child),
        child,
        completed: false,
        #[cfg(windows)]
        job,
    };
    #[cfg(windows)]
    process
        .job
        .assign(&process.child)
        .map_err(Failure::started)?;
    #[cfg(windows)]
    WindowsJob::resume(&process.child).map_err(Failure::started)?;
    let (send, receive) = mpsc::channel();
    let read = |stream: Box<dyn Read + Send>, event: fn(io::Result<Vec<u8>>) -> Event| {
        let send = send.clone();
        std::thread::spawn(move || {
            let mut data = Vec::new();
            let mut stream = stream.take(output_limit.map_or(u64::MAX, |limit| limit as u64 + 1));
            let result = stream.read_to_end(&mut data).and_then(|size| {
                if output_limit.is_some_and(|limit| size > limit) {
                    Err(std::io::Error::other("Subprocess output exceeded limit"))
                } else {
                    Ok(data)
                }
            });
            let _ = send.send(event(result));
        })
    };
    let out = read(
        Box::new(process.child.stdout.take().expect("piped stdout")),
        Event::Stdout,
    );
    let err = read(
        Box::new(process.child.stderr.take().expect("piped stderr")),
        Event::Stderr,
    );
    let mut stdin = process.child.stdin.take().expect("piped stdin");
    let bytes = input.map(Vec::from);
    let writer = std::thread::spawn(move || {
        let result = bytes.map_or(Ok(()), |bytes| stdin.write_all(&bytes));
        drop(stdin);
        let _ = send.send(Event::Stdin(result));
    });
    let result = (|| {
        let (mut status, mut stdout, mut stderr, mut stdin) =
            (None, None, None, None::<io::Result<()>>);
        loop {
            if status.is_none() {
                status = process.child.try_wait().map_err(Failure::started)?;
            }
            if let Some(status) = status {
                if stdout.is_some() && stderr.is_some() && stdin.is_some() {
                    if status.code().is_none() {
                        return Err(Failure::started(
                            "Subprocess terminated before confirming its outcome",
                        ));
                    }
                    // A nonzero command keeps its stderr diagnostic even if it
                    // closed stdin early. Success requires complete input.
                    if status.success() {
                        stdin
                            .take()
                            .expect("received stdin result")
                            .map_err(|error| {
                                Failure::started(format!("Subprocess stdin failed: {error}"))
                            })?;
                    }
                    process.completed = true;
                    return Ok(BinaryOutput {
                        success: status.success(),
                        stdout: stdout.take().expect("received stdout"),
                        stderr: stderr.take().expect("received stderr"),
                    });
                }
            }
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(Failure::started("Operation timed out"));
            }
            match receive.recv_timeout(remaining.min(Duration::from_millis(10))) {
                Ok(Event::Stdout(result)) => stdout = Some(result.map_err(Failure::started)?),
                Ok(Event::Stderr(result)) => stderr = Some(result.map_err(Failure::started)?),
                Ok(Event::Stdin(result)) => stdin = Some(result),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected)
                    if stdout.is_some() && stderr.is_some() && stdin.is_some() =>
                {
                    std::thread::sleep(remaining.min(Duration::from_millis(10)))
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(Failure::started("Subprocess pipe helper failed"))
                }
            }
        }
    })();
    drop(process); // stop the owned family and reap the direct child on failure
    for helper in [out, err, writer] {
        // Never extend the deadline by joining a pipe inherited by an escaped
        // process. Completed helpers have no I/O left after sending their event.
        if result.is_ok() || helper.is_finished() {
            let _ = helper.join();
        }
    }
    result
}

struct Process {
    child: Child,
    completed: bool,
    #[cfg(unix)]
    group: rustix::process::Pid,
    #[cfg(windows)]
    job: WindowsJob,
}

impl Drop for Process {
    fn drop(&mut self) {
        if !self.completed {
            #[cfg(unix)]
            let _ = rustix::process::kill_process_group(self.group, rustix::process::Signal::KILL);
            #[cfg(windows)]
            self.job.terminate();
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[cfg(windows)]
struct WindowsJob(std::os::windows::io::OwnedHandle);

#[cfg(windows)]
impl WindowsJob {
    fn new() -> io::Result<Self> {
        use std::os::windows::io::{AsRawHandle, FromRawHandle};
        use windows_sys::Win32::System::JobObjects::*;
        // The handle is private and non-inheritable. Closing it also stops
        // assigned descendants if any cleanup path returns early.
        let handle = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if handle.is_null() {
            return Err(io::Error::last_os_error());
        }
        let job = Self(unsafe { std::os::windows::io::OwnedHandle::from_raw_handle(handle) });
        let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        if unsafe {
            SetInformationJobObject(
                job.0.as_raw_handle(),
                JobObjectExtendedLimitInformation,
                &limits as *const _ as *const _,
                std::mem::size_of_val(&limits) as u32,
            )
        } == 0
        {
            return Err(io::Error::last_os_error());
        }
        Ok(job)
    }
    fn assign(&self, child: &Child) -> io::Result<()> {
        use std::os::windows::io::AsRawHandle;
        if unsafe {
            windows_sys::Win32::System::JobObjects::AssignProcessToJobObject(
                self.0.as_raw_handle(),
                child.as_raw_handle(),
            )
        } == 0
        {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
    fn terminate(&self) {
        use std::os::windows::io::AsRawHandle;
        unsafe {
            windows_sys::Win32::System::JobObjects::TerminateJobObject(self.0.as_raw_handle(), 1);
        }
    }
    fn resume(child: &Child) -> io::Result<()> {
        use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
        use windows_sys::Win32::{
            Foundation::INVALID_HANDLE_VALUE,
            System::{
                Diagnostics::ToolHelp::*,
                Threading::{OpenThread, ResumeThread, THREAD_SUSPEND_RESUME},
            },
        };
        // std::process exposes the process handle but not the primary thread
        // handle at our MSRV. A newly created suspended process has one thread.
        let raw = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
        if raw == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        let snapshot = unsafe { OwnedHandle::from_raw_handle(raw) };
        let mut entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        };
        let mut found = unsafe { Thread32First(snapshot.as_raw_handle(), &mut entry) };
        while found != 0 {
            if entry.th32OwnerProcessID == child.id() {
                let raw = unsafe { OpenThread(THREAD_SUSPEND_RESUME, 0, entry.th32ThreadID) };
                if raw.is_null() {
                    return Err(io::Error::last_os_error());
                }
                let thread = unsafe { OwnedHandle::from_raw_handle(raw) };
                return if unsafe { ResumeThread(thread.as_raw_handle()) } == u32::MAX {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(())
                };
            }
            entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;
            found = unsafe { Thread32Next(snapshot.as_raw_handle(), &mut entry) };
        }
        Err(io::Error::other("Suspended subprocess thread unavailable"))
    }
}
pub fn git(root: &Path, args: &[&str]) -> Result<String, ReqvireError> {
    let output = run(root, "git", args, None, Duration::from_secs(30))?;
    if !output.success {
        return Err(ReqvireError::ProcessError(format!(
            "Git {} failed",
            args.first().unwrap_or(&"operation")
        )));
    }
    Ok(output.stdout.trim().to_string())
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
