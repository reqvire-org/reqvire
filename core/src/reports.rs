use crate::element;
use crate::element_registry::ElementRegistry;
use serde::Serialize;
use std::collections::HashMap;
use crate::error::ReqvireError;
use crate::relation;
use globset::{Glob, GlobMatcher};
use regex::Regex;

pub struct Filters {
    file_glob:    Option<GlobMatcher>,
    name_re:      Option<Regex>,
    section_glob: Option<GlobMatcher>,
    type_pat:     Option<String>,
    content_re:   Option<Regex>,
    not_verified: bool,
    not_satisfied: bool,
}

impl Filters {
    /// Builds a Filters struct, or returns a ReqvireError::InvalidGlob / InvalidRegex
    pub fn new(
        file: Option<&str>,
        name_regex: Option<&str>,
        section: Option<&str>,
        typ: Option<&str>,
        content: Option<&str>,
        is_not_verified: bool,
        is_not_satisfied: bool,
    ) -> Result<Self, ReqvireError> {
        fn compile_glob(pat: &str) -> Result<GlobMatcher, ReqvireError> {
            let glob =Glob::new(pat)
                .map_err(|e| ReqvireError::InvalidGlob(e.to_string()))?
                .compile_matcher();
            Ok(glob)

        }

        let file_glob = file.map(|p| compile_glob(p)).transpose()?;
        let name_re = match name_regex {
            Some(r) => Some(Regex::new(r).map_err(|e| ReqvireError::InvalidRegex(e.to_string()))?),
            None => None,
        };
        let section_glob = section.map(|p| compile_glob(p)).transpose()?;
        let type_pat = typ.map(|s| s.to_lowercase());
        let content_re = match content {
            Some(r) => Some(Regex::new(r).map_err(|e| ReqvireError::InvalidRegex(e.to_string()))?),
            None => None,
        };

        Ok(Filters {
            file_glob,
            name_re,
            section_glob,
            type_pat,
            content_re,
            not_verified: is_not_verified,
            not_satisfied: is_not_satisfied,
        })
    }

    /// Returns true if this element passes *all* of the user’s filters.
    pub fn matches(&self, e: &element::Element) -> bool {
        // 1) file glob
        if let Some(g) = &self.file_glob {
            if !g.is_match(&e.file_path) {
                return false;
            }
        }
        // 2) name regex
        if let Some(re) = &self.name_re {
            if !re.is_match(&e.name) {
                return false;
            }
        }
        // 3) section glob
        if let Some(g) = &self.section_glob {
            if !g.is_match(&e.section) {
                return false;
            }
        }
        // 4) type filter
        if let Some(tp) = &self.type_pat {
            let filter_type = element::ElementType::from_metadata(tp);
            if &e.element_type != &filter_type {
                return false;
            }
        }
        // 5) content regex
        if let Some(re) = &self.content_re {
            let text = e.content.clone();
            if !re.is_match(&text) {
                return false;
            }
        }

        // Pre-compute verify/satisfy counts for later filters
        let verified_count = e.relations.iter()
            .filter(|r| relation::is_verification_relation(r.relation_type))
            .count();

        let satisfied_count = e.relations.iter()
            .filter(|r| relation::is_satisfaction_relation(r.relation_type))
            .count();
            
            
        // 6) not_verified: exclude any element that *has* a verified relation
        if self.not_verified && verified_count > 0 {
            return false;
        }
        // 7) not_satisfied: exclude any element that *has* a satisfied relation
        if self.not_satisfied && satisfied_count > 0 {
            return false;
        }

        // passed all filters
        true
    }
}

#[derive(Serialize)]
struct Summary {
    files: HashMap<String, FileSummary>,
    global_counters: GlobalCounters,
}

#[derive(Serialize)]
struct FileSummary {
    sections: HashMap<String, SectionSummary>,
}

#[derive(Serialize)]
struct SectionSummary {
    elements: Vec<ElementSummary>,
}

#[derive(Serialize)]
struct ElementSummary {
    identifier: String,
    name: String,
    section: String,
    file: String,
    #[serde(rename = "type")]
    element_type: String,
    content: String,
    verified_relations_count: usize,
    satisfied_relations_count: usize,
    relations: Vec<RelationSummary>,
}

#[derive(Serialize)]
struct RelationSummary {
    relation_type: String,
    target: TargetSummary,
}

#[derive(Serialize)]
struct TargetSummary {
    target: String,
    #[serde(rename = "type")]
    link_type: String,
}

#[derive(Serialize, Default)]
struct GlobalCounters {
    total_elements: usize,

    // Requirements by type
    total_requirements_system: usize,
    total_requirements_user: usize,

    // Verifications by type
    total_verifications_test: usize,
    total_verifications_analysis: usize,
    total_verifications_inspection: usize,
    total_verifications_demonstration: usize,

    // All requirements missing relations
    requirements_not_verified: usize,
    requirements_not_satisfied: usize,
}

pub fn print_registry_summary(
    registry: &ElementRegistry,
    as_json: bool,
    filters: &Filters,
) {
    let summary = build_summary(registry, filters);
    if as_json {
        let text = serde_json::to_string_pretty(&summary)
            .expect("failed to serialize summary");
        println!("{}", text);
    } else {
        print_summary_text(&summary);
    }
}

fn build_summary(
    registry: &ElementRegistry,
    filters: &Filters,
) -> Summary {
    let mut files: HashMap<String, FileSummary> = HashMap::new();
    let mut counters = GlobalCounters::default();

    for elem in registry.elements.values() {
        // Apply filters
        if !filters.matches(elem) {
            continue;
        }

        counters.total_elements += 1;

        // Count by element type, and accumulate missing‐relations for all requirements
        let (vc, sc) = match &elem.element_type {
            element::ElementType::Requirement(req_t) => {
                match req_t {
                    element::RequirementType::System => counters.total_requirements_system += 1,
                    element::RequirementType::User   => counters.total_requirements_user += 1,
                }

                    
                let vcount = elem.relations.iter()
                    .filter(|r| relation::is_verification_relation(r.relation_type))
                    .count();

                let scount = elem.relations.iter()
                    .filter(|r| relation::is_satisfaction_relation(r.relation_type))
                    .count();
                        
                if vcount == 0 {
                    counters.requirements_not_verified += 1;
                }
                if scount == 0 {
                    counters.requirements_not_satisfied += 1;
                }
                (vcount, scount)
            }
            element::ElementType::Verification(ver_t) => {
                match ver_t {
                    element::VerificationType::Default       => counters.total_verifications_test += 1,                
                    element::VerificationType::Test          => counters.total_verifications_test += 1,
                    element::VerificationType::Analysis      => counters.total_verifications_analysis += 1,
                    element::VerificationType::Inspection    => counters.total_verifications_inspection += 1,
                    element::VerificationType::Demonstration => counters.total_verifications_demonstration += 1,
                }
                (0, 0)
            }
            _ => (0, 0),
        };

        // Build a Vec<RelationSummary> rather than raw JSON values
        let rels: Vec<RelationSummary> = elem.relations.iter()
            .map(|relation| {
                let (tgt, lt) = match &relation.target.link {
                    relation::LinkType::Identifier(id)   => (id.clone(), "Identifier".to_string()),
                    relation::LinkType::ExternalUrl(url) => (url.clone(), "External URL".to_string()),
                };
                RelationSummary {
                    relation_type: relation.relation_type.name.to_string(),
                    target: TargetSummary { target: tgt, link_type: lt },
                }
            })
            .collect();

        // Assemble the element summary
        let es = ElementSummary {
            identifier: elem.identifier.clone(),
            name: elem.name.clone(),
            section: elem.section.clone(),
            file: elem.file_path.clone(),
            element_type: elem.element_type.as_str().to_string(),
            content: elem.content.clone(),
            verified_relations_count: vc,
            satisfied_relations_count: sc,
            relations: rels,
        };

        // Insert into nested file→section map
        files.entry(elem.file_path.clone())
            .or_insert_with(|| FileSummary { sections: HashMap::new() })
            .sections.entry(elem.section.clone())
            .or_insert_with(|| SectionSummary { elements: Vec::new() })
            .elements.push(es);
    }

    Summary {
        files,
        global_counters: counters,
    }
}
fn print_summary_text(summary: &Summary) {
    println!("--- MBSE Model summary ---");
    for (file, fsum) in &summary.files {
        println!("📂 File: {}", file);
        for (sec, ssum) in &fsum.sections {
            println!("  📖 Section: {}", sec);
            for e in &ssum.elements {
                println!("    🔹 Element: {}", e.identifier);
                println!("      - Name: {}", e.name);
                println!("      - Section: {}", e.section);
                println!("      - File: {}", e.file);
                println!("      - Type: {}", e.element_type);
                println!("      - Content: {:?}", e.content);
                println!("      - Verified relations count: {}", e.verified_relations_count);
                println!("      - Satisfied relations count: {}", e.satisfied_relations_count);
                if e.relations.is_empty() {
                    println!("      - No relations.");
                } else {
                    println!("      - Relations:");
                    for r in &e.relations {
                        println!("        ↪ {}: {} ({})",
                            r.relation_type, r.target.target, r.target.link_type);
                    }
                }
                println!();
            }
        }
    }

    let c = &summary.global_counters;
    println!("------------------------------------");
    println!("Total elements: {}", c.total_elements);
    println!("Total System Requirements: {}", c.total_requirements_system);
    println!("Total User Requirements: {}", c.total_requirements_user);
    println!("Total Verifications (Test): {}", c.total_verifications_test);
    println!("TotalVerifications (Analysis): {}", c.total_verifications_analysis);
    println!("TotalVerifications (Inspection): {}", c.total_verifications_inspection);
    println!("TotalVerifications (Demonstration): {}", c.total_verifications_demonstration);
    println!("Requirements not verified: {}", c.requirements_not_verified);
    println!("Requirements not satisfied: {}", c.requirements_not_satisfied);
}
