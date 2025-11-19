# Validating Structures

### Enhanced Validation Error Reporting

The system shall provide comprehensive validation messages that include file paths and line numbers when available, to help users quickly locate and fix model integrity and structure issues in their MBSE specifications.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
---

### Model Formatting

The system shall provide formatting capabilities to normalize and standardize MBSE models for consistency and readability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
---

### Model Linting

The system shall provide model linting capabilities to analyze model quality and detect issues in requirements relations.

#### Details
The linting capability should:
- Identify issues with model relations that may need attention
- Distinguish between issues that can be automatically fixed and those requiring human judgment
- Provide clear categorization of findings to help users understand what actions are needed
- Support both reporting mode and automatic fixing mode
- Allow filtering to focus on specific categories of issues

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Deterministic Output for All Generated Content](ReqvireTool/ValidationAndReporting/Reports.md#deterministic-output-for-all-generated-content)
---

### Formatting Output

The system shall display formatting changes suggestion in similar manner as git diffs.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
---

### Replace Absolute Links with Relative Links

The system shall replace absolute links with relative links, where applicable and contextually appropriate, to conform to repository standards and enhance portability.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * verifiedBy: [Format Command Requirements Verification](Verifications/Misc.md#format-command-requirements-verification)
---

### Format Consistency Enforcement

The system shall provide formatting capability to ensure consistent formatting in requirements documents.

#### Details
  * Trimming excess whitespace after element names and relation identifiers
  * Normalizing to exactly two newlines before subsections (e.g., "#### Details")
  * Automatically inserting separator lines ("---") between elements if not already present
  * Normalizing consecutive separators to single separators
  * Ensuring consistent indentation in relation lists (2-space format)
  * Normalizing relation entries to proper 2-space indentation format
  * Displaying changes with sequential line numbering that reflects final file positions
  * Providing context lines with proper line number continuity

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Model Formatting](#model-formatting)
  * derivedFrom: [Align with Industry Standards](Mission.md#align-with-industry-standards)
---

### Documentation Index for HTML Export

The system shall automatically generate an index document during HTML export that contains a structured summary of all specification documents and folders, serving as the primary entry point (index.html) for HTML documentation.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [HTML Navigation Bar](Export.md#html-navigation-bar)
---

### Validate Markdown Structure

The system shall validate the Markdown structure of MBSE documentation to ensure compliance with formatting standards.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](Mission.md#align-with-industry-standards)
---

### Validate Filesystem Structure

The system shall validate the organization of files and folders in the repository to ensure consistency with the MBSE methodology.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
---

### Validate Internal Consistency

The system shall check the internal consistency of the MBSE model, ensuring that relationships and elements align correctly.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](Mission.md#align-with-industry-standards)
---

### Validate Cross-Component Dependencies

The system shall validate dependencies across different components of the MBSE model to identify mismatches or gaps.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](Mission.md#align-with-industry-standards)
---

### Validate Relation Types

The system shall validate relation types and allow only supported types.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](Mission.md#align-with-industry-standards)
  * derivedFrom: [Relation Types and behaviors](Structure/SpecificationsRequirements.md#relation-types-and-behaviors)
---
