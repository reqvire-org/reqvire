---
allowed-tools: Read, Bash(npx:*)
argument-hint: <element-name>
description: Collect and summarize element trace context via upstream or downstream relations
model: claude-sonnet-4-5
---

# Collect Element Trace Context

Collect and present a comprehensive summary of capability, requirement, contract, verification, and implementation context via the upstream trace chain by default or the downstream trace chain when requested.

## Model Context

- Total elements: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" search --json 2>/dev/null | jq -r '.global_counters.total_elements // "N/A"'`
- Validation status: !`npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate 2>&1 | head -1`

## Element

${1:+Target element: **$1**}
${1:-The user will provide the element name.}

## Steps

### 1. Collect Raw Content

Run the `reqvire collect` command to gather the complete trace context:

```bash
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" collect "${1}" --json --output /tmp/collect_output.json
```

This command collects:
- The target element's content and metadata
- Full upstream trace chain, including owning capabilities and ancestor requirements
- Attached/refining specifications, constraints, behaviors, states, and input-output contracts, plus semantic contracts reached through explicit `constrain`/`constrainedBy` relations
- Related documentation files
- Source citations and file paths
- Verification, satisfaction, and contract relations

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
- **`ancestors`**: Array of upstream trace elements from root to target
- **`attachments`**: Specifications, constraints, behaviors, states, and input-output contracts refining the context; semantic contracts are reached through explicit `constrain`/`constrainedBy` relations
- **`documents`**: Related markdown documentation files with content
- **`citations`**: Source file paths and anchors for traceability
- **`relations`**: verifiedBy, satisfiedBy, and definedBy links to other elements

Extract and organize:
- **Target element details**: Name, type, full content
- **Trace hierarchy**: Owning capability path and requirement contract path
- **Attached contracts**: All specifications, constraints, behaviors, states, input-output contracts, and semantic contracts
- **Documentation**: Content from attached markdown files
- **Verification info**: verifiedBy relations and test criteria
- **Implementation info**: satisfiedBy relations and code references
- **Contract info**: definedBy relations to compatible contract elements
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
# Trace Context: [Element Name]

## Overview

[Write a comprehensive description of what this element represents, its purpose, and its engineering value. For a capability, explain what the system is able to accomplish. For a requirement, explain the obligation or guarantee it imposes. Rephrase content into narrative form, not just quoted text.]

## Background

[Explain the ontology, capability, requirement, and verification context around the target element. Describe the owning capability context, the specify bridge when present, and the derivedFrom chain down to this element. For each level in the hierarchy, explain how each child adds specificity or detail to its parent. This section tells the "why" story.]

## Detailed Specifications

### Core Requirements

[Include all relevant capability and requirement content, rephrased as a coherent narrative. Break down complex material into logical subsections. Explain what each capability means operationally and what each requirement guarantees.]

### Implementation Details

[Describe how this requirement is implemented and refined. Extract information from satisfiedBy and definedBy relations and attached specifications:]

- **Code/Components**: [List and describe all satisfiedBy elements - what they are and how they satisfy the requirement]
- **Contracts**: [List and describe all definedBy elements - specifications, constraints, behaviors that define the requirement]
- **API Endpoints**: [If applicable, detail all API specifications including methods, paths, parameters, responses]
- **Algorithms**: [If applicable, describe technical approaches and logic]
- **Data Structures**: [If applicable, describe data models and schemas]
- **Architecture**: [If applicable, explain system components and interactions]

### Constraints and Validation

[Detail all constraints, state rules, semantic contracts, and validation rules from attached contracts and explicit semantic-contract constraint/use relations. Explain what each rule means, why it exists, and what it prevents or ensures.]

### Attached Specifications

[Include content from all attached specifications, constraints, behaviors, states, input-output contracts, and explicitly constraining semantic contracts. Rephrase this content to flow naturally as part of the document narrative. Synthesize related contracts together rather than treating them as separate items.]

## Verification

[Describe how this element is verified, extracted from verifiedBy relations and attached verification elements:]

- **Verification Methods**: [List and describe verifiedBy elements - what type of verification (test, inspection, analysis, demonstration)]
- **Test Criteria**: [Explain what the tests validate and how they prove the capability or requirement is satisfied]
- **Acceptance Criteria**: [Detail what constitutes successful verification - what must pass]
- **Coverage**: [If applicable, mention any unverified aspects or verification gaps]

## Related Documentation

[Describe attached documentation files and their relevance. Include key insights and important information from these documents. Summarize what developers need to know from the documentation.]

## References

[All source citations, file paths, and traceability links go here at the end:]

**Source:**
- File: [path/to/file.md#element-anchor]
- Type: [requirement/specification/etc.]

**Trace Chain:**
[List all owning capabilities and parent requirements in the upstream chain with links]
- [Parent Capability 1](path/to/file.md#parent-1)
- [Parent Requirement 1](path/to/file.md#parent-2)

**Verified By:**
[List all verifications that verify this element]
- [Verification 1](path/to/file.md#verification-1)
- [Verification 2](path/to/file.md#verification-2)

**Satisfied By:**
[List all code implementations that satisfy this requirement]
- [Implementation 1](path/to/file.md#impl-1)
- [Implementation 2](path/to/file.md#impl-2)

**Refined By:**
[List all contract elements owned by this element]
- [Specification 1](path/to/file.md#spec-1)
- [Constraint 1](path/to/file.md#constraint-1)

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
- **Length**: As long as needed - simple capabilities might be 1-2 pages, complex capabilities might be 5-10 pages

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
