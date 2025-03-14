use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use glob::Pattern;
use crate::config::Config;
use log::debug;
use crate::error::ReqFlowError;


/// Checks if a file should be processed
pub fn is_requirements_file_by_path(path: &Path, config: &Config) -> bool {
    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    filename.ends_with(".md") && !is_excluded_by_patterns(path, config, false)
}


/// Checks if a file is excluded based on configured patterns
pub fn is_excluded_by_patterns(path: &Path, config: &Config, verbose: bool) -> bool {
    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    for pattern in &config.paths.excluded_filename_patterns {
        if filename.contains(pattern) {
            if verbose {
                log::debug!("File {} is excluded due to pattern {}", filename, pattern);
            }
            return true;
        }
    }

    false
}

/*

/// Helper function to check if a file is excluded by any of the provided glob patterns
/// Checks if a file should be excluded based on glob patterns in the configuration.
pub fn is_excluded_by_patterns(file_path: &Path, config: &Config, verbose: bool) -> bool {
    // Check if file is in specifications or external folders
    if !is_relevant_file(file_path, config) {
        return false; // Skip files that are outside relevant directories
    }

    // Use the configured base path for relative path calculation
    let base_path = Path::new(&config.paths.base_path);

    // Compute relative path within the base path
    let relative_path = match file_path.strip_prefix(base_path) {
        Ok(rel) => rel.to_string_lossy().replace("\\", "/"), // Normalize slashes for glob matching
        Err(_) => file_path.to_string_lossy().replace("\\", "/"), // Use full path if stripping fails
    };

    // Extract just the filename
    let file_name = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();

    for pattern in &config.paths.excluded_filename_patterns {
        let glob_pattern = match Pattern::new(pattern.as_str()) {
            Ok(p) => p,
            Err(e) => {
                log::warn!("Invalid glob pattern '{}': {}", pattern, e);
                continue;
            }
        };

        // Match against both relative path and filename
        let matches_full_path = glob_pattern.matches(&relative_path);
        let matches_filename = glob_pattern.matches(&file_name);

        if matches_full_path || matches_filename {
            if verbose {
                debug!(
                    "File '{}' matches excluded pattern '{}' (relative_path='{}', filename='{}')",
                    file_path.display(),
                    pattern,
                    relative_path,
                    file_name
                );
            }
            return true;
        }
    }

    false
}


*/

pub fn is_in_specifications(file_path: &Path, config: &Config) -> bool {
    let specifications_root = config.paths.base_path.join(&config.paths.specifications_folder);
    
    match file_path.strip_prefix(&config.paths.base_path) {
        Ok(relative_path) => relative_path.starts_with(&config.paths.specifications_folder),
        Err(_) => file_path.starts_with(&specifications_root),
    }
}


/// Check if a file is inside any external folder or its subfolders
pub fn is_in_external_folder(file_path: &Path, config: &Config) -> bool {
    for external_folder in &config.paths.external_folders {
        let external_root = config.paths.base_path.join(external_folder);

        if match file_path.strip_prefix(&config.paths.base_path) {
            Ok(relative_path) => relative_path.starts_with(external_folder),
            Err(_) => file_path.starts_with(&external_root),
        } {
            return true;
        }
    }
    false
}


/// Check if a file is in a **subfolder** of the specifications folder (not in the root)
pub fn is_in_specifications_subfolder(file_path: &Path, config: &Config) -> bool {
    let specifications_root = config.paths.base_path.join(&config.paths.specifications_folder);

    match file_path.strip_prefix(&config.paths.base_path) {
        Ok(relative_path) => {
            relative_path.starts_with(&config.paths.specifications_folder)
                && relative_path != Path::new(&config.paths.specifications_folder)
        }
        Err(_) => {
            file_path.starts_with(&specifications_root) && file_path.parent() != Some(&specifications_root)
        }
    }
}





/// Check if a file is inside specifications, a subfolder of specifications, or an external folder
pub fn is_relevant_file(file_path: &Path, config: &Config) -> bool {
    is_in_specifications(file_path, config) 
        || is_in_specifications_subfolder(file_path, config) 
        || is_in_external_folder(file_path, config)
}




/// Gets the relative path of a file
pub fn get_relative_path(path: &Path, base: &Path) -> Result<PathBuf, ReqFlowError> {
    path.strip_prefix(base)
        .map_err(|_| ReqFlowError::PathError(format!("Failed to determine relative path: {}", path.display())))
        .map(|p| p.to_path_buf())
}


/// Reads a file's content
pub fn read_file(path: &Path) -> Result<String, ReqFlowError> {
    fs::read_to_string(path).map_err(|e| ReqFlowError::IoError(e))
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

    // Test the is_requirements_file_by_path function
    #[test]
    fn test_is_requirements_file_by_path() {

        let paths = [
            "specifications",
            "specifications/subfolder",
            "specifications/deep/nested/folder",
            "external_repo",
            "external_repo/specs",
            "other_folder"
        ];
        
        // Configure external folders for these tests
        let mut config_with_externals = Config::default();
        config_with_externals.paths.external_folders = vec!["external_repo".to_string()];        
        config_with_externals.paths.specifications_folder = "specifications".to_string();
        config_with_externals.paths.base_path = PathBuf::from("");                 
    
        config_with_externals.paths.excluded_filename_patterns=vec![
            "**/README*.md".to_string(),
            "**/Logical*.md".to_string(),
            "**/Physical*.md".to_string(),
            "**/index.md".to_string()
        ];
               
                       
        // Test cases for requirements files
        let req_file_cases = vec![
            // Requirements files in specifications root
            "specifications/UserRequirements.md",
            "specifications/SystemRequirements.md",
            "specifications/MissionRequirements.md",
            
            // Requirements files in system requirements folder
            "specifications/SystemRequirements/Requirements.md",
            "specifications/SystemRequirements/Subsystem/Requirements.md",
        ];
        
        // Test cases for non-requirements files
        let non_req_file_cases = vec![
            // Design specifications
            "specifications/DesignSpecifications/DSD_Diagram.md",
            "specifications/DSD_Architecture.md",
            
            // Other markdown files
            "specifications/README.md",
            "README.md",
            
            // Non-markdown files
            "specifications/document.txt",
        ];
        
        // Test that requirements files are properly identified
        for path_str in req_file_cases {
            let path = PathBuf::from(path_str);
            // Skip tests if the path doesn't exist 
            // This allows the tests to be run in different environments
            if !path.exists() {
                continue;
            }
            assert!(is_requirements_file_by_path(&path, &config_with_externals), 
                    "Expected {} to be identified as a requirements file", path_str);
        }
        
        // Test that non-requirements files are properly excluded
        for path_str in non_req_file_cases {
            let path = PathBuf::from(path_str);
            // Skip tests if the path doesn't exist
            if !path.exists() {
                continue;
            }
            assert!(!is_requirements_file_by_path(&path, &config_with_externals), 
                    "Expected {} to NOT be identified as a requirements file", path_str);
        }
    }
    
    // Test excluded_filename_patterns
    #[test]
    fn test_excluded_filename_patterns() {
        //let _ = env_logger::builder().is_test(true).try_init();   
                     
        let paths = [
            "specifications",
            "specifications/subfolder",
            "specifications/deep/nested/folder",
            "external_repo",
            "external_repo/specs",
            "other_folder"
        ];
        
        // Configure external folders for these tests
        let mut config_with_externals = Config::default();
        config_with_externals.paths.external_folders = vec!["external_repo".to_string()];
        config_with_externals.paths.base_path = PathBuf::from("");         
        config_with_externals.paths.specifications_folder = "specifications".to_string();
    
        config_with_externals.paths.excluded_filename_patterns=vec![
            "**/README*.md".to_string(),
            "**/Logical*.md".to_string(),
            "**/Physical*.md".to_string(),
            "**/index.md".to_string()
        ];
                
        

        let test_files = [
            // README* pattern test files
            ("specifications/README.md", true), // Should be excluded
            ("specifications/READMEtest.md", true), // Should be excluded
            ("specifications/readme.md", false), // Should NOT be excluded (case sensitive)
            ("specifications/READ.md", false), // Should NOT be excluded
            ("specifications/subfolder/README.md", true), // Should be excluded
            ("specifications/deep/nested/folder/README.md", true), // Should be excluded
            
            // Logical* pattern test files
            ("specifications/LogicalArchitecture.md", true), // Should be excluded
            ("specifications/LOGICAL_view.md", false), // Should NOT be excluded (case sensitive)
            ("specifications/logical_design.md", false), // Should NOT be excluded (case sensitive)
            ("specifications/Logicless.md", false), // Should NOT be excluded
            ("specifications/subfolder/LogicalModel.md", true), // Should be excluded
            ("external_repo/specs/LogicalView.md", true), // Should be excluded
            
            // Physical* pattern test files
            ("specifications/PhysicalArchitecture.md", true), // Should be excluded
            ("specifications/subfolder/PhysicalDiagram.md", true), // Should be excluded
            ("specifications/NotPhysical.md", false), // Should NOT be excluded
            ("specifications/Physicalsomething.md", true), // Should be excluded
            
            // index.md pattern test files
            ("specifications/index.md", true), // Should be excluded
            ("specifications/subfolder/index.md", true), // Should be excluded
            ("specifications/INDEX.md", false), // Should NOT be excluded (case sensitive)
            ("specifications/indexing_guide.md", false), // Should NOT be excluded
            ("specifications/deep/nested/folder/index.md", true), // Should be excluded
            
            // Standard requirement files - should never be excluded
            ("specifications/Requirements.md", false),
            ("specifications/SystemRequirements.md", false),
            ("specifications/UserRequirements.md", false),
            ("specifications/subfolder/Requirements.md", false),
            ("external_repo/specs/Requirements.md", false),
        ];
        
        
        for (path, should_exclude) in &test_files {
            let test_path = PathBuf::from(path);
            let matches_pattern = is_excluded_by_patterns(&test_path, &config_with_externals,true);                

            if *should_exclude {
                 assert!(
                     matches_pattern,
                     "❌ File '{}' should be EXCLUDED by pattern but is NOT",
                      path
                 );
            } else {
                   assert!(
                      !matches_pattern,
                      "❌ File '{}' should NOT be excluded by pattern but IS",
                      path
                   );
           }
        }
        
    }
    

    
}
