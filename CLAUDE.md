# ReqFlow Development Guide

## 
## Build and Test Commands
- Build: `cargo build`
- Run: `cargo run -- <input_folder> <output_folder> [--html]`
- Test: `cargo test`
- Test specific: `cargo test <test_name>`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`

## Dependencies
- Markdown parsing: `pulldown-cmark`  
- HTML generation: `maud`
- Path handling: `camino`
- Command line: `clap` with derive feature
- Regex: `regex`

## Code Style Guidelines
- Follow Rust standard naming conventions
  - snake_case for functions, variables, modules
  - CamelCase for types, traits, enums
  - SCREAMING_SNAKE_CASE for constants
- Rust 2021 edition features
- Error handling: use Result with custom error types
- Document all public APIs with rustdoc
- Organize code in modules by functionality
- Prefer strong typing over string manipulation
- Follow the requirements, logical and physical architecture defined in specifications/
  - specifically cread and understand Design Specifications in specifications/DesignSpecifications/
