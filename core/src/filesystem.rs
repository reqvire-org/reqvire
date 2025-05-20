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

