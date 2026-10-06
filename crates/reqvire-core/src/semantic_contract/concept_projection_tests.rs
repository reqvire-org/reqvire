use super::*;
use crate::exclusions::ExclusionSetBuilder;
use crate::parser::parse_single_element;

const FILE: &str = "model.md";
const FIXTURE: &str =
    include_str!("../../../../tests/test-concept-elements/fixtures/Projection.md.fixture");
const SKOS: &str = "http://www.w3.org/2004/02/skos/core#";

fn put(registry: &mut GraphRegistry, content: &str) {
    registry
        .register_element(
            parse_single_element(content, FILE).expect("test fixture operation should succeed"),
            FILE,
        )
        .expect("test fixture operation should succeed");
}

fn resolve(registry: &mut GraphRegistry) -> (Vec<crate::error::ReqvireError>, SemanticIndex) {
    registry
        .build_relations(
            &ExclusionSetBuilder::new()
                .build()
                .expect("test fixture operation should succeed"),
        )
        .expect("test fixture operation should succeed")
}

fn fixture(reverse: bool) -> GraphRegistry {
    fixture_from_source(FIXTURE, reverse)
}

fn registry_from_source(source: &str, reverse: bool) -> GraphRegistry {
    let mut registry = GraphRegistry::new();
    let mut elements: Vec<_> = source.split("\n### ").skip(1).collect();
    if reverse {
        elements.reverse();
    }
    for content in elements {
        put(&mut registry, &format!("### {content}"));
    }
    registry
}

fn fixture_from_source(source: &str, reverse: bool) -> GraphRegistry {
    let mut registry = registry_from_source(source, reverse);
    let (errors, _) = resolve(&mut registry);
    assert!(errors.is_empty(), "{errors:?}");
    registry
}

fn facts(index: &SemanticIndex) -> BTreeSet<(String, String, String)> {
    index
        .blocks
        .iter()
        .filter(|block| matches!(block.kind, SemanticBlockKind::Concepts))
        .flat_map(|block| &block.quads)
        .filter(|quad| {
            ["broader", "narrower", "related", "exactMatch", "closeMatch"]
                .iter()
                .any(|predicate| quad.predicate.as_str() == format!("{SKOS}{predicate}"))
        })
        .map(|quad| {
            (
                quad.subject.to_string(),
                quad.predicate.to_string(),
                quad.object.to_string(),
            )
        })
        .collect()
}

fn expected_facts() -> BTreeSet<(String, String, String)> {
    let alpha = "https://example.test/projection-alpha#Projection";
    let beta = "https://example.test/projection-beta#ProjectionRemote";
    let child = format!("{alpha}Child");
    let mut output = BTreeSet::new();
    for (predicate, inverse, target) in [
        ("broader", "narrower", format!("{alpha}Parent")),
        ("related", "related", format!("{alpha}Peer")),
        ("related", "related", beta.to_string()),
        ("exactMatch", "exactMatch", beta.to_string()),
        ("closeMatch", "closeMatch", beta.to_string()),
        (
            "exactMatch",
            "exactMatch",
            "https://external.example/Exact".to_string(),
        ),
        (
            "closeMatch",
            "closeMatch",
            "https://external.example/Close".to_string(),
        ),
    ] {
        output.insert((
            format!("<{child}>"),
            format!("<{SKOS}{predicate}>"),
            format!("<{target}>"),
        ));
        output.insert((
            format!("<{target}>"),
            format!("<{SKOS}{inverse}>"),
            format!("<{child}>"),
        ));
    }
    output
}

#[test]
fn concept_projection_normalization_visits_each_element_and_link_once() {
    for size in [3, 12, 48] {
        let mut registry = GraphRegistry::new();
        put(&mut registry, "### Scheme\n#### Metadata\n  * type: concept-scheme\n  * concept_base: https://example.test/sparse\n  * concept_prefix: sparse\n");
        for number in 0..size {
            let relation = if number == 0 {
                String::new()
            } else {
                "  * related: [Term 0](#term-0)\n".to_string()
            };
            put(&mut registry, &format!("### Term {number}\nDefinition.\n#### Metadata\n  * type: concept\n#### Relations\n  * derivedFrom: [Scheme](#scheme)\n{relation}"));
            put(
                &mut registry,
                &format!("### Unrelated {number}\n#### Metadata\n  * type: capability\n"),
            );
        }
        let (errors, _) = resolve(&mut registry);
        assert!(errors.is_empty(), "{errors:?}");
        let links: usize = registry
            .nodes
            .values()
            .filter_map(|node| node.element.concept.as_ref())
            .map(|concept| {
                concept.broader.len()
                    + concept.narrower.len()
                    + concept.related.len()
                    + concept.exact_match.len()
                    + concept.close_match.len()
            })
            .sum();
        CONCEPT_NORMALIZATION_WORK.with(|count| count.set((0, 0)));
        let index = build_semantic_index(&registry);
        assert!(index.diagnostics.is_empty(), "{:?}", index.diagnostics);
        CONCEPT_NORMALIZATION_WORK.with(|count| {
            assert_eq!(
                count.get(),
                (registry.nodes.len(), links),
                "normalization work for {size} concepts plus unrelated elements"
            )
        });
    }
}

#[test]
fn concept_projection_preserves_endpoint_namespaces_and_deterministic_facts() {
    let registry = fixture(false);
    let before = serde_json::to_value(registry.get_all_elements())
        .expect("test fixture operation should succeed");
    let index = build_semantic_index(&registry);
    assert!(index.diagnostics.is_empty(), "{:?}", index.diagnostics);
    let expected = expected_facts();
    assert_eq!(facts(&index), expected);
    for declaration in [
        "  * narrower: [Projection Child](#projection-child)\n",
        "  * broader: [Projection Parent](#projection-parent)\n",
    ] {
        let singly_authored = fixture_from_source(&FIXTURE.replace(declaration, ""), false);
        assert_eq!(facts(&build_semantic_index(&singly_authored)), expected);
    }
    assert_eq!(
        serde_json::to_value(registry.get_all_elements())
            .expect("test fixture operation should succeed"),
        before
    );
    let mut reordered = fixture(true);
    for node in reordered.nodes.values_mut() {
        if let Some(concept) = &mut node.element.concept {
            concept.related.reverse();
            concept.exact_match.reverse();
            concept.close_match.reverse();
        }
    }
    let contents = |index: &SemanticIndex| {
        index
            .blocks
            .iter()
            .filter(|block| matches!(block.kind, SemanticBlockKind::Concepts))
            .map(|block| (block.source.clone(), block.content.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        contents(&index),
        contents(&build_semantic_index(&reordered))
    );
    // Reciprocal authoring emits one fact per direction, and isolated concepts survive.
    assert_eq!(
        index
            .blocks
            .iter()
            .flat_map(|block| &block.quads)
            .filter(|quad| expected.contains(&(
                quad.subject.to_string(),
                quad.predicate.to_string(),
                quad.object.to_string()
            )))
            .count(),
        14
    );
    let isolated = index
        .blocks
        .iter()
        .find(|block| block.source.ends_with("#projection-isolated"))
        .expect("test fixture operation should succeed");
    assert!(isolated.content.contains("skos:Concept"));
    assert!(!isolated.content.contains("skos:related"));
}

#[test]
fn concept_projection_candidates_rebuild_without_changing_accepted_relations() {
    let accepted = fixture(false);
    let accepted_index = build_semantic_index(&accepted);
    let accepted_facts = facts(&accepted_index);
    let mut edited = accepted.clone();
    // Remove a singly authored edge, requiring its generated inverse to disappear too.
    let child = &mut edited
        .nodes
        .get_mut("model.md#projection-child")
        .expect("test fixture operation should succeed")
        .element;
    child
        .concept
        .as_mut()
        .expect("test fixture operation should succeed")
        .exact_match
        .retain(|link| !link.target.ends_with("#projection-remote"));
    let edited_index = build_semantic_index(&edited);
    assert_eq!(facts(&edited_index).len(), accepted_facts.len() - 2);
    assert_eq!(facts(&accepted_index), accepted_facts);
    assert_eq!(facts(&build_semantic_index(&accepted)), accepted_facts);

    for (relation, target, diagnostic) in [
        ("broader", "projection-remote", "crosses concept schemes"),
        ("related", "missing", "missing target"),
    ] {
        let mut invalid = accepted.clone();
        put(&mut invalid, &format!("### Projection Parent\n#### Metadata\n  * type: concept\n#### Relations\n  * derivedFrom: [Projection Alpha Scheme](#projection-alpha-scheme)\n  * {relation}: [Target](#{target})\n"));
        let (errors, _) = resolve(&mut invalid);
        assert!(
            !errors.is_empty(),
            "invalid {relation} to {target} was accepted"
        );
        assert!(
            errors
                .iter()
                .any(|error| error.to_string().to_lowercase().contains(diagnostic)),
            "{errors:?}"
        );
        assert_eq!(facts(&build_semantic_index(&accepted)), accepted_facts);
    }
}

#[test]
fn concept_taxonomy_cycles_reject_canonical_paths_independent_of_authoring_order() {
    let parent_link = "narrower: [Projection Child](#projection-child)";
    for source in [
        FIXTURE.replace(
            parent_link,
            "broader: [Projection Parent](#projection-parent)",
        ),
        FIXTURE.replace(
            parent_link,
            "broader: [Projection Child](#projection-child)",
        ),
        FIXTURE.replace(
            "broader: [Projection Parent](#projection-parent)",
            "narrower: [Projection Parent](#projection-parent)",
        ),
        FIXTURE
            .replace(parent_link, "broader: [Projection Peer](#projection-peer)")
            .replace(
                "related: [Projection Child](#projection-child)",
                "broader: [Projection Child](#projection-child)",
            ),
        FIXTURE
            .replace(parent_link, "broader: [Projection Peer](#projection-peer)")
            .replace(
                "related: [Projection Peer](#projection-peer)",
                "narrower: [Projection Peer](#projection-peer)",
            ),
    ] {
        let mut diagnostics = Vec::new();
        for reverse in [false, true] {
            let (errors, _) = resolve(&mut registry_from_source(&source, reverse));
            let cycles: Vec<_> = errors
                .iter()
                .map(ToString::to_string)
                .filter(|error| error.contains("Concept taxonomy cycle"))
                .collect();
            assert_eq!(cycles.len(), 1, "expected one taxonomy cycle: {errors:?}");
            assert!(cycles[0].contains("model.md#projection-"));
            assert!(cycles[0].contains(" --broader--> "));
            diagnostics.push(cycles);
        }
        assert_eq!(diagnostics[0], diagnostics[1]);
    }
}

#[test]
fn concept_taxonomy_accepts_diamonds_and_symmetric_associations() {
    let source = FIXTURE
        .replace("narrower: [Projection Child](#projection-child)", "narrower: [Projection Child](#projection-child)\n  * broader: [Projection Isolated](#projection-isolated)")
        .replace("related: [Projection Peer](#projection-peer)", "broader: [Projection Peer](#projection-peer)")
        .replace("related: [Projection Child](#projection-child)", "related: [Projection Child](#projection-child)\n  * broader: [Projection Isolated](#projection-isolated)");
    for reverse in [false, true] {
        fixture_from_source(&source, reverse);
    }
}

#[test]
fn concept_taxonomy_deep_chain_validation_uses_bounded_stack() {
    let mut registry = GraphRegistry::new();
    for number in 0..2048 {
        let link = if number == 0 {
            String::new()
        } else {
            format!("  * broader: [Term {}](#term-{})\n", number - 1, number - 1)
        };
        put(
            &mut registry,
            &format!("### Term {number}\n#### Metadata\n  * type: concept\n#### Relations\n{link}"),
        );
    }
    assert!(registry.validate_concept_taxonomy_cycles().is_empty());
    put(&mut registry, "### Term 0\n#### Metadata\n  * type: concept\n#### Relations\n  * broader: [Term 2047](#term-2047)\n");
    let errors = registry.validate_concept_taxonomy_cycles();
    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].to_string().matches("model.md#term-").count(),
        2049
    );
}
