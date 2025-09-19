use anyhow::Result;
use std::collections::{HashSet};
use std::path::{ PathBuf};

use log::{debug};
use crate::element::Element;
use crate::element_registry::ElementRegistry;
use crate::error::ReqvireError;
use crate::relation;
use crate::relation::{get_parent_relation_types};
use crate::element::ElementType;
use crate::element::RequirementType;
use crate::filesystem;
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
        user_requirements_root_folder: &Option<PathBuf>,
        excluded_filename_patterns: &GlobSet
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
    
        let mut errors = Vec::new();
        
        let files = utils::scan_markdown_files(git_commit_hash, excluded_filename_patterns);
           
        debug!("Found {} markdown files.", files.len());


        let file_iterator = filesystem::FileReaderIterator::new(git_commit_hash, files);
        for file_result in file_iterator {
            match file_result {
                Err(e) =>return Err(e),
                Ok((path, file_name, file_content)) => {

                    debug!("Markdown File found: {}", file_name);

                    let relative_path_str = utils::get_relative_path(&path)?.to_string_lossy().to_string();

                    // Parse Elements, page content, and section content
                    let (elements, parse_errors, page_content, sections) = parser::parse_elements(
                        &file_name,
                        &file_content,
                        &path,
                        user_requirements_root_folder,
                    );

                    // Store page content
                    if !page_content.is_empty() {
                        let page = crate::element_registry::Page::new(page_content);
                        self.element_registry.pages.insert(relative_path_str.clone(), page);
                    }

                    // Store section content
                    for (section_name, section_content) in sections {
                        if !section_content.is_empty() {
                            let section = crate::element_registry::Section::new(section_content);
                            let section_key = crate::element_registry::SectionKey::new(
                                relative_path_str.clone(),
                                section_name
                            );
                            self.element_registry.sections.insert(section_key, section);
                        }
                    }

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

        for source_id in &element_ids {
            if let Some(source_element) = self.element_registry.elements.get(source_id) {
                for relation in &source_element.relations {
                    // Only process user-created relations to avoid infinite loops
                    // Auto-generated opposite relations are handled by the opposite creation logic
                    if !relation.user_created {
                        continue;
                    }

                    match &relation.target.link {
                        relation::LinkType::Identifier(ref target_id) => {
                            // Only skip excluded targets, don't filter by file type
                            if excluded_filename_patterns.is_match(target_id) {
                                log::debug!("Skipping excluded target: {}", target_id);
                                continue;
                            }

                            match self.element_registry.get_element(target_id) {
                                Err(_) => {
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
                        relation::LinkType::InternalPath(ref file_path) => {
                            // Validate file existence for InternalPath targets
                            if !file_path.exists() {
                                errors.push(ReqvireError::MissingRelationTarget(
                                    format!("Element '{}' references missing target '{}'",
                                        source_element.identifier,
                                        file_path.to_string_lossy()),
                                ));
                            }
                        }
                        relation::LinkType::ExternalUrl(_) => {
                            // Skip validation for external URLs as per specification
                            log::debug!("Skipping external URL validation");
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
        let valid_parent_relations = get_parent_relation_types();
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

        // Process only relations that could form cycles (hierarchical relations)
        for relation in &element.relations {
            if let relation::LinkType::Identifier(ref target_id) = relation.target.link {
                // Only traverse relations that could create circular dependencies
                // These are typically hierarchical relations that establish parent-child relationships
                if relation::IMPACT_PROPAGATION_RELATIONS.contains(&relation.relation_type.name) {
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
 

}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::linting::LintFix;

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





