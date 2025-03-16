use crate::error::ReqFlowError;
use std::path::{Path, PathBuf};
use std::fs;

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



