# Specifications Extraction Logic

**For common commands and validation workflow**, see [SKILL.md Command Reference](../SKILL.md#command-reference) and [Validation & Quality Checklist](../SKILL.md#validation--quality-checklist).

## Purpose

This document describes the logic and methodology for refactoring requirements to separate EARS statements (what/why) from technical specifications (how), following MBSE best practices.

## Problem Statement

Requirements with embedded technical specifications suffer from:
- **Reduced Clarity**: User needs are obscured by implementation details
- **Poor Reusability**: Technical specs can't be referenced by multiple requirements
- **Maintenance Burden**: Changing implementation details requires editing requirements
- **Reduced Traceability**: Ownership relationships are unclear between requirements and specifications

## Solution: Separation of Concerns

Extract technical specifications from requirement Details sections into separate specification elements, creating:
- **Concise Requirements**: EARS-style statements focused on user value (under 15 lines)
 - Main body has one more general statement and all other must be written in '#### Details' subsection
- **Reusable Specifications**: Technical details in standalone elements
- **Clear Ownership**: `definedBy` relations show which requirement owns the specification
- **Cross-References**: Reused Contract Context relations provide supporting context without ownership

## Refactoring Methodology

### Phase 1: Identification

**Candidates for Refactoring:**
- Requirements containing technical implementation details
- Requirements with algorithm descriptions
- Requirements with detailed format specifications
- Requirements with ordering rules or processing workflows

**NOT Candidates:**
- Elements already typed as `specification` or `constraints` (contract elements)
- Requirements where Details add essential context without implementation details like success criteria and such

**Examples of hardcoded limits to extract as constraints:**

*Web App:*
- "Session expires after 30 minutes of inactivity"
- "Maximum file upload size: 10 MB"
- "Password must be 8-128 characters"

*API:*
- "Rate limit: 100 requests per minute"
- "Maximum payload size: 1 MB"
- "Token expiration: 24 hours"

*Database:*
- "Connection pool maximum: 20 connections"
- "Query timeout: 30 seconds"

### Phase 2: Analysis

For each candidate requirement:

1. **Identify Embedded Specifications**: Look for technical content describing HOW the system implements the requirement
2. **Extract Technical Content**: Separate implementation details from the user need
3. **Determine Specification Granularity**: Decide if content forms one specification or multiple related specifications
4. **Identify Cross-References**: Find other requirements that would benefit from referencing these specifications

### Phase 3: Extraction

**Creating the Specification Element:**

```markdown
### Specification Name

Brief description of what this specification defines.

#### Details
[Technical content extracted from requirement]

**Section Headers** (where appropriate):
- Use bold headers to organize complex specifications
- Group related rules and behaviors
- Maintain readability

#### Metadata
  * type: specification

```

`  * type: specification` can be other contract type depending on what this contract element represents.

**Reducing the Requirement:**

```markdown
### Requirement Name

Concise EARS-style statement (1 sentence).

#### Details
- Other EARS statement
- Other EARS statement
- Or if bigger statement 

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Specification Name](path#specification-name)
  [... other existing relations ...]
```

### Phase 4: Cross-Referencing

**Ownership vs Reference:**

- **definedBy Relation**: Used by the requirement that OWNS the specification (one-to-one or one-to-many)
- **Reused Contract Context Relation**: Used by requirements OUTSIDE the owner's derivation hierarchy

**Reused Contract Context Scope Constraints:**

1. Contracts must have a `define` relation (established via requirement's `definedBy`)
2. Only requirements OUTSIDE the owner's hierarchy can reuse the contract
3. Requirements in the same hierarchy (ancestors or descendants of owner) CANNOT reuse

**Example:**
```markdown
# In the plan
**Deterministic Output Specification** - Owned by `Model Reports`, reuse to:
- `Some Other Capability` - *not in Model Reports hierarchy* ✓
  - MUST NOT ATTACH TO (in owner's hierarchy):
     - `Model Structure and Summaries` - *child of Model Reports* ✗
     - `Validation Report Generator` - *grandchild of Model Reports* ✗
```

### Phase 5: Validation

After refactoring:
1. Run `reqvire validate` - ensure no validation errors
2. Run `reqvire format --fix` - normalize formatting
3. Review requirement conciseness - confirm under 15 lines

## Refactoring Patterns

### Pattern 1: Single Specification Extraction

**Before:**
```markdown
### Model Reports

When requested the system shall provide deterministic model reports.

#### Details
All reports shall produce deterministic output with consistent ordering.

The system shall ensure deterministic output by:
1. Element Ordering: Sort by identifier
2. Relation Ordering: Sort by type then target
3. Section Ordering: Alphabetical
4. File Ordering: Alphabetical

This ensures:
- Byte-identical output on repeated runs
- Reliable test comparison
- Meaningful version control diffs

Applies to: model summary, verification tracing, coverage, change impact, validation, linting
```

**After:**
```markdown
### Model Reports

When requested the system shall provide human readable and machine readable System model reports with deterministic output and consistent ordering following clearly defined specifications.

#### Relations
  * definedBy: [Deterministic Output Specification](Specifications.md#deterministic-output-specification)

---

### Deterministic Output Specification

Technical specification for ensuring deterministic, reproducible output.

#### Details
[All technical content moved here]

#### Metadata
  * type: specification


```

### Pattern 2: Multiple Specification Extraction

**Before:**
```markdown
### Interactive Mermaid Diagrams

System shall produce interactive visual representations.

#### Details
**Diagram Generation Approach:**
- Report-output generation
- Shows the selected model scope and relationships
- Emits Markdown-embedded Mermaid or pure Mermaid text depending on command mode

**Diagram Styling:**
- Containment structure with subgraphs
- Element type-specific CSS classes
- Interactive highlighting on hover

**Navigation and Filtering:**
- Default shows capability-rooted model structure
- Filter from specific element using --from flag
- Complete model structure generation
```

**After:**
```markdown
### Interactive Mermaid Diagrams

System shall produce interactive visual representations enabling users to explore relations and navigate model structure following clearly defined specifications.

#### Relations
  * definedBy: [Mermaid Diagram Generation Specification](...)
  * definedBy: [Mermaid Interactive Capabilities Specification](...)

---

### Mermaid Diagram Generation Specification
[Generation approach and styling content]

#### Metadata
  * type: specification


---

### Mermaid Interactive Capabilities Specification
[Navigation and filtering content]

#### Metadata
  * type: specification


```

## Pattern 3: Constraint Consolidation:

**Before:**
```markdown
### Operational Constraints

System shall implement session and rate limits

#### Details

Session limit will be 23 hours duration .
Rate limit will be 100 request per hour.
```


**After:**
```markdown
### Operational Constraints

The system shall implement operational constraints and rate limits.

#### Metadata
  * type: capability

#### Relations
  * definedBy: [Rate Limits](../Specifications/Constraints.md#rate-limits)
  * definedBy: [Session Limits](../Specifications/Constraints.md#session-limits)
```


## Decision Rules

### When to Extract a Specification

Extract when ANY of these conditions are true:
2. Content describes HOW system implements (not WHAT or WHY)
3. Content includes algorithms, workflows, or processing rules
4. Content defines output formats or data structures
5. Content describes technical constraints or ordering rules
6. Multiple requirements could benefit from referencing (reusesContract) this content

### When NOT to Extract

Keep content in requirement when:
1. Element is already type refirement type
2. Details section with essential context
3. Content describes rationale or business justification or success criteria (WHY)
4. Content provides examples clarifying the requirement
5. Extraction would make requirement too abstract to understand

### Specification Naming

Specifications should be named:
- **Descriptively**: Clear indication of what technical aspect they specify
- **Consistently**: Follow existing naming patterns in the subsystem
- **Specifically**: Not too generic (avoid "General Specification")

Examples:
- ✓ `Deterministic Output Specification`
- ✓ `Diagram Relation Filtering Specification`
- ✓ `Resources Report Format Specification`
- ✗ `Output Specification` (too generic)
- ✗ `Report Spec` (not descriptive enough)

### Specification Content

Specifications MUST NOT use EARS statements as those are not requirements.

### Reused Contract Context vs definedBy

**Use definedBy when:**
- Requirement OWNS the specification
- Specification was extracted FROM this requirement
- Requirement has primary responsibility for the technical content

**Use Reused Contract Context when:**
- Requirement REFERENCES specification for context
- Specification owned by a requirement in a DIFFERENT derivation hierarchy
- Specification provides supporting technical details
- Multiple requirements (from different hierarchies) benefit from this specification

**Reused Contract Context Constraint:**
- Requirements in the same hierarchy as the owner CANNOT reuse the contract
- They access the contract through the hierarchy relationship instead
- Cross-hierarchy reused_contract_context enable requirements from separate branches to reference shared specs

## Quality Metrics

### Success Criteria

After refactoring, verify:
1. **Conciseness**: All requirements have Details under 15 lines
2. **Clarity**: Requirements focus on user value, not implementation
3. **Reusability**: Specifications referenced by multiple requirements where appropriate
4. **Traceability**: Clear ownership via definedBy relations
5. **Validation**: `reqvire validate` shows no errors
7. **Formatting**: All files properly formatted

### Quantitative Metrics

Track these metrics:
- **Line Reduction**: Requirements should be reduced by 80-90%
- **Specifications Created**: Typically 1-2 per complex requirement
- **Cross-References**: Average 3-5 reused_contract_context per specification

### Example Metrics (Phase 2)

```
Specifications Extracted:     5
Requirements Refactored:      4
Total Line Reduction:         ~179 → ~20 lines (88.8%)
Cross-Reference Reused Contract Context:  22 total
Hierarchical Reused Contract Context:     15 total
Validation Errors:            0
```

## Common Pitfalls

### Pitfall 1: Over-Extraction

**Problem**: Extracting every detail creates specification explosion
**Solution**: Merge several specifications and requirement into one.

### Pitfall 2: Losing Context

**Problem**: Requirement becomes too abstract after extraction
**Solution**: Keep essential context in requirement, extract only technical implementation

### Pitfall 3: Unclear Ownership

**Problem**: Multiple requirements use definedBy for same specification
**Solution**: Only owner uses definedBy, others use reused_contract_context

### Pitfall 4: Orphaned Specifications

**Problem**: Creating specifications not owned by any requirement
**Solution**: Always create definedBy relation from owner to specification

### Pitfall 5: Inconsistent Granularity

**Problem**: Some specifications too fine-grained, others too coarse
**Solution**: Balance specificity - aim for cohesive, reusable technical units

## Tools and Automation

### Finding Refactoring Candidates (Phase 1)

Use search to identify requirements that may need specification extraction:

```bash
# Find all requirements (candidates for review)
reqvire search --filter-type="requirement" --short

# Find capabilities whose prose may need concept references
reqvire search --filter-type="capability" --short

# Find requirements in specific subsystem
reqvire search --filter-type="requirement" --filter-file="system-model/System/**" --short

# Find requirements with reused_contract_context (may need conversion to satisfiedBy)
reqvire search --filter-type="requirement" --has-reused-contract-context --short

# Find contracts without define relations (orphaned specifications, constraints, behaviors, state, input-output, sources, and )
reqvire search --filter-type="specification,constraint,behavior,state,input-output,source" --not-have-relations="define" --short
```

### Validation Commands (Phase 5)

After extracting specifications, validate the refactored model using the standard validation workflow. See [SKILL.md Validation & Quality Checklist](../SKILL.md#validation--quality-checklist) for the complete procedure.

Quick validation: `reqvire validate && reqvire lint --fix && reqvire format --fix`

### Finding Candidates (Manual Review)

Use Explore agents to identify requirements with:
- Long Details sections (grep for element length)
- Technical keywords (algorithm, format, structure, ordering, rules)
- Implementation-focused content
