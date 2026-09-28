use super::*;
use crate::element::{ContractBindingEntry, ElementType, RequirementType, SizeEstimate};
use crate::relation::{Relation, RELATION_TYPES};

fn element(name: &str, content: &str) -> Element {
    let mut element = Element::new(
        name,
        &format!("Model.md#{}", name.to_lowercase().replace(' ', "-")),
        "Model.md",
        3,
        Some(ElementType::Capability),
    );
    element.content = content.into();
    element.metadata.insert("type".into(), "capability".into());
    element
}

fn minimal() -> Element {
    let mut element = element("Hash Subject", "Café 測定.");
    for (key, value) in [
        ("owner", "team-a"),
        ("status", "draft"),
        ("priority", "medium"),
        ("risk", "low"),
    ] {
        element.metadata.insert(key.into(), value.into());
    }
    element
}

fn relation(target: &str) -> Relation {
    Relation::new("satisfiedBy", "Display label".into(), target, None).expect("fixture relation")
}

fn binding(target: ContractBindingTarget) -> ContractBindingEntry {
    ContractBindingEntry {
        target,
        content_hash: None,
    }
}

fn bytes(element: &Element) -> Vec<u8> {
    canonical_element_bytes(element).expect("canonical fixture")
}

fn assert_changes(before: &Element, after: &Element) {
    assert_ne!(bytes(before), bytes(after));
    assert_ne!(
        fingerprint(&[before]).expect("before"),
        fingerprint(&[after]).expect("after")
    );
}

fn decode_hex(text: &str) -> Vec<u8> {
    let compact: String = text.split_whitespace().collect();
    (0..compact.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&compact[i..i + 2], 16).expect("fixture hex"))
        .collect()
}

#[test]
fn canonical_bytes_and_digests_match_fixed_v1_fixtures() {
    // The same independent byte fixtures are checked through the real HTTP MCP
    // server by test-model-revision-hashing. No production encoder created them.
    macro_rules! golden {
        ($elements:expr, $name:literal) => {{
            let encoded = canonical_model_bytes($elements).expect("canonical model");
            let expected = decode_hex(include_str!(concat!(
                "../../../../tests/test-model-revision-hashing/expected/",
                $name,
                ".hex"
            )));
            assert_eq!(encoded, expected);
            assert_eq!(
                sha256_hex(&encoded),
                include_str!(concat!(
                    "../../../../tests/test-model-revision-hashing/expected/",
                    $name,
                    ".sha256"
                ))
                .trim()
            );
        }};
    }
    golden!(&[], "empty");
    golden!(&[&minimal()], "minimal");
    golden!(
        &[&element("Omega", "Last."), &element("Alpha", "First.")],
        "multiple"
    );
    let mut rich = minimal();
    rich.element_type = ElementType::Requirement(RequirementType::System);
    rich.metadata.insert("type".into(), "requirement".into());
    rich.metadata.insert("note".into(), String::new());
    rich.relations = vec![
        Relation::new("specify", "Root".into(), "Model.md#root", None).expect("specify"),
        relation("artifacts/a.txt"),
        relation("https://example.test/A%2fb?q=a%20b#C"),
    ];
    rich.contract_bindings = vec![
        binding(ContractBindingTarget::FilePath("contracts/spec.txt".into())),
        binding(ContractBindingTarget::ElementIdentifier(
            "Other.md#spec".into(),
        )),
    ];
    golden!(&[&rich], "rich");
}

#[test]
fn every_scalar_and_authored_metadata_field_affects_the_record() {
    let original = minimal();
    for field in ["identifier", "name", "type", "content", "file_path"] {
        let mut changed = original.clone();
        match field {
            "identifier" => changed.identifier = "Model.md#changed".into(),
            "name" => changed.name.push('!'),
            "type" => changed.element_type = ElementType::Ontology,
            "content" => changed.content.push(' '),
            "file_path" => changed.file_path = "other/Model.md".into(),
            _ => unreachable!(),
        }
        assert_changes(&original, &changed);
    }
    for key in [
        "type",
        "owner",
        "status",
        "priority",
        "risk",
        "ontology_base",
        "ontology_prefix",
        "concept_base",
        "concept_prefix",
        "custom",
    ] {
        let mut before = original.clone();
        before.metadata.insert(key.into(), "before".into());
        let mut after = before.clone();
        after.metadata.insert(key.into(), "after".into());
        assert_changes(&before, &after);
        after.metadata.remove(key);
        assert_changes(&before, &after);
    }
}

#[test]
fn effective_relation_and_binding_tuples_are_sorted_and_deduplicated() {
    let mut original = minimal();
    original.relations = vec![
        relation("b.txt"),
        relation("a.txt"),
        relation("Model.md#alpha"),
    ];
    original.contract_bindings = vec![
        binding(ContractBindingTarget::FilePath("z.txt".into())),
        binding(ContractBindingTarget::ElementIdentifier(
            "Model.md#spec".into(),
        )),
    ];
    let mut permuted = original.clone();
    permuted.relations.reverse();
    permuted.relations.push(original.relations[0].clone());
    permuted.contract_bindings.reverse();
    permuted
        .contract_bindings
        .push(original.contract_bindings[0].clone());
    let mut entries: Vec<_> = original.metadata.iter().collect();
    entries.sort_by(|a, b| b.0.cmp(a.0));
    permuted.metadata = entries
        .into_iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(bytes(&original), bytes(&permuted));

    for edge in &mut permuted.relations {
        edge.user_created = false;
        edge.target.text = "Different display label".into();
        edge.target.element_id = Some("cached-id".into());
    }
    for entry in &mut permuted.contract_bindings {
        entry.content_hash = Some("cached-file-content".into());
    }
    assert_eq!(bytes(&original), bytes(&permuted));
}

#[test]
fn relation_and_binding_target_kinds_and_values_are_distinct() {
    let mut original = minimal();
    original.relations.push(relation("same-text"));
    for target in [
        LinkType::Identifier("same-text".into()),
        LinkType::ExternalUrl("same-text".into()),
        LinkType::InternalPath("different-text".into()),
    ] {
        let mut changed = original.clone();
        changed.relations[0].target.link = target;
        assert_changes(&original, &changed);
    }
    let mut changed = original.clone();
    changed.relations[0].relation_type = &RELATION_TYPES["verifiedBy"];
    assert_changes(&original, &changed);
    changed.relations.clear();
    assert_changes(&original, &changed);

    original
        .contract_bindings
        .push(binding(ContractBindingTarget::FilePath("same-text".into())));
    for target in [
        ContractBindingTarget::ElementIdentifier("same-text".into()),
        ContractBindingTarget::FilePath("different-text".into()),
    ] {
        let mut changed = original.clone();
        changed.contract_bindings[0].target = target;
        assert_changes(&original, &changed);
    }
}

#[test]
fn generated_inverse_edges_are_included_in_endpoint_records() {
    let child = minimal();
    let mut parent = element("Parent", "Parent capability.");
    let forward = Relation::new(
        "derivedFrom",
        parent.name.clone(),
        &parent.identifier,
        Some(parent.id.clone()),
    )
    .expect("forward");
    let inverse = forward
        .to_opposite(&child.name, &child.identifier, &child.id)
        .expect("inverse");
    let before = parent.clone();
    parent.relations.push(inverse);
    assert_changes(&before, &parent);
    let generated = parent.clone();
    parent.relations[0].user_created = true;
    assert_eq!(bytes(&parent), bytes(&generated));
}

#[test]
fn framing_preserves_utf8_lengths_empty_values_and_tuple_boundaries() {
    let mut framed = Vec::new();
    string(&mut framed, "é");
    string(&mut framed, "");
    count(&mut framed, 256);
    assert_eq!(
        framed,
        [0, 0, 0, 0, 0, 0, 0, 2, 0xc3, 0xa9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]
    );
    let mut a = minimal();
    let mut b = a.clone();
    a.name = "ab".into();
    a.content = "c".into();
    b.name = "a".into();
    b.content = "bc".into();
    assert_changes(&a, &b);
    a = minimal();
    b = a.clone();
    a.metadata.insert("ab".into(), "c".into());
    b.metadata.insert("a".into(), "bc".into());
    assert_changes(&a, &b);
    a = minimal();
    b = a.clone();
    a.metadata.insert("empty".into(), String::new());
    assert_changes(&a, &b);
    a = minimal();
    b = a.clone();
    a.relations.push(relation("evidence.txt"));
    b.contract_bindings
        .push(binding(ContractBindingTarget::FilePath(
            "evidence.txt".into(),
        )));
    assert_changes(&a, &b);
}

#[test]
fn runtime_fields_and_parser_bookkeeping_are_excluded() {
    let original = minimal();
    let mut changed = original.clone();
    changed.id = "cached-id".into();
    changed.line_number = 999;
    changed.file_order_index = 42;
    changed.hash_impact_content = "unrelated-fxhash".into();
    changed.changed_since_commit = true;
    changed.size_estimate = Some(SizeEstimate {
        content_bytes: 100,
        rendered_context_bytes: 200,
        estimated_tokens: 25,
    });
    changed
        .metadata
        .insert("_single_element_format".into(), "true".into());
    assert_eq!(bytes(&original), bytes(&changed));
}

#[test]
fn literal_whitespace_and_url_text_are_preserved() {
    let mut a = minimal();
    let mut b = a.clone();
    a.content = "`a b`".into();
    b.content = "`ab`".into();
    assert_changes(&a, &b);
    a = minimal();
    b = a.clone();
    a.relations
        .push(relation("https://example.test/A%2fb?q=a%20b#C"));
    b.relations
        .push(relation("https://example.test/A/b?q=a%20b#C"));
    assert_changes(&a, &b);
    assert!(bytes(&a)
        .windows(b"https://example.test/A%2fb?q=a%20b#C".len())
        .any(|part| part == b"https://example.test/A%2fb?q=a%20b#C"));
}

#[test]
fn internal_paths_use_native_components_and_reject_unresolved_roots() {
    let path = Path::new("folder").join("nested").join("file.txt");
    assert_eq!(
        relative_path(&path).expect("relative path"),
        "folder/nested/file.txt"
    );
    assert!(relative_path(Path::new("../outside.txt")).is_err());
    let absolute = std::env::current_dir().expect("cwd").join("source.md");
    assert!(relative_path(&absolute).is_err());
    let mut bad = minimal();
    bad.file_path = absolute.to_str().expect("UTF-8 cwd").into();
    assert!(canonical_element_bytes(&bad).is_err());
    bad = minimal();
    bad.identifier = "../source.md#element".into();
    assert!(canonical_element_bytes(&bad).is_err());
}

#[cfg(unix)]
#[test]
fn non_utf8_relation_and_binding_paths_fail_without_lossy_hashing() {
    use std::os::unix::ffi::OsStringExt;
    let path = std::path::PathBuf::from(std::ffi::OsString::from_vec(b"bad-\xff.txt".to_vec()));
    let mut bad = minimal();
    let mut edge = relation("valid.txt");
    edge.target.link = LinkType::InternalPath(path.clone());
    bad.relations.push(edge);
    assert!(matches!(
        canonical_element_bytes(&bad),
        Err(ReqvireError::PathError(_))
    ));
    bad.relations.clear();
    bad.contract_bindings
        .push(binding(ContractBindingTarget::FilePath(path)));
    assert!(matches!(
        canonical_element_bytes(&bad),
        Err(ReqvireError::PathError(_))
    ));
}
