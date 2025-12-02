# Reqvire CLI Commands Reference

## Search and Filtering

```bash
reqvire search [--json] [--short] [--filter-*]
```

Use `--short` when analyzing model structure without needing full content.
Use `--json` for programmatic processing.

### Filter Options

| Option | Description | Example |
|--------|-------------|---------|
| `--filter-file` | Filter by file glob | `--filter-file="requirements/**/*.md"` |
| `--filter-name` | Filter by element name (regex) | `--filter-name=".*Auth.*"` |
| `--filter-id` | Filter by exact identifier | `--filter-id="requirements/File.md#element"` |
| `--filter-type` | Filter by element type (see types below) | `--filter-type="user-requirement"` |
| `--filter-content` | Filter by content (regex) | `--filter-content="SHALL.*validate"` |
| `--filter-page-content` | Filter by file frontmatter | `--filter-page-content="security"` |
| `--have-relations` | Elements with ALL relations | `--have-relations="verifiedBy,satisfiedBy"` |
| `--not-have-relations` | Elements without ALL relations | `--not-have-relations="verifiedBy"` |
| `--has-attachments` | Elements with attachments | `--has-attachments` |
| `--filter-attachment` | Filter by attachment pattern | `--filter-attachment="*.pdf"` |

**Element types for --filter-type:** user-requirement, requirement, test-verification, analysis-verification, inspection-verification, demonstration-verification, constraint, behavior, specification. For custom types: `other-TYPENAME`

## Element Manipulation

### Add Element
```bash
# Add element to file (reads from stdin)
echo '### Element Name

Content here.

#### Metadata
  * type: requirement
' | reqvire add <file> [<index>]

# Add at specific position (0-based index)
echo '...' | reqvire add requirements/File.md 2

# Override existing element (replace by name)
reqvire add <file> --override <<'EOF'
### Existing Element Name

Updated content.

#### Metadata
  * type: requirement
---
EOF
```
Use `--override` to replace an existing element with the same name. Useful for cleanup after `reqvire merge`.

### Remove Element
```bash
reqvire rm "<element-name>"
reqvire rm "Feature Requirement"    # By name
```

### Move Element
```bash
reqvire mv "<element-name>" "<target-file>" [<index>]
reqvire mv "Feature Requirement" "requirements/NewFile.md"
reqvire mv "Feature Requirement" "requirements/NewFile.md" 0  # Move to top
```

### Move File
```bash
reqvire mv-file "<source>" "<target>"
reqvire mv-file "requirements/Old.md" "requirements/New.md"

# Merge into existing file (squash)
reqvire mv-file --squash "requirements/Source.md" "requirements/Target.md"
```

### Rename Element
```bash
reqvire rename "<current-name>" "<new-name>"
reqvire rename "Old Feature Name" "New Feature Name"
```

### Merge Elements
```bash
reqvire merge "<target>" "<source>" [<source2>...] [--dry-run] [--json]
reqvire merge "Parent Requirement" "Child Requirement"
reqvire merge "Main Feature" "Feature Part A" "Feature Part B" --dry-run
```
Combines source elements into target:
- Source content → target's Details section
- Source Details → "Merged Details (source name)" subsection
- Relations merged and deduplicated
- All references to sources redirected to target
- Source elements deleted after merge

Type compatibility: requirements merge with requirements, verifications with verifications, refinements with refinements.

For intelligent cleanup after merge (removing "Merged Details" artifacts), use `/reqvire:consolidate`.

### Move/Rename Asset
```bash
# Move asset file and update all references
reqvire mv-asset "<old-path>" "<new-path>"
reqvire mv-asset "docs/old-diagram.png" "docs/diagrams/new-diagram.png"
```

### Remove Asset
```bash
# Remove asset file and remove all references
reqvire rm-asset "<file-path>"
reqvire rm-asset "docs/obsolete.pdf"
```

### Link/Unlink (Relations and Attachments)
```bash
# Link two elements with a relation
reqvire link "<source>" "<relation-type>" "<target>"
reqvire link "Feature Requirement" "derivedFrom" "User Story"
reqvire link "System Requirement" "derive" "Feature Requirement"
reqvire link "Requirement" "verifiedBy" "Test Case"

# Link to implementation file or external URL
reqvire link "System Requirement" "satisfiedBy" "src/auth/login.rs"
reqvire link "Compliance Requirement" "trace" "https://example.com/spec.html"

# Attach file or refinement element (use 'attaching' keyword)
reqvire link "<element>" attaching "<path-or-refinement>"
reqvire link "Feature Requirement" attaching "docs/spec.pdf"
reqvire link "System Requirement" attaching "Performance Constraint"

# Unlink (auto-detects relation vs attachment)
reqvire unlink "<source>" "<target>"
reqvire unlink "Feature Requirement" "User Story"
reqvire unlink "Feature Requirement" "docs/spec.pdf"
```

Relation types: `derivedFrom`, `derive`, `verifiedBy`, `verify`, `satisfiedBy`, `satisfy`, `trace`
For attachments: use `attaching` keyword instead of relation type

## Validation and Analysis

### Validate Model
```bash
reqvire validate [--json]
```
Checks model consistency, broken references, invalid relations.

### Verification Coverage
```bash
reqvire coverage [--json]
```
Shows verification coverage for leaf requirements.

### Verification Traces
```bash
reqvire traces [--json] [--filter-*]
reqvire traces --filter-name=".*Auth.*"
reqvire traces --filter-type="test-verification"

# Generate with GitHub blob links
reqvire traces --links-with-blobs

# Relative links from specific folder
reqvire traces --from-folder="docs/specs"
```

### Resources Report
```bash
reqvire resources [--json]
```
Lists all files referenced by model (implementations, design docs).

### Collect Content
```bash
reqvire collect "<requirement-name>" [--json]
reqvire collect "Feature Requirement"
reqvire collect "System Requirement" --json
```
Collects and consolidates content from a requirement element and all its ancestors via derivedFrom relations. Includes attachment contents with source citations. Only works with requirement types.

### Lint Model
```bash
reqvire lint [--json]              # Show issues
reqvire lint --fixable             # Show only auto-fixable issues
reqvire lint --auditable           # Show issues needing manual review
reqvire lint --fix                 # Apply automatic fixes
```

### Change Impact
```bash
reqvire change-impact --git-commit=<hash> [--json]
reqvire change-impact --git-commit=HEAD~1
```

## Formatting

```bash
reqvire format                         # Preview changes (dry-run)
reqvire format --fix                   # Apply formatting changes
reqvire format --fix --with-full-relations  # Include auto-generated inverse relations
```

## Model Views

### Model-Centric View
```bash
reqvire model [--json]                 # All root requirements with nested relations
reqvire model --from "Element Name"    # Start from specific element

# Reverse traversal (leaf to root)
reqvire model --reverse                # Start from leaf elements (verifications)
reqvire model --reverse --json         # Reverse with JSON output

# Filter by element type
reqvire model --filter-type="user-requirement"
reqvire model --filter-type="test-verification"
reqvire model --reverse --filter-type="test-verification"  # Trace verifications upward
```

**Element types:** user-requirement, requirement, test-verification, analysis-verification, inspection-verification, demonstration-verification, constraint, behavior, specification. For custom types: `other-TYPENAME`

### Containment View
```bash
reqvire containment [--json]           # Physical file/folder structure
reqvire containment --short            # Root elements only
```

## Export

### HTML Export
```bash
reqvire export --output <directory>    # Export to directory
reqvire export                         # Export to temp directory (prints path)
```

### Serve HTML
```bash
reqvire serve [--port <port>] [--host <host>]
reqvire serve --port 8080
```

## Common Patterns

### Find unverified requirements
```bash
reqvire search --filter-type="requirement" --not-have-relations="verifiedBy" --short
```

### Find orphaned specifications
```bash
reqvire search --filter-type="specification" --not-have-relations="satisfy" --short --json
```

### Find unsatisfied test verifications
```bash
reqvire search --filter-type="test-verification" --not-have-relations="satisfiedBy" --short
```

### Quick model health check
```bash
reqvire validate && reqvire coverage --json | jq '.summary'
```

## Dry-Run Mode

Most manipulation commands support `--dry-run` to preview changes:

```bash
reqvire rm "Element Name" --dry-run
reqvire mv "Element" "target.md" --dry-run
reqvire mv-file "source.md" "target.md" --dry-run
reqvire merge "Target" "Source" --dry-run
reqvire link "Element" "derivedFrom" "Parent" --dry-run
reqvire link "Element" attaching "docs/spec.pdf" --dry-run
reqvire unlink "Element" "Parent" --dry-run
```
