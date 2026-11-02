//! Process-safe nonblocking ownership, shared by every workspace consumer.
use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};
#[cfg(unix)]
pub fn claim(file: &File) -> std::io::Result<()> {
    rustix::fs::flock(file, rustix::fs::FlockOperation::NonBlockingLockExclusive)
        .map_err(Into::into)
}
#[cfg(windows)]
pub fn claim(file: &File) -> std::io::Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::{
        Storage::FileSystem::{LockFileEx, LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY},
        System::IO::OVERLAPPED,
    };
    // The synchronous handle remains open for the session; Windows releases the
    // byte-range lock on close/process exit. No asynchronous operation is used.
    let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
    let result = unsafe {
        LockFileEx(
            file.as_raw_handle(),
            LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY,
            0,
            1,
            0,
            &mut overlapped,
        )
    };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
fn release(file: &File) -> std::io::Result<()> {
    rustix::fs::flock(file, rustix::fs::FlockOperation::Unlock).map_err(Into::into)
}

#[cfg(windows)]
fn release(file: &File) -> std::io::Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::{Storage::FileSystem::UnlockFileEx, System::IO::OVERLAPPED};
    let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
    let result = unsafe { UnlockFileEx(file.as_raw_handle(), 0, 1, 0, &mut overlapped) };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn lock_contention(failure: &std::io::Error) -> bool {
    #[cfg(unix)]
    {
        failure.kind() == std::io::ErrorKind::WouldBlock
    }
    #[cfg(windows)]
    {
        failure.raw_os_error() == Some(windows_sys::Win32::Foundation::ERROR_LOCK_VIOLATION as i32)
    }
}

pub fn unsupported_lock(failure: &std::io::Error) -> bool {
    if failure.kind() == std::io::ErrorKind::Unsupported {
        return true;
    }
    #[cfg(unix)]
    {
        failure.raw_os_error() == Some(rustix::io::Errno::NOSYS.raw_os_error())
    }
    #[cfg(windows)]
    {
        failure.raw_os_error()
            == Some(windows_sys::Win32::Foundation::ERROR_INVALID_FUNCTION as i32)
    }
}

pub fn lock_error(path: &Path, operation: &str, failure: std::io::Error) -> crate::Error {
    let guidance = if operation == "acquire" && lock_contention(&failure) {
        "already owned or busy; another operation holds this lock"
    } else if failure.kind() == std::io::ErrorKind::PermissionDenied {
        "permission denied; check this file and its directory permissions for the server/container user"
    } else if operation == "acquire" && unsupported_lock(&failure) {
        "filesystem locking is unsupported; use a filesystem with exclusive lock support"
    } else {
        "lock operation failed; ownership was not acquired"
    };
    crate::Error::ProcessError(format!(
        "Cannot {operation} MCP lock '{}': {failure}; {guidance}. Leave lock files in place.",
        path.display()
    ))
}

pub fn open_lock(path: &Path) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
}

/// Owns one OS lock, releasing it explicitly before closing the handle.
///
/// An incidental fork/exec descriptor copy must not extend the owner's lifetime.
/// The handle is private and cannot be cloned or transferred out of this guard.
#[derive(Debug)]
pub struct FileLock {
    file: File,
    path: PathBuf,
}

impl FileLock {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        if let Err(failure) = release(&self.file) {
            log::error!(
                "Cannot release MCP lock '{}': {failure}",
                self.path.display()
            );
        }
    }
}

// Shared by context ownership and repository administration. Never remove the
// filename to release a lock: other contenders must continue using its inode.
pub fn lock_file(path: &Path) -> Result<FileLock, crate::Error> {
    lock_file_with(path, open_lock, claim)
}

pub fn lock_file_with(
    path: &Path,
    open: impl FnOnce(&Path) -> std::io::Result<File>,
    acquire: impl FnOnce(&File) -> std::io::Result<()>,
) -> Result<FileLock, crate::Error> {
    let file = open(path).map_err(|failure| lock_error(path, "open", failure))?;
    acquire(&file).map_err(|failure| lock_error(path, "acquire", failure))?;
    Ok(FileLock {
        file,
        path: path.to_path_buf(),
    })
}
