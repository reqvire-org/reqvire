use std::fs;
use std::path::{PathBuf,Path};
use crate::html;
use crate::error::ReqvireError;
use crate::git_commands;
use crate::info_println;
use walkdir::WalkDir;
use log::debug;


/// Exports all markdown files to HTML without any processing or filtering
/// Uses auto-detection logic: if current directory is a subfolder of git root, 
/// only process files within that subfolder
pub fn export_markdown_to_html(
    _base_dir: &PathBuf,
    output_folder: &Path,
) -> Result<usize, ReqvireError> {

    let mut processed_count = 0;

    // Get git root directory
    let git_root = match git_commands::get_git_root_dir() {
        Ok(dir) => dir,
        Err(_) => {
            debug!("Not in a git repository, using current directory");
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        }
    };
    
    // Get current working directory
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // Determine scan directory - if current directory is within git root but not at git root,
    // scan only within the current directory subtree
    let scan_dir = if current_dir.starts_with(&git_root) && current_dir != git_root {
        current_dir
    } else {
        git_root.clone()
    };
    
    debug!("HTML export scanning for markdown files in: {}", scan_dir.display());

    // Process all markdown files in the determined directory
    processed_count += process_markdown_files(
        &scan_dir,
        &git_root,
        output_folder,
    )?;

    info_println!("✅ Total Markdown files exported: {}", processed_count);
    Ok(processed_count)
}

/// Processes all markdown files in a directory and converts them to HTML
fn process_markdown_files(
    scan_folder: &Path,
    base_folder: &Path,
    output_folder: &Path,
) -> Result<usize, ReqvireError> {
    let mut count = 0;
    let mut all_files = Vec::new();
    
    // Process SpecificationIndex.md first to ensure it becomes index.html
    let spec_index_path = scan_folder.join("SpecificationIndex.md");
    
    // Process all files using WalkDir
    for entry in WalkDir::new(scan_folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let file_path = entry.path().to_path_buf();
        
        
        // Process SpecificationIndex.md first if it exists
        if file_path == spec_index_path {
            export_file_to_html(&file_path, scan_folder, base_folder, output_folder)?;
            count += 1;
        } else {
            all_files.push(file_path);
        }
    }
    
    // Process all other Markdown files
    for file_path in all_files {
        export_file_to_html(&file_path, scan_folder, base_folder, output_folder)?;
        count += 1;
    }

    Ok(count)
}

/// Converts a single markdown file to HTML.
/// If the file is `SpecificationIndex.md`, it is renamed to `index.html`.
fn export_file_to_html(
    file_path: &Path,
    scan_folder: &Path,
    base_folder: &Path,
    output_folder: &Path,
) -> Result<(), ReqvireError> {
    let content = fs::read_to_string(file_path)?;
    let file_name = file_path
        .file_name()
        .ok_or_else(|| ReqvireError::PathError("Invalid file path".to_string()))?
        .to_string_lossy();
    let title = file_name.replace(".md", "");

    // Get the relative path for output directory structure - strip scan folder prefix
    let rel_path = file_path.strip_prefix(scan_folder)
        .map_err(|_| ReqvireError::PathError(format!("Failed to determine relative path for {}", file_path.display())))?;
    
    // Pass the file's path and base folder to convert_to_html 
    let html_content = html::convert_to_html(
        &file_path.to_path_buf(),
        &content,
        &title,
        &base_folder.to_path_buf()
    )?;

    // Determine where to place the output
    let mut html_path = output_folder.join(rel_path);

    // Special handling: SpecificationIndex.md
    if file_name == "SpecificationIndex.md" {
        // Always convert SpecificationIndex.md to index.html
        html_path.set_file_name("index.html");
    } 
    else {
        html_path.set_extension("html");
    }
    
    // Create parent directories if they don't exist
    if let Some(parent) = html_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&html_path, html_content)?;
    info_println!("✅ Exported: {} -> {}", file_path.display(), html_path.display());

    Ok(())
}



