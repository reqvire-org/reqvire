//! Bounded, noninteractive subprocesses with arguments kept separate from data.
use crate::Error as ReqvireError;
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

pub fn run_command(
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
///
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
