use globset::{Glob, GlobSetBuilder};
use reqvire::tool_interface::ReqvireToolRegistry;
use serde_json::json;
use std::env;

fn main() {
    let workspace = env::args().nth(1).expect("workspace argument is required");
    env::set_current_dir(workspace).expect("failed to enter workspace");

    let mut builder = GlobSetBuilder::new();
    for pattern in ["output/**", "fixtures/**", "expected/**"] {
        builder.add(Glob::new(pattern).expect("valid ignore glob"));
    }
    let ignored = builder.build().expect("ignore glob set builds");
    let registry = ReqvireToolRegistry::new(false, &ignored);

    let tools = registry.tool_definitions();
    assert!(tools.iter().any(|tool| tool["name"] == "reqvire.search"));
    assert!(!tools.iter().any(|tool| tool["name"] == "reqvire.add_element"));

    let search = registry
        .call_tool(
            "reqvire.search",
            &json!({
                "filter_name": "Test Requirement Beta"
            }),
        )
        .expect("search tool should execute through library registry");
    assert_eq!(search["global_counters"]["total_elements"], 1);

    let contract = registry
        .call_tool("reqvire.tool_contract", &json!({}))
        .expect("tool contract should execute through library registry");
    assert_eq!(contract["mutation_tools_enabled"], false);
    assert!(contract["tools"]
        .as_array()
        .expect("contract tools array")
        .iter()
        .any(|tool| tool["name"] == "reqvire.search"));

    println!("library-search-total={}", search["global_counters"]["total_elements"]);
    println!("library-contract-mutations={}", contract["mutation_tools_enabled"]);
}
