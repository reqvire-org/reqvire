//! Bounded, noninteractive subprocesses with arguments kept separate from data.
use reqvire::error::ReqvireError;
use std::{
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
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
    mut command: Command,
    input: Option<&[u8]>,
    timeout: Duration,
) -> Result<Output, ReqvireError> {
    let deadline = Instant::now() + timeout;
    command
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GH_PROMPT_DISABLED", "1")
        .env("GH_PAGER", "cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // Preserve configured transports; the default SSH invocation must not open a prompt.
    if std::env::var_os("GIT_SSH_COMMAND").is_none() && std::env::var_os("GIT_SSH").is_none() {
        command.env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes");
    }
    let mut child = command.spawn().map_err(|_| {
        ReqvireError::ProcessError("Could not start required Git/GitHub executable".into())
    })?;
    let read = |stream: Box<dyn Read + Send>| {
        let (send, receive) = mpsc::sync_channel(1);
        std::thread::spawn(move || {
            let mut data = Vec::new();
            let result = stream
                .take(16 * 1024 * 1024 + 1)
                .read_to_end(&mut data)
                .and_then(|size| {
                    if size > 16 * 1024 * 1024 {
                        Err(std::io::Error::other("Subprocess output exceeded limit"))
                    } else {
                        Ok(String::from_utf8_lossy(&data).into_owned())
                    }
                });
            let _ = send.send(result);
        });
        receive
    };
    let out = read(Box::new(child.stdout.take().expect("piped stdout")));
    let err = read(Box::new(child.stderr.take().expect("piped stderr")));
    let mut stdin = child.stdin.take().expect("piped stdin");
    let bytes = input.map(Vec::from);
    std::thread::spawn(move || {
        if let Some(bytes) = bytes {
            let _ = stdin.write_all(&bytes);
        }
    });
    let expired =
        || ReqvireError::ProcessError("Operation timed out; remote outcome may be unknown".into());
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(expired());
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    // A descendant keeping inherited stdout open must not make an otherwise bounded call hang.
    let stdout = out
        .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        .map_err(|_| expired())??;
    let stderr = err
        .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        .map_err(|_| expired())??;
    Ok(Output {
        success: status.success(),
        stdout,
        stderr,
    })
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
