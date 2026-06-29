use super::*;

impl GraphRegistry {
    /// Check if an element is in the derivation hierarchy of a root element.
    /// Returns true if element_id is the root itself, an ancestor, or a descendant of root_id.
    /// Used for contract_bindings scope validation to check hierarchical independence.
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
        let mut visited = FxHashSet::default();
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
        visited: &mut FxHashSet<String>,
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

    /// Get the owner requirements for a file contract_bindings (via satisfiedBy or definedBy relation).
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

    /// Find if the same contract_bindings exists in hierarchy (ancestor or descendant).
    /// Returns (direction, element_id) where direction is "ancestor" or "descendant".
    pub fn find_duplicate_contract_bindings_in_hierarchy(
        &self,
        element_id: &str,
        contract_bindings: &crate::element::ContractBindingTarget,
    ) -> Option<(&'static str, String)> {
        // Check ancestors
        if let Some(ancestor) =
            self.find_contract_bindings_in_ancestors(element_id, contract_bindings)
        {
            return Some(("ancestor", ancestor));
        }
        // Check descendants
        if let Some(descendant) =
            self.find_contract_bindings_in_descendants(element_id, contract_bindings)
        {
            return Some(("descendant", descendant));
        }
        None
    }

    fn find_contract_bindings_in_ancestors(
        &self,
        element_id: &str,
        contract_bindings: &crate::element::ContractBindingTarget,
    ) -> Option<String> {
        let hierarchical_types = get_hierarchical_relation_types();
        let mut visited = FxHashSet::default();
        self.find_contract_bindings_in_ancestors_recursive(
            element_id,
            contract_bindings,
            &hierarchical_types,
            &mut visited,
        )
    }

    fn find_contract_bindings_in_ancestors_recursive(
        &self,
        element_id: &str,
        contract_bindings: &crate::element::ContractBindingTarget,
        hierarchical_types: &[&str],
        visited: &mut FxHashSet<String>,
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
                            // Check if parent has this contract_bindings
                            if parent_node.element.contract_bindings.iter().any(|a| {
                                self.contract_bindings_targets_equal(&a.target, contract_bindings)
                            }) {
                                return Some(parent_id.clone());
                            }
                        }
                        // Check ancestors recursively
                        if let Some(found) = self.find_contract_bindings_in_ancestors_recursive(
                            parent_id,
                            contract_bindings,
                            hierarchical_types,
                            visited,
                        ) {
                            return Some(found);
                        }
                    }
                }
            }
        }
        None
    }

    fn find_contract_bindings_in_descendants(
        &self,
        element_id: &str,
        contract_bindings: &crate::element::ContractBindingTarget,
    ) -> Option<String> {
        let mut visited = FxHashSet::default();
        self.find_contract_bindings_in_descendants_recursive(
            element_id,
            contract_bindings,
            &mut visited,
        )
    }

    fn find_contract_bindings_in_descendants_recursive(
        &self,
        element_id: &str,
        contract_bindings: &crate::element::ContractBindingTarget,
        visited: &mut FxHashSet<String>,
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
                // Check if child has this contract_bindings
                if child_node
                    .element
                    .contract_bindings
                    .iter()
                    .any(|a| self.contract_bindings_targets_equal(&a.target, contract_bindings))
                {
                    return Some(child_id.clone());
                }
                // Check descendants recursively
                if let Some(found) = self.find_contract_bindings_in_descendants_recursive(
                    child_id,
                    contract_bindings,
                    visited,
                ) {
                    return Some(found);
                }
            }
        }
        None
    }

    fn contract_bindings_targets_equal(
        &self,
        a: &crate::element::ContractBindingTarget,
        b: &crate::element::ContractBindingTarget,
    ) -> bool {
        match (a, b) {
            (
                crate::element::ContractBindingTarget::FilePath(p1),
                crate::element::ContractBindingTarget::FilePath(p2),
            ) => p1 == p2,
            (
                crate::element::ContractBindingTarget::ElementIdentifier(id1),
                crate::element::ContractBindingTarget::ElementIdentifier(id2),
            ) => id1 == id2,
            _ => false,
        }
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
        let mut visited = FxHashSet::default();
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

                let has_parent = element.relations.iter().any(|r| {
                    if !hierarchical_relations.contains(&r.relation_type.name) {
                        return false;
                    }
                    if element.element_type.is_concept_scheme() {
                        return match &r.target.link {
                            LinkType::Identifier(target_id) => {
                                self.nodes.get(target_id).is_some_and(|target| {
                                    target.element.element_type.is_concept_scheme()
                                })
                            }
                            _ => false,
                        };
                    }
                    true
                });

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
    pub fn get_internal_path_targets(&self) -> FxHashSet<PathBuf> {
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

    /// Collects all InternalPath targets from element relations
    pub fn collect_internal_path_targets(&self) -> FxHashSet<PathBuf> {
        let mut internal_paths = FxHashSet::default();

        for node in self.nodes.values() {
            for relation in &node.element.relations {
                if let LinkType::InternalPath(ref path) = relation.target.link {
                    internal_paths.insert(path.clone());
                }
            }
        }

        internal_paths
    }
}
