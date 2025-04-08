use std::collections::{HashSet, BTreeSet};
use serde::Serialize;
use std::path::PathBuf;
use crate::utils;
use crate::relation::{Relation, RelationTarget, LinkType};
use crate::error::ReqFlowError;
use crate::element_registry;
use crate::relation;
use crate::element;
use difference::{Changeset, Difference};



/// Represents a simplified relation for reporting.
#[derive(Debug, Clone, Serialize)]
pub struct RelationSummary {
    pub relation_type: String,
    pub target: RelationTarget,
    pub is_opposite: bool,
}

/// Report for an element that is newly added (only in the current registry).
#[derive(Debug, Serialize)]
pub struct AddedElement {
    pub element_id: String,
    pub new_content: String,
    pub added_relations: Vec<RelationSummary>,
    pub change_impact_tree: element_registry::ElementNode,
}

/// Report for an element that has been removed (only in the reference registry).
#[derive(Debug, Serialize)]
pub struct RemovedElement {
    pub element_id: String,
    pub old_content: String,
    pub removed_relations: Vec<RelationSummary>,
}

/// Report for an element that exists in both registries but has differences.
#[derive(Debug, Serialize)]
pub struct ChangedElement {
    pub element_id: String,
    pub old_content: String,
    pub new_content: String,
    pub content_changed: bool,
    pub added_relations: Vec<RelationSummary>,
    pub removed_relations: Vec<RelationSummary>,
    pub change_impact_tree: element_registry::ElementNode,
}

/// Report detailing changes between two registries.
#[derive(Debug, Serialize)]
pub struct ChangeImpactReport<'a> {
    pub added: Vec<AddedElement>,
    pub removed: Vec<RemovedElement>,
    pub changed: Vec<ChangedElement>,
    #[serde(skip_serializing)]
    specification_folder: &'a PathBuf,
    #[serde(skip_serializing)]
    external_folders: &'a [PathBuf],
}

impl<'a> ChangeImpactReport<'a> {
    pub fn new(specification_folder: &'a PathBuf, external_folders: &'a [PathBuf]) -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            changed: Vec::new(),
            specification_folder,
            external_folders,
        }
    }

    fn to_relative_paths(&self) -> ChangeImpactReport<'a> {
        let mut report = ChangeImpactReport::new(self.specification_folder, self.external_folders);
        let to_relative = |path: &str| {
            utils::get_relative_path(&PathBuf::from(path), self.specification_folder, self.external_folders)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|err| {
                    eprintln!("Error converting path {}: {}", path, err);
                    "FAILED".to_string()
                })
        };

        report.added = self.added.iter().map(|elem| AddedElement {
            element_id: to_relative(&elem.element_id),
            new_content: elem.new_content.clone(),
            added_relations: elem.added_relations.iter().map(|rel| RelationSummary {
                relation_type: rel.relation_type.clone(),
                target: RelationTarget {
                    text: rel.target.text.clone(),
                    link: match rel.target.link {
                        LinkType::Identifier(_) => LinkType::Identifier(to_relative(rel.target.link.as_str())),
                        _ => rel.target.link.clone(),
                    },
                },
                is_opposite: rel.is_opposite,
            }).collect(),
            change_impact_tree: propagate_to_relative(&elem.change_impact_tree, &to_relative),
        }).collect();

        report.removed = self.removed.iter().map(|elem| RemovedElement {
            element_id: to_relative(&elem.element_id),
            old_content: elem.old_content.clone(),
            removed_relations: elem.removed_relations.iter().map(|rel| RelationSummary {
                relation_type: rel.relation_type.clone(),
                target: RelationTarget {
                    text: rel.target.text.clone(),
                    link: match rel.target.link {
                        LinkType::Identifier(_) => LinkType::Identifier(to_relative(rel.target.link.as_str())),
                        _ => rel.target.link.clone(),
                    },
                },
                is_opposite: rel.is_opposite,
            }).collect(),
        }).collect();

        report.changed = self.changed.iter().map(|elem| ChangedElement {
            element_id: to_relative(&elem.element_id),
            old_content: elem.old_content.clone(),
            new_content: elem.new_content.clone(),
            content_changed: elem.content_changed,
            added_relations: elem.added_relations.iter().map(|rel| RelationSummary {
                relation_type: rel.relation_type.clone(),
                target: RelationTarget {
                    text: rel.target.text.clone(),
                    link: match rel.target.link {
                        LinkType::Identifier(_) => LinkType::Identifier(to_relative(rel.target.link.as_str())),
                        _ => rel.target.link.clone(),
                    },
                },
                is_opposite: rel.is_opposite,
            }).collect(),
            removed_relations: elem.removed_relations.iter().map(|rel| RelationSummary {
                relation_type: rel.relation_type.clone(),
                target: RelationTarget {
                    text: rel.target.text.clone(),
                    link: match rel.target.link {
                        LinkType::Identifier(_) => LinkType::Identifier(to_relative(rel.target.link.as_str())),
                        _ => rel.target.link.clone(),
                    },
                },
                is_opposite: rel.is_opposite,
            }).collect(),
            change_impact_tree: propagate_to_relative(&elem.change_impact_tree, &to_relative),
        }).collect();

        report
    }
    
    /// Outputs the report as text with GitHub links included.
    pub fn to_text(&self, base_url: &str, branch_or_commit: &str) -> String {
        let mut output = String::new();
        output.push_str("# Change Impact Report\n\n");

        // Added Elements section
        if !self.added.is_empty() {
            output.push_str("## Added Elements\n\n");
        }
        for elem in &self.added {
            // Insert a GitHub link using the base URL and branch/commit.
            let element_url = format!("{}/blob/{}/{}", base_url, branch_or_commit, elem.element_id);
            output.push_str(&format!("### Element: [{}]({})\n\n", elem.element_id, element_url));
            output.push_str(&format!("#### New Content\n\n{}\n\n", elem.new_content));
            if !elem.added_relations.is_empty() {
                output.push_str("#### Added Relations\n");
                for rel in &elem.added_relations {
                    let target_url = match rel.target.link {
                        LinkType::Identifier(ref id) => format!("{}/blob/{}/{}", base_url, branch_or_commit, id),
                        _ => rel.target.link.as_str().to_string(),
                    };
                    output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, target_url));
                }
                output.push_str("\n");
            }
            let rendered_tree = render_change_impact_tree(&elem.change_impact_tree, 0, base_url, branch_or_commit);
            if !rendered_tree.trim().is_empty() {
                output.push_str("#### Change Impact Tree\n");
                output.push_str(&rendered_tree);
            }
            output.push_str("\n---\n\n");
        }

        // Removed Elements section
        if !self.removed.is_empty() {
            output.push_str("## Removed Elements\n\n");
        }
        for elem in &self.removed {
            let element_url = format!("{}/blob/{}/{}", base_url, branch_or_commit, elem.element_id);
            output.push_str(&format!("### Element: [{}]({})\n\n", elem.element_id, element_url));
            output.push_str(&format!("#### Removed Content\n\n{}\n\n", elem.old_content));
            if !elem.removed_relations.is_empty() {
                output.push_str("#### Removed Relations\n");
                for rel in &elem.removed_relations {
                    let target_url = match rel.target.link {
                        LinkType::Identifier(ref id) => format!("{}/blob/{}/{}", base_url, branch_or_commit, id),
                        _ => rel.target.link.as_str().to_string(),
                    };
                    output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, target_url));
                }
                output.push_str("\n");
            }
            output.push_str("\n---\n\n");
        }

        // Changed Elements section
        if !self.changed.is_empty() {
            output.push_str("## Changed Elements\n\n");
        }
        for elem in &self.changed {
            let element_url = format!("{}/blob/{}/{}", base_url, branch_or_commit, elem.element_id);
            output.push_str(&format!("### Element: [{}]({})\n\n", elem.element_id, element_url));
            let markdown_diff = generate_markdown_diff(&elem.old_content, &elem.new_content);
            output.push_str(&format!("{}\n", markdown_diff));
            if !elem.added_relations.is_empty() {
                output.push_str("#### Added Relations\n");
                for rel in &elem.added_relations {
                    let target_url = match rel.target.link {
                        LinkType::Identifier(ref id) => format!("{}/blob/{}/{}", base_url, branch_or_commit, id),
                        _ => rel.target.link.as_str().to_string(),
                    };
                    output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, target_url));
                }
                output.push_str("\n");
            }
            if !elem.removed_relations.is_empty() {
                output.push_str("#### Removed Relations\n");
                for rel in &elem.removed_relations {
                    let target_url = match rel.target.link {
                        LinkType::Identifier(ref id) => format!("{}/blob/{}/{}", base_url, branch_or_commit, id),
                        _ => rel.target.link.as_str().to_string(),
                    };
                    output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, target_url));
                }
                output.push_str("\n");
            }
            let rendered_tree = render_change_impact_tree(&elem.change_impact_tree, 0, base_url, branch_or_commit);
            if !rendered_tree.trim().is_empty() {
                output.push_str("#### Change Impact Tree\n");
                output.push_str(&rendered_tree);
            }
            output.push_str("\n---\n\n");
        }

        output
    }
    

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "Error serializing report".to_string())
    }

    pub fn print(&self, base_url: &str, branch_or_commit: &str, as_json: bool) {
        if as_json {
            println!("{}", self.to_relative_paths().to_json());
        } else {
            println!("{}", self.to_relative_paths().to_text(base_url, branch_or_commit));
        }
    }    
}

/// Helper to convert an `ElementNode` to use relative paths.
fn propagate_to_relative<F>(node: &element_registry::ElementNode, to_relative: &F) -> element_registry::ElementNode 
where F: Fn(&str) -> String {
     // Convert the element identifier to a relative path
     let mut relative_element =node.element.clone();
     relative_element.identifier= to_relative(&node.element.identifier);



    // Recursively convert the child relations to relative paths
    let relative_relations = node.relations.iter().map(|relation| {
        let mut rel_relative_element = relation.element_node.element.clone();
        rel_relative_element.identifier= to_relative(&relation.element_node.element.identifier);
        

        element_registry::RelationNode {
            relation_trigger: relation.relation_trigger.clone(),
            element_node: propagate_to_relative(&relation.element_node, to_relative)
        }
    }).collect();

    // Return the updated ElementNode with relative paths
    element_registry::ElementNode {
        element: relative_element,
        relations: relative_relations,
    }
}

/// Generate a unified diff in a diff-highlighted markdown code fence.
fn generate_markdown_diff(old: &str, new: &str) -> String {
    let changeset = Changeset::new(old, new, "\n");
    let mut diff_output = String::new();
    for diff in changeset.diffs {
        match diff {
            Difference::Same(ref x) => {
                for line in x.lines() {
                    diff_output.push_str(" ");
                    diff_output.push_str(line);
                    diff_output.push('\n');
                }
            },
            Difference::Rem(ref x) => {
                for line in x.lines() {
                    diff_output.push_str("-");
                    diff_output.push_str(line);
                    diff_output.push('\n');
                }
            },
            Difference::Add(ref x) => {
                for line in x.lines() {
                    diff_output.push_str("+");
                    diff_output.push_str(line);
                    diff_output.push('\n');
                }
            },
        }
    }
    format!("```diff\n{}```", diff_output)
}


/// Render the change impact tree recursively with GitHub links.
fn render_change_impact_tree(
    node: &element_registry::ElementNode,
    indent: usize,
    base_url: &str,
    branch_or_commit: &str,
) -> String {
    let mut output = String::new();
    let pad = "  ".repeat(indent);

    // Render the current node’s relations with GitHub links
    for relation_node in &node.relations {
        let element_url = format!("{}/blob/{}/{}", base_url, branch_or_commit, relation_node.element_node.element.identifier);
        output.push_str(&format!(
            "{}* {}: [{}]({})\n",
            pad,
            relation_node.relation_trigger,
            relation_node.element_node.element.name,
            element_url
        ));
        output.push_str(&render_change_impact_tree(
            &relation_node.element_node,
            indent + 1,
            base_url,
            branch_or_commit,
        ));
    }

    output
}

/// Converts a relation into a summarized representation.
fn convert_relation_to_summary(rel: &Relation) -> RelationSummary {
    RelationSummary {
        relation_type: rel.relation_type.name.to_string(),
        target: rel.target.clone(),
        is_opposite: rel.is_opposite,
    }
}	

/// Recursively walks the ChangeImpactReport and marks elements as invalidated based on conditions.
pub fn mark_report_elements_as_invalidated(report: &mut ChangeImpactReport) {
    // Collect sets of added and changed elements for quick lookup.
    let added_ids: HashSet<String> = report.added.iter()
        .map(|added| added.element_id.clone())
        .collect();

    let changed_ids: HashSet<String> = report.changed.iter()
        .filter(|changed| changed.content_changed)
        .map(|changed| changed.element_id.clone())
        .collect();

    // Mark all added elements
    for added_element in &mut report.added {
        // Mark the root element of the added element as invalidated
        added_element.change_impact_tree.element.invalidated = true;
        // Recursively mark related nodes as invalidated based on sets
        recursively_mark_related_elements(&mut added_element.change_impact_tree, &added_ids, &changed_ids);
    }

    // Mark changed elements if content_changed is true
    for changed_element in &mut report.changed {
        if changed_element.content_changed {
            // Mark the root element of the changed element as invalidated
            changed_element.change_impact_tree.element.invalidated = true;
        }
        // Recursively mark related nodes as invalidated based on sets
        recursively_mark_related_elements(&mut changed_element.change_impact_tree, &added_ids, &changed_ids);
    }
}

/// Recursively marks all related elements as invalidated if they are present in the added or changed sets.
fn recursively_mark_related_elements(
    node: &mut element_registry::ElementNode,
    added_ids: &HashSet<String>,
    changed_ids: &HashSet<String>,
) {
    for relation in &mut node.relations {
        let child_node = &mut relation.element_node;

        // Mark the child node as invalidated if it is in the added or changed set.
        if added_ids.contains(&child_node.element.identifier) || changed_ids.contains(&child_node.element.identifier) {
            child_node.element.invalidated = true;
        }

        // Recursively process child nodes
        recursively_mark_related_elements(child_node, added_ids, changed_ids);
    }
}


/// Builds the change impact tree recursively using `ElementNode` and keeps only forward relations.
pub fn build_change_impact_tree(
    current: &element_registry::ElementRegistry,
    element_id: String,
    visited: &mut BTreeSet<String>,
    fallback_name: Option<String>,
) -> element_registry::ElementNode {
    // Fetch the current element or generate a placeholder
    let element = current
        .elements
        .get(&element_id)
        .cloned()
        .unwrap_or_else(|| {
            let display_name = fallback_name.unwrap_or_else(|| "Missing Element".to_string());

            let mut placeholder = element::Element::new(
                &display_name,
                &element_id,
                "unknown",
                "Placeholder",
                None,
            );
            placeholder.content = "Element referenced but not found in registry".to_string();
            placeholder
        });

    // Recursively collect forward-impacting relation nodes
    let relations = current
        .change_impact_with_relation(&element)
        .into_iter()
        .filter_map(|(impacted_id, rels)| {
            // Skip cycles
            if !visited.insert(impacted_id.clone()) {
                return None;
            }

            // Use the text from the first relation as a fallback display name
            let fallback_name = rels.first().map(|rel| rel.target.text.clone());

            let child_node = build_change_impact_tree(
                current,
                impacted_id.clone(),
                visited,
                fallback_name,
            );

            // Only include forward relations
            let forward_relations: Vec<_> = rels
                .into_iter()
                .filter(|rel| rel.relation_type.direction == relation::RelationDirection::Forward)
                .map(|rel| element_registry::RelationNode {
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

    element_registry::ElementNode {
        element,
        relations,
    }
}


    
/// Computes the change impact report between two registries and builds the change impact trees
/// using the registry’s propagation algorithm. Propagation is computed only for added elements
/// or for elements whose content has changed.
pub fn compute_change_impact<'a>(
    current: &'a element_registry::ElementRegistry,
    reference: &'a element_registry::ElementRegistry,
    specification_folder: &'a PathBuf,
    external_folders: &'a [PathBuf],
) -> Result<ChangeImpactReport<'a>, ReqFlowError> {
    let mut report = ChangeImpactReport::new(specification_folder, external_folders);

    let current_ids: HashSet<&String> = current.elements.keys().collect();
    let reference_ids: HashSet<&String> = reference.elements.keys().collect();


    // Process elements present in both registries.
    for id in current_ids.intersection(&reference_ids) {
        let cur_elem = &current.elements[*id];
        let ref_elem = &reference.elements[*id];
        let content_changed = cur_elem.hash_impact_content != ref_elem.hash_impact_content;
        
        let cur_relations: HashSet<_> = cur_elem.relations.iter().filter(|r| !r.is_opposite).cloned().collect();
        let ref_relations: HashSet<_> = ref_elem.relations.iter().filter(|r| !r.is_opposite).cloned().collect();

        let added_relations: Vec<_> = cur_relations
            .difference(&ref_relations)
            .cloned()
            .map(|rel: Relation| convert_relation_to_summary(&rel))
            .collect();
        let removed_relations: Vec<_> = ref_relations
            .difference(&cur_relations)
            .cloned()
            .map(|rel: Relation| convert_relation_to_summary(&rel))
            .collect();

        let has_changed = content_changed || !added_relations.is_empty() || !removed_relations.is_empty();
        if has_changed {
            let mut visited = BTreeSet::new();
            visited.insert((*id).clone());
            let change_impact_tree = build_change_impact_tree(current, (*id).to_string(), &mut visited,None);

                           
            report.changed.push(ChangedElement {
                element_id: (*id).clone(),
                old_content: ref_elem.content.clone(),
                new_content: cur_elem.content.clone(),
                content_changed,
                added_relations,
                removed_relations,
                change_impact_tree,
            });
        }
    }

    // Process added elements (present only in current registry).
    for id in current_ids.difference(&reference_ids) {
        let cur_elem = &current.elements[*id];
        let added_relations: Vec<_> = cur_elem
            .relations
            .iter()
            .cloned()
            .map(|rel: Relation| convert_relation_to_summary(&rel))
            .collect();
        let mut visited = BTreeSet::new();
        visited.insert((*id).clone());

        let change_impact_tree = build_change_impact_tree(current, (*id).to_string(), &mut visited, None);

        report.added.push(AddedElement {
            element_id: (*id).clone(),
            new_content: cur_elem.content.clone(),
            added_relations,
            change_impact_tree,
        });
    }

    // Process removed elements (present only in reference registry).
    for id in reference_ids.difference(&current_ids) {
        let ref_elem = &reference.elements[*id];
        let removed_relations: Vec<_> = ref_elem
            .relations
            .iter()
            .cloned()
            .map(|rel: Relation| convert_relation_to_summary(&rel))
            .collect();
        report.removed.push(RemovedElement {
            element_id: (*id).clone(),
            old_content: ref_elem.content.clone(),
            removed_relations,
        });
    }

    mark_report_elements_as_invalidated(&mut report);
    
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ElementRegistry;
    use crate::element::Element;
    use crate::relation::{RelationTypeInfo, Relation, RelationTarget, RelationDirection};
    
    /// Helper function to create a simple element.
    fn create_element(identifier: &str, name: &str, content: &str) -> Element {
        let mut element = Element::new(
            name,
            identifier,
            "test.md",
            "TestSection",
            Some(crate::element::ElementType::Requirement(crate::element::RequirementType::System))
        );
        element.content = content.to_string();
        element
    }
        
    /// Helper function to add a relation to an element.
    fn add_relation(element: &mut Element, relation_type: &'static RelationTypeInfo, target_id: &str) {
        element.relations.push(Relation {
            relation_type,
            target: RelationTarget {
                text: target_id.to_string(),
                link: relation::LinkType::Identifier(target_id.to_string()),
            },
            is_opposite: false,
        });
    }
        
    #[test]
    fn test_build_change_impact_tree() {
        let mut my_struct = ElementRegistry::new();
        let element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");

        // Define a forward relation from B to A.
        add_relation(
            &mut element_b,
            &RelationTypeInfo {
                name: "derive",
                direction: RelationDirection::Forward,
                opposite: Some("derivedFrom"),
                description: "Element B derives from A",
            },
            "A",
        );

        my_struct.elements.insert("A".to_string(), element_a.clone());
        my_struct.elements.insert("B".to_string(), element_b.clone());

        let mut visited = BTreeSet::new();
        visited.insert("B".to_string());

        let tree = build_change_impact_tree(
            &ElementRegistry {
                elements: my_struct.elements.clone(),
            },
            "B".to_string(),
            &mut visited,
            None
        );

        assert_eq!(tree.element.identifier, "B");
        assert_eq!(tree.relations.len(), 1);
        // Access the child node via element_node.
        assert_eq!(
            tree.relations[0].element_node.element.identifier,
            "A"
        );
        assert_eq!(tree.relations[0].relation_trigger, "derive");
    }

    #[test]
    fn test_tree_with_cycle() {
        let mut my_struct = ElementRegistry::new();
        let mut element_a = create_element("A", "Element A", "Content A");
        let mut element_b = create_element("B", "Element B", "Content B");

        // Create a cycle: A -> B -> A.
        add_relation(
            &mut element_a,
            &RelationTypeInfo {
                name: "contain",
                direction: RelationDirection::Forward,
                opposite: Some("containedBy"),
                description: "Element A contains B",
            },
            "B",
        );

        add_relation(
            &mut element_b,
            &RelationTypeInfo {
                name: "derive",
                direction: RelationDirection::Forward,
                opposite: Some("derivedFrom"),
                description: "Element B derives from A",
            },
            "A",
        );

        my_struct.elements.insert("A".to_string(), element_a.clone());
        my_struct.elements.insert("B".to_string(), element_b.clone());

        let mut visited = BTreeSet::new();
        visited.insert("A".to_string());

        let tree = build_change_impact_tree(
            &ElementRegistry {
                elements: my_struct.elements.clone(),
            },
            "A".to_string(),
            &mut visited,
            None
        );

        // Check that the cycle is correctly handled and not infinite.
        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);
        // For the relation from A to B.
        assert_eq!(
            tree.relations[0].element_node.element.identifier,
            "B"
        );
        assert_eq!(tree.relations[0].relation_trigger, "contain");
        
        // The child node for B has no relations in its tree because B -> A is not processed by the tree
        // since we only consider Forward relations in change_impact_with_relation
        assert_eq!(tree.relations[0].element_node.relations.len(), 0);
        
        // However, the original element B still has its relation to A
        assert_eq!(
            tree.relations[0].element_node.element.relations.len(),
            1
        );
        assert_eq!(
            tree.relations[0].element_node.element.relations[0].target.text,
            "A"
        );
    }
}
