use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use log::{debug, info};
use rayon::prelude::*;

use crate::html;
use crate::error::ReqFlowError;
use crate::utils;

/// Exports all markdown files to HTML without any processing or filtering
pub fn export_markdown_to_html(input_folder: &Path, output_folder: &Path) -> Result<usize, ReqFlowError> {
    info!("Exporting all markdown files to HTML from {:?} to {:?}", input_folder, output_folder);

    
    // Create the output folder if it doesn't exist
    fs::create_dir_all(output_folder)?;
    
    // Find all markdown files in the input directory
    let files: Vec<PathBuf> = WalkDir::new(input_folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.is_file() && path.extension().map_or(false, |ext| ext == "md")
        })
        .map(|e| e.path().to_path_buf())
        .collect();
    
    info!("Found {} markdown files to convert to HTML", files.len());

    
    // Process files in parallel
    let processed_count = files.par_iter()
        .map(|file_path| {
            let result = export_file_to_html(file_path, input_folder, output_folder);
            match result {
                Ok(_) => {
                    debug!("Successfully converted {:?} to HTML", file_path);
                    1
                },
                Err(e) => {
                    eprintln!("Error converting {:?} to HTML: {}", file_path, e);
                    0
                }
            }
        })
        .sum();
    
    Ok(processed_count)
}

/// Converts a single markdown file to HTML
fn export_file_to_html(file_path: &Path, input_folder: &Path, output_folder: &Path) -> Result<(), ReqFlowError> {
    // Read the markdown content
    let content = fs::read_to_string(file_path)?;
    
    // Get the file name for the title
    let file_name = file_path.file_name()
        .ok_or_else(|| ReqFlowError::ValidationError("Invalid file path".to_string()))?
        .to_string_lossy();
    
    let title = file_name.replace(".md", "");
    
    // Convert to HTML
    let html_content = html::convert_to_html(&content, &title)?;
    
    // Determine the relative path and output file path
    let relative_path = utils::get_relative_path(file_path, input_folder)?;
    let mut html_path = output_folder.join(relative_path);
    html_path.set_extension("html");
    
    // Ensure the parent directory exists
    if let Some(parent) = html_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write the HTML file
    fs::write(&html_path, html_content)?;
    
    Ok(())
}
