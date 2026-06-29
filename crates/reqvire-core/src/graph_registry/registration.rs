use super::*;

impl GraphRegistry {
    pub(super) fn is_single_element_format_file(&self, file_path: &str) -> bool {
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
            nodes: FxHashMap::default(),
            pages: FxHashMap::default(),
            modified_files: FxHashSet::default(),
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

        // Materialize namespace-derived concept payload fields after relation
        // resolution so JSON evidence and generated RDF use the same SKOS IRIs.
        self.materialize_concept_payload_context();

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

        // Validate contract_bindings exist
        errors.extend(self.validate_contract_bindings()?);

        // Validate legacy contract relation names before the stricter
        // contract-element ownership checks.
        errors.extend(self.validate_legacy_contract_relations()?);

        // Validate contract elements have only define relations
        errors.extend(self.validate_contract_elements()?);

        // Validate ontology elements and ontology graph shape
        errors.extend(self.validate_ontology_elements()?);

        // Validate native concept-scheme and concept element constraints
        errors.extend(self.validate_concept_elements()?);

        // Validate reserved requirement governance metadata
        errors.extend(self.validate_governance_metadata()?);

        // Validate contract ownership uniqueness (each contract owned by at most one requirement)
        errors.extend(self.validate_contract_ownership_uniqueness()?);

        // Validate explicit 'other' type elements do not author semantic relations
        errors.extend(self.validate_other_element_relations()?);

        // Validate no cross-section duplicates (same target in Relations and Contract Bindings)
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
                let rendered_context_bytes = serde_json::to_vec(&node.element)?.len();
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

    /// Build the relation graph structure
    pub(super) fn build_relation_graph(&mut self) {
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
    pub(super) fn add_opposite_to_target(
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
    pub(super) fn remove_opposite_from_target(
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
            let target_node = self
                .nodes
                .get_mut(target_id)
                .expect("node not found in registry");
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
    pub(super) fn recreate_opposites_after_move(&mut self, old_id: &str, new_id: &str) {
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
    pub(super) fn populate_relation_element_ids(&mut self) {
        log::debug!("Populating element_id for all relations...");

        // First pass: collect mapping of identifier -> element_id
        let id_map: rustc_hash::FxHashMap<String, String> = self
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

    pub(super) fn resolve_relation_identifier(
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
    pub(super) fn normalize_relation_identifier_for_source(
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
                crate::utils::normalize_identifier(target_id, std::path::Path::new("/"))
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
    pub(super) fn relation_targets_same_identifier(
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
        if target_id.starts_with('#')
            && format!("{}{}", source_file_path, target_id) == expected_target
        {
            return true;
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
}
