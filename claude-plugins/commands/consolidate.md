---
allowed-tools: Read, Bash(npx:*)
description: Consolidate elements using automated merge + intelligent cleanup workflow
model: claude-sonnet-4-5
---

# Consolidate Requirements Model

Consolidate child requirements that only define their parents (without introducing new capabilities) into the parent requirement. This uses a two-phase workflow: automated merge followed by intelligent content cleanup.

## Model Context

- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json 2>/dev/null | jq -r '.global_counters.total_elements // "N/A"'`
- Leaf requirements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" coverage --json 2>/dev/null | jq -r '.summary.total_leaf_requirements // "N/A"'`
- Validation status: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate 2>&1 | head -1`

## When to Use

Use this command when:
- Model has grown with many small contract requirements
- Child requirements only elaborate on implementation details of parents
- You want to reduce model clutter while preserving all information
- Requirements are split via derivedFrom relations but don't add new capabilities

Use `/reqvire:semantic-refactor` instead when the model needs to split capability scope, reusable ontology meaning, and requirement obligations.

**When to use consolidate vs direct merge:**
- **Direct merge** (`reqvire merge`): Quick merge when raw output is acceptable, or when merging duplicates
- **Consolidate** (`/reqvire:consolidate`): When content needs intelligent restructuring - AI reads merged element, fixes the body, and overrides with clean version

## Consolidation Heuristics

A child requirement is a candidate for consolidation if it meets **multiple** of these criteria:

1. **Very similar names to parent** - e.g., parent: "Explorer Serve", child: "Explorer Serve Verification" or "Related System Elements"
2. **Short content** - Less than 200 words of requirement text (excluding relations)
3. **No verifications** - Has no verifiedBy relations of its own
4. **Implementation-level details** - Mentions specific technical details, file formats, parameters, or procedural steps
5. **Leaf requirement** - No children of its own, derives from only one parent
6. **Procedural/step-by-step details** - Contains "how-to" information that expands on parent
7. **Contract keywords** - Contains phrases like "shall support", "shall provide", "shall include", "formatting", "structure", "output", "options", "flags"

## Process Overview

The consolidation follows a **two-phase** approach:

**Phase 1: Automated Merge**
Use `reqvire merge` to combine elements. This automatically:
- Appends source content to target's Details section
- Creates "Merged Details (Source Name)" subsections
- Merges and deduplicates relations
- Redirects all references to sources to point to target
- Deletes source elements

**Phase 2: Intelligent Cleanup (CRITICAL)**
Read the merged element, restructure its content to remove merge artifacts, and override with the clean version using `reqvire add --override`.

## Steps

### Step 1: Analyze Model for Candidates

First, analyze the model to identify consolidation candidates:

```bash
# Get model structure
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --short --json --output /tmp/search.json
```

Review the model structure and identify parent-child requirement pairs based on:
- derivedFrom relationships
- Requirement naming patterns
- Content length and complexity
- Presence of verifications

### Step 2: Execute Merge

Once candidates are identified, execute the merge:

```bash
# Preview the merge first (recommended)
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" merge "<target-element>" "<source1>" "<source2>" --dry-run

# Execute the merge
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" merge "<target-element>" "<source1>" "<source2>"
```

**Example:**
```bash
# Merge two child contracts into parent
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" merge "Format Consistency Enforcement" "Excess Whitespace Format" "Missing Separators Format"
```

### Step 3: Read Merged Element (CRITICAL)

After merge, read the merged element to see the raw output:

```bash
# Read the merged element to see what needs cleanup
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-name="<target-name>" --json
```

The merged element will have **artifacts that need cleanup**:
- `#### Merged Details (Source Name)` subsections for each merged source
- Potentially duplicated content
- Awkward structure from concatenation
- Multiple "Details" sections or subsections

**Example of merge artifacts:**
```markdown
### Format Consistency Enforcement

The system shall provide formatting capability...

#### Details

[Original parent details]

#### Merged Details (Excess Whitespace Format)

Detect and fix excess whitespace after element headers...

#### Merged Details (Missing Separators Format)

Detect consecutive element sections...

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Formatting]
  * satisfiedBy: [format.rs]
```

### Step 4: Prepare Fixed Element Body (CRITICAL)

Analyze the merged content and create a **clean, restructured version**:

1. **Remove all `#### Merged Details (X)` subsection headers** - These are merge artifacts
2. **Integrate all content logically into a single `#### Details` section** - Combine related information
3. **Remove duplicate information** - Merge may have introduced redundancy
4. **Ensure proper EARS statement structure** - Main content should be clear requirement statement
5. **Maintain all Relations** - These are already correctly merged, keep them intact
6. **Keep Metadata and Attachments** - Preserve these unchanged

**Example of clean, restructured content:**
```markdown
### Format Consistency Enforcement

The system shall provide formatting capability for maintaining consistent document structure.

#### Details

**Excess Whitespace:**
- Detect and fix excess whitespace after element headers
- Maintain consistent formatting across all requirements documents

**Missing Separators:**
- Detect consecutive element sections that lack separators
- Insert separators to maintain consistent visual separation

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Formatting](../ModelFormatting.md#model-formatting)
  * satisfiedBy: [format.rs](../../core/src/format.rs)
```

### Step 5: Override with Fixed Content (CRITICAL)

Replace the merged element with the clean version:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "<file-path>" --override <<'EOF'
### <Element Name>

<Clean main content - EARS statement>

#### Details

<Consolidated details - all merged content properly integrated>

#### Metadata
  * type: <type>

#### Relations
  * <all merged relations - copy exactly from merged element>
---
EOF
```

**Example:**
```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "requirements/SystemRequirements/Formatting.md" --override <<'EOF'
### Format Consistency Enforcement

The system shall provide formatting capability for maintaining consistent document structure.

#### Details

**Excess Whitespace:**
- Detect and fix excess whitespace after element headers
- Maintain consistent formatting across all requirements documents

**Missing Separators:**
- Detect consecutive element sections that lack separators
- Insert separators to maintain consistent visual separation

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Model Formatting](../ModelFormatting.md#model-formatting)
  * satisfiedBy: [format.rs](../../core/src/format.rs)
---
EOF
```

### Step 6: Validate

After cleanup, validate the model:

```bash
# Validate model
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate

# If validation passes, continue to next consolidation
# If validation fails, fix issues before continuing
```

### Step 7: Format and Final Validation

After all consolidations:

```bash
# Format all files
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" format --fix

# Final validation
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
```

## Complete Workflow Example

Here's a complete example consolidating CLI option children into parent:

```bash
# 1. Identify candidates
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-name="CLI Traces" --short
# Found: CLI Traces Command, CLI Traces Filter Options, CLI Traces From-Folder Option

# 2. Execute merge
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" merge "CLI Traces Command" "CLI Traces Filter Options" "CLI Traces From-Folder Option"

# 3. Read merged element
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-name="CLI Traces Command" --json
# Shows element with "Merged Details" sections

# 4. Prepare and override with clean version
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" add "requirements/CLI/Commands.md" --override <<'EOF'
### CLI Traces Command

The system shall implement traces subcommand for generating verification trace reports.

#### Details

The traces command outputs verification traces showing upward paths from verifications to owning capability roots.

**Filter Options:**
The system shall support filtering verification traces by:
- `--filter-id=<id>`: Filter by verification element ID
- `--filter-name=<regex>`: Filter by name pattern
- `--filter-type=<type>`: Filter by verification type

**From-Folder Option:**
Support `--from-folder` option that specifies relative path for portable links:
- Accept relative path parameter
- Adjust clickable links in diagrams to be relative
- Work with both Markdown and JSON output

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Verification Trace Builder](../Capabilities/Verification/Traceability/VerificationTracesRequirements.md#verification-trace-builder)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)
---
EOF

# 5. Validate
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
```

## Anti-Patterns (When NOT to Consolidate)

Do NOT consolidate if:
- Child introduces **new functional capability** beyond parent
- Child has **extensive content** (>300 words) that would overwhelm parent Details
- Child has **many verifications** (3+) indicating significant independent functionality
- Child is referenced by **many other elements** as a key concept
- Child represents a **distinct abstraction level** (e.g., capability scope vs requirement obligation)
- There's **uncertainty** about whether child is truly contract-only

## Expected Benefits

After consolidation, the model will have:
- **Reduced clutter**: Fewer top-level requirements to navigate
- **Better organization**: Implementation details nested under conceptual parents
- **Improved readability**: Related information grouped together
- **Maintained traceability**: All relations preserved through parent requirements
- **Cleaner structure**: Hierarchical organization matches conceptual dependencies

## Verification

After consolidation is complete, verify:
- Model validates with no errors: `reqvire validate`
- All relations are preserved (no missing targets)
- Verification coverage percentage unchanged or improved
- Test verifications still link correctly to requirements
- Explorer serves correctly: `reqvire serve`
