use crate::diff::{generate_file_diff, FileDiff};
use crate::element::{Element, ElementType, REUSED_CONTRACT_CONTEXT_SECTION};
use crate::error::ReqvireError;
use crate::filesystem;
use crate::graph_registry::{ElementNode, GraphRegistry};
use crate::relation::{LinkType, Relation, RELATION_TYPES};
use crate::utils;
use globset::GlobSet;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum MigrationSafety {
    Automatic,
    ReviewRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MigrationCandidate {
    pub id: &'static str,
    pub from_version: &'static str,
    pub to_version: &'static str,
    pub safety: MigrationSafety,
    pub summary: &'static str,
    pub dry_run_hint: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MigrationPlan {
    pub candidates: Vec<MigrationCandidate>,
}

impl MigrationPlan {
    pub fn empty() -> Self {
        Self {
            candidates: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }
}

pub const VERIFICATION_OBJECTIVE_MIGRATION_ID: &str = "v0.16-verification-objective";
pub const VERIFICATION_OBJECTIVE_HOLDER_FILE: &str = "VerificationObjectiveMigration.md";
pub const VERIFICATION_OBJECTIVE_HOLDER_NAME: &str = "Verification Objective";
pub const VERIFICATION_OBJECTIVE_HOLDER_ID: &str =
    "VerificationObjectiveMigration.md#verification-objective";
pub const DOCUMENTS_HEADER_MIGRATION_ID: &str = "v0.15-documents-to-element-header";
pub const CONTRACT_RELATION_MIGRATION_ID: &str = "v1.0-contract-relations";
pub const REUSED_CONTRACT_CONTEXT_SECTION_MIGRATION_ID: &str =
    "v1.1-reused-contract-context-section";
pub const CONCEPT_REFERENCE_LINK_MIGRATION_ID: &str = "v1.2-concept-reference-links";
const LEGACY_ATTACHMENTS_SECTION: &str = "Attachments";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerificationObjectiveMigrationSummary {
    pub migration_id: &'static str,
    pub holder_file: &'static str,
    pub objectives_created: usize,
    pub derive_relations_added: usize,
    pub affected_verifications: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DocumentsHeaderMigrationSummary {
    pub migration_id: &'static str,
    pub files_changed: usize,
    pub affected_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReusedContractContextSectionMigrationSummary {
    pub migration_id: &'static str,
    pub files_changed: usize,
    pub affected_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContractRelationMigrationSummary {
    pub migration_id: &'static str,
    pub relations_rewritten: usize,
    pub affected_files: Vec<String>,
    pub affected_elements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConceptReferenceLinkMigrationSummary {
    pub migration_id: &'static str,
    pub references_rewritten: usize,
    pub affected_files: Vec<String>,
}

pub fn candidates_for_validation_errors(errors: &[ReqvireError]) -> MigrationPlan {
    let mut candidates = Vec::new();

    if errors.iter().any(is_verification_objective_migration_error) {
        candidates.push(MigrationCandidate {
            id: VERIFICATION_OBJECTIVE_MIGRATION_ID,
            from_version: "0.16",
            to_version: "0.17",
            safety: MigrationSafety::ReviewRequired,
            summary: "Convert verification planning elements to verification-objective and keep concrete verifications as verify/evidence carriers.",
            dry_run_hint: "A dry run should report affected verification-family elements, verify/verifiedBy edges, satisfiedBy evidence links, and coverage/report expectations before applying source edits.",
        });
    }

    if errors.iter().any(is_contract_relation_migration_error) {
        candidates.push(MigrationCandidate {
            id: CONTRACT_RELATION_MIGRATION_ID,
            from_version: "0.x",
            to_version: "1.0",
            safety: MigrationSafety::Automatic,
            summary: "Rewrite legacy contract relation names to requirement-owned contract relation names: refinedBy -> definedBy and refine -> define.",
            dry_run_hint: "A dry run should show every relation key rewrite before applying source edits.",
        });
    }

    if errors.iter().any(is_concept_reference_link_migration_error) {
        candidates.push(MigrationCandidate {
            id: CONCEPT_REFERENCE_LINK_MIGRATION_ID,
            from_version: "1.1",
            to_version: "1.2",
            safety: MigrationSafety::Automatic,
            summary: "Rewrite legacy Concept References entries from Label: IRI to Markdown links that target native concept elements.",
            dry_run_hint: "A dry run should show every concept-reference line rewritten before applying source edits.",
        });
    }

    MigrationPlan { candidates }
}

fn is_verification_objective_migration_error(error: &ReqvireError) -> bool {
    let message = error.to_string();
    message.contains("verification-objective")
        || (message.contains("verifiedBy") && message.contains("verification"))
        || (message.contains("verify") && message.contains("verification"))
}

fn is_contract_relation_migration_error(error: &ReqvireError) -> bool {
    let message = error.to_string();
    message.contains("legacy relation 'refinedBy'")
        || message.contains("legacy relation 'refine'")
        || message.contains("refinedBy -> definedBy")
        || message.contains("refine -> define")
}

fn is_concept_reference_link_migration_error(error: &ReqvireError) -> bool {
    let message = error.to_string();
    message.contains("Concept References line")
        && message.contains("Markdown link to a native concept element")
}

pub fn apply_contract_relation_migration(
    registry: &mut GraphRegistry,
) -> Result<ContractRelationMigrationSummary, ReqvireError> {
    let defined_by = RELATION_TYPES
        .get("definedBy")
        .ok_or_else(|| ReqvireError::UnsupportedRelationType("definedBy".to_string()))?;
    let define = RELATION_TYPES
        .get("define")
        .ok_or_else(|| ReqvireError::UnsupportedRelationType("define".to_string()))?;

    let mut relations_rewritten = 0;
    let mut affected_files = Vec::new();
    let mut affected_elements = Vec::new();

    let mut sorted_ids: Vec<_> = registry.nodes.keys().cloned().collect();
    sorted_ids.sort();

    for id in sorted_ids {
        let Some(node) = registry.nodes.get_mut(&id) else {
            continue;
        };

        let mut element_changed = false;
        for relation in &mut node.element.relations {
            if !relation.user_created {
                continue;
            }

            let replacement = match relation.relation_type.name {
                "refinedBy" => defined_by,
                "refine" => define,
                _ => continue,
            };
            relation.relation_type = replacement;
            relations_rewritten += 1;
            element_changed = true;
        }

        for relation_node in &mut node.relations {
            relation_node.relation_trigger = match relation_node.relation_trigger.as_str() {
                "refinedBy" => "definedBy".to_string(),
                "refine" => "define".to_string(),
                _ => relation_node.relation_trigger.clone(),
            };
        }

        if element_changed {
            affected_files.push(node.element.file_path.clone());
            affected_elements.push(node.element.identifier.clone());
        }
    }

    affected_files.sort();
    affected_files.dedup();
    affected_elements.sort();
    affected_elements.dedup();

    Ok(ContractRelationMigrationSummary {
        migration_id: CONTRACT_RELATION_MIGRATION_ID,
        relations_rewritten,
        affected_files,
        affected_elements,
    })
}

pub fn apply_verification_objective_holders(
    registry: &mut GraphRegistry,
) -> Result<VerificationObjectiveMigrationSummary, ReqvireError> {
    let mut concrete_verifications = Vec::new();

    let mut sorted_ids: Vec<_> = registry.nodes.keys().cloned().collect();
    sorted_ids.sort();

    for id in sorted_ids {
        let Some(node) = registry.nodes.get(&id) else {
            continue;
        };
        if !matches!(node.element.element_type, ElementType::Verification(_)) {
            continue;
        }
        if has_user_created_objective_parent(registry, &node.element.identifier) {
            continue;
        }
        concrete_verifications.push((node.element.identifier.clone(), node.element.name.clone()));
    }

    if concrete_verifications.is_empty() {
        return Ok(VerificationObjectiveMigrationSummary {
            migration_id: VERIFICATION_OBJECTIVE_MIGRATION_ID,
            holder_file: VERIFICATION_OBJECTIVE_HOLDER_FILE,
            objectives_created: 0,
            derive_relations_added: 0,
            affected_verifications: Vec::new(),
        });
    }

    let mut relations_added = 0;
    let mut affected_verifications = Vec::new();

    let objectives_created =
        if let Some(existing) = registry.nodes.get(VERIFICATION_OBJECTIVE_HOLDER_ID) {
            if !matches!(
                existing.element.element_type,
                ElementType::VerificationObjective
            ) {
                return Err(ReqvireError::InvalidOperation(format!(
                "Migration holder target '{}' already exists but is not a verification-objective",
                VERIFICATION_OBJECTIVE_HOLDER_ID
            )));
            }
            0
        } else {
            let objective = holder_objective_element();
            registry.nodes.insert(
                VERIFICATION_OBJECTIVE_HOLDER_ID.to_string(),
                ElementNode {
                    element: objective,
                    relations: Vec::new(),
                },
            );
            1
        };

    if let Some(holder_node) = registry.nodes.get_mut(VERIFICATION_OBJECTIVE_HOLDER_ID) {
        for (verification_id, verification_name) in concrete_verifications {
            if !holder_node.element.relations.iter().any(|relation| {
                relation.user_created
                    && relation.relation_type.name == "derive"
                    && matches!(&relation.target.link, LinkType::Identifier(target) if target == &verification_id)
            }) {
                holder_node.element.relations.push(Relation::new(
                    "derive",
                    verification_name,
                    &verification_id,
                    Some(verification_id.clone()),
                )?);
                relations_added += 1;
            }
            affected_verifications.push(verification_id);
        }
    }

    Ok(VerificationObjectiveMigrationSummary {
        migration_id: VERIFICATION_OBJECTIVE_MIGRATION_ID,
        holder_file: VERIFICATION_OBJECTIVE_HOLDER_FILE,
        objectives_created,
        derive_relations_added: relations_added,
        affected_verifications,
    })
}

pub fn apply_documents_header_migration(
    excluded_filename_patterns: &GlobSet,
    dry_run: bool,
) -> Result<(DocumentsHeaderMigrationSummary, Vec<FileDiff>), ReqvireError> {
    let mut affected_files = Vec::new();
    let mut diffs = Vec::new();

    for path in utils::scan_markdown_files(None, excluded_filename_patterns) {
        let current = filesystem::read_file(&path)?;
        let Some(next) = rewrite_documents_header_content(&current) else {
            continue;
        };
        let relative = utils::get_relative_path(&path)
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|_| path.to_string_lossy().to_string());
        let diff = generate_file_diff(&relative, &current, &next);
        if !diff.lines.is_empty() {
            diffs.push(diff);
        }
        if !dry_run {
            filesystem::write_file(&path, next.as_bytes())?;
        }
        affected_files.push(relative);
    }

    affected_files.sort();

    Ok((
        DocumentsHeaderMigrationSummary {
            migration_id: DOCUMENTS_HEADER_MIGRATION_ID,
            files_changed: affected_files.len(),
            affected_files,
        },
        diffs,
    ))
}

fn rewrite_documents_header_content(content: &str) -> Option<String> {
    let mut output = Vec::new();
    let mut changed = false;
    let mut saw_h1 = false;

    for line in content.split_inclusive('\n') {
        if !saw_h1 && line.trim_start().starts_with("# ") {
            saw_h1 = true;
            if line.trim() == "# Documents" {
                let suffix = if line.ends_with('\n') { "\n" } else { "" };
                output.push(format!("# Element{}", suffix));
                changed = true;
                continue;
            }
        }
        output.push(line.to_string());
    }

    if !saw_h1 && content.trim() == "# Documents" {
        return Some("# Element".to_string());
    }

    changed.then(|| output.concat())
}

pub fn apply_reused_contract_context_section_migration(
    excluded_filename_patterns: &GlobSet,
    dry_run: bool,
) -> Result<(ReusedContractContextSectionMigrationSummary, Vec<FileDiff>), ReqvireError> {
    let mut affected_files = Vec::new();
    let mut diffs = Vec::new();

    for path in utils::scan_markdown_files(None, excluded_filename_patterns) {
        let current = filesystem::read_file(&path)?;
        let Some(next) = rewrite_reused_contract_context_section_content(&current) else {
            continue;
        };
        let relative = utils::get_relative_path(&path)
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|_| path.to_string_lossy().to_string());
        let diff = generate_file_diff(&relative, &current, &next);
        if !diff.lines.is_empty() {
            diffs.push(diff);
        }
        if !dry_run {
            filesystem::write_file(&path, next.as_bytes())?;
        }
        affected_files.push(relative);
    }

    affected_files.sort();

    Ok((
        ReusedContractContextSectionMigrationSummary {
            migration_id: REUSED_CONTRACT_CONTEXT_SECTION_MIGRATION_ID,
            files_changed: affected_files.len(),
            affected_files,
        },
        diffs,
    ))
}

pub fn apply_concept_reference_link_migration(
    registry: &GraphRegistry,
    excluded_filename_patterns: &GlobSet,
    dry_run: bool,
) -> Result<(ConceptReferenceLinkMigrationSummary, Vec<FileDiff>), ReqvireError> {
    let concept_iri_targets = concept_iri_targets(registry);
    let mut affected_files = Vec::new();
    let mut diffs = Vec::new();
    let mut references_rewritten = 0;

    for path in utils::scan_markdown_files(None, excluded_filename_patterns) {
        let current = filesystem::read_file(&path)?;
        let relative = utils::get_relative_path(&path)
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|_| path.to_string_lossy().to_string());
        let Some((next, rewritten)) =
            rewrite_concept_reference_links_content(&current, &relative, &concept_iri_targets)?
        else {
            continue;
        };
        let diff = generate_file_diff(&relative, &current, &next);
        if !diff.lines.is_empty() {
            diffs.push(diff);
        }
        if !dry_run {
            filesystem::write_file(&path, next.as_bytes())?;
        }
        references_rewritten += rewritten;
        affected_files.push(relative);
    }

    affected_files.sort();
    affected_files.dedup();

    Ok((
        ConceptReferenceLinkMigrationSummary {
            migration_id: CONCEPT_REFERENCE_LINK_MIGRATION_ID,
            references_rewritten,
            affected_files,
        },
        diffs,
    ))
}

fn concept_iri_targets(registry: &GraphRegistry) -> BTreeMap<String, Option<String>> {
    let mut targets = BTreeMap::new();
    for element in registry.get_all_elements() {
        if !element.element_type.is_concept() {
            continue;
        }
        let Some(iri) = registry.generated_concept_iri_for_element(element) else {
            continue;
        };
        match targets.get(&iri) {
            Some(Some(existing)) if existing != &element.identifier => {
                targets.insert(iri, None);
            }
            Some(_) => {}
            None => {
                targets.insert(iri, Some(element.identifier.clone()));
            }
        }
    }
    targets
}

fn rewrite_concept_reference_links_content(
    content: &str,
    source_file: &str,
    concept_iri_targets: &BTreeMap<String, Option<String>>,
) -> Result<Option<(String, usize)>, ReqvireError> {
    let mut output = Vec::new();
    let mut in_section = false;
    let mut rewritten = 0;

    for line in content.split_inclusive('\n') {
        let (body, suffix) = if let Some(body) = line.strip_suffix("\r\n") {
            (body, "\r\n")
        } else if let Some(body) = line.strip_suffix('\n') {
            (body, "\n")
        } else {
            (line, "")
        };

        let trimmed = body.trim();
        if trimmed.starts_with("#### ") {
            in_section = trimmed == "#### Concept References";
            output.push(line.to_string());
            continue;
        }

        if !in_section
            || trimmed.is_empty()
            || utils::extract_markdown_link(trimmed.strip_prefix("* ").unwrap_or(trimmed)).is_some()
        {
            output.push(line.to_string());
            continue;
        }

        let Some(entry) = trimmed.strip_prefix("* ") else {
            output.push(line.to_string());
            continue;
        };
        let Some((label, iri)) = entry.split_once(':') else {
            output.push(line.to_string());
            continue;
        };
        let label = label.trim();
        let iri = iri.trim();
        let Some(Some(target_identifier)) = concept_iri_targets.get(iri) else {
            output.push(line.to_string());
            continue;
        };

        let link = concept_reference_relative_link(source_file, target_identifier)?;
        let leading_len = body.len() - body.trim_start().len();
        output.push(format!(
            "{}* [{}]({}){}",
            &body[..leading_len],
            label,
            link,
            suffix
        ));
        rewritten += 1;
    }

    Ok((rewritten > 0).then(|| (output.concat(), rewritten)))
}

fn concept_reference_relative_link(
    source_file: &str,
    target_identifier: &str,
) -> Result<String, ReqvireError> {
    let (target_file, target_fragment) = utils::extract_path_and_fragment(target_identifier);
    if target_file == source_file {
        return Ok(format!("#{}", target_fragment.unwrap_or(target_identifier)));
    }
    let source_parent = PathBuf::from(source_file)
        .parent()
        .map(PathBuf::from)
        .unwrap_or_default();
    utils::to_relative_identifier(target_identifier, &source_parent, false)
}

fn rewrite_reused_contract_context_section_content(content: &str) -> Option<String> {
    let mut output = Vec::new();
    let mut changed = false;

    for line in content.split_inclusive('\n') {
        let (body, suffix) = if let Some(body) = line.strip_suffix("\r\n") {
            (body, "\r\n")
        } else if let Some(body) = line.strip_suffix('\n') {
            (body, "\n")
        } else {
            (line, "")
        };
        let trimmed = body.trim();
        let replacement = if trimmed == format!("#### {}", LEGACY_ATTACHMENTS_SECTION) {
            Some(format!("#### {}", REUSED_CONTRACT_CONTEXT_SECTION))
        } else if trimmed == format!("## {}", LEGACY_ATTACHMENTS_SECTION) {
            Some(format!("## {}", REUSED_CONTRACT_CONTEXT_SECTION))
        } else {
            None
        };

        if let Some(replacement) = replacement {
            let leading_len = body.len() - body.trim_start().len();
            output.push(format!("{}{}{}", &body[..leading_len], replacement, suffix));
            changed = true;
        } else {
            output.push(line.to_string());
        }
    }

    if content.is_empty() {
        return None;
    }

    if !content.ends_with('\n') && output.is_empty() {
        let trimmed = content.trim();
        let replacement = if trimmed == format!("#### {}", LEGACY_ATTACHMENTS_SECTION) {
            Some(format!("#### {}", REUSED_CONTRACT_CONTEXT_SECTION))
        } else if trimmed == format!("## {}", LEGACY_ATTACHMENTS_SECTION) {
            Some(format!("## {}", REUSED_CONTRACT_CONTEXT_SECTION))
        } else {
            None
        };
        if let Some(replacement) = replacement {
            return Some(replacement);
        }
    }

    changed.then(|| output.concat())
}

fn has_user_created_objective_parent(registry: &GraphRegistry, verification_id: &str) -> bool {
    let Some(node) = registry.nodes.get(verification_id) else {
        return false;
    };
    node.element.relations.iter().any(|relation| {
        if relation.relation_type.name != "derivedFrom" {
            return false;
        }
        let LinkType::Identifier(target_id) = &relation.target.link else {
            return false;
        };
        registry.nodes.get(target_id).is_some_and(|target| {
            matches!(
                target.element.element_type,
                ElementType::VerificationObjective
            )
        })
    })
}

fn holder_objective_element() -> Element {
    let mut element = Element::new(
        VERIFICATION_OBJECTIVE_HOLDER_NAME,
        VERIFICATION_OBJECTIVE_HOLDER_ID,
        VERIFICATION_OBJECTIVE_HOLDER_FILE,
        1,
        Some(ElementType::VerificationObjective),
    );
    element.content = "This verification objective was created by migration as a shared holder for standalone concrete verifications.\n\nRead the current Reqvire verification authoring guidance before keeping this structure: verification objectives define intent and grouping, while concrete verification elements derive from objectives and carry `verify` and evidence relations.".to_string();
    element
        .metadata
        .insert("type".to_string(), "verification-objective".to_string());
    element.freeze_content();
    element
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_candidate_detects_verification_objective_validation_errors() {
        let errors = vec![ReqvireError::IncompatibleElementTypes(
            "'verify' should connect a concrete verification element to a requirement; capabilities are verified through requirement coverage rollup and verification-objective is not allowed".to_string(),
        )];

        let plan = candidates_for_validation_errors(&errors);

        assert_eq!(plan.candidates.len(), 1);
        assert_eq!(plan.candidates[0].id, VERIFICATION_OBJECTIVE_MIGRATION_ID);
    }

    #[test]
    fn migration_candidate_detects_legacy_contract_relation_errors() {
        let errors = vec![ReqvireError::InvalidMarkdownStructure(
            "File system-model/Example.md: Element 'Requirement' uses legacy relation 'refinedBy'. Use 'definedBy' for requirement-owned contract elements, or run `reqvire migrate`.".to_string(),
        )];

        let plan = candidates_for_validation_errors(&errors);

        assert!(plan
            .candidates
            .iter()
            .any(|candidate| candidate.id == CONTRACT_RELATION_MIGRATION_ID));
    }

    #[test]
    fn migration_candidate_ignores_unrelated_errors() {
        let errors = vec![ReqvireError::MissingElement(
            "Missing unrelated element".to_string(),
        )];

        let plan = candidates_for_validation_errors(&errors);

        assert!(plan.is_empty());
    }

    #[test]
    fn verification_objective_holder_migration_adds_one_shared_holder_and_relations() {
        let mut registry = GraphRegistry::new();
        let mut verification = Element::new(
            "CLI Help Structure Verification",
            "system-model/Verifications/CLI.md#cli-help-structure-verification",
            "system-model/Verifications/CLI.md",
            1,
            Some(ElementType::from_metadata("test-verification")),
        );
        verification
            .metadata
            .insert("type".to_string(), "test-verification".to_string());
        registry
            .register_element(verification, "system-model/Verifications/CLI.md")
            .unwrap();
        let mut second_verification = Element::new(
            "CLI Search Verification",
            "system-model/Verifications/CLI.md#cli-search-verification",
            "system-model/Verifications/CLI.md",
            2,
            Some(ElementType::from_metadata("test-verification")),
        );
        second_verification
            .metadata
            .insert("type".to_string(), "test-verification".to_string());
        registry
            .register_element(second_verification, "system-model/Verifications/CLI.md")
            .unwrap();

        let summary = apply_verification_objective_holders(&mut registry).unwrap();

        assert_eq!(summary.objectives_created, 1);
        assert_eq!(summary.derive_relations_added, 2);
        assert!(registry
            .nodes
            .contains_key(VERIFICATION_OBJECTIVE_HOLDER_ID));

        let holder = &registry.nodes[VERIFICATION_OBJECTIVE_HOLDER_ID].element;
        for verification_id in [
            "system-model/Verifications/CLI.md#cli-help-structure-verification",
            "system-model/Verifications/CLI.md#cli-search-verification",
        ] {
            assert!(holder.relations.iter().any(|relation| {
                relation.user_created
                    && relation.relation_type.name == "derive"
                    && relation.target.link.as_str() == verification_id
            }));
        }
    }

    #[test]
    fn contract_relation_migration_rewrites_legacy_relation_names() {
        let mut registry = GraphRegistry::new();

        let mut requirement = Element::new(
            "Invoice Requirement",
            "system-model/Billing.md#invoice-requirement",
            "system-model/Billing.md",
            1,
            Some(ElementType::from_metadata("requirement")),
        );
        requirement
            .metadata
            .insert("type".to_string(), "requirement".to_string());
        requirement.relations.push(
            Relation::new(
                "refinedBy",
                "Invoice Numbering Specification".to_string(),
                "system-model/Billing.md#invoice-numbering-specification",
                Some("invoice-numbering-specification".to_string()),
            )
            .unwrap(),
        );
        registry
            .register_element(requirement, "system-model/Billing.md")
            .unwrap();

        let mut specification = Element::new(
            "Invoice Numbering Specification",
            "system-model/Billing.md#invoice-numbering-specification",
            "system-model/Billing.md",
            2,
            Some(ElementType::from_metadata("specification")),
        );
        specification
            .metadata
            .insert("type".to_string(), "specification".to_string());
        specification.relations.push(
            Relation::new(
                "refine",
                "Invoice Requirement".to_string(),
                "system-model/Billing.md#invoice-requirement",
                Some("invoice-requirement".to_string()),
            )
            .unwrap(),
        );
        registry
            .register_element(specification, "system-model/Billing.md")
            .unwrap();

        let summary = apply_contract_relation_migration(&mut registry).unwrap();

        assert_eq!(summary.relations_rewritten, 2);
        assert_eq!(summary.affected_files, vec!["system-model/Billing.md"]);

        let requirement = &registry.nodes["system-model/Billing.md#invoice-requirement"].element;
        assert!(requirement
            .relations
            .iter()
            .any(|relation| relation.relation_type.name == "definedBy"));
        assert!(!requirement
            .relations
            .iter()
            .any(|relation| relation.relation_type.name == "refinedBy"));

        let specification =
            &registry.nodes["system-model/Billing.md#invoice-numbering-specification"].element;
        assert!(specification
            .relations
            .iter()
            .any(|relation| relation.relation_type.name == "define"));
        assert!(!specification
            .relations
            .iter()
            .any(|relation| relation.relation_type.name == "refine"));
    }

    #[test]
    fn documents_header_migration_rewrites_only_first_documents_h1() {
        let input = "\n# Documents\n\n## Metadata\n  * type: specification\n";
        let rewritten = rewrite_documents_header_content(input).unwrap();
        assert_eq!(
            rewritten,
            "\n# Element\n\n## Metadata\n  * type: specification\n"
        );
        assert!(rewrite_documents_header_content("# Elements\n").is_none());
        assert!(rewrite_documents_header_content("# Other\n\n# Documents\n").is_none());
    }

    #[test]
    fn reused_contract_context_migration_rewrites_legacy_attachment_headings() {
        let input = "### Requirement\n\n#### Attachments\n  * [Contract](Contracts.md#contract)\n\n## Attachments\n";
        let rewritten = rewrite_reused_contract_context_section_content(input).unwrap();

        assert_eq!(
            rewritten,
            "### Requirement\n\n#### Reused Contract Context\n  * [Contract](Contracts.md#contract)\n\n## Reused Contract Context\n"
        );
        assert!(
            rewrite_reused_contract_context_section_content("#### Reused Contract Context\n")
                .is_none()
        );
    }
}
