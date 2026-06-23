// Search functionality for filtering and searching model elements
// This module implements the unified search command with comprehensive filtering

use crate::element;
use crate::error::ReqvireError;
use crate::graph_registry::GraphRegistry;
use crate::relation;
use globset::{Glob, GlobMatcher};
use regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;

/// Filters for searching and filtering elements
pub struct SearchFilters {
    file_glob: Option<GlobMatcher>,
    name_re: Option<Regex>,
    type_patterns: Option<Vec<String>>,
    status_values: Option<Vec<String>>,
    priority_values: Option<Vec<String>>,
    risk_values: Option<Vec<String>>,
    owner_re: Option<Regex>,
    content_re: Option<Regex>,
    page_content_re: Option<Regex>,
    have_relations: Vec<String>,
    not_have_relations: Vec<String>,
    has_contract_bindings: bool,
    contract_bindings_glob: Option<GlobMatcher>,
}

impl SearchFilters {
    /// Creates new search filters with validation
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        file: Option<&str>,
        name_regex: Option<&str>,
        typ: Option<&str>,
        status: Option<&str>,
        priority: Option<&str>,
        risk: Option<&str>,
        owner_regex: Option<&str>,
        content: Option<&str>,
        page_content: Option<&str>,
        have_relations: Option<&str>,
        not_have_relations: Option<&str>,
        has_contract_bindings: bool,
        contract_bindings: Option<&str>,
    ) -> Result<Self, ReqvireError> {
        fn compile_glob(pat: &str) -> Result<GlobMatcher, ReqvireError> {
            let glob = Glob::new(pat)
                .map_err(|e| ReqvireError::InvalidGlob(e.to_string()))?
                .compile_matcher();
            Ok(glob)
        }

        fn compile_regex(pattern: &str) -> Result<Regex, ReqvireError> {
            Regex::new(pattern).map_err(|e| ReqvireError::InvalidRegex(e.to_string()))
        }

        let file_glob = file.map(compile_glob).transpose()?;
        let name_re = name_regex.map(compile_regex).transpose()?;

        // Parse and validate comma-separated element types
        let type_patterns = if let Some(t) = typ {
            let types: Vec<String> = t.split(',').map(|s| s.trim().to_lowercase()).collect();

            // Validate each type
            for typ in &types {
                if !element::is_valid_element_type(typ) {
                    return Err(ReqvireError::ProcessError(format!(
                        "Invalid element type '{}'. Valid types: {}",
                        typ,
                        element::element_types_help()
                    )));
                }
            }

            Some(types)
        } else {
            None
        };

        fn parse_enum_values(
            property: &str,
            input: Option<&str>,
            accepted: &[&str],
        ) -> Result<Option<Vec<String>>, ReqvireError> {
            if let Some(input) = input {
                let values: Vec<String> = input
                    .split(',')
                    .map(|value| value.trim().to_lowercase())
                    .filter(|value| !value.is_empty())
                    .collect();

                for value in &values {
                    if !accepted.contains(&value.as_str()) {
                        return Err(ReqvireError::ProcessError(format!(
                            "Invalid governance metadata {} '{}'. Accepted values: {}.",
                            property,
                            value,
                            accepted.join(", ")
                        )));
                    }
                }

                Ok(Some(values))
            } else {
                Ok(None)
            }
        }

        let status_values = parse_enum_values("status", status, element::GOVERNANCE_STATUS_VALUES)?;
        let priority_values =
            parse_enum_values("priority", priority, element::GOVERNANCE_PRIORITY_VALUES)?;
        let risk_values = parse_enum_values("risk", risk, element::GOVERNANCE_RISK_VALUES)?;
        let owner_re = owner_regex.map(compile_regex).transpose()?;
        let content_re = content.map(compile_regex).transpose()?;
        let page_content_re = page_content.map(compile_regex).transpose()?;
        let contract_bindings_glob = contract_bindings.map(compile_glob).transpose()?;

        // Parse and validate comma-separated relation lists
        let have_relations = if let Some(s) = have_relations {
            let rels: Vec<String> = s.split(',').map(|r| r.trim().to_string()).collect();
            // Validate each relation type
            for rel in &rels {
                if !relation::RELATION_TYPES.contains_key(rel.as_str()) {
                    let valid_types: Vec<&str> = relation::RELATION_TYPES.keys().copied().collect();
                    return Err(ReqvireError::UnsupportedRelationType(format!(
                        "Invalid relation type '{}'. Valid types are: {}",
                        rel,
                        valid_types.join(", ")
                    )));
                }
            }
            rels
        } else {
            Vec::new()
        };

        let not_have_relations = if let Some(s) = not_have_relations {
            let rels: Vec<String> = s.split(',').map(|r| r.trim().to_string()).collect();
            // Validate each relation type
            for rel in &rels {
                if !relation::RELATION_TYPES.contains_key(rel.as_str()) {
                    let valid_types: Vec<&str> = relation::RELATION_TYPES.keys().copied().collect();
                    return Err(ReqvireError::UnsupportedRelationType(format!(
                        "Invalid relation type '{}'. Valid types are: {}",
                        rel,
                        valid_types.join(", ")
                    )));
                }
            }
            rels
        } else {
            Vec::new()
        };

        Ok(SearchFilters {
            file_glob,
            name_re,
            type_patterns,
            status_values,
            priority_values,
            risk_values,
            owner_re,
            content_re,
            page_content_re,
            have_relations,
            not_have_relations,
            has_contract_bindings,
            contract_bindings_glob,
        })
    }

    /// Check if element matches all filters
    pub fn matches(&self, elem: &element::Element, registry: &GraphRegistry) -> bool {
        // File glob filter
        if let Some(g) = &self.file_glob {
            if !g.is_match(&elem.file_path) {
                return false;
            }
        }

        // Name regex filter
        if let Some(re) = &self.name_re {
            if !re.is_match(&elem.name) {
                return false;
            }
        }

        // Type filter - element must match ANY of the specified types (OR logic)
        if let Some(types) = &self.type_patterns {
            let mut matches_any = false;

            for tp in types {
                let matches = if let Some(custom_type_name) = tp.strip_prefix("other-") {
                    // Handle "other-TYPENAME" pattern for custom types
                    // Strip "other-" prefix and compare with stored custom type name
                    match &elem.element_type {
                        element::ElementType::Other(actual_name) => {
                            actual_name.to_lowercase() == custom_type_name
                        }
                        _ => false, // Not an Other type
                    }
                } else {
                    let filter_type = element::ElementType::from_metadata(tp);
                    elem.element_type == filter_type
                };

                if matches {
                    matches_any = true;
                    break;
                }
            }

            if !matches_any {
                return false;
            }
        }

        let has_governance_filter = self.status_values.is_some()
            || self.priority_values.is_some()
            || self.risk_values.is_some()
            || self.owner_re.is_some();

        if has_governance_filter {
            let Some(governance) = registry.resolve_governance_metadata(elem) else {
                return false;
            };

            if let Some(values) = &self.status_values {
                if !values.contains(&governance.status.value) {
                    return false;
                }
            }

            if let Some(values) = &self.priority_values {
                if !values.contains(&governance.priority.value) {
                    return false;
                }
            }

            if let Some(values) = &self.risk_values {
                if !values.contains(&governance.risk.value) {
                    return false;
                }
            }

            if let Some(re) = &self.owner_re {
                if !re.is_match(&governance.owner.value) {
                    return false;
                }
            }
        }

        // Content regex filter
        if let Some(re) = &self.content_re {
            if !re.is_match(&elem.content) {
                return false;
            }
        }

        // Page content filter
        if let Some(re) = &self.page_content_re {
            if let Some(page) = registry.pages.get(&elem.file_path) {
                if !re.is_match(&page.frontmatter_content) {
                    return false;
                }
            } else {
                // No page content means it doesn't match
                return false;
            }
        }

        // Have relations filter - must have ALL specified relations
        if !self.have_relations.is_empty() {
            for required_rel in &self.have_relations {
                let has_relation = elem
                    .relations
                    .iter()
                    .any(|r| r.relation_type.name.eq_ignore_ascii_case(required_rel));
                if !has_relation {
                    return false;
                }
            }
        }

        // Not have relations filter - must NOT have ALL specified relations
        if !self.not_have_relations.is_empty() {
            for forbidden_rel in &self.not_have_relations {
                let has_relation = elem
                    .relations
                    .iter()
                    .any(|r| r.relation_type.name.eq_ignore_ascii_case(forbidden_rel));
                if has_relation {
                    return false;
                }
            }
        }

        // Has contract_bindings filter - must have at least one contract_bindings
        if self.has_contract_bindings && elem.contract_bindings.is_empty() {
            return false;
        }

        // ContractBindingEntry glob filter - must have a contract binding matching the glob
        if let Some(g) = &self.contract_bindings_glob {
            let has_matching_contract_binding = elem
                .contract_bindings
                .iter()
                .any(|a| g.is_match(a.target.as_str().as_str()));
            if !has_matching_contract_binding {
                return false;
            }
        }

        true
    }
}

/// Search result structures for JSON output
#[derive(Serialize)]
pub struct SearchResult {
    files: BTreeMap<String, FileSearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    global_counters: Option<GlobalSearchCounters>,
}

#[derive(Serialize)]
struct FileSearchResult {
    elements: Vec<ElementSearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_elements: Option<usize>,
}

#[derive(Serialize)]
struct ElementSearchResult {
    identifier: String,
    name: String,
    file: String,
    #[serde(rename = "type")]
    element_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    governance_metadata: Option<element::RequirementGovernanceMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_contract: Option<element::SemanticContract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ontology: Option<element::Ontology>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concept_scheme: Option<element::ConceptScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concept: Option<element::Concept>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    concept_references: Vec<element::ConceptReference>,
    relations: Vec<RelationSearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contract_bindings: Option<Vec<String>>,
}

#[derive(Serialize)]
struct RelationSearchResult {
    relation_type: String,
    target: TargetSearchResult,
}

#[derive(Serialize)]
struct TargetSearchResult {
    target: String,
    #[serde(rename = "type")]
    link_type: String,
}

#[derive(Serialize)]
struct GlobalSearchCounters {
    total_elements: usize,
    total_files: usize,
    total_ontology_types: BTreeMap<String, usize>,
    total_concept_types: BTreeMap<String, usize>,
    total_requirements_types: BTreeMap<String, usize>,
    total_semantic_contract_types: BTreeMap<String, usize>,
    total_verifications_types: BTreeMap<String, usize>,
    total_contracts_types: BTreeMap<String, usize>,
    total_governance_metadata: GovernanceMetadataCounters,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    total_other_types: BTreeMap<String, usize>,
}

#[derive(Serialize)]
struct GovernanceMetadataCounters {
    status: BTreeMap<String, usize>,
    priority: BTreeMap<String, usize>,
    risk: BTreeMap<String, usize>,
    owner: BTreeMap<String, usize>,
}

impl Default for GlobalSearchCounters {
    fn default() -> Self {
        // Initialize with all standard types at 0
        let mut requirements = BTreeMap::new();
        requirements.insert("capability".to_string(), 0);
        requirements.insert("system-requirement".to_string(), 0);

        let mut ontology = BTreeMap::new();
        ontology.insert("ontology".to_string(), 0);

        let mut concepts = BTreeMap::new();
        concepts.insert("concept-scheme".to_string(), 0);
        concepts.insert("concept".to_string(), 0);

        let mut semantic_contracts = BTreeMap::new();
        semantic_contracts.insert("semantic-contract".to_string(), 0);

        let mut verifications = BTreeMap::new();
        verifications.insert("test-verification".to_string(), 0);
        verifications.insert("formal-proof-verification".to_string(), 0);
        verifications.insert("analysis-verification".to_string(), 0);
        verifications.insert("inspection-verification".to_string(), 0);
        verifications.insert("demonstration-verification".to_string(), 0);

        let mut contracts = BTreeMap::new();
        contracts.insert("source".to_string(), 0);
        contracts.insert("behavior".to_string(), 0);
        contracts.insert("constraint".to_string(), 0);
        contracts.insert("specification".to_string(), 0);
        contracts.insert("state".to_string(), 0);
        contracts.insert("input-output".to_string(), 0);

        let mut governance_status = BTreeMap::new();
        for value in element::GOVERNANCE_STATUS_VALUES {
            governance_status.insert((*value).to_string(), 0);
        }

        let mut governance_priority = BTreeMap::new();
        for value in element::GOVERNANCE_PRIORITY_VALUES {
            governance_priority.insert((*value).to_string(), 0);
        }

        let mut governance_risk = BTreeMap::new();
        for value in element::GOVERNANCE_RISK_VALUES {
            governance_risk.insert((*value).to_string(), 0);
        }

        let mut governance_owner = BTreeMap::new();
        governance_owner.insert("unassigned".to_string(), 0);

        Self {
            total_elements: 0,
            total_files: 0,
            total_ontology_types: ontology,
            total_concept_types: concepts,
            total_requirements_types: requirements,
            total_semantic_contract_types: semantic_contracts,
            total_verifications_types: verifications,
            total_contracts_types: contracts,
            total_governance_metadata: GovernanceMetadataCounters {
                status: governance_status,
                priority: governance_priority,
                risk: governance_risk,
                owner: governance_owner,
            },
            total_other_types: BTreeMap::new(),
        }
    }
}

/// Generate search results with filtering
pub fn generate_search_report(
    registry: &GraphRegistry,
    filters: &SearchFilters,
    json_output: bool,
    short_mode: bool,
) -> Result<String, ReqvireError> {
    let result = build_search_result(registry, filters, short_mode);

    if json_output {
        serde_json::to_string_pretty(&result)
            .map_err(|e| ReqvireError::SerializationError(e.to_string()))
    } else {
        Ok(generate_search_text(&result, short_mode))
    }
}

fn build_search_result(
    registry: &GraphRegistry,
    filters: &SearchFilters,
    short_mode: bool,
) -> SearchResult {
    let mut files: BTreeMap<String, FileSearchResult> = BTreeMap::new();
    let mut counters = if short_mode {
        None
    } else {
        Some(GlobalSearchCounters::default())
    };

    for elem in registry.get_all_elements() {
        // Apply filters
        if !filters.matches(elem, registry) {
            continue;
        }

        // Count element if not in short mode
        if let Some(ref mut c) = counters {
            c.total_elements += 1;

            // Count by element type category
            match &elem.element_type {
                element::ElementType::Capability => {
                    *c.total_requirements_types
                        .entry("capability".to_string())
                        .or_insert(0) += 1;

                    if let Some(governance) = registry.resolve_governance_metadata(elem) {
                        *c.total_governance_metadata
                            .status
                            .entry(governance.status.value)
                            .or_insert(0) += 1;
                        *c.total_governance_metadata
                            .priority
                            .entry(governance.priority.value)
                            .or_insert(0) += 1;
                        *c.total_governance_metadata
                            .risk
                            .entry(governance.risk.value)
                            .or_insert(0) += 1;
                        let owner = if governance.owner.value.trim().is_empty() {
                            "unassigned".to_string()
                        } else {
                            governance.owner.value
                        };
                        *c.total_governance_metadata.owner.entry(owner).or_insert(0) += 1;
                    }
                }
                element::ElementType::Requirement(req_t) => {
                    let type_name = match req_t {
                        element::RequirementType::System => "system-requirement",
                    };
                    *c.total_requirements_types
                        .entry(type_name.to_string())
                        .or_insert(0) += 1;

                    if let Some(governance) = registry.resolve_governance_metadata(elem) {
                        *c.total_governance_metadata
                            .status
                            .entry(governance.status.value)
                            .or_insert(0) += 1;
                        *c.total_governance_metadata
                            .priority
                            .entry(governance.priority.value)
                            .or_insert(0) += 1;
                        *c.total_governance_metadata
                            .risk
                            .entry(governance.risk.value)
                            .or_insert(0) += 1;

                        let owner = if governance.owner.value.is_empty() {
                            "unassigned".to_string()
                        } else {
                            governance.owner.value
                        };
                        *c.total_governance_metadata.owner.entry(owner).or_insert(0) += 1;
                    }
                }
                element::ElementType::Ontology => {
                    *c.total_ontology_types
                        .entry("ontology".to_string())
                        .or_insert(0) += 1;
                }
                element::ElementType::ConceptScheme => {
                    *c.total_concept_types
                        .entry("concept-scheme".to_string())
                        .or_insert(0) += 1;
                }
                element::ElementType::Concept => {
                    *c.total_concept_types
                        .entry("concept".to_string())
                        .or_insert(0) += 1;
                }
                element::ElementType::Verification(ver_t) => {
                    let type_name = match ver_t {
                        element::VerificationType::Default => "test-verification",
                        element::VerificationType::Test => "test-verification",
                        element::VerificationType::FormalProof => "formal-proof-verification",
                        element::VerificationType::Analysis => "analysis-verification",
                        element::VerificationType::Inspection => "inspection-verification",
                        element::VerificationType::Demonstration => "demonstration-verification",
                    };
                    *c.total_verifications_types
                        .entry(type_name.to_string())
                        .or_insert(0) += 1;
                }
                element::ElementType::VerificationObjective => {
                    *c.total_verifications_types
                        .entry("verification-objective".to_string())
                        .or_insert(0) += 1;
                }
                element::ElementType::SemanticContract => {
                    *c.total_semantic_contract_types
                        .entry("semantic-contract".to_string())
                        .or_insert(0) += 1;
                }
                element::ElementType::Contract(ref_t) => {
                    let type_name = match ref_t {
                        element::ContractType::Source => "source",
                        element::ContractType::Constraint => "constraint",
                        element::ContractType::Behavior => "behavior",
                        element::ContractType::Specification => "specification",
                        element::ContractType::State => "state",
                        element::ContractType::InputOutput => "input-output",
                    };
                    *c.total_contracts_types
                        .entry(type_name.to_string())
                        .or_insert(0) += 1;
                }
                element::ElementType::Other(custom_type) => {
                    *c.total_other_types.entry(custom_type.clone()).or_insert(0) += 1;
                }
                element::ElementType::File => {}
            }
        }

        // Build relation summary
        let mut rels: Vec<RelationSearchResult> = elem
            .relations
            .iter()
            .map(|relation| {
                let (tgt, lt) = match &relation.target.link {
                    relation::LinkType::Identifier(id) => (id.clone(), "identifier".to_string()),
                    relation::LinkType::ExternalUrl(url) => {
                        (url.clone(), "external-url".to_string())
                    }
                    relation::LinkType::InternalPath(path) => (
                        path.to_string_lossy().to_string(),
                        "internal-path".to_string(),
                    ),
                };
                RelationSearchResult {
                    relation_type: relation.relation_type.name.to_string(),
                    target: TargetSearchResult {
                        target: tgt,
                        link_type: lt,
                    },
                }
            })
            .collect();

        // Sort relations for deterministic output (by relation_type, then by target)
        rels.sort_by(|a, b| {
            a.relation_type
                .cmp(&b.relation_type)
                .then_with(|| a.target.target.cmp(&b.target.target))
        });

        // Build contract_bindings list (omit in short mode)
        let contract_bindings = if short_mode {
            None
        } else {
            Some(
                elem.contract_bindings
                    .iter()
                    .map(|a| a.target.as_str())
                    .collect(),
            )
        };

        // Build element summary
        let es = ElementSearchResult {
            identifier: elem.identifier.clone(),
            name: elem.name.clone(),
            file: elem.file_path.clone(),
            element_type: elem.element_type.as_str().to_string(),
            content: if short_mode {
                None
            } else {
                Some(elem.content.clone())
            },
            governance_metadata: if short_mode {
                None
            } else {
                registry.resolve_governance_metadata(elem)
            },
            semantic_contract: if short_mode {
                None
            } else {
                elem.semantic_contract.clone()
            },
            ontology: if short_mode {
                None
            } else {
                elem.ontology.clone()
            },
            concept_scheme: if short_mode {
                None
            } else {
                elem.concept_scheme.clone()
            },
            concept: if short_mode {
                None
            } else {
                elem.concept.clone()
            },
            concept_references: if short_mode {
                Vec::new()
            } else {
                elem.concept_references.clone()
            },
            relations: rels,
            contract_bindings,
        };

        // Insert into flat file→elements map
        files
            .entry(elem.file_path.clone())
            .or_insert_with(|| FileSearchResult {
                elements: Vec::new(),
                page_content: None,
                total_elements: None,
            })
            .elements
            .push(es);
    }

    // Add page content and calculate counts (skip in short mode)
    if !short_mode {
        for (file_path, file_result) in &mut files {
            // Get page content
            if let Some(page) = registry.pages.get(file_path) {
                file_result.page_content = Some(page.frontmatter_content.clone());
            }

            file_result.total_elements = Some(file_result.elements.len());
        }
    }

    // Calculate global counts (skip in short mode)
    if let Some(ref mut c) = counters {
        c.total_files = files.len();
    }

    SearchResult {
        files,
        global_counters: counters,
    }
}

fn generate_search_text(result: &SearchResult, short_mode: bool) -> String {
    let mut output = String::new();

    if short_mode {
        // Short mode: one line per element
        for file_result in result.files.values() {
            for elem in &file_result.elements {
                output.push_str(&format!(
                    "[{}] {} - {}\n",
                    elem.element_type, elem.identifier, elem.name
                ));
            }
        }
    } else {
        // Full mode: detailed hierarchical output
        output.push_str("--- MBSE Search results ---\n");

        for (file, file_result) in &result.files {
            output.push_str(&format!(
                "📂 File: {} (elements: {})\n",
                file,
                file_result.total_elements.unwrap_or(0)
            ));

            // Show page content if available
            if let Some(page_content) = &file_result.page_content {
                if !page_content.trim().is_empty() {
                    output.push_str(&format!("  📄 Page content: {:?}\n", page_content));
                    output.push('\n');
                }
            }

            for elem in &file_result.elements {
                output.push_str(&format!("    🔹 Element: {}\n", elem.identifier));
                output.push_str(&format!("      - Name: {}\n", elem.name));
                output.push_str(&format!("      - File: {}\n", elem.file));
                output.push_str(&format!("      - Type: {}\n", elem.element_type));

                if let Some(content) = &elem.content {
                    output.push_str(&format!("      - Content: {:?}\n", content));
                }

                if elem.relations.is_empty() {
                    output.push_str("      - No relations.\n");
                } else {
                    output.push_str("      - Relations:\n");
                    for r in &elem.relations {
                        output.push_str(&format!(
                            "        ↪ {}: {} ({})\n",
                            r.relation_type, r.target.target, r.target.link_type
                        ));
                    }
                }
                output.push('\n');
            }
        }

        // Global counters
        if let Some(c) = &result.global_counters {
            output.push_str("------------------------------------\n");
            output.push_str("📊 Summary Counts:\n");
            output.push_str(&format!("Total files: {}\n", c.total_files));
            output.push_str(&format!("Total elements: {}\n", c.total_elements));
            output.push('\n');

            // Requirements types
            if !c.total_requirements_types.is_empty() {
                output.push_str("📋 Requirement Types:\n");
                for (type_name, count) in &c.total_requirements_types {
                    output.push_str(&format!("  {}: {}\n", type_name, count));
                }
                output.push('\n');
            }

            if !c.total_ontology_types.is_empty() {
                output.push_str("📋 Ontology Types:\n");
                for (type_name, count) in &c.total_ontology_types {
                    output.push_str(&format!("  {}: {}\n", type_name, count));
                }
                output.push('\n');
            }

            if !c.total_concept_types.is_empty() {
                output.push_str("📋 Concept Types:\n");
                for (type_name, count) in &c.total_concept_types {
                    output.push_str(&format!("  {}: {}\n", type_name, count));
                }
                output.push('\n');
            }

            output.push_str("📋 Requirement Governance Metadata:\n");
            output.push_str("  Status:\n");
            for (value, count) in &c.total_governance_metadata.status {
                output.push_str(&format!("    {}: {}\n", value, count));
            }
            output.push_str("  Priority:\n");
            for (value, count) in &c.total_governance_metadata.priority {
                output.push_str(&format!("    {}: {}\n", value, count));
            }
            output.push_str("  Risk:\n");
            for (value, count) in &c.total_governance_metadata.risk {
                output.push_str(&format!("    {}: {}\n", value, count));
            }
            output.push_str("  Owner:\n");
            for (value, count) in &c.total_governance_metadata.owner {
                output.push_str(&format!("    {}: {}\n", value, count));
            }
            output.push('\n');

            // Verification types
            if !c.total_verifications_types.is_empty() {
                output.push_str("📋 Verification Types:\n");
                for (type_name, count) in &c.total_verifications_types {
                    output.push_str(&format!("  {}: {}\n", type_name, count));
                }
                output.push('\n');
            }

            // Semantic contract types
            if !c.total_semantic_contract_types.is_empty() {
                output.push_str("📋 Semantic Contract Types:\n");
                for (type_name, count) in &c.total_semantic_contract_types {
                    output.push_str(&format!("  {}: {}\n", type_name, count));
                }
                output.push('\n');
            }

            // Contract types
            if !c.total_contracts_types.is_empty() {
                output.push_str("📋 Contract Types:\n");
                for (type_name, count) in &c.total_contracts_types {
                    output.push_str(&format!("  {}: {}\n", type_name, count));
                }
                output.push('\n');
            }

            // Other/custom types
            if !c.total_other_types.is_empty() {
                output.push_str("📋 Other Types:\n");
                for (type_name, count) in &c.total_other_types {
                    output.push_str(&format!("  {}: {}\n", type_name, count));
                }
                output.push('\n');
            }
        }
    }

    output
}
