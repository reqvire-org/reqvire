use crate::error::ReqvireError;
use std::path::{Path,PathBuf};
use std::fs;
use std::vec::IntoIter;
use crate::git_commands;

pub struct FileReaderIterator<'a> {
    git_commit_hash: Option<&'a str>,
    files: IntoIter<PathBuf>,
}

impl<'a> FileReaderIterator<'a> {
    pub fn new(git_commit_hash: Option<&'a str>, files: Vec<PathBuf>) -> Self {
        Self {
            files: files.into_iter(),
            git_commit_hash: git_commit_hash,            
        }
    }
}

impl Iterator for FileReaderIterator<'_>{
    type Item = Result<(PathBuf, String, String), ReqvireError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.files.next().map(|file| {
            let filename_str = file.file_name()
                .ok_or_else(|| ReqvireError::PathError(format!("Problem reading file: {:?}", file)))?
                .to_string_lossy()
                .to_string();
                
            match self.git_commit_hash{
                Some(commit)=>{
                    // Get git root directory
                    let git_root = match git_commands::get_git_root_dir() {
                        Ok(dir) => dir,
                        Err(_) => {
                            return Err(ReqvireError::GitCommandError("Failed to get git root directory".to_string()));
                        }
                    };
                
                    match git_commands::get_file_at_commit(&file.to_string_lossy(), &git_root, commit) {
                        Ok(content)=> {
                            Ok((file, filename_str, content))
                        },
                        Err(e)=> {
                            Err(e)                    
                        }
                    }
                },
                None=>{
                    fs::read_to_string(&file)
                    .map(|content| (file, filename_str, content))
                    .map_err(ReqvireError::IoError)
                }            
            }
        })
    }
}
/// Reads a file's content
pub fn read_file(path: &Path) -> Result<String, ReqvireError> {
    fs::read_to_string(path).map_err(|e| ReqvireError::IoError(e))
}


/// Write content to a file, creating parent directories if needed
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> Result<(), ReqvireError> {
    if let Some(parent) = path.as_ref().parent() {
        create_dir_all(parent)?;
    }
    
    fs::write(path.as_ref(), content).map_err(|e| {
        ReqvireError::IoError(e)
    })
}



/// Create directory and any parent directories if they don't exist
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), ReqvireError> {
    fs::create_dir_all(path.as_ref()).map_err(|e| {
        ReqvireError::IoError(e)
    })
}

/// Creates a temporary working directory with a unique name based on process ID
pub fn create_temp_working_dir() -> Result<PathBuf, ReqvireError> {
    let temp_base = std::env::temp_dir();
    let temp_dir = temp_base.join(format!("reqvire-export-{}", std::process::id()));

    fs::create_dir_all(&temp_dir)
        .map_err(|e| ReqvireError::PathError(format!("Failed to create temp directory: {}", e)))?;

    Ok(temp_dir)
}

/// Copies a single file from source to destination, creating parent directories as needed
pub fn copy_file_with_structure(src: &Path, dst: &Path) -> Result<(), ReqvireError> {
    if let Some(parent) = dst.parent() {
        create_dir_all(parent)?;
    }

    fs::copy(src, dst)
        .map_err(|e| ReqvireError::IoError(e))?;

    Ok(())
}

/// Recursively copies all files and directories from source to destination
pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), ReqvireError> {
    fs::create_dir_all(dst)
        .map_err(|e| ReqvireError::IoError(e))?;

    for entry in fs::read_dir(src).map_err(|e| ReqvireError::IoError(e))? {
        let entry = entry.map_err(|e| ReqvireError::IoError(e))?;
        let ty = entry.file_type().map_err(|e| ReqvireError::IoError(e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)
                .map_err(|e| ReqvireError::IoError(e))?;
        }
    }
    Ok(())
}

/// Removes a directory and all its contents
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), ReqvireError> {
    if path.as_ref().exists() {
        fs::remove_dir_all(path.as_ref())
            .map_err(|e| ReqvireError::IoError(e))?;
    }
    Ok(())
}

