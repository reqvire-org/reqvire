# Reqvire Development Guide

## System Overview
Reqvire is an AI-driven framework for system modeling and requirements management that integrates with Git workflows. It processes semi-structured Markdown documents containing system requirements, manages relationships between elements, validates model consistency, and generates documentation and visualizations.

## MBSE Development Workflow

**CRITICAL**: This is a Model-Based Systems Engineering (MBSE) project. Always follow this workflow:

1. **Requirements First**: Before implementing any new feature, add requirements to the specifications with proper traceability
2. **Verifications**: Define how the feature will be verified before implementation
3. **Tests**: Create tests to satisfy verifications (can be refined during implementation if needed)
4. **Implementation**: Only after requirements and verifications are defined, implement the code
5. **Satisfaction Links**: Link implementation to requirements via satisfiedBy relations

**Never skip the requirements step.** Implementation without requirements violates the MBSE methodology and project principles.

## Domain-Specific Guides

This guide is split into domain-specific guides for better organization:

- **[tests/CLAUDE.md](tests/CLAUDE.md)** - Guide for writing and executing end-to-end tests
- **Core Development** (see sections below) - Guide for Rust code development, architecture, and components

## Using Specialized Skills and Commands

**IMPORTANT**: Use specialized skills and commands for specific Reqvire tasks:

### System and Requirements Engineer Skill (`/syseng`)
Use the **`/syseng` skill** for managing specifications and requirements:
- **Manage Model**: Managing project specifications and model structure
- **Create Requirements**: Adding new requirements or verifications to the specifications
- **Update Specifications**: Updating or fixing existing requirements and verifications
- **Add Features**: Adding new feature requirements to the model with proper traceability
- **Analyze Relations**: Understanding relations between elements (verify, derivedFrom, satisfiedBy, etc.)
- **Find Issues**: Finding redundant or missing relations, coverage gaps, or structural issues
- **Review Structure**: Reviewing requirement structures and traceability
- **Analyze Coverage**: Analyzing verification coverage and traceability
- **Change Impact**: Understanding change impacts and propagation through the model
- **Model Inquiries**: Any inquiry related to model analysis, requirements, verifications, or relations
- **Report Analysis**: Generating or interpreting reports (summary, traces, coverage, matrix, etc.)

**The `/syseng` skill orchestrates focused commands:**
- `/analyze-model` - Analyze model structure and identify issues
- `/add-requirement` - Add new requirement with proper structure
- `/add-verification` - Add verification for existing requirement
- `/add-feature` - Add complete feature with requirements and verifications
- `/analyze-coverage` - Check verification coverage
- `/analyze-impact` - Analyze change impact from git commits
- `/lint-model` - Fix auto-fixable issues and identify items needing review

**IMPORTANT**: The `/syseng` skill works ONLY with specification files (Markdown) and model analysis - **it NEVER implements code** (Rust, tests, etc.). Always use `/syseng` or its commands for specification management, but use other approaches for code implementation.

### E2E Test Engineer Agent
Use the **e2e-test-engineer agent** when needing to:
- **Implement Tests**: Creating new e2e tests for unverified functionality
- **Update Tests**: Updating existing tests to maintain consistency with implementation
- **Review Tests**: Reviewing test coverage and quality
- **Verify Functionality**: Ensuring all Reqvire functionality has corresponding e2e tests
- **Satisfy Verifications**: Creating tests that properly satisfy verification requirements
- **Maintain Tests**: Fixing broken tests or updating tests after implementation changes
- **Analyze Test Coverage**: Analyzing test coverage gaps or test effectiveness

The e2e-test-engineer agent is specifically designed to work with Reqvire's e2e test suite and ensure all functionality is properly verified. Always delegate test implementation and maintenance tasks to this specialized agent.

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
- **Format requirements (preview)**: `./target/debug/reqvire format`
- **Apply formatting fixes**: `./target/debug/reqvire format --fix`
- **Generate diagrams**: `./target/debug/reqvire generate-diagrams`
- **Generate model summary**: `./target/debug/reqvire summary`
- **Generate model summary (JSON)**: `./target/debug/reqvire summary --json > /tmp/model-summary.json`
- **Generate section summary**: `./target/debug/reqvire section-summary`
- **Generate section summary (JSON)**: `./target/debug/reqvire section-summary --json > /tmp/section-summary.json`
- **Export HTML documentation**: `./target/debug/reqvire export --output <OUTPUT_DIR>` - Exports HTML with index, diagrams, traces, coverage, and matrix
- **Serve HTML documentation**: `./target/debug/reqvire serve --host localhost --port 8080` - Exports to temp dir and serves via HTTP server (quiet mode)
- **Analyze change impact**: `./target/debug/reqvire change-impact --git-commit=<COMMIT_HASH>`
- **Analyze change impact (JSON)**: `./target/debug/reqvire change-impact --git-commit=HEAD~1 --json > /tmp/impact.json`
- **Generate verification traces**: `./target/debug/reqvire traces` - Generates upward traceability from verifications to root requirements with Mermaid diagrams
- **Generate verification traces (JSON)**: `./target/debug/reqvire traces --json > /tmp/verification-traces.json` - JSON output with trace trees for programmatic analysis
- **Filter verification traces**: `./target/debug/reqvire traces --filter-id=<id>` or `--filter-name=<regex>` or `--filter-type=<type>` - Filter to specific verifications
- **Generate verification matrix**: `./target/debug/reqvire matrix` - Generates verification traceability matrix
- **Generate coverage report**: `./target/debug/reqvire coverage` - Generates verification coverage report for leaf requirements


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
- Architecture documentation: [Architecture.md](specifications/Architecture.md)


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
