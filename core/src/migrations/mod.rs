use crate::diff::{generate_file_diff, FileDiff};
use crate::element::{Element, ElementType};
use crate::error::ReqvireError;
use crate::filesystem;
use crate::graph_registry::{ElementNode, GraphRegistry};
use crate::relation::{LinkType, Relation};
use crate::utils;
use globset::GlobSet;
use serde::Serialize;

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

    MigrationPlan { candidates }
}

fn is_verification_objective_migration_error(error: &ReqvireError) -> bool {
    let message = error.to_string();
    message.contains("verification-objective")
        || (message.contains("verifiedBy") && message.contains("verification"))
        || (message.contains("verify") && message.contains("verification"))
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
            "'verify' should connect a concrete verification element to a capability or requirement; verification-objective is not allowed".to_string(),
        )];

        let plan = candidates_for_validation_errors(&errors);

        assert_eq!(plan.candidates.len(), 1);
        assert_eq!(plan.candidates[0].id, VERIFICATION_OBJECTIVE_MIGRATION_ID);
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
            "requirements/Verifications/CLI.md#cli-help-structure-verification",
            "requirements/Verifications/CLI.md",
            1,
            Some(ElementType::from_metadata("test-verification")),
        );
        verification
            .metadata
            .insert("type".to_string(), "test-verification".to_string());
        registry
            .register_element(verification, "requirements/Verifications/CLI.md")
            .unwrap();
        let mut second_verification = Element::new(
            "CLI Search Verification",
            "requirements/Verifications/CLI.md#cli-search-verification",
            "requirements/Verifications/CLI.md",
            2,
            Some(ElementType::from_metadata("test-verification")),
        );
        second_verification
            .metadata
            .insert("type".to_string(), "test-verification".to_string());
        registry
            .register_element(second_verification, "requirements/Verifications/CLI.md")
            .unwrap();

        let summary = apply_verification_objective_holders(&mut registry).unwrap();

        assert_eq!(summary.objectives_created, 1);
        assert_eq!(summary.derive_relations_added, 2);
        assert!(registry
            .nodes
            .contains_key(VERIFICATION_OBJECTIVE_HOLDER_ID));

        let holder = &registry.nodes[VERIFICATION_OBJECTIVE_HOLDER_ID].element;
        for verification_id in [
            "requirements/Verifications/CLI.md#cli-help-structure-verification",
            "requirements/Verifications/CLI.md#cli-search-verification",
        ] {
            assert!(holder.relations.iter().any(|relation| {
                relation.user_created
                    && relation.relation_type.name == "derive"
                    && relation.target.link.as_str() == verification_id
            }));
        }
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
}
