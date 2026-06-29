use super::*;

impl GraphRegistry {
    /// Validates that no element has the same target in both Relations and Contract Bindings subsections
    pub(super) fn validate_cross_section_duplicates(
        &self,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        log::debug!("Running cross-section duplicate validation...");
        let mut errors = Vec::new();

        for node in self.nodes.values() {
            let element = &node.element;

            // Collect all relation targets (normalized identifiers)
            let relation_targets: rustc_hash::FxHashSet<String> = element
                .relations
                .iter()
                .filter(|r| r.user_created)
                .map(|r| r.target.link.as_str().to_string())
                .collect();

            // Check contract_bindings against relations
            for contract_bindings in &element.contract_bindings {
                let contract_bindings_target = contract_bindings.target.as_str();
                if relation_targets.contains(&contract_bindings_target) {
                    let msg = format!(
                        "Cross-section duplicate in element '{}': target '{}' appears in both Relations and Contract Bindings (file: {})",
                        element.name, contract_bindings_target, element.file_path
                    );
                    errors.push(ReqvireError::CrossSectionDuplicate(msg));
                }
            }
        }

        Ok(errors)
    }

    /// Validates relations for target existence and element type compatibility.
    pub(super) fn validate_relations(
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

    pub(super) fn validate_element_type_metadata(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
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
    pub(super) fn validate_non_test_verification_satisfied_by(
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

    pub(super) fn validate_verification_objective_parents(
        &self,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
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
    pub(super) fn validate_cross_component_dependencies(
        &self,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating cross-component dependencies...");
        let mut errors = Vec::new();
        let mut visited = FxHashSet::default();

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
        let mut memo: FxHashMap<String, BTreeSet<String>> = FxHashMap::default();
        let mut visiting: FxHashSet<String> = FxHashSet::default();

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
        memo: &mut FxHashMap<String, BTreeSet<String>>,
        visiting: &mut FxHashSet<String>,
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
        let mut memo: FxHashMap<String, BTreeSet<String>> = FxHashMap::default();
        let mut visiting: FxHashSet<String> = FxHashSet::default();
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

    fn has_contract_bindings_flow_between_roots(
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

            for contract_bindings in &node.element.contract_bindings {
                let crate::element::ContractBindingTarget::ElementIdentifier(contract_id) =
                    &contract_bindings.target
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

    pub fn build_contract_bindings_direction_scope_error(
        &self,
        contract_bindings_identifier: &str,
        element_id: &str,
        element_name: &str,
        file_path: Option<&str>,
    ) -> Option<String> {
        let source_root_id = self.resolve_single_owning_capability(element_id)?;

        let mut cross_subgraph_target = false;
        for defining_req_id in self.get_defining_requirements(contract_bindings_identifier) {
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
            .get_defining_requirements(contract_bindings_identifier)
            .into_iter()
            .filter_map(|defining_req_id| self.resolve_single_owning_capability(&defining_req_id))
            .find(|target_root_id| {
                target_root_id != &source_root_id
                    && self
                        .has_contract_bindings_flow_between_roots(target_root_id, &source_root_id)
            })?;
        let reused_context_name = self.display_name_for_element(contract_bindings_identifier);
        let conflicting_root_name = self.display_name_for_element(&conflicting_root_id);
        let source_root_name = self.display_name_for_element(&source_root_id);

        let mut msg = format!(
            "'{}' cannot be bound to '{}' because subgraph '{}' already binds contracts owned by subgraph '{}'. Contract Binding flow between subgraphs must remain one-directional.",
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

    /// Validates contract_bindings targets and scope rules.
    pub(super) fn validate_contract_bindings(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating contract_bindings targets...");
        let mut errors = Vec::new();

        let mut sorted_nodes: Vec<&ElementNode> = self.nodes.values().collect();
        sorted_nodes.sort_by(|a, b| a.element.identifier.cmp(&b.element.identifier));

        for element_node in sorted_nodes {
            let element = &element_node.element;

            for contract_bindings in &element.contract_bindings {
                if !element.element_type.is_requirement() {
                    errors.push(ReqvireError::InvalidContractBindingTarget(format!(
                        "File {}: Element '{}' (type: {}) cannot author contract_bindings. Only requirement elements may author contract_bindings to reusable requirement-owned contracts; ontology vocabulary uses Concept References and semantic contracts use use/usedBy.",
                        element.file_path,
                        element.name,
                        element.element_type.as_str(),
                    )));
                    continue;
                }

                match &contract_bindings.target {
                    crate::element::ContractBindingTarget::FilePath(file_path) => {
                        errors.push(ReqvireError::InvalidContractBindingTarget(format!(
                            "File {}: Element '{}' has contract_bindings '{}' which is a file path. Contract Bindings must target reusable element identifiers (file.md#element-id).",
                            element.file_path,
                            element.name,
                            file_path.display()
                        )));
                        continue;
                    }
                    crate::element::ContractBindingTarget::ElementIdentifier(identifier) => {
                        // Validate that the identifier points to an existing Contract element
                        if let Some(target_node) = self.nodes.get(identifier) {
                            if !target_node.element.element_type.is_contract() {
                                errors.push(ReqvireError::InvalidContractBindingTarget(
                                    format!(
                                        "File {}: Element '{}' has contract_bindings to '{}' which is not an reusable element",
                                        element.file_path,
                                        element.name,
                                        identifier
                                    ),
                                ));
                                continue;
                            }

                            let contract_binding_type_valid =
                                target_node.element.element_type.is_requirement_contract();

                            if !contract_binding_type_valid {
                                errors.push(ReqvireError::InvalidContractBindingTarget(
                                    format!(
                                        "File {}: Element '{}' (type: {}) has invalid contract_bindings to '{}' (type: {}). Requirement contract_bindings may target requirement-owned source, constraint, behavior, specification, state, or input-output only. Ontology vocabulary uses Concept References; semantic contracts constrain requirements through constrainedBy/constrain.",
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
                                errors.push(ReqvireError::InvalidContractBindingTarget(
                                    format!(
                                        "'{}' has no define relation. Contracts must define a requirement before they can be reused; contracts are requirement-owned only. Capabilities use concept references for SKOS concepts and are specified by requirements; verification coverage rolls up from verified requirements. (file: {}, element: {})",
                                        target_node.element.name,
                                        element.file_path,
                                        element.name
                                    ),
                                ));
                                continue;
                            }

                            // Check 2: Hierarchical Independence Constraint - bindContract element must not be in defining hierarchy
                            let defining_reqs = self.get_defining_requirements(identifier);
                            let mut hierarchy_violation = false;
                            for defining_req_id in defining_reqs {
                                if self.is_in_hierarchy(&element.identifier, &defining_req_id) {
                                    errors.push(ReqvireError::InvalidContractBindingScope(
                                        format!(
                                            "'{}' cannot be bound to '{}' because it is within the contract's defining hierarchy. Contract Bindings are only allowed from elements outside the definedBy chain. (file: {}, element: {})",
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
                                    .build_contract_bindings_direction_scope_error(
                                        identifier,
                                        &element.identifier,
                                        &element.name,
                                        Some(&element.file_path),
                                    )
                                {
                                    errors.push(ReqvireError::InvalidContractBindingScope(msg));
                                    hierarchy_violation = true;
                                }
                            }

                            // Check 4: Upstream propagation constraint (only if no other scope violation)
                            if !hierarchy_violation {
                                if let Some((direction, other_id)) = self
                                    .find_duplicate_contract_bindings_in_hierarchy(
                                        &element.identifier,
                                        &contract_bindings.target,
                                    )
                                {
                                    let other_name = self
                                        .nodes
                                        .get(&other_id)
                                        .map(|n| n.element.name.as_str())
                                        .unwrap_or(&other_id);
                                    let msg = if direction == "ancestor" {
                                        format!(
                                            "'{}' is already bound at '{}' which is an ancestor. Contract Bindings propagate downstream. (file: {}, element: {})",
                                            target_node.element.name,
                                            other_name,
                                            element.file_path,
                                            element.name
                                        )
                                    } else {
                                        format!(
                                            "'{}' is already bound at '{}' which is a descendant. Move contract_bindings to '{}' if you want it at higher level. (file: {}, element: {})",
                                            target_node.element.name,
                                            other_name,
                                            element.name,
                                            element.file_path,
                                            element.name
                                        )
                                    };
                                    errors.push(ReqvireError::InvalidContractBindingScope(msg));
                                }
                            }
                        } else {
                            errors.push(ReqvireError::MissingContractBindingTarget(format!(
                                "File {}: Element '{}' references missing contract_bindings element: {}",
                                element.file_path, element.name, identifier
                            )));
                        }
                    }
                }
            }
        }

        if errors.is_empty() {
            debug!("No contract_bindings validation errors found.");
        } else {
            debug!(
                "{} contract_bindings validation errors found.",
                errors.len()
            );
        }

        Ok(errors)
    }

    pub(super) fn validate_legacy_contract_relations(
        &self,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
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
    /// and cannot have contract_bindings.
    pub(super) fn validate_contract_elements(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
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

                if !element.contract_bindings.is_empty() {
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "File {}: Semantic contract element '{}' cannot have contract_bindings. Semantic contracts use ontology through use relations and constrain requirements through constrain/constrainedBy.",
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

                // Contract elements cannot have contract_bindings
                if !element.contract_bindings.is_empty() {
                    errors.push(ReqvireError::InvalidMarkdownStructure(
                        format!(
                            "File {}: Contract element '{}' (type: {}) cannot have contract_bindings. Contract elements are atomic documentation units meant to be bound to requirements.",
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

    pub(super) fn validate_ontology_elements(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
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

            if !element.contract_bindings.is_empty() {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Ontology element '{}' cannot have contract_bindings. Ontology vocabulary is referenced through Concept References or semantic-contract use relations.",
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

    pub(super) fn validate_concept_elements(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating concept element constraints...");
        let mut errors = Vec::new();
        let mut generated_iris: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for node in self.nodes.values() {
            let element = &node.element;
            if !element.element_type.is_concept_family() {
                continue;
            }

            for forbidden_key in ["concept_id", "concept_kind", "pref_label", "language"] {
                if element.metadata.contains_key(forbidden_key) {
                    errors.push(ReqvireError::InvalidMetadataFormat(format!(
                        "File {}: Concept element '{}' (type: {}) must not declare concept-specific metadata key '{}'. Native concepts derive identifiers, preferred labels, and generated RDF literals from element identity and Markdown content; language metadata is deferred until Reqvire has a global language policy.",
                        element.file_path,
                        element.name,
                        element.element_type.as_str(),
                        forbidden_key
                    )));
                }
            }

            for forbidden in [
                "Ontology",
                "Shapes",
                "Concepts",
                "Details",
                "Definition",
                "Top Concepts",
            ] {
                if crate::parser::has_subsection(&element.content, forbidden) {
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "File {}: Concept element '{}' (type: {}) must not contain a #### {} section. Native concepts use the main element body for skos:definition, #### Relations for concept links, and derived top concepts from direct concept-scheme children.",
                        element.file_path,
                        element.name,
                        element.element_type.as_str(),
                        forbidden
                    )));
                }
            }

            if element.element_type.is_concept_scheme() {
                let has_base = element
                    .metadata
                    .get("concept_base")
                    .map(|value| value.trim())
                    .is_some_and(|value| !value.is_empty());
                let has_prefix = element
                    .metadata
                    .get("concept_prefix")
                    .map(|value| value.trim())
                    .is_some_and(|value| !value.is_empty());
                if !has_base {
                    errors.push(ReqvireError::InvalidMetadataFormat(format!(
                        "File {}: Concept scheme '{}' must define non-empty concept_base metadata.",
                        element.file_path, element.name
                    )));
                }
                if !has_prefix {
                    errors.push(ReqvireError::InvalidMetadataFormat(format!(
                        "File {}: Concept scheme '{}' must define non-empty concept_prefix metadata.",
                        element.file_path, element.name
                    )));
                }
            }

            if element.element_type.is_concept()
                && (element.metadata.contains_key("concept_base")
                    || element.metadata.contains_key("concept_prefix"))
            {
                errors.push(ReqvireError::InvalidMetadataFormat(format!(
                    "File {}: Concept '{}' must not declare concept_base or concept_prefix metadata. Native concepts inherit namespace from the nearest concept-scheme ancestor.",
                    element.file_path, element.name
                )));
            }

            if element.element_type.is_concept()
                && self
                    .concept_scheme_context_id(&element.identifier)
                    .is_none()
            {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Concept '{}' must derive from a concept-scheme or another concept with scheme context.",
                    element.file_path, element.name
                )));
            }

            if let Some(scheme) = &element.concept_scheme {
                for top in &scheme.top_concepts {
                    if !self.concept_link_resolves_to_type(&top.target, |candidate| {
                        candidate.element_type.is_concept()
                    }) {
                        errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                            "File {}: Concept scheme '{}' top concept '{}' must resolve to a concept element.",
                            element.file_path, element.name, top.label
                        )));
                    }
                }
            }

            if let Some(concept) = &element.concept {
                for (relation, links) in [
                    ("broader", &concept.broader),
                    ("narrower", &concept.narrower),
                ] {
                    for link in links {
                        let target_id = self.resolve_concept_element_id(&link.target);
                        if target_id.is_none() {
                            errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                                "File {}: Concept '{}' {} target '{}' must resolve to a concept element.",
                                element.file_path, element.name, relation, link.target
                            )));
                            continue;
                        }
                        let Some(source_scheme_id) =
                            self.concept_scheme_context_id(&element.identifier)
                        else {
                            continue;
                        };
                        let target_id = target_id.expect("checked concept target");
                        let Some(target_scheme_id) = self.concept_scheme_context_id(&target_id)
                        else {
                            continue;
                        };
                        if source_scheme_id != target_scheme_id {
                            let source_scheme = self
                                .nodes
                                .get(&source_scheme_id)
                                .map(|node| node.element.name.as_str())
                                .unwrap_or(source_scheme_id.as_str());
                            let target_scheme = self
                                .nodes
                                .get(&target_scheme_id)
                                .map(|node| node.element.name.as_str())
                                .unwrap_or(target_scheme_id.as_str());
                            let target_name = self
                                .nodes
                                .get(&target_id)
                                .map(|node| node.element.name.as_str())
                                .unwrap_or(link.label.as_str());
                            errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                                "File {}: Concept taxonomy relation crosses concept schemes: concept '{}' uses {} to concept '{}', but source scheme '{}' differs from target scheme '{}'. Keep broader/narrower inside one concept scheme; use related, exactMatch, or closeMatch for cross-scheme concept alignment.",
                                element.file_path,
                                element.name,
                                relation,
                                target_name,
                                source_scheme,
                                target_scheme
                            )));
                        }
                    }
                }
                for link in &concept.related {
                    if self.concept_link_resolves_to_type(&link.target, |candidate| {
                        candidate.element_type.is_concept()
                    }) {
                        continue;
                    }
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "File {}: Concept '{}' related target '{}' must resolve to a concept element.",
                        element.file_path, element.name, link.target
                    )));
                }
                for (relation, links) in [
                    ("exactMatch", &concept.exact_match),
                    ("closeMatch", &concept.close_match),
                ] {
                    for link in links {
                        if self.valid_concept_mapping_target(&link.target) {
                            continue;
                        }
                        errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                            "File {}: Concept '{}' {} target '{}' must be an http(s) IRI or resolve to a concept element.",
                            element.file_path, element.name, relation, link.target
                        )));
                    }
                }
            }

            if let Some(iri) = self.generated_concept_iri_for_element(element) {
                generated_iris
                    .entry(iri)
                    .or_default()
                    .push(element.identifier.clone());
            }
        }

        for (iri, mut identifiers) in generated_iris {
            identifiers.sort();
            identifiers.dedup();
            if identifiers.len() > 1 {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Duplicate generated concept IRI <{}> for concept elements: {}.",
                    iri,
                    identifiers.join(", ")
                )));
            }
        }

        let mut prefixes: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut namespaces: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for node in self.nodes.values() {
            let element = &node.element;
            if !element.element_type.is_concept_scheme() {
                continue;
            }
            if let Some((base, prefix)) = self.concept_scheme_namespace_context(element) {
                prefixes
                    .entry(prefix)
                    .or_default()
                    .push(element.identifier.clone());
                namespaces
                    .entry(concept_namespace_iri(&base))
                    .or_default()
                    .push(element.identifier.clone());
            }
        }
        for (prefix, mut identifiers) in prefixes {
            identifiers.sort();
            identifiers.dedup();
            if identifiers.len() > 1 {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Duplicate concept_prefix '{}' for concept schemes: {}.",
                    prefix,
                    identifiers.join(", ")
                )));
            }
        }
        for (namespace, mut identifiers) in namespaces {
            identifiers.sort();
            identifiers.dedup();
            if identifiers.len() > 1 {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Duplicate concept namespace <{}> for concept schemes: {}.",
                    namespace,
                    identifiers.join(", ")
                )));
            }
        }

        Ok(errors)
    }

    pub(super) fn validate_governance_metadata(&self) -> Result<Vec<ReqvireError>, ReqvireError> {
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

    pub(super) fn validate_semantic_contracts(
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
            if element.element_type.is_semantic_contract()
                && self
                    .semantic_contract_used_ontology_context(&element.identifier)
                    .is_empty()
            {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Semantic contract '{}' must use at least one ontology element through a use/usedBy relation.",
                    element.identifier
                )));
            }
        }

        errors.extend(self.validate_semantic_contract_shape_alignment(
            &semantic_index,
            removed_declaration_source,
        ));

        errors.extend(self.validate_semantic_contract_shape_prefixes(&semantic_index));

        errors.extend(self.validate_concept_references(removed_declaration_source));

        errors.extend(self.validate_maps_to_concept_targets(&semantic_index));

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

    fn validate_semantic_contract_shape_alignment(
        &self,
        semantic_index: &semantic_contract::SemanticIndex,
        removed_declaration_source: Option<&str>,
    ) -> Vec<ReqvireError> {
        let mut errors = Vec::new();
        let mut seen = BTreeSet::new();

        for block in &semantic_index.blocks {
            if !matches!(block.kind, semantic_contract::SemanticBlockKind::Shapes) {
                continue;
            }

            let context = self.semantic_contract_used_ontology_context(&block.source);
            if context.is_empty() {
                continue;
            }
            let context: BTreeSet<String> = context.into_iter().collect();
            let domain_index = semantic_index.shacl_domain_ontology_index(&context);
            let registry = shacl::ShaclRegistry::parse(&block.quads);
            let aligner = shacl::OntologyAligner::new(&domain_index);

            for alignment_error in aligner.cross_check_shapes(&registry.compiled_shapes) {
                let Some((iri, kind)) = ontology::alignment_reference(&alignment_error) else {
                    continue;
                };
                let key = (block.source.clone(), kind.to_string(), iri.to_string());
                if !seen.insert(key) {
                    continue;
                }
                if owl_reserved::is_reserved_vocabulary_iri(iri) {
                    continue;
                }
                let kind_label = shacl::predicate_label(kind);

                if let Some(declarations) = semantic_index.ontology_declarations.get(iri) {
                    let declaration_sources: BTreeSet<String> = declarations
                        .iter()
                        .map(|declaration| declaration.element_identifier.clone())
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
                        block.source, kind_label, iri, declaring_contract
                    )));
                    continue;
                }

                let removed_source = removed_declaration_source
                    .map(|source| format!(" Removed declaration source: {}.", source))
                    .unwrap_or_default();
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "Semantic reference not found: semantic contract '{}' references {} <{}>, but no ontology element declares this IRI.{} Update or remove the SHACL reference before deleting or editing the declaring ontology.",
                    block.source, kind_label, iri, removed_source
                )));
            }
        }

        errors
    }

    fn validate_maps_to_concept_targets(
        &self,
        semantic_index: &semantic_contract::SemanticIndex,
    ) -> Vec<ReqvireError> {
        const REQVIRE_MAPS_TO_CONCEPT: &str = "https://www.reqvire.org/ontology#mapsToConcept";
        let mut errors = Vec::new();

        for block in &semantic_index.blocks {
            if !matches!(block.kind, semantic_contract::SemanticBlockKind::Ontology) {
                continue;
            }
            for quad in &block.quads {
                if quad.predicate.as_str() != REQVIRE_MAPS_TO_CONCEPT {
                    continue;
                }
                let subject = subject_iri(&quad.subject).unwrap_or("<blank>");
                let Some(target_iri) = term_iri(&quad.object) else {
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "File {}: ontology element '{}' maps <{}> to a non-IRI concept target. reqvire:mapsToConcept must target a generated native concept IRI.",
                        block.file_path, block.source, subject
                    )));
                    continue;
                };
                if !semantic_index.is_skos_concept_iri(target_iri) {
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "File {}: ontology element '{}' maps <{}> to <{}>, but reqvire:mapsToConcept must target a generated native concept resource typed as skos:Concept.",
                        block.file_path, block.source, subject, target_iri
                    )));
                }
            }
        }

        errors
    }

    fn validate_semantic_contract_shape_prefixes(
        &self,
        semantic_index: &semantic_contract::SemanticIndex,
    ) -> Vec<ReqvireError> {
        let mut errors = Vec::new();
        let mut references_by_contract: FxHashMap<
            &str,
            Vec<&semantic_contract::ShapeIriReference>,
        > = FxHashMap::default();

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
            let project_namespaces = semantic_index.used_ontology_project_namespaces(&context_set);
            if project_namespaces.is_empty() {
                continue;
            }

            let shape_prefixes =
                semantic_contract::parse_turtle_prefix_declarations(&block.content);
            let mut declared_namespaces = BTreeSet::new();
            for (prefix, namespace) in &shape_prefixes {
                declared_namespaces.insert(namespace.clone());
                if let Some(expected_namespaces) = project_namespaces.namespaces_for_prefix(prefix)
                {
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
        removed_declaration_source: Option<&str>,
    ) -> Vec<ReqvireError> {
        let mut errors = Vec::new();

        for node in self.nodes.values() {
            let element = &node.element;
            let (references, diagnostics) =
                crate::parser::extract_concept_references(&element.content);

            if element.element_type.is_ontology()
                && crate::parser::has_subsection(&element.content, "Concept References")
            {
                errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                    "File {}: Ontology element '{}' must not contain a #### Concept References section. Ontology elements declare terms in #### Ontology.",
                    element.file_path, element.name
                )));
            }
            if element.element_type.is_semantic_contract()
                && crate::parser::has_subsection(&element.content, "Concept References")
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

            for reference in references {
                let target_id = match crate::parser::normalize_concept_reference_target(
                    &element.file_path,
                    &reference.target,
                ) {
                    Ok(target_id) => target_id,
                    Err(error) => {
                        errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                            "Concept reference syntax error: element '{}' label '{}' at line {} references '{}': {}",
                            element.identifier,
                            reference.label,
                            reference.line_number,
                            reference.target,
                            error
                        )));
                        continue;
                    }
                };

                let Some(target_node) = self.nodes.get(&target_id) else {
                    let removed_source = removed_declaration_source
                        .map(|source| format!(" Removed declaration source: {}.", source))
                        .unwrap_or_default();
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "Concept reference not found: element '{}' label '{}' references concept element '{}'.{} Update or remove the Concept References entry before deleting or editing the concept element.",
                        element.identifier,
                        reference.label,
                        target_id,
                        removed_source
                    )));
                    continue;
                };

                if !target_node.element.element_type.is_concept() {
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "Invalid concept reference target: element '{}' label '{}' references '{}', but Concept References may target only native concept elements. Use reqvire:mapsToConcept in authored ontology to bridge structural OWL terms to curated SKOS concepts.",
                        element.identifier,
                        reference.label,
                        target_id
                    )));
                    continue;
                }

                if self
                    .generated_concept_iri_for_element(&target_node.element)
                    .is_none()
                {
                    errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                        "Invalid concept reference target: element '{}' label '{}' references '{}', but the target concept does not derive a concept-scheme namespace.",
                        element.identifier,
                        reference.label,
                        target_id
                    )));
                }
            }
        }

        errors
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

    /// Validates that each contract is owned by at most one requirement via definedBy.
    /// A contract element or file can only appear as a target of definedBy from one owner.
    pub(super) fn validate_contract_ownership_uniqueness(
        &self,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Validating contract ownership uniqueness...");
        let mut errors = Vec::new();
        // Map from contract target (identifier or file path) to owning element identifier
        let mut ownership_map: rustc_hash::FxHashMap<String, String> =
            rustc_hash::FxHashMap::default();

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
    pub(super) fn validate_other_element_relations(
        &self,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
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
        visited: &mut FxHashSet<String>,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{Element, ElementType};
    use crate::relation::{LinkType, Relation, RelationTarget, RELATION_TYPES};

    fn make_element(id: &str, name: &str, element_type: Option<ElementType>) -> Element {
        let mut element = Element::new(name, id, "file.md", 1, element_type);
        element.content = format!("This is {}", name);
        element.freeze_content();
        element
    }

    fn add_relation(from: &mut Element, relation_type: &'static str, to_id: &str) {
        let relation_info = RELATION_TYPES
            .get(relation_type)
            .expect("relation type should exist in RELATION_TYPES");
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
    fn validate_relations_accepts_compatible_derive() {
        let mut a = make_element("file.md#A", "A", None);
        let b = make_element("file.md#B", "B", None);
        add_relation(&mut a, "derive", "file.md#B");

        let mut registry = GraphRegistry::new();
        registry
            .register_element(a, "file.md")
            .expect("element A should register");
        registry
            .register_element(b, "file.md")
            .expect("element B should register");
        registry.build_relation_graph();

        let excluded = globset::GlobSetBuilder::new()
            .build()
            .expect("empty globset");
        let errors = registry
            .validate_relations(&excluded)
            .expect("validate_relations should return Ok");
        assert!(
            errors.is_empty(),
            "expected no errors for compatible derive, got {:?}",
            errors
        );
    }

    #[test]
    fn validate_relations_reports_missing_target() {
        let mut a = make_element("file.md#A", "A", None);
        add_relation(&mut a, "derive", "file.md#missing");

        let mut registry = GraphRegistry::new();
        registry
            .register_element(a, "file.md")
            .expect("element A should register");
        registry.build_relation_graph();

        let excluded = globset::GlobSetBuilder::new()
            .build()
            .expect("empty globset");
        let errors = registry
            .validate_relations(&excluded)
            .expect("validate_relations should return Ok");
        assert_eq!(errors.len(), 1, "expected exactly one error");
        match &errors[0] {
            ReqvireError::MissingRelationTarget(message) => {
                assert!(
                    message.contains("references missing target"),
                    "expected missing-target message, got: {}",
                    message
                );
            }
            other => panic!("expected MissingRelationTarget, got {:?}", other),
        }
    }

    #[test]
    fn validate_relations_reports_incompatible_types() {
        let mut cap = make_element("file.md#cap", "Cap", Some(ElementType::Capability));
        let req = make_element("file.md#req", "Req", None);
        add_relation(&mut cap, "derive", "file.md#req");

        let mut registry = GraphRegistry::new();
        registry
            .register_element(cap, "file.md")
            .expect("capability should register");
        registry
            .register_element(req, "file.md")
            .expect("requirement should register");
        registry.build_relation_graph();

        let excluded = globset::GlobSetBuilder::new()
            .build()
            .expect("empty globset");
        let errors = registry
            .validate_relations(&excluded)
            .expect("validate_relations should return Ok");
        assert_eq!(errors.len(), 1, "expected exactly one error");
        match &errors[0] {
            ReqvireError::IncompatibleElementTypes(message) => {
                assert!(
                    message.contains("incompatible element types"),
                    "expected incompatible-types message, got: {}",
                    message
                );
            }
            other => panic!("expected IncompatibleElementTypes, got {:?}", other),
        }
    }

    #[test]
    fn validate_single_root_hierarchy_accepts_lone_capability() {
        let cap = make_element("file.md#cap", "Cap", Some(ElementType::Capability));

        let mut registry = GraphRegistry::new();
        registry
            .register_element(cap, "file.md")
            .expect("capability should register");
        registry.build_relation_graph();

        let errors = registry
            .validate_single_root_hierarchy_ownership_in_memory()
            .expect("single-root validation should return Ok");
        assert!(
            errors.is_empty(),
            "lone capability should be a valid root, got {:?}",
            errors
        );
    }

    #[test]
    fn validate_single_root_hierarchy_rejects_split_specify() {
        let mut req = make_element("file.md#req", "Req", None);
        let cap1 = make_element("file.md#cap1", "Cap1", Some(ElementType::Capability));
        let cap2 = make_element("file.md#cap2", "Cap2", Some(ElementType::Capability));
        add_relation(&mut req, "specify", "file.md#cap1");
        add_relation(&mut req, "specify", "file.md#cap2");

        let mut registry = GraphRegistry::new();
        registry
            .register_element(req, "file.md")
            .expect("requirement should register");
        registry
            .register_element(cap1, "file.md")
            .expect("capability 1 should register");
        registry
            .register_element(cap2, "file.md")
            .expect("capability 2 should register");
        registry.build_relation_graph();

        let errors = registry
            .validate_single_root_hierarchy_ownership_in_memory()
            .expect("single-root validation should return Ok");
        assert!(
            !errors.is_empty(),
            "requirement specifying two capabilities should be rejected"
        );
        let mixed = errors.iter().any(|error| {
            matches!(
                error,
                ReqvireError::MixedHierarchicalRelations(message)
                    if message.contains("must resolve to exactly one owning capability")
            )
        });
        assert!(
            mixed,
            "expected a MixedHierarchicalRelations error, got {:?}",
            errors
        );
    }
}
