# Reqvire Development Guide

## System Overview
Reqvire is an AI-driven framework for system modeling and requirements management that integrates with Git workflows. It processes semi-structured Markdown documents containing system requirements, manages relationships between elements, validates model consistency, and generates documentation and visualizations.

## MBSE Development Workflow

**CRITICAL**: This is a Model-Based Systems Engineering (MBSE) project. Always follow this workflow:

1. **Requirements First**: Before implementing any new capability, add requirements to the specifications with proper traceability
2. **Verifications**: Define how the capability will be verified before implementation
3. **Tests**: Create tests to satisfy verifications (can be refined during implementation if needed)
4. **Implementation**: Only after requirements and verifications are defined, implement the code
5. **Satisfaction Links**: Link implementation to `requirement` elements (and test-verifications) via satisfiedBy relations

**Never skip the requirements step.** Implementation without requirements violates the MBSE methodology and project principles.

## Domain-Specific Guides

This guide is split into domain-specific guides for better organization:

- **[tests/CLAUDE.md](tests/CLAUDE.md)** - Guide for writing and executing end-to-end tests
- **Core Development** (see sections below) - Guide for Rust code development, architecture, and components

## Building and Running Reqvire

### Basic cargo Commands
- Build: `cargo build`
- Run with HTML output: `cargo run -- specifications output --html`
- Run specific commands: `cargo run -- format` or `./target/debug/reqvire format`
- Test: `cargo test`
- Test specific: `cargo test <test_name>`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`

### Development Commands

**For working with specifications, use the Reqvire skills:**
- `reqvire:syseng` - System and requirements engineering
- `reqvire:task-master` - Task planning from requirement changes

#### Essential Development Commands
- **Validate**: `./target/debug/reqvire validate [--json]`
- **Format**: `./target/debug/reqvire format [--fix]`
- **Search**: `./target/debug/reqvire search [--json] [--short] [--filter-*]`
- **Export**: `./target/debug/reqvire export --output <DIR>`
- **Serve**: `./target/debug/reqvire serve --port 8080`

For comprehensive command reference, see the Reqvire skills.

### Key File Locations
- Core specifications structure: [SpecificationsRequirements.md](requirements/SpecificationsRequirements.md)
- Capability roots: [Capabilities.md](requirements/Capabilities.md)
- Mission requirements: [MissionRequirements.md](requirements/MissionRequirements.md)
- System requirements: [SystemRequirements/Requirements.md](requirements/SystemRequirements/Requirements.md)
- Verification specifications: [Verifications folder](requirements/Verifications)
- Relationship types: [Relation Types and Behaviors](requirements/SpecificationsRequirements.md#relation-types-and-behaviors)
- Architecture documentation: [Architecture.md](requirements/Architecture.md)


## Core Architecture

### Core Components
- **ModelManager**: Central coordinator for processing requirements
- **ElementRegistry**: Tracks and retrieves elements by identifier
- **Element**: Represents MBSE model elements (requirements, verifications)
- **Relation**: Represents connections between elements
- **Parser**: Processes Markdown to extract elements and relations
- **Formating**: Validates and fixes markdown formatting issues
- **ChangeImpact**: Analyzes how changes propagate through the model
- **MatrixGenerator**: Creates traceability matrices
- **DiagramGenerator**: Creates Mermaid diagrams from relationships

### Project Structure
```
core/src/
├── lib.rs                 # Library entry point
├── element.rs             # Element representation and logic
├── relation.rs            # Relation types and behaviors
├── parser.rs              # Markdown parsing logic
├── model.rs               # Model management and coordination
├── graph_registry.rs      # Element registry and graph operations
├── diagrams.rs            # Diagram generation (Mermaid)
├── matrix_generator.rs    # Traceability matrix generation
├── change_impact.rs       # Change impact analysis
├── filters.rs             # Element filtering logic
├── search.rs              # Search and query functionality
├── report_coverage.rs     # Coverage report generation
├── report_model.rs        # Model-centric report generation
├── report_trace.rs        # Verification trace report generation
├── crud.rs                # Element manipulation (add, remove, move)
├── utils.rs               # Utility functions
├── error.rs               # Error handling
└── tests/                 # Unit tests

cli/src/
├── main.rs                # CLI entry point
├── cli.rs                 # Command-line interface logic
└── config.rs              # Configuration management
```

## Development Guidelines

### Code Style
- Follow Rust standard naming conventions
- snake_case for functions, variables, modules
- CamelCase for types, traits, enums
- SCREAMING_SNAKE_CASE for constants
- Rust 2021 edition features
- Use Result with custom error types for error handling
- Document all public APIs with rustdoc
- Organize code in modules by functionality
- Prefer strong typing over string manipulation

### Architecture Principles
- Follow the requirements and architecture defined in requirements/
- Link code and implementations via satisfiedBy only on `requirement` (system) and `test-verification` elements
- Use Result with custom error types for error handling
- Document all public APIs with rustdoc
- Organize code in modules by functionality

## Model Contract

The Reqvire system model has four first-class layers: ontology, capability, requirement, and verification. Refinements are requirement-owned subordinate details/contracts only, not a system-model layer.

Capabilities may derive child capabilities, attach ontology, be specified by requirements, and be verified. Capabilities must not own `source`, `constraint`, `behavior`, `specification`, `state`, `input-output`, `semantic-contract`, or `semantic-query-contract` refinements through `refinedBy`/`refine`.

Requirement-owned refinement types are `source`, `constraint`, `behavior`, `specification`, `state`, `input-output`, `semantic-contract`, and `semantic-query-contract`.

## Important Notes

- Use the `/tmp` directory to store JSON outputs for further analysis
- ALWAYS prefer editing existing files in the codebase over creating new ones
- NEVER proactively create documentation files (*.md) or README files unless explicitly requested
- Only use emojis if the user explicitly requests it
- Update generated docs/www only when the task explicitly asks for generated documentation updates.
