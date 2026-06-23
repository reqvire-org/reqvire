# Exploring and Understanding the Model

Use this reference when you need to understand the model structure, search for requirements, or browse the specifications.

**For common commands** (search, validate, lint, link, etc.), see [SKILL.md Command Reference](../SKILL.md#command-reference). This reference focuses on advanced search patterns and model exploration workflows.

## Quick Model Overview

Start exploring your model with these commands to understand its structure:

```bash
# Get model summary with element counts
reqvire search --short --json | jq '.summary'

# List all files with elements
reqvire search --short --json | jq '.files[].file_path'

# Check model health
reqvire validate
reqvire coverage
```

### Model-Centric View

For a deeper understanding of how requirements connect, use the model-centric view:

```bash
# Show all capability-rooted model structures with nested relations
reqvire model [--json]

# Start from specific element to see its subtree
reqvire model --from "Element Name"

# Reverse traversal: start from leaf elements (verifications) and trace upward
reqvire model --reverse

# Filter by element type
reqvire model --filter-type="capability"
reqvire model --filter-type="requirement"
reqvire model --filter-type="test-verification"

# Combine reverse with type filter (e.g., trace verifications upward)
reqvire model --reverse --filter-type="test-verification"
```

**Element types for `--filter-type`:** capability, requirement, ontology, semantic-contract, verification-objective, test-verification, formal-proof-verification, analysis-verification, inspection-verification, demonstration-verification, source, state, input-output, constraint, behavior, specification. For custom types: `other-TYPENAME`

## Searching Elements

The search command filters elements based on various criteria:

```bash
reqvire search [--json] [--short] [--filter-*]
```

Use `--short` when analyzing model structure without needing full content. Use `--json` for programmatic processing.

### Filter Options

| Option | Description | Example |
|--------|-------------|---------|
| `--filter-file` | Filter by file glob | `--filter-file="system-model/**/*.md"` |
| `--filter-name` | Filter by element name (regex) | `--filter-name=".*Auth.*"` |
| `--filter-id` | Filter by exact identifier | `--filter-id="system-model/File.md#element"` |
| `--filter-type` | Filter by element type (comma-separated, OR logic) | `--filter-type="capability"` or `--filter-type="requirement,behavior"` |
| `--filter-content` | Filter by content (regex) | `--filter-content="SHALL.*validate"` |
| `--filter-page-content` | Filter by file frontmatter | `--filter-page-content="security"` |
| `--have-relations` | Elements with ALL relations | `--have-relations="verifiedBy,satisfiedBy"` |
| `--not-have-relations` | Elements without ALL relations | `--not-have-relations="verifiedBy"` |
| `--has-contract-bindings` | Elements with contract_bindings | `--has-contract-bindings` |
| `--filter-contract-bindings` | Filter by contract_bindings pattern | `--filter-contract-bindings="*.pdf"` |

**Element types for --filter-type (supports comma-separated list):** capability, requirement, ontology, semantic-contract, verification-objective, test-verification, formal-proof-verification, analysis-verification, inspection-verification, demonstration-verification, source, state, input-output, constraint, behavior, specification. For custom types: `other-TYPENAME`

### By Type
```bash
# Find all capabilities
reqvire search --filter-type="capability" --short

# Find all requirements
reqvire search --filter-type="requirement" --short

# Find all verifications
reqvire search --filter-type="verification" --short

# Find specifications/constraints/behaviors
reqvire search --filter-type="specification" --short
reqvire search --filter-type="constraint" --short
reqvire search --filter-type="semantic-contract" --short
```

### By File Location
```bash
# Elements in specific folder
reqvire search --filter-file="system-model/System/**" --short

# Elements in specific file
reqvire search --filter-file="system-model/**/*.md" --short
```

### By Name or Content
```bash
# Find by name pattern
reqvire search --filter-name=".*Authentication.*" --short

# Find by content pattern
reqvire search --filter-content="SHALL.*validate" --short
```

### By Relations
```bash
# Find unverified requirements
reqvire search --filter-type="requirement" --not-have-relations="verifiedBy" --short

# Find unsatisfied verifications (test type without implementation)
reqvire search --filter-type="test-verification" --not-have-relations="satisfiedBy" --short

# Find elements with specific relations
reqvire search --have-relations="verifiedBy,satisfiedBy" --short

# Find specifications not linked to any requirement
reqvire search --filter-type="specification" --not-have-relations="define" --short
```

### By Contract Bindings
```bash
# Find elements with contract_bindings
reqvire search --has-contract-bindings --short

# Find elements with specific contract_bindings type
reqvire search --filter-contract-bindings="*.pdf" --short
reqvire search --filter-type="specification" --short
```

## Understanding Traceability

To understand how verifications trace to requirements, use the traces command:

```bash
# Show verification traces (upward from verifications to owning capability roots)
reqvire traces [--json] [--filter-*]

# Filter by specific element patterns
reqvire traces --filter-name=".*Auth.*"
reqvire traces --filter-type="test-verification"

# Generate with GitHub blob links for stable references
reqvire traces --links-with-blobs

# Generate relative links from specific folder
reqvire traces --from-folder="docs/specs"

# See verification coverage report
reqvire coverage [--json]

# See all files referenced by model (implementations, design docs)
reqvire resources
```

## Understanding Element Details

When you find an element of interest:
1. Read the full element content (not just --short output)
2. Check for **contract_bindings** - they contain critical details
3. Follow relations to understand context:
   - `derivedFrom` → parent requirements (why this exists)
   - `satisfiedBy` → implementations (how it's fulfilled)
   - `verifiedBy` → verifications (how it's tested)

To gather complete context for a requirement, use the collect command:

```bash
# Get full requirement chain with all ancestor content and contract_bindings
reqvire collect "<requirement-name>" [--json]

# Example: collect all context for a capability
reqvire collect "Capability Requirement"

# JSON format for programmatic use
reqvire collect "System Requirement" --json
```

The collect command traverses `derivedFrom` relations upward and includes:
- All ancestor requirement content
- Reused markdown files (read as content)
- Reused contract elements (specifications, constraints, behaviors)
- Source citations for traceability

**When to use collect:**
- Before implementing a requirement - get full specification context
- When analyzing impact of changes - understand complete requirement chain
- When creating tasks from requirements - gather all related specifications
- When reviewing requirements - see full derivation hierarchy with sources

## Containment View

View the physical organization of the model (folders → files → elements):

```bash
# See full containment structure
reqvire containment

# Compact view without element details
reqvire containment --short

# JSON format for programmatic analysis
reqvire containment --json
```

## Common Analysis Patterns

### Find gaps in verification
```bash
# Unverified leaf requirements
reqvire search --filter-type="requirement" --not-have-relations="verifiedBy" --short

# Check coverage percentage
reqvire coverage --json | jq '.summary'
```

### Find orphaned elements
```bash
# Specifications not refining any requirement
reqvire search --filter-type="specification" --not-have-relations="define" --short --json

# Verifications not verifying any requirement
reqvire search --filter-type="verification" --not-have-relations="verify" --short
```

### Find unsatisfied test verifications
```bash
# Test verifications without implementation (satisfiedBy)
reqvire search --filter-type="test-verification" --not-have-relations="satisfiedBy" --short
```

### Understand a capability area
```bash
# Start from a capability or requirement and follow model relations
reqvire model --from "Capability Name"

# See all elements in capability's folder
reqvire search --filter-file="**/CapabilityName/**" --short
```

### Quick model health check
```bash
# Validate model and check coverage summary
reqvire validate && reqvire coverage --json | jq '.summary'
```
