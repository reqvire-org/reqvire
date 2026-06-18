# Elements



### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Test Capability Test Search All Capabilities Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

This document contains correctly formatted relations to existing targets, to verify validation passes.

This is page frontmatter content that should appear in the summary.

```mermaid
graph TD;
    A[Start] --> B[Process];
    B --> C[End];
```

Additional page content to test mermaid diagram inclusion in page summaries.

### Root Capability

Top-level capability used as hierarchy parent for system requirements in this fixture.

#### Metadata
* type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-search-all-capabilities-specifications-requirements-md)
---

### Verification of Standard Relations

#### Metadata
* type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
* verify: #requirement-with-valid-standard-relations
* verify: #requirement-with-valid-markdown-relations

---

### Requirement with Valid Standard Relations

This requirement has valid relations to existing files, using standard format.

#### Metadata
* type: requirement

#### Relations
* derivedFrom: Requirements.md#requirement-with-valid-markdown-relations
* satisfiedBy: DesignSpecifications/SampleDSD.md
* verifiedBy: [Verification of Standard Relations](#verification-of-standard-relations)
* verifiedBy: #search-command-verification

---

### Requirement with Valid Markdown Relations

This requirement has valid relations to existing files, using markdown link format.

#### Metadata
* type: requirement

#### Relations
* derivedFrom: #root-capability
* satisfiedBy: [./DesignSpecifications/SampleDSD.md](./DesignSpecifications/SampleDSD.md)
* verifiedBy: [Verification of Standard Relations](#verification-of-standard-relations)
* verifiedBy: #search-command-verification

---

### Requirement with DesignSpecifications Reference

This requirement specifically tests validation of relations to files in the DesignSpecifications folder.

#### Metadata
* type: requirement

#### Relations
* derivedFrom: #requirement-with-valid-standard-relations
* satisfiedBy: [Sample DSD](DesignSpecifications/SampleDSD.md)

---

### Requirement with Many Subsections

This requirement specifically tests validation of 'Other' subsections

#### Metadata
* type: requirement
#### Subsection 1

Some text of subsection 1

#### Subsection 2

Some text of subsection 2


#### Relations
  * specify: [Test Capability](#test-capability-test-search-all-capabilities-specifications-requirements-md)
---

### Search Command Verification

This test verifies that the search command supports all filter types and combines them with AND logic.

#### Details

##### Acceptance Criteria
- Search command accepts all documented filter flags
- Multiple filters combine with AND logic (results must match ALL filters)
- Invalid regex patterns produce clear error messages

##### Test Criteria
1. `reqvire search --json` returns valid JSON with all elements
2. `reqvire search --filter-type=capability` returns only capabilities
3. Element count with two filters is <= element count with one filter (additive behavior)

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: #requirement-with-valid-standard-relations
  * verify: #requirement-with-valid-markdown-relations

---

### Complex chars, element/name example

This requirement tests special characters in element names including commas and slashes.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-search-all-capabilities-specifications-requirements-md)
  * derivedFrom: #requirement-with-valid-standard-relations

---
