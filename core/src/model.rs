use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::{ PathBuf,Path};

use log::{debug};
use crate::element::Element;
use crate::element_registry::ElementRegistry;
use crate::error::ReqvireError;
use crate::relation;
use crate::relation::{get_ontological_parent_relation_types};
use crate::element::ElementType;
use crate::element::RequirementType;
use crate::filesystem;
use crate::diagrams;
use regex::Regex;

use crate::utils;
use crate::parser;
use globset::GlobSet;

#[derive(Debug)]
pub struct ModelManager {
    /// In-memory registry of elements
    pub element_registry: ElementRegistry,

}

impl ModelManager {
    /// Creates a new ModelManager
    pub fn new() -> Self {
        // Initialize empty element registry
        let element_registry = ElementRegistry::new();

        Self {
            element_registry
        }
    }


    pub fn parse_and_validate(
        &mut self, 
        git_commit_hash: Option<&str>,
        specification_folder: &PathBuf, 
        external_folders: &[PathBuf],
        excluded_filename_patterns: &GlobSet
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
    
        let mut errors = Vec::new();
        
        let files = utils::scan_markdown_files(git_commit_hash, &specification_folder, &external_folders, excluded_filename_patterns);
        //   .into_iter().map(|(path,_)| path).collect();
           
        debug!("Found {} markdown files.", files.len());


        let file_iterator = filesystem::FileReaderIterator::new(git_commit_hash,files.to_vec());
        for file_result in file_iterator {
            match file_result {
                Err(e) =>return Err(e),
                Ok((path, file_name, file_content)) => {

                    debug!("Markdown File found: {}", file_name);

                    let relative_path_str =utils::get_relative_path(&path, specification_folder, external_folders)?.to_string_lossy().to_string();
    
                    // Parse Elements    
                    let (elements, parse_errors) = parser::parse_elements(
                        &file_name,
                        &file_content,
                        &path,
                        specification_folder,
                        external_folders,
                    );

                    // Collect parse-time errors
                    errors.extend(parse_errors);

                    // Register parsed elements
                    for element in elements {
                        if let Err(e) = self.element_registry.register_element(element, &relative_path_str) {
                            //Duplicate element error should not really happen here
                            errors.push(e);
                        }
                    }                    
                }
            }
        };
        
        // Add missing opposites
        self.propagate_missing_opposites(excluded_filename_patterns);

        // Validate Relations
        errors.extend(self.validate_relations(excluded_filename_patterns)?);

        // Validate Cross-Component Dependencies
        errors.extend(self.validate_cross_component_dependencies()?);

        
        Ok(errors)
    }
    


    /// Adds missing opposite relations into the registry (does not return errors).
    fn propagate_missing_opposites(&mut self, excluded_filename_patterns: &GlobSet) {
        log::debug!("Propagating missing opposite relations...");
        let mut to_add: Vec<(String, relation::Relation)> = Vec::new();
        let element_ids: Vec<String> = self.element_registry.elements.keys().cloned().collect();
        let md_regex = Regex::new(r"\.md(?:#|$)").unwrap();

        for source_id in &element_ids {
            if let Some(source_element) = self.element_registry.elements.get(source_id) {
                for relation in &source_element.relations {
                    if let relation::LinkType::Identifier(ref target_id) = relation.target.link {
                        if !md_regex.is_match(target_id) || excluded_filename_patterns.is_match(target_id) {
                            continue;
                        }

                        if let Some(opposite_name) = relation.relation_type.opposite {
                            if let Some(target_element) = self.element_registry.elements.get(target_id) {
                                let already_present = target_element.relations.iter().any(|r| {
                                    matches!(&r.target.link, relation::LinkType::Identifier(id) if id == source_id)
                                        && r.relation_type.name.eq_ignore_ascii_case(opposite_name)
                                });

                                if !already_present {
                                    if let Some(opposite_relation) =
                                        relation.to_opposite(&source_element.name, &source_element.identifier)
                                    {
                                        to_add.push((target_id.clone(), opposite_relation));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Apply mutations
        for (target_id, relation) in to_add {
            if let Some(target_element) = self.element_registry.elements.get_mut(&target_id) {
                target_element.relations.push(relation);
                log::debug!("Added opposite relation to '{}'", target_id);
            }
        }
    }

   
    /// Validates relations for target existence and element type compatibility.
    fn validate_relations(&self, excluded_filename_patterns: &GlobSet) -> Result<Vec<ReqvireError>, ReqvireError> {
        log::debug!("Running relation validation...");
        let mut errors = Vec::new();
        let element_ids: Vec<String> = self.element_registry.elements.keys().cloned().collect();
        let md_regex = Regex::new(r"\.md(?:#|$)").unwrap();

        for source_id in &element_ids {
            if let Some(source_element) = self.element_registry.elements.get(source_id) {
                for relation in &source_element.relations {
                    if relation.is_opposite{
                        continue;
                    }
                    if let relation::LinkType::Identifier(ref target_id) = relation.target.link {
                        if !md_regex.is_match(target_id) {
                            log::debug!("Skipping non-markdown target: {}", target_id);                        
                            continue;
                        }

                        if excluded_filename_patterns.is_match(target_id) {
                            log::debug!("Skipping excluded target: {}", target_id);
                            continue;
                        }

                        match self.element_registry.get_element(target_id) {
                            Err(_) => {
                                // TODO: refactor this, it cannot really happen as it would be caught in parser with ReqvireError::InvalidIdentifier
                                errors.push(ReqvireError::MissingRelationTarget(
                                    format!("Element '{}' references missing target '{}'", source_element.identifier, target_id),
                                ));
                            }
                            Ok(target_element) => {

                                if let Some(error) = self.validate_element_types(
                                    relation.relation_type.name,
                                    source_element,
                                    target_element,
                                ) {
                                    errors.push(error);
                                }
                            }
                        }
                    }
                }
            }
        };

        Ok(errors)
    }
    

    /// Validates element types for a relation and returns an error if validation fails
    /// Returns None if validation passes or if the relation type doesn't have element type restrictions
    fn validate_element_types(
        &self, 
        relation_type: &str,
        source_element: &crate::element::Element,
        target_element: &crate::element::Element
    ) -> Option<crate::error::ReqvireError> {
        // Only validate relation types with element type restrictions
        if let Some(expected_types) = relation::get_relation_element_type_description(relation_type) {
            // Check if the element types are compatible
            let is_valid = relation::validate_relation_element_types(
                relation_type, 
                &source_element.element_type, 
                &target_element.element_type
            );
        
            if !is_valid {
                return Some(crate::error::ReqvireError::IncompatibleElementTypes(
                    format!("Relation '{}' from '{}' ({:?}) to '{}' ({:?}) has incompatible element types. {}",
                        relation_type,
                        source_element.identifier,
                        source_element.element_type,
                        target_element.identifier,
                        target_element.element_type,
                        expected_types
                    )
                ));
            } 
        }
    
        None
    }
    
    /// Validates cross-component dependencies for circular dependencies and missing links.
    fn validate_cross_component_dependencies(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating cross-component dependencies...");
        let mut errors = Vec::new();
        let mut visited = HashSet::new();

        // Check for circular dependencies
        for element in self.element_registry.get_all_elements() {
            let mut path = Vec::new();
            self.check_circular_dependencies(element, &mut visited, &mut path, &mut errors);
        }

        // Check for missing parent relations
        let valid_parent_relations = get_ontological_parent_relation_types();
        for element in self.element_registry.get_all_elements() {
        
            let element_file= &element.file_path;
            
            // Important: Only system requirements needs parent
            if let ElementType::Requirement(req_type) = &element.element_type {
                match req_type {
                    RequirementType::User => continue,                    
                    RequirementType::System =>{
                    
                        let has_parent_relation = element.relations.iter()
                            .any(|r| valid_parent_relations.contains(&r.relation_type.name));

                        if !has_parent_relation {
                            errors.push(ReqvireError::MissingParentRelation(
                                format!("File {}: Element '{}' has no parent relation (needs one of: {:?})", element_file,element.name, valid_parent_relations),
                            ));
                    
                        }                    
                    
                    }
                }
            }

        };

        if errors.is_empty() {
            debug!("No cross-component dependency validation errors found.");
        } else {
            debug!("{} cross-component validation errors found.", errors.len());
        };

        Ok(errors)
    }
    

    
    /// Recursively checks for circular dependencies in the element graph,
    /// following only forward relations.
    fn check_circular_dependencies(
        &self,
        element: &Element,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        errors: &mut Vec<ReqvireError>,
    ) {
        let element_id = element.identifier.clone();

        // If we've already fully processed this element, no need to check again.
        if visited.contains(&element_id) {
            return;
        }

        // If the current path already contains this element, we've found a cycle.
        if let Some(pos) = path.iter().position(|id| id == &element_id) {
            let cycle = path[pos..].join(" -> ");
            errors.push(ReqvireError::CircularDependencyError(
                format!("{}", cycle),
            ));
            return;
        }

        // Add this element to the current traversal path.
        path.push(element_id.clone());

        // Process only forward relations (ignore backward ones, which should have already been inserted).
        for relation in &element.relations {
            if let relation::LinkType::Identifier(ref target_id) = relation.target.link {
                // Only traverse forward relations.
                if relation.relation_type.direction == relation::RelationDirection::Forward {
                    if let Ok(target_element) = self.element_registry.get_element(target_id) {
                        self.check_circular_dependencies(target_element, visited, path, errors);
                    }
                }
            }
        }

        // Mark the current element as completely processed and remove it from the current path.
        visited.insert(element_id);
        path.pop();
    }
 
    /// Processes diagram generation for markdown files in place (without writing to output).
    /// Used when the `--generate-diagrams` flag is set.
    pub fn process_diagrams(
        &mut self,
        specification_folder: &PathBuf, 
        external_folders: &[PathBuf],        
        diagram_direction: &str
    ) -> Result<(), ReqvireError> {
        
        // Generate diagrams by section
        let diagrams = diagrams::generate_diagrams_by_section(&self.element_registry, diagram_direction, specification_folder, external_folders)?;

        // Group diagrams by file path
        let mut files_to_update: HashMap<String, Vec<(&str, &String)>> = HashMap::new();

        for (file_section_key, new_diagram) in &diagrams {
            let parts: Vec<&str> = file_section_key.split("::").collect();
            if parts.len() != 2 {
                continue; // Skip invalid entries
            }
            let file_path = parts[0];
            let section = parts[1];

            files_to_update
                .entry(file_path.to_string())
                .or_insert_with(Vec::new)
                .push((section, new_diagram));
        }

        // Process each file
        for (file_path, section_diagrams) in files_to_update {
            let file_path_obj = Path::new(&file_path);

            // Read file content
            let mut file_content = match filesystem::read_file(file_path_obj) {
                Ok(content) => content,
                Err(e) => {
                    log::error!("Failed to read file '{}': {}", file_path, e);
                    continue;
                }
            };



            // Replace diagrams for all sections in this file
            for (section, new_diagram) in section_diagrams {              
                file_content = self.replace_section_diagram(&file_content, section, new_diagram);
            }

            // Write updated content back if modified
            if let Err(e) = filesystem::write_file(file_path_obj, &file_content) {
                log::error!("Failed to write updated diagrams to '{}': {}", file_path, e);
            } else {
                println!("Updated diagrams in '{}'", file_path);
            }
        }

        Ok(())
    }
    
     /// Replaces the old diagram in a specific section of a markdown file.
    ///
    /// - `content`: The original file content.
    /// - `section`: The section name where the diagram should be replaced.
    /// - `new_diagram`: The newly generated Mermaid diagram.
    ///
    /// Returns the modified file content as a `String`.
    fn replace_section_diagram(&mut self, content: &str, section: &str, new_diagram: &str) -> String {
        let section_header = format!("## {}", section);
        let mermaid_block_start = "```mermaid";
        let mermaid_block_end = "```";

        let mut new_lines = Vec::new();
        let mut lines = content.lines().peekable();
        while let Some(line) = lines.next() {
            if line.trim() == section_header {
                // Found the target section header.
                new_lines.push(line.to_string());
                // Insert the new diagram immediately after the header.
                new_lines.push(new_diagram.to_string());
                // Skip any blank lines immediately after the header.
                while let Some(&next_line) = lines.peek() {
                    if next_line.trim().is_empty() {
                        lines.next();
                    } else {
                        break;
                    }
                }
                // If the next non-empty line starts a Mermaid block, skip it.
                if let Some(&next_line) = lines.peek() {
                    if next_line.trim().starts_with(mermaid_block_start) {
                        // Skip the mermaid block: first skip the start marker.
                        lines.next();
                        // Then skip lines until the end marker is found.
                        while let Some(l) = lines.next() {
                            if l.trim().starts_with(mermaid_block_end) {
                                break;
                            }
                        }
                    }
                }
                // Continue with the rest of the file.
            } else {
                new_lines.push(line.to_string());
            }
        }
        new_lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use globset::{Glob, GlobSet, GlobSetBuilder};
    use crate::error::ReqvireError;
    use crate::element_registry::ElementRegistry;
    use crate::linting::LintFix;
    // Dummy implementation of utils::normalize_fragment for testing.
    mod utils {
        pub fn normalize_fragment(fragment: &str) -> String {
            // For testing, simply lowercase and replace spaces with hyphens.
            fragment.to_lowercase().replace(' ', "-")
        }
    }

    // Dummy implementation of get_supported_relation_types in crate::relation
    mod relation {
        pub fn get_supported_relation_types() -> Vec<&'static str> {
            vec!["derivedFrom", "satisfiedBy", "tracedFrom", "containedBy"]
        }
    }

    #[test]
    fn test_extract_path_and_fragment() {
        // Test file reference with fragment.
        let input = "/user/repo#readme";
        let (file, frag) = crate::utils::extract_path_and_fragment(input);
        assert_eq!(file, "/user/repo");
        assert_eq!(frag, Some("readme"));

        // Test fragment-only with leading '#'.
        let input = "#intro";
        let (file, frag) = crate::utils::extract_path_and_fragment(input);
        assert_eq!(file, "");
        assert_eq!(frag, Some("intro"));

        // Test file only.
        let input = "document.md";
        let (file, frag) = crate::utils::extract_path_and_fragment(input);
        assert_eq!(file, "document.md");
        assert_eq!(frag, None);

        // Test fragment-only without '#' (treated as fragment-only)
        let input = "onlyfragment";
        let (file, frag) = crate::utils::extract_path_and_fragment(input);
        assert_eq!(file, "");
        assert_eq!(frag, Some("onlyfragment"));
    }

    #[test]
    fn test_find_nonlink_identifiers_plain_file_md() {
        // This test verifies that non-link plain file references are detected.
        let content = "Check out file.md please.";
        let file_path = PathBuf::from("test.md");
        let suggestions = crate::linting::nonlink_identifiers::find_nonlink_identifiers(content, &file_path);
        // Our regex for relation lines only matches relation bullet lines.
        // So this should produce 0 suggestions.
        assert_eq!(suggestions.len(), 0);
    }

    #[test]
    fn test_find_nonlink_identifiers_file_md_with_fragment() {
        let content = " * derivedFrom: file.md#Element Name with spaces";
        let file_path = PathBuf::from("test.md");
        let suggestions = crate::linting::nonlink_identifiers::find_nonlink_identifiers(content, &file_path);
        assert_eq!(suggestions.len(), 1, "Expected one suggestion");

        let suggestion = &suggestions[0];
        if let LintFix::ReplacePattern { pattern, replacement } = &suggestion.fix {
            // Pattern should contain the original raw identifier.
            assert!(pattern.contains("file.md#Element Name with spaces"), "pattern: {:?}", pattern);
            // Normalized: "file.md#element-name-with-spaces", link text remains as "file.md#Element Name with spaces"
            let expected_link = "[file.md#Element Name with spaces](file.md#element-name-with-spaces)";
            assert!(replacement.contains(expected_link), "replacement: {:?}", replacement);
        } else {
            panic!("Expected ReplacePattern fix");
        }
    }

    #[test]
    fn test_find_nonlink_identifiers_hash_only_fragment() {
        let content = " * derivedFrom: #Some Fragment";
        let file_path = PathBuf::from("test.md");
        let suggestions = crate::linting::nonlink_identifiers::find_nonlink_identifiers(content, &file_path);
        assert_eq!(suggestions.len(), 1, "Expected one suggestion");

        let suggestion = &suggestions[0];
        if let LintFix::ReplacePattern { pattern, replacement } = &suggestion.fix {
            // For a hash-only fragment, the file part is empty.
            assert!(pattern.contains("#Some Fragment"), "pattern: {:?}", pattern);
            // Link text should be "Some Fragment" (without '#') and link target should be "#some-fragment"
            let expected_link = "[Some Fragment](#some-fragment)";
            assert!(replacement.contains(expected_link), "replacement: {:?}", replacement);
        } else {
            panic!("Expected ReplacePattern fix");
        }
    }

    #[test]
    fn test_find_nonlink_identifiers_already_bracketed_link_ignored() {
        let content = "Check out [file.md](file.md) for details.";
        let file_path = PathBuf::from("test.md");
        let suggestions = crate::linting::nonlink_identifiers::find_nonlink_identifiers(content, &file_path);
        // Should ignore already bracketed links.
        assert_eq!(suggestions.len(), 0, "Expected no suggestions for already bracketed links");
    }
}





