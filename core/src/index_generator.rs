use std::path::{PathBuf, Path};
use std::collections::HashMap;
use crate::graph_registry::GraphRegistry;
use crate::element::Element;
use crate::error::ReqvireError;
use crate::git_commands;


/// Generates a SpecificationsIndex.md index from the existing element registry
pub fn generate_readme_index(
    registry: &GraphRegistry,
    _output_folder: &PathBuf
) -> Result<String, ReqvireError> {
    let mut index_content = String::from("# Specification Index\n\n");

    // Group elements by file only
    let mut grouped_elements: HashMap<String, Vec<&Element>> = HashMap::new();

    for element in registry.get_all_elements() {
        grouped_elements
            .entry(element.file_path.clone())
            .or_insert_with(Vec::new)
            .push(element);
    }

    // Generate the README index
    let mut sorted_files: Vec<_> = grouped_elements.keys().collect();
    sorted_files.sort(); // Sort files alphabetically

    for file in sorted_files {
        let elements = grouped_elements.get(file).unwrap();

        // Compute relative path
        let relative_path = get_relative_path(file);

        index_content.push_str(&format!("## [{}]({})\n\n", relative_path, relative_path));

        // Sort elements by file_order_index for consistent ordering
        let mut sorted_elements: Vec<_> = elements.iter().collect();
        sorted_elements.sort_by_key(|e| e.file_order_index);

        for element in sorted_elements {
            // Extract fragment from identifier (part after #)
            let element_id = if let Some(pos) = element.identifier.rfind('#') {
                generate_element_slug(&element.identifier[pos + 1..])
            } else {
                generate_element_slug(&element.identifier)
            };
            index_content.push_str(&format!("- [{}]({}#{})\n", element.name, relative_path, element_id));
        }

        index_content.push_str("\n"); // Add spacing between files
    }

    let total_files = grouped_elements.len();
    let total_elements: usize = registry.get_all_elements().len();

    index_content.push_str(&format!(
        "\n---\n**Summary:**\n- {} Files\n- {} Elements\n",
        total_files, total_elements
    ));

    // Return the generated content (file writing is handled by CLI)
    Ok(index_content)
}

fn generate_element_slug(name: &str) -> String {
    name
        .to_lowercase()
        .replace(" ", "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "") // Remove special characters
}

/// Computes a relative path to a file from the git repository root
fn get_relative_path(file: &str) -> String {
    match git_commands::get_git_root_dir() {
        Ok(git_root) => {
            let file_path = Path::new(file);
            match file_path.strip_prefix(&git_root) {
                Ok(relative) => relative.to_string_lossy().into_owned(),
                Err(_) => file.to_string(), // Fallback to absolute if stripping fails
            }
        },
        Err(_) => file.to_string() // Fall back to the file name if git root not found
    }
}

