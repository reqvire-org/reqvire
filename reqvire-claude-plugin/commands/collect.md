---
allowed-tools: Read, Bash(reqvire:*)
argument-hint: <element-name>
description: Collect and summarize requirement context via derivedFrom chain
model: claude-sonnet-4-5-20250929
---

# Collect Requirement Context

Collect and present a comprehensive summary of requirement context via the derivedFrom chain.

## Element

${1:+Target element: **$1**}
${1:-The user will provide the element name.}

## Steps

### 1. Collect Raw Content

Run the `reqvire collect` command to gather the complete requirement context:

```bash
reqvire collect "${1}" --json > /tmp/collect_output.json
```

This command collects:
- The target element's content and metadata
- Full derivedFrom chain (all ancestor requirements)
- Attached specifications, constraints, and behaviors
- Related documentation files
- Source citations and file paths
- Verification and satisfaction relations

**Error Handling:**
- If the command fails, check that the element name is correct
- Element names are case-sensitive and must match exactly
- Use `reqvire search --short` to find the exact element name if needed

### 2. Read and Analyze JSON Output

Read the collected JSON file:

```bash
# The file /tmp/collect_output.json contains structured data
```

The JSON structure includes:
- **`element`**: The target element's content, metadata, and type
- **`ancestors`**: Array of parent elements in the derivedFrom chain (from root to target)
- **`attachments`**: Specifications, constraints, and behaviors attached to requirements
- **`documents`**: Related markdown documentation files with content
- **`citations`**: Source file paths and anchors for traceability
- **`relations`**: verifiedBy and satisfiedBy links to other elements

Extract and organize:
- **Target element details**: Name, type, full content
- **Derivation hierarchy**: Parent requirements showing refinement path
- **Attached specifications**: All specs, constraints, behaviors
- **Documentation**: Content from attached markdown files
- **Verification info**: verifiedBy relations and test criteria
- **Implementation info**: satisfiedBy relations and code references
- **Source locations**: File paths and anchors for all elements

### 3. Generate Comprehensive Context Document

Create a complete, readable markdown document that synthesizes all collected information into a coherent narrative.

**CRITICAL FORMATTING REQUIREMENTS:**
- **NO page limits** - include complete details regardless of length
- **Rephrase all content** into coherent, readable narrative (don't copy-paste raw data)
- **Synthesize and explain** - make it flow as documentation someone would want to read
- **Organize by topic** - not by source or element type
- **All references at end** - citations and traceability links in References section

#### Document Structure

```markdown
# Requirement Context: [Element Name]

## Overview

[Write a comprehensive description of what this requirement accomplishes, its purpose, and business value. Rephrase the requirement content to be narrative and readable, not just quoted. Explain what problem it solves and who benefits.]

## Background

[Explain the requirement chain and why this requirement exists. Describe the derivedFrom chain from root requirements down to this element. For each level in the hierarchy, explain the refinement - how each child adds specificity or detail to its parent. This section tells the "why" story.]

## Detailed Specifications

### Core Requirements

[Include ALL requirement content, rephrased as a coherent narrative. Break down into logical subsections if the requirement is complex. Don't just list - explain what the requirement means and implies. Include all details but make them readable.]

### Implementation Details

[Describe how this requirement is satisfied. Extract information from satisfiedBy relations and attached specifications:]

- **Code/Components**: [List and describe all satisfiedBy elements - what they are and how they satisfy the requirement]
- **API Endpoints**: [If applicable, detail all API specifications including methods, paths, parameters, responses]
- **Algorithms**: [If applicable, describe technical approaches and logic]
- **Data Structures**: [If applicable, describe data models and schemas]
- **Architecture**: [If applicable, explain system components and interactions]

### Constraints and Validation

[Detail ALL constraints and validation rules from attached constraint elements. Don't just list - explain what each constraint means, why it exists, and what it prevents or ensures. Make the constraints understandable.]

### Attached Specifications

[Include content from ALL attached specifications, constraints, and behaviors. Rephrase this content to flow naturally as part of the document narrative. Synthesize related specs together rather than treating them as separate items.]

## Verification

[Describe how this requirement is verified, extracted from verifiedBy relations and attached verification elements:]

- **Verification Methods**: [List and describe verifiedBy elements - what type of verification (test, inspection, analysis, demonstration)]
- **Test Criteria**: [Explain what the tests validate and how they prove the requirement is satisfied]
- **Acceptance Criteria**: [Detail what constitutes successful verification - what must pass]
- **Coverage**: [If applicable, mention any unverified aspects or verification gaps]

## Related Documentation

[Describe attached documentation files and their relevance. Include key insights and important information from these documents. Summarize what developers need to know from the documentation.]

## References

[All source citations, file paths, and traceability links go here at the end:]

**Source:**
- File: [path/to/file.md#element-anchor]
- Type: [requirement/specification/etc.]

**Derived From:**
[List all parent requirements in the derivedFrom chain with links]
- [Parent Requirement 1](path/to/file.md#parent-1)
- [Parent Requirement 2](path/to/file.md#parent-2)

**Verified By:**
[List all verifications that verify this requirement]
- [Verification 1](path/to/file.md#verification-1)
- [Verification 2](path/to/file.md#verification-2)

**Satisfied By:**
[List all implementations that satisfy this requirement]
- [Implementation 1](path/to/file.md#impl-1)
- [Implementation 2](path/to/file.md#impl-2)

**Attached Elements:**
[List all attached specifications, constraints, behaviors]
- [Specification 1](path/to/file.md#spec-1)
- [Constraint 1](path/to/file.md#constraint-1)

**Related Documents:**
[List all attached documentation files]
- [Document 1](path/to/doc.md)

---
*Generated by `reqvire collect "[Element Name]"`*
*For raw JSON data, see `/tmp/collect_output.json`*
```

## Output Guidelines

### Writing Style

- **Narrative, not lists**: Write flowing prose that explains concepts
- **Synthesis**: Combine related information from multiple sources
- **Context**: Explain the "why" behind decisions and requirements
- **Clarity**: Use clear language that new team members can understand
- **Completeness**: Include ALL details - nothing should be omitted
- **Length**: As long as needed - simple features might be 1-2 pages, complex features might be 5-10 pages

### Organization

- **Logical grouping**: Group related specifications together
- **Progressive detail**: Start with overview, then dive into details
- **Topic-based**: Organize by what the information is about, not where it came from
- **References at end**: All source links in one place for easy lookup

### Content Treatment

- **Rephrase everything**: Don't copy-paste requirement text verbatim
- **Explain implications**: What does this requirement mean in practice?
- **Connect the dots**: How do different pieces relate to each other?
- **Extract insights**: What are the key takeaways?
- **Preserve precision**: Keep technical accuracy while improving readability

## Notes

- The collect command gathers complete context, but your job is to make it readable
- Larger requirements will naturally produce longer documents - that's expected
- Focus on creating documentation that developers would want to read
- All traceability is preserved in the References section
- The goal is comprehensive understanding, not brevity
- Use the JSON data to inform the narrative, but don't dump JSON into the output
