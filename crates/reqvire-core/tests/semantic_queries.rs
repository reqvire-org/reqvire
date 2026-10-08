use reqvire::exclusions::ExclusionSetBuilder;
use reqvire::tool_interface::ReqvireToolRegistry;
use serde_json::json;
use std::{fs, path::PathBuf, process::Command};

#[test]
fn managed_queries_mcp_discovery_validation_projection_and_cache() {
    let workspace = tempfile::tempdir().expect("test fixture operation should succeed");
    struct Restore(PathBuf);
    impl Drop for Restore {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.0).expect("test fixture operation should succeed");
        }
    }
    let _restore = Restore(std::env::current_dir().expect("test fixture operation should succeed"));
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/test-semantic-queries/specifications/Queries.md");
    fs::create_dir(workspace.path().join("specifications"))
        .expect("test fixture operation should succeed");
    fs::copy(fixture, workspace.path().join("specifications/Queries.md"))
        .expect("test fixture operation should succeed");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(workspace.path())
        .status()
        .expect("test fixture operation should succeed")
        .success());
    std::env::set_current_dir(workspace.path()).expect("test fixture operation should succeed");
    let exclusions = ExclusionSetBuilder::new()
        .build()
        .expect("test fixture operation should succeed");
    let mut model = reqvire::model::ModelManager::new();
    model
        .parse_and_validate(None, &exclusions)
        .expect("test fixture operation should succeed");
    let graph = reqvire::ontology_graph::build_graph_data(
        model
            .semantic_store
            .as_ref()
            .expect("test fixture operation should succeed")
            .index(),
    );
    let native_query = graph
        .nodes
        .iter()
        .find(|node| node.id == "urn:reqvire:semantic-query:item-lookup")
        .expect("native query appears in ontology graph");
    assert_eq!(native_query.semantic_type, "semantic-query");
    assert_eq!(native_query.source_kind, "query");
    assert_eq!(native_query.label, "Item Lookup");
    assert_eq!(
        native_query.sources[0].source,
        "specifications/Queries.md#item-lookup"
    );
    let query_node =
        serde_json::to_value(native_query).expect("test fixture operation should succeed");
    assert_eq!(query_node["query"]["form"], "SELECT");
    assert!(query_node["query"]["text"]
        .as_str()
        .expect("expected a string in the test response")
        .contains("SELECT ?item"));
    assert!(graph.edges.iter().any(|edge| edge.source == native_query.id
        && edge.target == "https://example.org/items#name"
        && edge.label == "uses vocabulary"));
    let tools = ReqvireToolRegistry::new(true, &exclusions);
    let definitions = tools.tool_definitions();
    let export_tool = definitions
        .iter()
        .find(|tool| tool["name"] == "reqvire.semantic.export")
        .expect("test fixture operation should succeed");
    assert_eq!(
        export_tool["inputSchema"]["properties"]["layers"]["items"]["enum"],
        json!([
            "ontologies",
            "shapes",
            "concepts",
            "model",
            "external-used",
            "prefixes",
            "queries"
        ])
    );
    let result = tools
        .call_tool("reqvire.semantic.queries", &json!({"include_content":true}))
        .expect("test fixture operation should succeed");
    assert_eq!(result["queries"][0]["query_form"], "SELECT");
    let q = &result["queries"][0];
    assert_eq!(q["iri"], "urn:reqvire:semantic-query:item-lookup");
    assert_eq!(
        q["sha256"],
        reqvire::hashing::sha256_hex(
            q["content"]
                .as_str()
                .expect("expected a string in the test response")
                .as_bytes()
        )
    );
    for namespace in ["https://example.org/items", "https://example.org/items#"] {
        assert_eq!(
            tools
                .call_tool(
                    "reqvire.semantic.queries",
                    &json!({"namespace_base":namespace,"include_content":true})
                )
                .expect("test fixture operation should succeed"),
            result
        );
    }
    assert!(tools
        .call_tool("reqvire.semantic.queries", &json!({"name":"missing"}))
        .is_err());
    assert!(tools
        .call_tool(
            "reqvire.semantic.queries",
            &json!({"name":"Item Lookup","iri":q["iri"]})
        )
        .is_err());
    let export = tools
        .call_tool(
            "reqvire.semantic.export",
            &json!({"layers":["queries"],"namespace_base":"https://example.org/items"}),
        )
        .expect("test fixture operation should succeed");
    assert!(export
        .to_string()
        .contains("urn:reqvire:semantic-query:item-lookup"));
    let graph=tools.call_tool("reqvire.semantic.sparql", &json!({"query":"SELECT ?query WHERE { ?query a <https://www.reqvire.org/ontology#SemanticQuery> }", "full":true})).expect("test fixture operation should succeed");
    assert_eq!(graph["row_count"], 1);
    let validation = tools
        .call_tool("reqvire.semantic.queries.validate", &json!({}))
        .expect("test fixture operation should succeed");
    assert_eq!(validation["valid"], true);
    let original_revision = tools
        .call_tool("reqvire.model_revision", &json!({}))
        .expect("test fixture operation should succeed")["model_fingerprint"]
        .clone();
    let before = fs::read_to_string("specifications/Queries.md").expect("read test fixture");
    fs::write(
        "specifications/Queries.md",
        before.replace("Lists items for downstream consumers.", "New purpose."),
    )
    .expect("write test fixture");
    let changed = tools
        .call_tool("reqvire.semantic.queries", &json!({"include_content":true}))
        .expect("test fixture operation should succeed");
    assert_eq!(changed["queries"][0]["purpose"], "New purpose.");
    assert_eq!(changed["queries"][0]["sha256"], q["sha256"]);
    assert_ne!(
        tools
            .call_tool("reqvire.model_revision", &json!({}))
            .expect("test fixture operation should succeed")["model_fingerprint"],
        original_revision
    );
    let preserved = fs::read_to_string("specifications/Queries.md").expect("read test fixture");
    for (name, override_existing) in [("Item Lookup", true), ("Invalid New Query", false)] {
        let invalid = format!("### {name}\n\n#### Metadata\n  * type: semantic-query\n\n#### Query\n```sparql\nASK {{ ?s a <https://example.org/items#Unknown> }}\n```\n\n#### Relations\n  * use: [Item Ontology](#item-ontology)\n");
        assert!(tools.call_tool("reqvire.add_element", &json!({"file":"specifications/Queries.md", "override_existing":override_existing,"content":invalid})).is_err());
        assert_eq!(
            fs::read_to_string("specifications/Queries.md").expect("read test fixture"),
            preserved
        );
    }
    fs::write(
        "specifications/Queries.md",
        before.replace("?item a item:Item", "?item a item:Unknown"),
    )
    .expect("write test fixture");
    let invalid = tools
        .call_tool("reqvire.semantic.queries.validate", &json!({}))
        .expect("test fixture operation should succeed");
    assert_eq!(invalid["valid"], false);
    assert!(!invalid["queries"][0]["diagnostics"]
        .as_array()
        .expect("expected an array in the test response")
        .is_empty());
    assert!(tools
        .call_tool("reqvire.semantic.queries", &json!({"include_content":true}))
        .is_err());
}
