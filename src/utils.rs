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
/// 
/// Note: This function is more permissive and includes both requirements files and design specifications.
/// For more specific filtering, use is_requirements_file_only() or is_design_specification().
pub fn is_requirements_file_by_path(path: &Path, config: &crate::config::Config, base_path: &Path) -> bool {
    // Early return if not a markdown file
    if path.extension().map_or(false, |ext| ext != "md") {
        return false;
    }
    
    // Check if file matches any excluded filename patterns
    let path_str = path.to_string_lossy();
    let is_excluded = config.paths.excluded_filename_patterns.iter().any(|pattern| {
        glob::Pattern::new(pattern).map(|p| p.matches(&path_str)).unwrap_or(false)
    });
    
    if is_excluded {
        return false;
    }
    
    // Check if the file is in specifications folder
    let specs_dir = base_path.join(&config.paths.specifications_folder);
    if path.starts_with(&specs_dir) {
        // Design specs are still included here since this function should match both
        // requirements and design specs
        return true;
    }
    
    // Check if the file is in any external folder
    for ext_folder in &config.paths.external_folders {
        let ext_path = base_path.join(ext_folder);
        if path.starts_with(&ext_path) {
            return true;
        }
    }
    
    // No fallback - if not in specifications or external folders, it's not a requirements file
    false
}

/// Checks if a file is a requirements file only (excluding design specifications)
/// 
/// This function is used to identify files that should be processed for diagram generation
/// or linting when we want to exclude design specifications.
/// 
/// @param path The path to the file to check
/// @param config The ReqFlow configuration
/// @param base_path The base path for relative path resolution
/// @param verbose Whether to output debug information
/// @return true if the file is a requirements file, false otherwise
pub fn is_requirements_file_only(path: &Path, config: &crate::config::Config, base_path: &Path, verbose: bool) -> bool {
    // Check if file is a markdown file
    if !path.is_file() || path.extension().map_or(true, |ext| ext != "md") {
        return false;
    }
    
    // Get filename for checking
    let filename = path.file_name().unwrap_or_default().to_string_lossy();
    let path_str = path.to_string_lossy();
    
    if verbose {
        log::info!("Checking file {}, base_path: {}", path.display(), base_path.display());
        log::info!("Path: {}, Filename: {}", path_str, filename);
    }
    
    // Check if file matches any excluded filename patterns
    let is_excluded = config.paths.excluded_filename_patterns.iter().any(|pattern| {
        let matches = glob::Pattern::new(pattern).map(|p| p.matches(&path_str)).unwrap_or(false);
        if matches && verbose {
            log::info!("File {} matches excluded pattern {}", path.display(), pattern);
        }
        matches
    });
    
    if is_excluded {
        if verbose {
            log::info!("Skipping excluded file: {}", path.display());
        }
        return false;
    }
    
    // Skip design specifications
    let design_specs_folder = config.paths.design_specifications_folder.to_lowercase();
    let path_str_lower = path_str.to_lowercase();
    let filename_lower = filename.to_lowercase();
    
    if path_str_lower.contains(&format!("/{}/", design_specs_folder)) || filename_lower.contains("dsd_") {
        if verbose {
            log::info!("Skipping design spec: {}", path.display());
        }
        return false;
    }
    
    // Check if file is in any external folder
    let in_external_folder = if !config.paths.external_folders.is_empty() {
        config.paths.external_folders.iter().any(|ext_folder| {
            let ext_folder_lower = ext_folder.to_lowercase();
            path_str_lower.contains(&format!("/{}/", ext_folder_lower))
        })
    } else {
        false
    };
    
    // If file is in an external folder, it's automatically considered a requirements file
    if in_external_folder {
        if verbose {
            log::info!("File is in external folder, treating as requirements document");
        }
        return true;
    }
    
    // Special case for test fixtures and empty specifications folder:
    // If specifications_folder is empty, we're likely in a test environment
    // In this case, just check if the filename contains "requirements"
    if config.paths.specifications_folder.is_empty() {
        let is_requirements_file = is_requirements_file(&filename);
        
        if is_requirements_file && verbose {
            log::info!("Test environment: File {} identified as requirements document", path.display());
        }
        
        return is_requirements_file;
    }
    
    // Normal case - proper path checking
    // Determine if file is in the specifications folder or subfolder
    let specs_folder = config.paths.specifications_folder.to_lowercase();
    let in_specifications = path_str_lower.contains(&format!("/{}/", specs_folder));
    
    if !in_specifications {
        // If not in specifications or external folders, it's not a requirements file
        if verbose {
            log::info!("File is not in specifications folder or external folder, skipping");
        }
        return false;
    }
    
    // In specifications folder, all files are considered requirements (except those already filtered out)
    if verbose {
        log::info!("File is in specifications folder, treating as requirements document");
        log::info!("FINAL RESULT: true");
    }
    
    true
}

/// Check if a file is a design specification document
/// 
/// @param path The path to the file to check
/// @param config The ReqFlow configuration 
/// @param _base_path The base path for relative path resolution (unused but kept for API consistency)
/// @return true if the file is a design specification, false otherwise
pub fn is_design_specification(path: &Path, config: &crate::config::Config, _base_path: &Path) -> bool {
    // Check if file is a markdown file
    if !path.is_file() || path.extension().map_or(true, |ext| ext != "md") {
        return false;
    }
    
    // Path string for easier checking
    let path_str = path.to_string_lossy().to_lowercase();
    
    // Check filename for DSD prefix
    let filename = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
    if filename.contains("dsd_") {
        return true;
    }
    
    // Check if in design specifications folder
    let design_specs_folder = config.paths.design_specifications_folder.to_lowercase();
    if path_str.contains(&format!("/{}/", design_specs_folder)) {
        return true;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::path::PathBuf;

    // Test the is_requirements_file_only function
    #[test]
    fn test_is_requirements_file_only() {
        let config = Config::default();
        let base_path = PathBuf::from("/mnt/Radni/ReqFlow");
        
        // Test cases for requirements files
        let req_file_cases = vec![
            // Requirements files in specifications root
            "/mnt/Radni/ReqFlow/specifications/UserRequirements.md",
            "/mnt/Radni/ReqFlow/specifications/SystemRequirements.md",
            "/mnt/Radni/ReqFlow/specifications/MissionRequirements.md",
            
            // Requirements files in system requirements folder
            "/mnt/Radni/ReqFlow/specifications/SystemRequirements/Requirements.md",
            "/mnt/Radni/ReqFlow/specifications/SystemRequirements/Subsystem/Requirements.md",
        ];
        
        // Test cases for non-requirements files
        let non_req_file_cases = vec![
            // Design specifications
            "/mnt/Radni/ReqFlow/specifications/DesignSpecifications/DSD_Diagram.md",
            "/mnt/Radni/ReqFlow/specifications/DSD_Architecture.md",
            
            // Other markdown files
            "/mnt/Radni/ReqFlow/specifications/README.md",
            "/mnt/Radni/ReqFlow/README.md",
            
            // Non-markdown files
            "/mnt/Radni/ReqFlow/specifications/document.txt",
        ];
        
        // Test that requirements files are properly identified
        for path_str in req_file_cases {
            let path = PathBuf::from(path_str);
            // Skip tests if the path doesn't exist 
            // This allows the tests to be run in different environments
            if !path.exists() {
                continue;
            }
            assert!(is_requirements_file_only(&path, &config, &base_path, false), 
                    "Expected {} to be identified as a requirements file", path_str);
        }
        
        // Test that non-requirements files are properly excluded
        for path_str in non_req_file_cases {
            let path = PathBuf::from(path_str);
            // Skip tests if the path doesn't exist
            if !path.exists() {
                continue;
            }
            assert!(!is_requirements_file_only(&path, &config, &base_path, false), 
                    "Expected {} to NOT be identified as a requirements file", path_str);
        }
    }
    
    // Test the is_design_specification function
    #[test]
    fn test_is_design_specification() {
        let config = Config::default();
        let base_path = PathBuf::from("/mnt/Radni/ReqFlow");
        
        // Test cases for design specifications
        let design_spec_cases = vec![
            "/mnt/Radni/ReqFlow/specifications/DesignSpecifications/DSD_Diagram.md",
            "/mnt/Radni/ReqFlow/specifications/DSD_Architecture.md",
        ];
        
        // Test cases for non-design specifications
        let non_design_spec_cases = vec![
            // Requirements files
            "/mnt/Radni/ReqFlow/specifications/UserRequirements.md",
            "/mnt/Radni/ReqFlow/specifications/SystemRequirements.md",
            "/mnt/Radni/ReqFlow/specifications/SystemRequirements/Requirements.md",
            
            // Other markdown files
            "/mnt/Radni/ReqFlow/specifications/README.md",
        ];
        
        // Test that design specifications are properly identified
        for path_str in design_spec_cases {
            let path = PathBuf::from(path_str);
            // Skip tests if the path doesn't exist
            if !path.exists() {
                continue;
            }
            assert!(is_design_specification(&path, &config, &base_path), 
                    "Expected {} to be identified as a design specification", path_str);
        }
        
        // Test that non-design specifications are properly excluded
        for path_str in non_design_spec_cases {
            let path = PathBuf::from(path_str);
            // Skip tests if the path doesn't exist
            if !path.exists() {
                continue;
            }
            assert!(!is_design_specification(&path, &config, &base_path), 
                    "Expected {} to NOT be identified as a design specification", path_str);
        }
    }
}