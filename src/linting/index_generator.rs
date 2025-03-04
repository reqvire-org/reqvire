use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use log::{debug, info, warn};
use walkdir::WalkDir;

use crate::config::Config;
use crate::error::ReqFlowError;

/// Generate an index.md file in the specifications folder
pub fn generate_index(directory: &Path, config: &Config) -> Result<(), ReqFlowError> {
    info!("Generating index.md in specifications folder");
    
    // Get the specifications folder path
    let specs_dir_path = directory.join(&config.paths.specifications_folder);
    
    // Special case for testing - if input_folder is already "specifications", don't add it again
    let actual_specs_dir_path = if directory.file_name().map_or(false, |name| name == "specifications") {
        directory.to_path_buf()
    } else {
        specs_dir_path.clone()
    };
    
    debug!("Using specifications directory: {:?}", actual_specs_dir_path);
    
    if !actual_specs_dir_path.exists() || !actual_specs_dir_path.is_dir() {
        warn!("Specifications directory not found: {:?}", actual_specs_dir_path);
        return Err(ReqFlowError::ValidationError(
            format!("Specifications directory '{}' not found for index generation", 
                   actual_specs_dir_path.display())
        ));
    }
    
    // Create a map to store directory structures
    let mut directory_map: HashMap<String, Vec<FileInfo>> = HashMap::new();
    
    // Process all markdown files in the specifications directory
    for entry in WalkDir::new(&actual_specs_dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let file_path = entry.path();
        
        // Skip the index.md file itself if it already exists
        if file_path.file_name().map_or(false, |name| name == "index.md") {
            continue;
        }
        
        // Get the relative path from specifications root
        let relative_path = file_path.strip_prefix(&actual_specs_dir_path)
            .unwrap_or(file_path)
            .to_path_buf();
        
        // Get the parent directory (category)
        let parent_dir = match relative_path.parent() {
            Some(dir) => dir.to_string_lossy().to_string(),
            None => String::new() // Root directory
        };
        
        // Read the file to extract summary
        if let Ok(content) = fs::read_to_string(file_path) {
            let summary = extract_summary(&content);
            let file_name = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();
            
            // Add to the directory map
            directory_map.entry(parent_dir)
                .or_insert_with(Vec::new)
                .push(FileInfo {
                    file_name,
                    relative_path,
                    summary,
                });
        }
    }
    
    // Generate the index content
    let index_content = generate_index_content(&directory_map);
    
    // Write the index.md file
    let index_path = actual_specs_dir_path.join("index.md");
    let mut file = fs::File::create(&index_path)
        .map_err(|e| ReqFlowError::IoError(e))?;
    
    file.write_all(index_content.as_bytes())
        .map_err(|e| ReqFlowError::IoError(e))?;
    
    info!("Generated index.md at {:?}", index_path);
    Ok(())
}

/// Information about a file for the index
struct FileInfo {
    file_name: String,
    relative_path: PathBuf,
    summary: String,
}

/// Extract a summary from markdown content
fn extract_summary(content: &str) -> String {
    // Try to find the first level 1 heading
    let title_regex = regex::Regex::new(r"^#\s+(.+)$").unwrap();
    
    // First try to get the title from a level 1 heading
    if let Some(captures) = title_regex.captures(content) {
        if let Some(title_match) = captures.get(1) {
            let title = title_match.as_str().trim();
            
            // Try to find the first paragraph after the title
            let paragraphs: Vec<&str> = content.split("\n\n").collect();
            if paragraphs.len() > 1 {
                // Get the first non-empty paragraph after the title
                for paragraph in &paragraphs[1..] {
                    let cleaned = paragraph.trim();
                    if !cleaned.is_empty() && !cleaned.starts_with('#') {
                        // Truncate if too long and add ellipsis
                        if cleaned.len() > 150 {
                            return format!("{}: {:.147}...", title, cleaned);
                        } else {
                            return format!("{}: {}", title, cleaned);
                        }
                    }
                }
            }
            
            // If no paragraph found, return just the title
            return title.to_string();
        }
    }
    
    // If no title, try to get the first non-empty paragraph
    let paragraphs: Vec<&str> = content.split("\n\n").collect();
    for paragraph in paragraphs {
        let cleaned = paragraph.trim();
        if !cleaned.is_empty() && !cleaned.starts_with('#') {
            // Truncate if too long and add ellipsis
            if cleaned.len() > 200 {
                return format!("{:.197}...", cleaned);
            } else {
                return cleaned.to_string();
            }
        }
    }
    
    // Fallback if no suitable content found
    "No description available".to_string()
}

/// Generate the content for the index.md file
fn generate_index_content(directory_map: &HashMap<String, Vec<FileInfo>>) -> String {
    let mut content = String::new();
    
    // Add index title
    content.push_str("# ReqFlow Specifications Index\n\n");
    content.push_str("This index provides a structured overview of all specification documents.\n\n");
    
    // Add root documents first
    if let Some(root_files) = directory_map.get("") {
        content.push_str("## Root Documents\n\n");
        
        for file in root_files {
            let file_path = file.relative_path.to_string_lossy();
            content.push_str(&format!("- [{}]({}) - {}\n", 
                file.file_name,
                file_path, // Markdown link
                file.summary
            ));
        }
        
        content.push_str("\n");
    }
    
    // Process subdirectories (sorted alphabetically)
    let mut sorted_dirs: Vec<&String> = directory_map.keys()
        .filter(|k| !k.is_empty())
        .collect();
    sorted_dirs.sort();
    
    for dir in sorted_dirs {
        // Create a section for each directory
        let dir_display = if dir.is_empty() { "Root" } else { dir };
        content.push_str(&format!("## {}\n\n", dir_display));
        
        if let Some(files) = directory_map.get(dir) {
            // Sort files alphabetically
            let mut sorted_files = files.clone();
            sorted_files.sort_by(|a, b| a.file_name.cmp(&b.file_name));
            
            for file in sorted_files {
                let file_path = file.relative_path.to_string_lossy();
                content.push_str(&format!("- [{}]({}) - {}\n", 
                    file.file_name,
                    file_path, // Markdown link
                    file.summary
                ));
            }
            
            content.push_str("\n");
        }
    }
    
    content
}