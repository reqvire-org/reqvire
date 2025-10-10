# Reqvire Development Guide

## System Overview
Reqvire is an AI-driven framework for system modeling and requirements management that integrates with Git workflows. It processes semi-structured Markdown documents containing system requirements, manages relationships between elements, validates model consistency, and generates documentation and visualizations.

## Domain-Specific Guides

This guide is split into domain-specific guides for better organization:

- **[specifications/CLAUDE.md](specifications/CLAUDE.md)** - Guide for writing and editing specifications, requirements, and verifications
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

### Basic Reqvire Commands
- **Validate structure**: `./target/debug/reqvire validate --json > /tmp/validation.json`
- **Format requirements (preview only)**: `./target/debug/reqvire format --dry-run`
- **Apply formatting fixes**: `./target/debug/reqvire format`
- **Generate diagrams**: `./target/debug/reqvire generate-diagrams`
- **Generate index document**: `./target/debug/reqvire model index > SpecificationIndex.md`
- **Generate model summary**: `./target/debug/reqvire model summary`
- **Generate model summary (JSON)**: `./target/debug/reqvire model summary --json > /tmp/model-summary.json`
- **Generate section summary**: `./target/debug/reqvire model section-summary`
- **Generate section summary (JSON)**: `./target/debug/reqvire model section-summary --json > /tmp/section-summary.json`
- **Generate HTML documentation**: `./target/debug/reqvire html --output <OUTPUT_DIR>`
- **Analyze change impact**: `./target/debug/reqvire change-impact --git-commit=<COMMIT_HASH>`
- **Analyze change impact (JSON)**: `./target/debug/reqvire change-impact --git-commit=HEAD~1 --json > /tmp/impact.json`
- **Generate verification traces**: `./target/debug/reqvire verifications traces` - Generates upward traceability from verifications to root requirements with Mermaid diagrams
- **Generate verification traces (JSON)**: `./target/debug/reqvire verifications traces --json > /tmp/verification-traces.json` - JSON output with trace trees for programmatic analysis
- **Filter verification traces**: `./target/debug/reqvire verifications traces --filter-id=<id>` or `--filter-name=<regex>` or `--filter-type=<type>` - Filter to specific verifications
- **Generate verification matrix**: `./target/debug/reqvire verifications matrix` - Generates verification traceability matrix
- **Generate coverage report**: `./target/debug/reqvire verifications coverage` - Generates verification coverage report for leaf requirements


### Filtering Options

| Filter Type | Command Example | Description |
|-------------|----------------|-------------|
| File path | `--filter-file="src/**/*Reqs.md"` | Filter elements by file glob pattern |
| Name regex | `--filter-name=".*safety.*"` | Filter elements by name using regex |
| Section | `--filter-section="System*"` | Filter elements by section glob pattern |
| Type | `--filter-type="system-requirement"` | Filter by exact element type |
| Content | `--filter-content="MUST"` | Filter elements containing specific text |
| Not verified | `--filter-is-not-verified` | Show only unverified requirements |
| Not satisfied | `--filter-is-not-satisfied` | Show only unsatisfied requirements |

### Key File Locations
- Core specifications structure: [SpecificationsRequirements.md](specifications/SpecificationsRequirements.md)
- User requirements: [UserRequirements.md](specifications/UserRequirements.md)
- Mission requirements: [MissionRequirements.md](specifications/MissionRequirements.md)
- System requirements: [SystemRequirements/Requirements.md](specifications/SystemRequirements/Requirements.md)
- Verification specifications: [Verifications folder](specifications/Verifications)
- Relationship types: [Relation Types and Behaviors](specifications/SpecificationsRequirements.md#relation-types-and-behaviors)
- Architecture documentation: [LogicalArchitecture.md](specifications/LogicalArchitecture.md) and [PhysicalArchitecture.md](specifications/PhysicalArchitecture.md)


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
├── reports.rs             # Report generation
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
- Follow the requirements and architecture defined in specifications/
- Link code and implementations to requirements via satisfiedBy on requirement level
- Use Result with custom error types for error handling
- Document all public APIs with rustdoc
- Organize code in modules by functionality

## Important Notes

- Use the `/tmp` directory to store JSON outputs for further analysis
- ALWAYS prefer editing existing files in the codebase over creating new ones
- NEVER proactively create documentation files (*.md) or README files unless explicitly requested
- Only use emojis if the user explicitly requests it
