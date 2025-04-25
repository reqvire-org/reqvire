use std::collections::{HashSet, BTreeSet};
use serde::Serialize;
use std::path::PathBuf;
use crate::utils;
use crate::relation::{Relation, RelationTarget, LinkType};
use crate::error::ReqvireError;
use crate::element_registry;
use crate::relation;
use crate::element;
use difference::{Changeset, Difference};
use std::path::Path;
use serde_json::{json, Value};


/// Represents a simplified relation for reporting.
#[derive(Debug, Clone, Serialize)]
pub struct RelationSummary {
    pub relation_type: String,
    pub target: RelationTarget,
    pub target_changed: bool,    
    #[serde(skip_serializing)]        
    pub is_opposite: bool,
}

impl RelationSummary {
    pub fn to_repo_url(&self, repo_root: &Path, base_url: &str, commit: &str) -> Option<String> {
        match &self.target.link {
            LinkType::Identifier(id) => {
                let path = PathBuf::from(id);
                let relative = path.strip_prefix(repo_root).ok()?;
                Some(format!("{}/blob/{}/{}", base_url, commit, relative.to_string_lossy()))
            }
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
    pub change_impact_tree: element_registry::ElementNode,
}

/// Report for an element that has been removed (only in the reference registry).
#[derive(Debug, Serialize)]
pub struct RemovedElement {
    pub element_id: String,
    pub name: String,
    pub old_content: String,
    pub removed_relations: Vec<RelationSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvalidatedVerification {
    pub element_id: String,    
    pub name: String,
}

/// Report for an element that exists in both registries but has differences.
#[derive(Debug,Serialize)]
pub struct ChangedElement {
    pub element_id: String,
    pub name: String,    
    pub old_content: String,
    pub new_content: String,
    pub content_changed: bool,
    pub added_relations: Vec<RelationSummary>,
    pub removed_relations: Vec<RelationSummary>,
    pub change_impact_tree: element_registry::ElementNode,
}

impl ChangedElement {
    pub fn to_repo_url(&self, repo_root: &Path, base_url: &str, commit: &str) -> String {
        let path = PathBuf::from(&self.element_id);
        let relative = path.strip_prefix(repo_root).unwrap_or(&path);
        format!("{}/blob/{}/{}", base_url, commit, relative.to_string_lossy())
    }
}


/// Report detailing changes between two registries.
#[derive(Debug, Serialize)]
pub struct ChangeImpactReport<'a> {
    pub added: Vec<AddedElement>,
    pub removed: Vec<RemovedElement>,
    pub changed: Vec<ChangedElement>,
    pub invalidated_verifications: Vec<InvalidatedVerification>,    
    #[serde(skip_serializing)]
    specification_folder: &'a PathBuf,
    #[serde(skip_serializing)]
    external_folders: &'a [PathBuf],
    #[serde(skip_serializing)]    
    repo_root: &'a PathBuf
}

impl<'a> ChangeImpactReport<'a> {
    pub fn new(repo_root: &'a PathBuf, specification_folder: &'a PathBuf, external_folders: &'a [PathBuf]) -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            changed: Vec::new(),
            invalidated_verifications: Vec::new(),
            specification_folder,
            external_folders,
            repo_root
        }
    }


   
    fn to_relative_paths(&self) -> ChangeImpactReport<'a> {
        let mut report = ChangeImpactReport::new(self.repo_root, self.specification_folder, self.external_folders);
        let to_relative = |path: &str| {
            utils::get_relative_path_from_root(&PathBuf::from(path), &self.repo_root)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|err| {
                    eprintln!("Error converting path {}: {}", path, err);
                    "FAILED".to_string()
                })
        };

        report.added = self.added.iter().map(|elem| AddedElement {
            element_id: to_relative(&elem.element_id),
            name: elem.name.clone(),
            new_content: elem.new_content.clone(),
            added_relations: elem.added_relations.iter().map(|rel| RelationSummary {
                relation_type: rel.relation_type.clone(),
                target_changed: false,
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
            name: elem.name.clone(),
            old_content: elem.old_content.clone(),
            removed_relations: elem.removed_relations.iter().map(|rel| RelationSummary {
                relation_type: rel.relation_type.clone(),
                target_changed: false,                
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
            name: elem.name.clone(),            
            old_content: elem.old_content.clone(),
            new_content: elem.new_content.clone(),
            content_changed: elem.content_changed,
            added_relations: elem.added_relations.iter().map(|rel| RelationSummary {           
                relation_type: rel.relation_type.clone(),
                target_changed: false,                
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
                target_changed: false,                
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


        report.invalidated_verifications = self.invalidated_verifications
        .iter()
        .map(|v| InvalidatedVerification {
            element_id: to_relative(&v.element_id),
            name: v.name.clone(),
        })
        .collect();
    
        report
    }
    
    /// Outputs the report as json with GitHub links included.    
    pub fn to_json(
        &self,
        base_url: &str,
        git_commit: &str,
        previous_git_commit: &str,
    ) -> serde_json::Value {
    
        
        let added: Vec<_> = self.added.iter().map(|elem| {
            let element_url = format!("{}/blob/{}/{}", base_url, git_commit, elem.element_id);

            let added_relations: Vec<_> = elem.added_relations.iter().map(|rel| {
                let target_url = match rel.target.link {
                    LinkType::Identifier(ref id) => format!("{}/blob/{}/{}", base_url, git_commit, id),
                    _ => rel.target.link.as_str().to_string(),
                };

                json!({
                    "relation_type": rel.relation_type,
                    "target_changed": rel.target_changed,                    
                    "target_text": rel.target.text,
                    "target_url": target_url
                })
            }).collect();

            let impact_tree = render_change_impact_tree_json(&elem.change_impact_tree, base_url, git_commit);

            json!({
                "element_id": element_url,
                "new_content": elem.new_content,
                "added_relations": added_relations,
                "change_impact_tree": impact_tree
            })
        }).collect();

        let removed: Vec<_> = self.removed.iter().map(|elem| {
            let element_url = format!("{}/blob/{}/{}", base_url, previous_git_commit, elem.element_id);

            let removed_relations: Vec<_> = elem.removed_relations.iter().map(|rel| {
                let target_url = match rel.target.link {
                    LinkType::Identifier(ref id) => format!("{}/blob/{}/{}", base_url, previous_git_commit, id),
                    _ => rel.target.link.as_str().to_string(),
                };

                json!({
                    "relation_type": rel.relation_type,
                    "target_changed": rel.target_changed,                                        
                    "target_text": rel.target.text,
                    "target_url": target_url
                })
            }).collect();

            json!({
                "element_id": element_url,
                "old_content": elem.old_content,
                "removed_relations": removed_relations
            })
        }).collect();

        let changed: Vec<_> = self.changed.iter().map(|elem| {
            let element_url = format!("{}/blob/{}/{}", base_url, git_commit, elem.element_id);

            let added_relations: Vec<_> = elem.added_relations.iter().map(|rel| {
                let target_url = match rel.target.link {
                    LinkType::Identifier(ref id) => format!("{}/blob/{}/{}", base_url, git_commit, id),
                    _ => rel.target.link.as_str().to_string(),
                };

                json!({
                    "relation_type": rel.relation_type,
                    "target_changed": rel.target_changed,                                        
                    "target_text": rel.target.text,
                    "target_url": target_url
                })
            }).collect();

            let removed_relations: Vec<_> = elem.removed_relations.iter().map(|rel| {
                let target_url = match rel.target.link {
                    LinkType::Identifier(ref id) => format!("{}/blob/{}/{}", base_url, previous_git_commit, id),
                    _ => rel.target.link.as_str().to_string(),
                };

                json!({
                    "relation_type": rel.relation_type,
                    "target_changed": rel.target_changed,                                        
                    "target_text": rel.target.text,
                    "target_url": target_url
                })
            }).collect();
    
            let impact_tree = render_change_impact_tree_json(&elem.change_impact_tree, base_url, git_commit);

            json!({
                "element_id": element_url,
                "old_content": elem.old_content,
                "new_content": elem.new_content,
                "content_changed": elem.content_changed,
                "added_relations": added_relations,
                "removed_relations": removed_relations,
                "change_impact_tree": impact_tree
            })
        }).collect();

 
        let invalidated_verifications: Vec<_> = self.invalidated_verifications.iter().map(|invalidated_ver| {
            let target_url = format!("{}/blob/{}/{}", base_url, git_commit, invalidated_ver.element_id);
            json!({
                "target_text": invalidated_ver.name,
                "target_url": target_url
            })
        }).collect();

        json!({
            "added": added,
            "removed": removed,
            "changed": changed,
            "invalidated_verifications": invalidated_verifications
        })
    }

    /// Outputs the report as text with GitHub links included.
    pub fn to_text(&self, base_url: &str, git_commit: &str, previous_git_commit: &str) -> String {
        let mut output = String::new();
        output.push_str("## Change Impact Report\n\n");


        // Removed Elements section
        if !self.removed.is_empty() {
            output.push_str("### Removed Elements\n\n");
        }
        
        for elem in &self.removed {
            let element_url = format!("{}/blob/{}/{}", base_url, previous_git_commit, elem.element_id);
            output.push_str("* ");
            output.push_str(&format!(
                "[{}]({})\n",
                elem.name, element_url
            ));                    
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
            output.push_str(&format!(
                "[{}]({})\n",
                elem.name, element_url
            ));
            let rendered_tree = render_change_impact_tree(&elem.change_impact_tree, 2, base_url, git_commit);
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
            output.push_str("* ");
            output.push_str(&format!(
                "[{}]({})\n",
                elem.name, element_url
            ));
            let rendered_tree = render_change_impact_tree(&elem.change_impact_tree, 2, base_url, git_commit);
            if !rendered_tree.trim().is_empty() {
                //let markdown_diff = generate_markdown_diff(&elem.old_content, &elem.new_content);
                //output.push_str(&format!("{}\n", markdown_diff));                
                output.push_str(&rendered_tree);
                output.push_str("\n\n");                                                
            }
        }
        if !self.changed.is_empty() {        
            output.push_str("\n---\n\n");          
        }
            
        
        // Invalidated Verifications Section
        if !self.invalidated_verifications.is_empty() {
            output.push_str("## Invalidated Verifications\n\n");
            for invalidated_ver in &self.invalidated_verifications {
                let target_url = format!("{}/blob/{}/{}", base_url, git_commit, invalidated_ver.element_id);
                output.push_str(&format!("- [ ] [{}]({})\n", invalidated_ver.name, target_url));
            }
            output.push_str("\n");
        }   
        
        if self.removed.is_empty() && self.added.is_empty() && self.changed.is_empty() {
            output.push_str("\nNothing to report...\n");        
        }

        output
    }
    
    pub fn print(&self, base_url: &str, git_commit: &str,  previous_git_commit: &str, as_json: bool) {
        let report_with_relative_paths=self.to_relative_paths();
        if as_json {
            println!("{}",serde_json::to_string_pretty(&report_with_relative_paths.to_json(base_url, git_commit, previous_git_commit)).unwrap());
        } else {
            println!("{}", report_with_relative_paths.to_text(base_url, git_commit, previous_git_commit));
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
fn _generate_markdown_diff(old: &str, new: &str) -> String {
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
    git_commit: &str,
) -> String {
    let mut output = String::new();
    let pad = "  ".repeat(indent);

    for relation_node in &node.relations {
        let target = &relation_node.element_node.element;
        let element_url = format!("{}/blob/{}/{}", base_url, git_commit, target.identifier);

        let change_icon = if target.changed_since_commit { " ⚠️" } else { "" };

        output.push_str(&format!(
            "{}* {} -> [{}]({}){}\n",
            pad,
            relation_node.relation_trigger,
            target.name,
            element_url,
            change_icon
        ));

        output.push_str(&render_change_impact_tree(
            &relation_node.element_node,
            indent + 1,
            base_url,
            git_commit,
        ));
    }

    output
}
/// Render the change impact tree recursively as structured JSON with GitHub URLs.
fn render_change_impact_tree_json(
    node: &element_registry::ElementNode,
    base_url: &str,
    git_commit: &str,
) -> Vec<Value> {
    node.relations.iter().map(|relation_node| {
        let child = &relation_node.element_node;

        // Construct the GitHub URL using the base URL and commit hash for the identifier
        let github_url = format!("{}/blob/{}/{}", base_url, git_commit, &child.element.identifier);

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
            })
        );

        // Return the relation in the desired format
        json!(relation_obj)
    }).collect()
}

/// Converts a relation into a summarized representation.
fn convert_relation_to_summary(rel: &Relation) -> RelationSummary {
    RelationSummary {
        relation_type: rel.relation_type.name.to_string(),
        target_changed: false,
        target: rel.target.clone(),
        is_opposite: rel.is_opposite,
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

fn collect_verification_elements_from_impact_tree(
    node: &element_registry::ElementNode,
) -> Vec<InvalidatedVerification> {
    let mut collected = Vec::new();
    let mut seen = HashSet::new();

    fn walk(
        node: &element_registry::ElementNode,
        seen: &mut HashSet<String>,
        collected: &mut Vec<InvalidatedVerification>,
    ) {
        if let element::ElementType::Verification(_) = node.element.element_type {
            let id = node.element.identifier.clone();
            if seen.insert(id.clone()) {
                collected.push(InvalidatedVerification {
                    element_id: id,
                    name: node.element.name.clone(),
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

fn propagate_changed_flags(
    node: &mut element_registry::ElementNode,
    changed_ids: &HashSet<String>,
) {
    for relation in &mut node.relations {
        let child = &mut relation.element_node;

        if changed_ids.contains(&child.element.identifier) {
            child.element.changed_since_commit = true;
        }

        // Recurse further down the tree
        propagate_changed_flags(child, changed_ids);
    }
}

/// Computes the change impact report between two registries and builds the change impact trees
/// using the registry’s propagation algorithm. Propagation is computed only for added elements
/// or for elements whose content has changed.
pub fn compute_change_impact<'a>(
    current: &'a element_registry::ElementRegistry,
    reference: &'a element_registry::ElementRegistry,
    repo_root: &'a PathBuf,    
    specification_folder: &'a PathBuf,
    external_folders: &'a [PathBuf],
) -> Result<ChangeImpactReport<'a>, ReqvireError> {
    let mut report = ChangeImpactReport::new(repo_root, specification_folder, external_folders);

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
                name: cur_elem.name.clone(),
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
            name: cur_elem.name.clone(),
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
            name: ref_elem.name.clone(),
            old_content: ref_elem.content.clone(),
            removed_relations,
        });
    }


    let content_changed_ids: HashSet<String> = report.changed
        .iter()
        .filter(|e| e.content_changed)
        .map(|e| e.element_id.clone())
        .collect();

    for changed in &mut report.changed {
        propagate_changed_flags(&mut changed.change_impact_tree, &content_changed_ids);
    }
    for added in &mut report.added {
        propagate_changed_flags(&mut added.change_impact_tree, &content_changed_ids);
    }    
      
                
    // Gather a list of all invalidated verifications from changed and added change impache trees
    let from_added = report
        .added.iter()
        .flat_map(|elem| collect_verification_elements_from_impact_tree(&elem.change_impact_tree));

    let from_changed = report
        .changed.iter()
        .flat_map(|elem| collect_verification_elements_from_impact_tree(&elem.change_impact_tree));

    let mut inv_ver: Vec<_>=from_added.chain(from_changed).collect();
    inv_ver.sort_by_key(|v| v.element_id.clone());
    inv_ver.dedup_by_key(|v| v.element_id.clone());
    report.invalidated_verifications =inv_ver;
    
   
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
                arrow: "-->",
                label: "label"
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
                arrow: "-->",
                label: "label"                
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
                arrow: "-->",
                label: "label"                
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
