//! Containment hierarchy representation and generation
//!
//! This module provides data structures and functions for building and
//! representing the physical containment hierarchy (folders, files, elements)
//! of a requirements model.

use crate::element::{Element, ElementType};
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

/// Represents a single element in the containment hierarchy
#[derive(Debug, Clone, Serialize)]
pub struct ContainmentElement {
    pub id: String,
    pub name: String,
    pub element_type: ElementType,
    pub file_path: String,
    pub identifier: String,
}

impl ContainmentElement {
    pub fn from_element(element: &Element) -> Self {
        ContainmentElement {
            id: element.id.clone(),
            name: element.name.clone(),
            element_type: element.element_type.clone(),
            file_path: element.file_path.clone(),
            identifier: element.identifier.clone(),
        }
    }
}

/// Represents a file containing elements
#[derive(Debug, Clone, Serialize)]
pub struct ContainmentFile {
    pub path: String,
    pub name: String,
    pub elements: Vec<ContainmentElement>,
}

/// Represents a folder containing files and subfolders
#[derive(Debug, Clone, Serialize)]
pub struct ContainmentFolder {
    pub name: String,
    pub path: Vec<String>,
    pub files: Vec<ContainmentFile>,
    pub subfolders: Vec<ContainmentFolder>,
}

/// Root containment hierarchy structure
#[derive(Debug, Clone, Serialize)]
pub struct ContainmentHierarchy {
    pub root_folder: ContainmentFolder,
}

impl ContainmentHierarchy {
    /// Build containment hierarchy from a registry
    ///
    /// When `short` is true, shows only root elements (those without hierarchical parents).
    /// When `short` is false (default), shows all elements.
    pub fn build(registry: &GraphRegistry, short: bool) -> Result<Self, ReqvireError> {
        // Group elements by file
        let mut files_map: BTreeMap<String, Vec<&Element>> = BTreeMap::new();
        for element in registry.get_all_elements() {
            files_map.entry(element.file_path.clone())
                .or_insert_with(Vec::new)
                .push(element);
        }

        // Build elements map - filter if short mode, otherwise show all
        let mut elements_map: BTreeMap<String, Vec<ContainmentElement>> = BTreeMap::new();
        for (file_path, elements) in files_map.iter() {
            let selected_elements: Vec<ContainmentElement> = if short {
                // Short mode: only top-level elements
                filter_top_level_elements(elements)
                    .iter()
                    .map(|e| ContainmentElement::from_element(e))
                    .collect()
            } else {
                // Full mode: all elements
                elements.iter()
                    .map(|e| ContainmentElement::from_element(e))
                    .collect()
            };
            elements_map.insert(file_path.clone(), selected_elements);
        }

        // Build folder structure
        let root_folder = build_folder_structure(&elements_map);

        Ok(ContainmentHierarchy { root_folder })
    }
}

/// Filter elements to show only top-level parents (those without hierarchical parents in same file)
fn filter_top_level_elements<'a>(elements: &[&'a Element]) -> Vec<&'a Element> {
    use std::collections::HashSet;

    // Get hierarchical relation types (derivedFrom)
    let hierarchical_types = crate::relation::get_hierarchical_relation_types();

    // Collect all element IDs (fragments) in this file
    let element_ids: HashSet<String> = elements.iter()
        .map(|e| e.id.clone())
        .collect();

    // Find elements that have derivedFrom relations pointing to elements in the same file
    let mut child_elements: HashSet<String> = HashSet::new();
    for element in elements {
        for relation in &element.relations {
            if hierarchical_types.contains(&relation.relation_type.name) {
                // Check if the target element_id is in the same file
                if let Some(target_id) = &relation.target.element_id {
                    if element_ids.contains(target_id) {
                        // This element has a parent in the same file, so it's a child
                        child_elements.insert(element.identifier.clone());
                    }
                }
            }
        }
    }

    // Return only elements that are NOT children (i.e., top-level)
    elements.iter()
        .filter(|e| !child_elements.contains(&e.identifier))
        .copied()
        .collect()
}

/// Build folder structure from files map
fn build_folder_structure(files_map: &BTreeMap<String, Vec<ContainmentElement>>) -> ContainmentFolder {
    // Build intermediate structure: folder_path -> files in that folder
    let mut folder_files: BTreeMap<Vec<String>, Vec<ContainmentFile>> = BTreeMap::new();

    for (file_path, elements) in files_map {
        let path = Path::new(file_path);
        let folder_path: Vec<String> = path.parent()
            .map(|p| p.components()
                .filter_map(|c| c.as_os_str().to_str())
                .map(String::from)
                .collect())
            .unwrap_or_default();

        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let file = ContainmentFile {
            path: file_path.clone(),
            name: file_name,
            elements: elements.clone(),
        };

        folder_files.entry(folder_path)
            .or_insert_with(Vec::new)
            .push(file);
    }

    // Build hierarchical folder structure
    build_folder_recursive(&[], &folder_files)
}

/// Recursively build folder structure
fn build_folder_recursive(
    current_path: &[String],
    folder_files: &BTreeMap<Vec<String>, Vec<ContainmentFile>>
) -> ContainmentFolder {
    let folder_name = current_path.last()
        .map(|s| s.clone())
        .unwrap_or_else(|| "Reqvire root".to_string());

    // Get files directly in this folder
    let files = folder_files.get(current_path)
        .cloned()
        .unwrap_or_default();

    // Find all immediate subfolders
    let mut subfolders = Vec::new();
    let current_depth = current_path.len();

    let mut seen_subfolders: std::collections::HashSet<String> = std::collections::HashSet::new();

    for folder_path in folder_files.keys() {
        if folder_path.len() == current_depth + 1 {
            // Check if this is an immediate child
            let is_child = current_path.iter()
                .zip(folder_path.iter())
                .all(|(a, b)| a == b);

            if is_child {
                if let Some(subfolder_name) = folder_path.last() {
                    if seen_subfolders.insert(subfolder_name.clone()) {
                        let subfolder = build_folder_recursive(folder_path, folder_files);
                        subfolders.push(subfolder);
                    }
                }
            }
        }
    }

    ContainmentFolder {
        name: folder_name,
        path: current_path.to_vec(),
        files,
        subfolders,
    }
}
