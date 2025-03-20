use std::collections::{ HashSet};
use serde::Serialize;

use crate::relation::{LinkType,Relation};
use crate::element_registry::ElementRegistry;

/// Report detailing changes between two registries.
#[derive(Default, Debug, Serialize)]
pub struct ChangeImpactReport {
    /// Identifiers of elements present in the current registry but not in the reference.
    pub added: Vec<String>,
    /// Identifiers of elements present in the reference registry but not in the current.
    pub removed: Vec<String>,
    /// Identifiers of elements whose impactful content (hash) has changed.
    pub changed: Vec<String>,
    /// For elements present in both registries, the relation differences.
    pub relation_diffs: Vec<RelationDiff>,
}

/// Captures the relation differences for an element.
#[derive(Default, Debug, Serialize)]
pub struct RelationDiff {
    pub element_id: String,
    /// Relations present in the current element but not in the reference.
    pub added: Vec<Relation>,
    /// Relations present in the reference element but missing in the current.
    pub removed: Vec<Relation>,
}

/// Computes the change impact by comparing two element registries.
pub fn compute_change_impact(
    current: &ElementRegistry,
    reference: &ElementRegistry,
) -> ChangeImpactReport {
    let mut report = ChangeImpactReport::default();

    // Create sets of element identifiers for both registries.
    let current_ids: HashSet<&String> = current.elements.keys().collect();
    let reference_ids: HashSet<&String> = reference.elements.keys().collect();

    // Identify added elements.
    report.added = current_ids
        .difference(&reference_ids)
        .cloned()
        .cloned()
        .collect();

    // Identify removed elements.
    report.removed = reference_ids
        .difference(&current_ids)
        .cloned()
        .cloned()
        .collect();

    // For elements present in both, compare the impact hash and internal relations.
    for id in current_ids.intersection(&reference_ids) {
        let cur_elem = &current.elements[*id];
        let ref_elem = &reference.elements[*id];

        if cur_elem.hash_impact_content != ref_elem.hash_impact_content {
            report.changed.push((*id).clone());
        }

        // Compute the set of internal relations only.
        let cur_relations: HashSet<Relation> = cur_elem
            .relations
            .iter()
            .filter(|r| matches!(r.target.link, LinkType::Identifier(_)))
            .cloned()
            .collect();
        let ref_relations: HashSet<Relation> = ref_elem
            .relations
            .iter()
            .filter(|r| matches!(r.target.link, LinkType::Identifier(_)))
            .cloned()
            .collect();

        let added_relations: Vec<Relation> = cur_relations.difference(&ref_relations).cloned().collect();
        let removed_relations: Vec<Relation> = ref_relations.difference(&cur_relations).cloned().collect();

        if !added_relations.is_empty() || !removed_relations.is_empty() {
            report.relation_diffs.push(RelationDiff {
                element_id: (*id).clone(),
                added: added_relations,
                removed: removed_relations,
            });
        }
    }

    report
}

impl ChangeImpactReport {
    /// Returns a human-readable plain text representation of the report.
    pub fn to_text(&self) -> String {
        let mut output = String::new();
        output.push_str("Change Impact Report:\n\n");

        output.push_str("Added Elements:\n");
        for id in &self.added {
            output.push_str(&format!("  - {}\n", id));
        }
        output.push_str("\n");

        output.push_str("Removed Elements:\n");
        for id in &self.removed {
            output.push_str(&format!("  - {}\n", id));
        }
        output.push_str("\n");

        output.push_str("Changed Elements:\n");
        for id in &self.changed {
            output.push_str(&format!("  - {}\n", id));
        }
        output.push_str("\n");

        output.push_str("Relation Differences:\n");
        for diff in &self.relation_diffs {
            output.push_str(&format!(" Element: {}\n", diff.element_id));
            if !diff.added.is_empty() {
                output.push_str("  Added Relations:\n");
                for rel in &diff.added {
                    output.push_str(&format!(
                        "    - {} -> {} ({})\n",
                        rel.relation_type.name,
                        rel.target.text,
                        rel.target.link.as_str(),                        
                    ));
                }
            }
            if !diff.removed.is_empty() {
                output.push_str("  Removed Relations:\n");
                for rel in &diff.removed {
                    output.push_str(&format!(
                        "    - {} -> {} ({})\n",
                        rel.relation_type.name,
                        rel.target.text,
                        rel.target.link.as_str(),                        
                    ));
                }
            }
            output.push_str("\n");
        }
        output
    }

    /// Returns a pretty-printed JSON string representation of the report.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| String::from("Error serializing report"))
    }

    /// Prints the report, either as JSON (if as_json is true) or as plain text.
    pub fn print(&self, as_json: bool) {
        if as_json {
            println!("{}", self.to_json());
        } else {
            println!("{}", self.to_text());
        }
    }
}

