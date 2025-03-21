use std::collections::HashSet;
use serde::Serialize;
use std::path::PathBuf;
use crate::utils;
use crate::relation::{Relation, RelationTarget, LinkType};
use crate::error::ReqFlowError;
use crate::element_registry;
use difference::{Changeset, Difference};

/// A node in the change impact tree.
#[derive(Debug, Serialize)]
pub struct PropagationNode {
    pub element_id: String,
    /// A summary of the element (e.g. its name).
    pub element_name: String,
    /// Optionally, a snippet of the element’s content.
    pub element_content: Option<String>,
    /// true if the element’s content has changed (or if it’s newly added)
    pub is_changed: bool,
    /// Which relation (by type) triggered this propagation.
    pub trigger_relation: Option<String>,
    /// Children in the propagation chain.
    pub children: Vec<PropagationNode>,
}

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
    /// The change impact tree for this element (wrapped under a dummy root).
    pub change_impact_tree: PropagationNode,
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
    /// The change impact tree for this element (wrapped under a dummy root).
    pub change_impact_tree: PropagationNode,
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

    pub fn to_text(&self) -> String {
       let mut output = String::new();
       output.push_str("# Change Impact Report\n\n");

       // Added Elements section
       output.push_str("## Added Elements\n\n");
       for elem in &self.added {
           output.push_str(&format!("### Element: [{}]({})\n\n", elem.element_id, elem.element_id));
           output.push_str(&format!("#### New Content\n```\n{}\n```\n\n", elem.new_content));
           if !elem.added_relations.is_empty() {
               output.push_str("#### Added Relations\n");                          
               for rel in &elem.added_relations {
                   if !rel.is_opposite {
                       output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, rel.target.link.as_str()));
                   }
               }
               output.push_str("\n");               
           }
           output.push_str("#### Change Impact Tree\n");
           output.push_str(&format!("{}\n", render_change_impact_tree(&elem.change_impact_tree, 0)));
           output.push_str("\n---\n\n");

       }

       // Removed Elements section
       output.push_str("## Removed Elements\n\n");
       for elem in &self.removed {
           output.push_str(&format!("### Element: [{}]({})\n\n", elem.element_id, elem.element_id));
           output.push_str(&format!("#### Removed Content\n```\n{}\n```\n\n", elem.old_content));
           output.push_str("\n---\n\n");
       }

       // Changed Elements section
       output.push_str("## Changed Elements\n\n");
       for elem in &self.changed {
           output.push_str(&format!("### Element: [{}]({})\n\n", elem.element_id, elem.element_id));
           let markdown_diff = generate_markdown_diff(&elem.old_content, &elem.new_content);
           output.push_str(&format!("{}\n", markdown_diff));
           let added_relations: Vec<_> = elem.added_relations.iter().filter(|r| !r.is_opposite).cloned().collect();
           let removed_relations: Vec<_> = elem.removed_relations.iter().filter(|r| !r.is_opposite).cloned().collect();
           
           if !added_relations.is_empty() {
               output.push_str("#### Added Relations\n");
               for rel in &added_relations {
                   output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, rel.target.link.as_str()));
               }
               output.push_str("\n");               
           }
           if !removed_relations.is_empty()  {
               output.push_str("#### Removed Relations\n");
               for rel in &removed_relations {
                   output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, rel.target.link.as_str()));
               }
               output.push_str("\n");               
           }
           output.push_str("#### Change Impact Tree\n");
           output.push_str(&format!("{}\n", render_change_impact_tree(&elem.change_impact_tree, 0)));
           output.push_str("\n---\n\n");
       }


       output
    }

    pub fn to_text_pure(&self) -> String {
        // (Implementation similar to to_text, omitted for brevity.)
        String::new()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "Error serializing report".to_string())
    }

    pub fn print(&self, as_json: bool) {
        if as_json {
            println!("{}", self.to_relative_paths().to_json());
        } else {
            println!("{}", self.to_relative_paths().to_text());
        }
    }
}

/// Render the change impact tree recursively with indentation.
fn render_change_impact_tree(node: &PropagationNode, indent: usize) -> String {
    let mut output = String::new();
    let pad = "  ".repeat(indent);
    // Render current node.
    if let Some(ref relation) = node.trigger_relation {
        output.push_str(&format!("{}- * {}: [{}]({}) {}\n", 
            pad, 
            relation,             
            node.element_name,             
            node.element_id, 
            if node.is_changed { " [changed]" } else { "" }
        ));
    } else {
        output.push_str(&format!("{}- [{}]({}) {}\n", 
            pad, 
            node.element_name, 
            node.element_id,             
            if node.is_changed { " [changed]" } else { "" }
        ));
    }
    /*
    if let Some(ref content) = node.element_content {
        let snippet = if content.len() > 80 { &content[..80] } else { content };
        output.push_str(&format!("{}  > {}\n", pad, snippet));
    }
    */
    // Render children.
    for child in &node.children {
        output.push_str(&render_change_impact_tree(child, indent + 1));
    }
    output
}

/// Helper to convert a PropagationNode to use relative paths.
fn propagate_to_relative<F>(node: &PropagationNode, to_relative: &F) -> PropagationNode 
where F: Fn(&str) -> String {
    PropagationNode {
        element_id: to_relative(&node.element_id),
        element_name: node.element_name.clone(),
        element_content: node.element_content.clone(),
        is_changed: node.is_changed,
        trigger_relation: node.trigger_relation.clone(),
        children: node.children.iter().map(|child| propagate_to_relative(child, to_relative)).collect(),
    }
}

/// Converts a relation into a summarized representation.
fn convert_relation_to_summary(rel: &Relation) -> RelationSummary {
    RelationSummary {
        relation_type: rel.relation_type.name.to_string(),
        target: rel.target.clone(),
        is_opposite: rel.is_opposite,
    }
}		

/// This function uses `change_impact_with_relation` to obtain the flat list of
/// impacted neighbors (with trigger relation), then recursively builds a tree.
/// A visited set is used to prevent cycles.
fn build_change_impact_tree(
    registry: &element_registry::ElementRegistry,
    reference: &element_registry::ElementRegistry,
    root_id: &String,
    visited: &mut HashSet<String>,
) -> Vec<PropagationNode> {
    // Use the new function to get impacted neighbors along with trigger relation.
    let impacted = registry.change_impact_with_relation(
        registry.elements.get(root_id).expect("Root element exists")
    );
    let mut children = Vec::new();
    for (neighbor, trigger_relation) in impacted {
        if !visited.contains(&neighbor) {
            visited.insert(neighbor.clone());
            let (element_name, element_content) = if let Some(e) = registry.elements.get(&neighbor) {
                (e.name.clone(), Some(e.content.clone()))
            } else {
                (neighbor.clone(), None)
            };
            let child_changed = is_element_changed(&neighbor, registry, reference);
            let sub_children = build_change_impact_tree(registry, reference, &neighbor, visited);
            children.push(PropagationNode {
                element_id: neighbor,
                element_name,
                element_content,
                is_changed: child_changed,
                trigger_relation: Some(trigger_relation),
                children: sub_children,
            });
        }
    }
    children
}



/// Determine whether an element with `id` is changed between the current and reference registries.
fn is_element_changed(
    id: &String,
    current: &element_registry::ElementRegistry,
    reference: &element_registry::ElementRegistry,
) -> bool {
    if let Some(cur_elem) = current.elements.get(id) {
         if let Some(ref_elem) = reference.elements.get(id) {
             return cur_elem.hash_impact_content != ref_elem.hash_impact_content;
         }
         return true; // Not present in reference => added
    }
    false
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
        let cur_relations: HashSet<_> = cur_elem.relations.iter().cloned().collect();
        let ref_relations: HashSet<_> = ref_elem.relations.iter().cloned().collect();

        let added_relations: Vec<_> = cur_relations
            .difference(&ref_relations)
            .cloned()
            .filter(|rel: &Relation| !rel.is_opposite)
            .map(|rel: Relation| convert_relation_to_summary(&rel))
            .collect();
        let removed_relations: Vec<_> = ref_relations
            .difference(&cur_relations)
            .cloned()
            .filter(|rel: &Relation| !rel.is_opposite)
            .map(|rel: Relation| convert_relation_to_summary(&rel))
            .collect();

        let has_changed = content_changed || !added_relations.is_empty() || !removed_relations.is_empty();
        if has_changed {
            let mut visited = HashSet::new();
            visited.insert((*id).clone());
            let children = build_change_impact_tree(current, reference, *id, &mut visited);
            let root_node = PropagationNode {
                element_id: (*id).clone(),
                element_name: cur_elem.name.clone(),
                element_content: Some(cur_elem.content.clone()),
                is_changed: content_changed,
                trigger_relation: None,
                children,
            };
            report.changed.push(ChangedElement {
                element_id: (*id).clone(),
                old_content: ref_elem.content.clone(),
                new_content: cur_elem.content.clone(),
                content_changed,
                added_relations,
                removed_relations,
                change_impact_tree: root_node,
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
        let mut visited = HashSet::new();
        visited.insert((*id).clone());
        let children = build_change_impact_tree(current, reference, *id, &mut visited);
        let root_node = PropagationNode {
            element_id: (*id).clone(),
            element_name: cur_elem.name.clone(),
            element_content: Some(cur_elem.content.clone()),
            is_changed: true,
            trigger_relation: None,
            children,
        };

        
        report.added.push(AddedElement {
            element_id: (*id).clone(),
            new_content: cur_elem.content.clone(),
            added_relations,
            change_impact_tree: root_node,
        });
    }

    // Process removed elements (unchanged from before).
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


    
    Ok(report)
}
