use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ReqFlowError;

/// Check if a file is a requirements file or related specification according to ReqFlow
/// This is a legacy method that uses filename-based heuristics
/// Use is_requirements_file_by_path with configuration when possible
pub fn is_requirements_file(filename: &str) -> bool {
    let lowercase = filename.to_lowercase();
    
    // According to the documentation, we should process:
    // 1. Files with "requirements" in the name
    // 2. UserRequirements.md, MissionRequirements.md files
    // 3. User stories and other specification files
    // 4. Design specifications (DSD)
    
    lowercase.contains("requirements") || 
    lowercase.contains("userstories") || 
    lowercase.contains("usecases") || 
    lowercase.contains("moes") ||
    lowercase.contains("dsd_") // Include Design Specification Documents
}

/// Check if a file is a requirements file or design specification based on its path and configuration
pub fn is_requirements_file_by_path(path: &Path, config: &crate::config::Config, base_path: &Path) -> bool {
    // Early return if not a markdown file
    if path.extension().map_or(false, |ext| ext != "md") {
        return false;
    }
    
    // Check if the file is in the system requirements directory
    let specs_dir = base_path.join(&config.paths.specifications_folder);
    let system_reqs_path = specs_dir.join(&config.paths.system_requirements_folder);
    if path.starts_with(&system_reqs_path) {
        return true;
    }
    
    // Check if the file is in any design specifications directory
    // Design specs are identified by being in any folder that matches the design_specifications_folder name
    let rel_path = match path.strip_prefix(base_path) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string()
    };
    
    let design_specs_folder_name = &config.paths.design_specifications_folder;
    if rel_path.split('/').any(|component| component == design_specs_folder_name) {
        return true;
    }
    
    // Check if the file matches the requirements_filename_match pattern
    if let Some(filename) = path.file_name() {
        let filename_str = filename.to_string_lossy();
        if filename_str.contains(&config.paths.requirements_filename_match) {
            return true;
        }
    }
    
    // Fall back to filename-based check for compatibility
    if let Some(filename) = path.file_name() {
        return is_requirements_file(filename.to_string_lossy().as_ref());
    }
    
    false
}

/// Check if a file should be considered for processing in the ReqFlow system
/// This is more inclusive than is_requirements_file and includes all markdown files
/// that might be referenced or part of the specification set
pub fn is_processable_file(filename: &str) -> bool {
    // Only markdown files
    if !filename.to_lowercase().ends_with(".md") {
        return false;
    }
    
    // Include requirements files
    if is_requirements_file(filename) {
        return true;
    }
    
    // Include design specifications, relations documentation, etc.
    let lowercase = filename.to_lowercase();
    lowercase.contains("specification") || 
    lowercase.contains("design") || 
    lowercase.contains("dsd") || 
    lowercase.contains("relation") || 
    lowercase.contains("architecture") || 
    lowercase.contains("story") || 
    lowercase.contains("diagram") || 
    lowercase.contains("model") || 
    lowercase.contains("trace") || 
    // Also include README files as they're often referenced
    lowercase.contains("readme")
}

/// Get relative path from a base directory
pub fn get_relative_path<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: B) -> Result<PathBuf, ReqFlowError> {
    let path = path.as_ref();
    let base = base.as_ref();
    
    match path.strip_prefix(base) {
        Ok(rel_path) => Ok(rel_path.to_path_buf()),
        Err(_) => Err(ReqFlowError::PathError(format!(
            "Failed to get relative path for {:?} from base {:?}",
            path, base
        ))),
    }
}

/// Create directory and any parent directories if they don't exist
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), ReqFlowError> {
    fs::create_dir_all(path.as_ref()).map_err(|e| {
        ReqFlowError::IoError(e)
    })
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

/// Read file contents to string
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, ReqFlowError> {
    fs::read_to_string(path.as_ref()).map_err(|e| {
        ReqFlowError::IoError(e)
    })
}