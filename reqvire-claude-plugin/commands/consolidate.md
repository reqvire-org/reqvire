---
allowed-tools: Read, Edit, Bash(reqvire:*)
description: Consolidate refinement-only child requirements into their parent requirements' Details sections to improve model organization
model: claude-sonnet-4-5-20250929
---

# Consolidate Requirements Model

Consolidate child requirements that only refine their parents (without introducing new capabilities) into the parent requirement's Details section. This improves model readability by merging implementation-level details into their conceptual parents while maintaining full traceability.

## When to Use

Use this command when:
- Model has grown with many small refinement requirements
- Child requirements only elaborate on implementation details of parents
- You want to reduce model clutter while preserving all information
- Requirements are split via derivedFrom relations but don't add new capabilities

## Consolidation Heuristics

A child requirement is a candidate for consolidation if it meets **multiple** of these criteria:

1. **Very similar names to parent** - e.g., parent: "HTML Export", child: "HTML Export Verification" or "Export Related System Elements"
2. **Short content** - Less than 200 words of requirement text (excluding relations)
3. **No verifications** - Has no verifiedBy relations of its own
4. **Implementation-level details** - Mentions specific technical details, file formats, parameters, or procedural steps
5. **Leaf requirement** - No children of its own, derives from only one parent
6. **Procedural/step-by-step details** - Contains "how-to" information that expands on parent
7. **Refinement keywords** - Contains phrases like "shall support", "shall provide", "shall include", "formatting", "structure", "output", "options", "flags"

## Process Overview

The consolidation follows this systematic approach:

1. **Identify Candidates**: Find parent-child pairs where child is refinement-only
2. **Merge Content**: Move child requirement content into parent's Details section
3. **Move Relations**: Transfer all child relations (satisfiedBy, verifiedBy, etc.) to parent
4. **Remove Child**: Delete the child requirement element entirely
5. **Update References**: Find and update all references to child to point to parent
6. **Validate**: Ensure no broken relations after consolidation

## Steps

### 1. Analyze Model for Candidates

First, analyze the model to identify consolidation candidates:

```bash
# Remove diagrams to save tokens
reqvire remove-diagrams

# Get model structure
reqvire search --short --json > /tmp/search.json
```

Review the model structure and identify parent-child requirement pairs based on:
- derivedFrom relationships
- Requirement naming patterns
- Content length and complexity
- Presence of verifications

### 2. Review and Prioritize Candidates

For each candidate family, document:
- **Parent requirement**: File location, section, heading
- **Child requirements**: List of elements to consolidate
- **Justification**: Which heuristics apply (similar names, short content, etc.)
- **Relations to move**: All satisfiedBy, verifiedBy, trace relations from children
- **References to update**: Other elements that reference the children

Prioritize by:
- **High priority**: Clear refinement patterns, minimal relations, obvious implementation details
- **Medium priority**: Mixed signals, moderate impact
- **Low priority**: Substantial content, many verifications, borderline cases

### 3. Execute Consolidation (Per Requirement Family)

For each requirement family to consolidate:

#### 3.1 Read Parent and Children

```bash
# Read the parent requirement file
Read specifications/path/to/ParentFile.md

# Read sections containing children if in different files
Read specifications/path/to/ChildFile.md
```

#### 3.2 Create Enhanced Parent Details

Edit the parent requirement to add consolidated content from children:

**Structure for consolidated Details section:**
```markdown
#### Details

[Existing parent details content]

**[Child 1 Heading]:**
[Child 1 description and key details]

[Child 1 specific details as subsections or bullets]

**[Child 2 Heading]:**
[Child 2 description and key details]

[Child 2 specific details as subsections or bullets]
```

**Example:**
```markdown
#### Details

The system shall provide formatting capability...

**Excess Whitespace:**
- Detect and fix excess whitespace after element headers
- Maintain consistent formatting across all requirements documents

**Inconsistent Newlines:**
- Detect and fix excess or missing newlines before element headers
- Normalize to exactly two newlines before subsections
```

#### 3.3 Merge Relations to Parent

Add all relations from children to the parent's Relations section:

```markdown
#### Relations
  * derivedFrom: [Existing Parent Relations]
  * satisfiedBy: [parent-file.rs]
  * satisfiedBy: [child1-file.rs]  # Moved from Child 1
  * satisfiedBy: [child2-file.rs]  # Moved from Child 2
  * verifiedBy: [Parent Verification]
  * verifiedBy: [Child Verification]  # Moved from Child 1
```

#### 3.4 Remove Child Requirements

Delete each child requirement entirely (full H3 section including heading, content, metadata, relations, separator):

```markdown
### Child Requirement Name

[DELETE THIS ENTIRE SECTION]

---
```

#### 3.5 Update References to Children

Search for any elements that reference the removed children:

```bash
# Search for references to the child
reqvire search --filter-name="Child.*Name" --json
```

For each reference found, update it to point to the parent:

**Before:**
```markdown
* verifiedBy: [Child Requirement](File.md#child-requirement)
```

**After:**
```markdown
* verifiedBy: [Parent Requirement](File.md#parent-requirement)
```

#### 3.6 Validate After Each Family

```bash
# Validate model after consolidating each family
reqvire validate

# If validation passes, continue to next family
# If validation fails, fix broken references before continuing
```

### 4. Final Validation and Formatting

After all consolidations:

```bash
# Validate entire model
reqvire validate

# Format all files
reqvire format --fix

# Regenerate diagrams
reqvire generate-diagrams

# Final validation
reqvire validate
```

## Consolidation Examples

### Example 1: Format Implementation Family

**Before Consolidation:**
```markdown
### Format Consistency Enforcement
The system shall provide formatting capability...
#### Relations
  * derivedFrom: [Model Formatting]

---

### Excess Whitespace Format Implementation
Detect and fix excess whitespace...
#### Relations
  * derivedFrom: [Format Consistency Enforcement]
  * satisfiedBy: [format.rs]

---

### Missing Separators Format Implementation
Detect consecutive element sections...
#### Relations
  * derivedFrom: [Format Consistency Enforcement]
  * satisfiedBy: [format.rs]
```

**After Consolidation:**
```markdown
### Format Consistency Enforcement
The system shall provide formatting capability...

#### Details
**Excess Whitespace:**
- Detect and fix excess whitespace after element headers
- Maintain consistent formatting

**Missing Separators:**
- Detect consecutive element sections that lack separators
- Insert separators to maintain consistent visual separation

#### Relations
  * derivedFrom: [Model Formatting]
  * satisfiedBy: [format.rs]
```

### Example 2: CLI Options Family

**Before:**
```markdown
### CLI Traces Command
The system shall implement traces subcommand...
#### Relations
  * derivedFrom: [Verification Trace Builder]

---

### CLI Traces Filter Options
Support filtering verification traces...
#### Relations
  * derivedFrom: [CLI Traces Command]

---

### CLI Traces From-Folder Option
Support --from-folder option...
#### Relations
  * derivedFrom: [CLI Traces Command]
```

**After:**
```markdown
### CLI Traces Command
The system shall implement traces subcommand...

#### Details
[Original command details]

**Filter Options:**
The system shall support filtering verification traces by:
- --filter-id=<id>: Filter by verification element ID
- --filter-name=<regex>: Filter by name pattern
- --filter-type=<type>: Filter by verification type

**From-Folder Option:**
Support --from-folder option that specifies relative path for portable links:
- Accept relative path parameter
- Adjust clickable links in diagrams to be relative
- Work with both Markdown and JSON output

#### Relations
  * derivedFrom: [Verification Trace Builder]
  * satisfiedBy: [cli.rs]
```

## Best Practices

1. **Work incrementally**: Consolidate one requirement family at a time
2. **Validate frequently**: Run `reqvire validate` after each consolidation
3. **Preserve all information**: Don't lose any technical details during consolidation
4. **Maintain traceability**: Move ALL relations from children to parent
5. **Update references**: Search and fix all references to removed children
6. **Test thoroughly**: Ensure model validates and all links work after consolidation
7. **Document decisions**: Keep notes on which families were consolidated and why

## Anti-Patterns (When NOT to Consolidate)

Do NOT consolidate if:
- Child introduces **new functional capability** beyond parent
- Child has **extensive content** (>300 words) that would overwhelm parent Details
- Child has **many verifications** (3+) indicating significant independent functionality
- Child is referenced by **many other elements** as a key concept
- Child represents a **distinct abstraction level** (e.g., user requirement vs system requirement)
- There's **uncertainty** about whether child is truly refinement-only

## Expected Benefits

After consolidation, the model will have:
- **Reduced clutter**: Fewer top-level requirements to navigate
- **Better organization**: Implementation details nested under conceptual parents
- **Improved readability**: Related information grouped together
- **Maintained traceability**: All relations preserved through parent requirements
- **Cleaner structure**: Hierarchical organization matches conceptual dependencies

## Verification

After consolidation is complete, verify:
- ✅ Model validates with no errors: `reqvire validate`
- ✅ All relations are preserved (no missing targets)
- ✅ Verification coverage percentage unchanged or improved
- ✅ Test verifications still link correctly to requirements
- ✅ Diagrams generate without errors: `reqvire generate-diagrams`
- ✅ Documentation exports correctly: `reqvire export --html`
