use reqvire::exclusions::ExclusionSetBuilder;
use reqvire::tool_interface::ReqvireToolRegistry;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

// An integration-test executable has its own process, so workspace CWD and the
// persistent model cache cannot interfere with the library's parallel unit tests.
#[test]
fn coverage_tool_shares_rollup_and_refreshes_external_evidence_changes() {
    let workspace = tempfile::tempdir().expect("temporary model workspace");
    let original = std::env::current_dir().expect("working directory");
    struct RestoreDirectory(PathBuf);
    impl Drop for RestoreDirectory {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.0).expect("restore working directory");
        }
    }
    let _restore = RestoreDirectory(original);
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/test-implementation-rollup");
    for file in [
        "specifications/Model.md",
        "evidence/a.txt",
        "evidence/b.txt",
        "evidence/parent.txt",
    ] {
        let target = workspace.path().join(file);
        fs::create_dir_all(target.parent().expect("fixture parent")).expect("fixture directory");
        fs::copy(fixture.join(file), target).expect("copy fixture");
    }
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(workspace.path())
        .status()
        .expect("initialize worktree")
        .success());
    std::env::set_current_dir(workspace.path()).expect("select fixture workspace");
    let exclusions = ExclusionSetBuilder::new()
        .build()
        .expect("empty exclusions");
    let tools = ReqvireToolRegistry::new(false, &exclusions);
    let expected: Value = serde_json::from_str(
        &fs::read_to_string(fixture.join("expected/requirements.json"))
            .expect("expected assessments"),
    )
    .expect("expected JSON");
    let whole = tools
        .call_tool("reqvire.coverage", &json!({}))
        .expect("whole coverage tool");
    let mut actual = serde_json::Map::new();
    for section in ["covered_requirements", "uncovered_requirements"] {
        for rows in whole[section]["files"]
            .as_object()
            .expect("file groups")
            .values()
        {
            for row in rows.as_array().expect("requirement rows") {
                actual.insert(row["name"].as_str().expect("name").to_string(), row.clone());
            }
        }
    }
    assert_eq!(Value::Object(actual), expected);
    let scope = tools
        .call_tool("reqvire.coverage", &json!({"from": "Delegated Service"}))
        .expect("scoped coverage tool");
    assert_eq!(scope["summary"]["total_terminal_requirements"], 0);
    assert_eq!(
        scope["covered_requirements"]["files"]["specifications/Model.md"][0],
        expected["Complete Owner"]
    );

    // Mutate outside MCP after both reads have warmed the shared model cache.
    let model = fs::read_to_string("specifications/Model.md").expect("model source");
    let (before, consumer) = model
        .split_once("### Complete Consumer B\n")
        .expect("consumer heading");
    let consumer = consumer.replacen("  * satisfiedBy: [b](../evidence/b.txt)\n", "", 1);
    fs::write(
        "specifications/Model.md",
        format!("{before}### Complete Consumer B\n{consumer}"),
    )
    .expect("external mutation");
    let changed = tools
        .call_tool("reqvire.coverage", &json!({"from": "Delegated Service"}))
        .expect("refreshed scoped tool");
    let owner = &changed["uncovered_requirements"]["files"]["specifications/Model.md"][0];
    assert_eq!(owner["name"], "Complete Owner");
    assert_eq!(owner["evidence"], json!(["evidence/a.txt"]));
    assert_eq!(
        owner["blocking_requirements"],
        json!([
            "specifications/Model.md#complete-consumer",
            "specifications/Model.md#complete-consumer-b"
        ])
    );
    assert_eq!(owner["aggregate_terminal_requirements"], 2);
    assert_eq!(owner["aggregate_covered_terminal_requirements"], 1);
    assert_eq!(owner["aggregate_verified_leaf_requirements"], 1);
    assert_eq!(changed["summary"]["total_terminal_requirements"], 0);
}
