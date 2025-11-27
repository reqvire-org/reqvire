# Elements

### Two-Pass Validation Strategy

The system shall execute model validation in two phases: element collection and graph validation.

#### Details
The system shall define two-pass validation behavior.

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * derivedFrom: [Requirements Processing](Configuration.md#requirements-processing)
  * satisfiedBy: [Two-Pass Validation Behavior](Refinements.md#two-pass-validation-behavior)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces Reqvire's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Relations
  * derivedFrom: [Validate Markdown Structure](#validate-markdown-structure)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
---

### Enhanced Validation Error Reporting

The system shall include context for resolution in validation errors.

#### Details
The system shall define validation error reporting behavior.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
  * satisfiedBy: [Validation Error Reporting Behavior](Refinements.md#validation-error-reporting-behavior)
  * satisfiedBy: [error.rs](../../../core/src/error.rs)
---

### Integrated Validation

The system shall automatically perform validation when any command requires the parsed model, eliminating the need for a separate validate command.

#### Details
Commands shall be categorized into two groups:

**Commands requiring validated model:**
- model-summary: Needs complete element and relation data
- change-impact: Requires valid relations for impact analysis
- traces: Needs validated relationships for traceability
- generate-index: Requires complete element registry
- generate-diagrams: Needs valid relations for visualization
- remove-diagrams: Operates on validated markdown structure
- coverage-report: Requires complete verification data

**Commands operating on raw files:**
- html: Converts markdown to HTML without parsing elements
- format: Fixes markdown formatting without validation
- shell: Interactive mode with optional validation

Commands in the first group shall automatically run the two-pass validation and exit if any errors are found. Commands in the second group shall skip validation to allow operation on potentially invalid documents.

#### Relations
  * derivedFrom: [Provide Validation Reports](../Output/Reporting.md#provide-validation-reports)
  * derivedFrom: [CLI Interface Structure](../../Interfaces/CLI.md#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
---

### Validate Markdown Structure

The system shall validate the Markdown structure of system model to ensure compliance with formatting standards.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](../../UserStories.md#align-with-industry-standards)
---

### Internal Consistency Validator

The system shall implement a consistency validator that verifies logical coherence within the model, including checking for circular dependencies, orphaned elements, inconsistent relationship patterns, and element name uniqueness, with detailed error reporting.

#### Details
The consistency validator shall verify:
- **Global Element Name Uniqueness**: Element names are globally unique across all files in the model
- **Duplicate Detection**: Detect and report when multiple elements in different files share the same name
- **Location Reporting**: Report both file locations where duplicate element names occur
- **Clear Error Messages**: Error messages clearly indicate that element names must be globally unique
- **Circular Dependencies**: Detect and report circular dependency chains in requirements
- **Orphaned Elements**: Identify elements without proper traceability connections
- **Inconsistent Patterns**: Detect relationship patterns that violate model constraints

Rationale: Element names serve as stable IDs for element identity, independent of file location. Global uniqueness is essential for proper element identification and change tracking across the model.

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * derivedFrom: [Element Identity Model](StructureAndParsing.md#element-identity-model)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
---

### Validate Filesystem Structure

The system shall validate the organization of files and folders in the repository to ensure consistency with the MBSE methodology.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
---

### Cross-Component Dependency Validator

The system shall implement a specialized validator that analyzes dependencies across different model components, ensuring proper alignment between architectural layers, requirement levels, and verification elements.

#### Relations
  * derivedFrom: [Validate Cross-Component Dependencies](#validate-cross-component-dependencies)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
---

### Validation Error Handling

The system shall maintain consistent error handling across both validation passes, collecting all errors within each pass before reporting.

#### Details
Error handling shall follow these principles:

1. **Complete pass execution**: Each pass runs to completion, collecting all errors found
2. **Aggregated reporting**: All errors from a pass are reported together
3. **Early termination**: Process exits after reporting errors from either pass
4. **Existing error format**: Error messages maintain the current format and structure
5. **Exit codes**: Non-zero exit codes indicate validation failures

This ensures users see all relevant errors at once rather than fixing issues one at a time.

#### Relations
  * derivedFrom: [Detailed Error Handling and Logging](../../Interfaces/CLI.md#detailed-error-handling-and-logging)
  * derivedFrom: [Validation Report Generator](../Output/Reporting.md#validation-report-generator)
  * satisfiedBy: [error.rs](../../../core/src/error.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

### Relation Element Type Validator

The system shall implement validation that verifies relation endpoints have appropriate element types based on the relation type, following the Element Type Relation Compatibility matrix.

#### Details
The validator enforces the constraints defined in the [Element Type Relation Compatibility](DesignDocuments/RelationTypes.md#element-type-relation-compatibility) specification:

- For `derivedFrom`/`derive` relations, validate that both source and target are requirement types (`requirement` or `user-requirement`)
- For `verifiedBy`/`verify` relations, validate that one endpoint is a requirement element and the other is a verification element
- For `satisfiedBy`/`satisfy` relations, validate that one endpoint is a requirement or test-verification element and the other is an implementation element
- For verification elements with `satisfiedBy` relations, validate that only test-verification elements may use satisfiedBy (other verification types should not have satisfiedBy relations)
- `trace` relations are always allowed for any non-refinement element type
- Refinement types (`constraint`, `behavior`, `specification`) cannot have Relations subsections
- Warnings should be issued when relation endpoints have incompatible element types

#### Relations
  * derivedFrom: [Validate Relation Types](#validate-relation-types)
  * derivedFrom: [Element Type Relation Compatibility](ModelManagement.md#element-type-relation-compatibility)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
  * verifiedBy: [Element Type Relation Compatibility Test](Verifications/ValidationVerifications.md#element-type-relation-compatibility-test)
---

### GraphRegistry as Primary Registry

The system shall enhance GraphRegistry to serve as the primary structure for relation operations and validation during Pass 2.

#### Details
The GraphRegistry shall be responsible for:

1. **Graph construction**: Building adjacency lists from ElementRegistry
2. **Relation validation**: Checking target existence and type compatibility
3. **Opposite generation**: Creating missing bidirectional relations
4. **Cycle detection**: Identifying circular dependencies
5. **Orphan detection**: Finding isolated elements
6. **Impact analysis**: Supporting change propagation queries

The GraphRegistry shall be constructed from the ElementRegistry after Pass 1 completes successfully.

#### Relations
  * derivedFrom: [Requirements Processing](Configuration.md#requirements-processing)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Requirements Files Search and Detection Test](Verifications/ValidationVerifications.md#requirements-files-search-and-detection-test)
---

### Validate Internal Consistency

The system shall check the internal consistency of the system model, ensuring that relationships and elements align correctly.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](../../UserStories.md#align-with-industry-standards)
---

### Validate Cross-Component Dependencies

The system shall validate dependencies across different components of the System model to identify mismatches or gaps.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](../../UserStories.md#align-with-industry-standards)
---

### Relation Type Validation

The system shall validate relation types against a defined vocabulary and provide clear error messages for unsupported relation types, including suggestions for the correct relation types.

#### Relations
  * derivedFrom: [Enhanced Validation Error Reporting](#enhanced-validation-error-reporting)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
---

### Excluded File Relation Validation

The system shall properly validate relations targeting files matching excluded filename patterns, enabling references to excluded files while still respecting their exclusion from processing and formatting operations.

#### Details
The validation process for excluded files:
1. Files matching excluded patterns are registered in the element registry for relation validation only
2. Internal elements within excluded files are not processed or validated

#### Relations
  * derivedFrom: [File Pattern Exclusion for Format](../Operations/Formatting.md#file-pattern-exclusion-for-format)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
---

### Validate Relation Types

The system shall validate relation types and allow only supported types.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
  * derivedFrom: [Align with Industry Standards](../../UserStories.md#align-with-industry-standards)
  * derivedFrom: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
---
