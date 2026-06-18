use crate::element;
use crate::error::ReqvireError;
use crate::graph_registry::{self, ElementNode, RelationNode};
use crate::relation;
use crate::relation::{LinkType, Relation, RelationTarget};
use difference::{Changeset, Difference};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

/// Represents a simplified relation for reporting.
#[derive(Debug, Clone, Serialize)]
pub struct RelationSummary {
    pub relation_type: String,
    pub target: RelationTarget,
    pub target_changed: bool,
    #[serde(skip_serializing)]
    pub user_created: bool,
}

impl RelationSummary {
    pub fn to_repo_url(&self, base_url: &str, commit: &str) -> Option<String> {
        match &self.target.link {
            LinkType::Identifier(id) => {
                let path = PathBuf::from(id);
                Some(format!(
                    "{}/blob/{}/{}",
                    base_url,
                    commit,
                    path.to_string_lossy()
                ))
            }
            LinkType::InternalPath(path) => Some(format!(
                "{}/blob/{}/{}",
                base_url,
                commit,
                path.to_string_lossy()
            )),
            _ => None,
        }
    }
}

/// Report for an element that is newly added (only in the current registry).
#[derive(Debug, Serialize)]
pub struct AddedElement {
    pub element_id: String,
    pub name: String,
    pub new_content: String,
    pub added_relations: Vec<RelationSummary>,
    pub change_impact_tree: ElementNode,
}

/// Report for an element that has been removed (only in the reference registry).
#[derive(Debug, Serialize)]
pub struct RemovedElement {
    pub element_id: String,
    pub name: String,
    pub old_content: String,
    pub removed_relations: Vec<RelationSummary>,
}

/// Report for an element that has been relocated (name exists in both registries but identifier changed).
#[derive(Debug, Serialize)]
pub struct RelocatedElement {
    pub name: String,
    pub old_identifier: String,
    pub new_identifier: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvalidatedVerification {
    pub element_id: String,
    pub name: String,
    pub content: String,
}

/// A scope root representing a common parent requirement covering impacted elements.
#[derive(Debug, Clone, Serialize)]
pub struct ImpactScopeRoot {
    pub element_id: String,
    pub name: String,
}

/// Report for an element that exists in both registries but has differences.
#[derive(Debug, Serialize)]
pub struct ChangedElement {
    pub element_id: String,
    pub name: String,
    pub old_content: String,
    pub new_content: String,
    pub content_changed: bool,
    pub added_relations: Vec<RelationSummary>,
    pub removed_relations: Vec<RelationSummary>,
    pub change_impact_tree: ElementNode,
    /// Set of attachment target strings that changed (for rendering with ⚠️)
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub changed_attachments: HashSet<String>,
}

impl ChangedElement {
    pub fn to_repo_url(&self, base_url: &str, commit: &str) -> String {
        let path = PathBuf::from(&self.element_id);
        format!("{}/blob/{}/{}", base_url, commit, path.to_string_lossy())
    }
}

/// Report detailing changes between two registries.
#[derive(Debug, Serialize)]
pub struct ChangeImpactReport {
    pub added: Vec<AddedElement>,
    pub removed: Vec<RemovedElement>,
    pub changed: Vec<ChangedElement>,
    pub relocated: Vec<RelocatedElement>,
    pub impact_scope: Vec<ImpactScopeRoot>,
    pub invalidated_verifications: Vec<InvalidatedVerification>,
    #[serde(skip)]
    pub all_added_element_ids: HashSet<String>,
    #[serde(skip)]
    pub all_changed_element_ids: HashSet<String>,
}

impl Default for ChangeImpactReport {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangeImpactReport {
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            changed: Vec::new(),
            relocated: Vec::new(),
            impact_scope: Vec::new(),
            invalidated_verifications: Vec::new(),
            all_added_element_ids: HashSet::new(),
            all_changed_element_ids: HashSet::new(),
        }
    }

    /// Outputs the report as json with GitHub links included.
    pub fn to_json(
        &self,
        base_url: &str,
        git_commit: &str,
        previous_git_commit: &str,
    ) -> serde_json::Value {
        let added: Vec<_> = self
            .added
            .iter()
            .map(|elem| {
                let element_url = format!("{}/blob/{}/{}", base_url, git_commit, elem.element_id);
                let added_relations: Vec<_> = elem
                    .added_relations
                    .iter()
                    .map(|rel| {
                        let target_url = match rel.target.link {
                            LinkType::Identifier(ref id) => {
                                format!("{}/blob/{}/{}", base_url, git_commit, id)
                            }
                            LinkType::InternalPath(ref path) => format!(
                                "{}/blob/{}/{}",
                                base_url,
                                previous_git_commit,
                                path.display()
                            ),
                            _ => rel.target.link.as_str().to_string(),
                        };
                        json!({
                            "relation_type": rel.relation_type,
                            "target_changed": rel.target_changed,
                            "target_text": rel.target.text,
                            "target_url": target_url
                        })
                    })
                    .collect();
                let impact_tree =
                    render_change_impact_tree_json(&elem.change_impact_tree, base_url, git_commit);
                json!({
                    "element_id": element_url,
                    "new_content": elem.new_content,
                    "added_relations": added_relations,
                    "change_impact_tree": impact_tree
                })
            })
            .collect();
        let removed: Vec<_> = self
            .removed
            .iter()
            .map(|elem| {
                let element_url = format!(
                    "{}/blob/{}/{}",
                    base_url, previous_git_commit, elem.element_id
                );
                let removed_relations: Vec<_> = elem
                    .removed_relations
                    .iter()
                    .map(|rel| {
                        let target_url = match rel.target.link {
                            LinkType::Identifier(ref id) => {
                                format!("{}/blob/{}/{}", base_url, previous_git_commit, id)
                            }
                            LinkType::InternalPath(ref path) => format!(
                                "{}/blob/{}/{}",
                                base_url,
                                previous_git_commit,
                                path.display()
                            ),
                            _ => rel.target.link.as_str().to_string(),
                        };
                        json!({
                            "relation_type": rel.relation_type,
                            "target_changed": rel.target_changed,
                            "target_text": rel.target.text,
                            "target_url": target_url
                        })
                    })
                    .collect();
                json!({
                    "element_id": element_url,
                    "old_content": elem.old_content,
                    "removed_relations": removed_relations
                })
            })
            .collect();
        let changed: Vec<_> = self
            .changed
            .iter()
            .map(|elem| {
                let element_url = format!("{}/blob/{}/{}", base_url, git_commit, elem.element_id);
                let added_relations: Vec<_> = elem
                    .added_relations
                    .iter()
                    .map(|rel| {
                        let target_url = match rel.target.link {
                            LinkType::Identifier(ref id) => {
                                format!("{}/blob/{}/{}", base_url, git_commit, id)
                            }
                            LinkType::InternalPath(ref path) => format!(
                                "{}/blob/{}/{}",
                                base_url,
                                previous_git_commit,
                                path.display()
                            ),
                            _ => rel.target.link.as_str().to_string(),
                        };
                        json!({
                            "relation_type": rel.relation_type,
                            "target_changed": rel.target_changed,
                            "target_text": rel.target.text,
                            "target_url": target_url
                        })
                    })
                    .collect();
                let removed_relations: Vec<_> = elem
                    .removed_relations
                    .iter()
                    .map(|rel| {
                        let target_url = match rel.target.link {
                            LinkType::Identifier(ref id) => {
                                format!("{}/blob/{}/{}", base_url, previous_git_commit, id)
                            }
                            LinkType::InternalPath(ref path) => format!(
                                "{}/blob/{}/{}",
                                base_url,
                                previous_git_commit,
                                path.display()
                            ),
                            _ => rel.target.link.as_str().to_string(),
                        };
                        json!({
                            "relation_type": rel.relation_type,
                            "target_changed": rel.target_changed,
                            "target_text": rel.target.text,
                            "target_url": target_url
                        })
                    })
                    .collect();

                let impact_tree =
                    render_change_impact_tree_json(&elem.change_impact_tree, base_url, git_commit);
                json!({
                    "element_id": element_url,
                    "name": elem.name,
                    "old_content": elem.old_content,
                    "new_content": elem.new_content,
                    "content_changed": elem.content_changed,
                    "added_relations": added_relations,
                    "removed_relations": removed_relations,
                    "change_impact_tree": impact_tree,
                    "changed_attachments": elem.changed_attachments
                })
            })
            .collect();
        let relocated: Vec<_> = self
            .relocated
            .iter()
            .map(|elem| {
                json!({
                    "name": elem.name,
                    "old_location": elem.old_identifier,
                    "new_location": elem.new_identifier
                })
            })
            .collect();
        let invalidated_verifications: Vec<_> = self
            .invalidated_verifications
            .iter()
            .map(|invalidated_ver| {
                let target_url = format!(
                    "{}/blob/{}/{}",
                    base_url, git_commit, invalidated_ver.element_id
                );
                json!({
                    "target_text": invalidated_ver.name,
                    "target_url": target_url,
                    "content": invalidated_ver.content
                })
            })
            .collect();
        let impact_scope: Vec<_> = self
            .impact_scope
            .iter()
            .map(|scope| {
                let element_url = format!("{}/blob/{}/{}", base_url, git_commit, scope.element_id);
                json!({
                    "name": scope.name,
                    "element_id": element_url
                })
            })
            .collect();
        json!({
            "added": added,
            "removed": removed,
            "changed": changed,
            "relocated": relocated,
            "impact_scope": impact_scope,
            "invalidated_verifications": invalidated_verifications
        })
    }
    /// Outputs the report as text with GitHub links included.
    pub fn to_text(&self, base_url: &str, git_commit: &str, previous_git_commit: &str) -> String {
        let mut output = String::new();
        output.push_str("## Change Impact Report\n\n");

        // Use all_added_element_ids which contains IDs from before filtering
        let new_element_ids = &self.all_added_element_ids;

        // Relocated Elements section
        if !self.relocated.is_empty() {
            output.push_str("### Relocated Elements\n\n");
            for elem in &self.relocated {
                output.push_str(&format!("* [{}]({})\n", elem.name, elem.new_identifier));
                output.push_str(&format!("  * Old location: {}\n", elem.old_identifier));
                output.push_str(&format!("  * New location: {}\n", elem.new_identifier));
            }
            output.push_str("\n---\n\n");
        }

        // Removed Elements section
        if !self.removed.is_empty() {
            output.push_str("### Removed Elements\n\n");
        }

        for elem in &self.removed {
            let element_url = format!(
                "{}/blob/{}/{}",
                base_url, previous_git_commit, elem.element_id
            );
            output.push_str("* ");
            output.push_str(&format!("[{}]({})\n", elem.name, element_url));
        }
        if !self.removed.is_empty() {
            output.push_str("\n---\n\n");
        }
        // Added Elements section
        if !self.added.is_empty() {
            output.push_str("### New Elements\n\n");
        }

        for elem in &self.added {
            let element_url = format!("{}/blob/{}/{}", base_url, git_commit, elem.element_id);
            output.push_str("* ");
            output.push_str(&format!("[{}]({})\n", elem.name, element_url));
            let empty_changed: HashSet<String> = HashSet::new();
            let rendered_tree = render_change_impact_tree(
                &elem.change_impact_tree,
                1,
                base_url,
                git_commit,
                new_element_ids,
                &empty_changed,
            );
            if !rendered_tree.trim().is_empty() {
                output.push_str(&rendered_tree);
                output.push_str("\n\n");
            }
        }
        if !self.added.is_empty() {
            output.push_str("\n---\n\n");
        }

        // Changed Elements section
        if !self.changed.is_empty() {
            output.push_str("### Changed Elements\n\n");
        }
        for elem in &self.changed {
            let element_url = format!("{}/blob/{}/{}", base_url, git_commit, elem.element_id);
            // Add ⚠️ if element has content or attachment changes
            let change_marker = if elem.content_changed || !elem.changed_attachments.is_empty() {
                " ⚠️"
            } else {
                ""
            };
            output.push_str(&format!(
                "* [{}]({}){}\n",
                elem.name, element_url, change_marker
            ));

            // Render attachments with 📎 icon (and ⚠️ for changed ones)
            let attachments = &elem.change_impact_tree.element.attachments;
            for att in attachments {
                let att_target = att.target.as_str();
                let change_icon = if elem.changed_attachments.contains(&att_target) {
                    " ⚠️"
                } else {
                    ""
                };
                let att_url = format_attachment_url(
                    &att.target,
                    &elem.change_impact_tree.element.file_path,
                    base_url,
                    git_commit,
                );
                let att_name = format_attachment_name(&att.target);
                output.push_str(&format!(
                    "    * 📎 [{}]({}){}\n",
                    att_name, att_url, change_icon
                ));
            }

            // Render relations tree
            let rendered_tree = render_change_impact_tree(
                &elem.change_impact_tree,
                1,
                base_url,
                git_commit,
                new_element_ids,
                &elem.changed_attachments,
            );
            if !rendered_tree.trim().is_empty() {
                output.push_str(&rendered_tree);
            }
            output.push('\n');
        }
        if !self.changed.is_empty() {
            output.push_str("\n---\n\n");
        }

        // Impact Scope section
        if !self.impact_scope.is_empty() {
            output.push_str("### Impact Scope\n\n");
            for scope in &self.impact_scope {
                let element_url = format!("{}/blob/{}/{}", base_url, git_commit, scope.element_id);
                output.push_str(&format!("* [{}]({})\n", scope.name, element_url));
            }
            output.push_str("\n---\n\n");
        }

        // Invalidated Verifications Section
        if !self.invalidated_verifications.is_empty() {
            output.push_str("## Invalidated Verifications\n\n");
            for invalidated_ver in &self.invalidated_verifications {
                let target_url = format!(
                    "{}/blob/{}/{}",
                    base_url, git_commit, invalidated_ver.element_id
                );
                output.push_str(&format!(
                    "- [ ] [{}]({})\n",
                    invalidated_ver.name, target_url
                ));
            }
            output.push('\n');
        }

        if self.removed.is_empty()
            && self.added.is_empty()
            && self.changed.is_empty()
            && self.relocated.is_empty()
        {
            output.push_str("\nNothing to report...\n");
        }
        output
    }

    pub fn to_json_string(
        &self,
        base_url: &str,
        git_commit: &str,
        previous_git_commit: &str,
    ) -> String {
        serde_json::to_string_pretty(&self.to_json(base_url, git_commit, previous_git_commit))
            .unwrap()
    }

    pub fn print(
        &self,
        base_url: &str,
        git_commit: &str,
        previous_git_commit: &str,
        as_json: bool,
    ) {
        if as_json {
            println!(
                "{}",
                self.to_json_string(base_url, git_commit, previous_git_commit)
            );
        } else {
            println!(
                "{}",
                self.to_text(base_url, git_commit, previous_git_commit)
            );
        }
    }
}

/// Generate a unified diff in a diff-highlighted markdown code fence.
fn _generate_markdown_diff(old: &str, new: &str) -> String {
    let changeset = Changeset::new(old, new, "\n");
    let mut diff_output = String::new();
    for diff in changeset.diffs {
        match diff {
            Difference::Same(ref x) => {
                for line in x.lines() {
                    diff_output.push(' ');
                    diff_output.push_str(line);
                    diff_output.push('\n');
                }
            }
            Difference::Rem(ref x) => {
                for line in x.lines() {
                    diff_output.push('-');
                    diff_output.push_str(line);
                    diff_output.push('\n');
                }
            }
            Difference::Add(ref x) => {
                for line in x.lines() {
                    diff_output.push('+');
                    diff_output.push_str(line);
                    diff_output.push('\n');
                }
            }
        }
    }
    format!("```diff\n{}```", diff_output)
}

/// Format attachment URL for rendering
fn format_attachment_url(
    target: &element::AttachmentTarget,
    _element_file: &str,
    base_url: &str,
    git_commit: &str,
) -> String {
    match target {
        element::AttachmentTarget::FilePath(path) => {
            format!(
                "{}/blob/{}/{}",
                base_url,
                git_commit,
                path.to_string_lossy()
            )
        }
        element::AttachmentTarget::ElementIdentifier(id) => {
            format!("{}/blob/{}/{}", base_url, git_commit, id)
        }
    }
}

/// Format attachment name for rendering (extract just the name part)
fn format_attachment_name(target: &element::AttachmentTarget) -> String {
    match target {
        element::AttachmentTarget::FilePath(path) => path.to_string_lossy().to_string(),
        element::AttachmentTarget::ElementIdentifier(id) => {
            // Extract element name from identifier (after #)
            id.split('#')
                .next_back()
                .unwrap_or(id)
                .split('-')
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(c) => c.to_uppercase().chain(chars).collect(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
}

/// Render the change impact tree recursively with GitHub links.
fn render_change_impact_tree(
    node: &ElementNode,
    indent: usize,
    base_url: &str,
    git_commit: &str,
    new_element_ids: &HashSet<String>,
    changed_attachments: &HashSet<String>,
) -> String {
    let mut output = String::new();
    let pad = "    ".repeat(indent);
    for relation_node in &node.relations {
        let target = &relation_node.element_node.element;
        let element_url = format!("{}/blob/{}/{}", base_url, git_commit, target.identifier);
        let change_icon = if target.changed_since_commit {
            " ⚠️"
        } else {
            ""
        };
        let new_icon = if new_element_ids.contains(&target.identifier) {
            " (new)"
        } else {
            ""
        };
        output.push_str(&format!(
            "{}* {} -> [{}]({}){}{}\n",
            pad, relation_node.relation_trigger, target.name, element_url, new_icon, change_icon
        ));

        // Render attachments for child elements
        for att in &target.attachments {
            let att_target = att.target.as_str();
            let att_change_icon = if changed_attachments.contains(&att_target) {
                " ⚠️"
            } else {
                ""
            };
            let att_url =
                format_attachment_url(&att.target, &target.file_path, base_url, git_commit);
            let att_name = format_attachment_name(&att.target);
            output.push_str(&format!(
                "{}    * 📎 [{}]({}){}\n",
                pad, att_name, att_url, att_change_icon
            ));
        }

        output.push_str(&render_change_impact_tree(
            &relation_node.element_node,
            indent + 1,
            base_url,
            git_commit,
            new_element_ids,
            changed_attachments,
        ));
    }
    output
}

/// Render the change impact tree recursively as structured JSON with GitHub URLs.
fn render_change_impact_tree_json(
    node: &ElementNode,
    base_url: &str,
    git_commit: &str,
) -> Vec<Value> {
    node.relations
        .iter()
        .map(|relation_node| {
            let child = &relation_node.element_node;
            // Construct the GitHub URL using the base URL and commit hash for the identifier
            let github_url = format!(
                "{}/blob/{}/{}",
                base_url, git_commit, &child.element.identifier
            );
            // Render the nested relations in the JSON structure
            let nested_relations = render_change_impact_tree_json(child, base_url, git_commit);
            // Map the relation_trigger as a key and its nested element info as the value
            let mut relation_obj = serde_json::Map::new();
            relation_obj.insert(
                relation_node.relation_trigger.clone(),
                json!({
                    "name": child.element.name,
                    "identifier": github_url,
                    "element_type": format!("{:?}", child.element.element_type),
                    "relations": nested_relations
                }),
            );
            // Return the relation in the desired format
            json!(relation_obj)
        })
        .collect()
}

/// Converts a relation into a summarized representation.
fn convert_relation_to_summary(rel: &Relation) -> RelationSummary {
    RelationSummary {
        relation_type: rel.relation_type.name.to_string(),
        target_changed: false,
        target: rel.target.clone(),
        user_created: rel.user_created,
    }
}

fn should_traverse_change_impact_relation(source: &element::Element, rel: &Relation) -> bool {
    let relation_name = rel.relation_type.name;
    if !relation::IMPACT_PROPAGATION_RELATIONS.contains(&relation_name) {
        return false;
    }

    match relation_name {
        // A semantic contract depends on ontology vocabulary; a contract change
        // does not make the ontology an impacted downstream review target.
        "use" => false,
        // Ontology changes flow to semantic contracts that use that ontology.
        "usedBy" => matches!(source.element_type, element::ElementType::Ontology),
        // Requirement changes should flag semantic-contract consistency review.
        "constrainedBy" => matches!(source.element_type, element::ElementType::Requirement(_)),
        // Semantic-contract changes flow to constrained requirements.
        "constrain" => matches!(source.element_type, element::ElementType::SemanticContract),
        _ => true,
    }
}

/// Builds the change impact tree recursively using `ElementNode` and keeps only forward relations.
pub fn build_change_impact_tree(
    current: &graph_registry::GraphRegistry,
    element_id: String,
    visited: &mut BTreeSet<String>,
    fallback_name: Option<String>,
) -> ElementNode {
    // Fetch the current element or generate a placeholder
    let element = current
        .get_element(&element_id)
        .cloned()
        .unwrap_or_else(|| {
            let display_name = fallback_name.unwrap_or_else(|| "Missing Element".to_string());
            let mut placeholder = element::Element::new(
                &display_name,
                &element_id,
                "unknown",
                0, // Placeholder elements don't have real line numbers
                None,
            );
            placeholder.content = "Element referenced but not found in registry".to_string();
            placeholder
        });
    // Recursively collect forward-impacting relation nodes
    let mut impact_relations = current.change_impact_with_relation(&element);

    // Sort relations to prioritize verifiedBy over derive to prevent visited set conflicts
    // When a parent requirement has both verifiedBy and derive relations, we want to process
    // verifiedBy first so verifications are added before sibling requirements visit them
    impact_relations.sort_by_key(|(_, rels)| {
        let has_verified_by = rels.iter().any(|r| r.relation_type.name == "verifiedBy");
        let has_derive = rels.iter().any(|r| r.relation_type.name == "derive");

        // Priority: verifiedBy (0), satisfiedBy (1), derive (2), others (3)
        if has_verified_by {
            0
        } else if rels.iter().any(|r| r.relation_type.name == "satisfiedBy") {
            1
        } else if has_derive {
            2
        } else {
            3
        }
    });

    let relations = impact_relations
        .into_iter()
        .filter_map(|(impacted_id, rels)| {
            let impact_relations: Vec<_> = rels
                .into_iter()
                .filter(|rel| should_traverse_change_impact_relation(&element, rel))
                .collect();
            if impact_relations.is_empty() {
                return None;
            }

            // Skip cycles
            if !visited.insert(impacted_id.clone()) {
                return None;
            }

            // Use the text from the first relation as a fallback display name
            let fallback_name = impact_relations.first().map(|rel| rel.target.text.clone());
            let child_node =
                build_change_impact_tree(current, impacted_id.clone(), visited, fallback_name);
            let forward_relations: Vec<_> = impact_relations
                .into_iter()
                .map(|rel| RelationNode {
                    relation_trigger: rel.relation_type.name.to_string(),
                    element_node: child_node.clone(),
                })
                .collect();

            if forward_relations.is_empty() {
                None
            } else {
                Some(forward_relations)
            }
        })
        .flatten()
        .collect();
    ElementNode { element, relations }
}

fn collect_verification_elements_from_impact_tree(
    node: &ElementNode,
) -> Vec<InvalidatedVerification> {
    let mut collected = Vec::new();
    let mut seen = HashSet::new();
    fn walk(
        node: &ElementNode,
        seen: &mut HashSet<String>,
        collected: &mut Vec<InvalidatedVerification>,
    ) {
        if let element::ElementType::Verification(_) = node.element.element_type {
            let id = node.element.identifier.clone();
            if seen.insert(id.clone()) {
                collected.push(InvalidatedVerification {
                    element_id: id,
                    name: node.element.name.clone(),
                    content: node.element.content.clone(),
                });
            }
        }
        for rel in &node.relations {
            walk(&rel.element_node, seen, collected);
        }
    }
    walk(node, &mut seen, &mut collected);
    collected
}

fn propagate_changed_flags(node: &mut ElementNode, changed_ids: &HashSet<String>) {
    for relation in &mut node.relations {
        let child = &mut relation.element_node;
        if changed_ids.contains(&child.element.identifier) {
            child.element.changed_since_commit = true;
        }
        // Recurse further down the tree
        propagate_changed_flags(child, changed_ids);
    }
}

pub fn apply_smart_filtering(
    report: &mut ChangeImpactReport,
    _current: &graph_registry::GraphRegistry,
) {
    // Step 1: Collect ALL added element IDs BEFORE filtering

    // Step 2: Collect IDs of elements referenced in trees (i.e. children, not roots)
    let mut referenced_ids = HashSet::new();
    for added in &report.added {
        for rel_node in &added.change_impact_tree.relations {
            if is_smart_filter_child_relation(&rel_node.relation_trigger) {
                collect_tree_ids_recursively(&rel_node.element_node, &mut referenced_ids);
            }
        }
        // Also collect attachment element identifiers
        collect_attachment_element_ids(&added.change_impact_tree.element, &mut referenced_ids);
    }

    for changed in &report.changed {
        for rel_node in &changed.change_impact_tree.relations {
            if is_smart_filter_child_relation(&rel_node.relation_trigger) {
                collect_tree_ids_recursively(&rel_node.element_node, &mut referenced_ids);
            }
        }
        // Also collect attachment element identifiers
        collect_attachment_element_ids(&changed.change_impact_tree.element, &mut referenced_ids);
    }

    // Step 3: Also collect attachments from children in the tree
    for added in &report.added {
        collect_attachment_ids_from_tree(&added.change_impact_tree, &mut referenced_ids);
    }
    for changed in &report.changed {
        collect_attachment_ids_from_tree(&changed.change_impact_tree, &mut referenced_ids);
    }

    // Step 4: Filter out added/changed elements that are referenced elsewhere (not root).
    // This includes elements referenced via downstream relations OR via attachments.
    report
        .added
        .retain(|e| !referenced_ids.contains(&e.element_id));
    report
        .changed
        .retain(|e| !referenced_ids.contains(&e.element_id));
}

fn collect_tree_ids_recursively(node: &ElementNode, set: &mut HashSet<String>) {
    // Insert this node's identifier
    set.insert(node.element.identifier.clone());

    // Recursively process all children
    for relation in &node.relations {
        if is_smart_filter_child_relation(&relation.relation_trigger) {
            collect_tree_ids_recursively(&relation.element_node, set);
        }
    }
}

fn is_smart_filter_child_relation(relation_type: &str) -> bool {
    matches!(
        relation_type,
        "derive"
            | "specifiedBy"
            | "satisfiedBy"
            | "definedBy"
            | "constrainedBy"
            | "constrain"
            | "use"
            | "usedBy"
            | "verifiedBy"
    )
}

/// Collect element identifiers from attachments (for smart filtering)
fn collect_attachment_element_ids(elem: &element::Element, set: &mut HashSet<String>) {
    for attachment in &elem.attachments {
        if let element::AttachmentTarget::ElementIdentifier(id) = &attachment.target {
            set.insert(id.clone());
        }
    }
}

/// Recursively collect attachment element IDs from the entire tree
fn collect_attachment_ids_from_tree(node: &ElementNode, set: &mut HashSet<String>) {
    collect_attachment_element_ids(&node.element, set);
    for relation in &node.relations {
        collect_attachment_ids_from_tree(&relation.element_node, set);
    }
}

/// Get attachment hashes for an element, resolving element attachment hashes from registry
/// Returns a sorted list of (attachment_target_string, hash) tuples for deterministic comparison
fn get_attachment_hashes(
    element: &element::Element,
    registry: &graph_registry::GraphRegistry,
) -> Vec<(String, String)> {
    let mut hashes: Vec<(String, String)> = element
        .attachments
        .iter()
        .filter_map(|att| {
            let target_str = att.target.as_str();
            let hash = match &att.target {
                element::AttachmentTarget::FilePath(_) => {
                    // File attachment - use stored content_hash
                    att.content_hash.clone()
                }
                element::AttachmentTarget::ElementIdentifier(elem_id) => {
                    // Element attachment - look up hash from registry
                    registry
                        .get_element(elem_id)
                        .map(|e| e.hash_impact_content.clone())
                }
            };
            hash.map(|h| (target_str, h))
        })
        .collect();
    hashes.sort_by(|a, b| a.0.cmp(&b.0));
    hashes
}

/// Get the set of attachment target strings that changed between two versions
fn get_changed_attachments(
    cur_elem: &element::Element,
    ref_elem: &element::Element,
    current_registry: &graph_registry::GraphRegistry,
    reference_registry: &graph_registry::GraphRegistry,
) -> HashSet<String> {
    let cur_hashes: Vec<(String, String)> = get_attachment_hashes(cur_elem, current_registry);
    let ref_hashes: Vec<(String, String)> = get_attachment_hashes(ref_elem, reference_registry);

    // Collect just the content hashes for comparison
    let cur_hash_set: HashSet<&String> = cur_hashes.iter().map(|(_, h)| h).collect();
    let ref_hash_set: HashSet<&String> = ref_hashes.iter().map(|(_, h)| h).collect();

    let mut changed = HashSet::new();

    // Check for changed or added attachments (hash in current but not in reference)
    for (target, cur_hash) in &cur_hashes {
        if !ref_hash_set.contains(cur_hash) {
            changed.insert(target.clone());
        }
    }

    // Check for removed attachments (hash in reference but not in current)
    // We need to find the current target for hashes that were removed
    for (target, ref_hash) in &ref_hashes {
        if !cur_hash_set.contains(ref_hash) {
            // Hash was removed - find if there's a current attachment with same target
            // or use the reference target
            let current_target = cur_hashes
                .iter()
                .find(|(t, _)| t == target)
                .map(|(t, _)| t.clone())
                .unwrap_or_else(|| target.clone());
            changed.insert(current_target);
        }
    }

    changed
}

/// Helper to normalize a relation by resolving its target identifier to an element name
/// Returns (relation_type_name, element_name) tuple for semantic comparison
/// Falls back to identifier if element cannot be resolved
fn normalize_relation_for_comparison(rel: &Relation) -> (String, String) {
    let relation_type = rel.relation_type.name.to_string();

    // Use the stable element_id if available (populated by GraphRegistry)
    // This is location-independent and remains unchanged across relocations
    if let Some(ref element_id) = rel.target.element_id {
        return (relation_type, element_id.clone());
    }

    // Fallback to full link for external URLs or internal paths
    (relation_type, rel.target.link.as_str().to_string())
}

fn is_scope_element(elem: &element::Element) -> bool {
    matches!(
        elem.element_type,
        element::ElementType::Capability | element::ElementType::Requirement(_)
    )
}

fn resolve_scope_relation_target_id(
    registry: &graph_registry::GraphRegistry,
    relation: &Relation,
) -> Option<String> {
    if let LinkType::Identifier(id) = &relation.target.link {
        if registry.get_element(id).is_some() {
            return Some(id.clone());
        }
    }

    relation.target.element_id.as_ref().and_then(|stable_id| {
        registry
            .nodes
            .values()
            .find(|node| node.element.id == *stable_id)
            .map(|node| node.element.identifier.clone())
    })
}

fn find_scope_parent_id(
    registry: &graph_registry::GraphRegistry,
    element_id: &str,
) -> Option<String> {
    let elem = registry.get_element(element_id)?;

    let allowed_parent_type = match elem.element_type {
        element::ElementType::Capability => Some(element::ElementType::Capability),
        element::ElementType::Requirement(_) => None,
        _ => return None,
    };

    if matches!(
        elem.element_type,
        element::ElementType::Capability | element::ElementType::Requirement(_)
    ) {
        if let Some(parent_id) = elem
            .relations
            .iter()
            .find(|r| r.relation_type.name == "derivedFrom")
            .and_then(|r| resolve_scope_relation_target_id(registry, r))
        {
            if let Some(parent) = registry.get_element(&parent_id) {
                match (&allowed_parent_type, &parent.element_type) {
                    (Some(element::ElementType::Capability), element::ElementType::Capability)
                    | (None, element::ElementType::Requirement(_)) => return Some(parent_id),
                    _ => {}
                }
            }
        }
    }

    if matches!(elem.element_type, element::ElementType::Requirement(_)) {
        return elem
            .relations
            .iter()
            .find(|r| r.relation_type.name == "specify")
            .and_then(|r| resolve_scope_relation_target_id(registry, r))
            .filter(|parent_id| {
                registry.get_element(parent_id).is_some_and(|parent| {
                    matches!(parent.element_type, element::ElementType::Capability)
                })
            });
    }

    None
}

/// Compute the impact scope: the per-branch lowest common ancestors of all
/// impacted capability/requirement elements. Requirement scope walks
/// `derivedFrom` requirement parents first, then crosses to the specifying
/// capability through `specify`; capability scope walks only capability
/// `derivedFrom` parents.
///
/// Algorithm:
/// 1. Collect requirement IDs from changed + added elements
/// 2. For removed elements: walk derivedFrom in reference registry to find
///    first parent still existing in current registry
/// 3. Bottom-up merge: group siblings by parent, replace groups of 2+ with parent
/// 4. Repeat until stable
/// 5. Return sorted by element_id for deterministic output
pub fn compute_impact_scope(
    current: &graph_registry::GraphRegistry,
    reference: &graph_registry::GraphRegistry,
    report: &ChangeImpactReport,
) -> Vec<ImpactScopeRoot> {
    // Step 1: Collect capability/requirement element IDs from changed + added
    // Use all_changed/all_added IDs (pre-smart-filtering) for complete scope
    let mut scope_set: HashSet<String> = HashSet::new();

    for id in report
        .all_changed_element_ids
        .iter()
        .chain(report.all_added_element_ids.iter())
    {
        if let Some(elem) = current.get_element(id) {
            if is_scope_element(elem) {
                scope_set.insert(id.clone());
            }
        }
    }

    // Step 2: For removed elements, walk model parent relations in reference to find
    // first parent that still exists in current registry
    for removed in &report.removed {
        if let Some(ref_elem) = reference.get_element(&removed.element_id) {
            if !is_scope_element(ref_elem) {
                continue;
            }
            let mut visited = HashSet::new();
            let mut current_id = removed.element_id.clone();
            visited.insert(current_id.clone());

            loop {
                let parent_id = find_scope_parent_id(reference, &current_id);

                match parent_id {
                    Some(pid) => {
                        if !visited.insert(pid.clone()) {
                            break; // circular reference
                        }
                        // Check if this parent exists in current registry
                        if current.get_element(&pid).is_some() {
                            if let Some(elem) = current.get_element(&pid) {
                                if is_scope_element(elem) {
                                    scope_set.insert(pid);
                                }
                            }
                            break;
                        }
                        // Parent also deleted, walk further up
                        current_id = pid;
                    }
                    None => break, // no parent, exclude
                }
            }
        }
    }

    if scope_set.is_empty() {
        return Vec::new();
    }

    // Step 3: Bottom-up merge loop
    loop {
        // For each element in scope_set, find its model parent in current registry
        let mut parent_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut no_parent: Vec<String> = Vec::new();

        for id in &scope_set {
            let parent_id = find_scope_parent_id(current, id);

            match parent_id {
                Some(pid) => {
                    parent_map.entry(pid).or_default().push(id.clone());
                }
                None => {
                    no_parent.push(id.clone());
                }
            }
        }

        // Merge: for parents with 2+ children, replace children with parent
        let mut new_set: HashSet<String> = HashSet::new();
        for id in &no_parent {
            new_set.insert(id.clone());
        }

        for (parent_id, children) in &parent_map {
            if children.len() >= 2 {
                new_set.insert(parent_id.clone());
            } else {
                // Single child: keep child as-is
                for child in children {
                    new_set.insert(child.clone());
                }
            }
        }

        if new_set == scope_set {
            break; // stable
        }
        scope_set = new_set;
    }

    // Step 4: Build result sorted by element_id for deterministic output
    let mut result: Vec<ImpactScopeRoot> = scope_set
        .into_iter()
        .filter_map(|id| {
            current.get_element(&id).map(|elem| ImpactScopeRoot {
                element_id: id,
                name: elem.name.clone(),
            })
        })
        .collect();
    result.sort_by(|a, b| a.element_id.cmp(&b.element_id));
    result
}

pub fn compute_change_impact(
    current: &graph_registry::GraphRegistry,
    reference: &graph_registry::GraphRegistry,
) -> Result<ChangeImpactReport, ReqvireError> {
    let mut report = ChangeImpactReport::new();
    let current_ids: HashSet<String> = current
        .get_all_elements()
        .iter()
        .map(|e| e.identifier.clone())
        .collect();
    let reference_ids: HashSet<String> = reference
        .get_all_elements()
        .iter()
        .map(|e| e.identifier.clone())
        .collect();

    // Process elements present in both registries.
    for id in current_ids.intersection(&reference_ids) {
        let cur_elem = current.get_element(id).unwrap();
        let ref_elem = reference.get_element(id).unwrap();
        let content_changed = cur_elem.hash_impact_content != ref_elem.hash_impact_content;

        // Compare attachment hashes (resolved from registries) - compare only hash values, not targets
        let cur_attachment_hashes = get_attachment_hashes(cur_elem, current);
        let ref_attachment_hashes = get_attachment_hashes(ref_elem, reference);
        let cur_hash_set: HashSet<&String> = cur_attachment_hashes.iter().map(|(_, h)| h).collect();
        let ref_hash_set: HashSet<&String> = ref_attachment_hashes.iter().map(|(_, h)| h).collect();
        let attachments_changed = cur_hash_set != ref_hash_set;

        // Only track changes to relations that propagate impact according to specifications
        let cur_relations_raw: Vec<_> = cur_elem
            .relations
            .iter()
            .filter(|r| relation::IMPACT_PROPAGATION_RELATIONS.contains(&r.relation_type.name))
            .collect();
        let ref_relations_raw: Vec<_> = ref_elem
            .relations
            .iter()
            .filter(|r| relation::IMPACT_PROPAGATION_RELATIONS.contains(&r.relation_type.name))
            .collect();

        // Normalize relations for semantic comparison (by Element ID, not identifier)
        let cur_relations_normalized: HashSet<_> = cur_relations_raw
            .iter()
            .map(|r| normalize_relation_for_comparison(r))
            .collect();
        let ref_relations_normalized: HashSet<_> = ref_relations_raw
            .iter()
            .map(|r| normalize_relation_for_comparison(r))
            .collect();

        // Find truly added/removed relations based on semantic comparison
        let added_relation_keys: HashSet<_> = cur_relations_normalized
            .difference(&ref_relations_normalized)
            .cloned()
            .collect();
        let removed_relation_keys: HashSet<_> = ref_relations_normalized
            .difference(&cur_relations_normalized)
            .cloned()
            .collect();

        // Map back to actual Relation objects for reporting
        let added_relations: Vec<_> = cur_relations_raw
            .iter()
            .filter(|r| {
                let normalized = normalize_relation_for_comparison(r);
                added_relation_keys.contains(&normalized)
            })
            .map(|r| convert_relation_to_summary(r))
            .collect();
        let removed_relations: Vec<_> = ref_relations_raw
            .iter()
            .filter(|r| {
                let normalized = normalize_relation_for_comparison(r);
                removed_relation_keys.contains(&normalized)
            })
            .map(|r| convert_relation_to_summary(r))
            .collect();

        // Get changed attachments for rendering
        let changed_attachments = get_changed_attachments(cur_elem, ref_elem, current, reference);

        let has_changed = content_changed
            || attachments_changed
            || !added_relations.is_empty()
            || !removed_relations.is_empty();
        if has_changed {
            // Debug: print element changes
            log::debug!(
                "Changed element '{}': content={}, attachments={}, relations={}",
                cur_elem.name,
                content_changed,
                attachments_changed,
                cur_elem.relations.len()
            );
            for rel in &cur_elem.relations {
                log::debug!("  - {} -> {:?}", rel.relation_type.name, rel.target.link);
            }

            let mut visited = BTreeSet::new();
            visited.insert(id.clone());
            let change_impact_tree =
                build_change_impact_tree(current, id.to_string(), &mut visited, None);

            report.changed.push(ChangedElement {
                element_id: id.clone(),
                name: cur_elem.name.clone(),
                old_content: ref_elem.content.clone(),
                new_content: cur_elem.content.clone(),
                content_changed,
                added_relations,
                removed_relations,
                change_impact_tree,
                changed_attachments,
            });
        }
    }
    // Detect relocated elements (same name, different identifier)
    let mut relocated_element_names = HashSet::new();
    let current_by_name: std::collections::HashMap<String, &element::Element> = current
        .get_all_elements()
        .iter()
        .map(|e| (e.name.clone(), *e))
        .collect();
    let reference_by_name: std::collections::HashMap<String, &element::Element> = reference
        .get_all_elements()
        .iter()
        .map(|e| (e.name.clone(), *e))
        .collect();

    for (name, ref_elem) in &reference_by_name {
        if let Some(cur_elem) = current_by_name.get(name) {
            // Same name exists in both registries
            if ref_elem.identifier != cur_elem.identifier {
                // Different identifiers = relocation
                report.relocated.push(RelocatedElement {
                    name: name.clone(),
                    old_identifier: ref_elem.identifier.clone(),
                    new_identifier: cur_elem.identifier.clone(),
                });
                relocated_element_names.insert(name.clone());

                // Check if relocated element also has content or relation changes
                let content_changed = cur_elem.hash_impact_content != ref_elem.hash_impact_content;

                // Use semantic relation comparison (by element name, not identifier)
                let cur_relations_raw: Vec<_> = cur_elem
                    .relations
                    .iter()
                    .filter(|r| {
                        relation::IMPACT_PROPAGATION_RELATIONS.contains(&r.relation_type.name)
                    })
                    .collect();
                let ref_relations_raw: Vec<_> = ref_elem
                    .relations
                    .iter()
                    .filter(|r| {
                        relation::IMPACT_PROPAGATION_RELATIONS.contains(&r.relation_type.name)
                    })
                    .collect();

                let cur_relations_normalized: HashSet<_> = cur_relations_raw
                    .iter()
                    .map(|r| normalize_relation_for_comparison(r))
                    .collect();
                let ref_relations_normalized: HashSet<_> = ref_relations_raw
                    .iter()
                    .map(|r| normalize_relation_for_comparison(r))
                    .collect();

                let added_relation_keys: HashSet<_> = cur_relations_normalized
                    .difference(&ref_relations_normalized)
                    .cloned()
                    .collect();
                let removed_relation_keys: HashSet<_> = ref_relations_normalized
                    .difference(&cur_relations_normalized)
                    .cloned()
                    .collect();

                let added_relations: Vec<_> = cur_relations_raw
                    .iter()
                    .filter(|r| {
                        let normalized = normalize_relation_for_comparison(r);
                        added_relation_keys.contains(&normalized)
                    })
                    .map(|r| convert_relation_to_summary(r))
                    .collect();
                let removed_relations: Vec<_> = ref_relations_raw
                    .iter()
                    .filter(|r| {
                        let normalized = normalize_relation_for_comparison(r);
                        removed_relation_keys.contains(&normalized)
                    })
                    .map(|r| convert_relation_to_summary(r))
                    .collect();

                // Get changed attachments for rendering
                let changed_attachments =
                    get_changed_attachments(cur_elem, ref_elem, current, reference);

                let has_changed = content_changed
                    || !changed_attachments.is_empty()
                    || !added_relations.is_empty()
                    || !removed_relations.is_empty();
                if has_changed {
                    let mut visited = BTreeSet::new();
                    visited.insert(cur_elem.identifier.clone());
                    let change_impact_tree = build_change_impact_tree(
                        current,
                        cur_elem.identifier.to_string(),
                        &mut visited,
                        None,
                    );

                    report.changed.push(ChangedElement {
                        element_id: cur_elem.identifier.clone(),
                        name: cur_elem.name.clone(),
                        old_content: ref_elem.content.clone(),
                        new_content: cur_elem.content.clone(),
                        content_changed,
                        added_relations,
                        removed_relations,
                        change_impact_tree,
                        changed_attachments,
                    });
                }
            }
        }
    }

    // Sort relocated elements by name for deterministic output
    report.relocated.sort_by(|a, b| a.name.cmp(&b.name));

    // Process added elements (present only in current registry, excluding relocated).
    for id in current_ids.difference(&reference_ids) {
        let cur_elem = current.get_element(id).unwrap();
        // Skip if this element was relocated (not truly added)
        if relocated_element_names.contains(&cur_elem.name) {
            continue;
        }
        let added_relations: Vec<_> = cur_elem
            .relations
            .iter()
            .filter(|r| relation::IMPACT_PROPAGATION_RELATIONS.contains(&r.relation_type.name))
            .map(convert_relation_to_summary)
            .collect();
        let mut visited = BTreeSet::new();
        visited.insert(id.clone());
        let change_impact_tree =
            build_change_impact_tree(current, id.to_string(), &mut visited, None);
        report.added.push(AddedElement {
            element_id: id.clone(),
            name: cur_elem.name.clone(),
            new_content: cur_elem.content.clone(),
            added_relations,
            change_impact_tree,
        });
    }
    // Process removed elements (present only in reference registry, excluding relocated).
    for id in reference_ids.difference(&current_ids) {
        let ref_elem = reference.get_element(id).unwrap();
        // Skip if this element was relocated (not truly removed)
        if relocated_element_names.contains(&ref_elem.name) {
            continue;
        }
        let removed_relations: Vec<_> = ref_elem
            .relations
            .iter()
            .map(convert_relation_to_summary)
            .collect();
        report.removed.push(RemovedElement {
            element_id: id.clone(),
            name: ref_elem.name.clone(),
            old_content: ref_elem.content.clone(),
            removed_relations,
        });
    }
    // Collect all changed element IDs to propagate change flags in impact trees
    let changed_element_ids: HashSet<String> = report
        .changed
        .iter()
        .map(|elem| elem.element_id.clone())
        .collect();
    // Use changed_element_ids (all changed elements) instead of content_changed_ids
    // to ensure change impact trees show ⚠️ for all changed elements, not just content changes
    for changed in &mut report.changed {
        propagate_changed_flags(&mut changed.change_impact_tree, &changed_element_ids);
    }
    for added in &mut report.added {
        propagate_changed_flags(&mut added.change_impact_tree, &changed_element_ids);
    }

    // Gather a list of all invalidated verifications from changed and added change impact trees
    let from_added = report
        .added
        .iter()
        .flat_map(|elem| collect_verification_elements_from_impact_tree(&elem.change_impact_tree));
    let from_changed = report
        .changed
        .iter()
        .flat_map(|elem| collect_verification_elements_from_impact_tree(&elem.change_impact_tree));
    let mut inv_ver: Vec<_> = from_added.chain(from_changed).collect();
    inv_ver.sort_by_key(|v| v.element_id.clone());
    inv_ver.dedup_by_key(|v| v.element_id.clone());
    report.invalidated_verifications = inv_ver;

    // Sort all vectors by element_id for deterministic output
    report.added.sort_by(|a, b| a.element_id.cmp(&b.element_id));
    report
        .removed
        .sort_by(|a, b| a.element_id.cmp(&b.element_id));
    report
        .changed
        .sort_by(|a, b| a.element_id.cmp(&b.element_id));

    // Store all added and changed element IDs before smart filtering is applied
    report.all_added_element_ids = report
        .added
        .iter()
        .map(|elem| elem.element_id.clone())
        .collect();
    report.all_changed_element_ids = report
        .changed
        .iter()
        .map(|elem| elem.element_id.clone())
        .collect();

    // Apply smart filtering to eliminate redundant new elements
    apply_smart_filtering(&mut report, current);

    // Compute impact scope (per-branch LCAs of impacted requirements)
    report.impact_scope = compute_impact_scope(current, reference, &report);

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::Element;
    use crate::relation::{Relation, RelationTarget, RelationTypeInfo};
    use crate::GraphRegistry;

    /// Helper function to create a simple element.
    fn create_element(identifier: &str, name: &str, content: &str) -> Element {
        create_typed_element(
            identifier,
            name,
            content,
            crate::element::ElementType::Requirement(crate::element::RequirementType::System),
        )
    }

    fn create_typed_element(
        identifier: &str,
        name: &str,
        content: &str,
        element_type: crate::element::ElementType,
    ) -> Element {
        let mut element = Element::new(name, identifier, "test.md", 1, Some(element_type));
        element.add_content(content);
        element.freeze_content();
        element
    }

    /// Helper function to add a relation to an element.
    fn add_relation(
        element: &mut Element,
        relation_type: &'static RelationTypeInfo,
        target_id: &str,
    ) {
        // Extract element_id from identifier (fragment after #)
        let element_id = crate::utils::extract_path_and_fragment(target_id)
            .1
            .map(|f| f.to_string());
        element.relations.push(Relation {
            relation_type,
            target: RelationTarget {
                text: target_id.to_string(),
                link: relation::LinkType::Identifier(target_id.to_string()),
                element_id,
            },
            user_created: true,
        });
    }

    fn relation_type(name: &str) -> &'static RelationTypeInfo {
        relation::RELATION_TYPES
            .get(name)
            .unwrap_or_else(|| panic!("missing relation type {name}"))
    }

    fn tree_contains_name(node: &ElementNode, name: &str) -> bool {
        node.element.name == name
            || node
                .relations
                .iter()
                .any(|rel| tree_contains_name(&rel.element_node, name))
    }

    fn changed_tree_for<'a>(report: &'a ChangeImpactReport, name: &str) -> &'a ElementNode {
        &report
            .changed
            .iter()
            .find(|element| element.name == name)
            .unwrap_or_else(|| panic!("missing changed element {name}"))
            .change_impact_tree
    }

    fn register_semantic_contract_impact_fixture(
        registry: &mut GraphRegistry,
        ontology_content: &str,
        contract_content: &str,
        requirement_content: &str,
    ) {
        let mut ontology = create_typed_element(
            "test.md#payload-ontology",
            "Payload Ontology",
            ontology_content,
            crate::element::ElementType::Ontology,
        );
        add_relation(
            &mut ontology,
            relation_type("usedBy"),
            "test.md#payload-contract",
        );

        let mut contract = create_typed_element(
            "test.md#payload-contract",
            "Payload Contract",
            contract_content,
            crate::element::ElementType::SemanticContract,
        );
        add_relation(
            &mut contract,
            relation_type("use"),
            "test.md#payload-ontology",
        );
        add_relation(
            &mut contract,
            relation_type("constrain"),
            "test.md#payload-requirement",
        );

        let mut requirement = create_typed_element(
            "test.md#payload-requirement",
            "Payload Requirement",
            requirement_content,
            crate::element::ElementType::Requirement(crate::element::RequirementType::System),
        );
        add_relation(
            &mut requirement,
            relation_type("constrainedBy"),
            "test.md#payload-contract",
        );
        add_relation(
            &mut requirement,
            relation_type("verifiedBy"),
            "test.md#payload-verification",
        );

        let mut verification = create_typed_element(
            "test.md#payload-verification",
            "Payload Verification",
            "verification",
            crate::element::ElementType::Verification(crate::element::VerificationType::Test),
        );
        add_relation(
            &mut verification,
            relation_type("verify"),
            "test.md#payload-requirement",
        );

        registry.register_element(ontology, "test.md").unwrap();
        registry.register_element(contract, "test.md").unwrap();
        registry.register_element(requirement, "test.md").unwrap();
        registry.register_element(verification, "test.md").unwrap();
    }

    #[test]
    fn test_ontology_change_propagates_to_contract_requirement_and_verification() {
        let mut current = GraphRegistry::new();
        let mut reference = GraphRegistry::new();
        register_semantic_contract_impact_fixture(
            &mut current,
            "ontology v2",
            "contract",
            "requirement",
        );
        register_semantic_contract_impact_fixture(
            &mut reference,
            "ontology v1",
            "contract",
            "requirement",
        );

        let report = compute_change_impact(&current, &reference).unwrap();
        let tree = changed_tree_for(&report, "Payload Ontology");

        assert!(tree_contains_name(tree, "Payload Contract"));
        assert!(tree_contains_name(tree, "Payload Requirement"));
        assert!(tree_contains_name(tree, "Payload Verification"));
    }

    #[test]
    fn test_semantic_contract_change_propagates_to_requirement_not_ontology() {
        let mut current = GraphRegistry::new();
        let mut reference = GraphRegistry::new();
        register_semantic_contract_impact_fixture(
            &mut current,
            "ontology",
            "contract v2",
            "requirement",
        );
        register_semantic_contract_impact_fixture(
            &mut reference,
            "ontology",
            "contract v1",
            "requirement",
        );

        let report = compute_change_impact(&current, &reference).unwrap();
        let tree = changed_tree_for(&report, "Payload Contract");

        assert!(tree_contains_name(tree, "Payload Requirement"));
        assert!(tree_contains_name(tree, "Payload Verification"));
        assert!(
            !tree_contains_name(tree, "Payload Ontology"),
            "semantic-contract changes should not propagate through use to ontology"
        );
    }

    #[test]
    fn test_requirement_change_flags_semantic_contract_consistency_review() {
        let mut current = GraphRegistry::new();
        let mut reference = GraphRegistry::new();
        register_semantic_contract_impact_fixture(
            &mut current,
            "ontology",
            "contract",
            "requirement v2",
        );
        register_semantic_contract_impact_fixture(
            &mut reference,
            "ontology",
            "contract",
            "requirement v1",
        );

        let report = compute_change_impact(&current, &reference).unwrap();
        let tree = changed_tree_for(&report, "Payload Requirement");

        assert!(tree_contains_name(tree, "Payload Contract"));
        assert!(tree_contains_name(tree, "Payload Verification"));
    }

    #[test]
    fn test_build_change_impact_tree() {
        let mut my_struct = GraphRegistry::new();
        let element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");
        // Define a forward relation from B to A.
        add_relation(
            &mut element_b,
            &RelationTypeInfo {
                name: "derive",
                opposite: Some("derivedFrom"),
                description: "Element B derives from A",
                arrow: "-->",
                label: "label",
            },
            "A",
        );
        my_struct
            .register_element(element_a.clone(), "file.md")
            .unwrap();
        my_struct
            .register_element(element_b.clone(), "file.md")
            .unwrap();
        let mut visited = BTreeSet::new();
        visited.insert("B".to_string());
        let tree = build_change_impact_tree(&my_struct, "B".to_string(), &mut visited, None);
        assert_eq!(tree.element.identifier, "B");
        assert_eq!(tree.relations.len(), 1);
        // Access the child node via element_node.
        assert_eq!(tree.relations[0].element_node.element.identifier, "A");
        assert_eq!(tree.relations[0].relation_trigger, "derive");
    }
    #[test]
    fn test_tree_with_cycle() {
        let mut my_struct = GraphRegistry::new();
        let mut element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");
        // Create a cycle: A -> B -> A.
        add_relation(
            &mut element_a,
            &RelationTypeInfo {
                name: "derive",
                opposite: Some("containedBy"),
                description: "Element A contains B",
                arrow: "-->",
                label: "label",
            },
            "B",
        );
        add_relation(
            &mut element_b,
            &RelationTypeInfo {
                name: "derive",
                opposite: Some("derivedFrom"),
                description: "Element B derives from A",
                arrow: "-->",
                label: "label",
            },
            "A",
        );
        my_struct
            .register_element(element_a.clone(), "file.md")
            .unwrap();
        my_struct
            .register_element(element_b.clone(), "file.md")
            .unwrap();
        let mut visited = BTreeSet::new();
        visited.insert("A".to_string());
        let tree = build_change_impact_tree(&my_struct, "A".to_string(), &mut visited, None);
        // Check that the cycle is correctly handled and not infinite.
        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);
        // For the relation from A to B.
        assert_eq!(tree.relations[0].element_node.element.identifier, "B");
        assert_eq!(tree.relations[0].relation_trigger, "derive");

        // The child node for B has no relations in its tree because B -> A is not processed by the tree
        // since we only consider Forward relations in change_impact_with_relation
        assert_eq!(tree.relations[0].element_node.relations.len(), 0);

        // However, the original element B still has its relation to A
        assert_eq!(tree.relations[0].element_node.element.relations.len(), 1);
        assert_eq!(
            tree.relations[0].element_node.element.relations[0]
                .target
                .text,
            "A"
        );
    }
    #[test]
    fn test_smart_filtering_with_parent_child_requirements() {
        // Create current registry with parent and child requirements both added
        let mut current_registry = GraphRegistry::new();

        // Create parent requirement with forward relations
        let mut parent_req = create_element(
            "req1.md#parent-requirement",
            "Parent Requirement",
            "Parent content",
        );
        parent_req.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "derive",
                opposite: Some("derivedFrom"),
                description: "Parent derives child",
                arrow: "-->",
                label: "derive",
            },
            target: RelationTarget {
                text: "Child Requirement".to_string(),
                link: LinkType::Identifier("req1.md#child-requirement".to_string()),
                element_id: Some("child-requirement".to_string()),
            },
            user_created: true,
        });
        parent_req.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "verifiedBy",
                opposite: Some("verify"),
                description: "Verified by test",
                arrow: "-->",
                label: "verifiedBy",
            },
            target: RelationTarget {
                text: "Parent Verification".to_string(),
                link: LinkType::Identifier("verify.md#parent-verification".to_string()),
                element_id: Some("parent-verification".to_string()),
            },
            user_created: true,
        });

        // Create child requirement with backward relation
        let mut child_req = create_element(
            "req1.md#child-requirement",
            "Child Requirement",
            "Child content",
        );
        child_req.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "derivedFrom",
                opposite: Some("derive"),
                description: "Child derived from parent",
                arrow: "<--",
                label: "derivedFrom",
            },
            target: RelationTarget {
                text: "Parent Requirement".to_string(),
                link: LinkType::Identifier("req1.md#parent-requirement".to_string()),
                element_id: Some("parent-requirement".to_string()),
            },
            user_created: false, // Auto-generated opposite relations
        });

        // Create a verification with backward relation
        let mut verification = Element::new(
            "Parent Verification",
            "verify.md#parent-verification",
            "verify.md",
            1,
            Some(crate::element::ElementType::Verification(
                crate::element::VerificationType::Test,
            )),
        );
        verification.add_content("Verification content");
        verification.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "verify",
                opposite: Some("verifiedBy"),
                description: "Verifies requirement",
                arrow: "<--",
                label: "verify",
            },
            target: RelationTarget {
                text: "Parent Requirement".to_string(),
                link: LinkType::Identifier("req1.md#parent-requirement".to_string()),
                element_id: Some("parent-requirement".to_string()),
            },
            user_created: false, // Auto-generated opposite relations
        });

        current_registry
            .register_element(parent_req, "req1.md")
            .unwrap();
        current_registry
            .register_element(child_req, "req1.md")
            .unwrap();
        current_registry
            .register_element(verification, "verify.md")
            .unwrap();

        // Create empty reference registry (all elements are new)
        let reference_registry = GraphRegistry::new();

        // Compute change impact
        let report = compute_change_impact(&current_registry, &reference_registry).unwrap();

        // Verify results according to smart filtering requirement:
        // Child requirement is filtered out because it's referenced in parent's relations
        // Verification is also filtered out because it's referenced in parent's relations
        // Only parent requirement should remain
        assert_eq!(
            report.added.len(),
            1,
            "Should have 1 added element after filtering"
        );

        let added_ids: Vec<&str> = report.added.iter().map(|e| e.element_id.as_str()).collect();
        assert!(
            added_ids.contains(&"req1.md#parent-requirement"),
            "Parent requirement should be in added elements"
        );
        assert!(
            !added_ids.contains(&"verify.md#parent-verification"),
            "Verification should be filtered out"
        );
        assert!(
            !added_ids.contains(&"req1.md#child-requirement"),
            "Child requirement should be filtered out"
        );
    }
    #[test]
    fn test_smart_filtering_with_requirement_and_verification() {
        // Create current registry with a requirement and its verification both added
        let mut current_registry = GraphRegistry::new();

        // Create requirement with verifiedBy relation
        let mut requirement = create_element(
            "req.md#new-requirement",
            "New Requirement",
            "Requirement content",
        );
        requirement.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "verifiedBy",
                opposite: Some("verify"),
                description: "Verified by test",
                arrow: "-->",
                label: "verifiedBy",
            },
            target: RelationTarget {
                text: "New Verification".to_string(),
                link: LinkType::Identifier("verify.md#new-verification".to_string()),
                element_id: Some("new-verification".to_string()),
            },
            user_created: true,
        });

        // Create verification with verify relation to requirement
        let mut verification = Element::new(
            "New Verification",
            "verify.md#new-verification",
            "verify.md",
            1,
            Some(crate::element::ElementType::Verification(
                crate::element::VerificationType::Test,
            )),
        );
        verification.add_content("Verification content");
        verification.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "verify",
                opposite: Some("verifiedBy"),
                description: "Verifies requirement",
                arrow: "<--",
                label: "verify",
            },
            target: RelationTarget {
                text: "New Requirement".to_string(),
                link: LinkType::Identifier("req.md#new-requirement".to_string()),
                element_id: Some("new-requirement".to_string()),
            },
            user_created: false, // Auto-generated opposite relations
        });

        current_registry
            .register_element(requirement, "req.md")
            .unwrap();
        current_registry
            .register_element(verification, "verify.md")
            .unwrap();

        // Create empty reference registry (all elements are new)
        let reference_registry = GraphRegistry::new();

        // Compute change impact
        let report = compute_change_impact(&current_registry, &reference_registry).unwrap();

        // According to smart filtering requirement:
        // If both are new, and requirement has verifiedBy pointing to verification,
        // then verification should be filtered out
        assert_eq!(
            report.added.len(),
            1,
            "Should have 1 added element after filtering"
        );

        let added_ids: Vec<&str> = report.added.iter().map(|e| e.element_id.as_str()).collect();
        assert!(
            added_ids.contains(&"req.md#new-requirement"),
            "Requirement should be in added elements"
        );
        assert!(
            !added_ids.contains(&"verify.md#new-verification"),
            "Verification should be filtered out"
        );
    }

    #[test]
    fn test_normalize_relation_unchanged_in_modified_file() {
        // Bug: Auto-generated opposite relations appear as "added" when file is modified
        // even though the underlying user-created relation is unchanged

        // Reference registry (base commit)
        let mut reference_registry = GraphRegistry::new();
        let mut parent_ref = create_element("file.md#parent", "Parent Element", "Parent content");
        let mut child_ref = create_element("file.md#child", "Child Element", "Child content");

        // User-created relation on child
        child_ref.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "derivedFrom",
                opposite: Some("derive"),
                description: "Child derives from parent",
                arrow: "<--",
                label: "derivedFrom",
            },
            target: RelationTarget {
                text: "Parent Element".to_string(),
                link: LinkType::Identifier("file.md#parent".to_string()),
                element_id: Some("parent".to_string()),
            },
            user_created: true,
        });

        // Auto-generated opposite relation on parent
        parent_ref.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "derive",
                opposite: Some("derivedFrom"),
                description: "Parent has child",
                arrow: "-->",
                label: "derive",
            },
            target: RelationTarget {
                text: "Child Element".to_string(),
                link: LinkType::Identifier("file.md#child".to_string()),
                element_id: Some("child".to_string()),
            },
            user_created: false, // AUTO-GENERATED
        });

        reference_registry
            .register_element(parent_ref.clone(), "file.md")
            .unwrap();
        reference_registry
            .register_element(child_ref.clone(), "file.md")
            .unwrap();

        // Current registry (HEAD - file modified but relations unchanged)
        let mut current_registry = GraphRegistry::new();
        let mut parent_curr = create_element("file.md#parent", "Parent Element", "Parent content");
        let mut child_curr = create_element("file.md#child", "Child Element", "Child content");

        // Same user-created relation on child
        child_curr.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "derivedFrom",
                opposite: Some("derive"),
                description: "Child derives from parent",
                arrow: "<--",
                label: "derivedFrom",
            },
            target: RelationTarget {
                text: "Parent Element".to_string(),
                link: LinkType::Identifier("file.md#parent".to_string()),
                element_id: Some("parent".to_string()),
            },
            user_created: true,
        });

        // Same auto-generated opposite relation on parent
        parent_curr.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "derive",
                opposite: Some("derivedFrom"),
                description: "Parent has child",
                arrow: "-->",
                label: "derive",
            },
            target: RelationTarget {
                text: "Child Element".to_string(),
                link: LinkType::Identifier("file.md#child".to_string()),
                element_id: Some("child".to_string()),
            },
            user_created: false, // AUTO-GENERATED
        });

        // Add unrelated element (simulating file change)
        let new_elem = create_element("file.md#newelem", "New Element", "New content");
        current_registry
            .register_element(parent_curr.clone(), "file.md")
            .unwrap();
        current_registry
            .register_element(child_curr, "file.md")
            .unwrap();
        current_registry
            .register_element(new_elem, "file.md")
            .unwrap();

        // Test: Compute change impact
        let report = compute_change_impact(&current_registry, &reference_registry).unwrap();

        // Parent Element should NOT appear in changed elements
        // because its auto-generated derive relation hasn't actually changed
        let parent_in_changed = report
            .changed
            .iter()
            .any(|e| e.element_id == "file.md#parent");

        assert!(
            !parent_in_changed,
            "Parent Element should NOT be in changed elements - auto-generated relation unchanged"
        );
    }

    /// Helper: add a derivedFrom relation (backward/hierarchical) from child to parent
    fn add_derived_from(child: &mut Element, parent_id: &str) {
        let element_id = crate::utils::extract_path_and_fragment(parent_id)
            .1
            .map(|f| f.to_string());
        child.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "derivedFrom",
                opposite: Some("derive"),
                description: "Derived from parent",
                arrow: "<--",
                label: "derivedFrom",
            },
            target: RelationTarget {
                text: parent_id.to_string(),
                link: LinkType::Identifier(parent_id.to_string()),
                element_id,
            },
            user_created: true,
        });
    }

    /// Helper: add forward derive relation from parent to child
    fn add_derive(parent: &mut Element, child_id: &str) {
        let element_id = crate::utils::extract_path_and_fragment(child_id)
            .1
            .map(|f| f.to_string());
        parent.relations.push(Relation {
            relation_type: &RelationTypeInfo {
                name: "derive",
                opposite: Some("derivedFrom"),
                description: "Parent derives child",
                arrow: "-->",
                label: "derive",
            },
            target: RelationTarget {
                text: child_id.to_string(),
                link: LinkType::Identifier(child_id.to_string()),
                element_id,
            },
            user_created: false,
        });
    }

    #[test]
    fn test_impact_scope_merges_siblings() {
        // Two sibling requirements changed -> their parent should be scope root
        //   parent
        //   ├── child_a (changed)
        //   └── child_b (changed)

        let mut current = GraphRegistry::new();
        let mut reference = GraphRegistry::new();

        let mut parent = create_element("req.md#parent", "Parent Req", "Parent content");
        add_derive(&mut parent, "req.md#child-a");
        add_derive(&mut parent, "req.md#child-b");

        let mut child_a = create_element("req.md#child-a", "Child A", "Content A v2");
        add_derived_from(&mut child_a, "req.md#parent");
        let mut child_b = create_element("req.md#child-b", "Child B", "Content B v2");
        add_derived_from(&mut child_b, "req.md#parent");

        current.register_element(parent.clone(), "req.md").unwrap();
        current.register_element(child_a, "req.md").unwrap();
        current.register_element(child_b, "req.md").unwrap();

        // Reference: same structure, different content for children
        let mut ref_parent = create_element("req.md#parent", "Parent Req", "Parent content");
        add_derive(&mut ref_parent, "req.md#child-a");
        add_derive(&mut ref_parent, "req.md#child-b");

        let mut ref_child_a = create_element("req.md#child-a", "Child A", "Content A v1");
        add_derived_from(&mut ref_child_a, "req.md#parent");
        let mut ref_child_b = create_element("req.md#child-b", "Child B", "Content B v1");
        add_derived_from(&mut ref_child_b, "req.md#parent");

        reference.register_element(ref_parent, "req.md").unwrap();
        reference.register_element(ref_child_a, "req.md").unwrap();
        reference.register_element(ref_child_b, "req.md").unwrap();

        let report = compute_change_impact(&current, &reference).unwrap();

        // Both children changed, so they should merge into parent
        assert!(
            !report.impact_scope.is_empty(),
            "Impact scope should not be empty"
        );
        assert_eq!(
            report.impact_scope.len(),
            1,
            "Should have exactly 1 scope root"
        );
        assert_eq!(report.impact_scope[0].element_id, "req.md#parent");
        assert_eq!(report.impact_scope[0].name, "Parent Req");
    }

    #[test]
    fn test_impact_scope_deleted_parent() {
        // Deleted requirement whose parent still exists in current model
        //   parent (exists in both)
        //   └── child (deleted)

        let mut current = GraphRegistry::new();
        let mut reference = GraphRegistry::new();

        let parent = create_element("req.md#parent", "Parent Req", "Parent content");
        current.register_element(parent.clone(), "req.md").unwrap();

        // Reference has both parent and child
        let ref_parent = create_element("req.md#parent", "Parent Req", "Parent content");
        let mut ref_child = create_element("req.md#child", "Child Req", "Child content");
        add_derived_from(&mut ref_child, "req.md#parent");

        reference.register_element(ref_parent, "req.md").unwrap();
        reference.register_element(ref_child, "req.md").unwrap();

        let report = compute_change_impact(&current, &reference).unwrap();

        // Deleted child's parent should appear in scope
        assert_eq!(report.impact_scope.len(), 1, "Should have 1 scope root");
        assert_eq!(report.impact_scope[0].element_id, "req.md#parent");
    }

    #[test]
    fn test_impact_scope_disjoint_branches() {
        // Changes in separate branches should produce separate scope roots
        //   root_a
        //   └── child_a (changed)
        //   root_b
        //   └── child_b (changed)

        let mut current = GraphRegistry::new();
        let mut reference = GraphRegistry::new();

        // Branch A
        let mut root_a = create_element("req.md#root-a", "Root A", "Root A content");
        add_derive(&mut root_a, "req.md#child-a");
        let mut child_a = create_element("req.md#child-a", "Child A", "Child A v2");
        add_derived_from(&mut child_a, "req.md#root-a");

        // Branch B
        let mut root_b = create_element("req.md#root-b", "Root B", "Root B content");
        add_derive(&mut root_b, "req.md#child-b");
        let mut child_b = create_element("req.md#child-b", "Child B", "Child B v2");
        add_derived_from(&mut child_b, "req.md#root-b");

        current.register_element(root_a, "req.md").unwrap();
        current.register_element(child_a, "req.md").unwrap();
        current.register_element(root_b, "req.md").unwrap();
        current.register_element(child_b, "req.md").unwrap();

        // Reference: same structure, different content
        let mut ref_root_a = create_element("req.md#root-a", "Root A", "Root A content");
        add_derive(&mut ref_root_a, "req.md#child-a");
        let mut ref_child_a = create_element("req.md#child-a", "Child A", "Child A v1");
        add_derived_from(&mut ref_child_a, "req.md#root-a");

        let mut ref_root_b = create_element("req.md#root-b", "Root B", "Root B content");
        add_derive(&mut ref_root_b, "req.md#child-b");
        let mut ref_child_b = create_element("req.md#child-b", "Child B", "Child B v1");
        add_derived_from(&mut ref_child_b, "req.md#root-b");

        reference.register_element(ref_root_a, "req.md").unwrap();
        reference.register_element(ref_child_a, "req.md").unwrap();
        reference.register_element(ref_root_b, "req.md").unwrap();
        reference.register_element(ref_child_b, "req.md").unwrap();

        let report = compute_change_impact(&current, &reference).unwrap();

        // Each branch has only 1 changed child, so no merging happens
        // Both children remain as separate scope roots
        assert_eq!(report.impact_scope.len(), 2, "Should have 2 scope roots");
        let ids: Vec<&str> = report
            .impact_scope
            .iter()
            .map(|s| s.element_id.as_str())
            .collect();
        assert!(
            ids.contains(&"req.md#child-a"),
            "Child A should be a scope root"
        );
        assert!(
            ids.contains(&"req.md#child-b"),
            "Child B should be a scope root"
        );
    }

    #[test]
    fn test_impact_scope_single_element() {
        // Single changed element with no sibling -> stays as-is
        //   parent
        //   └── only_child (changed)

        let mut current = GraphRegistry::new();
        let mut reference = GraphRegistry::new();

        let mut parent = create_element("req.md#parent", "Parent Req", "Parent content");
        add_derive(&mut parent, "req.md#only-child");
        let mut only_child = create_element("req.md#only-child", "Only Child", "Child v2");
        add_derived_from(&mut only_child, "req.md#parent");

        current.register_element(parent.clone(), "req.md").unwrap();
        current.register_element(only_child, "req.md").unwrap();

        let mut ref_parent = create_element("req.md#parent", "Parent Req", "Parent content");
        add_derive(&mut ref_parent, "req.md#only-child");
        let mut ref_child = create_element("req.md#only-child", "Only Child", "Child v1");
        add_derived_from(&mut ref_child, "req.md#parent");

        reference.register_element(ref_parent, "req.md").unwrap();
        reference.register_element(ref_child, "req.md").unwrap();

        let report = compute_change_impact(&current, &reference).unwrap();

        // Only one child changed, no sibling to merge with -> child stays as scope root
        assert_eq!(report.impact_scope.len(), 1, "Should have 1 scope root");
        assert_eq!(report.impact_scope[0].element_id, "req.md#only-child");
    }
}
