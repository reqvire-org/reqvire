use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use glob::Pattern;
use crate::config::Config;
use log::debug;

use crate::error::ReqFlowError;


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



/// Check if a file is a processable file based on its path and configuration
pub fn is_requirements_file_by_path(path: &Path, config: &crate::config::Config) -> bool {
    // Early return if not a markdown file
    if path.extension().map_or(false, |ext| ext != "md") {
        return false;
    }
    
    // Check if file matches any excluded filename patterns
    let is_excluded = is_excluded_by_patterns(path, &config, false);
    
    if is_excluded {
        return false;
    }
      
    is_relevant_file(path,&config)
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


/// Normalize a path string for consistent registry lookups.
/// - **Relative paths (`../`)** → Resolved based on `current_file`.
/// - **Fragment-only paths (`#fragment`)** → Prefixed with `current_file`.
/// - **Other paths remain unchanged** (no conversion to absolute).
pub fn normalize_path(path: &str, config: &crate::config::Config, current_file: &str) -> String {
    if path.is_empty() {
        return "".to_string();
    }

    // Handle fragment-only references like `#fragment`
    if path.starts_with("#") {
        let current_file_normalized = if current_file.is_empty() {
            "".to_string()
        } else {
            normalize_path(current_file, config, "")
        };
        return format!("{}{}", current_file_normalized, path);
    }

    // Split path and fragment
    let (base_path, fragment) = if let Some(fragment_idx) = path.find('#') {
        (path[..fragment_idx].to_string(), path[fragment_idx..].to_string())
    } else {
        (path.to_string(), "".to_string())
    };

    let path_obj = Path::new(&base_path);

    // Handle relative paths with `../`
    if path_obj.components().any(|c| c == std::path::Component::ParentDir) {
        if current_file.is_empty() {
            return format!("{}{}", base_path, fragment);
        }
        let current_dir = Path::new(current_file).parent().unwrap_or(Path::new(""));
        let resolved_path = current_dir.join(path_obj).canonicalize().unwrap_or_else(|_| current_dir.join(path_obj));
        return format!("{}{}", resolved_path.to_string_lossy(), fragment);
    }

    // Other paths remain unchanged
    format!("{}{}", base_path, fragment)
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

/// Normalize a string to GitHub-style fragment identifier
/// This follows GitHub's rules for converting headings to link anchors:
/// 1. Convert to lowercase
/// 2. Replace spaces with hyphens
/// 3. Remove disallowed characters
/// 4. Remove leading and trailing whitespace
/// 
/// This function should be used consistently across the codebase whenever
/// creating or looking up fragment identifiers.
pub fn normalize_fragment(text: &str) -> String {
    // Trim whitespace
    let trimmed = text.trim();
    
    // Convert to lowercase
    let lowercase = trimmed.to_lowercase();
    
    // Replace spaces with hyphens
    let with_hyphens = lowercase.replace(' ', "-");
    
    // Remove any disallowed characters (keeping only alphanumeric and hyphens)
    let normalized = with_hyphens
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();
    
    normalized
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
