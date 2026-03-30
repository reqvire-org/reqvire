use crate::error::ReqvireError;
use crate::filesystem;
use crate::git_commands;
use crate::html;
use crate::info_println;
use log::debug;
use pulldown_cmark::{Event, Parser, Tag};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

    debug!(
        "HTML export scanning for markdown files in: {}",
        scan_dir.display()
    );

    // Process all markdown files in the determined directory
    processed_count += process_markdown_files(&scan_dir, &git_root, output_folder)?;

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
        .filter(|e| e.path().is_file() && e.path().extension().is_some_and(|ext| ext == "md"))
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
    let rel_path = file_path.strip_prefix(scan_folder).map_err(|_| {
        ReqvireError::PathError(format!(
            "Failed to determine relative path for {}",
            file_path.display()
        ))
    })?;

    // Pass the file's path and base folder to convert_to_html
    let html_content = html::convert_to_html(
        &file_path.to_path_buf(),
        &content,
        &title,
        &base_folder.to_path_buf(),
    )?;

    // Determine where to place the output
    let mut html_path = output_folder.join(rel_path);

    // Special handling: SpecificationIndex.md
    if file_name == "SpecificationIndex.md" {
        // Always convert SpecificationIndex.md to index.html
        html_path.set_file_name("index.html");
    } else {
        html_path.set_extension("html");
    }

    // Create parent directories if they don't exist
    if let Some(parent) = html_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&html_path, html_content)?;
    copy_local_linked_assets(file_path, &content, scan_folder, output_folder)?;
    info_println!(
        "✅ Exported: {} -> {}",
        file_path.display(),
        html_path.display()
    );

    Ok(())
}

pub(crate) fn extract_local_asset_paths(markdown_content: &str) -> Vec<PathBuf> {
    let parser = Parser::new(markdown_content);
    let mut asset_paths = Vec::new();
    let mut seen = HashSet::new();

    for event in parser {
        let dest_url = match event {
            Event::Start(Tag::Image { dest_url, .. }) => Some(dest_url),
            Event::Start(Tag::Link { dest_url, .. }) => Some(dest_url),
            _ => None,
        };

        if let Some(dest_url) = dest_url {
            if let Some(path) = normalize_local_asset_path(dest_url.as_ref()) {
                if seen.insert(path.clone()) {
                    asset_paths.push(path);
                }
            }
        }
    }

    asset_paths
}

fn normalize_local_asset_path(dest_url: &str) -> Option<PathBuf> {
    let trimmed = dest_url.trim();

    if trimmed.is_empty()
        || trimmed.starts_with('#')
        || trimmed.starts_with('/')
        || trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("data:")
        || trimmed.starts_with("mailto:")
    {
        return None;
    }

    let without_query = trimmed
        .split_once('?')
        .map(|(path, _)| path)
        .unwrap_or(trimmed);
    let stripped = without_query
        .split_once('#')
        .map(|(path, _)| path)
        .unwrap_or(without_query);

    let path = PathBuf::from(stripped);

    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
    {
        None
    } else {
        Some(path)
    }
}

fn copy_local_linked_assets(
    markdown_file_path: &Path,
    markdown_content: &str,
    scan_folder: &Path,
    output_folder: &Path,
) -> Result<(), ReqvireError> {
    let Some(markdown_parent) = markdown_file_path.parent() else {
        return Ok(());
    };
    let canonical_scan_folder =
        fs::canonicalize(scan_folder).unwrap_or_else(|_| scan_folder.to_path_buf());

    for relative_asset_path in extract_local_asset_paths(markdown_content) {
        let source_path = markdown_parent.join(&relative_asset_path);
        if !source_path.is_file() {
            debug!(
                "Skipping missing local linked asset during HTML export: {} (from {})",
                source_path.display(),
                markdown_file_path.display()
            );
            continue;
        }

        let Ok(canonical_source_path) = fs::canonicalize(&source_path) else {
            debug!(
                "Skipping unreadable local linked asset during HTML export: {}",
                source_path.display()
            );
            continue;
        };

        let Ok(asset_relative_path) = canonical_source_path.strip_prefix(&canonical_scan_folder)
        else {
            debug!(
                "Skipping local linked asset outside export scope: {} (scope {})",
                canonical_source_path.display(),
                canonical_scan_folder.display()
            );
            continue;
        };

        let destination_path = output_folder.join(asset_relative_path);
        if destination_path == canonical_source_path {
            continue;
        }

        filesystem::copy_file_with_structure(&canonical_source_path, &destination_path)?;
        debug!(
            "Copied local linked asset for HTML export: {} -> {}",
            canonical_source_path.display(),
            destination_path.display()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_extract_local_asset_paths_collects_local_non_markdown_targets() {
        let markdown = r#"
![Local](images/local-diagram.svg)
![Parent](../assets/overview.PNG?raw=1)
[Reference](docs/reference.txt)
[Executable](bin/tool)
![External](https://example.com/image.png)
![Data](data:image/png;base64,AAAA)
[Document](guide.md)
[Anchor](#section)
"#;

        let asset_paths = extract_local_asset_paths(markdown);

        assert_eq!(
            asset_paths,
            vec![
                PathBuf::from("images/local-diagram.svg"),
                PathBuf::from("../assets/overview.PNG"),
                PathBuf::from("docs/reference.txt"),
                PathBuf::from("bin/tool"),
            ]
        );
    }

    #[test]
    fn test_copy_local_linked_assets_preserves_relative_structure() {
        let source_dir = tempdir().unwrap();
        let output_dir = tempdir().unwrap();
        let markdown_dir = source_dir.path().join("specifications");
        let image_dir = markdown_dir.join("images");
        let docs_dir = markdown_dir.join("docs");
        fs::create_dir_all(&image_dir).unwrap();
        fs::create_dir_all(&docs_dir).unwrap();

        let markdown_file = markdown_dir.join("Requirements.md");
        let image_file = image_dir.join("local-diagram.svg");
        let doc_file = docs_dir.join("reference.txt");
        fs::write(
            &markdown_file,
            "![Local](images/local-diagram.svg)\n[Reference](docs/reference.txt)\n",
        )
        .unwrap();
        fs::write(&image_file, "<svg></svg>").unwrap();
        fs::write(&doc_file, "fixture").unwrap();

        copy_local_linked_assets(
            &markdown_file,
            "![Local](images/local-diagram.svg)\n[Reference](docs/reference.txt)\n",
            source_dir.path(),
            output_dir.path(),
        )
        .unwrap();

        assert!(output_dir
            .path()
            .join("specifications/images/local-diagram.svg")
            .is_file());
        assert!(output_dir
            .path()
            .join("specifications/docs/reference.txt")
            .is_file());
    }
}
