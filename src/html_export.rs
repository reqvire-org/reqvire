use std::fs;
use std::path::{Path, PathBuf};
use crate::html;
use crate::error::ReqFlowError;
use crate::utils;


/// Exports all markdown files to HTML without any processing or filtering
pub fn export_markdown_to_html(
    specification_folder: &PathBuf,
    external_folders: &[PathBuf],
    output_folder: &Path,
) -> Result<usize, ReqFlowError> {
    let mut processed_count = 0;

    // Process the main specification folder
    processed_count += process_markdown_folder(specification_folder, specification_folder,external_folders, output_folder)?;

    // Process each external folder
    for folder in external_folders {
        processed_count += process_markdown_folder(folder, specification_folder,external_folders, output_folder)?;
    }

    println!("✅ Total Markdown files exported: {}", processed_count);
    Ok(processed_count)
}

/// Recursively processes a folder and converts Markdown files to HTML.
fn process_markdown_folder(
    folder: &Path,
    specification_folder: &PathBuf,
    external_folders: &[PathBuf],    
    output_folder: &Path,
) -> Result<usize, ReqFlowError> {
    let mut count = 0;

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively process subdirectories
            count += process_markdown_folder(&path, specification_folder, external_folders, output_folder)?;
        } else if path.extension().map_or(false, |ext| ext == "md") {
            // Convert Markdown file to HTML
            export_file_to_html(&path, specification_folder, external_folders, output_folder)?;
            count += 1;
        }
    }

    Ok(count)
}


/// Converts a single markdown file to HTML.
/// If the file is `README.md`, it is renamed to `index.html`.
fn export_file_to_html(
    file_path: &Path,
    specification_folder: &PathBuf,
    external_folders: &[PathBuf],    
    output_folder: &Path,
) -> Result<(), ReqFlowError> {
    // Read the markdown content
    let content = fs::read_to_string(file_path)?;

    // Get the file name for the titleconvert_to_html
    let file_name = file_path.file_name()
        .ok_or_else(|| ReqFlowError::ValidationError("Invalid file path".to_string()))?
        .to_string_lossy();
    
    let title = file_name.replace(".md", "");

    // Convert Markdown to HTML
    let html_content = html::convert_to_html(&file_path.to_path_buf(), &content, &title, specification_folder, external_folders)?;

    // Determine the relative path
    let file=file_path.to_string_lossy().to_owned();
    
    let relative_path = utils::to_relative_identifier(&file, specification_folder,specification_folder,external_folders)?;

    let mut html_path = output_folder.join(&relative_path);

    //  Determine if the file is in `specification_folder`
    if file_path.starts_with(specification_folder) {
        if let Ok(relative) = file_path.strip_prefix(specification_folder) {
            html_path = output_folder.join(relative);
        }
    } else {
        //  Handle external files: Compute the correct relative path
        for external_folder in external_folders {
            if file_path.starts_with(external_folder) {
                if let Ok(relative) = file_path.strip_prefix(external_folder) {
                    let external_folder_name = external_folder.file_name()
                        .map(|name| name.to_string_lossy().to_string())
                        .unwrap_or("external".to_string()); // Default fallback

                    html_path = output_folder.join(external_folder_name).join(relative);
                    break;
                }
            }
        }
    }
    
    // Special Case: Rename `README.md` to `index.html`
    if file_name == "README.md" {
        if file_path.parent() == Some(specification_folder) {
            html_path.set_file_name("index.html"); // Root README.md → index.html
        } else {
            html_path.set_file_name("README.html"); // Subfolder README.md → README.html
        }
    } else {
        html_path.set_extension("html");
    }

    // Ensure the parent directory exists
    if let Some(parent) = html_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the converted HTML file
    fs::write(&html_path, html_content)?;

    println!("✅ Exported: {} -> {}", file_path.display(), html_path.display());

    Ok(())
}

