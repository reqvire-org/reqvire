use std::collections::{HashSet, BTreeSet};
use serde::Serialize;
use std::path::PathBuf;
use crate::utils;
use crate::relation::{Relation, RelationTarget, LinkType, needs_revalidation, needs_review};
use crate::error::ReqFlowError;
use crate::element_registry;
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
    
    pub fn to_text(&self) -> String {
       let mut output = String::new();
       output.push_str("# Change Impact Report\n\n");

       // Added Elements section
       if ! &self.added.is_empty(){
           output.push_str("## Added Elements\n\n");
       }
       for elem in &self.added {
           output.push_str(&format!("### Element: [{}]({})\n\n", elem.element_id, elem.element_id));
           output.push_str(&format!("#### New Content\n```\n{}\n```\n\n", elem.new_content));
           if !elem.added_relations.iter().filter(|rel| !rel.is_opposite).collect::<Vec::<_>>().is_empty() {
               output.push_str("#### Added Relations\n");                          
               for rel in &elem.added_relations {
                   if !rel.is_opposite {
                       output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, rel.target.link.as_str()));
                   }
               }
               output.push_str("\n");               
           }

           let rendered_tree = render_change_impact_tree(&elem.change_impact_tree, 0);
           if !rendered_tree.trim().is_empty() {
               output.push_str("#### Change Impact Tree\n");
               output.push_str(&rendered_tree);
           }
                      
           output.push_str("\n---\n\n");
       }

       // Removed Elements section
       if ! &self.removed.is_empty(){
           output.push_str("## Removed Elements\n\n");
       }
       

       for elem in &self.removed {
           output.push_str(&format!("### Element: [{}]({})\n\n", elem.element_id, elem.element_id));
           output.push_str(&format!("#### Removed Content\n```\n{}\n```\n\n", elem.old_content));
           if !elem.removed_relations.iter().filter(|rel| !rel.is_opposite).collect::<Vec::<_>>().is_empty() {
               output.push_str("#### Removed Relations\n");                          
               for rel in &elem.removed_relations {
                   if !rel.is_opposite {
                       output.push_str(&format!("- **{}** -> [{}]({})\n", rel.relation_type, rel.target.text, rel.target.link.as_str()));
                   }
               }
               output.push_str("\n");               
           }           
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
           
           let rendered_tree = render_change_impact_tree(&elem.change_impact_tree, 0);
           if !rendered_tree.trim().is_empty() {
               output.push_str("#### Change Impact Tree\n");
               output.push_str(&rendered_tree);
           }
           
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


/// Render the change impact tree recursively with indentation.
fn render_change_impact_tree(node: &element_registry::ElementNode, indent: usize) -> String {
    let mut output = String::new();
    let pad = "  ".repeat(indent);

    // Determine the change message
    let changed_msg = if node.element.invalidated {
        "[changed]"
    } else {
        ""
    };

/*
    // Render the current node
    output.push_str(&format!(
        "{}- [{}]({}) {}\n",
        pad,
        node.element.name,
        node.element.identifier,
        changed_msg
    ));
    */

    // Recursively render relation nodes as children
    for relation_node in &node.relations {
        let needs_rev_or_rew = if needs_revalidation(&relation_node.relation_trigger) {
            format!("{} [revalidate needed]",changed_msg)
        } else if needs_review(&relation_node.relation_trigger) {
            format!("{} [review needed]",changed_msg)
        } else {
            format!("{}",changed_msg)
        };

        output.push_str(&format!(
            "{}  * {}: [{}]({}) {}\n",
            pad,
            relation_node.relation_trigger,
            relation_node.element_node.element.name,
            relation_node.element_node.element.identifier,
            needs_rev_or_rew
        ));

        // Recursively render the child node from the relation node directly
        let child_node = element_registry::ElementNode {
            element: relation_node.element_node.element.clone(),
            relations: relation_node.element_node.relations.clone()
        };
        output.push_str(&render_change_impact_tree(&child_node, indent + 2));
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


/// Recursively marks nodes as invalidated if their element id is either newly added
/// or its content has changed.
fn mark_element_as_invalidated(
    node: &mut element_registry::ElementNode,
    added_ids: &HashSet<String>,
    changed_ids: &HashSet<String>,
) {
    if added_ids.contains(&node.element.identifier) || changed_ids.contains(&node.element.identifier) {
        node.element.invalidated = true;
    }
    for relation in &mut node.relations {
        mark_element_as_invalidated(&mut relation.element_node, added_ids, changed_ids);
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

    // Compute the set of added element IDs (present only in current registry).
    let added_ids: HashSet<String> = current_ids
        .difference(&reference_ids)
        .cloned()
        .cloned()  
        .collect();

    // Compute the set of changed element IDs (present in both registries with changed content).
    let changed_ids: HashSet<String> = current_ids
        .intersection(&reference_ids)
        .filter_map(|id| {
            let cur_elem = &current.elements[*id];
            let ref_elem = &reference.elements[*id];
            if cur_elem.hash_impact_content != ref_elem.hash_impact_content {
                Some((*id).clone())
            } else {
                None
            }
        })
        .collect();

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
            let mut visited = BTreeSet::new();
            visited.insert((*id).clone());
            let mut change_impact_tree = current.build_change_impact_tree(current, (*id).to_string(), &mut visited);

            // Mark nodes as invalidated if they are new or their content has changed.
            mark_element_as_invalidated(&mut change_impact_tree, &added_ids, &changed_ids);
            
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
        let mut change_impact_tree = current.build_change_impact_tree(current, (*id).to_string(), &mut visited);
        
        // Mark nodes as invalidated if they are new or (if applicable) changed.
        mark_element_as_invalidated(&mut change_impact_tree, &added_ids, &changed_ids);

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

    Ok(report)
}

