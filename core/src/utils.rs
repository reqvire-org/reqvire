        use anyhow::Result;
use std::path::{Path, PathBuf};
use pathdiff::diff_paths;
use log::debug;
use walkdir::WalkDir;
use crate::error::ReqFlowError;
use globset::GlobSet;
use regex::Regex;
use rustc_hash::FxHasher;
use std::hash::{Hasher};
use crate::git_commands;


/// Checks if a file should be processed
pub fn is_requirements_file_by_path(path: &Path, excluded_filename_patterns: &GlobSet) -> bool {
    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
       
    filename.ends_with(".md") && !is_excluded_by_patterns(path, &excluded_filename_patterns)
}


/// Checks if a file is excluded based on configured patterns
pub fn is_excluded_by_patterns(path: &Path, excluded_filename_patterns: &GlobSet) -> bool {
    let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if excluded_filename_patterns.is_match(path) || excluded_filename_patterns.is_match(filename) {
        debug!("File '{}' is excluded due to matching a glob pattern.", filename);
        return true;
    }

    false
}


pub fn is_in_specification_root(
    file_folder: &PathBuf,     
    specifications_folder: &PathBuf, 
) -> bool {
    let canonical_file_folder = file_folder.canonicalize().ok();
    let canonical_specifications_folder = specifications_folder.canonicalize().ok();

    match (canonical_file_folder, canonical_specifications_folder) {
        (Some(file), Some(spec)) => file == spec,
        _ => false,
    }
}

/// Scans the specification and external folders for markdown files, excluding files based on patterns.
pub fn scan_markdown_files(
    commit: Option<&str>,
    specification_folder: &PathBuf, 
    external_folders: &[PathBuf], 
    excluded_filename_patterns: &GlobSet
) -> Vec<(PathBuf,PathBuf)> {
    match commit {
        Some(commit_id) => {
            scan_markdown_files_from_commit(
                commit_id,
                specification_folder,
                external_folders,
                excluded_filename_patterns,
            )
        }
        None => {

            let mut files = Vec::new();
            
            // Define all folders to scan
            let all_folders: Vec<&PathBuf> = std::iter::once(specification_folder)
                .chain(external_folders.iter())
                .collect();

            for folder in all_folders {
                for entry in WalkDir::new(folder)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
                    .filter(|e| is_requirements_file_by_path(e.path(), excluded_filename_patterns))
                {
                    files.push((entry.path().to_path_buf(),folder.clone()));
                }
            }

            debug!("Scanned {} markdown files.", files.len());
            files
        }
    }
}

/// Scans the given Git commit for markdown files under the specified folders,
/// excluding files based on provided patterns.
/// 
/// - `commit`: The Git commit (e.g. `"HEAD"`) where we want to look for files.
pub fn scan_markdown_files_from_commit(
    commit: &str,
    specification_folder: &PathBuf,
    external_folders: &[PathBuf],
    excluded_filename_patterns: &GlobSet,
) -> Vec<(PathBuf, PathBuf)> {
    let mut files = Vec::new();

    // Gather all folders to "scan" (from Git's perspective).
    let all_folders: Vec<&PathBuf> = std::iter::once(specification_folder)
        .chain(external_folders.iter())
        .collect();

    // For each folder, we'll run `git ls-tree --name-only -r <commit>`
    // and filter out paths that don't lie within that folder or don't match
    // our markdown + exclusion patterns.
    for folder in all_folders {
        let result = git_commands::ls_tree_commit_in_folder(&commit,&folder);
        let documents_vec = match result {
            Err(e) => {
                //TODO: we need to return result with reqflow error here
                eprintln!("Error: {}", e);
                Vec::new()
            } ,       
            Ok(v) => v
        };

        let matching_paths = documents_vec
        .into_iter() 
        .map(|p| folder.join(p))             
        .filter(|p| p.extension().map_or(false, |ext| ext == "md"))
        .filter(|p| is_requirements_file_by_path(p, excluded_filename_patterns))
        .map(|p| (p, folder.clone()))
        .collect::<Vec<(PathBuf, PathBuf)>>();

        
        files.extend(matching_paths);        

    }

    files
}


/// Gets the relative path of a file
pub fn get_relative_path(path: &PathBuf, specification_folder: &Path, external_folders: &[PathBuf]) -> Result<PathBuf, ReqFlowError> {
    if let Ok(relative) = path.strip_prefix(specification_folder) {
        return Ok(relative.to_path_buf());
    }
    for ext in external_folders {
        if let Ok(relative) = path.strip_prefix(ext) {
            return Ok(relative.to_path_buf());
        }
    }
    Err(ReqFlowError::PathError(format!(
        "Failed to determine relative path: {}",
        path.display()
    )))
}


pub fn get_relative_path_from_root(path: &PathBuf, root: &PathBuf) -> Result<PathBuf, ReqFlowError> {
    path.strip_prefix(root)
        .map(PathBuf::from)
        .map_err(|_| ReqFlowError::PathError(format!(
            "Failed to compute relative path from given root: {}",
            path.display()
        )))
}


/// Splits an identifier into (file_part, Option(fragment)) following these rules:
/// - If the identifier starts with '#' then it is treated as a fragment-only reference 
///   (file_part is empty, and the fragment is the whole identifier without the leading '#').
/// - Otherwise, if the identifier contains a '/' or a '.' (indicating a file extension), 
///   it is treated as a file reference. In that case, if a '#' is present, split at the first '#' 
///   into file_part and fragment; otherwise, fragment is None.
/// - Otherwise (if there is neither '/' nor '.'), treat the entire identifier as a fragment-only reference.
pub fn extract_path_and_fragment(identifier: &str) -> (&str, Option<&str>) {
    if identifier.is_empty(){
        return ("",None);
    }
    if identifier.starts_with('#') {
        let frag = &identifier[1..];
        return ("", Some(frag));
    }
    // If identifier contains a '/' or a '.', assume it's a file reference.
    if identifier.contains('/') || identifier.contains('.') {
        if let Some(idx) = identifier.find('#') {
            let file_part = &identifier[..idx];
            let frag = &identifier[idx + 1..];
            (file_part, Some(frag))
        } else {
            (identifier, None)
        }
    } else {
        // Otherwise, treat as fragment-only.
        ("", Some(identifier))
    }
}
pub fn normalize_fragment(fragment: &str) -> String{
    fragment
        .trim()
        .to_lowercase()
        .replace(' ', "-")     // Replace spaces with hyphens
        .replace(['(', ')', ',', ':'], "") // Remove disallowed characters
}

pub fn normalize_identifier(
    identifier: &str,
    base_path: &PathBuf,
    specifications_folder: &PathBuf, 
    external_folders: &[PathBuf]
) -> Result<String, ReqFlowError> {

    let (path, fragment_opt) = extract_path_and_fragment(identifier);

    
    let result=match normalize_path(path, base_path, specifications_folder, external_folders){
        Ok(normalized_path) => {     
            // Normalize element name into GitHub-style fragment
            if let Some(fragment) = fragment_opt{
                let normalized_fragment =normalize_fragment(&fragment);                
                Ok(format!("{}#{}", normalized_path, normalized_fragment))            
            }else{
                Ok(normalized_path)
            }            
        },
        Err(e) => Err(e)
    };
    
    result

}


/// List of known external URL schemes
const EXTERNAL_SCHEMES: &[&str] = &[
    "http://", "https://", "ftp://", "file://", "mailto:", "ssh://", "git://", "data:",
];

/// Normalize a file path according to the `specifications_folder` and `external_folders`
pub fn normalize_path(
    path_str: &str, 
    base_path: &PathBuf, 
    specifications_folder: &PathBuf, 
    external_folders: &[PathBuf]
) -> Result<String, ReqFlowError> {
   
    let path =Path::new(path_str);

    // Check if the path is an external URL and return it as-is
    if EXTERNAL_SCHEMES.iter().any(|&scheme| path_str.starts_with(scheme)) {
        debug!("Skipping normalization for external URL: {}", path_str);
        return Ok(path_str.to_string());
    }

    // If the path is already absolute, return it but normalize if needed
    if path.is_absolute() {
            
        if let Some(spec_folder_name) = specifications_folder.file_name() {
            let spec_folder_str = spec_folder_name.to_string_lossy();        
            if path_str.starts_with(&format!("/{}", spec_folder_str)) {
                // Remove leading slash and join with specifications_folder
                let full_path = specifications_folder.parent().unwrap().join(path_str.trim_start_matches('/'));
                return Ok(full_path.to_string_lossy().into_owned());
            } 
        }
       
        for external_folder in external_folders {
            if let Some(ext_folder_name) = external_folder.file_name() {
                let ext_folder_str = ext_folder_name.to_string_lossy();
                if path_str.starts_with(&format!("/{}", ext_folder_str)) {
                    // Remove leading slash and join with specifications_folder
                    let full_path = external_folder.parent().unwrap().join(path_str.trim_start_matches('/'));
                    return Ok(full_path.to_string_lossy().into_owned());
                
                }
            }
        }

        return Ok(path_str.to_string());
    }

    // Convert relative path to absolute
    let absolute_path = PathBuf::from(base_path).join(path_str);

    // Explicitly canonicalize the path and handle any errors.
    let canonical_path = match absolute_path.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            return Err(ReqFlowError::PathError(format!(
                "Failed to normalize path'{}': {}",
                absolute_path.to_string_lossy(), e
            )))
        }
    };
    
    
    let normalized_path = canonical_path.to_string_lossy().to_string();
    return Ok(normalized_path);

}


/// Converts a normalized identifier back to a relative path identifier
/// depending on the location of `base_path` and its relation to 
/// `specifications_folder` and `external_folders`.
pub fn to_relative_identifier(
    normalized_identifier: &str,
    base_path: &PathBuf,
    specifications_folder: &PathBuf,
    external_folders: &[PathBuf],
) -> Result<String, ReqFlowError> {

    let (normalized_path, fragment_opt) = extract_path_and_fragment(normalized_identifier);
    let normalized_path = Path::new(normalized_path);

    // Check if the path is an external URL, return as is
    if EXTERNAL_SCHEMES.iter().any(|&scheme| normalized_identifier.starts_with(scheme)) {
        debug!("Skipping denormalization for external URL: {}", normalized_identifier);
        return Ok(normalized_identifier.to_string());
    }

    // Compute relative path if inside `specifications_folder`
    if normalized_path.starts_with(specifications_folder) {
        if let Some(relative_path) = diff_paths(normalized_path, base_path) {
            let relative_identifier = relative_path.to_string_lossy().into_owned();
            return Ok(append_fragment(&relative_identifier, fragment_opt));
        }
    }

    // Compute relative path if inside `external_folders`
    for external_folder in external_folders {
        if normalized_path.starts_with(external_folder) {
            if let Some(relative_path) = diff_paths(normalized_path, base_path) {
                let relative_identifier = relative_path.to_string_lossy().into_owned();
                return Ok(append_fragment(&relative_identifier, fragment_opt));
            }
        }
    }

    // Compute relative path based on `base_path`
    if let Some(relative_path) = diff_paths(normalized_path, base_path) {
        let relative_identifier = relative_path.to_string_lossy().into_owned();
        return Ok(append_fragment(&relative_identifier, fragment_opt));
    }

    // If all else fails, return absolute path
    Ok(append_fragment(&normalized_identifier.to_string(), fragment_opt))
}


/// Helper function to append a fragment (e.g., `#section-name`) to a path if it exists.
fn append_fragment(path: &str, fragment_opt: Option<&str>) -> String {
    match fragment_opt {
        Some(fragment) => format!("{}#{}", path, fragment),
        None => path.to_string(),
    }
}

/// Parses a metadata line and extracts a (key, value) pair if valid.
/// Expected format: `* key: value` or `- key: value`
pub fn parse_metadata_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();

    // Ensure it starts with `* ` or `- `
    if !trimmed.starts_with("* ") && !trimmed.starts_with("- ") {
        return None;
    }

    // Split at the first colon `:` to extract key and value
    if let Some((key, value)) = trimmed[2..].split_once(':') {
        let key = key.trim().to_string(); // Normalize key
        let value = value.trim().to_string(); // Normalize value

        if !key.is_empty() && !value.is_empty() {
            return Some((key, value));
        }
    }

    None
}




pub fn parse_relation_line(line: &str) -> Result<(String, (String, String)), ReqFlowError> {
    let parts: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        let relation_type = parts[0].trim_start_matches("* ").trim().to_string(); // Remove unwanted prefix
        let target = parse_target(parts[1]); // Parse target
        Ok((relation_type, target))
    } else {
        Err(ReqFlowError::InvalidRelationFormat(format!("Invalid relation format: '{}'", line)))
    }
}


/// Parses a given string and creates a RelationTarget based on rules.
fn parse_target(input: &str) -> (String, String) {
    // 1. Check if the input is a Markdown-style link: `[text](link)`
    if let Some((text, link)) = extract_markdown_link(input) {
        return (text,link)
    }

    // 2. If it's a simple identifier-style link (link only), use link as text also.
    (input.to_string(),input.to_string())

}

/// Extracts text and link from a Markdown-style link if present.
fn extract_markdown_link(input: &str) -> Option<(String, String)> {
    let input = input.trim();
    let markdown_regex = Regex::new(r"^\[(.+?)\]\((.+?)\)$").unwrap();
    if let Some(captures) = markdown_regex.captures(input) {
        let text = captures.get(1)?.as_str().to_string();
        let link = captures.get(2)?.as_str().to_string();
        Some((text, link))
    } else {
        None
    }
}


/// Generates a fast and lightweight hash ID
pub fn hash_identifier(identifier: &str) -> String {
    let mut hasher = FxHasher::default();
    hasher.write(identifier.as_bytes());
    format!("{:x}", hasher.finish()).to_string() 
}
pub fn hash_content(content: &str) -> String {
    let mut hasher = FxHasher::default();
    hasher.write(content.as_bytes());
    format!("{:x}", hasher.finish()).to_string() 
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{ PathBuf};
    use tempfile::TempDir;
    
    // Mock Config structure for tests
    struct MockConfig {
        pub paths: MockPaths,
    }
    
    struct MockPaths {
        pub specifications_folder: String,
        pub external_folders: Vec<String>,
        pub base_path: PathBuf,
        pub excluded_filename_patterns: Vec<String>,
    }
    
    impl MockConfig {
        fn default() -> Self {
            Self {
                paths: MockPaths {
                    specifications_folder: "specifications".to_string(),
                    external_folders: Vec::new(),
                    base_path: std::env::current_dir().expect("Failed to get current directory"),
                    excluded_filename_patterns: Vec::new(),
                }
            }
        }
        
        fn get_specification_folder(&self) -> PathBuf {
            self.paths.base_path.join(&self.paths.specifications_folder)
        }
        
        fn get_external_folders(&self) -> Vec<PathBuf> {
            self.paths.external_folders.iter()
                .map(|f| self.paths.base_path.join(f))
                .collect()
        }
        
        fn get_excluded_filename_patterns_glob_set(&self) -> globset::GlobSet {
            let mut builder = globset::GlobSetBuilder::new();
            for pattern in &self.paths.excluded_filename_patterns {
                if let Ok(glob) = globset::Glob::new(pattern) {
                    builder.add(glob);
                }
            }
            builder.build().expect("Failed to build glob set")
        }
    }
    
    
    #[test]
    fn test_extract_path_and_fragment() {
        let test_cases = vec![
            ("/repo/file.md#section", "/repo/file.md", Some("section")),
            ("/repo/path/to/file.md", "/repo/path/to/file.md", None),
            ("/user/repo#readme", "/user/repo", Some("readme")),
            ("/user/repo/", "/user/repo/", None),
            ("File1.md", "File1.md", None),            
            ("onlyfragment", "", Some("onlyfragment")),
            ("#onlyfragment", "", Some("onlyfragment")),
            ("", "", None), // Empty input
        ];

        for (input, expected_path, expected_fragment) in test_cases {
            let (path, fragment) = extract_path_and_fragment(input);
            assert_eq!(path, expected_path, "Failed for input: {:?}", input);
            assert_eq!(fragment, expected_fragment, "Failed for input: {:?}", input);
        }
    }

    
    #[test]
    fn test_normalize_identifier() {
        // Create a temporary directory
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let temp_path = temp_dir.path(); // Get the base path
        
        let base_path = temp_path.to_path_buf();
                    
        let mut config_with_externals = MockConfig::default();
        config_with_externals.paths.external_folders = vec!["./external".to_string()];        
        config_with_externals.paths.specifications_folder = "./specifications".to_string();
        config_with_externals.paths.base_path = base_path;               
                       
                
        let doc1=temp_path.join("specifications/documents/");
                
        // Define test cases with expected real paths
        let root_file=&temp_path.join("File5.md").to_string_lossy().into_owned();
        let test_cases = vec![
            ("File1.md", doc1.to_path_buf().clone(), "specifications/documents/File1.md"),        
            ("File2.md", doc1.to_path_buf().clone(), "specifications/documents/File2.md"),
            ("/specifications/documents/File2.md", doc1.to_path_buf().clone(), "specifications/documents/File2.md"),            
            ("subfolder/File3.md", doc1.to_path_buf().clone(), "specifications/documents/subfolder/File3.md"),
            ("../File4.md", doc1.to_path_buf().clone(), "specifications/File4.md"),
            ("../../somefolder/File4.md",doc1.to_path_buf().clone(), "somefolder/File4.md"),
            ("/external/somefolder/File4.md",doc1.to_path_buf().clone(), "external/somefolder/File4.md"),
            (&root_file, doc1.to_path_buf().clone(), "File5.md"),
            //end with fragments
            ("File1.md#Some Fragment", doc1.to_path_buf().clone(), "specifications/documents/File1.md#some-fragment"),                    
            ("/specifications/documents/File2.md#Some Test", doc1.to_path_buf().clone(), "specifications/documents/File2.md#some-test"),                        
            
        ];  
        
        for (_, _, file_path) in &test_cases {
            let full_file_path = temp_path.join(file_path);
        
            if let Some(parent) = full_file_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create directories");
            }
            fs::write(&full_file_path, "Test content").expect("Failed to create test file");
        }

        for (identifier, in_document, expected) in test_cases {
            let result = normalize_identifier(identifier, &in_document, &config_with_externals.get_specification_folder(),&config_with_externals.get_external_folders());
            
            // Doing canon because of macos tmp path symlinks, question is if code should work if we are adjusting tests??
            let expected_path = temp_path.join(expected);
            let expected_path_canon = expected_path.canonicalize().unwrap();
            let expected_canon = expected_path_canon.to_string_lossy();

            let result_str = result.unwrap(); // assuming it's a String
            let result_path = std::fs::canonicalize(&result_str).unwrap();
            let result_canon = result_path.to_string_lossy();
    
            assert_eq!(
                &result_canon,
                &expected_canon,
                "Failed for identifier: {}",
                identifier
            );


        }
        
    
    }


 
    // Test the is_requirements_file_by_path function
    #[test]
    fn test_is_requirements_file_by_path() {
     
        // Configure external folders for these tests
        let mut config_with_externals = MockConfig::default();
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
            assert!(is_requirements_file_by_path(&path, &config_with_externals.get_excluded_filename_patterns_glob_set()), 
                    "Expected {} to be identified as a requirements file", path_str);
        }
        
        // Test that non-requirements files are properly excluded
        for path_str in non_req_file_cases {
            let path = PathBuf::from(path_str);
            // Skip tests if the path doesn't exist
            if !path.exists() {
                continue;
            }
            assert!(!is_requirements_file_by_path(&path, &config_with_externals.get_excluded_filename_patterns_glob_set()), 
                    "Expected {} to NOT be identified as a requirements file", path_str);
        }
    }
    
    // Test excluded_filename_patterns
    #[test]
    fn test_excluded_filename_patterns() {
        //let _ = env_logger::builder().is_test(true).try_init();   

        // Configure external folders for these tests
        let mut config_with_externals = MockConfig::default();
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
            let matches_pattern = is_excluded_by_patterns(&test_path, &config_with_externals.get_excluded_filename_patterns_glob_set());                

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
    

    #[test]
    fn test_parse_target_cases() {
        let test_cases = vec![
            // Markdown External URL
            ("[OpenAI](https://openai.com)", "OpenAI", "https://openai.com"),
            
            // Markdown Internal Identifier
            ("[Docs](some-identifier)", "Docs", "some-identifier"),

            // Plain External URL
            ("https://github.com/user/repo", "https://github.com/user/repo", "https://github.com/user/repo"),

            // Plain Internal Identifier
            ("some-identifier", "some-identifier", "some-identifier"),

            // File URL (External)
            ("file:///usr/local/docs.txt", "file:///usr/local/docs.txt", "file:///usr/local/docs.txt"),

            // FTP Link (External)
            ("ftp://example.com/file.zip", "ftp://example.com/file.zip", "ftp://example.com/file.zip"),

            // Mailto Link (External)
            ("mailto:user@example.com", "mailto:user@example.com", "mailto:user@example.com"),

            // Git Repository Link (External)
            ("git://github.com/user/repo.git", "git://github.com/user/repo.git", "git://github.com/user/repo.git"),

            // SSH Link (External)
            ("ssh://192.168.1.1", "ssh://192.168.1.1", "ssh://192.168.1.1"),

            // Data URI (External)
            ("data:image/png;base64,XYZ", "data:image/png;base64,XYZ", "data:image/png;base64,XYZ"),

            // Internal Path Reference
            ("/docs/reference.md", "/docs/reference.md", "/docs/reference.md"),

            // Markdown Internal Path
            ("[Reference](/docs/ref.md)", "Reference", "/docs/ref.md"),
        ];

        for (input, expected_text, expected_link) in test_cases {
            let (text, link) = parse_target(input);
            assert_eq!(text, expected_text, "Failed on input: {}", input);

            match (&link, &expected_link) {
                (actual, expected) => {
                    assert_eq!(actual, expected, "Failed on Identifier input: {}", input);
                }
                (actual, expected) => {
                    assert_eq!(actual, expected, "Failed on External URL input: {}", input);
                }
                _ => panic!("Mismatch for input: {}. Expected: {:?}, Got: {:?}", input, expected_link, link),
            }
        }
    }
    
    #[test]
    fn test_to_relative_identifier() {

        let temp_spec_folder = TempDir::new().expect("Failed to create temp dir");
        let specifications_folder = temp_spec_folder.path().to_path_buf();

        let spec_folder_name = specifications_folder.file_name()
            .and_then(|name| name.to_str()) 
            .unwrap_or("<unknown>");


        let temp_external_folder =  TempDir::new().expect("Failed to create temp dir");
        let external_folder = temp_external_folder.path().to_path_buf();
        let external_folders = vec![external_folder.clone()];
        
        
        let base_path = specifications_folder.join("in/side/");
        let base_path_in_ext = external_folder.join("in/side/");


        // Test external URL (should be returned as-is)
        let external_url = "http://example.com/path/to/spec";
        let result = to_relative_identifier(external_url, &base_path, &specifications_folder, &external_folders)
            .expect("Should return external URL as-is");
        assert_eq!(result, external_url, "Failed external URL check");

        // Test path inside specifications_folder
        /*
            When specification folder is "/tmp/spec/" and indentifier is "/tmp/spec/subfolder/file.yaml"
            then for the base_path "/tmp/spec/in/side/":
                * relative identifier must be relative reference from base_path: "../../subfolder/file.yaml"
                
            Same functionality applies when identifier path and base_path are inside external folder
        
        */
        let spec_path = specifications_folder.join("subfolder/file.yaml");
        let spec_identifier = spec_path.to_string_lossy().to_string();
        let result = to_relative_identifier(&spec_identifier, &base_path, &specifications_folder, &external_folders)
            .expect("Should return relative path inside specifications folder");
        assert_eq!(result, "../../subfolder/file.yaml", "Failed specifications folder check");

        // Test path in same folder as specification folder
        let spec_path = specifications_folder.join("file.yaml");
        let spec_identifier = spec_path.to_string_lossy().to_string();
        let result = to_relative_identifier(&spec_identifier, &specifications_folder, &specifications_folder, &external_folders)
            .expect("Should return relative path inside specifications folder");
        assert_eq!(result, "file.yaml", "Failed specifications folder check");


        // Test path inside external_folders        
        let external_path = external_folder.join("external_spec.yaml");
        let external_identifier = external_path.to_string_lossy().to_string();
       
        let result = to_relative_identifier(&external_identifier, &base_path_in_ext, &specifications_folder, &external_folders)
            .expect("Should return relative path inside external folders");
        assert_eq!(result, "../../external_spec.yaml", "Failed external folder check");


        // Test path inside external_folders with identifer in specification folder      
        /*
            When external folder is "/tmp/external/" and indentifier is pointing to file in specification folder "/tmp/spec/subfolder/file.yaml"
            then for the base_path in external folder  "/tmp/external/in/side/":
                * relative identifier must be relative reference from base_path: "../../../spec/subfolder/file.yaml"
                
        */
        let spec_path = specifications_folder.join("subfolder/file.yaml");
        let spec_identifier = spec_path.to_string_lossy().to_string();
        
        let result = to_relative_identifier(&spec_identifier, &base_path_in_ext, &specifications_folder, &external_folders)
            .expect("Should return relative path inside specifications folder");
        assert_eq!(result, format!("../../../{}/subfolder/file.yaml",spec_folder_name), "Failed specifications folder check");
        
        
        // Test absolute path that does not match any provided folders
        // 
        let unmatched_path = "/some/other/path/spec.yaml".to_string();
        let result = to_relative_identifier(&unmatched_path, &base_path, &specifications_folder, &external_folders)
            .expect("Should return absolute path when no match is found");

        // Dynamically compute the expected path (resutl must be a path relative to a base_path)
        let expected = pathdiff::diff_paths(&unmatched_path, &base_path)
            .expect("Failed to compute relative path");

        assert_eq!(result, expected.to_string_lossy(), "Failed unmatched absolute path check");            
    }    
}
