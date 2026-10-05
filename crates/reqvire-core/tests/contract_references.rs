use reqvire::exclusions::ExclusionSetBuilder;
use reqvire::tool_interface::ReqvireToolRegistry;
use serde_json::json;
use std::{fs, path::PathBuf, process::Command};

// Exercise the same dispatcher used by MCP without depending on an HTTP listener.
#[test]
fn contract_reference_tools_preserve_semantics_and_refresh_revisions() {
    let workspace = tempfile::tempdir().expect("temporary workspace");
    struct RestoreDirectory(PathBuf);
    impl Drop for RestoreDirectory {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.0).expect("restore workspace");
        }
    }
    let _restore = RestoreDirectory(std::env::current_dir().expect("capture working directory"));
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/test-contract-references");
    for name in [
        "specifications/Model.md",
        "evidence/documentation.txt",
        "evidence/endpoint.txt",
        "evidence/test.txt",
    ] {
        let target = workspace.path().join(name);
        fs::create_dir_all(target.parent().expect("fixture target parent"))
            .expect("create fixture directory");
        fs::copy(fixture.join(name), target).expect("copy fixture");
    }
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(workspace.path())
        .status()
        .expect("initialize Git fixture")
        .success());
    std::env::set_current_dir(workspace.path()).expect("enter fixture workspace");
    let exclusions = ExclusionSetBuilder::new()
        .build()
        .expect("build exclusions");
    let tools = ReqvireToolRegistry::new(true, &exclusions);
    let read = tools
        .call_tool("reqvire.read_element", &json!({"name": "Documentation"}))
        .expect("read referencing requirement");
    assert_eq!(
        read["contract_references"][0]["target"]["ElementIdentifier"],
        "specifications/Model.md#error-response-specification"
    );
    assert_eq!(read["contract_bindings"], json!([]));
    let before = fs::read_to_string("specifications/Model.md").expect("source before target edit");
    let invalid = json!({"file": "specifications/Model.md", "override_existing": true, "content": "### Error Response Specification\n\nContract content.\n\n#### Metadata\n  * type: capability\n"});
    tools
        .call_tool("reqvire.add_element", &invalid)
        .expect_err("referenced target type edit must fail");
    assert_eq!(
        fs::read_to_string("specifications/Model.md").expect("source after rejection"),
        before
    );
    for dry_run in [false, true] {
        let invalid_owner = json!({
            "file": "specifications/Model.md",
            "override_existing": true,
            "dry_run": dry_run,
            "content": "### Contract Owner\n\n#### Metadata\n  * type: capability\n\n#### Relations\n  * derivedFrom: [Contract Provider](#contract-provider)\n  * definedBy: [Error Response Specification](#error-response-specification)\n"
        });
        let error = tools
            .call_tool("reqvire.add_element", &invalid_owner)
            .expect_err("a capability cannot replace a referenced contract's requirement owner");
        let diagnostic = format!("{error:?}");
        assert!(diagnostic.contains("requirement owner"), "{diagnostic}");
        assert_eq!(
            fs::read_to_string("specifications/Model.md").expect("source after owner rejection"),
            before
        );
    }
    let valid = json!({"file": "specifications/Model.md", "override_existing": true, "content": "### Error Response Specification\n\nError responses contain a code, message, and details.\n\n#### Metadata\n  * type: specification\n"});
    tools
        .call_tool("reqvire.add_element", &valid)
        .expect("valid referenced contract override");
    let retained = tools
        .call_tool("reqvire.read_element", &json!({"name": "Documentation"}))
        .expect("read preserved references");
    assert_eq!(retained["contract_references"], read["contract_references"]);
    for filter in [
        json!({"has_contract_references": true}),
        json!({"filter_contract_references": "*#error-response-*", "short": true}),
        json!({"has_contract_references": true, "have_relations": "satisfiedBy", "filter_name": "^Documentation$"}),
    ] {
        let result = tools
            .call_tool("reqvire.search", &filter)
            .expect("search references");
        let names: Vec<_> = result["files"]
            .as_object()
            .expect("search files")
            .values()
            .flat_map(|file| file["elements"].as_array().expect("search elements"))
            .map(|element| element["name"].as_str().expect("element name"))
            .collect();
        assert_eq!(names, vec!["Documentation"]);
    }
    assert!(tools
        .call_tool(
            "reqvire.search",
            &json!({"filter_contract_references": "["})
        )
        .is_err());
    assert!(tools
        .call_tool(
            "reqvire.search",
            &json!({"have_relations": "referenceContract"})
        )
        .is_err());
    let original_revision = tools
        .call_tool("reqvire.model_revision", &json!({}))
        .expect("read original revision")["model_fingerprint"]
        .clone();
    let original_source =
        fs::read_to_string("specifications/Model.md").expect("read original source");
    let invalid = tools.call_tool("reqvire.link", &json!({"source": "Documentation", "relation_type": "bindContract", "target": "specifications/Model.md#other-specification"}));
    let error = format!(
        "{:?}",
        invalid.expect_err("mixed dependency sections must fail")
    );
    assert!(
        error.contains("Contract References") && error.contains("Contract Bindings"),
        "{error}"
    );
    assert_eq!(
        fs::read_to_string("specifications/Model.md").expect("read unchanged source"),
        original_source
    );
    tools.call_tool("reqvire.link", &json!({"source": "Documentation", "relation_type": "referenceContract", "target": "#Other-Specification"})).expect("normalize and link reference through MCP");
    let revised = tools
        .call_tool("reqvire.model_revision", &json!({}))
        .expect("read revised fingerprint")["model_fingerprint"]
        .clone();
    assert_ne!(revised, original_revision);
    let two = tools
        .call_tool("reqvire.read_element", &json!({"name": "Documentation"}))
        .expect("read added reference");
    assert_eq!(
        two["contract_references"]
            .as_array()
            .expect("reference array")
            .len(),
        2
    );
    // Formatting and labels do not alter the canonical reference set.
    let source = fs::read_to_string("specifications/Model.md").expect("read reference source");
    fs::write(
        "specifications/Model.md",
        source.replace(
            "[Other Specification](#other-specification)",
            "[Other label](#other-specification)",
        ),
    )
    .expect("edit reference label");
    assert_eq!(
        tools
            .call_tool("reqvire.model_revision", &json!({}))
            .expect("read revision after external label edit")["model_fingerprint"],
        revised
    );
    tools
        .call_tool(
            "reqvire.unlink",
            &json!({"source": "Documentation", "target": "Other Specification"}),
        )
        .expect("unlink reference through MCP");
    assert_eq!(
        tools
            .call_tool("reqvire.model_revision", &json!({}))
            .expect("read revision after unlink")["model_fingerprint"],
        original_revision
    );
    // A shared dispatcher relink is one atomic operation.
    tools.call_tool("reqvire.relink", &json!({"source": "Documentation", "relation_type": "referenceContract", "from_target": "Error Response Specification", "to_target": "Other Specification"})).expect("relink reference atomically");
    let read = tools
        .call_tool("reqvire.read_element", &json!({"name": "Documentation"}))
        .expect("read relinked requirement");
    assert_eq!(
        read["contract_references"][0]["target"]["ElementIdentifier"],
        "specifications/Model.md#other-specification"
    );
    tools.call_tool("reqvire.link", &json!({"source": "Contract Owner", "relation_type": "referenceContract", "target": "Other Specification"})).expect("acyclic reference through MCP");
    let before = fs::read_to_string("specifications/Model.md").expect("source before cycle");
    let cycle = tools.call_tool("reqvire.link", &json!({"source": "Other Contract Owner", "relation_type": "referenceContract", "target": "Error Response Specification"})).expect_err("cyclic reference must fail through MCP");
    let error = format!("{cycle:?}");
    assert!(
        error.contains("CircularDependency") && error.contains("Contract References"),
        "{error}"
    );
    assert_eq!(
        fs::read_to_string("specifications/Model.md").expect("source after cycle rejection"),
        before
    );
    let start = before
        .find("### Other Contract Owner\n")
        .expect("owner header");
    let end = start + before[start..].find("\n---").expect("owner boundary");
    let replacement = before[start..end].replace(
        "#### Relations",
        "#### Contract References\n  * [Error Response Specification](#error-response-specification)\n\n#### Relations",
    );
    for (tool, arguments) in [
        (
            "reqvire.add_element",
            json!({"file": "specifications/Model.md", "content": replacement, "override_existing": true}),
        ),
        (
            "reqvire.link",
            json!({"source": "Other Contract Owner", "relation_type": "bindContract", "target": "specifications/Model.md#error-response-specification"}),
        ),
    ] {
        let error = format!(
            "{:?}",
            tools
                .call_tool(tool, &arguments)
                .expect_err("cycle mutation must fail")
        );
        assert!(
            error.contains("CircularDependency") && error.contains("Contract References"),
            "{error}"
        );
        assert_eq!(
            fs::read_to_string("specifications/Model.md").expect("source after rejection"),
            before
        );
    }
    // A relink can close a cycle while every individual endpoint remains valid.
    tools.call_tool("reqvire.add_element", &json!({"file": "specifications/Model.md", "content": "### Documentation Specification\n\nDocumentation contract.\n\n#### Metadata\n  * type: specification\n\n#### Relations\n  * define: [Documentation](#documentation)\n"})).expect("create owned contract");
    tools.call_tool("reqvire.relink", &json!({"source": "Documentation", "relation_type": "referenceContract", "from_target": "Other Specification", "to_target": "Error Response Specification"})).expect("acyclic relink");
    tools
        .call_tool(
            "reqvire.unlink",
            &json!({"source": "Contract Owner", "target": "Other Specification"}),
        )
        .expect("remove earlier dependency");
    tools.call_tool("reqvire.link", &json!({"source": "Other Contract Owner", "relation_type": "referenceContract", "target": "Documentation Specification"})).expect("acyclic dependency");
    let before =
        fs::read_to_string("specifications/Model.md").expect("source before cyclic relink");
    let error = format!("{:?}", tools.call_tool("reqvire.relink", &json!({"source": "Documentation", "relation_type": "referenceContract", "from_target": "Error Response Specification", "to_target": "Other Specification"})).expect_err("cyclic relink must fail"));
    assert!(
        error.contains("CircularDependency") && error.contains("Contract References"),
        "{error}"
    );
    assert_eq!(
        fs::read_to_string("specifications/Model.md").expect("source after relink rejection"),
        before
    );
    let mut relocation = json!({
        "file": "specifications/Other.md",
        "override_existing": true,
        "dry_run": true,
        "content": "### Error Response Specification\n\nError responses contain a code, message, and details.\n\n#### Metadata\n  * type: specification\n"
    });
    tools
        .call_tool("reqvire.add_element", &relocation)
        .expect("preview cross-file contract override");
    assert_eq!(
        fs::read_to_string("specifications/Model.md").expect("source after override dry-run"),
        before
    );
    assert!(!workspace.path().join("specifications/Other.md").exists());
    let read_reference = || {
        tools
            .call_tool("reqvire.read_element", &json!({"name": "Documentation"}))
            .expect("read consumer after override")["contract_references"][0]["target"]
            ["ElementIdentifier"]
            .clone()
    };
    assert_eq!(
        read_reference(),
        "specifications/Model.md#error-response-specification"
    );
    relocation["dry_run"] = json!(false);
    tools
        .call_tool("reqvire.add_element", &relocation)
        .expect("apply cross-file contract override");
    assert_eq!(
        read_reference(),
        "specifications/Other.md#error-response-specification"
    );
}

#[test]
fn runtime_shapes_encode_reference_types_ownership_and_section_exclusivity() {
    use o_kernel::shacl::{AstPath, ShaclRegistry, Shape, SyntaxConstraint, TargetIdentifier};
    use oxigraph::{
        io::{RdfFormat, RdfParser},
        model::NamedNode,
    };
    let quads = RdfParser::from_format(RdfFormat::Turtle)
        .for_reader(reqvire::runtime_ontology::REQVIRE_SHACL_TTL.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .expect("parse embedded SHACL");
    let ontology_quads = RdfParser::from_format(RdfFormat::Turtle)
        .for_reader(reqvire::runtime_ontology::REQVIRE_ONTOLOGY_TTL.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .expect("parse embedded vocabulary");
    for (predicate, target) in [
        ("relationFamilyForwardProperty", "referencesContract"),
        ("relationFamilyInverseProperty", "contractReferencedBy"),
    ] {
        assert!(ontology_quads.iter().any(|quad| quad.subject.to_string()
            == "<https://www.reqvire.org/ontology#contractReferenceRelationFamily>"
            && quad.predicate.as_str() == format!("https://www.reqvire.org/ontology#{predicate}")
            && quad.object.to_string() == format!("<https://www.reqvire.org/ontology#{target}>")));
    }
    let registry = ShaclRegistry::parse(&quads);
    assert!(
        registry.diagnostics.is_empty(),
        "{:?}",
        registry.diagnostics
    );
    let node = |local: &str| {
        NamedNode::new(format!("https://www.reqvire.org/ontology#{local}"))
            .expect("construct vocabulary IRI")
    };
    let Shape::Node(source) =
        &registry.compiled_shapes[&node("ContractReferenceSourceShape").into()]
    else {
        panic!("source shape")
    };
    assert!(source
        .targets
        .contains(&TargetIdentifier::SubjectsOf(node("referencesContract"))));
    assert!(source.constraints.contains(&SyntaxConstraint::Class {
        class_node: node("Requirement").into()
    }));
    let properties: Vec<_> = source
        .property_shapes
        .iter()
        .map(|id| match &registry.compiled_shapes[id] {
            Shape::Property(property) => property,
            _ => panic!("property shape"),
        })
        .collect();
    assert!(properties.iter().any(|property| property.path
        == Some(AstPath::Iri(node("bindsContract")))
        && property
            .constraints
            .contains(&SyntaxConstraint::MaxCount(0))));
    let Shape::Node(target) = &registry.compiled_shapes[&node("ReferencedContractShape").into()]
    else {
        panic!("target shape")
    };
    assert!(target
        .constraints
        .iter()
        .any(|constraint| matches!(constraint, SyntaxConstraint::Or(types) if types.len() == 6)));
    assert!(target.property_shapes.iter().any(
        |id| matches!(&registry.compiled_shapes[id], Shape::Property(property)
        if property.path == Some(AstPath::Iri(node("define")))
        && property.constraints.contains(&SyntaxConstraint::MinCount(1))
        && property.constraints.contains(&SyntaxConstraint::MaxCount(1)))
    ));
}

#[test]
fn shipped_shacl_rejects_reference_cycles_and_accepts_acyclic_dependencies() {
    use o_kernel::shacl::{ShaclRegistry, Shape, SyntaxConstraint};
    use oxigraph::{
        io::{RdfFormat, RdfParser},
        model::NamedNode,
        sparql::{QueryResults, SparqlEvaluator},
        store::Store,
    };
    let quads = RdfParser::from_format(RdfFormat::Turtle)
        .for_reader(reqvire::runtime_ontology::REQVIRE_SHACL_TTL.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .expect("parse shipped SHACL");
    let registry = ShaclRegistry::parse(&quads);
    let id = NamedNode::new("https://www.reqvire.org/ontology#ContractReferenceSourceShape")
        .expect("source shape IRI");
    let Shape::Node(source) = &registry.compiled_shapes[&id.into()] else {
        panic!("reference source node shape");
    };
    let query = source
        .constraints
        .iter()
        .find_map(|constraint| match constraint {
            SyntaxConstraint::Sparql { query } => Some(query),
            _ => None,
        })
        .expect("shipped reference cycle constraint");
    for (name, dependencies, violates) in [
        ("independent", ":a r:referencesContract :cb .", false),
        ("ancestor", ":a r:referencesContract :cb ; r:derivedFrom :b .", false),
        ("converging", ":a r:referencesContract :cb, :cc . :b r:referencesContract :cc .", false),
        ("self", ":a r:referencesContract :ca .", true),
        ("reciprocal", ":a r:referencesContract :cb . :b r:referencesContract :ca .", true),
        ("three requirements", ":a r:referencesContract :cb . :b r:referencesContract :cc . :c r:referencesContract :ca .", true),
        ("mixed binding", ":a r:referencesContract :cb . :b r:bindsContract :ca .", true),
        ("mixed ancestry", ":a r:referencesContract :cb . :b r:derivedFrom :a .", true),
    ] {
        let store = Store::new().expect("SHACL data store");
        let data = format!("@prefix : <urn:test:> . @prefix r: <https://www.reqvire.org/ontology#> . :ca r:define :a . :cb r:define :b . :cc r:define :c . {dependencies}");
        store.load_from_reader(RdfParser::from_format(RdfFormat::Turtle), data.as_bytes()).expect("load dependency graph");
        let results = SparqlEvaluator::new().parse_query(query).expect("parse shipped query").on_store(&store).execute().expect("evaluate shipped constraint");
        let QueryResults::Solutions(solutions) = results else { panic!("expected SELECT results") };
        let violations = solutions.collect::<Result<Vec<_>, _>>().expect("constraint results");
        assert_eq!(!violations.is_empty(), violates, "{name}");
    }
}
