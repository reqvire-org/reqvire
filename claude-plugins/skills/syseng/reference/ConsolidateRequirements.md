# Model Refactoring & Optimization

Use this reference when reorganizing the model structure without changing requirements intent. The goal is better organization, traceability, and maintainability.

**Key principle**: The system behavior specification remains unchanged - only the model structure improves.

**For common commands** (search, merge, link, validate, etc.), see [SKILL.md Command Reference](../SKILL.md#command-reference).

## Refactoring Activities

- Splitting mixed-type requirements (user vs system)
- Moving elements between files for better organization
- Adding missing relations (definedBy, derivedFrom)
- Removing redundant verify relations
- Consolidating scattered specifications

## Workflow

### Step 1: Audit Current State

Find elements that need attention during refactoring:

```bash
# Find specifications not linked to any requirement
reqvire search --filter-type='specification' --not-have-relations='define' --short --json

# Find constraints without satisfy relations
reqvire search --filter-type='constraint' --not-have-relations='define' --short

# Find requirements with reused_contract_context (candidates for conversion to relations)
reqvire search --has-reused-contract-context --short

# Find duplicate or similar requirement names
reqvire search --filter-name=".*Capability.*" --short

# Find elements in specific files that might need reorganization
reqvire search --filter-file="system-model/System/**" --short
```

These findings guide the refactoring work:
- Orphaned specifications need `definedBy` relations from appropriate requirements
- Reused Contract Context may need conversion to `definedBy` relations
- Duplicate names suggest potential merge candidates

### Step 2: Find Requirements Asking for Specifications

Search for patterns like:
- "following clearly defined specifications"
- "adhering to precondition rules"
- "shall support distinct states"
- "shall implement constraints"
- "shall enforce standardized policies"

These requirements should have `definedBy` relations to the specifications they ask for.

### Step 3: Convert Reused Contract Context to definedBy Where Appropriate

For each specification reused_contract_context, ask:
- Does this requirement *define* this specification? → Use `definedBy`
- Does this requirement *reference* or *depend on* this specification? → Keep as `Reused Contract Context`

Convert reused_contract_context to relations using link and unlink commands:

```bash
# Remove reused_contract_context from requirement
reqvire unlink "API Authorization Specification" "Authorization System Specification"

# Add definedBy relation instead
reqvire link "API Authorization Specification" "definedBy" "Authorization System Specification"
```

**When to keep reused_contract_context:**
- Requirement references but doesn't define the specification
- Specification is defined by a different requirement
- The reusesContract requirement is OUTSIDE the owner's derivation hierarchy
- The reused_contract_context target is a contract element identifier owned by another requirement

**Reused Contract Context constraints:**
- Contracts must have a `define` relation (established via requirement's `definedBy`)
- Only requirements outside the owner's hierarchy can reuse a contract
- Requirements in the same hierarchy cannot reuse - they access through the hierarchy

### Step 4: Consolidate Constraints

Find the owning requirement that asks for constraints to be defined. Add `definedBy` relations to all constraint elements using the link command:

```bash
# Link constraint to requirement that defines it
reqvire link "System Constraints Requirement" "definedBy" "Performance Constraint"
reqvire link "System Constraints Requirement" "definedBy" "Security Constraint"
```

When you find duplicate constraints, merge them:

```bash
# Preview merge to see what will happen
reqvire merge "Primary Constraint" "Duplicate Constraint" --dry-run

# Execute merge if preview looks correct
reqvire merge "Primary Constraint" "Duplicate Constraint"
```

### Step 5: Remove Obsolete Elements

After consolidation, remove elements that are no longer needed:

```bash
# Remove deprecated or duplicate element
reqvire rm "Old Requirement Name"

# Move element to better location before removing
reqvire mv "Element Name" "system-model/Archive.md"
```

**Important**: Delete cleanly - don't leave "DEPRECATED" markers. Use git history to track what was removed and why.

### Step 6: Validate After Each Change

After each refactoring step, follow the standard validation workflow. See [SKILL.md Validation & Quality Checklist](../SKILL.md#validation--quality-checklist) for the complete procedure.

Quick validation: `reqvire validate && reqvire lint --fix && reqvire coverage`


## Identifying Missing Relations

Look for specification elements with empty relations. For each:

1. Find which requirement asks for this specification to be defined
2. Change the reused_contract_context to a `definedBy` relation on that requirement
3. Keep reused_contract_context on other requirements that just reference (don't define) the specification

## Merging Duplicate Requirements

When you find overlapping requirements, use `reqvire merge`:

```bash
# Preview the merge
reqvire merge "Primary Requirement" "Duplicate Requirement" --dry-run

# Execute the merge
reqvire merge "Primary Requirement" "Duplicate Requirement"

# Merge multiple sources at once
reqvire merge "Main Capability" "Capability Part A" "Capability Part B"
```

The merge command:
1. Consolidates content from duplicate into primary's Details section
2. Merges all relations (verifiedBy, derivedFrom, etc.) with deduplication
3. Updates all references pointing to duplicate to point to primary
4. Deletes the duplicate element

**Type compatibility:**
- Requirements merge with requirements (all subtypes compatible)
- Verifications merge with verifications
- Contracts (constraint, behavior, specification) merge with each other

**When to merge vs when to link:**
- **Merge**: Elements express the same capability (duplicates)
- **Link (derivedFrom)**: Elements have hierarchical relationship (parent/child)
- **Link (trace)**: Elements are related but represent distinct capabilities

### Merge vs Link: Acceptability Criteria

When merge is **not** acceptable:
- `derivedFrom` becomes one upstream source instead of two.
- Impact analysis gets weaker: downstream behaviors cannot be attributed to either a billing-type concern or a usage-period concern.
- Verification mapping becomes blurrier: tests/evidence are reused to a combined requirement rather than the specific concern.
- Change churn increases: edits to either concern now touch the same merged requirement.

When merge is acceptable:
- They always change together.
- They share owner, lifecycle, and acceptance criteria.
- Separate compliance/audit evidence is unnecessary because the concerns are inseparable in practice.

**Post-merge cleanup**: If the merged result needs content restructuring (removing "Merged Details" artifacts), follow the two-phase consolidate workflow below.

## When to Split Requirements (using derivedFrom)

**1. Type separation** - Don't mix requirement types:
- Capabilities should capture product/stakeholder/regulatory scope, not detailed system obligations
- Requirements should capture what the system shall do, not reusable domain vocabulary or ontology structure
- Split when a capability mixes capability scope with implementable obligations, or when a requirement mixes obligations with reusable semantic definitions

**2. Change impact & containment:**
- Scope isolation - Changes shouldn't require re-verification of unrelated aspects
- Independent verification - Each child can be verified separately
- Different ownership - Parts owned by different teams
- Risk isolation - Separate high-risk from stable parts

**3. Granularity:**
- Atomic testability - Each requirement maps to clear pass/fail verification
- Single responsibility - One requirement = one clear purpose
- Stop splitting when further decomposition adds overhead without value

## Example: Before and After

### Before (Reused Contract Context):
```markdown
### API Authorization Specification

The system shall implement API Access Authorization following clearly defined specifications.

#### Metadata
  * type: capability

#### Reused Contract Context
  * [Authorization System Specification](../Specifications/AuthSpecifications.md#authorization-system-specification)
```

### After (definedBy):
```markdown
### API Authorization Specification

The system shall implement API Access Authorization following clearly defined specifications.

#### Metadata
  * type: capability

#### Relations
  * definedBy: [Authorization System Specification](../Specifications/AuthSpecifications.md#authorization-system-specification)
```

Referencing requirement (keeps reused_contract_context):
```markdown
### Add IP to Whitelist

The system shall allow adding IPs to whitelist.

#### Reused Contract Context
  * [Environment Limits](../Specifications/Constraints.md#environment-limits)
```

## Final Formatting

After completing all refactoring changes, apply formatting to ensure consistency:

```bash
# Preview formatting changes (dry-run by default)
reqvire format

# Apply formatting fixes
reqvire format --fix

# Apply formatting with full relation expansion (includes auto-generated inverse relations)
reqvire format --fix --with-full-relations
```

The format command ensures:
- Consistent markdown structure
- Proper element separator lines (`---`)
- Correct subsection ordering (Metadata, Relations, Details, Reused Contract Context)
- Clean whitespace and indentation

## Two-Phase Consolidate Workflow

Use this when content needs intelligent restructuring after merging — the AI reads the merged element, fixes the body, and overrides with a clean version.

**When to use consolidate vs direct merge:**
- **Direct merge** (`reqvire merge`): Quick merge when raw output is acceptable, or when merging duplicates
- **Consolidate** (this workflow): When content needs intelligent restructuring

### Consolidation Heuristics

A child requirement is a candidate for consolidation if it meets **multiple** of these criteria:

1. **Very similar names to parent** - e.g., parent: "Explorer Serve", child: "Explorer Serve Verification"
2. **Short content** - Less than 200 words of requirement text (excluding relations)
3. **No verifications** - Has no verifiedBy relations of its own
4. **Implementation-level details** - Mentions specific technical details, file formats, parameters, or procedural steps
5. **Leaf requirement** - No children of its own, derives from only one parent
6. **Procedural/step-by-step details** - Contains "how-to" information that expands on parent
7. **Contract keywords** - Contains phrases like "shall support", "shall provide", "shall include", "formatting", "structure", "output", "options", "flags"

### Step 1: Analyze Model for Candidates

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --short --json --output /tmp/search.json
```

Review derivedFrom relationships, requirement naming patterns, content length, and presence of verifications.

### Step 2: Execute Merge

```bash
# Preview first
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" merge "<target-element>" "<source1>" "<source2>" --dry-run

# Execute
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" merge "<target-element>" "<source1>" "<source2>"
```

### Step 3: Read Merged Element

After merge, read the merged element to see the raw output:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --filter-name="<target-name>" --json
```

The merged element will have **artifacts that need cleanup**:
- `#### Merged Details (Source Name)` subsections for each merged source
- Potentially duplicated content
- Awkward structure from concatenation

### Step 4: Prepare Fixed Element Body

Analyze the merged content and create a **clean, restructured version**:

1. **Remove all `#### Merged Details (X)` subsection headers** - These are merge artifacts
2. **Integrate all content logically into a single `#### Details` section**
3. **Remove duplicate information**
4. **Ensure proper EARS statement structure**
5. **Maintain all Relations** - These are already correctly merged
6. **Keep Metadata and Reused Contract Context** - Preserve unchanged

### Step 5: Override with Fixed Content

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

### Step 6: Validate

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate
```

### Anti-Patterns (When NOT to Consolidate)

Do NOT consolidate if:
- Child introduces **new functional capability** beyond parent
- Child has **extensive content** (>300 words) that would overwhelm parent Details
- Child has **many verifications** (3+) indicating significant independent functionality
- Child is referenced by **many other elements** as a key concept
- Child represents a **distinct abstraction level**
- There's **uncertainty** about whether child is truly contract-only

## Git Philosophy

**IMPORTANT**: When removing or changing requirements:
- **DELETE** deprecated requirements completely
- **REMOVE** broken relations
- **USE** git history to track changes (commit messages explain rationale)
- **NEVER** keep "DEPRECATED" notes or "Previous behavior" documentation
- Clean specifications are more valuable than inline deprecation notes
