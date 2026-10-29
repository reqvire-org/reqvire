//! Test-only typed operation driver for core workspace fixtures. This is not a
//! supported Reqvire CLI authoring interface or an MCP permission substitute.
use reqvire::tool_interface::ReqvireToolRegistry;
use serde_json::{json, Value};
use std::io::Read;

fn main() {
    reqvire::utils::enable_quiet_mode();
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("fixture request");
    let request: Value = serde_json::from_str(&input).expect("typed fixture request");
    let exclusions = reqvire::config::get_excluded_filename_patterns_glob_set();
    let service = ReqvireToolRegistry::new(true, &exclusions);
    let name = request["name"].as_str().expect("tool name");
    service
        .validate_tool_arguments(name, &request["arguments"])
        .expect("typed fixture arguments");
    match service.call_tool(name, &request["arguments"]) {
        Ok(value) => println!("{value}"),
        Err(error) => {
            println!("{}", json!({"error":{"message":error.to_string()}}));
            std::process::exit(1);
        }
    }
}
