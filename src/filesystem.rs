use crate::error::ReqFlowError;
use std::path::{Path,PathBuf};
use std::fs;
use std::vec::IntoIter;
use crate::git_commands;

pub struct FileReaderIterator<'a> {
    git_commit_hash: Option<&'a str>,
    files: IntoIter<(PathBuf,PathBuf)>,
}

impl<'a> FileReaderIterator<'a> {
    pub fn new(git_commit_hash: Option<&'a str>, files: Vec<(PathBuf,PathBuf)>) -> Self {
        Self {
            files: files.into_iter(),
            git_commit_hash: git_commit_hash,            
        }
    }
}

impl Iterator for FileReaderIterator<'_>{
    type Item = Result<(PathBuf, String, String), ReqFlowError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.files.next().map(|(file,folder)| {
            let filename_str = file.file_name()
                .ok_or_else(|| ReqFlowError::PathError(format!("Problem reading file: {:?}", file)))?
                .to_string_lossy()
                .to_string();
                
            match self.git_commit_hash{
                Some(commit)=>{                  
                    match git_commands::get_file_at_commit(&file.to_string_lossy(), &folder, commit)  {
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
                    .map_err(ReqFlowError::IoError)
                }            
            }
        })
    }
}
/// Reads a file's content
pub fn read_file(path: &Path) -> Result<String, ReqFlowError> {
    fs::read_to_string(path).map_err(|e| ReqFlowError::IoError(e))
}


/// Write content to a file, creating parent directories if needed
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> Result<(), ReqFlowError> {
    if let Some(parent) = path.as_ref().parent() {
        create_dir_all(parent)?;
    }
    
    fs::write(path.as_ref(), content).map_err(|e| {
        ReqFlowError::IoError(e)
    })
}



/// Create directory and any parent directories if they don't exist
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), ReqFlowError> {
    fs::create_dir_all(path.as_ref()).map_err(|e| {
        ReqFlowError::IoError(e)
    })
}

