use log::{debug, warn};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::element::{
    ConceptLink, Element, ElementType, GovernanceMetadataEntry, GovernanceMetadataSource,
    RequirementGovernanceMetadata, SizeEstimate, CONTRACT_BINDINGS_SECTION,
};
use crate::error::ReqvireError;
use crate::git_commands;
use crate::relation::{
    self, get_hierarchical_relation_types, LinkType, CONTRACT_RELATIONS,
    IMPACT_PROPAGATION_RELATIONS, SATISFACTION_RELATIONS,
};
use crate::semantic_contract;
use crate::Relation;
use globset::GlobSet;
use o_kernel::rdf::{subject_iri, term_iri};
use o_kernel::vocab::reserved as owl_reserved;
use o_kernel::{ontology, shacl};
use regex::Regex;

mod content_merge;
mod crud_ops;
mod hierarchy;
mod ontology_context;
mod registration;
mod validation;

use content_merge::{
    extract_content_parts, extract_leading_prose, merge_content_into_details,
    merge_ontology_blocks_into_target, replace_single_fenced_subsection,
};

use crate::concept::concept_local_name as concept_validation_local_name;

fn concept_namespace_iri(base: &str) -> String {
    format!("{}#", base.trim_end_matches('#'))
}

/// Cached regex for matching .md file references in relation targets
static MD_FILE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\.md(?:#|$)").expect("invalid regex pattern"));

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub frontmatter_content: String,
}

impl Page {
    pub fn new(frontmatter_content: String) -> Self {
        Self {
            frontmatter_content,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RelationNode {
    pub relation_trigger: String,
    pub element_node: ElementNode,
}

#[derive(Debug, Clone, Serialize)]
pub struct ElementNode {
    pub element: Element,
    pub relations: Vec<RelationNode>,
}

#[derive(Debug, Clone)]
pub struct GraphRegistry {
    pub nodes: FxHashMap<String, ElementNode>,
    pub pages: FxHashMap<String, Page>,
    pub modified_files: FxHashSet<String>, // Track files modified during CRUD operations
}

struct ConceptPayloadContextUpdate {
    element_id: String,
    iri: String,
    scheme_iri: Option<String>,
    namespace_base: String,
    namespace_prefix: String,
    top_concepts: Vec<ConceptLink>,
}

impl Default for GraphRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphRegistry {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::{Element, ElementType, RequirementType};
    use crate::relation::{LinkType, Relation, RelationTarget, RELATION_TYPES};

    fn make_element(id: &str, name: &str) -> Element {
        let mut element = Element::new(
            name,
            id,
            "file.md",
            1, // Test elements at line 1
            Some(ElementType::Requirement(RequirementType::System)),
        );
        element.content = format!("This is {}", name);
        element.freeze_content();
        element
    }

    fn add_relation(from: &mut Element, relation_type: &'static str, to_id: &str) {
        let relation_info = RELATION_TYPES.get(relation_type).unwrap();
        // Extract element_id from identifier (fragment after #)
        let element_id = crate::utils::extract_path_and_fragment(to_id)
            .1
            .map(|f| f.to_string());
        from.relations.push(Relation {
            relation_type: relation_info,
            target: RelationTarget {
                text: to_id.to_string(),
                link: LinkType::Identifier(to_id.to_string()),
                element_id,
            },
            user_created: true,
        });
    }

    #[test]
    fn populate_size_estimates_adds_non_recursive_element_metadata() {
        let mut registry = GraphRegistry::new();
        let element = make_element("file.md#size-estimate", "Size Estimate");

        registry
            .register_element(element, "file.md")
            .expect("element should register");
        registry
            .populate_size_estimates()
            .expect("size estimates should populate");

        let element = registry
            .get_element("file.md#size-estimate")
            .expect("element should be present");
        let estimate = element
            .size_estimate
            .as_ref()
            .expect("size estimate should be present");

        let mut without_estimate = element.clone();
        without_estimate.size_estimate = None;
        let expected_rendered_context_bytes = serde_json::to_vec(&without_estimate).unwrap().len();

        assert_eq!(estimate.content_bytes, element.content.len());
        assert_eq!(
            estimate.rendered_context_bytes,
            expected_rendered_context_bytes
        );
        assert_eq!(
            estimate.estimated_tokens,
            expected_rendered_context_bytes.div_ceil(4)
        );
    }

    #[test]
    fn test_graph_from_registry_resolves_forward_links() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        let b = make_element("B", "Element B");

        add_relation(&mut a, "derive", "B");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();

        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.relations.len(), 1);
        assert_eq!(a_node.relations[0].relation_trigger, "derive");
        assert_eq!(a_node.relations[0].element_node.element.identifier, "B");
    }

    #[test]
    fn test_update_identifier_updates_links_and_graph() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        let b = make_element("B", "Element B");

        add_relation(&mut a, "derive", "B");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();
        graph.update_identifier("B", "B_NEW");

        // B should no longer exist, B_NEW should
        assert!(graph.nodes.get("B").is_none());
        assert!(graph.nodes.get("B_NEW").is_some());

        // A's relation should now point to B_NEW
        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.relations.len(), 1);
        assert_eq!(a_node.relations[0].element_node.element.identifier, "B_NEW");
    }

    #[test]
    fn test_get_impact_tree_traverses_correctly() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        let mut b = make_element("B", "Element B");
        let c = make_element("C", "Element C");

        add_relation(&mut a, "derive", "B");
        add_relation(&mut b, "derive", "C");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();
        registry.register_element(c.clone(), "file.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();
        let tree = graph.get_impact_tree("A");

        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);

        let b_node = &tree.relations[0].element_node;
        assert_eq!(b_node.element.identifier, "B");
        assert_eq!(b_node.relations.len(), 1);
        assert_eq!(b_node.relations[0].element_node.element.identifier, "C");
    }

    #[test]
    fn test_cycle_is_handled_gracefully() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        let mut b = make_element("B", "Element B");

        // A -> B and B -> A (cycle)
        add_relation(&mut a, "derive", "B");
        add_relation(&mut b, "derive", "A");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();
        let tree = graph.get_impact_tree("A");

        assert_eq!(tree.element.identifier, "A");
        assert_eq!(tree.relations.len(), 1);
        assert_eq!(tree.relations[0].element_node.element.identifier, "B");

        // Because of cycle protection, B should not recurse into A again
        assert_eq!(tree.relations[0].element_node.relations.len(), 0);
    }

    #[test]
    fn test_move_element_to_existing_location() {
        let mut registry = GraphRegistry::new();

        // Create elements in different files
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file2.md".to_string();

        add_relation(&mut a, "derivedFrom", "B");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();

        let mut graph = registry;

        // Move A to B's file
        let result = graph.move_element_to_location("A", "file2.md");
        assert!(result.is_ok());

        // Verify A is now in file2.md
        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.element.file_path, "file2.md");
    }

    #[test]
    fn test_move_element_to_nonexistent_location() {
        let mut registry = GraphRegistry::new();
        let a = make_element("A", "Element A");

        registry.register_element(a.clone(), "file.md").unwrap();
        let mut graph = registry;

        // Try to move to non-existent file
        let result = graph.move_element_to_location("A", "nonexistent.md");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("does not exist in the graph"));
    }

    #[test]
    fn test_get_available_locations() {
        let mut registry = GraphRegistry::new();

        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file2.md".to_string();

        let mut c = make_element("C", "Element C");
        c.file_path = "file1.md".to_string(); // Same file as A

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();
        registry.register_element(c.clone(), "file1.md").unwrap();

        let graph = registry;
        let locations = graph.get_available_locations();

        // Should only have 2 unique files
        assert_eq!(locations.len(), 2);
        assert!(locations.contains(&"file1.md".to_string()));
        assert!(locations.contains(&"file2.md".to_string()));
    }

    #[test]
    fn test_get_move_impact() {
        let mut registry = GraphRegistry::new();
        let a = make_element("A", "Element A");
        let mut b = make_element("B", "Element B");
        let mut c = make_element("C", "Element C");

        // B and C both reference A
        add_relation(&mut b, "derive", "A");
        add_relation(&mut c, "derivedFrom", "A");

        registry.register_element(a.clone(), "file.md").unwrap();
        registry.register_element(b.clone(), "file.md").unwrap();
        registry.register_element(c.clone(), "file.md").unwrap();

        let graph = registry;
        let impact = graph.get_move_impact("A");

        // Both B and C should be affected by moving A
        assert_eq!(impact.len(), 2);
        assert!(impact.contains(&"B".to_string()));
        assert!(impact.contains(&"C".to_string()));
    }

    #[test]
    fn test_move_element_to_new_file() {
        let mut registry = GraphRegistry::new();
        let a = make_element("A", "Element A");

        registry.register_element(a.clone(), "file.md").unwrap();
        let mut graph = registry;

        // Move A to a new file
        let result = graph.move_element_to_new_file("A", "new_file.md");
        assert!(result.is_ok());

        // Verify A is now in the new file
        let a_node = graph.nodes.get("A").unwrap();
        assert_eq!(a_node.element.file_path, "new_file.md");
    }

    #[test]
    fn test_add_file_location() {
        let mut registry = GraphRegistry::new();
        let mut a = make_element("A", "Element A");
        a.file_path = "existing.md".to_string();

        registry.register_element(a.clone(), "existing.md").unwrap();
        let mut graph = registry;

        // Add a new file location
        let result = graph.add_file_location("new_file.md");
        assert!(result.is_ok());

        // Try to add the same file again (should fail)
        let result = graph.add_file_location("existing.md");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_move_element_updates_relation_identifiers() {
        let mut registry = GraphRegistry::new();

        // Create elements A, B, C
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file1.md".to_string();

        let mut c = make_element("C", "Element C");
        c.file_path = "file2.md".to_string();

        // Create relations: B -> A, C -> A
        add_relation(&mut b, "derive", "A");
        add_relation(&mut c, "derive", "A");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file1.md").unwrap();
        registry.register_element(c.clone(), "file2.md").unwrap();

        let mut graph = registry;
        graph.build_relation_graph();

        // Verify initial relations exist
        let b_relations = graph.list_relations("B").unwrap();
        let c_relations = graph.list_relations("C").unwrap();
        assert_eq!(b_relations.len(), 1);
        assert_eq!(c_relations.len(), 1);
        assert_eq!(b_relations[0], ("derive".to_string(), "A".to_string()));
        assert_eq!(c_relations[0], ("derive".to_string(), "A".to_string()));

        // Move A to a new file - this should update its identifier
        let result = graph.move_element_to_new_file("A", "file3.md");
        assert!(result.is_ok());

        // Check that A's location has changed
        let a_element = graph.get_element("A").unwrap();
        assert_eq!(a_element.file_path, "file3.md");

        // CRITICAL: Relations from B and C should still point to A
        // But they should be pointing to the NEW identifier if A's identifier changed
        let b_relations_after = graph.list_relations("B").unwrap();
        let c_relations_after = graph.list_relations("C").unwrap();

        // These should still exist and point to the moved element
        assert_eq!(
            b_relations_after.len(),
            1,
            "B should still have 1 relation after A is moved"
        );
        assert_eq!(
            c_relations_after.len(),
            1,
            "C should still have 1 relation after A is moved"
        );

        // The target should still be "A" (or updated identifier if it changed)
        let b_target = &b_relations_after[0].1;
        let c_target = &c_relations_after[0].1;

        // Verify the targets still exist in the graph
        assert!(
            graph.get_element(b_target).is_some(),
            "B's relation target '{}' should exist in graph",
            b_target
        );
        assert!(
            graph.get_element(c_target).is_some(),
            "C's relation target '{}' should exist in graph",
            c_target
        );
    }

    #[test]
    fn test_move_element_updates_identifiers_in_flushed_markdown() {
        let mut registry = GraphRegistry::new();

        // Create elements A, B where B references A
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file1.md".to_string();

        // B has a relation pointing to A
        add_relation(&mut b, "derivedFrom", "A");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file1.md").unwrap();

        let mut graph = registry;

        // Move A to a different file
        let result = graph.move_element_to_new_file("A", "file2.md");
        assert!(result.is_ok());

        // Check B's original relations in the element

        // The issue: B's element still has a relation pointing to "A"
        // But A is now in file2.md, so the relation should be "file2.md#A" if it's a cross-file reference
        // Or if identifiers include file paths, it should be updated accordingly

        // Check if this would cause issues in markdown generation
        // We expect that when we flush, the relations should be correctly written
        // based on the current location of elements

        // The PROBLEM: B's relation still points to "A" but A is now in a different file
        // When B gets written to file1.md, it should reference A as "file2.md#A" not just "A"

        // Let's check what the markdown would look like:
        let b_element = graph.nodes.get("B").unwrap().element.clone();
        let b_markdown = graph.element_to_markdown_with_context(&b_element, "file1.md", true);
        println!("B's markdown after A is moved:");
        println!("{}", b_markdown);

        // The relation should be "file2.md#A" since A is now in a different file
        // but it's probably still "A" which would be incorrect
        assert!(
            b_markdown.contains("file2.md#A") || b_markdown.contains("[A](file2.md#A)"),
            "B's relation should reference A in its new location: {}",
            b_markdown
        );
    }

    #[test]
    fn test_moved_element_relations_update_paths() {
        let mut registry = GraphRegistry::new();

        // Create elements A, B, C where A has relations to both B and C
        let mut a = make_element("A", "Element A");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("B", "Element B");
        b.file_path = "file2.md".to_string(); // B is in different file

        let mut c = make_element("C", "Element C");
        c.file_path = "file1.md".to_string(); // C is in same file as A initially

        // A has relations to both B (cross-file) and C (same-file)
        add_relation(&mut a, "derivedFrom", "B");
        add_relation(&mut a, "derive", "C");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();
        registry.register_element(c.clone(), "file1.md").unwrap();

        let mut graph = registry;

        // Check A's initial relations in markdown
        let a_element_initial = graph.nodes.get("A").unwrap().element.clone();
        let a_markdown_initial =
            graph.element_to_markdown_with_context(&a_element_initial, "file1.md", true);
        println!("A's initial markdown (in file1.md):");
        println!("{}", a_markdown_initial);

        // A is in file1.md, B is in file2.md, C is in file1.md
        // So A should reference B as "file2.md#B" and C as just "C" (same file)

        // Move A to file3.md
        let result = graph.move_element_to_new_file("A", "file3.md");
        assert!(result.is_ok());

        // Check A's relations after the move
        let a_element_moved = graph.nodes.get("A").unwrap().element.clone();
        let a_markdown_moved =
            graph.element_to_markdown_with_context(&a_element_moved, "file3.md", true);
        println!("A's markdown after move to file3.md:");
        println!("{}", a_markdown_moved);

        // Now A is in file3.md, so:
        // - A should reference B as "file2.md#B" (cross-file, B is in file2.md)
        // - A should reference C as "file1.md#C" (cross-file, C is in file1.md)
        // Both should be cross-file references now since A moved to file3.md

        println!("A's relations after move:");
        for relation in &a_element_moved.relations {
            println!(
                "  {} -> {}",
                relation.relation_type.name,
                match &relation.target.link {
                    crate::relation::LinkType::Identifier(id) => id.clone(),
                    crate::relation::LinkType::InternalPath(path) =>
                        path.to_string_lossy().to_string(),
                    crate::relation::LinkType::ExternalUrl(url) => url.clone(),
                }
            );
        }

        // PROBLEM: A's relations likely still point to "B" and "C"
        // but should now point to "file2.md#B" and "file1.md#C" respectively
        // since A is now in a different file than both of them

        assert!(
            a_markdown_moved.contains("file2.md#B") || a_markdown_moved.contains("[B](file2.md#B)"),
            "A should reference B with file path since they're in different files: {}",
            a_markdown_moved
        );
        assert!(
            a_markdown_moved.contains("file1.md#C") || a_markdown_moved.contains("[C](file1.md#C)"),
            "A should reference C with file path since they're in different files: {}",
            a_markdown_moved
        );
    }

    #[test]
    fn test_flush_creates_proper_markdown_with_cross_file_relations() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create elements in different files with cross-file relations
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "file1.md".to_string();

        let mut b = make_element("ElementB", "Element B Description");
        b.file_path = "file2.md".to_string();

        let mut c = make_element("ElementC", "Element C Description");
        c.file_path = "file1.md".to_string(); // Same file as A

        // Create cross-file relations:
        // A -> B (file1.md -> file2.md)
        // B -> A (file2.md -> file1.md)
        // A -> C (file1.md -> file1.md, same file)
        add_relation(&mut a, "derivedFrom", "ElementB");
        add_relation(&mut a, "derive", "ElementC");
        add_relation(&mut b, "derivedFrom", "ElementA");

        registry.register_element(a.clone(), "file1.md").unwrap();
        registry.register_element(b.clone(), "file2.md").unwrap();
        registry.register_element(c.clone(), "file1.md").unwrap();

        let mut graph = registry;

        // Move ElementB to file3.md to create more cross-file relations
        let result = graph.move_element_to_new_file("ElementB", "file3.md");
        assert!(result.is_ok());

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path, true);
        assert!(result.is_ok());

        // List what files were actually created
        println!("Files created in output directory:");
        for entry in fs::read_dir(output_path).unwrap() {
            let entry = entry.unwrap();
            println!("  {}", entry.file_name().to_string_lossy());
        }

        // Read the generated markdown files and verify their content
        let file1_content = fs::read_to_string(output_path.join("file1.md")).unwrap();
        let file3_content = fs::read_to_string(output_path.join("file3.md")).unwrap();

        // file2.md might not exist if it only contained ElementB which moved to file3.md
        let file2_content = fs::read_to_string(output_path.join("file2.md")).unwrap_or_else(|_| {
            println!("file2.md does not exist (expected if no elements remain in it)");
            String::new()
        });

        println!("=== file1.md content ===");
        println!("{}", file1_content);
        println!("=== file2.md content ===");
        println!("{}", file2_content);
        println!("=== file3.md content ===");
        println!("{}", file3_content);

        // Verify file1.md content (contains ElementA and ElementC)
        assert!(file1_content.contains("### Element A Description"));
        assert!(file1_content.contains("### Element C Description"));
        assert!(!file1_content.contains("### Element B Description")); // ElementB moved to file3.md

        // Verify ElementA's relations in file1.md
        // A -> B should be cross-file reference with proper display name and fragment anchor
        assert!(
            file1_content.contains("[Element B Description](file3.md#ElementB)"),
            "ElementA should reference ElementB with proper display name: {}",
            file1_content
        );

        // A -> C should be same-file reference (no file prefix needed)
        assert!(
            file1_content.contains("[ElementC](ElementC)")
                || file1_content.contains("[ElementC](#ElementC)")
                || file1_content.contains("ElementC"),
            "ElementA should reference ElementC in same file: {}",
            file1_content
        );

        // Verify file3.md content (contains ElementB)
        assert!(file3_content.contains("### Element B Description"));
        assert!(!file3_content.contains("### Element A Description"));
        assert!(!file3_content.contains("### Element C Description"));

        // Verify ElementB's relations in file3.md
        // B -> A should be cross-file reference with proper display name and fragment anchor
        assert!(
            file3_content.contains("[Element A Description](file1.md#ElementA)"),
            "ElementB should reference ElementA with proper display name: {}",
            file3_content
        );

        // Verify no virtual placeholder content appears in any file
        assert!(!file1_content.contains("Virtual placeholder"));
        assert!(!file2_content.contains("Virtual placeholder"));
        assert!(!file3_content.contains("Virtual placeholder"));

        // Verify proper markdown structure - all files start with "# Elements"
        assert!(file1_content.starts_with("# Elements\n"));
        assert!(file3_content.starts_with("# Elements\n"));
    }

    #[test]
    fn test_flush_includes_page_content() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create an element
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "test_file.md".to_string();

        registry
            .register_element(a.clone(), "test_file.md")
            .unwrap();

        // Add page content
        let page =
            Page::new("This is page frontmatter content.\n\nMore page content here.".to_string());
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path, true);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("test_file.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // Verify file header is present - all files start with "# Elements"
        assert!(file_content.starts_with("# Elements\n\n"));

        // Verify page content is included after header and before elements
        assert!(file_content.contains("This is page frontmatter content."));
        assert!(file_content.contains("More page content here."));

        // Verify element is still present
        assert!(file_content.contains("### Element A Description"));

        // Verify order: header, page content, element
        let header_pos = file_content.find("# Elements").unwrap();
        let page_content_pos = file_content
            .find("This is page frontmatter content.")
            .unwrap();
        let element_pos = file_content.find("### Element A Description").unwrap();

        assert!(header_pos < page_content_pos);
        assert!(page_content_pos < element_pos);
    }

    #[test]
    fn test_flush_multiple_elements() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create multiple elements
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "test_file.md".to_string();
        a.file_order_index = 1;

        let mut b = make_element("ElementB", "Element B Description");
        b.file_path = "test_file.md".to_string();
        b.file_order_index = 2;

        registry
            .register_element(a.clone(), "test_file.md")
            .unwrap();
        registry
            .register_element(b.clone(), "test_file.md")
            .unwrap();

        // Add page content
        let page = Page::new("Page frontmatter content.".to_string());
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path, true);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("test_file.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // Verify all content is present
        assert!(file_content.contains("Page frontmatter content."));
        assert!(file_content.contains("### Element A Description"));
        assert!(file_content.contains("### Element B Description"));
    }

    #[test]
    fn test_flush_handles_empty_page_content() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create an element
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "test_file.md".to_string();

        registry
            .register_element(a.clone(), "test_file.md")
            .unwrap();

        // Add empty page content (should be skipped)
        let page = Page::new("   \n\t  \n  ".to_string()); // only whitespace
        registry.pages.insert("test_file.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path, true);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("test_file.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // Verify element is still present
        assert!(file_content.contains("### Element A Description"));
    }

    #[test]
    fn test_flush_always_outputs_elements_header() {
        use std::fs;
        use tempfile::TempDir;

        let mut registry = GraphRegistry::new();

        // Create an element in MOEs.md
        let mut a = make_element("ElementA", "Element A Description");
        a.file_path = "MOEs.md".to_string();

        registry.register_element(a.clone(), "MOEs.md").unwrap();

        // Add page content (without header - parser strips the H1)
        let page = Page::new("This is the MOEs page content.".to_string());
        registry.pages.insert("MOEs.md".to_string(), page);

        let graph = registry;

        // Create temp directory for flush output
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();

        // Flush the graph to markdown files
        let result = graph.flush_to_directory(output_path, true);
        assert!(result.is_ok());

        // Read the generated markdown file
        let file_content = fs::read_to_string(output_path.join("MOEs.md")).unwrap();

        println!("=== Generated file content ===");
        println!("{}", file_content);

        // All specification files should start with "# Elements"
        assert!(file_content.starts_with("# Elements\n\n"));

        // Page content should be included after the header
        assert!(file_content.contains("This is the MOEs page content."));
    }
}
