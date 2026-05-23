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

/// Represents an attachment for display in containment hierarchy
#[derive(Debug, Clone, Serialize)]
pub struct ContainmentAttachment {
    /// Display name (element name or file name)
    pub name: String,
    /// Whether this is an element attachment (true) or file attachment (false)
    pub is_element: bool,
    /// Link to the attachment (element identifier or file path)
    pub link: Option<String>,
}

/// Represents a single element in the containment hierarchy
#[derive(Debug, Clone, Serialize)]
pub struct ContainmentElement {
    pub id: String,
    pub name: String,
    pub element_type: ElementType,
    pub file_path: String,
    pub identifier: String,
    pub attachments: Vec<ContainmentAttachment>,
}

impl ContainmentElement {
    pub fn from_element(element: &Element, registry: &GraphRegistry) -> Self {
        ContainmentElement {
            id: element.id.clone(),
            name: element.name.clone(),
            element_type: element.element_type.clone(),
            file_path: element.file_path.clone(),
            identifier: element.identifier.clone(),
            attachments: element
                .attachments
                .iter()
                .map(|a| match &a.target {
                    crate::element::AttachmentTarget::FilePath(path) => {
                        let file_name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_else(|| path.to_string_lossy().into_owned());
                        ContainmentAttachment {
                            name: file_name,
                            is_element: false,
                            link: Some(path.to_string_lossy().into_owned()),
                        }
                    }
                    crate::element::AttachmentTarget::ElementIdentifier(id) => {
                        // Look up element name from registry using the identifier
                        let name = registry
                            .get_element(id)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| id.clone());
                        ContainmentAttachment {
                            name,
                            is_element: true,
                            link: Some(id.clone()),
                        }
                    }
                })
                .collect(),
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

/// Represents a design document (non-specification markdown file in DesignDocuments folder)
#[derive(Debug, Clone, Serialize)]
pub struct DesignDocument {
    pub path: String,
    pub name: String,
}

/// Represents a folder containing files and subfolders
#[derive(Debug, Clone, Serialize)]
pub struct ContainmentFolder {
    pub name: String,
    pub path: Vec<String>,
    pub files: Vec<ContainmentFile>,
    pub design_documents: Vec<DesignDocument>,
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
            files_map
                .entry(element.file_path.clone())
                .or_default()
                .push(element);
        }

        // Build elements map - filter if short mode, otherwise show all
        let mut elements_map: BTreeMap<String, Vec<ContainmentElement>> = BTreeMap::new();
        for (file_path, elements) in files_map.iter() {
            let selected_elements: Vec<ContainmentElement> = if short {
                // Short mode: only top-level elements
                filter_top_level_elements(elements)
                    .iter()
                    .map(|e| ContainmentElement::from_element(e, registry))
                    .collect()
            } else {
                // Full mode: all elements
                elements
                    .iter()
                    .map(|e| ContainmentElement::from_element(e, registry))
                    .collect()
            };
            elements_map.insert(file_path.clone(), selected_elements);
        }

        // Scan for design documents
        let design_docs = scan_design_documents(&elements_map);

        // Build folder structure
        let root_folder = build_folder_structure(&elements_map, &design_docs);

        Ok(ContainmentHierarchy { root_folder })
    }
}

/// Filter elements to show only top-level parents (those without hierarchical parents in same file)
fn filter_top_level_elements<'a>(elements: &[&'a Element]) -> Vec<&'a Element> {
    use std::collections::HashSet;

    // Get hierarchical relation types (derivedFrom)
    let hierarchical_types = crate::relation::get_hierarchical_relation_types();

    // Collect all element IDs (fragments) in this file
    let element_ids: HashSet<String> = elements.iter().map(|e| e.id.clone()).collect();

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
    elements
        .iter()
        .filter(|e| !child_elements.contains(&e.identifier))
        .copied()
        .collect()
}

/// Scan for design documents in DesignDocuments folders
fn scan_design_documents(
    files_map: &BTreeMap<String, Vec<ContainmentElement>>,
) -> BTreeMap<Vec<String>, Vec<DesignDocument>> {
    use std::fs;

    let mut design_docs: BTreeMap<Vec<String>, Vec<DesignDocument>> = BTreeMap::new();

    // Get all unique parent folders from the files map
    let mut parent_folders: std::collections::HashSet<Vec<String>> =
        std::collections::HashSet::new();
    for file_path in files_map.keys() {
        let path = Path::new(file_path);
        if let Some(parent) = path.parent() {
            let folder_path: Vec<String> = parent
                .components()
                .filter_map(|c| c.as_os_str().to_str())
                .map(String::from)
                .collect();
            // Add this folder and all parent folders
            for i in 1..=folder_path.len() {
                parent_folders.insert(folder_path[..i].to_vec());
            }
        }
    }

    // For each folder, check if it has a DesignDocuments subfolder
    for folder_path in parent_folders {
        let design_docs_path = folder_path
            .iter()
            .cloned()
            .chain(std::iter::once("DesignDocuments".to_string()))
            .collect::<Vec<_>>();

        let design_docs_dir = design_docs_path.join("/");
        if let Ok(entries) = fs::read_dir(&design_docs_dir) {
            let mut docs = Vec::new();
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Some(ext) = entry_path.extension() {
                        if ext == "md" {
                            let file_name = entry_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("")
                                .to_string();
                            let relative_path = design_docs_path
                                .iter()
                                .cloned()
                                .chain(std::iter::once(file_name.clone()))
                                .collect::<Vec<_>>()
                                .join("/");
                            docs.push(DesignDocument {
                                path: relative_path,
                                name: file_name,
                            });
                        }
                    }
                }
            }
            if !docs.is_empty() {
                // Sort for deterministic output
                docs.sort_by(|a, b| a.name.cmp(&b.name));
                design_docs.insert(design_docs_path, docs);
            }
        }
    }

    design_docs
}

/// Build folder structure from files map
fn build_folder_structure(
    files_map: &BTreeMap<String, Vec<ContainmentElement>>,
    design_docs: &BTreeMap<Vec<String>, Vec<DesignDocument>>,
) -> ContainmentFolder {
    // Build intermediate structure: folder_path -> files in that folder
    let mut folder_files: BTreeMap<Vec<String>, Vec<ContainmentFile>> = BTreeMap::new();

    // Track all folder paths (including intermediate folders without direct files)
    let mut all_folder_paths: std::collections::HashSet<Vec<String>> =
        std::collections::HashSet::new();

    for (file_path, elements) in files_map {
        let path = Path::new(file_path);
        let folder_path: Vec<String> = path
            .parent()
            .map(|p| {
                p.components()
                    .filter_map(|c| c.as_os_str().to_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        // Add all intermediate folder paths (e.g., for "a/b/c", add "", "a", "a/b", "a/b/c")
        for i in 0..=folder_path.len() {
            all_folder_paths.insert(folder_path[..i].to_vec());
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let file = ContainmentFile {
            path: file_path.clone(),
            name: file_name,
            elements: elements.clone(),
        };

        folder_files.entry(folder_path).or_default().push(file);
    }

    // Add design document folder paths
    for design_docs_path in design_docs.keys() {
        for i in 0..=design_docs_path.len() {
            all_folder_paths.insert(design_docs_path[..i].to_vec());
        }
    }

    // Build hierarchical folder structure using all folder paths
    build_folder_recursive(&[], &folder_files, &all_folder_paths, design_docs)
}

/// Recursively build folder structure
fn build_folder_recursive(
    current_path: &[String],
    folder_files: &BTreeMap<Vec<String>, Vec<ContainmentFile>>,
    all_folder_paths: &std::collections::HashSet<Vec<String>>,
    design_docs: &BTreeMap<Vec<String>, Vec<DesignDocument>>,
) -> ContainmentFolder {
    let folder_name = current_path
        .last()
        .cloned()
        .unwrap_or_else(|| "Reqvire root".to_string());

    // Get files directly in this folder
    let files = folder_files.get(current_path).cloned().unwrap_or_default();

    // Get design documents directly in this folder
    let folder_design_docs = design_docs.get(current_path).cloned().unwrap_or_default();

    // Find all immediate subfolders (using all_folder_paths to include intermediate folders)
    let mut subfolders = Vec::new();
    let current_depth = current_path.len();

    let mut seen_subfolders: std::collections::HashSet<String> = std::collections::HashSet::new();

    for folder_path in all_folder_paths.iter() {
        if folder_path.len() == current_depth + 1 {
            // Check if this is an immediate child
            let is_child = current_path
                .iter()
                .zip(folder_path.iter())
                .all(|(a, b)| a == b);

            if is_child {
                if let Some(subfolder_name) = folder_path.last() {
                    if seen_subfolders.insert(subfolder_name.clone()) {
                        let subfolder = build_folder_recursive(
                            folder_path,
                            folder_files,
                            all_folder_paths,
                            design_docs,
                        );
                        subfolders.push(subfolder);
                    }
                }
            }
        }
    }

    // Sort subfolders for deterministic output
    subfolders.sort_by(|a, b| a.name.cmp(&b.name));

    ContainmentFolder {
        name: folder_name,
        path: current_path.to_vec(),
        files,
        design_documents: folder_design_docs,
        subfolders,
    }
}

/// D3.js tree node structure for hierarchical visualization
#[derive(Debug, Clone, Serialize)]
pub struct D3TreeNode {
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<D3TreeNode>,
}

impl ContainmentHierarchy {
    /// Convert containment hierarchy to D3.js tree format
    pub fn to_d3_tree(&self) -> D3TreeNode {
        folder_to_d3_node(&self.root_folder)
    }
}

/// Convert markdown links to HTML links for the D3 tree
fn md_to_html_link(link: &str) -> String {
    link.replace(".md", ".html")
}

/// Convert a ContainmentFolder to D3TreeNode recursively
fn folder_to_d3_node(folder: &ContainmentFolder) -> D3TreeNode {
    let mut children = Vec::new();

    // Add subfolders first
    for subfolder in &folder.subfolders {
        children.push(folder_to_d3_node(subfolder));
    }

    // Add design documents
    for doc in &folder.design_documents {
        children.push(D3TreeNode {
            name: doc.name.clone(),
            node_type: "design-document".to_string(),
            link: Some(md_to_html_link(&doc.path)),
            children: Vec::new(),
        });
    }

    // Add files
    for file in &folder.files {
        let mut file_children = Vec::new();

        // Add elements as children of file
        for element in &file.elements {
            let element_type = match &element.element_type {
                ElementType::Feature => "feature",
                ElementType::Requirement(crate::element::RequirementType::System) => {
                    "system-requirement"
                }
                ElementType::Verification(_) => "verification",
                ElementType::Refinement(_) => "refinement",
                _ => "element",
            };

            // Build element children: attachments only (relations removed)
            let mut element_children = Vec::new();

            // Add attachments as children
            for attachment in &element.attachments {
                let (node_type, link) = if attachment.is_element {
                    // Element attachment - show as refinement with link
                    (
                        "attachment-element".to_string(),
                        attachment.link.as_ref().map(|l| md_to_html_link(l)),
                    )
                } else {
                    // File attachment - show path without conversion
                    ("attachment-file".to_string(), attachment.link.clone())
                };
                element_children.push(D3TreeNode {
                    name: attachment.name.clone(),
                    node_type,
                    link,
                    children: Vec::new(),
                });
            }

            file_children.push(D3TreeNode {
                name: element.name.clone(),
                node_type: element_type.to_string(),
                link: Some(md_to_html_link(&element.identifier)),
                children: element_children,
            });
        }

        children.push(D3TreeNode {
            name: file.name.clone(),
            node_type: "file".to_string(),
            link: Some(md_to_html_link(&file.path)),
            children: file_children,
        });
    }

    D3TreeNode {
        name: folder.name.clone(),
        node_type: "folder".to_string(),
        link: None,
        children,
    }
}
