// Search functionality for filtering and searching model elements
// This module implements the unified search command with comprehensive filtering

use crate::element;
use crate::graph_registry::GraphRegistry;
use crate::relation;
use crate::error::ReqvireError;
use globset::{Glob, GlobMatcher};
use regex::Regex;
use serde::Serialize;
use std::collections::BTreeMap;

/// Filters for searching and filtering elements
pub struct SearchFilters {
    file_glob: Option<GlobMatcher>,
    name_re: Option<Regex>,
    type_pat: Option<String>,
    content_re: Option<Regex>,
    page_content_re: Option<Regex>,
    have_relations: Vec<String>,
    not_have_relations: Vec<String>,
    has_attachments: bool,
    attachment_glob: Option<GlobMatcher>,
}

impl SearchFilters {
    /// Creates new search filters with validation
    pub fn new(
        file: Option<&str>,
        name_regex: Option<&str>,
        typ: Option<&str>,
        content: Option<&str>,
        page_content: Option<&str>,
        have_relations: Option<&str>,
        not_have_relations: Option<&str>,
        has_attachments: bool,
        attachment: Option<&str>,
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

        let file_glob = file.map(|p| compile_glob(p)).transpose()?;
        let name_re = name_regex.map(|r| compile_regex(r)).transpose()?;
        let type_pat = typ.map(|s| s.to_lowercase());
        let content_re = content.map(|r| compile_regex(r)).transpose()?;
        let page_content_re = page_content.map(|r| compile_regex(r)).transpose()?;
        let attachment_glob = attachment.map(|p| compile_glob(p)).transpose()?;

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
            type_pat,
            content_re,
            page_content_re,
            have_relations,
            not_have_relations,
            has_attachments,
            attachment_glob,
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

        // Type filter
        if let Some(tp) = &self.type_pat {
            let filter_type = element::ElementType::from_metadata(tp);
            if &elem.element_type != &filter_type {
                return false;
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
                let has_relation = elem.relations.iter().any(|r| {
                    r.relation_type.name.eq_ignore_ascii_case(required_rel)
                });
                if !has_relation {
                    return false;
                }
            }
        }

        // Not have relations filter - must NOT have ALL specified relations
        if !self.not_have_relations.is_empty() {
            for forbidden_rel in &self.not_have_relations {
                let has_relation = elem.relations.iter().any(|r| {
                    r.relation_type.name.eq_ignore_ascii_case(forbidden_rel)
                });
                if has_relation {
                    return false;
                }
            }
        }

        // Has attachments filter - must have at least one attachment
        if self.has_attachments && elem.attachments.is_empty() {
            return false;
        }

        // Attachment glob filter - must have an attachment matching the glob
        if let Some(g) = &self.attachment_glob {
            let has_matching_attachment = elem.attachments.iter()
                .any(|a| g.is_match(a.target.as_str().as_str()));
            if !has_matching_attachment {
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
    verified_relations_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    satisfied_relations_count: Option<usize>,
    relations: Vec<RelationSearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<String>>,
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
    total_requirements_system: usize,
    total_requirements_user: usize,
    total_verifications_test: usize,
    total_verifications_analysis: usize,
    total_verifications_inspection: usize,
    total_verifications_demonstration: usize,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    custom_element_types: BTreeMap<String, usize>,
}

impl Default for GlobalSearchCounters {
    fn default() -> Self {
        Self {
            total_elements: 0,
            total_files: 0,
            total_requirements_system: 0,
            total_requirements_user: 0,
            total_verifications_test: 0,
            total_verifications_analysis: 0,
            total_verifications_inspection: 0,
            total_verifications_demonstration: 0,
            custom_element_types: BTreeMap::new(),
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

            // Count by element type
            match &elem.element_type {
                element::ElementType::Requirement(req_t) => {
                    match req_t {
                        element::RequirementType::System => c.total_requirements_system += 1,
                        element::RequirementType::User => c.total_requirements_user += 1,
                    }
                }
                element::ElementType::Verification(ver_t) => {
                    match ver_t {
                        element::VerificationType::Default => c.total_verifications_test += 1,
                        element::VerificationType::Test => c.total_verifications_test += 1,
                        element::VerificationType::Analysis => c.total_verifications_analysis += 1,
                        element::VerificationType::Inspection => c.total_verifications_inspection += 1,
                        element::VerificationType::Demonstration => c.total_verifications_demonstration += 1,
                    }
                }
                element::ElementType::Refinement(ref_t) => {
                    // Count refinement types as custom element types for now
                    let type_name = match ref_t {
                        element::RefinementType::Constraint => "constraint",
                        element::RefinementType::Behavior => "behavior",
                        element::RefinementType::Specification => "specification",
                    };
                    *c.custom_element_types.entry(type_name.to_string()).or_insert(0) += 1;
                }
                element::ElementType::Other(custom_type) => {
                    *c.custom_element_types.entry(custom_type.clone()).or_insert(0) += 1;
                }
                element::ElementType::File => {}
            }
        }

        // Build relation summary
        let mut rels: Vec<RelationSearchResult> = elem.relations.iter()
            .map(|relation| {
                let (tgt, lt) = match &relation.target.link {
                    relation::LinkType::Identifier(id) => (id.clone(), "identifier".to_string()),
                    relation::LinkType::ExternalUrl(url) => (url.clone(), "external-url".to_string()),
                    relation::LinkType::InternalPath(path) => {
                        (path.to_string_lossy().to_string(), "internal-path".to_string())
                    }
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
            a.relation_type.cmp(&b.relation_type)
                .then_with(|| a.target.target.cmp(&b.target.target))
        });

        // Count verified and satisfied relations for this element (only if not short mode)
        let (vc, sc) = if short_mode {
            (None, None)
        } else {
            let verified_count = elem.relations.iter()
                .filter(|r| relation::is_verification_relation(r.relation_type))
                .count();
            let satisfied_count = elem.relations.iter()
                .filter(|r| relation::is_satisfaction_relation(r.relation_type))
                .count();
            (Some(verified_count), Some(satisfied_count))
        };

        // Build attachments list (omit in short mode)
        let attachments = if short_mode {
            None
        } else {
            Some(elem.attachments
                .iter()
                .map(|a| a.target.as_str())
                .collect())
        };

        // Build element summary
        let es = ElementSearchResult {
            identifier: elem.identifier.clone(),
            name: elem.name.clone(),
            file: elem.file_path.clone(),
            element_type: elem.element_type.as_str().to_string(),
            content: if short_mode { None } else { Some(elem.content.clone()) },
            verified_relations_count: vc,
            satisfied_relations_count: sc,
            relations: rels,
            attachments,
        };

        // Insert into flat file→elements map
        files.entry(elem.file_path.clone())
            .or_insert_with(|| FileSearchResult {
                elements: Vec::new(),
                page_content: None,
                total_elements: None,
            })
            .elements.push(es);
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
                output.push_str(&format!("[{}] {} - {}\n",
                    elem.element_type, elem.identifier, elem.name));
            }
        }
    } else {
        // Full mode: detailed hierarchical output
        output.push_str("--- MBSE Search results ---\n");

        for (file, file_result) in &result.files {
            output.push_str(&format!("📂 File: {} (elements: {})\n",
                file,
                file_result.total_elements.unwrap_or(0)));

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
                if let Some(vc) = elem.verified_relations_count {
                    output.push_str(&format!("      - Verified relations count: {}\n", vc));
                }
                if let Some(sc) = elem.satisfied_relations_count {
                    output.push_str(&format!("      - Satisfied relations count: {}\n", sc));
                }

                if elem.relations.is_empty() {
                    output.push_str("      - No relations.\n");
                } else {
                    output.push_str("      - Relations:\n");
                    for r in &elem.relations {
                        output.push_str(&format!("        ↪ {}: {} ({})\n",
                            r.relation_type, r.target.target, r.target.link_type));
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
            output.push_str("📋 Element Types:\n");
            output.push_str(&format!("System Requirements: {}\n", c.total_requirements_system));
            output.push_str(&format!("User Requirements: {}\n", c.total_requirements_user));
            output.push_str(&format!("Verifications (Test): {}\n", c.total_verifications_test));
            output.push_str(&format!("Verifications (Analysis): {}\n", c.total_verifications_analysis));
            output.push_str(&format!("Verifications (Inspection): {}\n", c.total_verifications_inspection));
            output.push_str(&format!("Verifications (Demonstration): {}\n", c.total_verifications_demonstration));

            if !c.custom_element_types.is_empty() {
                let mut custom_types: Vec<_> = c.custom_element_types.iter().collect();
                custom_types.sort_by_key(|(type_name, _)| *type_name);
                for (type_name, count) in custom_types {
                    output.push_str(&format!("Custom ({}): {}\n", type_name, count));
                }
            }

        }
    }

    output
}
