# Specifications Extraction Logic

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
- **Clear Ownership**: `satisfiedBy` relations show which requirement owns the specification
- **Cross-References**: Attachment relations provide supporting context without ownership

## Refactoring Methodology

### Phase 1: Identification

**Candidates for Refactoring:**
- Requirements containing technical implementation details
- Requirements with algorithm descriptions
- Requirements with detailed format specifications
- Requirements with ordering rules or processing workflows

**NOT Candidates:**
- Elements already typed as `specification` or `constraints` (refinement elements)
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

#### Relations
  * satisfy: [Parent Requirement](path#requirement-name)
```

`  * type: specification` can be other refinement type depending on what this refinement element represents.

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
  * satisfiedBy: [Specification Name](path#specification-name)
  [... other existing relations ...]
```

### Phase 4: Cross-Referencing

**Ownership vs Reference:**

- **satisfiedBy Relation**: Used by the requirement that OWNS the specification (one-to-one or one-to-many)
- **Attachment Relation**: Used by requirements that REFERENCE the specification for context (many-to-one) and ARE NOT children of  specification owner requiremnent

**Hierarchical Attachment Pattern:**

When adding attachments, DO NOT ATTACH same attachment if any depth parent already has attachment attached.

**Example:**
```markdown
# In the plan
**Deterministic Output Specification** - Owned by `Model Reports`, attach to:
- `Collect Content from Requirement Chain` - *cross reference requirement, not direct child* ✓
  - MUST NOT ATTACH TO:
     - `Model Diagram Output Formats` - *grandchild ofCollect Content from Requirement Chain* X
     - `Validation Report Generator` - *grandchild of Model Diagram Output Formats* X
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
  * satisfiedBy: [Deterministic Output Specification](Specifications.md#deterministic-output-specification)

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
- File-based generation
- Shows all elements and relationships
- External resources as linked boxes

**Diagram Styling:**
- Containment structure with subgraphs
- Element type-specific CSS classes
- Interactive highlighting on hover

**Navigation and Filtering:**
- Default shows root requirements
- Filter from specific element using --from flag
- Complete model structure generation
```

**After:**
```markdown
### Interactive Mermaid Diagrams

System shall produce interactive visual representations enabling users to explore relations and navigate model structure following clearly defined specifications.

#### Relations
  * satisfiedBy: [Mermaid Diagram Generation Specification](...)
  * satisfiedBy: [Mermaid Interactive Features Specification](...)

---

### Mermaid Diagram Generation Specification
[Generation approach and styling content]

#### Metadata
  * type: specification


---

### Mermaid Interactive Features Specification
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
  * type: user-requirement

#### Relations
  * satisfiedBy: [Rate Limits](../Specifications/Constraints.md#rate-limits)
  * satisfiedBy: [Session Limits](../Specifications/Constraints.md#session-limits)
```


## Decision Rules

### When to Extract a Specification

Extract when ANY of these conditions are true:
2. Content describes HOW system implements (not WHAT or WHY)
3. Content includes algorithms, workflows, or processing rules
4. Content defines output formats or data structures
5. Content describes technical constraints or ordering rules
6. Multiple requirements could benefit from referencing (attaching) this content

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

### Attachment vs satisfiedBy

**Use satisfiedBy when:**
- Requirement OWNS the specification
- Specification was extracted FROM this requirement
- Requirement has primary responsibility for the technical content

**Use Attachment when:**
- Requirement REFERENCES specification for context
- Specification owned by another requirement
- Specification provides supporting technical details
- Multiple requirements benefit from this specification and must adhere / implement it

**Hierarchical Priority:**
- Attachments are inherit from parent and if ALL childrend must adhere to same specifications attach it to parent then
- Child should not inherit specification if not relevant to it
- Mark hierarchical relationships in refactoring plans for clarity

## Quality Metrics

### Success Criteria

After refactoring, verify:
1. **Conciseness**: All requirements have Details under 15 lines
2. **Clarity**: Requirements focus on user value, not implementation
3. **Reusability**: Specifications referenced by multiple requirements where appropriate
4. **Traceability**: Clear ownership via satisfiedBy relations
5. **Validation**: `reqvire validate` shows no errors
7. **Formatting**: All files properly formatted

### Quantitative Metrics

Track these metrics:
- **Line Reduction**: Requirements should be reduced by 80-90%
- **Specifications Created**: Typically 1-2 per complex requirement
- **Cross-References**: Average 3-5 attachments per specification

### Example Metrics (Phase 2)

```
Specifications Extracted:     5
Requirements Refactored:      4
Total Line Reduction:         ~179 → ~20 lines (88.8%)
Cross-Reference Attachments:  22 total
Hierarchical Attachments:     15 total
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

**Problem**: Multiple requirements use satisfiedBy for same specification
**Solution**: Only owner uses satisfiedBy, others use attachments

### Pitfall 4: Orphaned Specifications

**Problem**: Creating specifications not owned by any requirement
**Solution**: Always create satisfiedBy relation from owner to specification

### Pitfall 5: Inconsistent Granularity

**Problem**: Some specifications too fine-grained, others too coarse
**Solution**: Balance specificity - aim for cohesive, reusable technical units

## Tools and Automation

### Finding Refactoring Candidates (Phase 1)

Use search to identify requirements that may need specification extraction:

```bash
# Find all requirements (candidates for review)
reqvire search --filter-type="requirement,user-requirement" --short

# Find requirements in specific subsystem
reqvire search --filter-type="requirement" --filter-file="requirements/System/**" --short

# Find requirements with attachments (may need conversion to satisfiedBy)
reqvire search --filter-type="requirement" --has-attachments --short

# Find refinements without satisfy relations (orphaned specifications, constraints, behaviors)
reqvire search --filter-type="specification,constraint,behavior" --not-have-relations="satisfy" --short
```

### Validation Commands (Phase 5)

After extracting specifications, validate the refactored model:

```bash
# Validate model structure and relations
reqvire validate

# Preview formatting changes
reqvire format

# Apply formatting fixes
reqvire format --fix

# Apply formatting with full relation expansion
reqvire format --fix --with-full-relations
```

### Finding Candidates (Manual Review)

Use Explore agents to identify requirements with:
- Long Details sections (grep for element length)
- Technical keywords (algorithm, format, structure, ordering, rules)
- Implementation-focused content
