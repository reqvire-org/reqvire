use std::fs;
use std::path::{Path};
use crate::html;
use crate::error::ReqvireError;
use crate::git_commands;
use walkdir::WalkDir;
use std::io::Write;


fn prepare_output_folder(output_folder: &Path) -> std::io::Result<()> {
    // Clean output folder
    if output_folder.exists() {
        fs::remove_dir_all(output_folder)?;
    }
    fs::create_dir_all(output_folder)?;

    // Create a .gitignore file that ignores everything except itself
    let gitignore_path = output_folder.join(".gitignore");
    let mut file = fs::File::create(gitignore_path)?;
    writeln!(
        file,
        "*\n!.gitignore"
    )?;

    Ok(())
}

/// Exports all markdown files to HTML without any processing or filtering
pub fn export_markdown_to_html(
    output_folder: &Path,
) -> Result<usize, ReqvireError> {
    // Try to get repository root as base directory
    let base_dir = match git_commands::get_git_root_dir() {
        Ok(git_root) => git_root,
        Err(_) => {
            // If Git repository root can't be found, use the current working directory
            std::env::current_dir()
                .map_err(|e| ReqvireError::PathError(format!("Failed to get current directory: {}", e)))?
        }
    };
    
    // prepare output folder
    prepare_output_folder(&output_folder)?;

    let mut processed_count = 0;

    // Process all markdown files in the repository
    processed_count += process_markdown_files(
        &base_dir,
        &base_dir,
        output_folder,
    )?;

    println!("✅ Total Markdown files exported: {}", processed_count);
    Ok(processed_count)
}

/// Processes all markdown files in a directory and converts them to HTML
fn process_markdown_files(
    root_folder: &Path,
    base_folder: &Path,
    output_folder: &Path,
) -> Result<usize, ReqvireError> {
    let mut count = 0;
    let mut all_files = Vec::new();
    
    // Process SpecificationIndex.md first to ensure it becomes index.html
    let spec_index_path = base_folder.join("SpecificationIndex.md");
    
    // Process all files using WalkDir
    for entry in WalkDir::new(root_folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let file_path = entry.path().to_path_buf();
        
        
        // Process SpecificationIndex.md first if it exists
        if file_path == spec_index_path {
            export_file_to_html(&file_path, base_folder, output_folder)?;
            count += 1;
        } else {
            all_files.push(file_path);
        }
    }
    
    // Process all other Markdown files
    for file_path in all_files {
        export_file_to_html(&file_path, base_folder, output_folder)?;
        count += 1;
    }

    Ok(count)
}

/// Converts a single markdown file to HTML.
/// If the file is `SpecificationIndex.md`, it is renamed to `index.html`.
fn export_file_to_html(
    file_path: &Path,
    base_folder: &Path,
    output_folder: &Path,
) -> Result<(), ReqvireError> {
    let content = fs::read_to_string(file_path)?;
    let file_name = file_path
        .file_name()
        .ok_or_else(|| ReqvireError::PathError("Invalid file path".to_string()))?
        .to_string_lossy();
    let title = file_name.replace(".md", "");

    // Get the relative path for link resolution
    let rel_path = file_path.strip_prefix(base_folder)
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
    println!("✅ Exported: {} -> {}", file_path.display(), html_path.display());

    Ok(())
}



