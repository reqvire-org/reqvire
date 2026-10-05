use reqvire::exclusions::ExclusionSetBuilder;
use reqvire::tool_interface::ReqvireToolRegistry;
use serde_json::json;
use std::{fs, path::PathBuf, process::Command};

#[test]
fn rejected_taxonomy_mutations_preserve_mcp_accepted_state() {
    let workspace = tempfile::tempdir().unwrap();
    struct RestoreDirectory(PathBuf);
    impl Drop for RestoreDirectory {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.0).unwrap();
        }
    }
    let _restore = RestoreDirectory(std::env::current_dir().unwrap());
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(workspace.path())
        .status()
        .unwrap()
        .success());
    fs::write(
        workspace.path().join("Concepts.md"),
        include_str!("../../../tests/test-concept-elements/fixtures/Projection.md.fixture"),
    )
    .unwrap();
    std::env::set_current_dir(workspace.path()).unwrap();
    let exclusions = ExclusionSetBuilder::new().build().unwrap();
    let tools = ReqvireToolRegistry::new(true, &exclusions);
    let before = fs::read("Concepts.md").unwrap();
    let revision = tools
        .call_tool("reqvire.model_revision", &json!({}))
        .unwrap();
    let element = tools
        .call_tool(
            "reqvire.read_element",
            &json!({"name": "Projection Parent"}),
        )
        .unwrap();
    for dry_run in [false, true] {
        for (tool, args) in [
            (
                "reqvire.link",
                json!({"source": "Projection Parent", "relation_type": "broader", "target": "Projection Child", "dry_run": dry_run}),
            ),
            (
                "reqvire.add_element",
                json!({"file": "Concepts.md", "override_existing": true, "dry_run": dry_run,
                "content": "### Projection Parent\n#### Metadata\n  * type: concept\n#### Relations\n  * derivedFrom: [Projection Alpha Scheme](#projection-alpha-scheme)\n  * broader: [Projection Child](#projection-child)\n"}),
            ),
        ] {
            let error = tools
                .call_tool(tool, &args)
                .expect_err("taxonomy cycle must reject candidate");
            assert!(
                format!("{error:?}").contains("Concept taxonomy cycle"),
                "{error:?}"
            );
            assert_eq!(fs::read("Concepts.md").unwrap(), before);
            assert_eq!(
                tools
                    .call_tool("reqvire.model_revision", &json!({}))
                    .unwrap(),
                revision
            );
            assert_eq!(
                tools
                    .call_tool(
                        "reqvire.read_element",
                        &json!({"name": "Projection Parent"})
                    )
                    .unwrap(),
                element
            );
        }
    }
    tools.call_tool("reqvire.link", &json!({"source": "Projection Parent", "relation_type": "related", "target": "Projection Isolated"})).expect("valid association after rejection");
    assert_ne!(
        tools
            .call_tool("reqvire.model_revision", &json!({}))
            .unwrap(),
        revision
    );
}
