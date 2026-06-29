use super::*;

impl GraphRegistry {
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
        let mut visited = FxHashSet::default();
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
        let mut ontology_ids = BTreeSet::new();

        for target_id in self.concept_reference_target_ids(element) {
            let Some(target) = self.nodes.get(&target_id).map(|node| &node.element) else {
                continue;
            };
            if let Some(scheme_id) = self.concept_scheme_context_id(&target.identifier) {
                ontology_ids.insert(scheme_id);
            }
        }

        self.expand_ontology_context(ontology_ids)
    }

    pub fn build_concept_reference_element_context(&self, element_id: &str) -> Vec<String> {
        let Some(node) = self.nodes.get(element_id) else {
            return Vec::new();
        };
        let element = &node.element;
        self.expand_concept_context(self.concept_reference_target_ids(element))
    }

    fn concept_reference_target_ids(&self, element: &Element) -> BTreeSet<String> {
        if element.element_type.is_ontology() || element.element_type.is_semantic_contract() {
            return BTreeSet::new();
        }

        let (references, _) = crate::parser::extract_concept_references(&element.content);
        if references.is_empty() {
            return BTreeSet::new();
        }

        let mut concept_ids = BTreeSet::new();

        for reference in references {
            let Ok(target_id) = crate::parser::normalize_concept_reference_target(
                &element.file_path,
                &reference.target,
            ) else {
                continue;
            };
            if self
                .nodes
                .get(&target_id)
                .is_some_and(|node| node.element.element_type.is_concept())
            {
                concept_ids.insert(target_id);
            }
        }

        concept_ids
    }

    fn expand_concept_context(&self, concept_ids: BTreeSet<String>) -> Vec<String> {
        let mut context = BTreeSet::new();
        let mut stack: Vec<String> = concept_ids.into_iter().collect();

        while let Some(concept_id) = stack.pop() {
            if !context.insert(concept_id.clone()) {
                continue;
            }
            let Some(node) = self.nodes.get(&concept_id) else {
                continue;
            };
            for relation in &node.element.relations {
                if relation.relation_type.name != "derivedFrom" {
                    continue;
                }
                let LinkType::Identifier(target_id) = &relation.target.link else {
                    continue;
                };
                if self.nodes.get(target_id).is_some_and(|target| {
                    target.element.element_type.is_concept_scheme()
                        || target.element.element_type.is_concept()
                }) {
                    stack.push(target_id.clone());
                }
            }
        }

        context.into_iter().collect()
    }

    pub(super) fn expand_ontology_context(&self, ontology_ids: BTreeSet<String>) -> Vec<String> {
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

    pub(super) fn concept_scheme_namespace_context(
        &self,
        scheme: &Element,
    ) -> Option<(String, String)> {
        if !scheme.element_type.is_concept_scheme() {
            return None;
        }
        let base = scheme
            .metadata
            .get("concept_base")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())?;
        let prefix = scheme
            .metadata
            .get("concept_prefix")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())?;
        Some((base.to_string(), prefix.to_string()))
    }

    pub(super) fn materialize_concept_payload_context(&mut self) {
        let mut updates = Vec::new();
        let mut element_ids: Vec<String> = self.nodes.keys().cloned().collect();
        element_ids.sort();

        for element_id in element_ids {
            let Some(element) = self.nodes.get(&element_id).map(|node| &node.element) else {
                continue;
            };

            if element.element_type.is_concept_scheme() {
                let Some((base, prefix)) = self.concept_scheme_namespace_context(element) else {
                    continue;
                };
                let iri = format!(
                    "{}#{}",
                    base.trim_end_matches('#'),
                    concept_validation_local_name(&element.name)
                );
                updates.push(ConceptPayloadContextUpdate {
                    top_concepts: self.derived_top_concept_links(&element_id),
                    element_id,
                    iri,
                    scheme_iri: None,
                    namespace_base: base,
                    namespace_prefix: prefix,
                });
                continue;
            }

            if element.element_type.is_concept() {
                let Some(scheme_id) = self.concept_scheme_context_id(&element_id) else {
                    continue;
                };
                let Some(scheme) = self.nodes.get(&scheme_id).map(|node| &node.element) else {
                    continue;
                };
                let Some((base, prefix)) = self.concept_scheme_namespace_context(scheme) else {
                    continue;
                };
                let iri = format!(
                    "{}#{}",
                    base.trim_end_matches('#'),
                    concept_validation_local_name(&element.name)
                );
                let scheme_iri = format!(
                    "{}#{}",
                    base.trim_end_matches('#'),
                    concept_validation_local_name(&scheme.name)
                );
                updates.push(ConceptPayloadContextUpdate {
                    element_id,
                    iri,
                    scheme_iri: Some(scheme_iri),
                    namespace_base: base,
                    namespace_prefix: prefix,
                    top_concepts: Vec::new(),
                });
            }
        }

        for update in updates {
            let Some(node) = self.nodes.get_mut(&update.element_id) else {
                continue;
            };
            if let Some(scheme) = node.element.concept_scheme.as_mut() {
                scheme.iri = update.iri;
                scheme.namespace_base = Some(update.namespace_base);
                scheme.namespace_prefix = Some(update.namespace_prefix);
                scheme.top_concepts = update.top_concepts;
            } else if let Some(concept) = node.element.concept.as_mut() {
                concept.iri = update.iri;
                concept.scheme_iri = update.scheme_iri;
                concept.namespace_base = Some(update.namespace_base);
                concept.namespace_prefix = Some(update.namespace_prefix);
            }
        }
    }

    fn derived_top_concept_links(&self, scheme_id: &str) -> Vec<ConceptLink> {
        let mut links = Vec::new();
        for node in self.nodes.values() {
            let concept = &node.element;
            if !concept.element_type.is_concept() {
                continue;
            }
            if concept
                .concept
                .as_ref()
                .is_some_and(|payload| !payload.broader.is_empty())
            {
                continue;
            }
            let directly_under_scheme = concept.relations.iter().any(|relation| {
                relation.relation_type.name == "derivedFrom"
                    && matches!(&relation.target.link, LinkType::Identifier(target_id) if target_id == scheme_id)
            });
            if directly_under_scheme {
                links.push(ConceptLink {
                    relation: "hasTopConcept".to_string(),
                    label: concept.name.clone(),
                    target: concept.identifier.clone(),
                });
            }
        }
        links.sort_by(|a, b| a.label.cmp(&b.label).then(a.target.cmp(&b.target)));
        links
    }

    pub(super) fn concept_scheme_context_id(&self, element_id: &str) -> Option<String> {
        self.concept_scheme_context_element(element_id)
            .map(|element| element.identifier.clone())
    }

    pub(crate) fn concept_scheme_context_element(&self, element_id: &str) -> Option<&Element> {
        let mut visited = BTreeSet::new();
        self.concept_scheme_context_element_recursive(element_id, &mut visited)
    }

    fn concept_scheme_context_element_recursive(
        &self,
        element_id: &str,
        visited: &mut BTreeSet<String>,
    ) -> Option<&Element> {
        if !visited.insert(element_id.to_string()) {
            return None;
        }
        let element = self.nodes.get(element_id).map(|node| &node.element)?;
        if element.element_type.is_concept_scheme() {
            return Some(element);
        }
        if !element.element_type.is_concept() {
            return None;
        }
        for relation in &element.relations {
            if relation.relation_type.name != "derivedFrom" {
                continue;
            }
            let LinkType::Identifier(target_id) = &relation.target.link else {
                continue;
            };
            let target = self.nodes.get(target_id).map(|node| &node.element)?;
            if target.element_type.is_concept_scheme() {
                return Some(target);
            }
            if target.element_type.is_concept() {
                if let Some(scheme_id) =
                    self.concept_scheme_context_element_recursive(target_id, visited)
                {
                    return Some(scheme_id);
                }
            }
        }
        None
    }

    pub(super) fn concept_link_resolves_to_type<F>(&self, target: &str, predicate: F) -> bool
    where
        F: Fn(&Element) -> bool,
    {
        self.nodes
            .get(target)
            .map(|node| &node.element)
            .or_else(|| {
                self.nodes
                    .values()
                    .find(|node| node.element.identifier.ends_with(target))
                    .map(|node| &node.element)
            })
            .is_some_and(predicate)
    }

    pub(crate) fn resolve_concept_element_id(&self, target: &str) -> Option<String> {
        self.nodes
            .get(target)
            .filter(|node| node.element.element_type.is_concept())
            .map(|_| target.to_string())
            .or_else(|| {
                self.nodes
                    .values()
                    .find(|node| {
                        node.element.element_type.is_concept()
                            && node.element.identifier.ends_with(target)
                    })
                    .map(|node| node.element.identifier.clone())
            })
    }

    pub(super) fn valid_concept_mapping_target(&self, target: &str) -> bool {
        target.starts_with("http://")
            || target.starts_with("https://")
            || self.concept_link_resolves_to_type(target, |candidate| {
                candidate.element_type.is_concept()
            })
    }

    pub(crate) fn generated_concept_iri_for_element(&self, element: &Element) -> Option<String> {
        let scheme_id = if element.element_type.is_concept_scheme() {
            element.identifier.clone()
        } else if element.element_type.is_concept() {
            self.concept_scheme_context_id(&element.identifier)?
        } else {
            return None;
        };
        let scheme = self.nodes.get(&scheme_id).map(|node| &node.element)?;
        let (base, _prefix) = self.concept_scheme_namespace_context(scheme)?;
        Some(format!(
            "{}#{}",
            base.trim_end_matches('#'),
            concept_validation_local_name(&element.name)
        ))
    }
}
