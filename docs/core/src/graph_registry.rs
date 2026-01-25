use std::collections::{HashMap, BTreeSet, HashSet, BTreeMap};
use std::fs;
use std::path::{Path, PathBuf};
use log::{debug, warn};
use serde::Serialize;

use crate::Relation;
use crate::relation::{self, LinkType, get_hierarchical_relation_types, IMPACT_PROPAGATION_RELATIONS, SATISFACTION_RELATIONS};
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

        // Validate Refinement elements have no relations
        errors.extend(self.validate_refinement_elements()?);

        // Validate 'other' type elements only use trace relations
        errors.extend(self.validate_other_element_relations()?);

        // Validate no cross-section duplicates (same target in Relations and Attachments)
        errors.extend(self.validate_cross_section_duplicates()?);

        Ok(errors)
    }

    /// Validates that no element has the same target in both Relations and Attachments subsections
    fn validate_cross_section_duplicates(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        log::debug!("Running cross-section duplicate validation...");
        let mut errors = Vec::new();

        for (_identifier, node) in &self.nodes {
            let element = &node.element;

            // Collect all relation targets (normalized identifiers)
            let relation_targets: std::collections::HashSet<String> = element.relations.iter()
                .filter(|r| r.user_created)
                .map(|r| r.target.link.as_str().to_string())
                .collect();

            // Check attachments against relations
            for attachment in &element.attachments {
                let attachment_target = attachment.target.as_str();
                if relation_targets.contains(&attachment_target) {
                    let msg = format!(
                        "Cross-section duplicate in element '{}': target '{}' appears in both Relations and Attachments (file: {})",
                        element.name, attachment_target, element.file_path
                    );
                    errors.push(ReqvireError::CrossSectionDuplicate(msg));
                }
            }
        }

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

    /// Adds opposite relation to target element (in-memory only)
    ///
    /// Creates opposite relation using `relation.to_opposite()` and adds it to the target element.
    /// Does NOT mark target file as modified since opposites are auto-generated (user_created=false).
    ///
    /// # Arguments
    /// * `relation` - The original relation that needs an opposite
    /// * `source_id` - Full identifier of the source element
    /// * `source_name` - Name of the source element
    /// * `source_element_id` - Fragment/ID of the source element
    ///
    /// # Returns
    /// * `true` if opposite was added successfully
    /// * `false` if target element not found or relation has no opposite
    fn add_opposite_to_target(
        &mut self,
        relation: &Relation,
        source_id: &str,
        source_name: &str,
        source_element_id: &str,
    ) -> bool {
        // Check if relation has an opposite type
        if relation.relation_type.opposite.is_none() {
            return false;
        }

        // Only process relations to elements (not external URLs/paths)
        let target_id = match &relation.target.link {
            crate::relation::LinkType::Identifier(id) => id.as_str(),
            _ => return false,
        };

        // Check if target element exists
        if !self.nodes.contains_key(target_id) {
            return false;
        }

        // Create opposite relation using existing to_opposite() method
        if let Some(opposite_relation) = relation.to_opposite(source_name, source_id, source_element_id) {
            // Add opposite to target element
            if let Some(target_node) = self.nodes.get_mut(target_id) {
                target_node.element.relations.push(opposite_relation);
                // Do NOT mark target file as modified - opposite is auto-generated (user_created=false)
                return true;
            }
        }

        false
    }

    /// Removes opposite relation from target element
    ///
    /// Tracks file modification only if opposite was user_created (written to file).
    /// Removes both user_created and auto-generated opposites from in-memory model.
    ///
    /// # Arguments
    /// * `target_id` - Full identifier of element to remove opposite from
    /// * `source_id` - Full identifier of source element (opposite points to this)
    /// * `opposite_type_name` - Name of opposite relation type
    fn remove_opposite_from_target(
        &mut self,
        target_id: &str,
        source_id: &str,
        opposite_type_name: &str,
    ) {
        if let Some(target_node) = self.nodes.get(target_id) {
            // Check if target has user_created opposite (written to file)
            let had_user_created_opposite = target_node.element.relations.iter().any(|r| {
                r.user_created &&
                r.relation_type.name == opposite_type_name &&
                r.target.link.as_str() == source_id
            });

            let target_file_path = target_node.element.file_path.clone();

            // Remove opposite relation (both user_created and auto-generated)
            let target_node = self.nodes.get_mut(target_id).unwrap();
            target_node.element.relations.retain(|r| {
                !(r.relation_type.name == opposite_type_name &&
                  r.target.link.as_str() == source_id)
            });

            // Mark file as modified only if opposite was user_created
            if had_user_created_opposite {
                self.modified_files.insert(target_file_path);
            }
        }
    }

    /// Recreates opposite relations with updated identifiers after moving an element
    ///
    /// When an element is moved, its identifier changes. This function updates all opposite
    /// relations that point to the moved element, removing old opposites and creating new ones.
    ///
    /// # Arguments
    /// * `old_id` - Old full identifier of the moved element
    /// * `new_id` - New full identifier of the moved element
    fn recreate_opposites_after_move(
        &mut self,
        old_id: &str,
        new_id: &str,
    ) {
        // Get moved element info
        let (moved_name, moved_element_id, relations_to_process) = if let Some(moved_node) = self.nodes.get(new_id) {
            let name = moved_node.element.name.clone();
            let element_id = moved_node.element.id.clone();
            // Only process user_created relations (these have opposites that need updating)
            let relations: Vec<_> = moved_node.element.relations.iter()
                .filter(|r| r.user_created)
                .cloned()
                .collect();
            (name, element_id, relations)
        } else {
            return; // Moved element not found
        };

        // For each user_created relation in moved element, update its opposite
        for relation in relations_to_process {
            if let Some(opposite_type_name) = relation.relation_type.opposite {
                // Get target identifier
                if let LinkType::Identifier(ref target_id) = relation.target.link {
                    // Remove old opposite (pointing to old_id)
                    self.remove_opposite_from_target(target_id, old_id, opposite_type_name);

                    // Create and add new opposite (pointing to new_id)
                    self.add_opposite_to_target(&relation, new_id, &moved_name, &moved_element_id);
                }
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
                match &attachment.target {
                    crate::element::AttachmentTarget::FilePath(file_path) => {
                        // Resolve attachment path relative to git root
                        let full_path = git_root.join(file_path);

                        if !full_path.exists() {
                            errors.push(ReqvireError::MissingAttachmentFile(
                                format!(
                                    "File {}: Element '{}' references missing attachment file: {}",
                                    element.file_path,
                                    element.name,
                                    file_path.display()
                                ),
                            ));
                        }
                    }
                    crate::element::AttachmentTarget::ElementIdentifier(identifier) => {
                        // Validate that the identifier points to an existing Refinement element
                        if let Some(target_node) = self.nodes.get(identifier) {
                            // Check if target is a Refinement element type
                            if !target_node.element.element_type.is_refinement() {
                                errors.push(ReqvireError::InvalidAttachmentTarget(
                                    format!(
                                        "File {}: Element '{}' has attachment to '{}' which is not a Refinement element (constraint, behavior, specification)",
                                        element.file_path,
                                        element.name,
                                        identifier
                                    ),
                                ));
                                continue;
                            }

                            // Check 1: Satisfied Refinement Constraint - refinement must have satisfy relations
                            if !self.refinement_has_satisfy_relations(identifier) {
                                errors.push(ReqvireError::InvalidAttachmentTarget(
                                    format!(
                                        "'{}' has no satisfy relations. Refinements must satisfy a requirement before they can be attached. (file: {}, element: {})",
                                        target_node.element.name,
                                        element.file_path,
                                        element.name
                                    ),
                                ));
                                continue;
                            }

                            // Check 2: Hierarchical Independence Constraint - attaching element must not be in defining hierarchy
                            let defining_reqs = self.get_defining_requirements(identifier);
                            for defining_req_id in defining_reqs {
                                if self.is_in_hierarchy(&element.identifier, &defining_req_id) {
                                    errors.push(ReqvireError::InvalidAttachmentScope(
                                        format!(
                                            "'{}' cannot be attached to '{}' because it is within the refinement's defining hierarchy. Attachments are only allowed from requirements outside the satisfiedBy chain. (file: {}, element: {})",
                                            target_node.element.name,
                                            element.name,
                                            element.file_path,
                                            element.name
                                        ),
                                    ));
                                    break;
                                }
                            }
                        } else {
                            errors.push(ReqvireError::MissingAttachmentTarget(
                                format!(
                                    "File {}: Element '{}' references missing attachment element: {}",
                                    element.file_path,
                                    element.name,
                                    identifier
                                ),
                            ));
                        }
                    }
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

    /// Get the defining requirements for a refinement element.
    /// A defining requirement is one that has a `satisfiedBy` relation pointing to the refinement.
    /// Returns a list of requirement identifiers.
    pub fn get_defining_requirements(&self, refinement_id: &str) -> Vec<String> {
        let mut defining_reqs = Vec::new();

        for (element_id, element_node) in &self.nodes {
            // Check if this element has a satisfiedBy relation pointing to the refinement
            for relation in &element_node.element.relations {
                // Use SATISFACTION_RELATIONS - satisfiedBy is the forward satisfaction relation from requirement
                if relation::is_satisfaction_relation(relation.relation_type)
                    && relation.relation_type.name == SATISFACTION_RELATIONS[1] // satisfiedBy
                {
                    if let LinkType::Identifier(target_id) = &relation.target.link {
                        if target_id == refinement_id {
                            defining_reqs.push(element_id.clone());
                        }
                    }
                }
            }
        }

        defining_reqs
    }

    /// Check if a refinement element has at least one `satisfy` relation.
    /// Returns true if the refinement has satisfy relations, false otherwise.
    pub fn refinement_has_satisfy_relations(&self, refinement_id: &str) -> bool {
        if let Some(node) = self.nodes.get(refinement_id) {
            node.element.relations.iter()
                // Use SATISFACTION_RELATIONS - satisfy is the backward satisfaction relation from refinement
                .any(|r| relation::is_satisfaction_relation(r.relation_type)
                    && r.relation_type.name == SATISFACTION_RELATIONS[0]) // satisfy
        } else {
            false
        }
    }

    /// Check if an element is in the derivation hierarchy of a root element.
    /// Returns true if element_id is the root itself, an ancestor, or a descendant of root_id.
    /// Used for attachment scope validation to check hierarchical independence.
    pub fn is_in_hierarchy(&self, element_id: &str, root_id: &str) -> bool {
        // Same element
        if element_id == root_id {
            return true;
        }

        // Check if element_id is an ancestor of root_id (element_id is above root_id)
        if self.is_ancestor_of(element_id, root_id) {
            return true;
        }

        // Check if element_id is a descendant of root_id (element_id is below root_id)
        if self.is_descendant_of(element_id, root_id) {
            return true;
        }

        false
    }

    /// Check if potential_ancestor is an ancestor of element_id via hierarchical relations.
    fn is_ancestor_of(&self, potential_ancestor: &str, element_id: &str) -> bool {
        let hierarchical_types = get_hierarchical_relation_types();
        let mut visited = HashSet::new();
        self.is_ancestor_of_recursive(potential_ancestor, element_id, &hierarchical_types, &mut visited)
    }

    fn is_ancestor_of_recursive(
        &self,
        potential_ancestor: &str,
        element_id: &str,
        hierarchical_types: &[&str],
        visited: &mut HashSet<String>,
    ) -> bool {
        if visited.contains(element_id) {
            return false;
        }
        visited.insert(element_id.to_string());

        if let Some(node) = self.nodes.get(element_id) {
            for relation in &node.element.relations {
                if hierarchical_types.contains(&relation.relation_type.name) {
                    if let LinkType::Identifier(parent_id) = &relation.target.link {
                        if parent_id == potential_ancestor {
                            return true;
                        }
                        if self.is_ancestor_of_recursive(potential_ancestor, parent_id, hierarchical_types, visited) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Check if potential_descendant is a descendant of element_id via hierarchical relations.
    fn is_descendant_of(&self, potential_descendant: &str, element_id: &str) -> bool {
        // A is descendant of B means B is ancestor of A
        self.is_ancestor_of(element_id, potential_descendant)
    }

    /// Validate Refinement element constraints
    /// Refinement elements (constraint, behavior, specification) can only have satisfy relations
    /// and cannot have attachments.
    fn validate_refinement_elements(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating Refinement element constraints...");
        let mut errors = Vec::new();

        for element_node in self.nodes.values() {
            let element = &element_node.element;

            // Check if this is a Refinement element type
            if element.element_type.is_refinement() {
                // Refinement elements can only have satisfy relations
                let invalid_relations: Vec<_> = element.relations.iter()
                    .filter(|r| r.user_created)
                    .filter(|r| r.relation_type.name.to_lowercase() != "satisfy")
                    .collect();

                if !invalid_relations.is_empty() {
                    let invalid_types: Vec<_> = invalid_relations.iter()
                        .map(|r| &r.relation_type.name)
                        .collect();
                    errors.push(ReqvireError::InvalidMarkdownStructure(
                        format!(
                            "File {}: Refinement element '{}' (type: {}) can only have satisfy relations. Invalid relations: {:?}",
                            element.file_path,
                            element.name,
                            element.element_type.as_str(),
                            invalid_types
                        ),
                    ));
                }

                // Refinement elements cannot have attachments
                if !element.attachments.is_empty() {
                    errors.push(ReqvireError::InvalidMarkdownStructure(
                        format!(
                            "File {}: Refinement element '{}' (type: {}) cannot have attachments. Refinement elements are atomic documentation units meant to be attached to requirements.",
                            element.file_path,
                            element.name,
                            element.element_type.as_str(),
                        ),
                    ));
                }
            }
        }

        if errors.is_empty() {
            debug!("No Refinement element validation errors found.");
        } else {
            debug!("{} Refinement element validation errors found.", errors.len());
        }

        Ok(errors)
    }

    /// Validates that 'other' type elements only use trace relations
    /// Returns a list of validation errors for 'other' elements using non-trace relations
    fn validate_other_element_relations(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating 'other' element type relation constraints...");
        let mut errors = Vec::new();

        for element_node in self.nodes.values() {
            let element = &element_node.element;

            // Check if this is an 'other' type element
            if let crate::element::ElementType::Other(type_str) = &element.element_type {
                if type_str == "other" {
                    // 'other' type can only use trace relations
                    let non_trace_relations: Vec<_> = element.relations.iter()
                        .filter(|r| r.user_created && r.relation_type.name != "trace")
                        .collect();

                    for relation in non_trace_relations {
                        errors.push(ReqvireError::IncompatibleElementTypes(
                            format!(
                                "Element type 'other' can only use 'trace' relations: '{}' uses '{}' relation to '{}'. See Element Type Relation Compatibility specification.",
                                element.identifier,
                                relation.relation_type.name,
                                &relation.target.text
                            )
                        ));
                    }
                }
            }
        }

        if errors.is_empty() {
            debug!("No 'other' element type relation errors found.");
        } else {
            debug!("{} 'other' element type relation errors found.", errors.len());
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
        let search_name = element_name.trim();

        // Find all elements with matching name
        let matching: Vec<&String> = self.nodes
            .iter()
            .filter(|(_, node)| node.element.name == search_name)
            .map(|(id, _)| id)
            .collect();

        if matching.is_empty() {
            return Err(ReqvireError::MissingElement(
                format!("Element not found: {}", element_name)
            ));
        } else if matching.len() > 1 {
            return Err(ReqvireError::ProcessError(
                format!("Multiple elements found with name '{}': {:?}", element_name, matching)
            ));
        }

        Ok(matching[0].clone())
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

    /// Find leaf elements for reverse traversal
    /// Leaf elements are those that:
    /// 1. Have backward relations (derivedFrom, satisfy, verify) - they trace upward to something
    /// 2. Have no outgoing forward relations to other elements - nothing derives from them
    /// Optionally filter by element types
    pub fn find_leaf_elements(&self, type_filter: Option<&[&str]>) -> Vec<String> {
        let mut leaves: Vec<String> = self.nodes.values()
            .map(|node| &node.element)
            .filter(|element| {
                // Apply type filter if provided
                if let Some(types) = type_filter {
                    let element_type_str = element.element_type.as_str();
                    if !types.iter().any(|t| *t == element_type_str) {
                        return false;
                    }
                }

                // Must have at least one backward relation (to trace upward)
                let has_backward_relations = element.relations.iter()
                    .any(|r| {
                        relation::BACKWARD_RELATIONS.contains(&r.relation_type.name) &&
                        matches!(r.target.link, relation::LinkType::Identifier(_))
                    });

                if !has_backward_relations {
                    return false;
                }

                // Must NOT have outgoing forward relations to elements (nothing derives from it)
                let has_forward_children = element.relations.iter()
                    .any(|r| {
                        relation::DIAGRAM_RELATIONS.contains(&r.relation_type.name) &&
                        matches!(r.target.link, relation::LinkType::Identifier(_))
                    });

                !has_forward_children
            })
            .map(|e| e.identifier.clone())
            .collect();

        // Sort for deterministic output
        leaves.sort();
        leaves
    }

    /// Find starting elements filtered by type (for both forward and reverse traversal)
    pub fn find_elements_by_type(&self, type_filter: &[&str]) -> Vec<String> {
        let mut elements: Vec<String> = self.nodes.values()
            .map(|node| &node.element)
            .filter(|element| {
                let element_type_str = element.element_type.as_str();
                type_filter.iter().any(|t| *t == element_type_str)
            })
            .map(|e| e.identifier.clone())
            .collect();

        // Sort for deterministic output
        elements.sort();
        elements
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


    fn element_to_markdown_with_context(&self, element: &Element, _current_file: &str, with_full_relations: bool) -> String {
        let mut markdown = String::new();

        // Add the element header
        markdown.push_str(&format!("### {}\n\n", element.name));

        // Add the element content
        if !element.content.trim().is_empty() {
            markdown.push_str(element.content.trim_end());
            markdown.push_str("\n");
        }

        // Add metadata subsection
        // Always include metadata to preserve structure during CRUD operations
        let mut custom_metadata: Vec<_> = element.metadata.iter()
            .filter(|(key, _)| *key != "type") // type is handled separately
            .collect();
        custom_metadata.sort_by_key(|(key, _)| *key);

        markdown.push_str("#### Metadata\n");

        // Add type metadata
        markdown.push_str(&format!("  * type: {}\n", element.element_type.as_str()));

        // Add other metadata
        for (key, value) in custom_metadata {
            markdown.push_str(&format!("  * {}: {}\n", key, value));
        }
        markdown.push_str("\n");

        // Add attachments subsection if there are attachments
        // Deduplicate attachments by target, keeping first occurrence
        let mut seen_attachments: std::collections::HashSet<String> = std::collections::HashSet::new();
        let unique_attachments: Vec<_> = element.attachments.iter()
            .filter(|a| seen_attachments.insert(a.target.as_str()))
            .collect();

        if !unique_attachments.is_empty() {
            markdown.push_str("#### Attachments\n");
            for attachment in unique_attachments {
                match &attachment.target {
                    crate::element::AttachmentTarget::FilePath(file_path) => {
                        // Attachment paths are stored as git-root-relative paths
                        let attachment_path = file_path.to_string_lossy().to_string();

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
                        let display_name = file_path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(&attachment_path);

                        markdown.push_str(&format!("  * [{}]({})\n", display_name, relative_path));
                    }
                    crate::element::AttachmentTarget::ElementIdentifier(identifier) => {
                        // Element identifier attachments - format as markdown link
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_folder = current_file_path.parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .to_path_buf();

                        // Use to_relative_identifier to make identifier relative to current file
                        let relative_id = crate::utils::to_relative_identifier(
                            identifier,
                            &current_folder,
                            true
                        ).unwrap_or_else(|_| identifier.clone());

                        // Look up actual element name from registry for human-readable display
                        let display_name = self.get_element(identifier)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| {
                                // Fallback to identifier fragment if element not found
                                identifier.split('#').last().unwrap_or(identifier).to_string()
                            });
                        markdown.push_str(&format!("  * [{}]({})\n", display_name, relative_id));
                    }
                }
            }
            markdown.push_str("\n");
        }

        // Add relations subsection if there are relations to include
        // When with_full_relations is true, include all relations (user-created and auto-generated)
        // Otherwise, only include user-created relations
        let mut relations_to_include: Vec<_> = if with_full_relations {
            element.relations.iter().collect()
        } else {
            element.relations.iter().filter(|r| r.user_created).collect()
        };
        // Sort relations for deterministic output: by relation type name, then by target link
        relations_to_include.sort_by(|a, b| {
            (&a.relation_type.name, a.target.link.as_str())
                .cmp(&(&b.relation_type.name, b.target.link.as_str()))
        });
        // Remove duplicate relations (same relation_type + same target), keeping first occurrence
        relations_to_include.dedup_by(|a, b| {
            a.relation_type.name == b.relation_type.name && a.target.link.as_str() == b.target.link.as_str()
        });
        if !relations_to_include.is_empty() {
            markdown.push_str("#### Relations\n");
            for relation in relations_to_include {
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

    /// Groups elements by their file path and orders them following Element Ordering Behavior
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

        // Apply Element Ordering Behavior to each file
        for elements in file_elements.values_mut() {
            self.order_elements_hierarchically(elements);
        }

        file_elements
    }

    /// Orders elements following Element Ordering Behavior:
    /// - Parent elements appear before their children (file-local derivedFrom hierarchy)
    /// - Root elements (no file-local parent) sorted alphabetically
    /// - Siblings at each level sorted alphabetically
    fn order_elements_hierarchically(&self, elements: &mut Vec<&Element>) {
        if elements.len() <= 1 {
            return;
        }

        // Build a map of element fragment (slug) -> index for quick lookup
        // The fragment is the part after # in the identifier (e.g., "parent-a" from "file.md#parent-a")
        let fragment_to_idx: HashMap<String, usize> = elements
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let fragment = e.identifier.split('#').last().unwrap_or(&e.identifier).to_string();
                (fragment, i)
            })
            .collect();

        // Build parent -> children map based on file-local derivedFrom relations
        // Using indices to avoid lifetime issues
        let mut children_map: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut has_parent: HashSet<usize> = HashSet::new();

        for (idx, element) in elements.iter().enumerate() {
            // Find file-local derivedFrom relations
            for relation in &element.relations {
                if relation.relation_type.name == "derivedFrom" {
                    // Check if target is in the same file
                    if let Some(target_id) = &relation.target.element_id {
                        // target_id is the fragment (slug) like "parent-a"
                        // Check if this target exists in the same file
                        if let Some(&parent_idx) = fragment_to_idx.get(target_id) {
                            // This element has a file-local parent
                            children_map
                                .entry(parent_idx)
                                .or_insert_with(Vec::new)
                                .push(idx);
                            has_parent.insert(idx);
                        }
                    }
                }
            }
        }

        // Identify root element indices (those without file-local parents)
        let mut roots: Vec<usize> = (0..elements.len())
            .filter(|idx| !has_parent.contains(idx))
            .collect();

        // Sort roots alphabetically by element name
        roots.sort_by(|&a, &b| elements[a].name.cmp(&elements[b].name));

        // Sort children at each level alphabetically by element name
        for children in children_map.values_mut() {
            children.sort_by(|&a, &b| elements[a].name.cmp(&elements[b].name));
        }

        // Build ordered list using depth-first traversal with stack (iterative)
        let mut ordered_indices: Vec<usize> = Vec::with_capacity(elements.len());
        let mut visited: HashSet<usize> = HashSet::new();

        // Process roots in reverse order so they come out in correct order
        let mut stack: Vec<usize> = Vec::new();
        for &root in roots.iter().rev() {
            stack.push(root);
        }

        while let Some(idx) = stack.pop() {
            if visited.contains(&idx) {
                continue;
            }
            visited.insert(idx);
            ordered_indices.push(idx);

            // Push children in reverse alphabetical order so they come out in correct order
            if let Some(children) = children_map.get(&idx) {
                for &child_idx in children.iter().rev() {
                    if !visited.contains(&child_idx) {
                        stack.push(child_idx);
                    }
                }
            }
        }

        // Reorder elements based on ordered indices
        let original: Vec<&Element> = elements.drain(..).collect();
        for idx in ordered_indices {
            elements.push(original[idx]);
        }
    }

    /// Generates markdown content for a file
    /// When with_full_relations is true, includes all relations (user-created and auto-generated)
    pub fn generate_file_markdown(&self, file_path: &str, elements: &[&Element], with_full_relations: bool) -> String {
        let mut markdown = String::new();

        // All specification files must have "# Elements" as the page header
        markdown.push_str("# Elements\n\n");

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
            markdown.push_str(&self.element_to_markdown_with_context(element, file_path, with_full_relations));
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

        // Find all files with attachments pointing to this element
        for file in self.find_files_with_attachment_to(element_id) {
            if !modified_files.contains(&file) {
                modified_files.push(file);
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

        // Update all attachment identifiers pointing to this element
        self.update_attachment_identifiers(&old_id, &new_identifier);

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

        // Find all files with attachments to elements in the source file
        for old_id in &elements_in_source {
            for file in self.find_files_with_attachment_to(old_id) {
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

        // Update all attachment identifiers pointing to moved elements
        for (old_id, new_id) in &identifier_mappings {
            self.update_attachment_identifiers(old_id, new_id);
        }

        modified_files.push(target_file.to_string());

        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok(identifier_mappings)
    }

    /// Flushes all elements to markdown files and copies InternalPath files to the specified directory
    /// When with_full_relations is true, includes all relations (user-created and auto-generated inverse relations)
    pub fn flush_to_directory(&self, output_dir: &Path, with_full_relations: bool) -> Result<(usize, usize), ReqvireError> {
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
            let markdown_content = self.generate_file_markdown(&file_path, &elements, with_full_relations);

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
    /// When with_full_relations is true, includes all relations (user-created and auto-generated inverse relations)
    pub fn flush_files_to_directory(&self, file_paths: &[String], output_dir: &Path, with_full_relations: bool) -> Result<(usize, usize), ReqvireError> {
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
                let markdown_content = self.generate_file_markdown(file_path, elements, with_full_relations);

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

    /// Updates attachment identifiers when a Refinement element is moved or renamed
    /// Similar to update_relation_identifiers but for attachment references
    fn update_attachment_identifiers(&mut self, old_identifier: &str, new_identifier: &str) {
        // Find and update all attachment identifiers pointing to the old identifier
        for node in self.nodes.values_mut() {
            for attachment in &mut node.element.attachments {
                if let crate::element::AttachmentTarget::ElementIdentifier(ref mut id) = attachment.target {
                    if id == old_identifier {
                        *id = new_identifier.to_string();
                    }
                }
            }
        }
    }

    /// Finds all files that have attachments pointing to the given element identifier
    fn find_files_with_attachment_to(&self, element_id: &str) -> Vec<String> {
        let mut files = Vec::new();
        for node in self.nodes.values() {
            let has_attachment = node.element.attachments.iter().any(|att| {
                matches!(&att.target, crate::element::AttachmentTarget::ElementIdentifier(id) if id == element_id)
            });
            if has_attachment {
                let file = node.element.file_path.clone();
                if !files.contains(&file) {
                    files.push(file);
                }
            }
        }
        files
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

    /// Remove an attachment from an element
    pub fn remove_element_attachment(&mut self, element_id: &str, attachment: &str) -> Result<(), ReqvireError> {
        if let Some(node) = self.nodes.get_mut(element_id) {
            let original_len = node.element.attachments.len();
            node.element.attachments.retain(|a| {
                a.target.as_str() != attachment
            });

            if node.element.attachments.len() < original_len {
                self.modified_files.insert(node.element.file_path.clone());
                Ok(())
            } else {
                Err(ReqvireError::ProcessError(format!(
                    "Attachment '{}' not found on element '{}'", attachment, element_id
                )))
            }
        } else {
            Err(ReqvireError::ProcessError(format!(
                "Element '{}' not found", element_id
            )))
        }
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

    /// Adds a relation to an element with full validation and target resolution
    /// This is the comprehensive method used by CRUD operations
    ///
    /// # Arguments
    /// * `source_id` - Source element identifier
    /// * `target` - Target (element name, URL, or file path)
    /// * `relation_type` - Relation type name
    /// * `git_root` - Git root path for file resolution
    ///
    /// # Returns
    /// Modified file path
    pub fn add_element_relation_full(
        &mut self,
        source_id: &str,
        target: &str,
        relation_type: &str,
        git_root: &std::path::Path,
    ) -> Result<String, ReqvireError> {
        use crate::relation::{RELATION_TYPES, Relation, RelationTarget, LinkType};
        use std::path::PathBuf;

        // Validate source element exists
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::ElementNotFound(
                format!("Source element '{}' not found", source_id)
            ));
        }

        // Validate relation type
        if !RELATION_TYPES.contains_key(relation_type) {
            return Err(ReqvireError::UnsupportedRelationType(
                format!("Invalid relation type '{}'. Valid types: {}",
                    relation_type, crate::relation::supported_relation_types_list())
            ));
        }

        // Get source element info
        let source_node = self.nodes.get(source_id).unwrap();
        let source_name = source_node.element.name.clone();
        let source_file_path = source_node.element.file_path.clone();
        let source_type = source_node.element.element_type.clone();

        // Determine target type: element name, external URL, or internal path
        let is_external_url = crate::utils::is_external_url(target);
        let is_internal_path = !is_external_url && (
            target.ends_with(".md") ||
            target.contains('/') ||
            git_root.join(target).exists()
        );

        // Resolve target and create relation components
        let (target_display_name, relation_target_link, target_id_for_check, element_id_opt) =
            if is_external_url {
                // External URL - use as-is
                (target.to_string(), LinkType::ExternalUrl(target.to_string()), target.to_string(), None)
            } else if is_internal_path {
                // Internal file path
                let source_folder = crate::utils::get_parent_dir(&source_file_path);

                // Calculate relative path from source file to target
                let target_path = PathBuf::from(target);
                let relative_path = pathdiff::diff_paths(&target_path, &source_folder)
                    .unwrap_or_else(|| target_path.clone());

                // Extract filename for display name
                let display = target_path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| target.to_string());

                (display, LinkType::InternalPath(relative_path), target.to_string(), None)
            } else {
                // Element name - resolve to get identifier
                let target_element = self.get_element_by_name(target)
                    .ok_or_else(|| ReqvireError::ElementNotFound(
                        format!("Target element '{}' not found", target)
                    ))?;

                let target_id = target_element.identifier.clone();
                let target_display_name = target_element.name.clone();
                let target_file_path = target_element.file_path.clone();
                let target_type = target_element.element_type.clone();

                // Validate type compatibility for element-to-element relations
                // TODO: Add relation-type-specific validation here
                // For now, just check that we're not linking incompatible types
                if !source_type.is_merge_compatible(&target_type) {
                    // Note: This is a simplified check - we may want more nuanced validation
                    // based on the specific relation type
                    log::warn!(
                        "Adding relation '{}' between potentially incompatible types: {} ({}) -> {} ({})",
                        relation_type, source_name, source_type.as_str(),
                        target_display_name, target_type.as_str()
                    );
                }

                // Calculate relative identifier from source element's file to target element
                let source_folder = crate::utils::get_parent_dir(&source_file_path);

                let relation_target = if source_file_path == target_file_path {
                    // Same file - use just the fragment
                    let (_path, fragment_opt) = crate::utils::extract_path_and_fragment(&target_id);
                    let fragment = fragment_opt.unwrap_or(&target_id);
                    LinkType::Identifier(format!("#{}", fragment))
                } else {
                    // Different files - calculate relative path
                    let relative_id = crate::utils::to_relative_identifier(
                        &target_id,
                        &source_folder,
                        true
                    ).unwrap_or_else(|_| target_id.clone());
                    LinkType::Identifier(relative_id)
                };

                // Extract element ID (fragment) for change tracking
                let (_path, fragment_opt) = crate::utils::extract_path_and_fragment(&target_id);
                let element_id = fragment_opt.map(|s| s.to_string());

                (target_display_name, relation_target, target_id, element_id)
            };

        // Get source node again (mutable this time)
        let source_node = self.nodes.get(source_id).unwrap();

        // Validate: Check if relation already exists (idempotent)
        let relation_exists = source_node.element.relations.iter().any(|r| {
            r.user_created &&
            r.relation_type.name == relation_type &&
            r.target.link.as_str() == target_id_for_check
        });

        if relation_exists {
            return Err(ReqvireError::RelationError(
                format!("Relation '{}' from '{}' to '{}' already exists",
                    relation_type, source_name, target)
            ));
        }

        // Validate: Check for cross-section duplicate (target in Attachments)
        let in_attachments = source_node.element.attachments.iter().any(|a| {
            a.target.as_str() == target_id_for_check
        });

        if in_attachments {
            return Err(ReqvireError::CrossSectionDuplicate(
                format!("Target '{}' already exists in Attachments of '{}'. Cannot add to Relations.",
                    target, source_name)
            ));
        }

        // Create the relation
        let relation_type_info = RELATION_TYPES.get(relation_type).unwrap();
        let relation = Relation {
            relation_type: relation_type_info,
            target: RelationTarget {
                text: target_display_name,
                link: relation_target_link,
                element_id: element_id_opt,
            },
            user_created: true,
        };

        // Get source element info for opposite relation before mutation
        let source_node = self.nodes.get(source_id).unwrap();
        let source_name = source_node.element.name.clone();
        let source_element_id = source_node.element.id.clone();
        let file_path = source_node.element.file_path.clone();

        // Add relation to source element
        let source_node = self.nodes.get_mut(source_id).unwrap();
        source_node.element.relations.push(relation.clone());

        // Mark file as modified
        self.modified_files.insert(file_path.clone());

        // CRITICAL: Maintain bidirectional consistency for in-memory model
        // Use helper to add opposite relation to target element (if applicable)
        self.add_opposite_to_target(
            &relation,
            source_id,
            &source_name,
            &source_element_id,
        );

        Ok(file_path)
    }

    /// Removes a relation from an element with full target resolution
    /// This is the comprehensive method used by CRUD operations
    ///
    /// # Arguments
    /// * `source_id` - Source element identifier
    /// * `target` - Target (element name, URL, or file path)
    ///
    /// # Returns
    /// Tuple of (modified file path, relation type, target display name) or None if no relation found
    pub fn remove_element_relation_full(
        &mut self,
        source_id: &str,
        target: &str,
    ) -> Result<Option<(String, String, String)>, ReqvireError> {
        // Validate source element exists
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::ElementNotFound(
                format!("Source element '{}' not found", source_id)
            ));
        }

        let source_node = self.nodes.get(source_id).unwrap();
        let source_file_path = source_node.element.file_path.clone();

        // Try to resolve target as element name first
        let target_id_to_find = if let Some(target_element) = self.get_element_by_name(target) {
            target_element.identifier.clone()
        } else {
            // Not an element name - could be URL or path
            target.to_string()
        };

        // Find matching relation (check both user_created and auto-generated)
        // This allows unlinking from either side of a bidirectional relation
        let relation_match = source_node.element.relations.iter()
            .find(|r| {
                r.target.link.as_str() == target_id_to_find
            })
            .cloned(); // Clone to avoid borrow issues

        if let Some(relation) = relation_match {
            let relation_type = relation.relation_type.name.to_string();
            let target_display_name = relation.target.text.clone();
            let relation_type_info = crate::relation::RELATION_TYPES.get(relation_type.as_str()).unwrap();
            let source_relation_was_user_created = relation.user_created;

            // Remove the relation (both user_created and auto-generated)
            let source_node = self.nodes.get_mut(source_id).unwrap();
            source_node.element.relations.retain(|r| {
                !(r.relation_type.name == relation_type &&
                  r.target.link.as_str() == target_id_to_find)
            });

            // Mark source file as modified only if relation was user_created (written to file)
            if source_relation_was_user_created {
                self.modified_files.insert(source_file_path.clone());
            }

            // CRITICAL: Maintain bidirectional consistency for in-memory model
            // Use helper to remove opposite relation from target element (if applicable)
            if let Some(opposite_type_name) = relation_type_info.opposite {
                self.remove_opposite_from_target(&target_id_to_find, source_id, opposite_type_name);
            }

            Ok(Some((source_file_path, relation_type, target_display_name)))
        } else {
            // No relation found - could be an attachment (handled by crud layer)
            Ok(None)
        }
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

        // Set file_order_index: append to end of file
        let mut new_element = element.clone();
        let max_index = self.nodes.values()
            .filter(|node| node.element.file_path == target_file)
            .map(|node| node.element.file_order_index)
            .max()
            .unwrap_or(0);
        new_element.file_order_index = max_index + 1;

        // Add to graph
        self.add_element(new_element.clone())?;

        // Populate element_id for all relations (including the newly added element)
        // This is necessary for hierarchical ordering to recognize parent-child relationships
        self.populate_relation_element_ids();

        // CRITICAL: Maintain bidirectional consistency for in-memory model
        // Use helper to create opposite relations for all relations in the newly added element
        let new_element_id = new_element.identifier.clone();
        let new_element_name = new_element.name.clone();
        let new_element_fragment_id = new_element.id.clone();
        let relations_to_process: Vec<_> = self.nodes.get(&new_element_id)
            .unwrap()
            .element
            .relations
            .clone();

        for relation in relations_to_process {
            self.add_opposite_to_target(
                &relation,
                &new_element_id,
                &new_element_name,
                &new_element_fragment_id,
            );
        }

        // Track modified file
        self.modified_files.insert(target_file.to_string());

        Ok(new_element)
    }

    /// Check if removing an element would orphan any children
    ///
    /// Returns a sorted list of child element names that would be orphaned
    fn check_for_orphaned_children(&self, element_id: &str) -> Result<Vec<String>, ReqvireError> {
        let mut orphaned_children: Vec<String> = Vec::new();
        let hierarchical_types = crate::relation::get_hierarchical_relation_types();

        for (_child_id, child_node) in &self.nodes {
            // Count how many hierarchical parent relations this child has to the element being deleted
            let mut parents_to_target = 0;
            let mut total_parents = 0;

            for rel in &child_node.element.relations {
                let target_id = match &rel.target.link {
                    crate::relation::LinkType::Identifier(id) => id.as_str(),
                    _ => continue, // Skip external links
                };
                if hierarchical_types.contains(&rel.relation_type.name) {
                    total_parents += 1;
                    if target_id == element_id {
                        parents_to_target += 1;
                    }
                }
            }

            // If child only has hierarchical parent relations to the target element, it will be orphaned
            if parents_to_target > 0 && total_parents == parents_to_target {
                orphaned_children.push(child_node.element.name.clone());
            }
        }

        orphaned_children.sort();
        Ok(orphaned_children)
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
        let element_name = element.name.clone();
        let file_path = element.file_path.clone();

        // Validate: Check for orphaned children before removal
        let orphaned_children = self.check_for_orphaned_children(element_id)?;
        if !orphaned_children.is_empty() {
            return Err(ReqvireError::InvalidOperation(
                format!(
                    "Cannot delete '{}' because it has {} child element(s) with parent hierarchical relations that would become orphaned: {}.\n\n\
                    To proceed, either:\n\
                    1. Delete the child elements first, or\n\
                    2. Update the child elements to link to a different parent element",
                    element_name,
                    orphaned_children.len(),
                    orphaned_children.join(", ")
                )
            ));
        }

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

        // Find all files with attachments pointing to this element
        for file in self.find_files_with_attachment_to(element_id) {
            if !modified_files.contains(&file) {
                modified_files.push(file);
            }
        }

        // Now perform the move
        self.move_element_to_location(element_id, target_file)?;

        // Update all relations (TO and FROM the moved element)
        self.update_relation_identifiers(&old_identifier, &source_file, target_file);

        // Construct the new identifier (file path changed, fragment stays the same)
        let fragment = old_identifier.split('#').last().unwrap_or("");
        let new_identifier = format!("{}#{}", target_file, fragment);

        // Update the element's identifier field in the node
        if let Some(node) = self.nodes.get_mut(&old_identifier) {
            node.element.identifier = new_identifier.clone();
        }

        // Update all attachment identifiers pointing to this element
        self.update_attachment_identifiers(&old_identifier, &new_identifier);

        // CRITICAL: Recreate opposite relations with updated identifiers
        // After moving, opposite relations pointing to the old identifier must be updated
        self.recreate_opposites_after_move(&old_identifier, &new_identifier);

        // Track all modified files
        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok((new_identifier, modified_files))
    }

    /// Merge multiple source elements into a target element
    ///
    /// # Arguments
    /// * `target_id` - Identifier of the target element (must exist)
    /// * `source_ids` - Identifiers of source elements to merge into target (must exist)
    ///
    /// # Behavior
    /// - Source content is appended to target's Details section
    /// - Source Details sections become "Merged Details (source name)" subsections
    /// - Relations and attachments are merged with deduplication
    /// - Source elements are deleted after successful merge
    /// - Relations pointing to source elements are redirected to target
    pub fn merge_elements(
        &mut self,
        target_id: &str,
        source_ids: &[String],
    ) -> Result<(), ReqvireError> {
        // Validate target exists
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::ElementNotFound(
                format!("Target element '{}' not found", target_id)
            ));
        }

        // Get target element data first (needed for validation)
        let target_node = self.nodes.get(target_id).unwrap();
        let target_name = target_node.element.name.clone();
        let target_type = target_node.element.element_type.clone();

        // Validate all sources exist and collect their data
        let mut source_data: Vec<(String, String, String, Vec<crate::relation::Relation>, Vec<crate::element::Attachment>)> = Vec::new();
        for source_id in source_ids {
            let source_node = self.nodes.get(source_id)
                .ok_or_else(|| ReqvireError::ElementNotFound(
                    format!("Source element '{}' not found", source_id)
                ))?;

            let source_element = &source_node.element;

            // Validate: Check if source would merge into itself
            if source_id == target_id {
                return Err(ReqvireError::InvalidOperation(
                    "Cannot merge element into itself".to_string()
                ));
            }

            // Validate: Check type compatibility
            if !target_type.is_merge_compatible(&source_element.element_type) {
                return Err(ReqvireError::MergeTypeMismatch(format!(
                    "Cannot merge '{}' ({}) into '{}' ({}): type mismatch. \
                     Elements must be in the same category (requirement/verification/refinement/other).",
                    source_element.name, source_element.element_type.as_str(),
                    target_name, target_type.as_str()
                )));
            }

            source_data.push((
                source_id.clone(),
                source_element.name.clone(),
                source_element.content.clone(),
                source_element.relations.iter()
                    .filter(|r| r.user_created)
                    .cloned()
                    .collect(),
                source_element.attachments.clone(),
            ));
        }

        // Re-get target element data (needed after validation)
        let target_node = self.nodes.get(target_id).unwrap();
        let target_file_path = target_node.element.file_path.clone();
        let mut merged_content = String::new();
        let mut merged_relations: Vec<crate::relation::Relation> = target_node.element.relations.iter()
            .filter(|r| r.user_created)
            .cloned()
            .collect();
        let mut merged_attachments: Vec<crate::element::Attachment> = target_node.element.attachments.clone();

        // Process each source element
        for (source_id, source_name, source_content, source_relations, source_attachments) in &source_data {
            // Extract main content and details from source
            let (main_content, details_content) = extract_content_parts(source_content);

            // Add main content to merged content (will go into target's Details)
            if !main_content.trim().is_empty() {
                merged_content.push_str(&format!("\n{}\n", main_content.trim()));
            }

            // Add details to "Merged Details (element name)" subsection
            if !details_content.trim().is_empty() {
                merged_content.push_str(&format!(
                    "\n#### Merged Details ({})\n{}\n",
                    source_name, details_content.trim()
                ));
            }

            // Collect relations
            for rel in source_relations {
                merged_relations.push(rel.clone());
            }

            // Collect attachments
            for att in source_attachments {
                merged_attachments.push(att.clone());
            }

            // Track source file as modified
            let source_file = self.nodes.get(source_id).unwrap().element.file_path.clone();
            self.modified_files.insert(source_file);
        }

        // Deduplicate relations by (relation_type, target)
        let mut seen_relations: HashSet<(String, String)> = HashSet::new();
        merged_relations.retain(|r| {
            let key = (r.relation_type.name.to_string(), r.target.link.as_str().to_string());
            if seen_relations.contains(&key) {
                false
            } else {
                seen_relations.insert(key);
                true
            }
        });

        // Deduplicate attachments by target
        let mut seen_attachments: HashSet<String> = HashSet::new();
        merged_attachments.retain(|a| {
            let key = a.target.as_str().to_string();
            if seen_attachments.contains(&key) {
                false
            } else {
                seen_attachments.insert(key);
                true
            }
        });

        // Validate attachment scope constraints for target element
        for attachment in &merged_attachments {
            if let crate::element::AttachmentTarget::ElementIdentifier(ref att_id) = attachment.target {
                // Check orphan refinement constraint
                if !self.refinement_has_satisfy_relations(att_id) {
                    let att_name = self.nodes.get(att_id)
                        .map(|n| n.element.name.as_str())
                        .unwrap_or(att_id);
                    return Err(ReqvireError::InvalidAttachmentTarget(
                        format!(
                            "'{}' has no satisfy relations. Refinements must satisfy a requirement before they can be attached.",
                            att_name
                        ),
                    ));
                }

                // Check hierarchical independence constraint
                let defining_reqs = self.get_defining_requirements(att_id);
                for defining_req_id in defining_reqs {
                    if self.is_in_hierarchy(target_id, &defining_req_id) {
                        let att_name = self.nodes.get(att_id)
                            .map(|n| n.element.name.as_str())
                            .unwrap_or(att_id);
                        return Err(ReqvireError::InvalidAttachmentScope(
                            format!(
                                "'{}' cannot be attached to '{}' because it is within the refinement's defining hierarchy. Attachments are only allowed from requirements outside the satisfiedBy chain.",
                                att_name,
                                target_name
                            ),
                        ));
                    }
                }
            }
        }

        // Check for cross-section duplicates
        let relation_targets: HashSet<String> = merged_relations.iter()
            .map(|r| r.target.link.as_str().to_string())
            .collect();

        for attachment in &merged_attachments {
            let target = attachment.target.as_str();
            if relation_targets.contains(&target) {
                return Err(ReqvireError::MergeCrossSectionDuplicate(format!(
                    "Target '{}' would appear in both Relations and Attachments after merge. Remove one before merging.",
                    target
                )));
            }
        }

        // Update target element with merged data
        {
            let target_node = self.nodes.get_mut(target_id).unwrap();
            let target_element = &mut target_node.element;

            // Merge content into target's Details section
            if !merged_content.trim().is_empty() {
                target_element.content = merge_content_into_details(&target_element.content, &merged_content);
            }

            target_element.relations = merged_relations;
            target_element.attachments = merged_attachments;
        }

        self.modified_files.insert(target_file_path);

        // CRITICAL: Before removing sources, handle opposite relations
        // Find all elements with relations TO sources and recreate their opposites to point to target
        let target_node = self.nodes.get(target_id).unwrap();
        let target_name = target_node.element.name.clone();
        let target_element_id = target_node.element.id.clone();

        for (source_id, _, _, _, _) in &source_data {
            // Find all elements that have user_created relations pointing TO this source
            let elements_with_relations_to_source: Vec<(String, Vec<Relation>)> = self.nodes.iter()
                .filter(|(id, _)| *id != source_id && *id != target_id)
                .filter_map(|(referrer_id, node)| {
                    let rels: Vec<_> = node.element.relations.iter()
                        .filter(|r| r.user_created)
                        .filter(|r| matches!(&r.target.link, LinkType::Identifier(id) if id == source_id))
                        .cloned()
                        .collect();
                    if rels.is_empty() {
                        None
                    } else {
                        Some((referrer_id.clone(), rels))
                    }
                })
                .collect();

            // For each element with relations to source, update opposite relations
            for (referrer_id, relations_to_source) in elements_with_relations_to_source {
                for relation in relations_to_source {
                    if let Some(opposite_type_name) = relation.relation_type.opposite {
                        // Remove old opposite pointing to source
                        self.remove_opposite_from_target(source_id, &referrer_id, opposite_type_name);

                        // Create new opposite pointing to target (will be added to target after merge)
                        // Note: The referrer's relation will be redirected by redirect_relations_to_target(),
                        // so we create opposite on target now
                        self.add_opposite_to_target(&relation, target_id, &target_name, &target_element_id);
                    }
                }
            }
        }

        // Redirect relations from other elements pointing to sources to point to target
        for (source_id, _, _, _, _) in &source_data {
            self.redirect_relations_to_target(source_id, target_id)?;
        }

        // Remove source elements (this also removes them from the graph)
        for (source_id, _, _, _, _) in &source_data {
            self.remove_element(source_id)?;
        }

        Ok(())
    }

    /// Redirect all relations pointing to source_id to point to target_id
    fn redirect_relations_to_target(
        &mut self,
        source_id: &str,
        target_id: &str,
    ) -> Result<(), ReqvireError> {
        // Find all nodes with relations pointing to source_id
        let nodes_to_update: Vec<String> = self.nodes.iter()
            .filter(|(id, node)| {
                *id != source_id && *id != target_id &&
                node.element.relations.iter().any(|r| {
                    match &r.target.link {
                        LinkType::Identifier(ref id) => {
                            id == source_id || id.ends_with(&format!("#{}", source_id))
                        },
                        _ => false,
                    }
                })
            })
            .map(|(id, _)| id.clone())
            .collect();

        for node_id in nodes_to_update {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                let file_path = node.element.file_path.clone();
                for relation in &mut node.element.relations {
                    if let LinkType::Identifier(ref id) = relation.target.link {
                        if id == source_id || id.ends_with(&format!("#{}", source_id)) {
                            // Update to point to target
                            relation.target = crate::relation::RelationTarget {
                                text: target_id.to_string(),
                                link: LinkType::Identifier(target_id.to_string()),
                                element_id: Some(target_id.to_string()),
                            };
                        }
                    }
                }
                self.modified_files.insert(file_path);
            }
        }

        // CRITICAL: Also redirect opposite relations (auto-generated, user_created=false)
        // Get target element info for creating correct opposite targets
        let (target_name, target_element_id) = if let Some(target_node) = self.nodes.get(target_id) {
            (target_node.element.name.clone(), target_node.element.id.clone())
        } else {
            return Ok(()); // Target doesn't exist
        };

        // Find and update opposite relations pointing to source
        for (node_id, node) in self.nodes.iter_mut() {
            if *node_id == source_id || *node_id == target_id {
                continue;
            }

            // Update auto-generated opposite relations pointing to source
            for relation in &mut node.element.relations {
                if !relation.user_created { // Only auto-generated opposites
                    if let LinkType::Identifier(ref id) = relation.target.link {
                        if id == source_id || id.ends_with(&format!("#{}", source_id)) {
                            // Update opposite to point to target
                            relation.target = crate::relation::RelationTarget {
                                text: target_name.clone(),
                                link: LinkType::Identifier(target_id.to_string()),
                                element_id: Some(target_element_id.clone()),
                            };
                            // Note: Do NOT mark file as modified - opposite is user_created=false
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Flushes only modified files to directory (optimization)
    pub fn flush_modified_files(&self, directory: &Path) -> Result<(), ReqvireError> {
        if self.modified_files.is_empty() {
            return Ok(());
        }

        let file_vec: Vec<String> = self.modified_files.iter().cloned().collect();
        let _result = self.flush_files_to_directory(&file_vec, directory, false)?;

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

/// Extract main content and details section from element content
///
/// Returns (main_content, details_content) where:
/// - main_content: Everything before the first "#### Details" header
/// - details_content: Everything after "#### Details" header until the next #### section
fn extract_content_parts(content: &str) -> (String, String) {
    let details_marker = "#### Details";
    if let Some(pos) = content.find(details_marker) {
        let main = content[..pos].to_string();
        let after_marker = pos + details_marker.len();
        let rest = &content[after_marker..];

        // Find end of details (next #### or end)
        let details_end = rest.find("\n#### ")
            .map(|p| p)
            .unwrap_or(rest.len());

        (main, rest[..details_end].to_string())
    } else {
        (content.to_string(), String::new())
    }
}

/// Merge additional content into the Details section of target content
fn merge_content_into_details(target_content: &str, additional: &str) -> String {
    if additional.trim().is_empty() {
        return target_content.to_string();
    }

    let details_marker = "#### Details";
    if let Some(pos) = target_content.find(details_marker) {
        // Find end of existing details
        let after_marker = pos + details_marker.len();
        let rest = &target_content[after_marker..];
        let details_end = rest.find("\n#### ")
            .map(|p| after_marker + p)
            .unwrap_or(target_content.len());

        // Insert additional content at end of Details
        let mut result = target_content[..details_end].to_string();
        result.push_str(additional);
        result.push_str(&target_content[details_end..]);
        result
    } else {
        // No Details section - create one
        format!("{}\n#### Details\n{}", target_content.trim_end(), additional)
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
        let b_markdown = graph.element_to_markdown_with_context(&b_element, "file1.md",true);
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
        let a_markdown_initial = graph.element_to_markdown_with_context(&a_element_initial, "file1.md",true);
        println!("A's initial markdown (in file1.md):");
        println!("{}", a_markdown_initial);

        // A is in file1.md, B is in file2.md, C is in file1.md
        // So A should reference B as "file2.md#B" and C as just "C" (same file)

        // Move A to file3.md
        let result = graph.move_element_to_new_file("A", "file3.md");
        assert!(result.is_ok());

        // Check A's relations after the move
        let a_element_moved = graph.nodes.get("A").unwrap().element.clone();
        let a_markdown_moved = graph.element_to_markdown_with_context(&a_element_moved, "file3.md",true);
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
        let result = graph.flush_to_directory(output_path,true);
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

        // Verify proper markdown structure - all files start with "# Elements"
        assert!(file1_content.starts_with("# Elements\n"));
        assert!(file3_content.starts_with("# Elements\n"));
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
        let result = graph.flush_to_directory(output_path,true);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("test_file.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // Verify file header is present - all files start with "# Elements"
        assert!(file_content.starts_with("# Elements\n\n"));

        // Verify page content is included after header and before elements
        assert!(file_content.contains("This is page frontmatter content."));
        assert!(file_content.contains("More page content here."));

        // Verify element is still present
        assert!(file_content.contains("### Element A Description"));

        // Verify order: header, page content, element
        let header_pos = file_content.find("# Elements").unwrap();
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
        let result = graph.flush_to_directory(output_path,true);
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
        let result = graph.flush_to_directory(output_path,true);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("test_file.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // Verify element is still present
        assert!(file_content.contains("### Element A Description"));
    }

    #[test]
    fn test_flush_always_outputs_elements_header() {
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
        let result = graph.flush_to_directory(output_path,true);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("MOEs.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // All specification files should start with "# Elements"
        assert!(file_content.starts_with("# Elements\n\n"));

        // Page content should be included after the header
        assert!(file_content.contains("This is the MOEs page content."));
    }
}
