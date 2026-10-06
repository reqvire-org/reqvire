use reqvire::exclusions::ExclusionSetBuilder;
use reqvire::graph_registry::GraphRegistry;
use reqvire::parser::parse_single_element;
use reqvire::semantic_contract::{build_semantic_index, SemanticIndex};
use reqvire::shacl::test_support::parse_count;

const FILE: &str = "model.md";
const ONTOLOGY: &str = r#"### Vocabulary
#### Metadata
  * type: ontology
  * ontology_base: https://example.org/model
  * ontology_prefix: ex
#### Ontology
```turtle
@prefix ex: <https://example.org/model#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<https://example.org/model> a owl:Ontology .
ex:Item a owl:Class .
ex:name a owl:DatatypeProperty .
```
"#;
const VALID_SHAPES: &str = r#"
ex:Shape a sh:NodeShape ; sh:targetClass ex:Item ; sh:property ex:Property .
ex:Property sh:path ex:name ; sh:datatype xsd:string ; sh:minCount 1 ; sh:maxCount 2 .
"#;

fn put(registry: &mut GraphRegistry, content: &str) {
    let element =
        parse_single_element(content, FILE).expect("test fixture operation should succeed");
    registry
        .register_element(element, FILE)
        .expect("test fixture operation should succeed");
}

fn put_shapes(registry: &mut GraphRegistry, number: usize, shapes: &str) {
    put(
        registry,
        &format!(
            "### Contract {number}\n#### Metadata\n  * type: semantic-contract\n\
             #### Relations\n  * use: [Vocabulary](#vocabulary)\n\
             #### Shapes\n```turtle\n\
             @prefix ex: <https://example.org/model#> .\n\
             @prefix sh: <http://www.w3.org/ns/shacl#> .\n\
             @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n{shapes}\n```\n"
        ),
    );
}

fn fixture(blocks: usize) -> GraphRegistry {
    let mut registry = GraphRegistry::new();
    put(&mut registry, ONTOLOGY);
    for number in 0..blocks {
        put_shapes(&mut registry, number, VALID_SHAPES);
    }
    let exclusions = ExclusionSetBuilder::new()
        .build()
        .expect("test fixture operation should succeed");
    let (errors, _) = registry
        .build_relations(&exclusions)
        .expect("test fixture operation should succeed");
    assert!(errors.is_empty(), "{errors:?}");
    registry
}

fn once_per_block<T>(blocks: usize, operation: impl FnOnce() -> T) -> T {
    let before = parse_count();
    let result = operation();
    assert_eq!(
        parse_count() - before,
        blocks,
        "SHACL compilations per build"
    );
    result
}

fn references(index: &SemanticIndex) -> Vec<(String, String, String)> {
    index
        .shape_references
        .iter()
        .map(|reference| {
            (
                reference.element_identifier.clone(),
                reference.kind.to_string(),
                reference.iri.clone(),
            )
        })
        .collect()
}

#[test]
fn shacl_build_compiles_each_block_once_in_all_validation_paths() {
    let exclusions = ExclusionSetBuilder::new()
        .build()
        .expect("test fixture operation should succeed");
    for blocks in [1, 2, 4] {
        let mut registry = fixture(blocks);
        let index = once_per_block(blocks, || build_semantic_index(&registry));
        assert!(index.diagnostics.is_empty(), "{:?}", index.diagnostics);
        assert_eq!(index.shape_references.len(), blocks * 2);
        let (errors, validated_index) = once_per_block(blocks, || {
            registry
                .build_relations(&exclusions)
                .expect("test fixture operation should succeed")
        });
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(references(&index), references(&validated_index));
        let errors = once_per_block(blocks, || {
            registry
                .validate_semantic_contracts_in_memory()
                .expect("test fixture operation should succeed")
        });
        assert!(errors.is_empty(), "{errors:?}");
        let errors = once_per_block(blocks, || {
            registry
                .validate_semantic_contracts_after_removal("model.md#unrelated")
                .expect("test fixture operation should succeed")
        });
        assert!(errors.is_empty(), "{errors:?}");
    }
}

#[test]
fn shacl_build_edited_candidates_do_not_reuse_accepted_shapes_or_ontology() {
    let accepted = fixture(1);
    let accepted_references = references(&build_semantic_index(&accepted));
    let mut edited = accepted.clone();
    put_shapes(
        &mut edited,
        0,
        &VALID_SHAPES.replace("sh:path ex:name", "sh:path ex:missing"),
    );
    let errors = once_per_block(1, || {
        edited
            .validate_semantic_contracts_in_memory()
            .expect("test fixture operation should succeed")
    });
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].to_string(), "Invalid markdown structure: Semantic reference not found: semantic contract 'model.md#contract-0' references sh:path <https://example.org/model#missing>, but no ontology element declares this IRI. Update or remove the SHACL reference before deleting or editing the declaring ontology.");

    let mut removed = accepted.clone();
    put(
        &mut removed,
        &ONTOLOGY.replace("ex:name a owl:DatatypeProperty .", ""),
    );
    let errors = once_per_block(1, || {
        removed
            .validate_semantic_contracts_after_removal("model.md#vocabulary")
            .expect("test fixture operation should succeed")
    });
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].to_string(), "Invalid markdown structure: Semantic reference not found: semantic contract 'model.md#contract-0' references sh:path <https://example.org/model#name>, but no ontology element declares this IRI. Removed declaration source: model.md#vocabulary. Update or remove the SHACL reference before deleting or editing the declaring ontology.");

    assert!(once_per_block(1, || accepted
        .validate_semantic_contracts_in_memory()
        .expect("test fixture operation should succeed"))
    .is_empty());
    assert_eq!(
        references(&build_semantic_index(&accepted)),
        accepted_references
    );
}

#[test]
fn shacl_build_malformed_shapes_keep_sanity_diagnostics_and_provenance() {
    for (shapes, expected) in [
        (
            "ex:Property a sh:PropertyShape ; sh:path ex:name, ex:Item .",
            "path",
        ),
        (
            "ex:Property sh:path ex:name ; sh:minCount 3 ; sh:maxCount 1 .",
            "maxCount",
        ),
        ("ex:Property sh:path 42 .", "path"),
    ] {
        let mut registry = fixture(1);
        put_shapes(&mut registry, 0, shapes);
        let index = once_per_block(1, || build_semantic_index(&registry));
        assert!(!index.diagnostics.is_empty(), "{shapes}");
        let errors = once_per_block(1, || {
            registry
                .validate_semantic_contracts_in_memory()
                .expect("test fixture operation should succeed")
        });
        let expected_errors: Vec<_> = index.diagnostics.iter().map(|diagnostic| {
            assert_eq!(diagnostic.source, "model.md#contract-0");
            assert_eq!(diagnostic.file_path, FILE);
            assert!(diagnostic.line_number > 0);
            assert!(diagnostic.message.contains(expected), "{}", diagnostic.message);
            format!("Invalid markdown structure: File {}: semantic model element '{}' at line {}: {}", diagnostic.file_path, diagnostic.source, diagnostic.line_number, diagnostic.message)
        }).collect();
        assert_eq!(
            errors.iter().map(ToString::to_string).collect::<Vec<_>>(),
            expected_errors
        );
    }
}
