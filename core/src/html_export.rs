use std::fs;
use std::path::{Path, PathBuf};
use crate::html;
use crate::error::ReqvireError;


/// Exports all markdown files to HTML without any processing or filtering
pub fn export_markdown_to_html(
    specification_folder: &PathBuf,
    external_folders: &[PathBuf],
    output_folder: &Path,
) -> Result<usize, ReqvireError> {
    // Clean output folder
    if output_folder.exists() {
        fs::remove_dir_all(output_folder)?;
    }
    fs::create_dir_all(output_folder)?;

    let mut processed_count = 0;

    // 1. Process the main specification folder
    if let Some(spec_name) = specification_folder.file_name() {
        let spec_output = output_folder.join(spec_name);
        processed_count += process_markdown_folder(
            specification_folder,
            specification_folder,
            external_folders,
            &spec_output,
        )?;
    } else {
        return Err(ReqvireError::PathError("Invalid specification folder name".to_string()));
    }

    // 2. Process external folders
    for folder in external_folders {
        if let Some(ext_name) = folder.file_name() {
            let ext_output = output_folder.join(ext_name);
            processed_count += process_markdown_folder(
                folder,
                specification_folder,
                external_folders,
                &ext_output,
            )?;
        } else {
            return Err(ReqvireError::PathError("Invalid external folder name".to_string()));
        }
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
) -> Result<usize, ReqvireError> {
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
) -> Result<(), ReqvireError> {
    let content = fs::read_to_string(file_path)?;
    let file_name = file_path
        .file_name()
        .ok_or_else(|| ReqvireError::PathError("Invalid file path".to_string()))?
        .to_string_lossy();
    let title = file_name.replace(".md", "");

    let html_content = html::convert_to_html(
        &file_path.to_path_buf(),
        &content,
        &title,
        specification_folder,
        external_folders,
    )?;

    // Determine where to place the output
    let mut html_path: PathBuf = {
        // Try specification folder
        if let Ok(rel_path) = file_path.strip_prefix(specification_folder) {
            output_folder.join(rel_path)
        }
        // Try external folders
        else if let Some((external_folder_name, rel_path)) = external_folders.iter().find_map(|folder| {
            if let Ok(rel) = file_path.strip_prefix(folder) {
                folder.file_name().map(|n| (n.to_string_lossy().to_string(), rel.to_path_buf()))
            } else {
                None
            }
        }) {
            output_folder.join(external_folder_name).join(rel_path)
        }
        // Fallback — flat export with just filename
        else {
            output_folder.join(file_name.to_string())
        }
    };

    // Special handling: README.md
    if file_name == "README.md" {
        if file_path.parent() == Some(specification_folder) {
            html_path.set_file_name("index.html");
        } else {
            html_path.set_file_name("README.html");
        }
    } else {
        html_path.set_extension("html");
    }

    if let Some(parent) = html_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&html_path, html_content)?;
    println!("✅ Exported: {} -> {}", file_path.display(), html_path.display());

    Ok(())
}



