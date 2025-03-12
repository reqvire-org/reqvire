# ReqFlow Development Guide

## System Overview
ReqFlow is an AI-driven framework for system modeling and requirements management that integrates with Git workflows. It processes Markdown documents containing system requirements, manages relationships between elements, validates model consistency, and generates documentation and visualizations.

## Core Components
- **ModelManager**: Central coordinator for processing requirements
- **ElementRegistry**: Tracks and retrieves elements by identifier
- **Element**: Represents MBSE model elements (requirements, verifications)
- **Relation**: Represents connections between elements (derives, satisfies, verifies, etc.)
- **Linting**: Modular system for validating and fixing markdown formatting issues

## Command Line Usage
- Initialize ReqFlow: `reqflow init`
- Validate requirements: `reqflow validate specifications/`
- Generate HTML: `reqflow specifications/ output/ --html`
- Lint requirements: `reqflow --lint specifications/`
- Generate diagrams: `reqflow --lint specifications/ --generate-diagrams`

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

## Key File Locations
- Configuration example: `reqflow.yml.example`
- Design specifications: `specifications/DesignSpecifications/`
- Test fixtures: `test-fixtures/`
- E2E tests: `tests/e2e-*/`

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
  - specifically read and understand Design Specifications in specifications/DesignSpecifications/

## Element Structure
- Elements are uniquely identifiable sections starting with `###` headers
- Each element contains structured subsections:
  - Relations: Bullet points with format `* relationType: identifier`
  - Details: Additional information about the element
  - Metadata: Defines element types (requirement, verification)
- Supported relation types include: containedBy, contain, derivedFrom, derive, refine, satisfiedBy, satisfy, verifiedBy, verify, tracedFrom, trace
