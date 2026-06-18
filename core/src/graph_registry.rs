use log::{debug, warn};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::crud::replace_prefix_token;
use crate::element::{
    extract_single_fenced_subsection, Element, ElementType, GovernanceMetadataEntry,
    GovernanceMetadataSource, RequirementGovernanceMetadata, SizeEstimate,
    REUSED_CONTRACT_CONTEXT_SECTION,
};
use crate::error::ReqvireError;
use crate::git_commands;
use crate::relation::{
    self, get_hierarchical_relation_types, LinkType, CONTRACT_RELATIONS,
    IMPACT_PROPAGATION_RELATIONS, SATISFACTION_RELATIONS,
};
use crate::semantic_contract;
use crate::Relation;
use globset::GlobSet;
use regex::Regex;

/// Cached regex for matching .md file references in relation targets
static MD_FILE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\.md(?:#|$)").unwrap());

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

#[derive(Debug, Clone)]
pub struct GraphRegistry {
    pub nodes: HashMap<String, ElementNode>,
    pub pages: HashMap<String, Page>,
    pub modified_files: HashSet<String>, // Track files modified during CRUD operations
}

impl Default for GraphRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphRegistry {
    fn is_single_element_format_file(&self, file_path: &str) -> bool {
        self.nodes.values().any(|node| {
            node.element.file_path == file_path
                && node
                    .element
                    .metadata
                    .get("_single_element_format")
                    .map(|v| v == "true")
                    .unwrap_or(false)
        })
    }

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
    pub fn register_element(
        &mut self,
        element: Element,
        _file_path: &str,
    ) -> Result<(), ReqvireError> {
        let element_id = element.identifier.clone();

        // Note: Duplicate checking is now done at global level in ModelManager::pass1_collect_elements
        // to properly report all duplicate locations

        self.nodes.insert(
            element_id,
            ElementNode {
                element,
                relations: Vec::new(),
            },
        );

        Ok(())
    }

    /// Build relations and validate graph structure
    pub fn build_relations(
        &mut self,
        excluded_filename_patterns: &GlobSet,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("GraphRegistry: Building relations and validating graph structure");

        // Normalize any non-canonical relation references before validation:
        // keep canonical full identifiers in-memory and keep markdown rendering compact
        // through serialization-time relative-link conversion.
        self.normalize_element_relation_targets();

        // First build the relation graph
        self.build_relation_graph();

        // Add missing opposites
        self.propagate_missing_opposites(excluded_filename_patterns);

        // Populate element_id for all relations
        self.populate_relation_element_ids();

        // Validate relations
        let mut errors = self.validate_relations(excluded_filename_patterns)?;

        // Validate authored element type metadata
        errors.extend(self.validate_element_type_metadata()?);

        // Validate non-test-verification satisfiedBy relations
        errors.extend(self.validate_non_test_verification_satisfied_by()?);

        // Validate concrete verifications are organized under verification objectives
        errors.extend(self.validate_verification_objective_parents()?);

        // Validate cross-component dependencies
        errors.extend(self.validate_cross_component_dependencies()?);

        // Validate reused_contract_context exist
        errors.extend(self.validate_reused_contract_context()?);

        // Validate legacy contract relation names before the stricter
        // contract-element ownership checks.
        errors.extend(self.validate_legacy_contract_relations()?);

        // Validate contract elements have only define relations
        errors.extend(self.validate_contract_elements()?);

        // Validate ontology elements and ontology graph shape
        errors.extend(self.validate_ontology_elements()?);

        // Validate reserved requirement governance metadata
        errors.extend(self.validate_governance_metadata()?);

        // Validate contract ownership uniqueness (each contract owned by at most one requirement)
        errors.extend(self.validate_contract_ownership_uniqueness()?);

        // Validate explicit 'other' type elements do not author semantic relations
        errors.extend(self.validate_other_element_relations()?);

        // Validate no cross-section duplicates (same target in Relations and Reused Contract Context)
        errors.extend(self.validate_cross_section_duplicates()?);

        // Validate semantic-contract reserved sections, declarations, and references
        errors.extend(self.validate_semantic_contracts(None)?);

        Ok(errors)
    }

    /// Populate optional element-level size estimates for JSON evidence consumers.
    pub fn populate_size_estimates(&mut self) -> Result<(), ReqvireError> {
        let mut element_ids: Vec<String> = self.nodes.keys().cloned().collect();
        element_ids.sort();

        for element_id in element_ids {
            if let Some(node) = self.nodes.get_mut(&element_id) {
                node.element.size_estimate = None;
                let rendered_context_bytes = serde_json::to_vec(&node.element)
                    .map_err(|e| ReqvireError::SerializationError(e.to_string()))?
                    .len();
                let content_bytes = node.element.content.len();
                node.element.size_estimate = Some(SizeEstimate {
                    content_bytes,
                    rendered_context_bytes,
                    estimated_tokens: rendered_context_bytes.div_ceil(4),
                });
            }
        }

        self.build_relation_graph();
        Ok(())
    }

    /// Validates that no element has the same target in both Relations and Reused Contract Context subsections
    fn validate_cross_section_duplicates(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        log::debug!("Running cross-section duplicate validation...");
        let mut errors = Vec::new();

        for node in self.nodes.values() {
            let element = &node.element;

            // Collect all relation targets (normalized identifiers)
            let relation_targets: std::collections::HashSet<String> = element
                .relations
                .iter()
                .filter(|r| r.user_created)
                .map(|r| r.target.link.as_str().to_string())
                .collect();

            // Check reused_contract_context against relations
            for reused_contract_context in &element.reused_contract_context {
                let reused_contract_context_target = reused_contract_context.target.as_str();
                if relation_targets.contains(&reused_contract_context_target) {
                    let msg = format!(
                        "Cross-section duplicate in element '{}': target '{}' appears in both Relations and Reused Contract Context (file: {})",
                        element.name, reused_contract_context_target, element.file_path
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
                        if relation::IMPACT_PROPAGATION_RELATIONS
                            .contains(&relation.relation_type.name)
                        {
                            let resolved_target = self
                                .resolve_relation_identifier(&source_node.element, target_id)
                                .unwrap_or_else(|| target_id.to_string());

                            if let Some(target_node) = self.nodes.get(&resolved_target) {
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
        for source_id in &element_ids {
            if let Some(source_node) = self.nodes.get(source_id) {
                for relation in &source_node.element.relations {
                    if let crate::relation::LinkType::Identifier(ref target_id) =
                        relation.target.link
                    {
                        let source_identifier = source_node.element.identifier.clone();
                        let resolved_target = self
                            .resolve_relation_identifier(&source_node.element, target_id)
                            .unwrap_or_else(|| target_id.to_string());

                        if !MD_FILE_RE.is_match(&resolved_target)
                            || excluded_filename_patterns.is_match(&resolved_target)
                        {
                            continue;
                        }

                        if let Some(opposite_name) = relation.relation_type.opposite {
                            if let Some(target_node) = self.nodes.get(&resolved_target) {
                                let already_present =
                                    target_node.element.relations.iter().any(|r| {
                                        matches!(
                                            &r.target.link,
                                            crate::relation::LinkType::Identifier(id)
                                                if self
                                                    .normalize_relation_identifier_for_source(
                                                        &target_node.element.file_path,
                                                        id
                                                    )
                                                    == Some(source_identifier.clone())
                                        ) && r
                                            .relation_type
                                            .name
                                            .eq_ignore_ascii_case(opposite_name)
                                    });

                                if !already_present {
                                    if let Some(opposite_relation) = relation.to_opposite(
                                        &source_node.element.name,
                                        &source_node.element.identifier,
                                        &source_node.element.id,
                                    ) {
                                        to_add.push((resolved_target.clone(), opposite_relation));
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
        if let Some(opposite_relation) =
            relation.to_opposite(source_name, source_id, source_element_id)
        {
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
                r.user_created
                    && r.relation_type.name == opposite_type_name
                    && r.target.link.as_str() == source_id
            });

            let target_file_path = target_node.element.file_path.clone();

            // Remove opposite relation (both user_created and auto-generated)
            let target_node = self.nodes.get_mut(target_id).unwrap();
            target_node.element.relations.retain(|r| {
                !(r.relation_type.name == opposite_type_name && r.target.link.as_str() == source_id)
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
    fn recreate_opposites_after_move(&mut self, old_id: &str, new_id: &str) {
        // Get moved element info
        let (moved_name, moved_element_id, relations_to_process) =
            if let Some(moved_node) = self.nodes.get(new_id) {
                let name = moved_node.element.name.clone();
                let element_id = moved_node.element.id.clone();
                // Only process user_created relations (these have opposites that need updating)
                let relations: Vec<_> = moved_node
                    .element
                    .relations
                    .iter()
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
        let id_map: std::collections::HashMap<String, String> = self
            .nodes
            .iter()
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
    fn validate_relations(
        &self,
        excluded_filename_patterns: &GlobSet,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
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
                                    errors.push(ReqvireError::MissingRelationTarget(format!(
                                        "Element '{}' references missing target '{}'",
                                        source_node.element.identifier, target_id
                                    )));
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
                            let file_target_type = ElementType::File;
                            if crate::relation::get_relation_element_type_description(
                                relation.relation_type.name,
                            )
                            .is_some()
                                && !crate::relation::validate_relation_element_types(
                                    relation.relation_type.name,
                                    &source_node.element.element_type,
                                    &file_target_type,
                                )
                            {
                                let expected =
                                    crate::relation::get_relation_element_type_description(
                                        relation.relation_type.name,
                                    )
                                    .unwrap_or_default();
                                errors.push(ReqvireError::IncompatibleElementTypes(format!(
                                    "Relation '{}' from '{}' ({:?}) to file target '{}' has incompatible element types. {}",
                                    relation.relation_type.name,
                                    source_node.element.identifier,
                                    source_node.element.element_type,
                                    file_path.to_string_lossy(),
                                    expected
                                )));
                            }

                            // Validate file existence for InternalPath targets
                            // InternalPath contains normalized paths from normalize_identifier which are git-root-relative
                            let git_root = match crate::git_commands::get_git_root_dir() {
                                Ok(root) => root,
                                Err(_) => {
                                    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
                                }
                            };
                            let absolute_path = git_root.join(file_path);
                            if !absolute_path.exists() {
                                errors.push(ReqvireError::MissingRelationTarget(format!(
                                    "Element '{}' references missing target '{}'",
                                    source_node.element.identifier,
                                    file_path.to_string_lossy()
                                )));
                                continue;
                            }

                            if relation.relation_type.name == "definedBy" {
                                errors.push(ReqvireError::IncompatibleElementTypes(format!(
                                    "definedBy target '{}' is invalid. 'definedBy' must point to a contract element identifier, not a file path.",
                                    file_path.to_string_lossy()
                                )));
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
        target_element: &Element,
    ) -> Option<ReqvireError> {
        // Only validate relation types with element type restrictions
        if let Some(expected_types) =
            crate::relation::get_relation_element_type_description(relation_type)
        {
            // Check if the element types are compatible
            let is_valid = crate::relation::validate_relation_element_types(
                relation_type,
                &source_element.element_type,
                &target_element.element_type,
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

    fn validate_element_type_metadata(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        let mut errors = Vec::new();

        for node in self.nodes.values() {
            let element = &node.element;
            let Some(type_value) = element.metadata.get("type") else {
                continue;
            };

            if !crate::element::is_valid_element_type(type_value) {
                errors.push(ReqvireError::InvalidMetadataFormat(format!(
                    "Invalid element type '{}'. Valid types: {}",
                    type_value,
                    crate::element::element_types_help()
                )));
            }
        }

        Ok(errors)
    }

    /// Validates that only test-verification elements can have satisfiedBy relations
    /// Returns a list of validation errors for non-test-verification elements with satisfiedBy
    fn validate_non_test_verification_satisfied_by(
        &self,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
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
                            crate::element::VerificationType::Analysis
                            | crate::element::VerificationType::Inspection
                            | crate::element::VerificationType::Demonstration => {
                                errors.push(ReqvireError::IncompatibleElementTypes(
                                    format!("Non-evidence-backed verification element with satisfiedBy relation: '{}' (type: {:?}) cannot have satisfiedBy relations. Only test-verification and formal-proof-verification elements may use satisfiedBy.",
                                        element.identifier,
                                        verification_type
                                    )
                                ));
                            }
                            crate::element::VerificationType::Default
                            | crate::element::VerificationType::Test
                            | crate::element::VerificationType::FormalProof => {
                                // These are valid evidence-backed verifications.
                            }
                        }
                    }
                    _ => {
                        // Requirement-type compatibility is validated by relation element-type checks.
                        // is validated by relation element-type checks.
                    }
                }
            }
        }

        Ok(errors)
    }

    fn validate_verification_objective_parents(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        log::debug!("Validating concrete verification objective parents...");
        let mut errors = Vec::new();

        let mut sorted_nodes: Vec<&ElementNode> = self.nodes.values().collect();
        sorted_nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

        for element_node in sorted_nodes {
            let element = &element_node.element;
            if !matches!(element.element_type, ElementType::Verification(_)) {
                continue;
            }

            let has_objective_parent = element.relations.iter().any(|relation| {
                relation.relation_type.name == "derivedFrom"
                    && matches!(
                        &relation.target.link,
                        LinkType::Identifier(target_id)
                            if self.nodes.get(target_id).is_some_and(|target_node| {
                                matches!(
                                    target_node.element.element_type,
                                    ElementType::VerificationObjective
                                )
                            })
                    )
            });

            if !has_objective_parent {
                errors.push(ReqvireError::MissingParentRelation(format!(
                    "File {}: Concrete verification element '{}' must have a derivedFrom relation to a verification-objective parent. Standalone concrete verifications require migration to an objective-backed verification plan.",
                    element.file_path,
                    element.name
                )));
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
        let mut sorted_nodes: Vec<&ElementNode> = self.nodes.values().collect();
        sorted_nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

        for element_node in &sorted_nodes {
            let mut path = Vec::new();
            self.check_circular_dependencies(
                &element_node.element,
                &mut visited,
                &mut path,
                &mut errors,
            );
        }

        // Check for missing requirement parent relations.
        for element_node in &sorted_nodes {
            let element = &element_node.element;
            let element_file = &element.file_path;

            if matches!(element.element_type, ElementType::Requirement(_))
                && !self.has_requirement_parent(element)
            {
                errors.push(ReqvireError::MissingParentRelation(
                    format!("File {}: Element '{}' has no requirement parent relation (needs derivedFrom to a requirement or specify to a capability)", element_file, element.name),
                ));
            }
        }

        // Enforce single capability ownership for capability/requirement graph elements.
        errors.extend(self.validate_single_root_hierarchy_ownership()?);

        if errors.is_empty() {
            debug!("No cross-component dependency validation errors found.");
        } else {
            debug!("{} cross-component validation errors found.", errors.len());
        }

        Ok(errors)
    }

    fn validate_single_root_hierarchy_ownership(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        let mut errors = Vec::new();
        let mut memo: HashMap<String, BTreeSet<String>> = HashMap::new();
        let mut visiting: HashSet<String> = HashSet::new();

        let mut sorted_ids: Vec<&String> = self.nodes.keys().collect();
        sorted_ids.sort();

        for element_id in sorted_ids {
            let Some(element_node) = self.nodes.get(element_id) else {
                continue;
            };
            let element = &element_node.element;
            let is_hierarchy_element = matches!(
                element.element_type,
                ElementType::Capability | ElementType::Requirement(_)
            );
            if !is_hierarchy_element {
                continue;
            }

            if matches!(element.element_type, ElementType::Requirement(_))
                && !self.has_requirement_parent(element)
            {
                continue;
            }

            let roots = self.resolve_owning_capabilities(element_id, &mut memo, &mut visiting);
            if roots.len() != 1 {
                if roots.is_empty() {
                    errors.push(ReqvireError::MixedHierarchicalRelations(format!(
                        "Element '{}' ({}) must resolve to exactly one owning capability via capability/requirement relations, but resolved to 0 capabilities",
                        element.name, element.identifier
                    )));
                } else {
                    let roots_count = roots.len();
                    let roots_list = roots.into_iter().collect::<Vec<_>>().join(", ");
                    errors.push(ReqvireError::MixedHierarchicalRelations(format!(
                        "Element '{}' ({}) must resolve to exactly one owning capability via capability/requirement relations, but resolved to {} capabilities: {}",
                        element.name,
                        element.identifier,
                        roots_count,
                        roots_list
                    )));
                }
            }
        }

        Ok(errors)
    }

    /// Validates single-root hierarchy ownership on current in-memory graph state.
    /// Used by mutating CRUD operations to reject invalid post-mutation ownership.
    pub fn validate_single_root_hierarchy_ownership_in_memory(
        &self,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        self.validate_single_root_hierarchy_ownership()
    }

    fn has_requirement_parent(&self, element: &Element) -> bool {
        element.relations.iter().any(|relation| {
            matches!(relation.relation_type.name, "derivedFrom" | "specify")
                && matches!(relation.target.link, LinkType::Identifier(_))
        })
    }

    fn resolve_owning_capabilities(
        &self,
        element_id: &str,
        memo: &mut HashMap<String, BTreeSet<String>>,
        visiting: &mut HashSet<String>,
    ) -> BTreeSet<String> {
        if let Some(cached) = memo.get(element_id) {
            return cached.clone();
        }

        // Hierarchical cycles without a user root resolve to zero roots.
        if visiting.contains(element_id) {
            return BTreeSet::new();
        }
        visiting.insert(element_id.to_string());

        let mut result = BTreeSet::new();
        let Some(element) = self.get_element(element_id) else {
            visiting.remove(element_id);
            return result;
        };

        let parent_ids = self.get_capability_ownership_parent_ids(element);
        if matches!(element.element_type, ElementType::Capability) && parent_ids.is_empty() {
            result.insert(element.identifier.clone());
        }

        for parent_id in parent_ids {
            let parent_roots = self.resolve_owning_capabilities(&parent_id, memo, visiting);
            for root in parent_roots {
                result.insert(root);
            }
        }

        visiting.remove(element_id);
        memo.insert(element_id.to_string(), result.clone());
        result
    }

    fn resolve_single_owning_capability(&self, element_id: &str) -> Option<String> {
        let mut memo: HashMap<String, BTreeSet<String>> = HashMap::new();
        let mut visiting: HashSet<String> = HashSet::new();
        let roots = self.resolve_owning_capabilities(element_id, &mut memo, &mut visiting);
        if roots.len() == 1 {
            roots.into_iter().next()
        } else {
            None
        }
    }

    fn display_name_for_element(&self, element_id: &str) -> String {
        self.nodes
            .get(element_id)
            .map(|n| n.element.name.clone())
            .unwrap_or_else(|| element_id.to_string())
    }

    fn has_reused_contract_context_flow_between_roots(
        &self,
        source_root_id: &str,
        target_root_id: &str,
    ) -> bool {
        let mut sorted_ids: Vec<&String> = self.nodes.keys().collect();
        sorted_ids.sort();

        for element_id in sorted_ids {
            let Some(reuser_root_id) = self.resolve_single_owning_capability(element_id) else {
                continue;
            };

            if reuser_root_id != source_root_id {
                continue;
            }

            let Some(node) = self.nodes.get(element_id) else {
                continue;
            };

            for reused_contract_context in &node.element.reused_contract_context {
                let crate::element::ReusedContractContextTarget::ElementIdentifier(contract_id) =
                    &reused_contract_context.target
                else {
                    continue;
                };

                if !self.contract_has_define_relation(contract_id) {
                    continue;
                }

                for defining_req_id in self.get_defining_requirements(contract_id) {
                    let Some(defining_root_id) =
                        self.resolve_single_owning_capability(&defining_req_id)
                    else {
                        continue;
                    };

                    if defining_root_id == target_root_id {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn build_reused_contract_context_direction_scope_error(
        &self,
        reused_contract_context_identifier: &str,
        element_id: &str,
        element_name: &str,
        file_path: Option<&str>,
    ) -> Option<String> {
        let source_root_id = self.resolve_single_owning_capability(element_id)?;

        let mut cross_subgraph_target = false;
        for defining_req_id in self.get_defining_requirements(reused_contract_context_identifier) {
            let Some(defining_root_id) = self.resolve_single_owning_capability(&defining_req_id)
            else {
                continue;
            };

            if defining_root_id != source_root_id {
                cross_subgraph_target = true;
                break;
            }
        }

        if !cross_subgraph_target {
            return None;
        }

        let conflicting_root_id = self
            .get_defining_requirements(reused_contract_context_identifier)
            .into_iter()
            .filter_map(|defining_req_id| self.resolve_single_owning_capability(&defining_req_id))
            .find(|target_root_id| {
                target_root_id != &source_root_id
                    && self.has_reused_contract_context_flow_between_roots(
                        target_root_id,
                        &source_root_id,
                    )
            })?;
        let reused_context_name = self.display_name_for_element(reused_contract_context_identifier);
        let conflicting_root_name = self.display_name_for_element(&conflicting_root_id);
        let source_root_name = self.display_name_for_element(&source_root_id);

        let mut msg = format!(
            "'{}' cannot be reused to '{}' because subgraph '{}' already reuses contracts owned by subgraph '{}'. ReusedContractContextEntry flow between subgraphs must remain one-directional.",
            reused_context_name,
            element_name,
            conflicting_root_name,
            source_root_name
        );

        if let Some(file_path) = file_path {
            msg.push_str(&format!(
                " (file: {}, element: {})",
                file_path, element_name
            ));
        }

        Some(msg)
    }

    fn get_capability_ownership_parent_ids(&self, element: &Element) -> Vec<String> {
        let mut parent_ids: BTreeSet<String> = BTreeSet::new();

        for relation in &element.relations {
            let expected_parent_type = match (&element.element_type, relation.relation_type.name) {
                (ElementType::Capability, "derivedFrom") => "capability",
                (ElementType::Requirement(_), "derivedFrom") => "requirement",
                (ElementType::Requirement(_), "specify") => "capability",
                _ => continue,
            };

            if let LinkType::Identifier(ref target_id) = relation.target.link {
                if let Some(parent_identifier) =
                    self.resolve_relation_identifier(element, target_id)
                {
                    if let Some(parent) = self.get_element(&parent_identifier) {
                        let parent_matches = match expected_parent_type {
                            "capability" => matches!(parent.element_type, ElementType::Capability),
                            "requirement" => {
                                matches!(parent.element_type, ElementType::Requirement(_))
                            }
                            _ => false,
                        };
                        if parent_matches {
                            parent_ids.insert(parent.identifier.clone());
                        }
                    }
                }
            }
        }

        parent_ids.into_iter().collect()
    }

    fn resolve_relation_identifier(
        &self,
        source_element: &Element,
        target_id: &str,
    ) -> Option<String> {
        if self.nodes.contains_key(target_id) {
            return Some(target_id.to_string());
        }

        // Same-file fragment forms used during CRUD mutations ("#fragment" or "fragment")
        let fragment = target_id.trim_start_matches('#');
        if !fragment.is_empty() {
            let same_file_identifier = format!("{}#{}", source_element.file_path, fragment);
            if self.nodes.contains_key(&same_file_identifier) {
                return Some(same_file_identifier);
            }
        }

        // Relative path identifiers (e.g. ../X.md#y) can be normalized with source file context.
        let base_path = std::path::Path::new(&source_element.file_path)
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        if let Ok(normalized) = crate::utils::normalize_identifier(target_id, base_path) {
            if self.nodes.contains_key(&normalized) {
                return Some(normalized);
            }
        }

        None
    }

    /// Canonicalize an identifier-style relation target for a given source context.
    ///
    /// Returns the full canonical identifier when possible, including same-file fragments.
    fn normalize_relation_identifier_for_source(
        &self,
        source_file_path: &str,
        target_id: &str,
    ) -> Option<String> {
        if self.nodes.contains_key(target_id) {
            return Some(target_id.to_string());
        }

        let (_, fragment_opt) = crate::utils::extract_path_and_fragment(target_id);

        // Same-file fragment references stay local to the source file.
        if target_id.starts_with('#') {
            if let Some(fragment) = fragment_opt {
                let same_file_identifier = format!("{}#{}", source_file_path, fragment);
                if self.nodes.contains_key(&same_file_identifier) {
                    return Some(same_file_identifier);
                }
            }
        }

        // For relative references such as "../File.md#x" and "File.md#x", resolve from source file.
        let base_path = std::path::Path::new(source_file_path)
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        if let Ok(normalized) = crate::utils::normalize_identifier(target_id, base_path) {
            if self.nodes.contains_key(&normalized) {
                return Some(normalized);
            }
        }

        // For root-relative references like "/dir/File.md#x"
        if target_id.starts_with('/') {
            if let Ok(normalized) =
                crate::utils::normalize_identifier(target_id, &std::path::Path::new("/"))
            {
                if self.nodes.contains_key(&normalized) {
                    return Some(normalized);
                }
            }
        }

        None
    }

    /// Returns true if `target_id`, after canonicalization in the source context,
    /// resolves to `expected_target`.
    fn relation_targets_same_identifier(
        &self,
        source_file_path: &str,
        target_id: &str,
        expected_target: &str,
    ) -> bool {
        // Graph mutation callers should normally see already-canonical identifiers.
        if target_id == expected_target {
            return true;
        }

        // Keep fragment-only support for CRUD inputs that are scoped to the source file.
        if target_id.starts_with('#') {
            if format!("{}{}", source_file_path, target_id) == expected_target {
                return true;
            }
        }

        let base_path = std::path::Path::new(source_file_path)
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        if let Ok(normalized) = crate::utils::normalize_identifier(target_id, base_path) {
            return normalized == expected_target;
        }

        self.normalize_relation_identifier_for_source(source_file_path, target_id)
            .is_some_and(|resolved| resolved == expected_target)
    }

    fn normalize_element_relation_targets(&mut self) {
        let element_ids: Vec<String> = self.nodes.keys().cloned().collect();

        for source_id in &element_ids {
            let source_file_path = self
                .nodes
                .get(source_id)
                .map(|source_node| source_node.element.file_path.clone());
            let current_relations = self
                .nodes
                .get(source_id)
                .map(|source_node| source_node.element.relations.clone());

            let (Some(source_file_path), Some(mut normalized_relations)) =
                (source_file_path, current_relations)
            else {
                continue;
            };

            if normalized_relations.is_empty() {
                continue;
            }

            let mut changed = false;
            for relation in &mut normalized_relations {
                if let crate::relation::LinkType::Identifier(target_id) = &relation.target.link {
                    if let Some(canonical_id) =
                        self.normalize_relation_identifier_for_source(&source_file_path, target_id)
                    {
                        relation.target.link =
                            crate::relation::LinkType::Identifier(canonical_id.clone());
                        relation.target.element_id = Some(canonical_id);
                        changed = true;
                    }
                }
            }

            if changed {
                if let Some(mut_node) = self.nodes.get_mut(source_id) {
                    mut_node.element.relations = normalized_relations;
                }
            }
        }
    }

    /// Validates reused_contract_context targets and scope rules.
    fn validate_reused_contract_context(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating reused_contract_context targets...");
        let mut errors = Vec::new();

        let mut sorted_nodes: Vec<&ElementNode> = self.nodes.values().collect();
        sorted_nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

        for element_node in sorted_nodes {
            let element = &element_node.element;

            for reused_contract_context in &element.reused_contract_context {
                if !element.element_type.is_requirement() {
                    errors.push(ReqvireError::InvalidReusedContractContextTarget(format!(
                        "File {}: Element '{}' (type: {}) cannot author reused_contract_context. Only requirement elements may author reused_contract_context to reusable requirement-owned contracts; ontology vocabulary uses Concept References and semantic contracts use use/usedBy.",
                        element.file_path,
                        element.name,
                        element.element_type.as_str(),
                    )));
                    continue;
                }

                match &reused_contract_context.target {
                    crate::element::ReusedContractContextTarget::FilePath(file_path) => {
                        errors.push(ReqvireError::InvalidReusedContractContextTarget(format!(
                            "File {}: Element '{}' has reused_contract_context '{}' which is a file path. Reused Contract Context must target reusable element identifiers (file.md#element-id).",
                            element.file_path,
                            element.name,
                            file_path.display()
                        )));
                        continue;
                    }
                    crate::element::ReusedContractContextTarget::ElementIdentifier(identifier) => {
                        // Validate that the identifier points to an existing Contract element
                        if let Some(target_node) = self.nodes.get(identifier) {
                            if !target_node.element.element_type.is_contract() {
                                errors.push(ReqvireError::InvalidReusedContractContextTarget(
                                    format!(
                                        "File {}: Element '{}' has reused_contract_context to '{}' which is not an reusable element",
                                        element.file_path,
                                        element.name,
                                        identifier
                                    ),
                                ));
                                continue;
                            }

                            let reused_context_type_valid =
                                target_node.element.element_type.is_requirement_contract();

                            if !reused_context_type_valid {
                                errors.push(ReqvireError::InvalidReusedContractContextTarget(
                                    format!(
                                        "File {}: Element '{}' (type: {}) has invalid reused_contract_context to '{}' (type: {}). Requirement reused_contract_context may target requirement-owned source, constraint, behavior, specification, state, or input-output only. Ontology vocabulary uses Concept References; semantic contracts constrain requirements through constrainedBy/constrain.",
                                        element.file_path,
                                        element.name,
                                        element.element_type.as_str(),
                                        identifier,
                                        target_node.element.element_type.as_str()
                                    ),
                                ));
                                continue;
                            }

                            // Check 1: Contract targets must have a define relation. Ontology is not a contract.
                            if target_node.element.element_type.is_contract()
                                && !self.contract_has_define_relation(identifier)
                            {
                                errors.push(ReqvireError::InvalidReusedContractContextTarget(
                                    format!(
                                        "'{}' has no define relation. Contracts must define a requirement before they can be reused; contracts are requirement-owned only. Capabilities use concept references for ontology terms and are specified/verified, not defined by implementation-detail contracts. (file: {}, element: {})",
                                        target_node.element.name,
                                        element.file_path,
                                        element.name
                                    ),
                                ));
                                continue;
                            }

                            // Check 2: Hierarchical Independence Constraint - reusesContract element must not be in defining hierarchy
                            let defining_reqs = self.get_defining_requirements(identifier);
                            let mut hierarchy_violation = false;
                            for defining_req_id in defining_reqs {
                                if self.is_in_hierarchy(&element.identifier, &defining_req_id) {
                                    errors.push(ReqvireError::InvalidReusedContractContextScope(
                                        format!(
                                            "'{}' cannot be reused to '{}' because it is within the contract's defining hierarchy. Reused Contract Context are only allowed from elements outside the definedBy chain. (file: {}, element: {})",
                                            target_node.element.name,
                                            element.name,
                                            element.file_path,
                                            element.name
                                        ),
                                    ));
                                    hierarchy_violation = true;
                                    break;
                                }
                            }

                            // Check 3: One-direction subgraph flow constraint (only if no hierarchy violation)
                            if !hierarchy_violation {
                                if let Some(msg) = self
                                    .build_reused_contract_context_direction_scope_error(
                                        identifier,
                                        &element.identifier,
                                        &element.name,
                                        Some(&element.file_path),
                                    )
                                {
                                    errors
                                        .push(ReqvireError::InvalidReusedContractContextScope(msg));
                                    hierarchy_violation = true;
                                }
                            }

                            // Check 4: Upstream propagation constraint (only if no other scope violation)
                            if !hierarchy_violation {
                                if let Some((direction, other_id)) = self
                                    .find_duplicate_reused_contract_context_in_hierarchy(
                                        &element.identifier,
                                        &reused_contract_context.target,
                                    )
                                {
                                    let other_name = self
                                        .nodes
                                        .get(&other_id)
                                        .map(|n| n.element.name.as_str())
                                        .unwrap_or(&other_id);
                                    let msg = if direction == "ancestor" {
                                        format!(
                                            "'{}' is already reused at '{}' which is an ancestor. Reused Contract Context propagate downstream. (file: {}, element: {})",
                                            target_node.element.name,
                                            other_name,
                                            element.file_path,
                                            element.name
                                        )
                                    } else {
                                        format!(
                                            "'{}' is already reused at '{}' which is a descendant. Move reused_contract_context to '{}' if you want it at higher level. (file: {}, element: {})",
                                            target_node.element.name,
                                            other_name,
                                            element.name,
                                            element.file_path,
                                            element.name
                                        )
                                    };
                                    errors
                                        .push(ReqvireError::InvalidReusedContractContextScope(msg));
                                }
                            }
                        } else {
                            errors.push(ReqvireError::MissingReusedContractContextTarget(format!(
                                "File {}: Element '{}' references missing reused_contract_context element: {}",
                                element.file_path, element.name, identifier
                            )));
                        }
                    }
                }
            }
        }

        if errors.is_empty() {
            debug!("No reused_contract_context validation errors found.");
        } else {
            debug!(
                "{} reused_contract_context validation errors found.",
                errors.len()
            );
        }

        Ok(errors)
    }

    /// Return the derived semantic-contract IRI for a semantic-contract element.
    pub fn semantic_contract_iri(&self, element_id: &str) -> Option<String> {
        let element = &self.nodes.get(element_id)?.element;
        if element.element_type.is_semantic_contract() {
            Some(element.semantic_contract_iri())
        } else {
            None
        }
    }

    /// Get the elements that own a contract via `definedBy`.
    pub fn get_contract_owners(&self, contract_id: &str) -> Vec<String> {
        let mut owners = Vec::new();

        let mut sorted_nodes: Vec<(&String, &ElementNode)> = self.nodes.iter().collect();
        sorted_nodes.sort_by(|(a_id, _), (b_id, _)| a_id.cmp(b_id));

        for (element_id, element_node) in sorted_nodes {
            // Check if this element has a definedBy relation pointing to the contract
            for relation in &element_node.element.relations {
                // Use CONTRACT_RELATIONS - definedBy is the forward contract relation from requirement
                if relation::is_contract_relation(relation.relation_type)
                    && relation.relation_type.name == CONTRACT_RELATIONS[1]
                // definedBy
                {
                    if let LinkType::Identifier(target_id) = &relation.target.link {
                        if target_id == contract_id {
                            owners.push(element_id.clone());
                        }
                    }
                }
            }
        }

        owners
    }

    /// Get the defining owners for a contract element.
    ///
    /// Kept for existing callers; the returned IDs may now be capability or requirement
    /// owners depending on the contract subtype.
    pub fn get_defining_requirements(&self, contract_id: &str) -> Vec<String> {
        self.get_contract_owners(contract_id)
    }

    pub fn get_capability_contract_owner(&self, contract_id: &str) -> Option<String> {
        let owners = self.get_contract_owners(contract_id);
        if owners.len() != 1 {
            return None;
        }
        let owner_id = owners[0].clone();
        let owner = self.nodes.get(&owner_id)?;
        if owner.element.element_type.is_capability() {
            Some(owner_id)
        } else {
            None
        }
    }

    pub fn get_requirement_contract_owner(&self, contract_id: &str) -> Option<String> {
        let owners = self.get_contract_owners(contract_id);
        if owners.len() != 1 {
            return None;
        }
        let owner_id = owners[0].clone();
        let owner = self.nodes.get(&owner_id)?;
        if owner.element.element_type.is_requirement() {
            Some(owner_id)
        } else {
            None
        }
    }

    pub fn get_requirement_ancestor_ids(&self, requirement_id: &str) -> Vec<String> {
        let mut ancestors = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = vec![requirement_id.to_string()];

        while let Some(current_id) = stack.pop() {
            let Some(node) = self.nodes.get(&current_id) else {
                continue;
            };

            let mut parents = Vec::new();
            for relation in &node.element.relations {
                if relation.relation_type.name != "derivedFrom" {
                    continue;
                }
                let LinkType::Identifier(parent_id) = &relation.target.link else {
                    continue;
                };
                if self
                    .nodes
                    .get(parent_id)
                    .is_some_and(|parent| parent.element.element_type.is_requirement())
                {
                    parents.push(parent_id.clone());
                }
            }
            parents.sort();
            for parent_id in parents {
                if visited.insert(parent_id.clone()) {
                    ancestors.push(parent_id.clone());
                    stack.push(parent_id);
                }
            }
        }

        ancestors
    }

    pub fn build_concept_reference_ontology_context(&self, element_id: &str) -> Vec<String> {
        let Some(node) = self.nodes.get(element_id) else {
            return Vec::new();
        };
        let element = &node.element;
        if element.element_type.is_ontology() || element.element_type.is_semantic_contract() {
            return Vec::new();
        }

        let (references, _) = crate::element::extract_concept_references(&element.content);
        if references.is_empty() {
            return Vec::new();
        }

        let semantic_index = semantic_contract::build_semantic_index(self);
        let context = self.build_concept_reference_context(element_id);
        let prefixes = self.build_ontology_prefix_map(&context, &semantic_index);
        let mut ontology_ids = BTreeSet::new();

        for reference in references {
            let Ok(resolved_iri) = resolve_concept_reference_iri(&reference.iri, &prefixes) else {
                continue;
            };
            let Some(declarations) = semantic_index.ontology_declarations.get(&resolved_iri) else {
                continue;
            };
            for declaration in declarations {
                ontology_ids.insert(declaration.element_identifier.clone());
            }
        }

        self.expand_ontology_context(ontology_ids)
    }

    fn expand_ontology_context(&self, ontology_ids: BTreeSet<String>) -> Vec<String> {
        let mut context = BTreeSet::new();
        let mut stack: Vec<String> = ontology_ids.into_iter().collect();

        while let Some(ontology_id) = stack.pop() {
            if !context.insert(ontology_id.clone()) {
                continue;
            }
            let Some(node) = self.nodes.get(&ontology_id) else {
                continue;
            };
            for relation in &node.element.relations {
                if relation.relation_type.name != "derivedFrom" {
                    continue;
                }
                let LinkType::Identifier(target_id) = &relation.target.link else {
                    continue;
                };
                if self
                    .nodes
                    .get(target_id)
                    .is_some_and(|target| target.element.element_type.is_ontology())
                {
                    stack.push(target_id.clone());
                }
            }
            for (candidate_id, candidate_node) in &self.nodes {
                if !candidate_node.element.element_type.is_ontology() {
                    continue;
                }
                if candidate_node.element.relations.iter().any(|relation| {
                    relation.relation_type.name == "derive"
                        && matches!(&relation.target.link, LinkType::Identifier(target_id) if target_id == &ontology_id)
                }) {
                    stack.push(candidate_id.clone());
                }
            }
        }

        context.into_iter().collect()
    }

    /// Check if a contract element has at least one `define` relation.
    /// Returns true if the contract has a define relation, false otherwise.
    pub fn contract_has_define_relation(&self, contract_id: &str) -> bool {
        if let Some(node) = self.nodes.get(contract_id) {
            node.element
                .relations
                .iter()
                // Use CONTRACT_RELATIONS - define is the backward contract relation from contract
                .any(|r| {
                    relation::is_contract_relation(r.relation_type)
                        && r.relation_type.name == CONTRACT_RELATIONS[0]
                }) // define
        } else {
            false
        }
    }

    /// Check if an element is in the derivation hierarchy of a root element.
    /// Returns true if element_id is the root itself, an ancestor, or a descendant of root_id.
    /// Used for reused_contract_context scope validation to check hierarchical independence.
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
        self.is_ancestor_of_recursive(
            potential_ancestor,
            element_id,
            &hierarchical_types,
            &mut visited,
        )
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
                        if self.is_ancestor_of_recursive(
                            potential_ancestor,
                            parent_id,
                            hierarchical_types,
                            visited,
                        ) {
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

    /// Get the owner requirements for a file reused_contract_context (via satisfiedBy or definedBy relation).
    pub fn get_file_owners(&self, file_path: &std::path::Path) -> Vec<String> {
        let mut owners = Vec::new();
        let file_path_str = file_path.to_string_lossy();

        for (element_id, node) in &self.nodes {
            for relation in &node.element.relations {
                // File ownership can come from satisfiedBy (implementations) or definedBy (contracts)
                let is_ownership_relation = (relation::is_satisfaction_relation(relation.relation_type)
                    && relation.relation_type.name == SATISFACTION_RELATIONS[1]) // satisfiedBy
                    || (relation::is_contract_relation(relation.relation_type)
                    && relation.relation_type.name == CONTRACT_RELATIONS[1]); // definedBy
                if is_ownership_relation {
                    if let LinkType::InternalPath(ref target_path) = relation.target.link {
                        if target_path.to_string_lossy() == file_path_str {
                            owners.push(element_id.clone());
                        }
                    }
                }
            }
        }
        owners
    }

    /// Find if the same reused_contract_context exists in hierarchy (ancestor or descendant).
    /// Returns (direction, element_id) where direction is "ancestor" or "descendant".
    pub fn find_duplicate_reused_contract_context_in_hierarchy(
        &self,
        element_id: &str,
        reused_contract_context: &crate::element::ReusedContractContextTarget,
    ) -> Option<(&'static str, String)> {
        // Check ancestors
        if let Some(ancestor) =
            self.find_reused_contract_context_in_ancestors(element_id, reused_contract_context)
        {
            return Some(("ancestor", ancestor));
        }
        // Check descendants
        if let Some(descendant) =
            self.find_reused_contract_context_in_descendants(element_id, reused_contract_context)
        {
            return Some(("descendant", descendant));
        }
        None
    }

    fn find_reused_contract_context_in_ancestors(
        &self,
        element_id: &str,
        reused_contract_context: &crate::element::ReusedContractContextTarget,
    ) -> Option<String> {
        let hierarchical_types = get_hierarchical_relation_types();
        let mut visited = HashSet::new();
        self.find_reused_contract_context_in_ancestors_recursive(
            element_id,
            reused_contract_context,
            &hierarchical_types,
            &mut visited,
        )
    }

    fn find_reused_contract_context_in_ancestors_recursive(
        &self,
        element_id: &str,
        reused_contract_context: &crate::element::ReusedContractContextTarget,
        hierarchical_types: &[&str],
        visited: &mut HashSet<String>,
    ) -> Option<String> {
        if visited.contains(element_id) {
            return None;
        }
        visited.insert(element_id.to_string());

        if let Some(node) = self.nodes.get(element_id) {
            for relation in &node.element.relations {
                if hierarchical_types.contains(&relation.relation_type.name) {
                    if let LinkType::Identifier(parent_id) = &relation.target.link {
                        if let Some(parent_node) = self.nodes.get(parent_id) {
                            // Check if parent has this reused_contract_context
                            if parent_node.element.reused_contract_context.iter().any(|a| {
                                self.reused_contract_context_targets_equal(
                                    &a.target,
                                    reused_contract_context,
                                )
                            }) {
                                return Some(parent_id.clone());
                            }
                        }
                        // Check ancestors recursively
                        if let Some(found) = self
                            .find_reused_contract_context_in_ancestors_recursive(
                                parent_id,
                                reused_contract_context,
                                hierarchical_types,
                                visited,
                            )
                        {
                            return Some(found);
                        }
                    }
                }
            }
        }
        None
    }

    fn find_reused_contract_context_in_descendants(
        &self,
        element_id: &str,
        reused_contract_context: &crate::element::ReusedContractContextTarget,
    ) -> Option<String> {
        let mut visited = HashSet::new();
        self.find_reused_contract_context_in_descendants_recursive(
            element_id,
            reused_contract_context,
            &mut visited,
        )
    }

    fn find_reused_contract_context_in_descendants_recursive(
        &self,
        element_id: &str,
        reused_contract_context: &crate::element::ReusedContractContextTarget,
        visited: &mut HashSet<String>,
    ) -> Option<String> {
        if visited.contains(element_id) {
            return None;
        }
        visited.insert(element_id.to_string());

        // Find all elements that have derivedFrom pointing to element_id
        for (child_id, child_node) in &self.nodes {
            let is_child = child_node.element.relations.iter().any(|r| {
                get_hierarchical_relation_types().contains(&r.relation_type.name)
                    && matches!(&r.target.link, LinkType::Identifier(id) if id == element_id)
            });

            if is_child {
                // Check if child has this reused_contract_context
                if child_node.element.reused_contract_context.iter().any(|a| {
                    self.reused_contract_context_targets_equal(&a.target, reused_contract_context)
                }) {
                    return Some(child_id.clone());
                }
                // Check descendants recursively
                if let Some(found) = self.find_reused_contract_context_in_descendants_recursive(
                    child_id,
                    reused_contract_context,
                    visited,
                ) {
                    return Some(found);
                }
            }
        }
        None
    }

    fn reused_contract_context_targets_equal(
        &self,
        a: &crate::element::ReusedContractContextTarget,
        b: &crate::element::ReusedContractContextTarget,
    ) -> bool {
        match (a, b) {
            (
                crate::element::ReusedContractContextTarget::FilePath(p1),
                crate::element::ReusedContractContextTarget::FilePath(p2),
            ) => p1 == p2,
            (
                crate::element::ReusedContractContextTarget::ElementIdentifier(id1),
                crate::element::ReusedContractContextTarget::ElementIdentifier(id2),
            ) => id1 == id2,
            _ => false,
        }
    }

    fn validate_legacy_contract_relations(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        let mut errors = Vec::new();

        for element_node in self.nodes.values() {
            let element = &element_node.element;
            for relation in &element.relations {
                if !relation.user_created {
                    continue;
                }
                let replacement = match relation.relation_type.name {
                    "refinedBy" => "definedBy",
                    "refine" => "define",
                    _ => continue,
                };
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Element '{}' uses legacy relation '{}'. Use '{}' for requirement-owned contract elements, or run `reqvire migrate`.",
                    element.file_path,
                    element.name,
                    relation.relation_type.name,
                    replacement
                )));
            }
        }

        Ok(errors)
    }

    /// Validate contract element constraints
    /// Contract elements (constraint, behavior, specification) can only have define relations
    /// and cannot have reused_contract_context.
    fn validate_contract_elements(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating Contract element constraints...");
        let mut errors = Vec::new();

        for element_node in self.nodes.values() {
            let element = &element_node.element;

            if element.element_type.is_semantic_contract() {
                let invalid_relations: Vec<_> = element
                    .relations
                    .iter()
                    .filter(|r| r.user_created)
                    .filter(|r| !matches!(r.relation_type.name, "constrain" | "use"))
                    .collect();

                if !invalid_relations.is_empty() {
                    let invalid_types: Vec<_> = invalid_relations
                        .iter()
                        .map(|r| &r.relation_type.name)
                        .collect();
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "File {}: Semantic contract element '{}' can only have constrain and use relations. Invalid relations: {:?}",
                        element.file_path, element.name, invalid_types
                    )));
                }

                if !element.reused_contract_context.is_empty() {
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "File {}: Semantic contract element '{}' cannot have reused_contract_context. Semantic contracts use ontology through use relations and constrain requirements through constrain/constrainedBy.",
                        element.file_path, element.name
                    )));
                }

                continue;
            }

            // Check if this is a non-semantic-contract Contract element type
            if element.element_type.is_contract() {
                // Contract elements can only have define relations
                let invalid_relations: Vec<_> = element
                    .relations
                    .iter()
                    .filter(|r| r.user_created)
                    .filter(|r| r.relation_type.name != "define")
                    .collect();

                if !invalid_relations.is_empty() {
                    let invalid_types: Vec<_> = invalid_relations
                        .iter()
                        .map(|r| &r.relation_type.name)
                        .collect();
                    errors.push(ReqvireError::InvalidMarkdownStructure(
                        format!(
                            "File {}: Contract element '{}' (type: {}) can only have define relations. Invalid relations: {:?}",
                            element.file_path,
                            element.name,
                            element.element_type.as_str(),
                            invalid_types
                        ),
                    ));
                }

                // Contract elements cannot have reused_contract_context
                if !element.reused_contract_context.is_empty() {
                    errors.push(ReqvireError::InvalidMarkdownStructure(
                        format!(
                            "File {}: Contract element '{}' (type: {}) cannot have reused_contract_context. Contract elements are atomic documentation units meant to be reused to requirements.",
                            element.file_path,
                            element.name,
                            element.element_type.as_str(),
                        ),
                    ));
                }
            }
        }

        if errors.is_empty() {
            debug!("No Contract element validation errors found.");
        } else {
            debug!("{} Contract element validation errors found.", errors.len());
        }

        Ok(errors)
    }

    fn validate_ontology_elements(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating ontology element constraints...");
        let mut errors = Vec::new();
        let mut ontology_ids: Vec<String> = self
            .nodes
            .iter()
            .filter_map(|(id, node)| {
                if node.element.element_type.is_ontology() {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();
        ontology_ids.sort();

        for ontology_id in &ontology_ids {
            let Some(node) = self.nodes.get(ontology_id) else {
                continue;
            };
            let element = &node.element;

            if !element.reused_contract_context.is_empty() {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Ontology element '{}' cannot have reused_contract_context. Ontology vocabulary is referenced through Concept References or semantic-contract use relations.",
                    element.file_path, element.name
                )));
            }

            for relation in element.relations.iter().filter(|r| r.user_created) {
                if !matches!(
                    relation.relation_type.name,
                    "derive" | "derivedFrom" | "usedBy"
                ) {
                    errors.push(ReqvireError::IncompatibleElementTypes(format!(
                        "Ontology element '{}' can only use derive, derivedFrom, or usedBy relations. Invalid relation: {}.",
                        element.identifier, relation.relation_type.name
                    )));
                }
            }

            let has_ontology_base = element
                .metadata
                .get("ontology_base")
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false);
            let has_ontology_prefix = element
                .metadata
                .get("ontology_prefix")
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false);
            if has_ontology_base && !has_ontology_prefix {
                errors.push(ReqvireError::InvalidMetadataFormat(format!(
                    "File {}: Ontology element '{}' with ontology_base metadata must define non-empty ontology_prefix metadata.",
                    element.file_path, element.name
                )));
            }
        }

        for ontology_id in self.find_root_elements_by_type("ontology") {
            let Some(node) = self.nodes.get(&ontology_id) else {
                continue;
            };
            let element = &node.element;
            if element
                .metadata
                .get("ontology_base")
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .is_none()
            {
                errors.push(ReqvireError::InvalidMetadataFormat(format!(
                    "File {}: Top parent ontology element '{}' must define non-empty ontology_base metadata.",
                    element.file_path, element.name
                )));
            }
            if element
                .metadata
                .get("ontology_prefix")
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .is_none()
            {
                errors.push(ReqvireError::InvalidMetadataFormat(format!(
                    "File {}: Top parent ontology element '{}' must define non-empty ontology_prefix metadata.",
                    element.file_path, element.name
                )));
            }
        }

        if ontology_ids.len() > 1 {
            let mut visited = BTreeSet::new();
            let mut stack = vec![ontology_ids[0].clone()];
            while let Some(current_id) = stack.pop() {
                if !visited.insert(current_id.clone()) {
                    continue;
                }
                let Some(node) = self.nodes.get(&current_id) else {
                    continue;
                };
                for relation in &node.element.relations {
                    if !matches!(relation.relation_type.name, "derive" | "derivedFrom") {
                        continue;
                    }
                    let LinkType::Identifier(target_id) = &relation.target.link else {
                        continue;
                    };
                    if self
                        .nodes
                        .get(target_id)
                        .is_some_and(|target| target.element.element_type.is_ontology())
                    {
                        stack.push(target_id.clone());
                    }
                }
                for (candidate_id, candidate_node) in &self.nodes {
                    if !candidate_node.element.element_type.is_ontology() {
                        continue;
                    }
                    for relation in &candidate_node.element.relations {
                        if !matches!(relation.relation_type.name, "derive" | "derivedFrom") {
                            continue;
                        }
                        let LinkType::Identifier(target_id) = &relation.target.link else {
                            continue;
                        };
                        if target_id == &current_id {
                            stack.push(candidate_id.clone());
                        }
                    }
                }
            }

            let disconnected: Vec<String> = ontology_ids
                .iter()
                .filter(|id| !visited.contains(*id))
                .cloned()
                .collect();
            if !disconnected.is_empty() {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Disconnected ontology graph: ontology elements must belong to one connected ontology root graph through derive/derivedFrom relations. Disconnected ontology elements: {}.",
                    disconnected.join(", ")
                )));
            }
        }

        Ok(errors)
    }

    fn validate_governance_metadata(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating requirement governance metadata...");
        let mut errors = Vec::new();

        for element_node in self.nodes.values() {
            let element = &element_node.element;
            let governance_keys: Vec<&str> = element
                .metadata
                .keys()
                .filter(|key| crate::element::is_governance_metadata_key(key))
                .map(String::as_str)
                .collect();

            if governance_keys.is_empty() {
                continue;
            }

            if !element.element_type.is_governance_bearing() {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Element '{}' (type: {}) declares requirement governance metadata keys {:?}. Governance metadata is only valid on capability and requirement elements.",
                    element.file_path,
                    element.name,
                    element.element_type.as_str(),
                    governance_keys
                )));
                continue;
            }

            if let Some(value) = element.metadata.get("status") {
                if !crate::element::is_valid_governance_status(value) {
                    errors.push(ReqvireError::InvalidMetadataFormat(format!(
                        "File {}: Requirement '{}' has invalid governance metadata status '{}'. Accepted values: {}.",
                        element.file_path,
                        element.name,
                        value,
                        crate::element::GOVERNANCE_STATUS_VALUES.join(", ")
                    )));
                }
            }

            if let Some(value) = element.metadata.get("priority") {
                if !crate::element::is_valid_governance_priority(value) {
                    errors.push(ReqvireError::InvalidMetadataFormat(format!(
                        "File {}: Requirement '{}' has invalid governance metadata priority '{}'. Accepted values: {}.",
                        element.file_path,
                        element.name,
                        value,
                        crate::element::GOVERNANCE_PRIORITY_VALUES.join(", ")
                    )));
                }
            }

            if let Some(value) = element.metadata.get("risk") {
                if !crate::element::is_valid_governance_risk(value) {
                    errors.push(ReqvireError::InvalidMetadataFormat(format!(
                        "File {}: Requirement '{}' has invalid governance metadata risk '{}'. Accepted values: {}.",
                        element.file_path,
                        element.name,
                        value,
                        crate::element::GOVERNANCE_RISK_VALUES.join(", ")
                    )));
                }
            }
        }

        Ok(errors)
    }

    pub fn validate_semantic_contracts_in_memory(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        self.validate_semantic_contracts(None)
    }

    pub fn validate_semantic_contracts_after_removal(
        &self,
        removed_declaration_source: &str,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        self.validate_semantic_contracts(Some(removed_declaration_source))
    }

    fn validate_semantic_contracts(
        &self,
        removed_declaration_source: Option<&str>,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        let mut errors = Vec::new();
        let semantic_index = semantic_contract::build_semantic_index(self);
        for diagnostic in &semantic_index.diagnostics {
            errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                "File {}: semantic model element '{}' at line {}: {}",
                diagnostic.file_path, diagnostic.source, diagnostic.line_number, diagnostic.message
            )));
        }

        for node in self.nodes.values() {
            let element = &node.element;
            if element.element_type.is_semantic_contract() {
                if self
                    .semantic_contract_used_ontology_context(&element.identifier)
                    .is_empty()
                {
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "Semantic contract '{}' must use at least one ontology element through a use/usedBy relation.",
                        element.identifier
                    )));
                }
            }
        }

        for reference in &semantic_index.shape_references {
            if semantic_index
                .ontology_declarations
                .contains_key(&reference.iri)
            {
                let context =
                    self.semantic_contract_used_ontology_context(&reference.element_identifier);
                if context.is_empty() {
                    continue;
                }
                let context: BTreeSet<String> = context.into_iter().collect();
                let declaration_sources: BTreeSet<String> = semantic_index
                    .ontology_declarations
                    .get(&reference.iri)
                    .into_iter()
                    .flat_map(|declarations| {
                        declarations
                            .iter()
                            .map(|declaration| declaration.element_identifier.clone())
                    })
                    .collect();
                if declaration_sources
                    .iter()
                    .any(|declaration_source| context.contains(declaration_source))
                {
                    continue;
                }

                let declaring_contract = declaration_sources
                    .iter()
                    .next()
                    .cloned()
                    .unwrap_or_else(|| "unknown semantic contract".to_string());
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "Semantic reference outside context: semantic contract '{}' references {} <{}>, declared by ontology '{}', but that ontology is not reachable through the contract's use relations. Add a use relation to the declaring ontology or an ontology descendant with the declaring ontology in its hierarchy.",
                        reference.element_identifier,
                        reference.kind,
                        reference.iri,
                        declaring_contract
                    )));
                continue;
            }

            let removed_source = removed_declaration_source
                .map(|source| format!(" Removed declaration source: {}.", source))
                .unwrap_or_default();
            errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Semantic reference not found: semantic contract '{}' references {} <{}>, but no ontology element declares this IRI.{} Update or remove the SHACL reference before deleting or editing the declaring ontology.",
                    reference.element_identifier, reference.kind, reference.iri, removed_source
                )));
        }

        errors.extend(self.validate_semantic_contract_shape_prefixes(&semantic_index));

        errors
            .extend(self.validate_concept_references(&semantic_index, removed_declaration_source));

        for (iri, declarations) in semantic_index.ontology_declarations {
            let authored_declarations: Vec<_> = declarations
                .iter()
                .filter(|declaration| !declaration.external)
                .collect();
            let owners: BTreeSet<String> = authored_declarations
                .iter()
                .map(|declaration| declaration.element_identifier.clone())
                .collect();
            if owners.len() > 1 {
                let owner_list = owners.iter().cloned().collect::<Vec<_>>().join(", ");
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Duplicate ontology term declaration: <{}> is declared by multiple ontology elements: {}.",
                    iri, owner_list
                )));
            }

            let mut conflicting_roles = BTreeSet::new();
            for left in &authored_declarations {
                for right in &authored_declarations {
                    if left.role.conflicts_with(right.role) {
                        conflicting_roles.insert(left.role);
                        conflicting_roles.insert(right.role);
                    }
                }
            }

            if !conflicting_roles.is_empty() {
                let roles = conflicting_roles
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                let owner_list = owners.iter().cloned().collect::<Vec<_>>().join(", ");
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Conflicting ontology term declaration: <{}> is declared with incompatible roles ({}) across ontology elements: {}.",
                    iri, roles, owner_list
                )));
            }
        }

        Ok(errors)
    }

    fn validate_semantic_contract_shape_prefixes(
        &self,
        semantic_index: &semantic_contract::SemanticIndex,
    ) -> Vec<ReqvireError> {
        let mut errors = Vec::new();
        let mut references_by_contract: HashMap<&str, Vec<&semantic_contract::ShapeIriReference>> =
            HashMap::new();

        for reference in &semantic_index.shape_references {
            references_by_contract
                .entry(reference.element_identifier.as_str())
                .or_default()
                .push(reference);
        }

        for block in &semantic_index.blocks {
            if !matches!(block.kind, semantic_contract::SemanticBlockKind::Shapes) {
                continue;
            }

            let context = self.semantic_contract_used_ontology_context(&block.source);
            if context.is_empty() {
                continue;
            }

            let context_set: BTreeSet<&str> = context.iter().map(String::as_str).collect();
            let project_namespaces =
                self.used_ontology_project_namespaces(&context_set, semantic_index);
            if project_namespaces.is_empty() {
                continue;
            }

            let shape_prefixes = parse_turtle_prefixes(&block.content);
            let mut declared_namespaces = BTreeSet::new();
            for (prefix, namespace) in &shape_prefixes {
                declared_namespaces.insert(namespace.clone());
                if let Some(expected_namespaces) = project_namespaces.by_prefix.get(prefix) {
                    if !expected_namespaces.contains(namespace) {
                        let expected = expected_namespaces
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ");
                        errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                            "Shapes prefix validation failed: semantic contract '{}' declares prefix '{}' as '{}', but the used ontology graph defines that prefix as {}.",
                            block.source, prefix, namespace, expected
                        )));
                    }
                }
            }

            let Some(references) = references_by_contract.get(block.source.as_str()) else {
                continue;
            };

            for reference in references {
                let Some(declarations) = semantic_index.ontology_declarations.get(&reference.iri)
                else {
                    continue;
                };
                let declared_in_context = declarations.iter().any(|declaration| {
                    context_set.contains(declaration.element_identifier.as_str())
                });
                if !declared_in_context {
                    continue;
                }

                for namespace in project_namespaces.namespaces_for_iri(&reference.iri) {
                    if declared_namespaces.contains(namespace) {
                        continue;
                    }
                    let preferred_prefix = project_namespaces
                        .prefix_for_namespace(namespace)
                        .unwrap_or("project");
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "Shapes prefix validation failed: semantic contract '{}' references {} <{}> from used ontology namespace '{}', but its Shapes block does not explicitly declare a prefix for that namespace. Add `@prefix {}: <{}> .`; Reqvire does not inject hidden SHACL prefixes.",
                        block.source,
                        reference.kind,
                        reference.iri,
                        namespace,
                        preferred_prefix,
                        namespace
                    )));
                }
            }
        }

        errors
    }

    fn validate_concept_references(
        &self,
        semantic_index: &semantic_contract::SemanticIndex,
        removed_declaration_source: Option<&str>,
    ) -> Vec<ReqvireError> {
        let mut errors = Vec::new();

        for node in self.nodes.values() {
            let element = &node.element;
            let (references, diagnostics) =
                crate::element::extract_concept_references(&element.content);

            if element.element_type.is_ontology()
                && crate::element::has_subsection(&element.content, "Concept References")
            {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Ontology element '{}' must not contain a #### Concept References section. Ontology elements declare terms in #### Ontology.",
                    element.file_path, element.name
                )));
            }
            if element.element_type.is_semantic_contract()
                && crate::element::has_subsection(&element.content, "Concept References")
            {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Semantic contract element '{}' must not contain a #### Concept References section. Semantic contracts are semantic graph artifacts and use ontology through use/usedBy relations.",
                    element.file_path, element.name
                )));
            }

            for diagnostic in diagnostics {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Element '{}' {}",
                    element.file_path, element.name, diagnostic
                )));
            }

            if references.is_empty() {
                continue;
            }

            let context = self.build_concept_reference_context(&element.identifier);
            let context_set: BTreeSet<String> = context.iter().cloned().collect();
            let prefixes = self.build_ontology_prefix_map(&context, semantic_index);

            for reference in references {
                let resolved_iri = match resolve_concept_reference_iri(&reference.iri, &prefixes) {
                    Ok(iri) => iri,
                    Err(message) => {
                        errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                            "Concept reference syntax error: element '{}' label '{}' at line {} references '{}': {}",
                            element.identifier,
                            reference.label,
                            reference.line_number,
                            reference.iri,
                            message
                        )));
                        continue;
                    }
                };

                let Some(declarations) = semantic_index.ontology_declarations.get(&resolved_iri)
                else {
                    let removed_source = removed_declaration_source
                        .map(|source| format!(" Removed declaration source: {}.", source))
                        .unwrap_or_default();
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "Concept reference not found: element '{}' label '{}' references <{}>, but no ontology element declares this IRI.{} Update or remove the Concept References entry before deleting or editing the declaring ontology.",
                        element.identifier,
                        reference.label,
                        resolved_iri,
                        removed_source
                    )));
                    continue;
                };

                let declaration_sources: BTreeSet<String> = declarations
                    .iter()
                    .map(|declaration| declaration.element_identifier.clone())
                    .collect();
                if declaration_sources
                    .iter()
                    .any(|declaration_source| context_set.contains(declaration_source))
                {
                    continue;
                }

                continue;
            }
        }

        errors
    }

    fn build_concept_reference_context(&self, element_id: &str) -> Vec<String> {
        let Some(node) = self.nodes.get(element_id) else {
            return Vec::new();
        };
        let element = &node.element;

        if element.element_type.is_ontology() || element.element_type.is_semantic_contract() {
            return Vec::new();
        }

        self.nodes
            .iter()
            .filter_map(|(id, node)| {
                if node.element.element_type.is_ontology() {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn build_ontology_prefix_map(
        &self,
        ontology_context: &[String],
        semantic_index: &semantic_contract::SemanticIndex,
    ) -> HashMap<String, String> {
        let context: BTreeSet<&str> = ontology_context.iter().map(String::as_str).collect();
        let mut prefixes = HashMap::new();

        for block in &semantic_index.blocks {
            if !matches!(block.kind, semantic_contract::SemanticBlockKind::Ontology)
                || !context.contains(block.source.as_str())
            {
                continue;
            }

            for (prefix, iri) in parse_turtle_prefixes(&block.content) {
                prefixes.entry(prefix).or_insert(iri);
            }
        }

        for declaration in &semantic_index.ontology_documents {
            if declaration
                .element_identifiers
                .iter()
                .any(|identifier| context.contains(identifier.as_str()))
            {
                prefixes
                    .entry(declaration.ontology_prefix.clone())
                    .or_insert(declaration.term_namespace.clone());
            }
        }

        for source in &semantic_index.external_sources {
            if context.contains(source.owner_identifier.as_str()) {
                prefixes
                    .entry(source.prefix.clone())
                    .or_insert(source.namespace.clone());
            }
        }

        prefixes
    }

    fn semantic_contract_used_ontology_context(&self, contract_id: &str) -> Vec<String> {
        let Some(contract) = self.nodes.get(contract_id) else {
            return Vec::new();
        };
        if !contract.element.element_type.is_semantic_contract() {
            return Vec::new();
        }

        let mut context = BTreeSet::new();
        for relation in &contract.element.relations {
            if relation.relation_type.name != "use" {
                continue;
            }
            let LinkType::Identifier(ontology_id) = &relation.target.link else {
                continue;
            };
            if self
                .nodes
                .get(ontology_id)
                .is_some_and(|target| target.element.element_type.is_ontology())
            {
                context.insert(ontology_id.clone());
            }
        }

        for (ontology_id, ontology_node) in &self.nodes {
            if !ontology_node.element.element_type.is_ontology() {
                continue;
            }
            if ontology_node.element.relations.iter().any(|relation| {
                relation.relation_type.name == "usedBy"
                    && matches!(&relation.target.link, LinkType::Identifier(target_id) if target_id == contract_id)
            }) {
                context.insert(ontology_id.clone());
            }
        }

        self.expand_ontology_context(context)
    }

    fn used_ontology_project_namespaces(
        &self,
        ontology_context: &BTreeSet<&str>,
        semantic_index: &semantic_contract::SemanticIndex,
    ) -> UsedOntologyProjectNamespaces {
        let mut by_prefix: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut prefix_by_namespace: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for declaration in &semantic_index.ontology_documents {
            if !declaration
                .element_identifiers
                .iter()
                .any(|identifier| ontology_context.contains(identifier.as_str()))
            {
                continue;
            }

            by_prefix
                .entry(declaration.ontology_prefix.clone())
                .or_default()
                .insert(declaration.term_namespace.clone());
            prefix_by_namespace
                .entry(declaration.term_namespace.clone())
                .or_default()
                .insert(declaration.ontology_prefix.clone());
        }

        for source in &semantic_index.external_sources {
            if !ontology_context.contains(source.owner_identifier.as_str()) {
                continue;
            }

            by_prefix
                .entry(source.prefix.clone())
                .or_default()
                .insert(source.namespace.clone());
            prefix_by_namespace
                .entry(source.namespace.clone())
                .or_default()
                .insert(source.prefix.clone());
        }

        UsedOntologyProjectNamespaces {
            by_prefix,
            prefix_by_namespace,
        }
    }

    /// Validates that each contract is owned by at most one requirement via definedBy.
    /// A contract element or file can only appear as a target of definedBy from one owner.
    fn validate_contract_ownership_uniqueness(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating contract ownership uniqueness...");
        let mut errors = Vec::new();
        // Map from contract target (identifier or file path) to owning element identifier
        let mut ownership_map: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        for (element_id, element_node) in &self.nodes {
            for relation in &element_node.element.relations {
                if relation::is_contract_relation(relation.relation_type)
                    && relation.relation_type.name == CONTRACT_RELATIONS[1]
                // definedBy
                {
                    let target_key = relation.target.link.as_str().to_string();
                    if let Some(existing_owner) = ownership_map.get(&target_key) {
                        let target_name = &relation.target.text;
                        let owner_name = self
                            .nodes
                            .get(existing_owner)
                            .map(|n| n.element.name.as_str())
                            .unwrap_or(existing_owner);
                        let current_name = element_node.element.name.as_str();
                        let (first, second) = if owner_name < current_name {
                            (owner_name, current_name)
                        } else {
                            (current_name, owner_name)
                        };
                        errors.push(ReqvireError::InvalidMarkdownStructure(
                            format!(
                                "Contract '{}' is owned by multiple elements: '{}' and '{}'. Each contract can only be owned by one requirement via definedBy; contracts are requirement-owned only.",
                                target_name,
                                first,
                                second
                            ),
                        ));
                    } else {
                        ownership_map.insert(target_key, element_id.clone());
                    }
                }
            }
        }

        if errors.is_empty() {
            debug!("No contract ownership uniqueness violations found.");
        } else {
            debug!(
                "{} contract ownership uniqueness violations found.",
                errors.len()
            );
        }

        Ok(errors)
    }

    /// Validates that explicit 'other' type elements do not author semantic relations.
    fn validate_other_element_relations(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating 'other' element type relation constraints...");
        let mut errors = Vec::new();

        for element_node in self.nodes.values() {
            let element = &element_node.element;

            // Check if this is an 'other' type element
            if let crate::element::ElementType::Other(type_str) = &element.element_type {
                if type_str == "other" {
                    let authored_relations: Vec<_> = element
                        .relations
                        .iter()
                        .filter(|r| r.user_created)
                        .collect();

                    for relation in authored_relations {
                        errors.push(ReqvireError::IncompatibleElementTypes(
                            format!(
                                "Element type 'other' cannot author semantic relations: '{}' uses '{}' relation to '{}'. Use a specific supported element type or ontology concept reference.",
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
            debug!(
                "{} 'other' element type relation errors found.",
                errors.len()
            );
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
            errors.push(ReqvireError::CircularDependencyError(format!(
                "Circular dependency error: {}",
                full_cycle
            )));
            return;
        }

        // Add this element to the current traversal path.
        path.push(element_id.clone());

        // Traverse relations using their metadata to determine canonical direction
        for relation in &element.relations {
            if let LinkType::Identifier(ref target_id) = relation.target.link {
                // Skip relations that don't participate in dependency propagation, including backward relations.
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
        let matching: Vec<&String> = self
            .nodes
            .iter()
            .filter(|(_, node)| node.element.name == search_name)
            .map(|(id, _)| id)
            .collect();

        if matching.is_empty() {
            return Err(ReqvireError::MissingElement(format!(
                "Element not found: {}",
                element_name
            )));
        } else if matching.len() > 1 {
            return Err(ReqvireError::ProcessError(format!(
                "Multiple elements found with name '{}': {:?}",
                element_name, matching
            )));
        }

        Ok(matching[0].clone())
    }

    /// Moves an element to an existing file in the graph
    pub fn move_element_to_location(
        &mut self,
        element_id: &str,
        new_file_path: &str,
    ) -> Result<(), ReqvireError> {
        // Verify the target file exists in the graph (either has elements or is registered as a page)
        let target_has_elements = self
            .nodes
            .values()
            .any(|node| node.element.file_path == new_file_path);
        let target_is_page = self.pages.contains_key(new_file_path);

        if !target_has_elements && !target_is_page {
            return Err(ReqvireError::LocationNotFound(format!(
                "Target file '{}' does not exist in the graph",
                new_file_path
            )));
        }

        // '# Element' files represent exactly one implicit element.
        // Disallow moving additional elements into an existing single-element file.
        if target_has_elements && self.is_single_element_format_file(new_file_path) {
            let source_file_path = self
                .nodes
                .get(element_id)
                .map(|n| n.element.file_path.clone())
                .unwrap_or_default();
            if source_file_path != new_file_path {
                return Err(ReqvireError::InvalidOperation(format!(
                    "Cannot move element '{}' into '{}': target is a '# Element' file and can contain only one element.",
                    element_id, new_file_path
                )));
            }
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
                element_id,
                old_file_path,
                new_file_path
            );

            Ok(())
        } else {
            Err(ReqvireError::MissingElement(format!(
                "Element '{}' not found in graph",
                element_id
            )))
        }
    }

    /// Adds a new file location to the graph (virtual - no filesystem changes)
    pub fn add_file_location(&mut self, new_file_path: &str) -> Result<(), ReqvireError> {
        // Check if the file already exists
        let file_exists = self
            .nodes
            .values()
            .any(|node| node.element.file_path == new_file_path);

        if file_exists {
            return Err(ReqvireError::LocationAlreadyExists(format!(
                "File '{}' already exists in the graph",
                new_file_path
            )));
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

        self.nodes.insert(
            virtual_id,
            ElementNode {
                element: virtual_element,
                relations: Vec::new(),
            },
        );

        log::debug!("Added virtual file location '{}'", new_file_path);
        Ok(())
    }

    /// Moves element to a new file location (creates file location if needed)
    pub fn move_element_to_new_file(
        &mut self,
        element_id: &str,
        new_file_path: &str,
    ) -> Result<(), ReqvireError> {
        // Check if file exists, if not, create it virtually
        let file_exists = self
            .nodes
            .values()
            .any(|node| node.element.file_path == new_file_path);

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
                element_id,
                old_file_path,
                new_file_path
            );

            Ok(())
        } else {
            Err(ReqvireError::MissingElement(format!(
                "Element '{}' not found in graph",
                element_id
            )))
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

    fn build_impact_tree_recursive(
        &self,
        current_id: &str,
        visited: &mut BTreeSet<String>,
    ) -> ElementNode {
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

    pub fn resolve_governance_metadata(
        &self,
        element: &Element,
    ) -> Option<RequirementGovernanceMetadata> {
        if !element.element_type.is_governance_bearing() {
            return None;
        }

        Some(RequirementGovernanceMetadata {
            status: self.resolve_governance_entry(element, "status", "approved"),
            priority: self.resolve_governance_entry(element, "priority", "medium"),
            risk: self.resolve_governance_entry(element, "risk", "low"),
            owner: self.resolve_governance_entry(element, "owner", ""),
        })
    }

    fn resolve_governance_entry(
        &self,
        element: &Element,
        key: &str,
        default_value: &str,
    ) -> GovernanceMetadataEntry {
        if let Some(value) = element.metadata.get(key) {
            return GovernanceMetadataEntry {
                value: value.clone(),
                source: GovernanceMetadataSource::Explicit,
                source_identifier: None,
            };
        }

        for ancestor_id in self.governance_ancestors_nearest_first(&element.identifier) {
            if let Some(ancestor) = self.nodes.get(&ancestor_id).map(|node| &node.element) {
                if let Some(value) = ancestor.metadata.get(key) {
                    return GovernanceMetadataEntry {
                        value: value.clone(),
                        source: GovernanceMetadataSource::Inherited,
                        source_identifier: Some(ancestor.identifier.clone()),
                    };
                }
            }
        }

        GovernanceMetadataEntry {
            value: default_value.to_string(),
            source: GovernanceMetadataSource::Default,
            source_identifier: None,
        }
    }

    fn governance_ancestors_nearest_first(&self, element_id: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut current_level = vec![element_id.to_string()];

        visited.insert(element_id.to_string());

        while !current_level.is_empty() {
            let mut next_level = Vec::new();

            for current_id in &current_level {
                let mut parents = self.governance_parent_ids(current_id);
                parents.sort();

                for parent_id in parents {
                    if visited.insert(parent_id.clone()) {
                        result.push(parent_id.clone());
                        next_level.push(parent_id);
                    }
                }
            }

            current_level = next_level;
        }

        result
    }

    fn governance_parent_ids(&self, element_id: &str) -> Vec<String> {
        let mut parents = BTreeSet::new();

        if let Some(node) = self.nodes.get(element_id) {
            for relation in &node.element.relations {
                if matches!(relation.relation_type.name, "derivedFrom" | "specify") {
                    if let LinkType::Identifier(parent_id) = &relation.target.link {
                        if self.nodes.get(parent_id).is_some_and(|parent| {
                            parent.element.element_type.is_governance_bearing()
                        }) {
                            parents.insert(parent_id.clone());
                        }
                    }
                }
            }
        }

        for (candidate_id, candidate) in &self.nodes {
            if candidate.element.relations.iter().any(|relation| {
                relation.relation_type.name == "derive"
                    && matches!(&relation.target.link, LinkType::Identifier(target_id) if target_id == element_id)
            }) && candidate.element.element_type.is_governance_bearing()
            {
                parents.insert(candidate_id.clone());
            }
        }

        parents.into_iter().collect()
    }

    /// Find all elements of a type without hierarchical parent relations.
    pub fn find_root_elements_by_type(&self, element_type: &str) -> Vec<String> {
        let hierarchical_relations = relation::get_hierarchical_relation_types();

        let mut roots: Vec<String> = self
            .nodes
            .values()
            .map(|node| &node.element)
            .filter(|element| {
                if element.element_type.as_str() != element_type {
                    return false;
                }

                let has_parent = element
                    .relations
                    .iter()
                    .any(|r| hierarchical_relations.contains(&r.relation_type.name));

                !has_parent
            })
            .map(|e| e.identifier.clone())
            .collect();

        roots.sort();
        roots
    }

    /// Find all capabilities without hierarchical parent relations (capability roots).
    pub fn find_root_capabilities(&self) -> Vec<String> {
        self.find_root_elements_by_type("capability")
    }

    /// Find leaf elements for reverse traversal
    /// Leaf elements are those that:
    /// 1. Have backward relations (derivedFrom, satisfy, verify) - they trace upward to something
    /// 2. Have no outgoing forward relations to other elements - nothing derives from them
    ///    Optionally filter by element types
    pub fn find_leaf_elements(&self, type_filter: Option<&[&str]>) -> Vec<String> {
        let mut leaves: Vec<String> = self
            .nodes
            .values()
            .map(|node| &node.element)
            .filter(|element| {
                // Apply type filter if provided
                if let Some(types) = type_filter {
                    let element_type_str = element.element_type.as_str();
                    if !types.contains(&element_type_str) {
                        return false;
                    }
                }

                // Must have at least one backward relation (to trace upward)
                let has_backward_relations = element.relations.iter().any(|r| {
                    relation::BACKWARD_RELATIONS.contains(&r.relation_type.name)
                        && matches!(r.target.link, relation::LinkType::Identifier(_))
                });

                if !has_backward_relations {
                    return false;
                }

                // Must NOT have outgoing forward relations to elements (nothing derives from it)
                let has_forward_children = element.relations.iter().any(|r| {
                    relation::DIAGRAM_RELATIONS.contains(&r.relation_type.name)
                        && matches!(r.target.link, relation::LinkType::Identifier(_))
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
        let mut elements: Vec<String> = self
            .nodes
            .values()
            .map(|node| &node.element)
            .filter(|element| {
                let element_type_str = element.element_type.as_str();
                type_filter.contains(&element_type_str)
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
        let root_elements: Vec<&Element> = all_elements
            .iter()
            .filter(|element| {
                !element
                    .relations
                    .iter()
                    .any(|rel| parent_relation_types.contains(&rel.relation_type.name))
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
    fn collect_descendants<'a>(
        &self,
        all_elements: &[&'a Element],
        descendants: &mut Vec<&'a Element>,
    ) {
        let mut found_new = true;

        while found_new {
            found_new = false;
            let descendants_len = descendants.len();

            for element in all_elements {
                // Skip if already collected
                if descendants
                    .iter()
                    .any(|d| d.identifier == element.identifier)
                {
                    continue;
                }

                // Check if this element has a parent relation pointing to any element in descendants
                let has_parent_in_descendants = element.relations.iter().any(|rel| {
                    matches!(&rel.target.link, crate::relation::LinkType::Identifier(target_id)
                        if descendants.iter().any(|d| d.identifier == *target_id)
                        && ["containedBy", "derivedFrom", "define", "refine", "satisfy", "verify"].contains(&rel.relation_type.name))
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
    pub fn change_impact_with_relation(
        &self,
        element: &Element,
    ) -> Vec<(String, Vec<crate::relation::Relation>)> {
        if let Some(node) = self.nodes.get(&element.identifier) {
            // Group original relations by target ID using BTreeMap for deterministic ordering
            let mut relations_by_target: std::collections::BTreeMap<
                String,
                Vec<crate::relation::Relation>,
            > = std::collections::BTreeMap::new();

            for relation in &node.element.relations {
                let target_id = match &relation.target.link {
                    crate::relation::LinkType::Identifier(ref target_id) => target_id.clone(),
                    crate::relation::LinkType::InternalPath(ref path) => {
                        path.to_string_lossy().to_string()
                    }
                    crate::relation::LinkType::ExternalUrl(_) => continue, // Skip external URLs for change impact
                };

                relations_by_target
                    .entry(target_id)
                    .or_default()
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
        self.nodes
            .values()
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

    fn element_to_markdown_with_context(
        &self,
        element: &Element,
        _current_file: &str,
        with_full_relations: bool,
    ) -> String {
        let mut markdown = String::new();

        // Add the element header
        markdown.push_str(&format!("### {}\n\n", element.name));

        // Add the element content
        if !element.content.trim().is_empty() {
            markdown.push_str(element.content.trim_end());
            markdown.push('\n');
        }

        // Add metadata subsection
        // Always include metadata to preserve structure during CRUD operations
        let mut custom_metadata: Vec<_> = element
            .metadata
            .iter()
            .filter(|(key, _)| *key != "type" && *key != "_single_element_format") // type is handled separately
            .collect();
        custom_metadata.sort_by_key(|(key, _)| *key);

        markdown.push_str("#### Metadata\n");

        // Add type metadata
        markdown.push_str(&format!(
            "  * type: {}\n",
            element.element_type.to_metadata_string()
        ));

        // Add other metadata
        for (key, value) in custom_metadata {
            markdown.push_str(&format!("  * {}: {}\n", key, value));
        }
        markdown.push('\n');

        // Add reused_contract_context subsection if there are reused_contract_context
        // Deduplicate reused_contract_context by target, keeping first occurrence
        let mut seen_reused_contract_context: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        let unique_reused_contract_context: Vec<_> = element
            .reused_contract_context
            .iter()
            .filter(|a| seen_reused_contract_context.insert(a.target.as_str()))
            .collect();

        if !unique_reused_contract_context.is_empty() {
            markdown.push_str("#### ");
            markdown.push_str(REUSED_CONTRACT_CONTEXT_SECTION);
            markdown.push('\n');
            for reused_contract_context in unique_reused_contract_context {
                match &reused_contract_context.target {
                    crate::element::ReusedContractContextTarget::FilePath(file_path) => {
                        // ReusedContractContextEntry paths are stored as git-root-relative paths
                        let reused_context_path = file_path.to_string_lossy().to_string();

                        // Make the path relative to the current file's directory (same as relations)
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .to_path_buf();

                        // Use to_relative_identifier like we do for InternalPath relations
                        // Prepend "/" to indicate git-root-relative path
                        let absolute_path = format!("/{}", reused_context_path);
                        let relative_path = crate::utils::to_relative_identifier(
                            &absolute_path,
                            &current_folder,
                            false,
                        )
                        .unwrap_or_else(|_| reused_context_path.clone());

                        // Use filename as display text for cleaner markdown
                        let display_name = file_path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(&reused_context_path);

                        markdown.push_str(&format!("  * [{}]({})\n", display_name, relative_path));
                    }
                    crate::element::ReusedContractContextTarget::ElementIdentifier(identifier) => {
                        // Element identifier reused_contract_context - format as markdown link
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .to_path_buf();

                        // Use to_relative_identifier to make identifier relative to current file
                        let relative_id =
                            crate::utils::to_relative_identifier(identifier, &current_folder, true)
                                .unwrap_or_else(|_| identifier.clone());

                        // Look up actual element name from registry for human-readable display
                        let display_name = self
                            .get_element(identifier)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| {
                                // Fallback to identifier fragment if element not found
                                identifier
                                    .split('#')
                                    .next_back()
                                    .unwrap_or(identifier)
                                    .to_string()
                            });
                        markdown.push_str(&format!("  * [{}]({})\n", display_name, relative_id));
                    }
                }
            }
            markdown.push('\n');
        }

        // Add relations subsection if there are relations to include
        // When with_full_relations is true, include all relations (user-created and auto-generated)
        // Otherwise, only include user-created relations
        let mut relations_to_include: Vec<_> = if with_full_relations {
            element.relations.iter().collect()
        } else {
            element
                .relations
                .iter()
                .filter(|r| r.user_created)
                .collect()
        };
        // Sort relations for deterministic output: by relation type name, then by target link
        relations_to_include.sort_by(|a, b| {
            a.relation_type
                .name
                .cmp(&b.relation_type.name)
                .then(a.target.link.as_str().cmp(&b.target.link.as_str()))
        });
        // Remove duplicate relations (same relation_type + same target), keeping first occurrence
        relations_to_include.dedup_by(|a, b| {
            a.relation_type.name == b.relation_type.name
                && a.target.link.as_str() == b.target.link.as_str()
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
                    }
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
                            fragment
                                .replace('-', " ")
                                .split_whitespace()
                                .map(|word| {
                                    let mut chars = word.chars();
                                    match chars.next() {
                                        None => String::new(),
                                        Some(first) => {
                                            first.to_uppercase().collect::<String>()
                                                + chars.as_str()
                                        }
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
                        if target_file.is_empty()
                            || target_file == current_file_str
                            || target_id.starts_with('#')
                        {
                            format!("[{}](#{})", display_name, fragment)
                        } else {
                            // Make the link relative using just the folder of the current file
                            let current_folder = current_file_path
                                .parent()
                                .unwrap_or_else(|| std::path::Path::new("."))
                                .to_path_buf();

                            let relative_link = crate::utils::to_relative_identifier(
                                relation.target.link.as_str(),
                                &current_folder,
                                false,
                            )
                            .unwrap_or_else(|_| relation.target.link.as_str().to_string());

                            format!("[{}]({})", display_name, relative_link)
                        }
                    }
                    LinkType::InternalPath(path) => {
                        // For InternalPath, use the filename as display text and full relative path as link
                        let path_str = path.to_str().unwrap_or("invalid_path");
                        let display_name = std::path::Path::new(path_str)
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(path_str);

                        // Make the path relative using just the folder of the current file
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .to_path_buf();

                        let relative_link = crate::utils::to_relative_identifier(
                            relation.target.link.as_str(),
                            &current_folder,
                            false,
                        )
                        .unwrap_or_else(|_| relation.target.link.as_str().to_string());

                        format!("[{}]({})", display_name, relative_link)
                    }
                };

                markdown.push_str(&format!(
                    "  * {}: {}\n",
                    relation.relation_type.name, target_text
                ));
            }
            markdown.push('\n');
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
                let prev_line_is_header = result
                    .lines()
                    .last()
                    .is_some_and(|l| l.trim_start().starts_with("####"));
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
                .or_default()
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
                let fragment = e
                    .identifier
                    .split('#')
                    .next_back()
                    .unwrap_or(&e.identifier)
                    .to_string();
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
                            children_map.entry(parent_idx).or_default().push(idx);
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
        let original: Vec<&Element> = std::mem::take(elements);
        for idx in ordered_indices {
            elements.push(original[idx]);
        }
    }

    fn single_element_file_markdown(
        &self,
        file_path: &str,
        element: &Element,
        with_full_relations: bool,
    ) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Element\n\n");

        markdown.push_str("## Metadata\n");
        markdown.push_str(&format!(
            "  * type: {}\n",
            element.element_type.to_metadata_string()
        ));
        let mut custom_metadata: Vec<_> = element
            .metadata
            .iter()
            .filter(|(k, _)| *k != "type" && *k != "_single_element_format")
            .collect();
        custom_metadata.sort_by_key(|(k, _)| *k);
        for (k, v) in custom_metadata {
            markdown.push_str(&format!("  * {}: {}\n", k, v));
        }
        markdown.push('\n');

        let mut relations_to_include: Vec<_> = if with_full_relations {
            element.relations.iter().collect()
        } else {
            element
                .relations
                .iter()
                .filter(|r| r.user_created)
                .collect()
        };
        relations_to_include.sort_by(|a, b| {
            a.relation_type
                .name
                .cmp(&b.relation_type.name)
                .then(a.target.link.as_str().cmp(&b.target.link.as_str()))
        });
        relations_to_include.dedup_by(|a, b| {
            a.relation_type.name == b.relation_type.name
                && a.target.link.as_str() == b.target.link.as_str()
        });
        if !relations_to_include.is_empty() {
            markdown.push_str("## Relations\n");
            for relation in relations_to_include {
                let target_text = match &relation.target.link {
                    LinkType::ExternalUrl(url) => format!("[{}]({})", relation.target.text, url),
                    LinkType::Identifier(target_id) => {
                        let current_file_path = PathBuf::from(file_path);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| Path::new("."))
                            .to_path_buf();
                        let relative_id =
                            crate::utils::to_relative_identifier(target_id, &current_folder, true)
                                .unwrap_or_else(|_| target_id.clone());
                        let display_name = self
                            .get_element(target_id)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| relation.target.text.clone());
                        format!("[{}]({})", display_name, relative_id)
                    }
                    LinkType::InternalPath(path) => {
                        let current_file_path = PathBuf::from(file_path);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| Path::new("."))
                            .to_path_buf();
                        let path_str = path.to_string_lossy().to_string();
                        let absolute_path = format!("/{}", path_str);
                        let relative_path = crate::utils::to_relative_identifier(
                            &absolute_path,
                            &current_folder,
                            false,
                        )
                        .unwrap_or(path_str.clone());
                        let display_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(&path_str);
                        format!("[{}]({})", display_name, relative_path)
                    }
                };

                markdown.push_str(&format!(
                    "  * {}: {}\n",
                    relation.relation_type.name, target_text
                ));
            }
            markdown.push('\n');
        }

        if !element.reused_contract_context.is_empty() {
            markdown.push_str("## ");
            markdown.push_str(REUSED_CONTRACT_CONTEXT_SECTION);
            markdown.push('\n');
            for reused_contract_context in &element.reused_contract_context {
                match &reused_contract_context.target {
                    crate::element::ReusedContractContextTarget::FilePath(path) => {
                        let path_str = path.to_string_lossy().to_string();
                        markdown.push_str(&format!("  * [{}]({})\n", path_str, path_str));
                    }
                    crate::element::ReusedContractContextTarget::ElementIdentifier(id) => {
                        let display = self
                            .get_element(id)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| id.clone());
                        markdown.push_str(&format!("  * [{}]({})\n", display, id));
                    }
                }
            }
            markdown.push('\n');
        }

        markdown.push_str(&format!("## {}\n\n", element.name));
        if !element.content.trim().is_empty() {
            markdown.push_str(element.content.trim_end());
            markdown.push('\n');
        }

        markdown
    }

    /// Generates markdown content for a file
    /// When with_full_relations is true, includes all relations (user-created and auto-generated)
    pub fn generate_file_markdown(
        &self,
        file_path: &str,
        elements: &[&Element],
        with_full_relations: bool,
    ) -> String {
        if elements.len() == 1
            && elements[0]
                .metadata
                .get("_single_element_format")
                .map(|v| v == "true")
                .unwrap_or(false)
        {
            return self.single_element_file_markdown(file_path, elements[0], with_full_relations);
        }

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
            markdown.push_str(&self.element_to_markdown_with_context(
                element,
                file_path,
                with_full_relations,
            ));
        }

        // Add final separator after the last element (if there were any elements)
        if !elements.is_empty() {
            markdown.push_str("---\n\n");
        }

        markdown
    }

    /// Copies InternalPath files to the output directory
    fn copy_internal_path_files(
        &self,
        internal_paths: &HashSet<PathBuf>,
        output_dir: &Path,
    ) -> Result<usize, ReqvireError> {
        let base_dir = match git_commands::get_git_root_dir() {
            Ok(git_root) => git_root,
            Err(_) => {
                // If Git repository root can't be found, use the current working directory
                std::env::current_dir().map_err(|e| {
                    ReqvireError::PathError(format!("Failed to get current directory: {}", e))
                })?
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
                debug!(
                    "Skipping InternalPath file (same source and destination): {:?}",
                    src_path
                );
                continue;
            }

            // Create parent directories if needed
            if let Some(parent_dir) = dst_path.parent() {
                fs::create_dir_all(parent_dir).map_err(ReqvireError::IoError)?;
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
        let node = self.nodes.get(element_id).ok_or_else(|| {
            ReqvireError::MissingElement(format!("Element '{}' not found", element_id))
        })?;

        let file_path = node.element.file_path.clone();
        let _old_name = node.element.name.clone();

        // Generate new identifier (slug from new name - same logic as markdown heading to ID)
        let new_slug = new_name.trim().replace(' ', "-").to_lowercase();
        let new_identifier = format!("{}#{}", file_path, new_slug);

        // Check if new identifier already exists (globally unique check)
        if self.nodes.contains_key(&new_identifier) {
            return Err(ReqvireError::DuplicateElement(format!(
                "An element with name '{}' already exists (identifier: {})",
                new_name, new_identifier
            )));
        }

        // Find all files with relations to this element
        let mut modified_files = vec![file_path.clone()];
        for node in self.nodes.values() {
            let has_relation = node.element.relations.iter().any(
                |rel| matches!(&rel.target.link, LinkType::Identifier(id) if id == element_id),
            );

            if has_relation {
                let file = node.element.file_path.clone();
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Find all files with reused_contract_context pointing to this element
        for file in self.find_files_with_reused_contract_context_to(element_id) {
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

        // Update all reused_contract_context identifiers pointing to this element
        self.update_reused_contract_context_identifiers(&old_id, &new_identifier);

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
        let elements_in_source: Vec<String> = self
            .nodes
            .values()
            .filter(|node| node.element.file_path == source_file)
            .map(|node| node.element.identifier.clone())
            .collect();

        if elements_in_source.is_empty() {
            return Err(ReqvireError::LocationNotFound(format!(
                "Source file '{}' not found or contains no elements",
                source_file
            )));
        }

        // Validate target file doesn't exist (unless squash mode)
        let target_exists = self
            .nodes
            .values()
            .any(|node| node.element.file_path == target_file);

        if target_exists && !squash {
            return Err(ReqvireError::DuplicateElement(format!(
                "Target file '{}' already exists",
                target_file
            )));
        }

        // '# Element' files represent one implicit element.
        // Squashing multiple elements into such file would violate the format.
        if squash && target_exists && self.is_single_element_format_file(target_file) {
            return Err(ReqvireError::InvalidOperation(format!(
                "Cannot use --squash into '{}': target is a '# Element' file and can contain only one element.",
                target_file
            )));
        }

        // Track old -> new identifier mappings
        let mut identifier_mappings: Vec<(String, String)> = Vec::new();
        let mut modified_files = vec![source_file.to_string()];

        // In squash mode, move elements to target file
        if squash && target_exists {
            // Move each element to target file
            for old_id in &elements_in_source {
                let slug = if let Some(pos) = old_id.rfind('#') {
                    &old_id[pos + 1..]
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
                    &old_id[pos + 1..]
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

        // Find all files with reused_contract_context to elements in the source file
        for old_id in &elements_in_source {
            for file in self.find_files_with_reused_contract_context_to(old_id) {
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

        // Update all reused_contract_context identifiers pointing to moved elements
        for (old_id, new_id) in &identifier_mappings {
            self.update_reused_contract_context_identifiers(old_id, new_id);
        }

        modified_files.push(target_file.to_string());

        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok(identifier_mappings)
    }

    /// Flushes all elements to markdown files and copies InternalPath files to the specified directory
    /// When with_full_relations is true, includes all relations (user-created and auto-generated inverse relations)
    pub fn flush_to_directory(
        &self,
        output_dir: &Path,
        with_full_relations: bool,
    ) -> Result<(usize, usize), ReqvireError> {
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).map_err(ReqvireError::IoError)?;
        }

        // Generate and write markdown files
        let grouped_elements = self.group_elements_by_location();
        let mut markdown_files_written = 0;

        for (file_path, elements) in grouped_elements {
            // Generate the markdown content for this file
            let markdown_content =
                self.generate_file_markdown(&file_path, &elements, with_full_relations);

            // Determine the output file path
            let output_file_path = output_dir.join(&file_path);

            // Create parent directories if needed
            if let Some(parent_dir) = output_file_path.parent() {
                fs::create_dir_all(parent_dir).map_err(ReqvireError::IoError)?;
            }

            // Write the markdown file
            fs::write(&output_file_path, markdown_content).map_err(ReqvireError::IoError)?;

            debug!(
                "Flushed {} elements to {}",
                elements.len(),
                output_file_path.display()
            );

            markdown_files_written += 1;
        }

        // Copy InternalPath files
        let internal_paths = self.collect_internal_path_targets();
        let internal_files_copied = self.copy_internal_path_files(&internal_paths, output_dir)?;

        log::info!(
            "Successfully flushed {} markdown files and copied {} internal files to {}",
            markdown_files_written,
            internal_files_copied,
            output_dir.display()
        );

        Ok((markdown_files_written, internal_files_copied))
    }

    /// Flushes elements from specific files to markdown files and copies related InternalPath files
    /// When with_full_relations is true, includes all relations (user-created and auto-generated inverse relations)
    pub fn flush_files_to_directory(
        &self,
        file_paths: &[String],
        output_dir: &Path,
        with_full_relations: bool,
    ) -> Result<(usize, usize), ReqvireError> {
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).map_err(ReqvireError::IoError)?;
        }

        let grouped_elements = self.group_elements_by_location();
        let mut markdown_files_written = 0;
        let mut related_internal_paths = HashSet::new();

        for file_path in file_paths {
            if let Some(elements) = grouped_elements.get(file_path) {
                // Generate the markdown content for this file
                let markdown_content =
                    self.generate_file_markdown(file_path, elements, with_full_relations);

                // Determine the output file path
                let output_file_path = output_dir.join(file_path);

                // Create parent directories if needed
                if let Some(parent_dir) = output_file_path.parent() {
                    fs::create_dir_all(parent_dir).map_err(ReqvireError::IoError)?;
                }

                // Write the markdown file
                fs::write(&output_file_path, markdown_content).map_err(ReqvireError::IoError)?;

                // Collect InternalPath relations from elements in this file
                for element in elements {
                    for relation in &element.relations {
                        if let LinkType::InternalPath(ref path) = relation.target.link {
                            related_internal_paths.insert(path.clone());
                        }
                    }
                }

                debug!(
                    "Flushed {} elements to {}",
                    elements.len(),
                    output_file_path.display()
                );

                markdown_files_written += 1;
            }
        }

        // Copy related InternalPath files
        let internal_files_copied =
            self.copy_internal_path_files(&related_internal_paths, output_dir)?;

        log::info!(
            "Successfully flushed {} markdown files and copied {} internal files to {}",
            markdown_files_written,
            internal_files_copied,
            output_dir.display()
        );

        Ok((markdown_files_written, internal_files_copied))
    }

    // Dynamic graph manipulation methods

    /// Updates relation identifiers when elements move between files
    fn update_relation_identifiers(
        &mut self,
        moved_element_id: &str,
        _old_file_path: &str,
        new_file_path: &str,
    ) {
        // Extract just the fragment (element name) from the moved element's identifier
        let moved_fragment = moved_element_id
            .split('#')
            .next_back()
            .unwrap_or(moved_element_id);

        // 1. Update relations FROM other elements TO the moved element
        let source_node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        for source_id in source_node_ids {
            if source_id == moved_element_id {
                continue;
            }

            let source_file_path = self
                .nodes
                .get(&source_id)
                .map(|node| node.element.file_path.clone());
            let mut relations = self
                .nodes
                .get(&source_id)
                .map(|node| node.element.relations.clone())
                .unwrap_or_default();

            if relations.is_empty() {
                continue;
            }

            let Some(source_file_path) = source_file_path else {
                continue;
            };

            let mut changed = false;
            let canonical_target = format!("{}#{}", new_file_path, moved_fragment);
            for relation in &mut relations {
                if let crate::relation::LinkType::Identifier(ref mut target_id) =
                    relation.target.link
                {
                    if self.relation_targets_same_identifier(
                        &source_file_path,
                        target_id,
                        moved_element_id,
                    ) {
                        *target_id = canonical_target.clone();
                        relation.target.text = canonical_target.clone();
                        relation.target.element_id = Some(canonical_target.clone());
                        changed = true;
                    }
                }
            }

            if changed {
                if let Some(source_node) = self.nodes.get_mut(&source_id) {
                    source_node.element.relations = relations;
                }
            }
        }

        // 2. Update relations FROM the moved element TO other elements
        let moved_node_file = self
            .nodes
            .get(moved_element_id)
            .map(|node| node.element.file_path.clone());
        if let (Some(moved_node_file), Some(mut relations)) = (
            moved_node_file,
            self.nodes
                .get(moved_element_id)
                .map(|node| node.element.relations.clone()),
        ) {
            let mut changed = false;
            for relation in &mut relations {
                if let crate::relation::LinkType::Identifier(ref mut target_id) =
                    relation.target.link
                {
                    if let Some(resolved_target) =
                        self.normalize_relation_identifier_for_source(&moved_node_file, target_id)
                    {
                        if let Some(target_node) = self.nodes.get(&resolved_target) {
                            let target_file_path = target_node.element.file_path.clone();
                            let target_fragment =
                                crate::utils::extract_path_and_fragment(&resolved_target)
                                    .1
                                    .unwrap_or(&resolved_target);
                            let canonical_target =
                                format!("{}#{}", target_file_path, target_fragment);
                            *target_id = canonical_target.clone();
                            relation.target.text = canonical_target;
                            relation.target.element_id = Some(resolved_target);
                            changed = true;
                        }
                    }
                }
            }

            if changed {
                if let Some(moved_node) = self.nodes.get_mut(moved_element_id) {
                    moved_node.element.relations = relations;
                }
            }
        }
    }

    /// Updates reused_contract_context identifiers when a Contract element is moved or renamed
    /// Similar to update_relation_identifiers but for reused_contract_context references
    fn update_reused_contract_context_identifiers(
        &mut self,
        old_identifier: &str,
        new_identifier: &str,
    ) {
        // Find and update all reused_contract_context identifiers pointing to the old identifier
        for node in self.nodes.values_mut() {
            for reused_contract_context in &mut node.element.reused_contract_context {
                if let crate::element::ReusedContractContextTarget::ElementIdentifier(ref mut id) =
                    reused_contract_context.target
                {
                    if id == old_identifier {
                        *id = new_identifier.to_string();
                    }
                }
            }
        }
    }

    /// Finds all files that have reused_contract_context pointing to the given element identifier
    fn find_files_with_reused_contract_context_to(&self, element_id: &str) -> Vec<String> {
        let mut files = Vec::new();
        for node in self.nodes.values() {
            let has_reused_contract_context = node.element.reused_contract_context.iter().any(|att| {
                matches!(&att.target, crate::element::ReusedContractContextTarget::ElementIdentifier(id) if id == element_id)
            });
            if has_reused_contract_context {
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
            return Err(ReqvireError::ElementMoveError(format!(
                "Element '{}' already exists in the graph",
                element_id
            )));
        }

        self.nodes.insert(
            element_id,
            ElementNode {
                element,
                relations: Vec::new(),
            },
        );

        Ok(())
    }

    /// Removes an element from the graph and all relations pointing to it
    pub fn remove_element(&mut self, element_id: &str) -> Result<(), ReqvireError> {
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Element '{}' not found in the graph",
                element_id
            )));
        }

        // Remove the element itself
        self.nodes.remove(element_id);

        // Remove all relations pointing to this element from graph structure
        for node in self.nodes.values_mut() {
            node.relations
                .retain(|rel| rel.element_node.element.identifier != element_id);
        }

        // Remove all relations pointing to this element from element's own relations list
        let mut node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        for node_id in node_ids.drain(..) {
            let source_file_path = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.file_path.clone());
            let mut relations = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.relations.clone())
                .unwrap_or_default();

            let Some(source_file_path) = source_file_path else {
                continue;
            };

            let mut filtered = Vec::new();
            for relation in relations.drain(..) {
                let keep = match &relation.target.link {
                    crate::relation::LinkType::Identifier(target) => !self
                        .relation_targets_same_identifier(&source_file_path, target, element_id),
                    _ => true,
                };
                if keep {
                    filtered.push(relation);
                }
            }
            if let Some(mut_node) = self.nodes.get_mut(&node_id) {
                mut_node.element.relations = filtered;
            }
        }

        Ok(())
    }

    /// Adds a relation between two elements in the graph
    pub fn add_relation(
        &mut self,
        source_id: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<(), ReqvireError> {
        // Validate both elements exist
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Source element '{}' not found",
                source_id
            )));
        }
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Target element '{}' not found",
                target_id
            )));
        }

        // Check if relation type is valid for impact propagation
        if !relation::IMPACT_PROPAGATION_RELATIONS.contains(&relation_type) {
            return Err(ReqvireError::ProcessError(format!(
                "Relation type '{}' is not valid for impact propagation",
                relation_type
            )));
        }

        // Get the target node to create the relation
        let target_node = self.nodes.get(target_id).unwrap().clone();

        // Add the relation to the source element
        let source_node = self.nodes.get_mut(source_id).unwrap();

        // Check if relation already exists
        let relation_exists = source_node.relations.iter().any(|rel| {
            rel.element_node.element.identifier == target_id
                && rel.relation_trigger == relation_type
        });

        if relation_exists {
            return Err(ReqvireError::ProcessError(format!(
                "Relation '{}' from '{}' to '{}' already exists",
                relation_type, source_id, target_id
            )));
        }

        source_node.relations.push(RelationNode {
            relation_trigger: relation_type.to_string(),
            element_node: target_node,
        });

        Ok(())
    }

    /// Removes a specific relation between two elements (graph structure only)
    pub fn remove_relation(
        &mut self,
        source_id: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<(), ReqvireError> {
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Source element '{}' not found",
                source_id
            )));
        }

        let source_node = self.nodes.get_mut(source_id).unwrap();
        let initial_count = source_node.relations.len();

        source_node.relations.retain(|rel| {
            !(rel.element_node.element.identifier == target_id
                && rel.relation_trigger == relation_type)
        });

        if source_node.relations.len() == initial_count {
            return Err(ReqvireError::ProcessError(format!(
                "Relation '{}' from '{}' to '{}' not found",
                relation_type, source_id, target_id
            )));
        }

        Ok(())
    }

    /// Removes a relation from an element's relations array with bidirectional handling
    /// This removes the relation from element.relations (which gets written to markdown)
    /// and also removes the opposite relation if one exists
    pub fn remove_element_relation(
        &mut self,
        element_id: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<(), ReqvireError> {
        // Check if source element exists
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Element '{}' not found",
                element_id
            )));
        }

        // Check if target element exists
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Target element '{}' not found",
                target_id
            )));
        }

        // Remove the relation from source element's relations array
        let source_node = self.nodes.get_mut(element_id).unwrap();
        let initial_count = source_node.element.relations.len();

        source_node.element.relations.retain(|rel| {
            !(rel.relation_type.name == relation_type &&
              matches!(&rel.target.link, crate::relation::LinkType::Identifier(id) if id == target_id))
        });

        if source_node.element.relations.len() == initial_count {
            return Err(ReqvireError::ProcessError(format!(
                "Relation '{}' from '{}' to '{}' not found",
                relation_type, element_id, target_id
            )));
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

    /// Remove an reused_contract_context from an element
    pub fn remove_element_reused_contract_context(
        &mut self,
        element_id: &str,
        reused_contract_context: &str,
    ) -> Result<(), ReqvireError> {
        if let Some(node) = self.nodes.get_mut(element_id) {
            let original_len = node.element.reused_contract_context.len();
            node.element
                .reused_contract_context
                .retain(|a| a.target.as_str() != reused_contract_context);

            if node.element.reused_contract_context.len() < original_len {
                self.modified_files.insert(node.element.file_path.clone());
                Ok(())
            } else {
                Err(ReqvireError::ProcessError(format!(
                    "ReusedContractContextEntry '{}' not found on element '{}'",
                    reused_contract_context, element_id
                )))
            }
        } else {
            Err(ReqvireError::ProcessError(format!(
                "Element '{}' not found",
                element_id
            )))
        }
    }

    /// Lists all relations for a given element
    pub fn list_relations(&self, element_id: &str) -> Result<Vec<(String, String)>, ReqvireError> {
        let node = self.nodes.get(element_id).ok_or_else(|| {
            ReqvireError::LocationNotFound(format!("Element '{}' not found", element_id))
        })?;

        let relations = node
            .relations
            .iter()
            .map(|rel| {
                (
                    rel.relation_trigger.clone(),
                    rel.element_node.element.identifier.clone(),
                )
            })
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
        use crate::relation::{
            get_relation_element_type_description, validate_relation_element_types, LinkType,
            Relation, RelationTarget, LEGACY_CONTRACT_RELATIONS, RELATION_TYPES,
        };
        use std::path::PathBuf;

        // Validate source element exists
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::ElementNotFound(format!(
                "Source element '{}' not found",
                source_id
            )));
        }

        // Validate relation type
        if LEGACY_CONTRACT_RELATIONS.contains(&relation_type) {
            let replacement = if relation_type == "refinedBy" {
                "definedBy"
            } else {
                "define"
            };
            return Err(ReqvireError::UnsupportedRelationType(format!(
                "Legacy relation type '{}'. Use '{}' for requirement-owned contract elements, or run `reqvire migrate` on existing sources.",
                relation_type, replacement
            )));
        }

        if !RELATION_TYPES.contains_key(relation_type) {
            return Err(ReqvireError::UnsupportedRelationType(format!(
                "Invalid relation type '{}'. Valid types: {}",
                relation_type,
                crate::relation::supported_relation_types_list()
            )));
        }

        // Get source element info
        let source_node = self.nodes.get(source_id).unwrap();
        let source_name = source_node.element.name.clone();
        let source_file_path = source_node.element.file_path.clone();
        let source_type = source_node.element.element_type.clone();

        // Determine target type: element name, external URL, or internal path
        let is_external_url = crate::utils::is_external_url(target);
        let is_internal_path = !is_external_url
            && (target.ends_with(".md") || target.contains('/') || git_root.join(target).exists());

        // Resolve target and create relation components
        let (target_display_name, relation_target_link, target_id_for_check, element_id_opt) =
            if is_external_url {
                // External URL - use as-is
                (
                    target.to_string(),
                    LinkType::ExternalUrl(target.to_string()),
                    target.to_string(),
                    None,
                )
            } else if is_internal_path {
                // Internal file path
                let source_folder = crate::utils::get_parent_dir(&source_file_path);
                let target_type = crate::element::ElementType::File;

                if !validate_relation_element_types(relation_type, &source_type, &target_type) {
                    let description = get_relation_element_type_description(relation_type)
                        .unwrap_or_else(|| {
                            format!(
                                "Relation '{}' is not compatible with source type '{}' and internal file targets",
                                relation_type,
                                source_type.as_str()
                            )
                        });
                    return Err(ReqvireError::IncompatibleElementTypes(format!(
                        "Relation '{}' from '{}' ({}) to '{}' (file) has incompatible element types. {}",
                        relation_type,
                        source_name,
                        source_type.as_str(),
                        target,
                        description
                    )));
                }

                // Calculate relative path from source file to target
                let target_path = PathBuf::from(target);
                let relative_path = pathdiff::diff_paths(&target_path, &source_folder)
                    .unwrap_or_else(|| target_path.clone());

                // Extract filename for display name
                let display = target_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| target.to_string());

                (
                    display,
                    LinkType::InternalPath(relative_path),
                    target.to_string(),
                    None,
                )
            } else {
                // Element name - resolve to get identifier
                let target_element = self.get_element_by_name(target).ok_or_else(|| {
                    ReqvireError::ElementNotFound(format!("Target element '{}' not found", target))
                })?;

                let target_id = target_element.identifier.clone();
                let target_display_name = target_element.name.clone();
                let target_type = target_element.element_type.clone();

                if source_node.element.reused_contract_context.iter().any(|a| {
                    let reused_contract_context_target = a.target.as_str();
                    reused_contract_context_target == target_id
                        || self
                            .resolve_relation_identifier(
                                &source_node.element,
                                &reused_contract_context_target,
                            )
                            .is_some_and(|resolved| resolved == target_id)
                        || self.relation_targets_same_identifier(
                            &source_file_path,
                            &reused_contract_context_target,
                            &target_id,
                        )
                }) {
                    return Err(ReqvireError::CrossSectionDuplicate(format!(
                        "Target '{}' already exists in Reused Contract Context of '{}'. Cannot add to Relations.",
                        target, source_name
                    )));
                }

                if !validate_relation_element_types(relation_type, &source_type, &target_type) {
                    let description = get_relation_element_type_description(relation_type)
                        .unwrap_or_else(|| {
                            format!(
                                "Relation '{}' is not compatible with source type '{}' and target type '{}'",
                                relation_type,
                                source_type.as_str(),
                                target_type.as_str()
                            )
                        });
                    return Err(ReqvireError::IncompatibleElementTypes(format!(
                        "Relation '{}' from '{}' ({}) to '{}' ({}) has incompatible element types. {}",
                        relation_type,
                        source_name,
                        source_type.as_str(),
                        target_display_name,
                        target_type.as_str(),
                        description
                    )));
                }

                let relation_target = LinkType::Identifier(target_id.clone());

                // Extract element ID (fragment) for change tracking
                let (_path, fragment_opt) = crate::utils::extract_path_and_fragment(&target_id);
                let element_id = fragment_opt.map(|s| s.to_string());

                (target_display_name, relation_target, target_id, element_id)
            };

        // Get source node again (mutable this time)
        let source_node = self.nodes.get(source_id).unwrap();

        // Validate: Check if relation already exists (idempotent)
        let relation_exists = source_node.element.relations.iter().any(|r| {
            r.user_created
                && r.relation_type.name == relation_type
                && r.target.link.as_str() == target_id_for_check
        });

        if relation_exists {
            return Err(ReqvireError::RelationError(format!(
                "Relation '{}' from '{}' to '{}' already exists",
                relation_type, source_name, target
            )));
        }

        // Validate: Check for cross-section duplicate (target in Reused Contract Context)
        let in_reused_contract_context = source_node
            .element
            .reused_contract_context
            .iter()
            .any(|a| a.target.as_str() == target_id_for_check);

        if in_reused_contract_context {
            return Err(ReqvireError::CrossSectionDuplicate(format!(
                "Target '{}' already exists in Reused Contract Context of '{}'. Cannot add to Relations.",
                target, source_name
            )));
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
        self.add_opposite_to_target(&relation, source_id, &source_name, &source_element_id);

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
            return Err(ReqvireError::ElementNotFound(format!(
                "Source element '{}' not found",
                source_id
            )));
        }

        let source_node = self.nodes.get(source_id).unwrap();
        let source_file_path = source_node.element.file_path.clone();

        // Try to resolve target as element name first
        let target_id_to_find = if let Some(target_element) = self.get_element_by_name(target) {
            target_element.identifier.clone()
        } else {
            let normalized_target =
                crate::utils::normalize_relation_identifier_for_registry(&source_file_path, target);

            if self.nodes.contains_key(&normalized_target) {
                normalized_target
            } else {
                target.to_string()
            }
        };

        // Find matching relation (check both user_created and auto-generated)
        // This allows unlinking from either side of a bidirectional relation
        let relation_match = source_node
            .element
            .relations
            .iter()
            .find(|r| r.target.link.as_str() == target_id_to_find)
            .cloned(); // Clone to avoid borrow issues

        if let Some(relation) = relation_match {
            let relation_type = relation.relation_type.name.to_string();
            let target_display_name = relation.target.text.clone();
            let relation_type_info = crate::relation::RELATION_TYPES
                .get(relation_type.as_str())
                .unwrap();
            let source_relation_was_user_created = relation.user_created;

            // Remove the relation (both user_created and auto-generated)
            let source_node = self.nodes.get_mut(source_id).unwrap();
            source_node.element.relations.retain(|r| {
                !(r.relation_type.name == relation_type
                    && r.target.link.as_str() == target_id_to_find)
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
            // No relation found - could be an reused_contract_context (handled by crud layer)
            Ok(None)
        }
    }

    /// Gets statistics about the graph
    pub fn get_graph_stats(&self) -> (usize, usize) {
        let element_count = self.nodes.len();
        let relation_count = self.nodes.values().map(|node| node.relations.len()).sum();

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
                validation
                    .error_message
                    .unwrap_or_else(|| "Invalid target path".to_string()),
            ));
        }

        // Parse element from markdown string
        let element = crate::parser::parse_single_element(markdown, target_file)?;

        // Check for duplicate element name (global uniqueness)
        if self.nodes.contains_key(&element.identifier) {
            return Err(ReqvireError::DuplicateElement(format!(
                "Element '{}' already exists in the model",
                element.name
            )));
        }

        // Validate that all relation targets exist in the model
        // External links (http://, https://, etc.) are allowed and not validated
        for relation in &element.relations {
            if let crate::relation::LinkType::Identifier(target_id) = &relation.target.link {
                // Check if this is an external link using the predefined list
                let is_external = crate::utils::EXTERNAL_SCHEMES
                    .iter()
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
        let max_index = self
            .nodes
            .values()
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
        let relations_to_process: Vec<_> = self
            .nodes
            .get(&new_element_id)
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

        for child_node in self.nodes.values() {
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
    pub fn remove_element_with_cleanup(
        &mut self,
        element_id: &str,
    ) -> Result<Vec<String>, ReqvireError> {
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Element '{}' not found in the graph",
                element_id
            )));
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
                let source_file_path = node.element.file_path.clone();
                let has_relation_to_target = node.element.relations.iter().any(|rel| {
                    matches!(
                        &rel.target.link,
                        LinkType::Identifier(target_id)
                            if self.relation_targets_same_identifier(
                                &source_file_path,
                                target_id,
                                element_id,
                            )
                    )
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
        !self
            .nodes
            .values()
            .any(|node| node.element.file_path == file_path)
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
            return Err(ReqvireError::LocationNotFound(format!(
                "Element '{}' not found",
                element_id
            )));
        }

        // Get source file before move
        let source_file = self
            .nodes
            .get(element_id)
            .unwrap()
            .element
            .file_path
            .clone();

        // Validate target path
        let validation = crate::utils::validate_target_path(target_file, None, excluded_patterns)?;

        if !validation.is_valid {
            return Err(ReqvireError::InvalidPath(
                validation
                    .error_message
                    .unwrap_or_else(|| "Invalid target path".to_string()),
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
            let source_file_path = node.element.file_path.clone();
            let has_relation = node.element.relations.iter().any(|rel| {
                matches!(
                    &rel.target.link,
                    LinkType::Identifier(target_id)
                        if self.relation_targets_same_identifier(
                            &source_file_path,
                            target_id,
                            &old_identifier,
                        )
                )
            });

            if has_relation {
                let file = node.element.file_path.clone();
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Find all files with reused_contract_context pointing to this element
        for file in self.find_files_with_reused_contract_context_to(element_id) {
            if !modified_files.contains(&file) {
                modified_files.push(file);
            }
        }

        // Now perform the move
        self.move_element_to_location(element_id, target_file)?;

        // Update all relations (TO and FROM the moved element)
        self.update_relation_identifiers(&old_identifier, &source_file, target_file);

        // Construct the new identifier (file path changed, fragment stays the same)
        let fragment = old_identifier.split('#').next_back().unwrap_or("");
        let new_identifier = format!("{}#{}", target_file, fragment);

        // Re-key the node in the HashMap: remove with old key, update identifier, insert with new key
        if let Some(mut node) = self.nodes.remove(&old_identifier) {
            node.element.identifier = new_identifier.clone();
            self.nodes.insert(new_identifier.clone(), node);
        }

        // Update all reused_contract_context identifiers pointing to this element
        self.update_reused_contract_context_identifiers(&old_identifier, &new_identifier);

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
    /// - Relations and reused_contract_context are merged with deduplication
    /// - Source elements are deleted after successful merge
    /// - Relations pointing to source elements are redirected to target
    pub fn merge_elements(
        &mut self,
        target_id: &str,
        source_ids: &[String],
    ) -> Result<(), ReqvireError> {
        // Validate target exists
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::ElementNotFound(format!(
                "Target element '{}' not found",
                target_id
            )));
        }

        // Get target element data first (needed for validation)
        let target_node = self.nodes.get(target_id).unwrap();
        let target_name = target_node.element.name.clone();
        let target_type = target_node.element.element_type.clone();
        let target_file_path = target_node.element.file_path.clone();
        let target_is_single_element = self.is_single_element_format_file(&target_file_path);

        // Validate all sources exist and collect their data
        #[allow(clippy::type_complexity)]
        let mut source_data: Vec<(
            String,
            String,
            String,
            Vec<crate::relation::Relation>,
            Vec<crate::element::ReusedContractContextEntry>,
            Element,
        )> = Vec::new();
        for source_id in source_ids {
            let source_node = self.nodes.get(source_id).ok_or_else(|| {
                ReqvireError::ElementNotFound(format!("Source element '{}' not found", source_id))
            })?;

            let source_element = &source_node.element;
            let source_file_path = source_element.file_path.clone();
            let source_is_single_element = self.is_single_element_format_file(&source_file_path);

            // Validate: Check if source would merge into itself
            if source_id == target_id {
                return Err(ReqvireError::InvalidOperation(
                    "Cannot merge element into itself".to_string(),
                ));
            }

            // Merging single-element source content into # Elements target is disallowed.
            // '# Element' bodies permit headers that violate # Elements parsing constraints.
            if source_is_single_element && !target_is_single_element {
                return Err(ReqvireError::InvalidOperation(format!(
                    "Cannot merge '{}' into '{}': source is in a '# Element' file and target is in a '# Elements' file. This conversion can break '# Elements' parsing rules and must be performed manually.",
                    source_element.name, target_name
                )));
            }

            // Validate: Check type compatibility
            if !target_type.is_merge_compatible(&source_element.element_type) {
                return Err(ReqvireError::MergeTypeMismatch(format!(
                    "Cannot merge '{}' ({}) into '{}' ({}): type mismatch. \
                     Elements must be in the same category (requirement/verification/contract/other).",
                    source_element.name, source_element.element_type.as_str(),
                    target_name, target_type.as_str()
                )));
            }

            source_data.push((
                source_id.clone(),
                source_element.name.clone(),
                source_element.content.clone(),
                source_element
                    .relations
                    .iter()
                    .filter(|r| r.user_created)
                    .cloned()
                    .collect(),
                source_element.reused_contract_context.clone(),
                source_element.clone(),
            ));
        }

        // Re-get target element data (needed after validation)
        let target_node = self.nodes.get(target_id).unwrap();
        let mut merged_content = String::new();
        let mut merged_relations: Vec<crate::relation::Relation> = target_node
            .element
            .relations
            .iter()
            .filter(|r| r.user_created)
            .cloned()
            .collect();
        let mut merged_reused_contract_context: Vec<crate::element::ReusedContractContextEntry> =
            target_node.element.reused_contract_context.clone();
        let target_is_ontology = target_type.is_ontology();
        let target_element_for_merge = target_node.element.clone();
        let mut merged_source_ids: HashSet<String> = source_ids.iter().cloned().collect();
        merged_source_ids.insert(target_id.to_string());

        // Process each source element
        for (
            source_id,
            source_name,
            source_content,
            source_relations,
            source_reused_contract_context,
            source_element,
        ) in &source_data
        {
            // Extract main content and details from source
            let (main_content, details_content) =
                if target_is_ontology && source_element.element_type.is_ontology() {
                    (extract_leading_prose(source_content), String::new())
                } else {
                    extract_content_parts(source_content)
                };

            // Add main content to merged content (will go into target's Details)
            if !main_content.trim().is_empty() {
                merged_content.push_str(&format!("\n{}\n", main_content.trim()));
            }

            // Add details to "Merged Details (element name)" subsection
            if !details_content.trim().is_empty() {
                merged_content.push_str(&format!(
                    "\n#### Merged Details ({})\n{}\n",
                    source_name,
                    details_content.trim()
                ));
            }

            // Collect relations
            for rel in source_relations {
                let skip_relation = match &rel.target.link {
                    LinkType::Identifier(relation_target) => {
                        self.resolve_relation_identifier(source_element, relation_target)
                            .is_some_and(|resolved| merged_source_ids.contains(&resolved))
                            || merged_source_ids.contains(relation_target)
                    }
                    _ => false,
                };
                if !skip_relation {
                    merged_relations.push(rel.clone());
                }
            }

            // Collect reused_contract_context
            for att in source_reused_contract_context {
                merged_reused_contract_context.push(att.clone());
            }

            // Track source file as modified
            let source_file = self.nodes.get(source_id).unwrap().element.file_path.clone();
            self.modified_files.insert(source_file);
        }

        // Deduplicate relations by (relation_type, target)
        let mut seen_relations: HashSet<(String, String)> = HashSet::new();
        merged_relations.retain(|r| {
            let key = (
                r.relation_type.name.to_string(),
                r.target.link.as_str().to_string(),
            );
            if seen_relations.contains(&key) {
                false
            } else {
                seen_relations.insert(key);
                true
            }
        });

        // Deduplicate reused_contract_context by target
        let mut seen_reused_contract_context: HashSet<String> = HashSet::new();
        merged_reused_contract_context.retain(|a| {
            let key = a.target.as_str().to_string();
            if seen_reused_contract_context.contains(&key) {
                false
            } else {
                seen_reused_contract_context.insert(key);
                true
            }
        });

        // Validate reused_contract_context scope constraints for target element
        for reused_contract_context in &merged_reused_contract_context {
            if let crate::element::ReusedContractContextTarget::ElementIdentifier(ref att_id) =
                reused_contract_context.target
            {
                // Check orphan contract constraint
                if !self.contract_has_define_relation(att_id) {
                    let att_name = self
                        .nodes
                        .get(att_id)
                        .map(|n| n.element.name.as_str())
                        .unwrap_or(att_id);
                    return Err(ReqvireError::InvalidReusedContractContextTarget(
                        format!(
                            "'{}' has no define relation. Contracts must define a requirement before they can be reused; contracts are requirement-owned only. Capabilities use concept references for ontology terms and are specified/verified, not defined by implementation-detail contracts.",
                            att_name
                        ),
                    ));
                }

                // Check hierarchical independence constraint
                let defining_reqs = self.get_defining_requirements(att_id);
                for defining_req_id in defining_reqs {
                    if self.is_in_hierarchy(target_id, &defining_req_id) {
                        let att_name = self
                            .nodes
                            .get(att_id)
                            .map(|n| n.element.name.as_str())
                            .unwrap_or(att_id);
                        return Err(ReqvireError::InvalidReusedContractContextScope(
                            format!(
                                "'{}' cannot be reused to '{}' because it is within the contract's defining hierarchy. Reused Contract Context are only allowed from elements outside the definedBy chain.",
                                att_name,
                                target_name
                            ),
                        ));
                    }
                }

                if let Some(msg) = self.build_reused_contract_context_direction_scope_error(
                    att_id,
                    target_id,
                    &target_name,
                    None,
                ) {
                    return Err(ReqvireError::InvalidReusedContractContextScope(msg));
                }
            }
        }

        // Check for cross-section duplicates
        let relation_targets: HashSet<String> = merged_relations
            .iter()
            .map(|r| r.target.link.as_str().to_string())
            .collect();

        for reused_contract_context in &merged_reused_contract_context {
            let target = reused_contract_context.target.as_str();
            if relation_targets.contains(&target) {
                return Err(ReqvireError::MergeCrossSectionDuplicate(format!(
                    "Target '{}' would appear in both Relations and Reused Contract Context after merge. Remove one before merging.",
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
                target_element.content =
                    merge_content_into_details(&target_element.content, &merged_content);
            }

            if target_is_ontology {
                let merged_ontology_block = merge_ontology_blocks_into_target(
                    &target_element.content,
                    &target_element_for_merge,
                    &source_data
                        .iter()
                        .map(|(_, _, _, _, _, element)| element.clone())
                        .collect::<Vec<_>>(),
                )?;
                target_element.content = replace_single_fenced_subsection(
                    &target_element.content,
                    "Ontology",
                    &merged_ontology_block,
                )?;
            }

            target_element.relations = merged_relations;
            target_element.reused_contract_context = merged_reused_contract_context;
        }

        self.modified_files.insert(target_file_path);

        // CRITICAL: Before removing sources, handle opposite relations
        // Find all elements with relations TO sources and recreate their opposites to point to target
        let target_node = self.nodes.get(target_id).unwrap();
        let target_name = target_node.element.name.clone();
        let target_element_id = target_node.element.id.clone();

        for (source_id, _, _, _, _, _) in &source_data {
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
                        self.remove_opposite_from_target(
                            source_id,
                            &referrer_id,
                            opposite_type_name,
                        );

                        // Create new opposite pointing to target (will be added to target after merge)
                        // Note: The referrer's relation will be redirected by redirect_relations_to_target(),
                        // so we create opposite on target now
                        self.add_opposite_to_target(
                            &relation,
                            target_id,
                            &target_name,
                            &target_element_id,
                        );
                    }
                }
            }
        }

        // Redirect relations from other elements pointing to sources to point to target
        for (source_id, _, _, _, _, _) in &source_data {
            self.redirect_relations_to_target(source_id, target_id)?;
        }

        // Remove source elements (this also removes them from the graph)
        for (source_id, _, _, _, _, _) in &source_data {
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
        let nodes_to_update: Vec<String> = self
            .nodes
            .iter()
            .filter(|(id, node)| {
                *id != source_id
                    && *id != target_id
                    && node.element.relations.iter().any(|r| match &r.target.link {
                        LinkType::Identifier(ref id) => self.relation_targets_same_identifier(
                            &node.element.file_path,
                            id,
                            source_id,
                        ),
                        _ => false,
                    })
            })
            .map(|(id, _)| id.clone())
            .collect();

        for node_id in nodes_to_update {
            let mut relations = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.relations.clone())
                .unwrap_or_default();
            let file_path = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.file_path.clone())
                .unwrap_or_default();
            let mut changed = false;
            for relation in &mut relations {
                if let LinkType::Identifier(ref id) = relation.target.link {
                    if self.relation_targets_same_identifier(&file_path, id, source_id) {
                        relation.target = crate::relation::RelationTarget {
                            text: target_id.to_string(),
                            link: LinkType::Identifier(target_id.to_string()),
                            element_id: Some(target_id.to_string()),
                        };
                        changed = true;
                    }
                }
            }
            if changed {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.element.relations = relations;
                    self.modified_files.insert(file_path);
                }
            }
        }

        // CRITICAL: Also redirect opposite relations (auto-generated, user_created=false)
        // Get target element info for creating correct opposite targets
        let (target_name, target_element_id) = if let Some(target_node) = self.nodes.get(target_id)
        {
            (
                target_node.element.name.clone(),
                target_node.element.id.clone(),
            )
        } else {
            return Ok(()); // Target doesn't exist
        };

        // Find and update opposite relations pointing to source
        for node_id in self.nodes.keys().cloned().collect::<Vec<_>>() {
            if node_id == source_id || node_id == target_id {
                continue;
            }

            let source_file_path = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.file_path.clone());
            let mut relations = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.relations.clone())
                .unwrap_or_default();

            let Some(source_file_path) = source_file_path else {
                continue;
            };

            let mut changed = false;

            // Update auto-generated opposite relations pointing to source
            for relation in &mut relations {
                if !relation.user_created {
                    // Only auto-generated opposites
                    if let LinkType::Identifier(ref id) = relation.target.link {
                        if self.relation_targets_same_identifier(&source_file_path, id, source_id) {
                            relation.target = crate::relation::RelationTarget {
                                text: target_name.clone(),
                                link: LinkType::Identifier(target_id.to_string()),
                                element_id: Some(target_element_id.clone()),
                            };
                            // Note: Do NOT mark file as modified - opposite is user_created=false
                            changed = true;
                        }
                    }
                }
            }
            if changed {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.element.relations = relations;
                }
            }
        }

        Ok(())
    }

    /// Flushes only modified files to directory (optimization)
    pub fn flush_modified_files(&mut self, directory: &Path) -> Result<(), ReqvireError> {
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
                    fs::remove_file(&file_full_path).map_err(ReqvireError::IoError)?;
                    log::info!("Deleted empty file: {}", file_path);
                }
            }
        }

        self.modified_files.clear();
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
        let details_end = rest.find("\n#### ").unwrap_or(rest.len());

        (main, rest[..details_end].to_string())
    } else {
        (content.to_string(), String::new())
    }
}

fn extract_leading_prose(content: &str) -> String {
    let mut lines = Vec::new();
    for line in content.lines() {
        if line.trim_start().starts_with("#### ") {
            break;
        }
        lines.push(line);
    }
    lines.join("\n")
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
        let details_end = rest
            .find("\n#### ")
            .map(|p| after_marker + p)
            .unwrap_or(target_content.len());

        // Insert additional content at end of Details
        let mut result = target_content[..details_end].to_string();
        result.push_str(additional);
        result.push_str(&target_content[details_end..]);
        result
    } else {
        // No Details section - create one
        format!(
            "{}\n#### Details\n{}",
            target_content.trim_end(),
            additional
        )
    }
}

fn merge_ontology_blocks_into_target(
    target_content: &str,
    target_element: &Element,
    source_elements: &[Element],
) -> Result<String, ReqvireError> {
    let mut merged_blocks = Vec::new();

    let target_block = extract_single_fenced_subsection(target_content, "Ontology")
        .into_iter()
        .next()
        .ok_or_else(|| {
            ReqvireError::InvalidOperation(format!(
                "Ontology element '{}' must contain exactly one #### Ontology fenced Turtle block.",
                target_element.name
            ))
        })?;
    merged_blocks.push(target_block.content.trim_end().to_string());

    for source_element in source_elements {
        let Some(source_block) =
            extract_single_fenced_subsection(&source_element.content, "Ontology")
                .into_iter()
                .next()
        else {
            continue;
        };
        let rewritten =
            rewrite_ontology_block_for_merge(&source_block.content, source_element, target_element);
        if !rewritten.trim().is_empty() {
            merged_blocks.push(rewritten);
        }
    }

    Ok(dedupe_turtle_block(merged_blocks.join("\n\n").trim_end()))
}

fn rewrite_ontology_block_for_merge(
    block: &str,
    source_element: &Element,
    target_element: &Element,
) -> String {
    let source_base = source_element
        .metadata
        .get("ontology_base")
        .cloned()
        .unwrap_or_default();
    let source_prefix = source_element
        .metadata
        .get("ontology_prefix")
        .cloned()
        .unwrap_or_default();
    let target_base = target_element
        .metadata
        .get("ontology_base")
        .cloned()
        .unwrap_or_default();
    let target_prefix = target_element
        .metadata
        .get("ontology_prefix")
        .cloned()
        .unwrap_or_default();

    let mut rewritten_lines = Vec::new();
    let source_namespace = if source_base.is_empty() {
        String::new()
    } else {
        format!("{}#", source_base)
    };

    for line in block.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("@prefix {}: <", source_prefix))
            && trimmed.contains(&format!("<{}> .", source_namespace))
        {
            continue;
        }
        if trimmed.starts_with("@prefix owl: ") || trimmed.starts_with("prefix owl: ") {
            continue;
        }
        if trimmed.contains("owl:Ontology")
            || trimmed.contains("owl#imports")
            || trimmed.contains("owl:imports")
        {
            continue;
        }
        rewritten_lines.push(line.to_string());
    }

    let mut rewritten = rewritten_lines.join("\n");
    if !source_base.is_empty() && !target_base.is_empty() {
        rewritten = rewritten.replace(&source_base, &target_base);
    }
    if !source_prefix.is_empty() && !target_prefix.is_empty() && source_prefix != target_prefix {
        rewritten = replace_prefix_token(&rewritten, &source_prefix, &target_prefix);
    }

    rewritten.trim().to_string()
}

fn dedupe_turtle_block(block: &str) -> String {
    let mut seen_prefix_lines = HashSet::new();
    let mut seen_exact_lines = HashSet::new();
    let mut output = Vec::new();

    for line in block.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if output
                .last()
                .map(|prev: &String| prev.trim().is_empty())
                .unwrap_or(false)
            {
                continue;
            }
            output.push(String::new());
            continue;
        }

        if trimmed.starts_with("@prefix ") || trimmed.starts_with("prefix ") {
            if !seen_prefix_lines.insert(trimmed.to_string()) {
                continue;
            }
        } else if !seen_exact_lines.insert(trimmed.to_string()) {
            continue;
        }

        output.push(line.to_string());
    }

    output.join("\n").trim_end().to_string()
}

fn replace_single_fenced_subsection(
    content: &str,
    subsection: &str,
    replacement: &str,
) -> Result<String, ReqvireError> {
    let header = format!("#### {}", subsection);
    let mut output = String::new();
    let mut in_target_section = false;
    let mut saw_fence = false;
    let mut skipping_block = false;
    let mut replaced = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if !replaced && trimmed == header {
            in_target_section = true;
            output.push_str(line);
            output.push('\n');
            continue;
        }

        if in_target_section && !saw_fence {
            output.push_str(line);
            output.push('\n');
            if trimmed.starts_with("```") {
                saw_fence = true;
                if !replacement.trim().is_empty() {
                    output.push_str(replacement.trim_end());
                    output.push('\n');
                }
                skipping_block = true;
            }
            continue;
        }

        if skipping_block {
            if trimmed.starts_with("```") {
                output.push_str(line);
                output.push('\n');
                skipping_block = false;
                in_target_section = false;
                replaced = true;
            }
            continue;
        }

        output.push_str(line);
        output.push('\n');
    }

    if !replaced {
        return Err(ReqvireError::InvalidOperation(format!(
            "Expected to replace #### {} fenced Turtle block but the section was not found.",
            subsection
        )));
    }

    Ok(output.trim_end_matches('\n').to_string())
}

struct UsedOntologyProjectNamespaces {
    by_prefix: BTreeMap<String, BTreeSet<String>>,
    prefix_by_namespace: BTreeMap<String, BTreeSet<String>>,
}

impl UsedOntologyProjectNamespaces {
    fn is_empty(&self) -> bool {
        self.by_prefix.is_empty()
    }

    fn namespaces_for_iri(&self, iri: &str) -> Vec<&str> {
        self.prefix_by_namespace
            .keys()
            .filter(|namespace| iri.starts_with(namespace.as_str()))
            .map(String::as_str)
            .collect()
    }

    fn prefix_for_namespace(&self, namespace: &str) -> Option<&str> {
        self.prefix_by_namespace
            .get(namespace)?
            .iter()
            .next()
            .map(String::as_str)
    }
}

fn parse_turtle_prefixes(content: &str) -> Vec<(String, String)> {
    let mut prefixes = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();
        let Some(rest) = lower
            .strip_prefix("@prefix ")
            .map(|_| &trimmed["@prefix ".len()..])
            .or_else(|| {
                lower
                    .strip_prefix("prefix ")
                    .map(|_| &trimmed["prefix ".len()..])
            })
        else {
            continue;
        };

        let mut parts = rest.split_whitespace();
        let Some(prefix_token) = parts.next() else {
            continue;
        };
        let Some(iri_token) = parts.next() else {
            continue;
        };
        let Some(prefix) = prefix_token.strip_suffix(':') else {
            continue;
        };
        let Some(iri) = iri_token
            .strip_prefix('<')
            .and_then(|value| value.strip_suffix('>'))
        else {
            continue;
        };
        prefixes.push((prefix.to_string(), iri.to_string()));
    }
    prefixes
}

fn resolve_concept_reference_iri(
    value: &str,
    prefixes: &HashMap<String, String>,
) -> Result<String, String> {
    let trimmed = value.trim();
    if let Some(iri) = trimmed
        .strip_prefix('<')
        .and_then(|value| value.strip_suffix('>'))
    {
        return Ok(iri.to_string());
    }
    if trimmed.starts_with("urn:")
        || trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
    {
        return Ok(trimmed.to_string());
    }

    let Some((prefix, local)) = trimmed.split_once(':') else {
        return Err("expected absolute IRI, <IRI>, or CURIE".to_string());
    };
    let Some(base) = prefixes.get(prefix) else {
        return Err(format!(
            "prefix '{}' is not declared by a reachable ontology",
            prefix
        ));
    };
    if local.is_empty() {
        return Err("CURIE local name is empty".to_string());
    }
    Ok(format!("{}{}", base, local))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{Element, ElementType, RequirementType};
    use crate::relation::{LinkType, Relation, RelationTarget, RELATION_TYPES};

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
        let element_id = crate::utils::extract_path_and_fragment(to_id)
            .1
            .map(|f| f.to_string());
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
    fn populate_size_estimates_adds_non_recursive_element_metadata() {
        let mut registry = GraphRegistry::new();
        let element = make_element("file.md#size-estimate", "Size Estimate");

        registry
            .register_element(element, "file.md")
            .expect("element should register");
        registry
            .populate_size_estimates()
            .expect("size estimates should populate");

        let element = registry
            .get_element("file.md#size-estimate")
            .expect("element should be present");
        let estimate = element
            .size_estimate
            .as_ref()
            .expect("size estimate should be present");

        let mut without_estimate = element.clone();
        without_estimate.size_estimate = None;
        let expected_rendered_context_bytes = serde_json::to_vec(&without_estimate).unwrap().len();

        assert_eq!(estimate.content_bytes, element.content.len());
        assert_eq!(
            estimate.rendered_context_bytes,
            expected_rendered_context_bytes
        );
        assert_eq!(
            estimate.estimated_tokens,
            expected_rendered_context_bytes.div_ceil(4)
        );
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
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("does not exist in the graph"));
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
        assert_eq!(
            b_relations_after.len(),
            1,
            "B should still have 1 relation after A is moved"
        );
        assert_eq!(
            c_relations_after.len(),
            1,
            "C should still have 1 relation after A is moved"
        );

        // The target should still be "A" (or updated identifier if it changed)
        let b_target = &b_relations_after[0].1;
        let c_target = &c_relations_after[0].1;

        // Verify the targets still exist in the graph
        assert!(
            graph.get_element(b_target).is_some(),
            "B's relation target '{}' should exist in graph",
            b_target
        );
        assert!(
            graph.get_element(c_target).is_some(),
            "C's relation target '{}' should exist in graph",
            c_target
        );
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
        let b_markdown = graph.element_to_markdown_with_context(&b_element, "file1.md", true);
        println!("B's markdown after A is moved:");
        println!("{}", b_markdown);

        // The relation should be "file2.md#A" since A is now in a different file
        // but it's probably still "A" which would be incorrect
        assert!(
            b_markdown.contains("file2.md#A") || b_markdown.contains("[A](file2.md#A)"),
            "B's relation should reference A in its new location: {}",
            b_markdown
        );
    }

    #[test]
    fn test_moved_element_relations_update_paths() {
        let mut registry = GraphRegistry::new();

        // Create elements A, B, C where A has relations to both B and C
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file2.md".to_string(); // B is in different file

        let mut c = make_element("C", "Element C");
        c.file_path = "file1.md".to_string(); // C is in same file as A initially

        // A has relations to both B (cross-file) and C (same-file)
        add_relation(&mut a, "derivedFrom", "B");
        add_relation(&mut a, "derive", "C");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();
        registry.register_element(c.clone(), "file1.md").unwrap();

        let mut graph = registry;

        // Check A's initial relations in markdown
        let a_element_initial = graph.nodes.get("A").unwrap().element.clone();
        let a_markdown_initial =
            graph.element_to_markdown_with_context(&a_element_initial, "file1.md", true);
        println!("A's initial markdown (in file1.md):");
        println!("{}", a_markdown_initial);

        // A is in file1.md, B is in file2.md, C is in file1.md
        // So A should reference B as "file2.md#B" and C as just "C" (same file)

        // Move A to file3.md
        let result = graph.move_element_to_new_file("A", "file3.md");
        assert!(result.is_ok());

        // Check A's relations after the move
        let a_element_moved = graph.nodes.get("A").unwrap().element.clone();
        let a_markdown_moved =
            graph.element_to_markdown_with_context(&a_element_moved, "file3.md", true);
        println!("A's markdown after move to file3.md:");
        println!("{}", a_markdown_moved);

        // Now A is in file3.md, so:
        // - A should reference B as "file2.md#B" (cross-file, B is in file2.md)
        // - A should reference C as "file1.md#C" (cross-file, C is in file1.md)
        // Both should be cross-file references now since A moved to file3.md

        println!("A's relations after move:");
        for relation in &a_element_moved.relations {
            println!(
                "  {} -> {}",
                relation.relation_type.name,
                match &relation.target.link {
                    crate::relation::LinkType::Identifier(id) => id.clone(),
                    crate::relation::LinkType::InternalPath(path) =>
                        path.to_string_lossy().to_string(),
                    crate::relation::LinkType::ExternalUrl(url) => url.clone(),
                }
            );
        }

        // PROBLEM: A's relations likely still point to "B" and "C"
        // but should now point to "file2.md#B" and "file1.md#C" respectively
        // since A is now in a different file than both of them

        assert!(
            a_markdown_moved.contains("file2.md#B") || a_markdown_moved.contains("[B](file2.md#B)"),
            "A should reference B with file path since they're in different files: {}",
            a_markdown_moved
        );
        assert!(
            a_markdown_moved.contains("file1.md#C") || a_markdown_moved.contains("[C](file1.md#C)"),
            "A should reference C with file path since they're in different files: {}",
            a_markdown_moved
        );
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
        let result = graph.flush_to_directory(output_path, true);
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
        assert!(
            file1_content.contains("[Element B Description](file3.md#ElementB)"),
            "ElementA should reference ElementB with proper display name: {}",
            file1_content
        );

        // A -> C should be same-file reference (no file prefix needed)
        assert!(
            file1_content.contains("[ElementC](ElementC)")
                || file1_content.contains("[ElementC](#ElementC)")
                || file1_content.contains("ElementC"),
            "ElementA should reference ElementC in same file: {}",
            file1_content
        );

        // Verify file3.md content (contains ElementB)
        assert!(file3_content.contains("### Element B Description"));
        assert!(!file3_content.contains("### Element A Description"));
        assert!(!file3_content.contains("### Element C Description"));

        // Verify ElementB's relations in file3.md
        // B -> A should be cross-file reference with proper display name and fragment anchor
        assert!(
            file3_content.contains("[Element A Description](file1.md#ElementA)"),
            "ElementB should reference ElementA with proper display name: {}",
            file3_content
        );

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

        registry
            .register_element(a.clone(), "test_file.md")
            .unwrap();

        // Add page content
        let page =
            Page::new("This is page frontmatter content.\n\nMore page content here.".to_string());
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path, true);
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
        let page_content_pos = file_content
            .find("This is page frontmatter content.")
            .unwrap();
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

        registry
            .register_element(a.clone(), "test_file.md")
            .unwrap();
        registry
            .register_element(b.clone(), "test_file.md")
            .unwrap();

        // Add page content
        let page = Page::new("Page frontmatter content.".to_string());
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path, true);
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

        registry
            .register_element(a.clone(), "test_file.md")
            .unwrap();

        // Add empty page content (should be skipped)
        let page = Page::new("   \n\t  \n  ".to_string()); // only whitespace
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path, true);
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
        let result = graph.flush_to_directory(output_path, true);
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
