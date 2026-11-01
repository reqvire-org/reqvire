# Reqvire Development Guide

## System Overview
Reqvire is an AI-driven framework for system modeling and requirements management that integrates with Git workflows. It processes semi-structured Markdown documents containing system requirements, manages relationships between elements, validates model consistency, and generates documentation and visualizations.

## Workspace Management Decisions

Persistent project decisions from the workspace-management discussion on 2026-10-31:

- **The default manifest location is already implemented and specified:** Reqvire automatically selects `.repo/manifest.xml`. See `selected_manifest_at` in `crates/reqvire-core/src/repo_workspace.rs` and `system-model/ModelStructure/RepoWorkspaces.md`. Do not reopen this as an undecided design question.
- Do not require a duplicate workspace-root manifest, a root-tracked XML copy, or a routine `--manifest` override. The override remains an optional alternate selection.
- `.repo/manifest.xml` is Repo's entry point; resolve its actual source files and Git owners. A native manifest checkout can share Git history with the workspace-root repository or have its own history. Its filesystem location alone does not establish either case.
- Manifest sources must be versioned in their actual owning Git repositories. Ignoring `/.repo/` in the outer repository does not mean the manifest is unversioned in its owner. Do not add Repo administration to the outer Git tree.
- The user's example uses the server/client monorepo as the workspace-root repository, with a shared model and separate declared website/documentation repositories. Do not silently substitute a different workspace layout or require a new manifest-only repository.
- Native discovery/model loading already work. Workspace management includes historical reconstruction, owner-aware manifest checkpoints, and coordinated branch/publication management. Check current verification evidence before claiming completion or release; the default manifest location was never a missing feature.
- Public MCP management uses branches; worktrees are internal. Single- and multi-repository use share the same operations, without a repository-mode argument. Explicit branch creation should prepare the requested branch in the root and all declared components from the selected base composition; branch creation alone does not advance pins.
- The user explicitly approved one coordinated MCP PR-create call producing linked PRs for changed components and the root. Push and PR creation remain explicit operations.
- Review should use the selected PR's actual baseline, with one explicit Git-baseline override. Do not default examples to `origin/main` or introduce a second manifest-commit flag.
- Additional `branch.open` reload flags, a separate adopt tool, and refresh modes were discussed but **not approved**. Do not present them as settled decisions.
- Explicit `branch.open` now performs full validated admission of a prepared changed composition; unchanged contexts remain idempotent. Guarded `branch.refresh` remains artifact-only. Rejected admission preserves the live accepted view, including reads already in flight.
- Keep the Git executable backend for this iteration. Core local/object reads share `git_commands` and its batched `GitReader`; MCP retains bounded execution for writes, cancellation and uncertain outcomes. A Rust Git-library migration requires a separate measured reason and compatibility work.

## MBSE Development Workflow

**CRITICAL**: This is a Model-Based Systems Engineering (MBSE) project. Always follow this workflow:

1. **Requirements First**: Before implementing any new capability, add requirements to the specifications with proper traceability
2. **Verifications**: Define how the capability will be verified before implementation
3. **Tests**: Create tests to satisfy verifications (can be refined during implementation if needed)
4. **Implementation**: Only after requirements and verifications are defined, implement the code
5. **Satisfaction Links**: Link implementation to `requirement` elements (and test-verifications) via satisfiedBy relations

**Never skip the requirements step.** Implementation without requirements violates the MBSE methodology and project principles.

Apply this workflow to engineering obligations. Cosmetic adjustments reuse existing UI requirements; do not add verification elements, acceptance criteria, or tests for individual width, spacing, alignment, or color tweaks. Keep verifications focused on meaningful behavior and accessibility. Use the existing design-system guards and showcase review for cosmetic changes, and report when visual review could not be performed.

## Domain-Specific Guides

This guide is split into domain-specific guides for better organization:

- **[tests/CLAUDE.md](tests/CLAUDE.md)** - Guide for writing and executing end-to-end tests
- **Explorer UI** - Use the DS workflow skills for visual changes. The [local design-system guide](explorer/design-system/readme.md) records Reqvire conventions and source locations; component and pattern guides document their APIs and behavior.
- **Core Development** (see sections below) - Guide for Rust code development, architecture, and components

## Building and Running Reqvire

The optional Hermit environment pins the development tools. From the repository
root, run `source bin/activate-hermit` then `hermit install`; see
[Contributor setup](doc/README.md#development-quick-start) for full prerequisites.
Commit only bootstrap/version selectors and package definitions. Downloaded
packages stay in Hermit's user cache; `.hermit/`, build outputs and dependencies
are Git-ignored.

### Basic cargo Commands
- Build: `cargo build`
- Run inspection commands: `cargo run -- validate` or `./target/debug/reqvire lint`
- Test: `cargo test`
- Test specific: `cargo test <test_name>`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`

### Development Commands

The E2E runner defaults to `target/release/reqvire` and copies that executable
at startup. Use `make e2e` to rebuild release before testing, or explicitly set
`REQVIRE_BIN="$PWD/target/debug/reqvire"` after a debug build. Verify the printed
source executable; a rebuild does not update a run already in progress.

**For working with specifications, use the Reqvire skills:**
- `reqvire-syseng` - System and requirements engineering
- `reqvire-audit` - Model quality, coverage, and change-impact review

Shared packages live in `skills/`; task planning is part of `reqvire-syseng`.

#### Essential Development Commands
- **Validate**: `./target/debug/reqvire validate [--json]`
- **Authoring and formatting**: typed MCP tools; start `./target/debug/reqvire mcp --enable-mutations`. `reqvire.format` previews by default; `fix: true` applies formatting. Successful authored-model mutations format affected documents automatically.
- **Search**: `./target/debug/reqvire search [--json] [--short] [--filter-*]`
- **Serve**: `./target/debug/reqvire serve --port 8080`

For comprehensive command reference, see the Reqvire skills.

### Key File Locations
- Capability-rooted model: root-level scope folders under [requirements](requirements)
- Shared ontology plane: [Ontologies folder](system-model/Ontologies)
- Refinements: capability-local files next to the requirements they refine
- Architecture documentation: [Architecture.md](system-model/Architecture.md)
- Relationship behavior: [Relation Types and Behaviors](system-model/ModelStructure/ModelManagement.md#relation-types-and-behaviors)
- Architecture documentation: [Architecture.md](system-model/Architecture.md)


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
crates/reqvire-core/src/
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

crates/reqvire-cli/src/
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
- Follow the requirements and architecture defined in system-model/
- Link code and implementations via satisfiedBy only on `requirement` elements and evidence-backed concrete verification elements (`test-verification`, `formal-proof-verification`)
- Use Result with custom error types for error handling
- Document all public APIs with rustdoc
- Organize code in modules by functionality

## Model Contract

The Reqvire system model has four first-class layers: ontology, capability, requirement, and verification-family. `verification-objective` elements are mandatory parents for concrete verification planning and hierarchy; concrete verification elements carry `verify`/`verifiedBy` and optional evidence. Refinements are requirement-owned subordinate details/contracts only, not a system-model layer.

Capabilities may derive child capabilities, author Concept References, be specified by requirements, and receive verification and implementation coverage through those requirements. Capabilities cannot be directly verified or satisfied and do not own contracts through `definedBy`/`define`.

Requirement-owned refinement types are `source`, `constraint`, `behavior`, `specification`, `state`, and `input-output`. Semantic contracts are first-class ontology-plane elements, not requirement-owned refinements.

Contract dependency is the umbrella term for Contract Bindings and Contract References. Use Contract Bindings on requirements responsible for shared implementation obligations. A Contract Reference declares a content dependency that propagates change impact without contributing to the contract owner's implementation fulfillment. Use references for documentation, report consumers, and adapters using existing services. Both kinds preserve contract context in collection. A requirement cannot author both sections, and contract dependencies must remain acyclic through contract owners and requirement ancestry.

## Important Notes

- Use the `/tmp` directory to store JSON outputs for further analysis
- ALWAYS prefer editing existing files in the codebase over creating new ones
- NEVER proactively create documentation files (*.md) or README files unless explicitly requested
- Only use emojis if the user explicitly requests it
- Update generated website artifacts only when the task explicitly asks for website output changes.
