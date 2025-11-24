use std::collections::{HashMap, BTreeSet, HashSet, BTreeMap};
use std::fs;
use std::path::{Path, PathBuf};
use log::{debug, warn};
use serde::Serialize;

use crate::relation::{self, LinkType, get_hierarchical_relation_types, IMPACT_PROPAGATION_RELATIONS};
use crate::element::{Element, ElementType, RequirementType};
use crate::error::ReqvireError;
use crate::git_commands;
use globset::GlobSet;
use regex::Regex;


#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub frontmatter_content: String,
}

impl Page {
    pub fn new(frontmatter_content: String) -> Self {
        Self {
            frontmatter_content,
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct RelationNode {
    pub relation_trigger: String,
    pub element_node: ElementNode,
}

#[derive(Debug, Clone, Serialize)]
pub struct ElementNode {
    pub element: Element,
    pub relations: Vec<RelationNode>,
}

#[derive(Debug)]
pub struct GraphRegistry {
    pub nodes: HashMap<String, ElementNode>,
    pub pages: HashMap<String, Page>,
    pub modified_files: HashSet<String>, // Track files modified during CRUD operations
}

impl GraphRegistry {
    /// Creates a new empty GraphRegistry
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            pages: HashMap::new(),
            modified_files: HashSet::new(),
        }
    }

    /// Registers a page with content
    pub fn register_page(&mut self, file_path: String, page_content: String) {
        self.pages.insert(file_path, Page::new(page_content));
    }

    /// Registers an element with local validation
    pub fn register_element(&mut self, element: Element, _file_path: &str) -> Result<(), ReqvireError> {
        let element_id = element.identifier.clone();

        // Note: Duplicate checking is now done at global level in ModelManager::pass1_collect_elements
        // to properly report all duplicate locations

        self.nodes.insert(element_id, ElementNode {
            element,
            relations: Vec::new(),
        });

        Ok(())
    }

    /// Build relations and validate graph structure
    pub fn build_relations(&mut self, excluded_filename_patterns: &GlobSet) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("GraphRegistry: Building relations and validating graph structure");

        // First build the relation graph
        self.build_relation_graph();

        // Add missing opposites
        self.propagate_missing_opposites(excluded_filename_patterns);

        // Populate element_id for all relations
        self.populate_relation_element_ids();

        // Validate relations
        let mut errors = self.validate_relations(excluded_filename_patterns)?;

        // Validate non-test-verification satisfiedBy relations
        errors.extend(self.validate_non_test_verification_satisfied_by()?);

        // Validate cross-component dependencies
        errors.extend(self.validate_cross_component_dependencies()?);

        // Validate attachments exist
        errors.extend(self.validate_attachments()?);

        Ok(errors)
    }

    /// Build the relation graph structure
    fn build_relation_graph(&mut self) {
        let element_ids: Vec<String> = self.nodes.keys().cloned().collect();

        for source_id in &element_ids {
            let mut relation_nodes = Vec::new();

            if let Some(source_node) = self.nodes.get(source_id) {
                for relation in &source_node.element.relations {
                    if let LinkType::Identifier(ref target_id) = relation.target.link {
                        // Only handle relations that propagate impact
                        if relation::IMPACT_PROPAGATION_RELATIONS.contains(&relation.relation_type.name) {
                            if let Some(target_node) = self.nodes.get(target_id) {
                                relation_nodes.push(RelationNode {
                                    relation_trigger: relation.relation_type.name.to_string(),
                                    element_node: target_node.clone(),
                                });
                            }
                        }
                    }
                }
            }

            if let Some(entry) = self.nodes.get_mut(source_id) {
                entry.relations = relation_nodes;
            }
        }
    }

    /// Adds missing opposite relations into the registry (does not return errors).
    fn propagate_missing_opposites(&mut self, excluded_filename_patterns: &GlobSet) {
        log::debug!("Propagating missing opposite relations...");
        let mut to_add: Vec<(String, crate::relation::Relation)> = Vec::new();
        let element_ids: Vec<String> = self.nodes.keys().cloned().collect();
        let md_regex = Regex::new(r"\.md(?:#|$)").unwrap();

        for source_id in &element_ids {
            if let Some(source_node) = self.nodes.get(source_id) {
                for relation in &source_node.element.relations {
                    if let crate::relation::LinkType::Identifier(ref target_id) = relation.target.link {
                        if !md_regex.is_match(target_id) || excluded_filename_patterns.is_match(target_id) {
                            continue;
                        }

                        if let Some(opposite_name) = relation.relation_type.opposite {
                            if let Some(target_node) = self.nodes.get(target_id) {
                                let already_present = target_node.element.relations.iter().any(|r| {
                                    matches!(&r.target.link, crate::relation::LinkType::Identifier(id) if id == source_id)
                                        && r.relation_type.name.eq_ignore_ascii_case(opposite_name)
                                });

                                if !already_present {
                                    if let Some(opposite_relation) =
                                        relation.to_opposite(&source_node.element.name, &source_node.element.identifier, &source_node.element.id)
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
            if let Some(target_node) = self.nodes.get_mut(&target_id) {
                target_node.element.relations.push(relation);
                log::debug!("Added opposite relation to '{}'", target_id);
            }
        }
    }

    /// Populates element_id for all relations by looking up targets in the registry
    fn populate_relation_element_ids(&mut self) {
        log::debug!("Populating element_id for all relations...");

        // First pass: collect mapping of identifier -> element_id
        let id_map: std::collections::HashMap<String, String> = self.nodes.iter()
            .map(|(identifier, node)| (identifier.clone(), node.element.id.clone()))
            .collect();

        // Second pass: update all relations with element_id
        for source_node in self.nodes.values_mut() {
            for relation in &mut source_node.element.relations {
                // Populate element_id for all Identifier links
                if let crate::relation::LinkType::Identifier(ref target_id) = relation.target.link {
                    if let Some(element_id) = id_map.get(target_id) {
                        relation.target.element_id = Some(element_id.clone());
                    }
                }
            }
        }
    }

    /// Validates relations for target existence and element type compatibility.
    fn validate_relations(&self, excluded_filename_patterns: &GlobSet) -> Result<Vec<ReqvireError>, ReqvireError> {
        log::debug!("Running relation validation...");
        let mut errors = Vec::new();
        let element_ids: Vec<String> = self.nodes.keys().cloned().collect();

        for source_id in &element_ids {
            if let Some(source_node) = self.nodes.get(source_id) {
                for relation in &source_node.element.relations {
                    // Only process user-created relations to avoid infinite loops
                    if !relation.user_created {
                        continue;
                    }

                    match &relation.target.link {
                        crate::relation::LinkType::Identifier(ref target_id) => {
                            if excluded_filename_patterns.is_match(target_id) {
                                log::debug!("Skipping excluded target: {}", target_id);
                                continue;
                            }

                            match self.get_element(target_id) {
                                None => {
                                    errors.push(ReqvireError::MissingRelationTarget(
                                        format!("Element '{}' references missing target '{}'", source_node.element.identifier, target_id),
                                    ));
                                }
                                Some(target_element) => {
                                    if let Some(error) = self.validate_element_types(
                                        relation.relation_type.name,
                                        &source_node.element,
                                        target_element,
                                    ) {
                                        errors.push(error);
                                    }
                                }
                            }
                        }
                        crate::relation::LinkType::InternalPath(ref file_path) => {
                            // Validate file existence for InternalPath targets
                            // InternalPath contains normalized paths from normalize_identifier which are git-root-relative
                            let git_root = match crate::git_commands::get_git_root_dir() {
                                Ok(root) => root,
                                Err(_) => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
                            };
                            let absolute_path = git_root.join(file_path);
                            if !absolute_path.exists() {
                                errors.push(ReqvireError::MissingRelationTarget(
                                    format!("Element '{}' references missing target '{}'",
                                        source_node.element.identifier,
                                        file_path.to_string_lossy()),
                                ));
                            }
                        }
                        crate::relation::LinkType::ExternalUrl(_) => {
                            // Skip validation for external URLs as per specification
                            log::debug!("Skipping external URL validation");
                        }
                    }
                }
            }
        }

        Ok(errors)
    }

    /// Validates element types for a relation and returns an error if validation fails
    fn validate_element_types(
        &self,
        relation_type: &str,
        source_element: &Element,
        target_element: &Element
    ) -> Option<ReqvireError> {
        // Only validate relation types with element type restrictions
        if let Some(expected_types) = crate::relation::get_relation_element_type_description(relation_type) {
            // Check if the element types are compatible
            let is_valid = crate::relation::validate_relation_element_types(
                relation_type,
                &source_element.element_type,
                &target_element.element_type
            );

            if !is_valid {
                return Some(ReqvireError::IncompatibleElementTypes(
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

    /// Validates that only test-verification elements can have satisfiedBy relations
    /// Returns a list of validation errors for non-test-verification elements with satisfiedBy
    fn validate_non_test_verification_satisfied_by(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        log::debug!("Validating non-test-verification satisfiedBy relations...");
        let mut errors = Vec::new();

        for element_node in self.nodes.values() {
            let element = &element_node.element;

            // Check if element has satisfiedBy relations
            let has_satisfied_by = element.relations.iter().any(|relation| {
                relation.relation_type.name == "satisfiedBy" && relation.user_created
            });

            if has_satisfied_by {
                // Check if the element is a non-test-verification
                match &element.element_type {
                    crate::element::ElementType::Verification(verification_type) => {
                        // Allow only test-verification (Default and Test types) to have satisfiedBy
                        match verification_type {
                            crate::element::VerificationType::Analysis |
                            crate::element::VerificationType::Inspection |
                            crate::element::VerificationType::Demonstration => {
                                errors.push(ReqvireError::IncompatibleElementTypes(
                                    format!("Non-test-verification element with satisfiedBy relation: '{}' (type: {:?}) cannot have satisfiedBy relations. Only test-verification elements may use satisfiedBy.",
                                        element.identifier,
                                        verification_type
                                    )
                                ));
                            }
                            crate::element::VerificationType::Default |
                            crate::element::VerificationType::Test => {
                                // These are valid - test verifications can have satisfiedBy
                            }
                        }
                    }
                    _ => {
                        // Requirements and other elements can have satisfiedBy relations
                        // This is valid behavior
                    }
                }
            }
        }

        Ok(errors)
    }

    /// Validates cross-component dependencies for circular dependencies and missing links.
    fn validate_cross_component_dependencies(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating cross-component dependencies...");
        let mut errors = Vec::new();
        let mut visited = HashSet::new();

        // Check for circular dependencies - but be less strict about what constitutes a cycle
        for element_node in self.nodes.values() {
            let mut path = Vec::new();
            self.check_circular_dependencies(&element_node.element, &mut visited, &mut path, &mut errors);
        }

        // Check for missing hierarchical parent relations
        let valid_hierarchical_relations = get_hierarchical_relation_types();
        for element_node in self.nodes.values() {
            let element = &element_node.element;
            let element_file = &element.file_path;

            // Important: Only system requirements need hierarchical parent (derivedFrom)
            if let ElementType::Requirement(req_type) = &element.element_type {
                match req_type {
                    RequirementType::User => continue,
                    RequirementType::System => {
                        let has_hierarchical_parent = element.relations.iter()
                            .any(|r| valid_hierarchical_relations.contains(&r.relation_type.name));

                        if !has_hierarchical_parent {
                            errors.push(ReqvireError::MissingParentRelation(
                                format!("File {}: Element '{}' has no hierarchical parent relation (needs derivedFrom)", element_file, element.name),
                            ));
                        }
                    }
                }
            }
        }

        if errors.is_empty() {
            debug!("No cross-component dependency validation errors found.");
        } else {
            debug!("{} cross-component validation errors found.", errors.len());
        }

        Ok(errors)
    }

    /// Validates that all attachment files referenced by elements exist.
    fn validate_attachments(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating attachment file existence...");
        let mut errors = Vec::new();

        // Get git root for resolving attachment paths
        let git_root = crate::git_commands::get_git_root_dir()
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());

        for element_node in self.nodes.values() {
            let element = &element_node.element;

            for attachment in &element.attachments {
                // Resolve attachment path relative to git root
                let full_path = git_root.join(&attachment.file_path);

                if !full_path.exists() {
                    errors.push(ReqvireError::MissingAttachmentFile(
                        format!(
                            "File {}: Element '{}' references missing attachment file: {}",
                            element.file_path,
                            element.name,
                            attachment.file_path.display()
                        ),
                    ));
                }
            }
        }

        if errors.is_empty() {
            debug!("No attachment validation errors found.");
        } else {
            debug!("{} attachment validation errors found.", errors.len());
        }

        Ok(errors)
    }

    /// Check for circular dependencies using relation metadata to traverse in canonical direction
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
            let full_cycle = format!("{} -> {}", cycle, element_id);
            errors.push(ReqvireError::CircularDependencyError(
                format!("Circular dependency error: {}", full_cycle),
            ));
            return;
        }

        // Add this element to the current traversal path.
        path.push(element_id.clone());

        // Traverse relations using their metadata to determine canonical direction
        for relation in &element.relations {
            if let LinkType::Identifier(ref target_id) = relation.target.link {
                // Skip relations that don't participate in dependency propagation (like trace and backward relations)
                // Only traverse relations that are in IMPACT_PROPAGATION_RELATIONS for cycle detection
                if !IMPACT_PROPAGATION_RELATIONS.contains(&relation.relation_type.name) {
                    continue;
                }

                // Use relation metadata to traverse in canonical direction only
                let should_traverse = if let Some(_opposite) = relation.relation_type.opposite {
                    // For bidirectional relations, only traverse in one canonical direction
                    // to avoid detecting the same logical cycle twice
                    // Traverse if this relation type is "lexicographically smaller" than its opposite
                    // or if this is the primary direction for this relation type
                    relation.relation_type.name < relation.relation_type.opposite.unwrap_or("")
                } else {
                    // For unidirectional relations, always traverse
                    true
                };

                if should_traverse {
                    if let Some(target_element) = self.get_element(target_id) {
                        self.check_circular_dependencies(target_element, visited, path, errors);
                    }
                }
            }
        }

        // Mark the current element as completely processed and remove it from the current path.
        visited.insert(element_id);
        path.pop();
    }

    /// Updates an element's identifier and rewires all incoming relations
    pub fn update_identifier(&mut self, old_id: &str, new_id: &str) {
        if let Some(mut node) = self.nodes.remove(old_id) {
            node.element.identifier = new_id.to_string();

            // Update relations within this element (if any self-refs)
            for relation in &mut node.element.relations {
                if let LinkType::Identifier(ref mut link_id) = relation.target.link {
                    if link_id == old_id {
                        *link_id = new_id.to_string();
                    }
                }
            }

            // Reinsert with new ID
            self.nodes.insert(new_id.to_string(), node);

            // Update all relations pointing to this identifier
            for (_id, other_node) in self.nodes.iter_mut() {
                for relation in &mut other_node.element.relations {
                    if let LinkType::Identifier(ref mut link_id) = relation.target.link {
                        if link_id == old_id {
                            *link_id = new_id.to_string();
                        }
                    }
                }

                for relation_node in &mut other_node.relations {
                    if relation_node.element_node.element.identifier == old_id {
                        relation_node.element_node.element.identifier = new_id.to_string();
                    }
                }
            }
        }
    }

    /// Find element identifier by element name (globally unique)
    ///
    /// # Arguments
    /// * `element_name` - Element name to search for
    ///
    /// # Returns
    /// * Element identifier if found and unique
    /// * Error if not found or multiple matches
    pub fn find_element_by_name(&self, element_name: &str) -> Result<String, ReqvireError> {
        // Generate slug from element name (same logic as HTML heading to ID)
        let name_slug = element_name.trim().replace(' ', "-").to_lowercase();

        // Find all matching identifiers
        let matching_ids: Vec<String> = self.nodes
            .keys()
            .filter(|id| id.ends_with(&format!("#{}", name_slug)))
            .cloned()
            .collect();

        if matching_ids.is_empty() {
            return Err(ReqvireError::MissingElement(
                format!("Element not found: {}", element_name)
            ));
        } else if matching_ids.len() > 1 {
            return Err(ReqvireError::ProcessError(
                format!("Multiple elements found with name '{}': {:?}", element_name, matching_ids)
            ));
        }

        Ok(matching_ids[0].clone())
    }

    /// Moves an element to an existing file in the graph
    pub fn move_element_to_location(&mut self, element_id: &str, new_file_path: &str) -> Result<(), ReqvireError> {
        // Verify the target file exists in the graph (either has elements or is registered as a page)
        let target_has_elements = self.nodes.values().any(|node| {
            node.element.file_path == new_file_path
        });
        let target_is_page = self.pages.contains_key(new_file_path);

        if !target_has_elements && !target_is_page {
            return Err(ReqvireError::LocationNotFound(format!(
                "Target file '{}' does not exist in the graph",
                new_file_path
            )));
        }

        if let Some(node) = self.nodes.get_mut(element_id) {
            let old_file_path = node.element.file_path.clone();

            node.element.file_path = new_file_path.to_string();

            // Update the element in all relation nodes that reference it
            for (_id, other_node) in self.nodes.iter_mut() {
                for relation_node in &mut other_node.relations {
                    if relation_node.element_node.element.identifier == element_id {
                        relation_node.element_node.element.file_path = new_file_path.to_string();
                    }
                }
            }

            log::debug!(
                "Moved element '{}' from '{}' to '{}'",
                element_id, old_file_path, new_file_path
            );

            Ok(())
        } else {
            Err(ReqvireError::MissingElement(format!("Element '{}' not found in graph", element_id)))
        }
    }

    /// Adds a new file location to the graph (virtual - no filesystem changes)
    pub fn add_file_location(&mut self, new_file_path: &str) -> Result<(), ReqvireError> {
        // Check if the file already exists
        let file_exists = self.nodes.values().any(|node| node.element.file_path == new_file_path);

        if file_exists {
            return Err(ReqvireError::LocationAlreadyExists(format!("File '{}' already exists in the graph", new_file_path)));
        }

        // Create a virtual placeholder element to track this file location
        let virtual_id = format!("__virtual__{}", new_file_path);
        let virtual_element = Element::new(
            &format!("Virtual placeholder for {}", new_file_path),
            &virtual_id,
            new_file_path,
            0, // Virtual elements don't have real line numbers
            None,
        );

        self.nodes.insert(virtual_id, ElementNode {
            element: virtual_element,
            relations: Vec::new(),
        });

        log::debug!("Added virtual file location '{}'", new_file_path);
        Ok(())
    }

    /// Moves element to a new file location (creates file location if needed)
    pub fn move_element_to_new_file(&mut self, element_id: &str, new_file_path: &str) -> Result<(), ReqvireError> {
        // Check if file exists, if not, create it virtually
        let file_exists = self.nodes.values().any(|node| node.element.file_path == new_file_path);

        if !file_exists {
            self.add_file_location(new_file_path)?;
        }

        if let Some(node) = self.nodes.get_mut(element_id) {
            let old_file_path = node.element.file_path.clone();

            node.element.file_path = new_file_path.to_string();

            // Update the element in all relation nodes that reference it
            for (_id, other_node) in self.nodes.iter_mut() {
                for relation_node in &mut other_node.relations {
                    if relation_node.element_node.element.identifier == element_id {
                        relation_node.element_node.element.file_path = new_file_path.to_string();
                    }
                }
            }

            // Update relation identifiers for cross-file references
            self.update_relation_identifiers(element_id, &old_file_path, new_file_path);

            log::debug!(
                "Moved element '{}' from '{}' to new file '{}'",
                element_id, old_file_path, new_file_path
            );

            Ok(())
        } else {
            Err(ReqvireError::MissingElement(format!("Element '{}' not found in graph", element_id)))
        }
    }

    /// Gets all available file locations in the graph
    pub fn get_available_locations(&self) -> Vec<String> {
        let mut locations = std::collections::BTreeSet::new();

        for node in self.nodes.values() {
            locations.insert(node.element.file_path.clone());
        }

        locations.into_iter().collect()
    }

    /// Gets all elements that would be affected by moving the specified element
    pub fn get_move_impact(&self, element_id: &str) -> Vec<String> {
        let mut affected_elements = Vec::new();

        // Find all elements that reference this element
        for (id, node) in &self.nodes {
            if id == element_id {
                continue; // Skip the element being moved
            }

            // Check if this element has relations pointing to the element being moved
            let has_reference = node.element.relations.iter().any(|relation| {
                matches!(&relation.target.link, LinkType::Identifier(link_id) if link_id == element_id)
            });

            if has_reference {
                affected_elements.push(id.clone());
            }
        }

        affected_elements.sort();
        affected_elements
    }

    pub fn get_impact_tree(&self, root_id: &str) -> ElementNode {
        let mut visited = BTreeSet::new();
        self.build_impact_tree_recursive(root_id, &mut visited)
    }

    fn build_impact_tree_recursive(&self, current_id: &str, visited: &mut BTreeSet<String>) -> ElementNode {
        if !visited.insert(current_id.to_string()) {
            // Already visited, stop recursion to prevent cycles
            let current_node = self.nodes.get(current_id).unwrap();
            return ElementNode {
                element: current_node.element.clone(),
                relations: Vec::new(), // Empty relations to break the cycle
            };
        }

        let current_node = self.nodes.get(current_id).unwrap();
        let mut child_nodes = Vec::new();

        for relation_node in &current_node.relations {
            let target_id = &relation_node.element_node.element.identifier;

            // Skip relations to already visited nodes to prevent cycles
            if visited.contains(target_id) {
                continue;
            }

            let subtree = self.build_impact_tree_recursive(target_id, visited);
            child_nodes.push(RelationNode {
                relation_trigger: relation_node.relation_trigger.clone(),
                element_node: subtree,
            });
        }

        ElementNode {
            element: current_node.element.clone(),
            relations: child_nodes,
        }
    }

    /// Gets all elements as a vector, sorted by identifier for deterministic output
    pub fn get_all_elements(&self) -> Vec<&Element> {
        let mut elements: Vec<&Element> = self.nodes.values().map(|node| &node.element).collect();
        elements.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        elements
    }

    /// Find all requirements without hierarchical parent relations (root requirements)
    pub fn find_root_requirements(&self) -> Vec<String> {
        let hierarchical_relations = relation::get_hierarchical_relation_types();

        let mut roots: Vec<String> = self.nodes.values()
            .map(|node| &node.element)
            .filter(|element| {
                // Only consider requirements
                if !matches!(element.element_type, ElementType::Requirement(_)) {
                    return false;
                }

                // Check if has any hierarchical parent relation
                let has_parent = element.relations.iter()
                    .any(|r| hierarchical_relations.contains(&r.relation_type.name));

                !has_parent
            })
            .map(|e| e.identifier.clone())
            .collect();

        // Sort for deterministic output
        roots.sort();
        roots
    }

    /// Collects all InternalPath targets from element relations
    pub fn get_internal_path_targets(&self) -> HashSet<PathBuf> {
        self.collect_internal_path_targets()
    }

    /// Gets requirements grouped by root folder
    pub fn get_requirements_by_root(&self) -> BTreeMap<String, Vec<&Element>> {
        let mut requirements_by_root = BTreeMap::new();
        let parent_relation_types = ["containedBy", "derivedFrom", "satisfy", "verify"];

        let all_elements = self.get_all_elements();

        // Find root elements (elements without parent relations)
        let root_elements: Vec<&Element> = all_elements.iter()
            .filter(|element| {
                !element.relations.iter().any(|rel| {
                    parent_relation_types.contains(&rel.relation_type.name)
                })
            })
            .copied()
            .collect();

        // For each root element, find all its descendants recursively
        for root_element in &root_elements {
            let mut descendants = vec![*root_element];
            self.collect_descendants(&all_elements, &mut descendants);
            requirements_by_root.insert(root_element.name.clone(), descendants);
        }

        // If no root elements found, group by file path as fallback
        if requirements_by_root.is_empty() {
            for element in &all_elements {
                let root_folder = if let Some(slash_pos) = element.file_path.find('/') {
                    element.file_path[..slash_pos].to_string()
                } else {
                    "root".to_string()
                };
                requirements_by_root
                    .entry(root_folder)
                    .or_insert_with(Vec::new)
                    .push(*element);
            }
        }

        requirements_by_root
    }

    /// Recursively collect all descendants of the elements already in descendants
    fn collect_descendants<'a>(&self, all_elements: &[&'a Element], descendants: &mut Vec<&'a Element>) {
        let mut found_new = true;

        while found_new {
            found_new = false;
            let descendants_len = descendants.len();

            for element in all_elements {
                // Skip if already collected
                if descendants.iter().any(|d| d.identifier == element.identifier) {
                    continue;
                }

                // Check if this element has a parent relation pointing to any element in descendants
                let has_parent_in_descendants = element.relations.iter().any(|rel| {
                    matches!(&rel.target.link, crate::relation::LinkType::Identifier(target_id)
                        if descendants.iter().any(|d| d.identifier == *target_id)
                        && ["containedBy", "derivedFrom", "refine", "satisfy", "verify"].contains(&rel.relation_type.name))
                });

                if has_parent_in_descendants {
                    descendants.push(*element);
                    found_new = true;
                }
            }

            // If we didn't find any new descendants, break to avoid infinite loop
            if descendants.len() == descendants_len {
                break;
            }
        }
    }

    /// Change impact analysis with relation information
    pub fn change_impact_with_relation(&self, element: &Element) -> Vec<(String, Vec<crate::relation::Relation>)> {
        if let Some(node) = self.nodes.get(&element.identifier) {
            // Group original relations by target ID using BTreeMap for deterministic ordering
            let mut relations_by_target: std::collections::BTreeMap<String, Vec<crate::relation::Relation>> = std::collections::BTreeMap::new();

            for relation in &node.element.relations {
                let target_id = match &relation.target.link {
                    crate::relation::LinkType::Identifier(ref target_id) => target_id.clone(),
                    crate::relation::LinkType::InternalPath(ref path) => path.to_string_lossy().to_string(),
                    crate::relation::LinkType::ExternalUrl(_) => continue, // Skip external URLs for change impact
                };

                relations_by_target
                    .entry(target_id)
                    .or_insert_with(Vec::new)
                    .push(relation.clone());
            }

            relations_by_target.into_iter().collect()
        } else {
            Vec::new()
        }
    }

    /// Gets a specific element by ID
    pub fn get_element(&self, element_id: &str) -> Option<&Element> {
        self.nodes.get(element_id).map(|node| &node.element)
    }

    /// Gets an element by its display name
    pub fn get_element_by_name(&self, name: &str) -> Option<&Element> {
        self.nodes.values()
            .map(|node| &node.element)
            .find(|elem| elem.name == name)
    }

    /// Creates a virtual file location
    pub fn create_virtual_file(&mut self, file_path: &str) -> Result<(), ReqvireError> {
        self.add_file_location(file_path)
    }

    /// Collects all InternalPath targets from element relations
    pub fn collect_internal_path_targets(&self) -> HashSet<PathBuf> {
        let mut internal_paths = HashSet::new();

        for node in self.nodes.values() {
            for relation in &node.element.relations {
                if let LinkType::InternalPath(ref path) = relation.target.link {
                    internal_paths.insert(path.clone());
                }
            }
        }

        internal_paths
    }


    fn element_to_markdown_with_context(&self, element: &Element, _current_file: &str) -> String {
        let mut markdown = String::new();

        // Add the element header
        markdown.push_str(&format!("### {}\n\n", element.name));

        // Add the element content
        if !element.content.trim().is_empty() {
            markdown.push_str(element.content.trim_end());
            markdown.push_str("\n");
        }

        // Add metadata subsection if there are custom metadata
        let mut custom_metadata: Vec<_> = element.metadata.iter()
            .filter(|(key, _)| *key != "type") // type is handled separately
            .collect();
        custom_metadata.sort_by_key(|(key, _)| *key);

        // Only add metadata section if there are custom metadata or non-default type
        // Default type is only "requirement" - user-requirement should always be explicit
        let has_non_default_type = element.element_type.as_str() != "requirement";

        if !custom_metadata.is_empty() || has_non_default_type {
            markdown.push_str("#### Metadata\n");

            // Add type metadata
            markdown.push_str(&format!("  * type: {}\n", element.element_type.as_str()));

            // Add other metadata
            for (key, value) in custom_metadata {
                markdown.push_str(&format!("  * {}: {}\n", key, value));
            }
            markdown.push_str("\n");
        }

        // Add attachments subsection if there are attachments
        if !element.attachments.is_empty() {
            markdown.push_str("#### Attachments\n");
            for attachment in &element.attachments {
                // Attachment paths are stored as git-root-relative paths
                let attachment_path = attachment.file_path.to_string_lossy().to_string();

                // Make the path relative to the current file's directory (same as relations)
                let current_file_path = std::path::PathBuf::from(_current_file);
                let current_folder = current_file_path.parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .to_path_buf();

                // Use to_relative_identifier like we do for InternalPath relations
                // Prepend "/" to indicate git-root-relative path
                let absolute_path = format!("/{}", attachment_path);
                let relative_path = crate::utils::to_relative_identifier(
                    &absolute_path,
                    &current_folder,
                    false
                ).unwrap_or_else(|_| attachment_path.clone());

                // Use filename as display text for cleaner markdown
                let display_name = attachment.file_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or(&attachment_path);

                markdown.push_str(&format!("  * [{}]({})\n", display_name, relative_path));
            }
            markdown.push_str("\n");
        }

        // Add relations subsection if there are user-created relations
        let user_relations: Vec<_> = element.relations.iter().filter(|r| r.user_created).collect();
        if !user_relations.is_empty() {
            markdown.push_str("#### Relations\n");
            for relation in user_relations {
                // Format relation target based on type
                // Format as proper markdown link using element name when possible
                let target_text = match &relation.target.link {
                    LinkType::ExternalUrl(url) => {
                        // For external URLs, preserve the original markdown link format
                        format!("[{}]({})", relation.target.text, url)
                    },
                    LinkType::Identifier(target_id) => {
                        // Extract fragment to look up the target element
                        let fragment = if let Some(fragment_pos) = target_id.find('#') {
                            &target_id[fragment_pos + 1..]
                        } else {
                            target_id
                        };

                        // Use actual element name if available, otherwise fallback to fragment conversion
                        // First try to lookup by full target_id, then by fragment only
                        let display_name = if let Some(target_node) = self.nodes.get(target_id) {
                            target_node.element.name.clone()
                        } else if let Some(target_node) = self.nodes.get(fragment) {
                            target_node.element.name.clone()
                        } else {
                            // Fallback: convert fragment to title case
                            fragment.replace('-', " ")
                                .split_whitespace()
                                .map(|word| {
                                    let mut chars = word.chars();
                                    match chars.next() {
                                        None => String::new(),
                                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(" ")
                        };

                        // Check if target is in the same file
                        let target_file = if let Some(file_pos) = target_id.find('#') {
                            &target_id[..file_pos]
                        } else {
                            target_id
                        };

                        // Get current file path for comparison
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_file_str = _current_file;

                        // If target is in the same file, use just the fragment
                        if target_file.is_empty() || target_file == current_file_str ||
                           target_id.starts_with('#') {
                            format!("[{}](#{})", display_name, fragment)
                        } else {
                            // Make the link relative using just the folder of the current file
                            let current_folder = current_file_path.parent()
                                .unwrap_or_else(|| std::path::Path::new("."))
                                .to_path_buf();

                            let relative_link = crate::utils::to_relative_identifier(
                                relation.target.link.as_str(),
                                &current_folder,
                                false
                            ).unwrap_or_else(|_| relation.target.link.as_str().to_string());

                            format!("[{}]({})", display_name, relative_link)
                        }
                    },
                    LinkType::InternalPath(path) => {
                        // For InternalPath, use the filename as display text and full relative path as link
                        let path_str = path.to_str().unwrap_or("invalid_path");
                        let display_name = std::path::Path::new(path_str)
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(path_str);

                        // Make the path relative using just the folder of the current file
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_folder = current_file_path.parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .to_path_buf();

                        let relative_link = crate::utils::to_relative_identifier(
                            relation.target.link.as_str(),
                            &current_folder,
                            false
                        ).unwrap_or_else(|_| relation.target.link.as_str().to_string());

                        format!("[{}]({})", display_name, relative_link)
                    }
                };

                markdown.push_str(&format!("  * {}: {}\n",
                    relation.relation_type.name,
                    target_text
                ));
            }
            markdown.push_str("\n");
        }

        // Apply generic formatting to ensure exactly one blank line before all #### headers
        Self::ensure_blank_lines_before_subsections(&markdown)
    }

    /// Ensures every #### header has exactly one blank line before it (skips content inside <details> blocks)
    /// and removes blank lines immediately after #### headers
    fn ensure_blank_lines_before_subsections(content: &str) -> String {
        let mut result = String::new();
        let mut in_details = false;

        for line in content.lines() {
            let trimmed_line = line.trim_start().to_lowercase();

            // Track <details> blocks
            if trimmed_line.starts_with("<details") {
                in_details = true;
            }

            // Add blank line before #### headers (if not in <details>)
            if !in_details && line.trim_start().starts_with("####") {
                // Remove any trailing newlines
                while result.ends_with('\n') {
                    result.pop();
                }
                if !result.is_empty() {
                    result.push_str("\n\n");
                }
            }

            // Skip blank lines immediately after #### headers
            if !in_details && line.trim().is_empty() {
                // Check if the previous line was a #### header
                let prev_line_is_header = result.lines().last()
                    .map_or(false, |l| l.trim_start().starts_with("####"));
                if prev_line_is_header {
                    continue;
                }
            }

            result.push_str(line);
            result.push('\n');

            // Track end of <details> blocks
            if trimmed_line.starts_with("</details>") {
                in_details = false;
            }
        }

        // Trim end
        let trimmed = result.trim_end();
        if trimmed.is_empty() {
            String::new()
        } else {
            format!("{}\n", trimmed)
        }
    }

    /// Groups elements by their file path
    pub fn group_elements_by_location(&self) -> HashMap<String, Vec<&Element>> {
        let mut file_elements: HashMap<String, Vec<&Element>> = HashMap::new();

        for node in self.nodes.values() {
            let element = &node.element;

            // Skip virtual placeholder elements
            if element.identifier.starts_with("__virtual__") {
                continue;
            }

            file_elements
                .entry(element.file_path.clone())
                .or_insert_with(Vec::new)
                .push(element);
        }

        // Sort elements within each file by their original order index
        for elements in file_elements.values_mut() {
            elements.sort_by_key(|element| element.file_order_index);
        }

        file_elements
    }

    /// Generates markdown content for a file
    pub fn generate_file_markdown(&self, file_path: &str, elements: &[&Element]) -> String {
        let mut markdown = String::new();

        // All specification files must have "# Requirements" as the page header
        markdown.push_str("# Requirements\n\n");

        // Add page content if available
        if let Some(page) = self.pages.get(file_path) {
            if !page.frontmatter_content.trim().is_empty() {
                markdown.push_str(&page.frontmatter_content);
                if !page.frontmatter_content.ends_with('\n') {
                    markdown.push('\n');
                }
                markdown.push('\n');
            }
        }

        // Add elements in file order
        for (i, element) in elements.iter().enumerate() {
            // Add separator before each element (except the first)
            if i > 0 {
                markdown.push_str("---\n\n");
            }
            markdown.push_str(&self.element_to_markdown_with_context(element, file_path));
        }

        // Add final separator after the last element (if there were any elements)
        if !elements.is_empty() {
            markdown.push_str("---\n\n");
        }

        markdown
    }

    /// Copies InternalPath files to the output directory
    fn copy_internal_path_files(&self, internal_paths: &HashSet<PathBuf>, output_dir: &Path) -> Result<usize, ReqvireError> {
        let base_dir = match git_commands::get_git_root_dir() {
            Ok(git_root) => git_root,
            Err(_) => {
                // If Git repository root can't be found, use the current working directory
                std::env::current_dir()
                    .map_err(|e| ReqvireError::PathError(format!("Failed to get current directory: {}", e)))?
            }
        };

        let mut files_copied = 0;

        for internal_path in internal_paths {
            // Resolve the source path relative to base directory
            let src_path = if internal_path.is_absolute() {
                internal_path.clone()
            } else {
                base_dir.join(internal_path)
            };

            // Skip if source file doesn't exist
            if !src_path.is_file() {
                warn!("Skipping missing InternalPath file: {:?}", src_path);
                continue;
            }

            // Build the destination path
            let dst_path = output_dir.join(internal_path);

            // Skip if source and destination are the same (in-place operations)
            if src_path == dst_path {
                debug!("Skipping InternalPath file (same source and destination): {:?}", src_path);
                continue;
            }

            // Create parent directories if needed
            if let Some(parent_dir) = dst_path.parent() {
                fs::create_dir_all(parent_dir)
                    .map_err(|e| ReqvireError::IoError(e))?;
            }

            // Copy the file
            match fs::copy(&src_path, &dst_path) {
                Ok(_) => {
                    debug!("Copied InternalPath file: {:?} -> {:?}", src_path, dst_path);
                    files_copied += 1;
                }
                Err(e) => {
                    warn!("Failed to copy InternalPath file {:?}: {}", src_path, e);
                }
            }
        }

        Ok(files_copied)
    }

    /// Rename an element while updating all relations
    ///
    /// # Arguments
    /// * `element_id` - Current element identifier
    /// * `new_name` - New name for the element
    ///
    /// # Returns
    /// * New element identifier after rename
    pub fn rename_element(
        &mut self,
        element_id: &str,
        new_name: &str,
    ) -> Result<String, ReqvireError> {
        // Validate element exists
        let node = self.nodes.get(element_id)
            .ok_or_else(|| ReqvireError::MissingElement(
                format!("Element '{}' not found", element_id)
            ))?;

        let file_path = node.element.file_path.clone();
        let _old_name = node.element.name.clone();

        // Generate new identifier (slug from new name - same logic as markdown heading to ID)
        let new_slug = new_name.trim().replace(' ', "-").to_lowercase();
        let new_identifier = format!("{}#{}", file_path, new_slug);

        // Check if new identifier already exists (globally unique check)
        if self.nodes.contains_key(&new_identifier) {
            return Err(ReqvireError::DuplicateElement(
                format!("An element with name '{}' already exists (identifier: {})", new_name, new_identifier)
            ));
        }

        // Find all files with relations to this element
        let mut modified_files = vec![file_path.clone()];
        for node in self.nodes.values() {
            let has_relation = node.element.relations.iter().any(|rel| {
                matches!(&rel.target.link, LinkType::Identifier(id) if id == element_id)
            });

            if has_relation {
                let file = node.element.file_path.clone();
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Update the element's name and identifier in the node
        if let Some(node) = self.nodes.get_mut(element_id) {
            node.element.name = new_name.to_string();
            node.element.identifier = new_identifier.clone();
        }

        // Move node in the map (remove old key, insert with new key)
        if let Some(node) = self.nodes.remove(element_id) {
            self.nodes.insert(new_identifier.clone(), node);
        }

        // Update all relations (both forward and backward)
        // Update relations in all elements that reference the old identifier
        let old_id = element_id.to_string();
        for node in self.nodes.values_mut() {
            for relation in &mut node.element.relations {
                if let LinkType::Identifier(ref mut id) = relation.target.link {
                    if id == &old_id {
                        *id = new_identifier.clone();
                        // Update the text reference too
                        relation.target.text = new_name.to_string();
                    }
                }
            }
        }

        // Track all modified files
        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok(new_identifier)
    }

    /// Move entire file with all its elements to a new location
    /// Updates all element identifiers and relations referencing moved elements
    pub fn move_file(
        &mut self,
        source_file: &str,
        target_file: &str,
        squash: bool,
    ) -> Result<Vec<(String, String)>, ReqvireError> {
        // Validate source file exists in the model
        let elements_in_source: Vec<String> = self.nodes.values()
            .filter(|node| node.element.file_path == source_file)
            .map(|node| node.element.identifier.clone())
            .collect();

        if elements_in_source.is_empty() {
            return Err(ReqvireError::LocationNotFound(
                format!("Source file '{}' not found or contains no elements", source_file)
            ));
        }

        // Validate target file doesn't exist (unless squash mode)
        let target_exists = self.nodes.values()
            .any(|node| node.element.file_path == target_file);

        if target_exists && !squash {
            return Err(ReqvireError::DuplicateElement(
                format!("Target file '{}' already exists", target_file)
            ));
        }

        // Track old -> new identifier mappings
        let mut identifier_mappings: Vec<(String, String)> = Vec::new();
        let mut modified_files = vec![source_file.to_string()];

        // In squash mode, move elements to target file
        if squash && target_exists {
            // Move each element to target file
            for old_id in &elements_in_source {
                let slug = if let Some(pos) = old_id.rfind('#') {
                    &old_id[pos+1..]
                } else {
                    continue;
                };
                let new_id = format!("{}#{}", target_file, slug);

                // Update element
                if let Some(node) = self.nodes.get_mut(old_id) {
                    node.element.file_path = target_file.to_string();
                    node.element.identifier = new_id.clone();
                }

                identifier_mappings.push((old_id.clone(), new_id.clone()));
            }
        } else {
            // Normal mode: move entire file (keep sections as-is)
            for old_id in &elements_in_source {
                let slug = if let Some(pos) = old_id.rfind('#') {
                    &old_id[pos+1..]
                } else {
                    continue;
                };
                let new_id = format!("{}#{}", target_file, slug);
                identifier_mappings.push((old_id.clone(), new_id.clone()));
            }

            // Update all elements in the source file
            for (old_id, new_id) in &identifier_mappings {
                if let Some(node) = self.nodes.get_mut(old_id) {
                    node.element.file_path = target_file.to_string();
                    node.element.identifier = new_id.clone();
                }
            }
        }

        // Find all files with relations to elements in the source file
        for node in self.nodes.values() {
            let has_relation = node.element.relations.iter().any(|rel| {
                if let LinkType::Identifier(id) = &rel.target.link {
                    elements_in_source.contains(id)
                } else {
                    false
                }
            });
            if has_relation {
                let file = node.element.file_path.clone();
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Move nodes in HashMap (remove old key, insert with new key)
        for (old_id, new_id) in &identifier_mappings {
            if let Some(node) = self.nodes.remove(old_id) {
                self.nodes.insert(new_id.clone(), node);
            }
        }

        // Update all relations pointing to moved elements
        for (old_id, new_id) in &identifier_mappings {
            for node in self.nodes.values_mut() {
                for relation in &mut node.element.relations {
                    if let LinkType::Identifier(ref mut target_id) = relation.target.link {
                        if target_id == old_id {
                            *target_id = new_id.clone();
                        }
                    }
                }
            }
        }

        modified_files.push(target_file.to_string());

        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok(identifier_mappings)
    }

    /// Flushes all elements to markdown files and copies InternalPath files to the specified directory
    pub fn flush_to_directory(&self, output_dir: &Path) -> Result<(usize, usize), ReqvireError> {
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .map_err(|e| ReqvireError::IoError(e))?;
        }

        // Generate and write markdown files
        let grouped_elements = self.group_elements_by_location();
        let mut markdown_files_written = 0;

        for (file_path, elements) in grouped_elements {
            // Generate the markdown content for this file
            let markdown_content = self.generate_file_markdown(&file_path, &elements);

            // Determine the output file path
            let output_file_path = output_dir.join(&file_path);

            // Create parent directories if needed
            if let Some(parent_dir) = output_file_path.parent() {
                fs::create_dir_all(parent_dir)
                    .map_err(|e| ReqvireError::IoError(e))?;
            }

            // Write the markdown file
            fs::write(&output_file_path, markdown_content)
                .map_err(|e| ReqvireError::IoError(e))?;

            debug!("Flushed {} elements to {}",
                elements.len(),
                output_file_path.display()
            );

            markdown_files_written += 1;
        }

        // Copy InternalPath files
        let internal_paths = self.collect_internal_path_targets();
        let internal_files_copied = self.copy_internal_path_files(&internal_paths, output_dir)?;

        log::info!("Successfully flushed {} markdown files and copied {} internal files to {}",
                   markdown_files_written, internal_files_copied, output_dir.display());

        Ok((markdown_files_written, internal_files_copied))
    }

    /// Flushes elements from specific files to markdown files and copies related InternalPath files
    pub fn flush_files_to_directory(&self, file_paths: &[String], output_dir: &Path) -> Result<(usize, usize), ReqvireError> {
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .map_err(|e| ReqvireError::IoError(e))?;
        }

        let grouped_elements = self.group_elements_by_location();
        let mut markdown_files_written = 0;
        let mut related_internal_paths = HashSet::new();

        for file_path in file_paths {
            if let Some(elements) = grouped_elements.get(file_path) {
                // Generate the markdown content for this file
                let markdown_content = self.generate_file_markdown(file_path, elements);

                // Determine the output file path
                let output_file_path = output_dir.join(file_path);

                // Create parent directories if needed
                if let Some(parent_dir) = output_file_path.parent() {
                    fs::create_dir_all(parent_dir)
                        .map_err(|e| ReqvireError::IoError(e))?;
                }

                // Write the markdown file
                fs::write(&output_file_path, markdown_content)
                    .map_err(|e| ReqvireError::IoError(e))?;

                // Collect InternalPath relations from elements in this file
                for element in elements {
                    for relation in &element.relations {
                        if let LinkType::InternalPath(ref path) = relation.target.link {
                            related_internal_paths.insert(path.clone());
                        }
                    }
                }

                debug!("Flushed {} elements to {}",
                    elements.len(),
                    output_file_path.display()
                );

                markdown_files_written += 1;
            }
        }

        // Copy related InternalPath files
        let internal_files_copied = self.copy_internal_path_files(&related_internal_paths, output_dir)?;

        log::info!("Successfully flushed {} markdown files and copied {} internal files to {}",
                   markdown_files_written, internal_files_copied, output_dir.display());

        Ok((markdown_files_written, internal_files_copied))
    }


    // Dynamic graph manipulation methods

    /// Updates relation identifiers when elements move between files
    fn update_relation_identifiers(&mut self, moved_element_id: &str, _old_file_path: &str, new_file_path: &str) {
        // Extract just the fragment (element name) from the moved element's identifier
        let moved_fragment = moved_element_id.split('#').last().unwrap_or(moved_element_id);

        // 1. Update relations FROM other elements TO the moved element
        for (_id, node) in self.nodes.iter_mut() {
            // Skip the moved element itself for now
            if node.element.identifier == moved_element_id {
                continue;
            }

            // Update relations in the Element object (for markdown generation)
            for relation in &mut node.element.relations {
                if let crate::relation::LinkType::Identifier(ref mut target_id) = relation.target.link {
                    if target_id == moved_element_id {
                        // The target element moved to a different file
                        if node.element.file_path != new_file_path {
                            // Cross-file reference needed - use just the fragment
                            *target_id = format!("{}#{}", new_file_path, moved_fragment);
                            relation.target.text = format!("{}#{}", new_file_path, moved_fragment);
                        }
                        // If same file, keep as-is
                    }
                }
            }
        }

        // 2. Update relations FROM the moved element TO other elements
        // First collect target file paths to avoid borrowing issues
        let target_file_paths: std::collections::HashMap<String, String> = self.nodes.values()
            .map(|node| (node.element.identifier.clone(), node.element.file_path.clone()))
            .collect();

        if let Some(moved_node) = self.nodes.get_mut(moved_element_id) {
            for relation in &mut moved_node.element.relations {
                if let crate::relation::LinkType::Identifier(ref mut target_id) = relation.target.link {
                    // Extract the original target identifier (remove any file path prefix)
                    let original_target_id = if target_id.contains('#') {
                        target_id.split('#').last().unwrap_or("").to_string()
                    } else {
                        target_id.clone()
                    };

                    // Find the target element to check its file location
                    if let Some(target_file_path) = target_file_paths.get(&original_target_id) {
                        // If moved element is now in different file than target
                        if new_file_path != target_file_path {
                            // Update to cross-file reference
                            *target_id = format!("{}#{}", target_file_path, original_target_id);
                            relation.target.text = format!("{}#{}", target_file_path, original_target_id);
                        } else {
                            // Same file, use simple reference
                            *target_id = original_target_id.clone();
                            relation.target.text = original_target_id;
                        }
                    }
                }
            }
        }
    }

    /// Adds a new element to the graph
    pub fn add_element(&mut self, element: Element) -> Result<(), ReqvireError> {
        let element_id = element.identifier.clone();

        if self.nodes.contains_key(&element_id) {
            return Err(ReqvireError::ElementMoveError(format!("Element '{}' already exists in the graph", element_id)));
        }

        self.nodes.insert(element_id, ElementNode {
            element,
            relations: Vec::new(),
        });

        Ok(())
    }

    /// Removes an element from the graph and all relations pointing to it
    pub fn remove_element(&mut self, element_id: &str) -> Result<(), ReqvireError> {
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!("Element '{}' not found in the graph", element_id)));
        }

        // Remove the element itself
        self.nodes.remove(element_id);

        // Remove all relations pointing to this element from graph structure
        for node in self.nodes.values_mut() {
            node.relations.retain(|rel| rel.element_node.element.identifier != element_id);
        }

        // Remove all relations pointing to this element from element's own relations list
        for node in self.nodes.values_mut() {
            node.element.relations.retain(|rel| {
                match &rel.target.link {
                    crate::relation::LinkType::Identifier(target) => {
                        // Remove if it points to the deleted element (handle both forms)
                        target != element_id && !target.ends_with(&format!("#{}", element_id))
                    },
                    _ => true, // Keep external links
                }
            });
        }

        Ok(())
    }

    /// Adds a relation between two elements in the graph
    pub fn add_relation(&mut self, source_id: &str, target_id: &str, relation_type: &str) -> Result<(), ReqvireError> {
        // Validate both elements exist
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::LocationNotFound(format!("Source element '{}' not found", source_id)));
        }
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::LocationNotFound(format!("Target element '{}' not found", target_id)));
        }

        // Check if relation type is valid for impact propagation
        if !relation::IMPACT_PROPAGATION_RELATIONS.contains(&relation_type) {
            return Err(ReqvireError::ProcessError(format!("Relation type '{}' is not valid for impact propagation", relation_type)));
        }

        // Get the target node to create the relation
        let target_node = self.nodes.get(target_id).unwrap().clone();

        // Add the relation to the source element
        let source_node = self.nodes.get_mut(source_id).unwrap();

        // Check if relation already exists
        let relation_exists = source_node.relations.iter().any(|rel|
            rel.element_node.element.identifier == target_id && rel.relation_trigger == relation_type
        );

        if relation_exists {
            return Err(ReqvireError::ProcessError(format!("Relation '{}' from '{}' to '{}' already exists", relation_type, source_id, target_id)));
        }

        source_node.relations.push(RelationNode {
            relation_trigger: relation_type.to_string(),
            element_node: target_node,
        });

        Ok(())
    }

    /// Removes a specific relation between two elements (graph structure only)
    pub fn remove_relation(&mut self, source_id: &str, target_id: &str, relation_type: &str) -> Result<(), ReqvireError> {
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::LocationNotFound(format!("Source element '{}' not found", source_id)));
        }

        let source_node = self.nodes.get_mut(source_id).unwrap();
        let initial_count = source_node.relations.len();

        source_node.relations.retain(|rel|
            !(rel.element_node.element.identifier == target_id && rel.relation_trigger == relation_type)
        );

        if source_node.relations.len() == initial_count {
            return Err(ReqvireError::ProcessError(format!("Relation '{}' from '{}' to '{}' not found", relation_type, source_id, target_id)));
        }

        Ok(())
    }

    /// Removes a relation from an element's relations array with bidirectional handling
    /// This removes the relation from element.relations (which gets written to markdown)
    /// and also removes the opposite relation if one exists
    pub fn remove_element_relation(&mut self, element_id: &str, target_id: &str, relation_type: &str) -> Result<(), ReqvireError> {
        // Check if source element exists
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!("Element '{}' not found", element_id)));
        }

        // Check if target element exists
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::LocationNotFound(format!("Target element '{}' not found", target_id)));
        }

        // Remove the relation from source element's relations array
        let source_node = self.nodes.get_mut(element_id).unwrap();
        let initial_count = source_node.element.relations.len();

        source_node.element.relations.retain(|rel| {
            !(rel.relation_type.name == relation_type &&
              matches!(&rel.target.link, crate::relation::LinkType::Identifier(id) if id == target_id))
        });

        if source_node.element.relations.len() == initial_count {
            return Err(ReqvireError::ProcessError(
                format!("Relation '{}' from '{}' to '{}' not found", relation_type, element_id, target_id)
            ));
        }

        // Check if this relation type has an opposite (bidirectional)
        if let Some(relation_info) = crate::relation::RELATION_TYPES.get(relation_type) {
            if let Some(opposite_type) = relation_info.opposite {
                // Remove the opposite relation from target element
                let target_node = self.nodes.get_mut(target_id).unwrap();
                target_node.element.relations.retain(|rel| {
                    !(rel.relation_type.name == opposite_type &&
                      matches!(&rel.target.link, crate::relation::LinkType::Identifier(id) if id == element_id))
                });
            }
        }

        Ok(())
    }

    /// Lists all relations for a given element
    pub fn list_relations(&self, element_id: &str) -> Result<Vec<(String, String)>, ReqvireError> {
        let node = self.nodes.get(element_id)
            .ok_or_else(|| ReqvireError::LocationNotFound(format!("Element '{}' not found", element_id)))?;

        let relations = node.relations.iter()
            .map(|rel| (rel.relation_trigger.clone(), rel.element_node.element.identifier.clone()))
            .collect();

        Ok(relations)
    }

    /// Gets statistics about the graph
    pub fn get_graph_stats(&self) -> (usize, usize) {
        let element_count = self.nodes.len();
        let relation_count = self.nodes.values()
            .map(|node| node.relations.len())
            .sum();

        (element_count, relation_count)
    }

    // ================================
    // CRUD Operations (Add, Delete, Move)
    // ================================

    /// Creates an element from markdown string and adds it to the graph
    /// Used by CLI add command
    pub fn create_element_from_string(
        &mut self,
        markdown: &str,
        target_file: &str,
        index: Option<usize>,
        excluded_patterns: &GlobSet,
    ) -> Result<Element, ReqvireError> {
        // Validate target path
        let validation = crate::utils::validate_target_path(target_file, None, excluded_patterns)?;

        if !validation.is_valid {
            return Err(ReqvireError::InvalidPath(
                validation.error_message.unwrap_or_else(|| "Invalid target path".to_string())
            ));
        }

        // Parse element from markdown string
        let element = crate::parser::parse_single_element(markdown, target_file)?;

        // Check for duplicate element name (global uniqueness)
        if self.nodes.contains_key(&element.identifier) {
            return Err(ReqvireError::DuplicateElement(
                format!("Element '{}' already exists in the model", element.name)
            ));
        }

        // Validate that all relation targets exist in the model
        // External links (http://, https://, etc.) are allowed and not validated
        for relation in &element.relations {
            if let crate::relation::LinkType::Identifier(target_id) = &relation.target.link {
                // Check if this is an external link using the predefined list
                let is_external = crate::utils::EXTERNAL_SCHEMES.iter()
                    .any(|scheme| target_id.starts_with(scheme));

                // If not external, validate that the target exists
                if !is_external && !self.nodes.contains_key(target_id) {
                    return Err(ReqvireError::MissingElement(
                        format!(
                            "Relation target '{}' does not exist in the model. Cannot add element '{}' with relation to non-existent element.",
                            target_id,
                            element.name
                        )
                    ));
                }
            }
        }

        // Auto-create file if needed
        if validation.needs_file_creation {
            self.add_file_location(target_file)?;

            // Add page content (file header based on filename)
            let file_stem = Path::new(target_file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Document");

            self.register_page(target_file.to_string(), format!("# {}\n", file_stem));
        }

        // Set file_order_index if index provided
        let mut new_element = element.clone();
        if let Some(idx) = index {
            new_element.file_order_index = idx;
        } else {
            // Default: append to end of file
            let max_index = self.nodes.values()
                .filter(|node| node.element.file_path == target_file)
                .map(|node| node.element.file_order_index)
                .max()
                .unwrap_or(0);
            new_element.file_order_index = max_index + 1;
        }

        // Add to graph
        self.add_element(new_element.clone())?;

        // Track modified file
        self.modified_files.insert(target_file.to_string());

        Ok(new_element)
    }

    /// Enhanced remove element that tracks modifications and performs cleanup
    pub fn remove_element_with_cleanup(&mut self, element_id: &str) -> Result<Vec<String>, ReqvireError> {
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(
                format!("Element '{}' not found in the graph", element_id)
            ));
        }

        // Get element info before removal
        let element = &self.nodes.get(element_id).unwrap().element;
        let file_path = element.file_path.clone();

        // Track all files that will be modified
        let mut modified_files = vec![file_path.clone()];

        // Find all elements with relations pointing to this element
        for (other_id, node) in self.nodes.iter() {
            if other_id != element_id {
                let has_relation_to_target = node.element.relations.iter().any(|rel| {
                    matches!(&rel.target.link, LinkType::Identifier(id) if id == element_id)
                });

                if has_relation_to_target {
                    let other_file = node.element.file_path.clone();
                    if !modified_files.contains(&other_file) {
                        modified_files.push(other_file);
                    }
                }
            }
        }

        // Remove element and relations
        self.remove_element(element_id)?;

        // Track modified files
        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok(modified_files)
    }

    /// Checks if a file has no elements remaining
    pub fn is_file_empty(&self, file_path: &str) -> bool {
        !self.nodes.values().any(|node| node.element.file_path == file_path)
    }

    /// Comprehensive move operation with full relation updates and file tracking
    pub fn move_element_comprehensive(
        &mut self,
        element_id: &str,
        target_file: &str,
        index: Option<usize>,
        excluded_patterns: &GlobSet,
    ) -> Result<(String, Vec<String>), ReqvireError> {
        // Validate element exists
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(
                format!("Element '{}' not found", element_id)
            ));
        }

        // Get source file before move
        let source_file = self.nodes.get(element_id).unwrap().element.file_path.clone();

        // Validate target path
        let validation = crate::utils::validate_target_path(target_file, None, excluded_patterns)?;

        if !validation.is_valid {
            return Err(ReqvireError::InvalidPath(
                validation.error_message.unwrap_or_else(|| "Invalid target path".to_string())
            ));
        }

        // Auto-create file if needed
        if validation.needs_file_creation {
            self.add_file_location(target_file)?;

            let file_stem = Path::new(target_file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Document");

            self.register_page(target_file.to_string(), format!("# {}\n", file_stem));
        }

        // Perform the move using existing move_element_to_location
        let old_identifier = element_id.to_string();

        // Find all files with relations to this element BEFORE updating relations
        let mut modified_files = vec![source_file.clone()];
        if target_file != source_file {
            modified_files.push(target_file.to_string());
        }

        for node in self.nodes.values() {
            let has_relation = node.element.relations.iter().any(|rel| {
                matches!(&rel.target.link, LinkType::Identifier(id) if id == &old_identifier)
            });

            if has_relation {
                let file = node.element.file_path.clone();
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Now perform the move
        self.move_element_to_location(element_id, target_file)?;

        // Update all relations (TO and FROM the moved element)
        self.update_relation_identifiers(&old_identifier, &source_file, target_file);

        // Get new identifier after move
        let new_identifier = self.nodes.values()
            .find(|node| {
                node.element.file_path == target_file &&
                node.element.identifier.ends_with(&old_identifier.split('#').last().unwrap())
            })
            .map(|node| node.element.identifier.clone())
            .ok_or_else(|| ReqvireError::ProcessError(
                "Failed to find element after move".to_string()
            ))?;

        // Update file order index if provided
        if let Some(idx) = index {
            if let Some(node) = self.nodes.get_mut(&new_identifier) {
                node.element.file_order_index = idx;
            }
        }

        // Track all modified files
        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok((new_identifier, modified_files))
    }

    /// Flushes only modified files to directory (optimization)
    pub fn flush_modified_files(&self, directory: &Path) -> Result<(), ReqvireError> {
        if self.modified_files.is_empty() {
            return Ok(());
        }

        let file_vec: Vec<String> = self.modified_files.iter().cloned().collect();
        let _result = self.flush_files_to_directory(&file_vec, directory)?;

        // Check for and delete empty files (files with no elements)
        let grouped_elements = self.group_elements_by_location();
        for file_path in &file_vec {
            if !grouped_elements.contains_key(file_path) {
                // This file has no elements, delete it
                let file_full_path = directory.join(file_path);
                if file_full_path.exists() {
                    fs::remove_file(&file_full_path)
                        .map_err(|e| ReqvireError::IoError(e))?;
                    log::info!("Deleted empty file: {}", file_path);
                }
            }
        }

        Ok(())
    }

    /// Clears the modified files tracking
    pub fn clear_modified_files(&mut self) {
        self.modified_files.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{Element, ElementType, RequirementType};
    use crate::relation::{Relation, RelationTarget, LinkType, RELATION_TYPES};

    fn make_element(id: &str, name: &str) -> Element {
        let mut element = Element::new(
            name,
            id,
            "file.md",
            1, // Test elements at line 1
            Some(ElementType::Requirement(RequirementType::System)),
        );
        element.content = format!("This is {}", name);
        element.freeze_content();
        element
    }

    fn add_relation(from: &mut Element, relation_type: &'static str, to_id: &str) {
        let relation_info = RELATION_TYPES.get(relation_type).unwrap();
        // Extract element_id from identifier (fragment after #)
        let element_id = crate::utils::extract_path_and_fragment(to_id).1.map(|f| f.to_string());
        from.relations.push(Relation {
            relation_type: relation_info,
            target: RelationTarget {
                text: to_id.to_string(),
                link: LinkType::Identifier(to_id.to_string()),
                element_id,
            },
            user_created: true,
        });
    }

    #[test]
    fn test_graph_from_registry_resolves_forward_links() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        let b = make_element("B", "Element B");

        add_relation(&mut a, "derive", "B");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();

        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.relations.len(), 1);
        assert_eq!(a_node.relations[0].relation_trigger, "derive");
        assert_eq!(a_node.relations[0].element_node.element.identifier, "B");
    }

    #[test]
    fn test_update_identifier_updates_links_and_graph() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        let b = make_element("B", "Element B");

        add_relation(&mut a, "derive", "B");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();
        graph.update_identifier("B", "B_NEW");

        // B should no longer exist, B_NEW should
        assert!(graph.nodes.get("B").is_none());
        assert!(graph.nodes.get("B_NEW").is_some());

        // A's relation should now point to B_NEW
        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.relations.len(), 1);
        assert_eq!(a_node.relations[0].element_node.element.identifier, "B_NEW");
    }

    #[test]
    fn test_get_impact_tree_traverses_correctly() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        let mut b = make_element("B", "Element B");
        let c = make_element("C", "Element C");

        add_relation(&mut a, "derive", "B");
        add_relation(&mut b, "derive", "C");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();
        registry.register_element(c.clone(), "file.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();
        let tree = graph.get_impact_tree("A");

        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);

        let b_node = &tree.relations[0].element_node;
        assert_eq!(b_node.element.identifier, "B");
        assert_eq!(b_node.relations.len(), 1);
        assert_eq!(b_node.relations[0].element_node.element.identifier, "C");
    }

    #[test]
    fn test_cycle_is_handled_gracefully() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        let mut b = make_element("B", "Element B");

        // A -> B and B -> A (cycle)
        add_relation(&mut a, "derive", "B");
        add_relation(&mut b, "derive", "A");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();
        let tree = graph.get_impact_tree("A");

        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);
        assert_eq!(tree.relations[0].element_node.element.identifier, "B");

        // Because of cycle protection, B should not recurse into A again
        assert_eq!(tree.relations[0].element_node.relations.len(), 0);
    }

    #[test]
    fn test_move_element_to_existing_location() {
        let mut registry = GraphRegistry::new();

        // Create elements in different files
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file2.md".to_string();

        add_relation(&mut a, "derivedFrom", "B");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();

        let mut graph = registry;

        // Move A to B's file
        let result = graph.move_element_to_location("A", "file2.md");
        assert!(result.is_ok());

        // Verify A is now in file2.md
        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.element.file_path, "file2.md");
    }

    #[test]
    fn test_move_element_to_nonexistent_location() {
        let mut registry = GraphRegistry::new();
        let a = make_element("A", "Element A");

        registry.register_element(a.clone(), "file.md").unwrap();
        let mut graph = registry;

        // Try to move to non-existent file
        let result = graph.move_element_to_location("A", "nonexistent.md");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist in the graph"));
    }

    #[test]
    fn test_get_available_locations() {
        let mut registry = GraphRegistry::new();

        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file2.md".to_string();

        let mut c = make_element("C", "Element C");
        c.file_path = "file1.md".to_string(); // Same file as A

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();
        registry.register_element(c.clone(), "file1.md").unwrap();

        let graph = registry;
        let locations = graph.get_available_locations();

        // Should only have 2 unique files
        assert_eq!(locations.len(), 2);
        assert!(locations.contains(&"file1.md".to_string()));
        assert!(locations.contains(&"file2.md".to_string()));
    }

    #[test]
    fn test_get_move_impact() {
        let mut registry = GraphRegistry::new();
        let a = make_element("A", "Element A");
        let mut b = make_element("B", "Element B");
        let mut c = make_element("C", "Element C");

        // B and C both reference A
        add_relation(&mut b, "derive", "A");
        add_relation(&mut c, "derivedFrom", "A");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();
        registry.register_element(c.clone(), "file.md").unwrap();

        let graph = registry;
        let impact = graph.get_move_impact("A");

        // Both B and C should be affected by moving A
        assert_eq!(impact.len(), 2);
        assert!(impact.contains(&"B".to_string()));
        assert!(impact.contains(&"C".to_string()));
    }

    #[test]
    fn test_move_element_to_new_file() {
        let mut registry = GraphRegistry::new();
        let a = make_element("A", "Element A");

        registry.register_element(a.clone(), "file.md").unwrap();
        let mut graph = registry;

        // Move A to a new file
        let result = graph.move_element_to_new_file("A", "new_file.md");
        assert!(result.is_ok());

        // Verify A is now in the new file
        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.element.file_path, "new_file.md");
    }

    #[test]
    fn test_add_file_location() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        a.file_path = "existing.md".to_string();

        registry.register_element(a.clone(), "existing.md").unwrap();
        let mut graph = registry;

        // Add a new file location
        let result = graph.add_file_location("new_file.md");
        assert!(result.is_ok());

        // Try to add the same file again (should fail)
        let result = graph.add_file_location("existing.md");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_move_element_updates_relation_identifiers() {
        let mut registry = GraphRegistry::new();

        // Create elements A, B, C
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file1.md".to_string();

        let mut c = make_element("C", "Element C");
        c.file_path = "file2.md".to_string();

        // Create relations: B -> A, C -> A
        add_relation(&mut b, "derive", "A");
        add_relation(&mut c, "derive", "A");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file1.md").unwrap();
        registry.register_element(c.clone(), "file2.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();

        // Verify initial relations exist
        let b_relations = graph.list_relations("B").unwrap();
        let c_relations = graph.list_relations("C").unwrap();
        assert_eq!(b_relations.len(), 1);
        assert_eq!(c_relations.len(), 1);
        assert_eq!(b_relations[0], ("derive".to_string(), "A".to_string()));
        assert_eq!(c_relations[0], ("derive".to_string(), "A".to_string()));

        // Move A to a new file - this should update its identifier
        let result = graph.move_element_to_new_file("A", "file3.md");
        assert!(result.is_ok());

        // Check that A's location has changed
        let a_element = graph.get_element("A").unwrap();
        assert_eq!(a_element.file_path, "file3.md");

        // CRITICAL: Relations from B and C should still point to A
        // But they should be pointing to the NEW identifier if A's identifier changed
        let b_relations_after = graph.list_relations("B").unwrap();
        let c_relations_after = graph.list_relations("C").unwrap();

        // These should still exist and point to the moved element
        assert_eq!(b_relations_after.len(), 1, "B should still have 1 relation after A is moved");
        assert_eq!(c_relations_after.len(), 1, "C should still have 1 relation after A is moved");

        // The target should still be "A" (or updated identifier if it changed)
        let b_target = &b_relations_after[0].1;
        let c_target = &c_relations_after[0].1;

        // Verify the targets still exist in the graph
        assert!(graph.get_element(b_target).is_some(), "B's relation target '{}' should exist in graph", b_target);
        assert!(graph.get_element(c_target).is_some(), "C's relation target '{}' should exist in graph", c_target);

    }

    #[test]
    fn test_move_element_updates_identifiers_in_flushed_markdown() {
        let mut registry = GraphRegistry::new();

        // Create elements A, B where B references A
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file1.md".to_string();

        // B has a relation pointing to A
        add_relation(&mut b, "derivedFrom", "A");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file1.md").unwrap();

        let mut graph = registry;

        // Move A to a different file
        let result = graph.move_element_to_new_file("A", "file2.md");
        assert!(result.is_ok());

        // Check B's original relations in the element

        // The issue: B's element still has a relation pointing to "A"
        // But A is now in file2.md, so the relation should be "file2.md#A" if it's a cross-file reference
        // Or if identifiers include file paths, it should be updated accordingly

        // Check if this would cause issues in markdown generation
        // We expect that when we flush, the relations should be correctly written
        // based on the current location of elements

        // The PROBLEM: B's relation still points to "A" but A is now in a different file
        // When B gets written to file1.md, it should reference A as "file2.md#A" not just "A"

        // Let's check what the markdown would look like:
        let b_element = graph.nodes.get("B").unwrap().element.clone();
        let b_markdown = graph.element_to_markdown_with_context(&b_element, "file1.md");
        println!("B's markdown after A is moved:");
        println!("{}", b_markdown);

        // The relation should be "file2.md#A" since A is now in a different file
        // but it's probably still "A" which would be incorrect
        assert!(b_markdown.contains("file2.md#A") || b_markdown.contains("[A](file2.md#A)"),
                "B's relation should reference A in its new location: {}", b_markdown);
    }

    #[test]
    fn test_moved_element_relations_update_paths() {
        let mut registry = GraphRegistry::new();

        // Create elements A, B, C where A has relations to both B and C
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file2.md".to_string();  // B is in different file

        let mut c = make_element("C", "Element C");
        c.file_path = "file1.md".to_string();  // C is in same file as A initially

        // A has relations to both B (cross-file) and C (same-file)
        add_relation(&mut a, "derivedFrom", "B");
        add_relation(&mut a, "derive", "C");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();
        registry.register_element(c.clone(), "file1.md").unwrap();

        let mut graph = registry;

        // Check A's initial relations in markdown
        let a_element_initial = graph.nodes.get("A").unwrap().element.clone();
        let a_markdown_initial = graph.element_to_markdown_with_context(&a_element_initial, "file1.md");
        println!("A's initial markdown (in file1.md):");
        println!("{}", a_markdown_initial);

        // A is in file1.md, B is in file2.md, C is in file1.md
        // So A should reference B as "file2.md#B" and C as just "C" (same file)

        // Move A to file3.md
        let result = graph.move_element_to_new_file("A", "file3.md");
        assert!(result.is_ok());

        // Check A's relations after the move
        let a_element_moved = graph.nodes.get("A").unwrap().element.clone();
        let a_markdown_moved = graph.element_to_markdown_with_context(&a_element_moved, "file3.md");
        println!("A's markdown after move to file3.md:");
        println!("{}", a_markdown_moved);

        // Now A is in file3.md, so:
        // - A should reference B as "file2.md#B" (cross-file, B is in file2.md)
        // - A should reference C as "file1.md#C" (cross-file, C is in file1.md)
        // Both should be cross-file references now since A moved to file3.md

        println!("A's relations after move:");
        for relation in &a_element_moved.relations {
            println!("  {} -> {}", relation.relation_type.name,
                    match &relation.target.link {
                        crate::relation::LinkType::Identifier(id) => id.clone(),
                        crate::relation::LinkType::InternalPath(path) => path.to_string_lossy().to_string(),
                        crate::relation::LinkType::ExternalUrl(url) => url.clone(),
                    });
        }

        // PROBLEM: A's relations likely still point to "B" and "C"
        // but should now point to "file2.md#B" and "file1.md#C" respectively
        // since A is now in a different file than both of them

        assert!(a_markdown_moved.contains("file2.md#B") || a_markdown_moved.contains("[B](file2.md#B)"),
                "A should reference B with file path since they're in different files: {}", a_markdown_moved);
        assert!(a_markdown_moved.contains("file1.md#C") || a_markdown_moved.contains("[C](file1.md#C)"),
                "A should reference C with file path since they're in different files: {}", a_markdown_moved);
    }

    #[test]
    fn test_flush_creates_proper_markdown_with_cross_file_relations() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create elements in different files with cross-file relations
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("ElementB", "Element B Description");
        b.file_path = "file2.md".to_string();

        let mut c = make_element("ElementC", "Element C Description");
        c.file_path = "file1.md".to_string(); // Same file as A

        // Create cross-file relations:
        // A -> B (file1.md -> file2.md)
        // B -> A (file2.md -> file1.md)
        // A -> C (file1.md -> file1.md, same file)
        add_relation(&mut a, "derivedFrom", "ElementB");
        add_relation(&mut a, "derive", "ElementC");
        add_relation(&mut b, "derivedFrom", "ElementA");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();
        registry.register_element(c.clone(), "file1.md").unwrap();

        let mut graph = registry;

        // Move ElementB to file3.md to create more cross-file relations
        let result = graph.move_element_to_new_file("ElementB", "file3.md");
        assert!(result.is_ok());

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path);
        assert!(result.is_ok());

        // List what files were actually created
        println!("Files created in output directory:");
        for entry in fs::read_dir(output_path).unwrap() {
            let entry = entry.unwrap();
            println!("  {}", entry.file_name().to_string_lossy());
        }

        // Read the generated markdown files and verify their content
        let file1_content = fs::read_to_string(output_path.join("file1.md")).unwrap();
        let file3_content = fs::read_to_string(output_path.join("file3.md")).unwrap();

        // file2.md might not exist if it only contained ElementB which moved to file3.md
        let file2_content = fs::read_to_string(output_path.join("file2.md")).unwrap_or_else(|_| {
            println!("file2.md does not exist (expected if no elements remain in it)");
            String::new()
        });

        println!("=== file1.md content ===");
        println!("{}", file1_content);
        println!("=== file2.md content ===");
        println!("{}", file2_content);
        println!("=== file3.md content ===");
        println!("{}", file3_content);

        // Verify file1.md content (contains ElementA and ElementC)
        assert!(file1_content.contains("### Element A Description"));
        assert!(file1_content.contains("### Element C Description"));
        assert!(!file1_content.contains("### Element B Description")); // ElementB moved to file3.md

        // Verify ElementA's relations in file1.md
        // A -> B should be cross-file reference with proper display name and fragment anchor
        assert!(file1_content.contains("[Element B Description](file3.md#ElementB)"),
                "ElementA should reference ElementB with proper display name: {}", file1_content);

        // A -> C should be same-file reference (no file prefix needed)
        assert!(file1_content.contains("[ElementC](ElementC)") ||
                file1_content.contains("[ElementC](#ElementC)") ||
                file1_content.contains("ElementC"),
                "ElementA should reference ElementC in same file: {}", file1_content);

        // Verify file3.md content (contains ElementB)
        assert!(file3_content.contains("### Element B Description"));
        assert!(!file3_content.contains("### Element A Description"));
        assert!(!file3_content.contains("### Element C Description"));

        // Verify ElementB's relations in file3.md
        // B -> A should be cross-file reference with proper display name and fragment anchor
        assert!(file3_content.contains("[Element A Description](file1.md#ElementA)"),
                "ElementB should reference ElementA with proper display name: {}", file3_content);

        // Verify no virtual placeholder content appears in any file
        assert!(!file1_content.contains("Virtual placeholder"));
        assert!(!file2_content.contains("Virtual placeholder"));
        assert!(!file3_content.contains("Virtual placeholder"));

        // Verify proper markdown structure - all files start with "# Requirements"
        assert!(file1_content.starts_with("# Requirements\n"));
        assert!(file3_content.starts_with("# Requirements\n"));
    }

    #[test]
    fn test_flush_includes_page_content() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create an element
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "test_file.md".to_string();

        registry.register_element(a.clone(), "test_file.md").unwrap();

        // Add page content
        let page = Page::new("This is page frontmatter content.\n\nMore page content here.".to_string());
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("test_file.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // Verify file header is present - all files start with "# Requirements"
        assert!(file_content.starts_with("# Requirements\n\n"));

        // Verify page content is included after header and before elements
        assert!(file_content.contains("This is page frontmatter content."));
        assert!(file_content.contains("More page content here."));

        // Verify element is still present
        assert!(file_content.contains("### Element A Description"));

        // Verify order: header, page content, element
        let header_pos = file_content.find("# Requirements").unwrap();
        let page_content_pos = file_content.find("This is page frontmatter content.").unwrap();
        let element_pos = file_content.find("### Element A Description").unwrap();

        assert!(header_pos < page_content_pos);
        assert!(page_content_pos < element_pos);
    }

    #[test]
    fn test_flush_multiple_elements() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create multiple elements
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "test_file.md".to_string();
        a.file_order_index = 1;

        let mut b = make_element("ElementB", "Element B Description");
        b.file_path = "test_file.md".to_string();
        b.file_order_index = 2;

        registry.register_element(a.clone(), "test_file.md").unwrap();
        registry.register_element(b.clone(), "test_file.md").unwrap();

        // Add page content
        let page = Page::new("Page frontmatter content.".to_string());
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("test_file.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // Verify all content is present
        assert!(file_content.contains("Page frontmatter content."));
        assert!(file_content.contains("### Element A Description"));
        assert!(file_content.contains("### Element B Description"));
    }

    #[test]
    fn test_flush_handles_empty_page_content() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create an element
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "test_file.md".to_string();

        registry.register_element(a.clone(), "test_file.md").unwrap();

        // Add empty page content (should be skipped)
        let page = Page::new("   \n\t  \n  ".to_string()); // only whitespace
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("test_file.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // Verify element is still present
        assert!(file_content.contains("### Element A Description"));
    }

    #[test]
    fn test_flush_always_outputs_requirements_header() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create an element in MOEs.md
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "MOEs.md".to_string();

        registry.register_element(a.clone(), "MOEs.md").unwrap();

        // Add page content (without header - parser strips the H1)
        let page = Page::new("This is the MOEs page content.".to_string());
        registry.pages.insert("MOEs.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("MOEs.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // All specification files should start with "# Requirements"
        assert!(file_content.starts_with("# Requirements\n\n"));

        // Page content should be included after the header
        assert!(file_content.contains("This is the MOEs page content."));
    }
}
