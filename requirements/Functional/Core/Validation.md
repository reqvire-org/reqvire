# Elements

### Excluded File Relation Validation

The system shall properly validate relations targeting files matching excluded filename patterns, enabling references to excluded files while still respecting their exclusion from processing and formatting operations.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [File Pattern Exclusion for Format](../Operations/Formatting.md#file-pattern-exclusion-for-format)
  * refinedBy: [Excluded File Relation Validation Refinement Specification](Specifications.md#excluded-file-relation-validation-refinement-specification)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Unstructured Documents Test](Verifications/ValidationVerifications.md#unstructured-documents-test)
---

### Validate Cross-Component Dependencies

The system shall validate dependencies across different components of the System model to identify mismatches or gaps.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Cross-Component Dependency Validator](#cross-component-dependency-validator)
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
---

### Cross-Component Dependency Validator

The system shall implement a specialized validator that analyzes dependencies across different model components, ensuring proper alignment between architectural layers, requirement levels, and verification elements.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Cross-Component Dependencies](#validate-cross-component-dependencies)
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

### Validate Internal Consistency

The system shall check the internal consistency of the system model, ensuring that relationships and elements align correctly.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Internal Consistency Validator](#internal-consistency-validator)
  * derive: [Two-Pass Validation Strategy](#two-pass-validation-strategy)
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
---

### Attachment Target Validation

The system shall validate attachment targets and reject invalid attachment references during model validation.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * refinedBy: [Attachment Target Validation Refinement Specification](Specifications.md#attachment-target-validation-refinement-specification)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Attachments Subsection Parsing Verification](Verifications/AttachmentsVerifications.md#attachments-subsection-parsing-verification)
  * verifiedBy: [Attachments Validation Verification](Verifications/AttachmentsVerifications.md#attachments-validation-verification)
---

### Attachment Scope Validation

The system shall validate attachment scope constraints and report validation errors for violations.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Attachment Hierarchical Independence Constraint](Constraints.md#attachment-hierarchical-independence-constraint)
  * [Attachment Satisfied Refinement Constraint](Constraints.md#attachment-satisfied-refinement-constraint)

#### Relations
  * derivedFrom: [Attachment Target Validation](#attachment-target-validation)
  * refinedBy: [Attachment Scope Validation Refinement Specification](Specifications.md#attachment-scope-validation-refinement-specification)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Attachment Scope Constraints Test](Verifications/AttachmentsVerifications.md#attachment-scope-constraints-test)
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

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * refinedBy: [Requirements Processing Specification](Specifications.md#requirements-processing-specification)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Requirements Files Search and Detection Test](Verifications/ValidationVerifications.md#requirements-files-search-and-detection-test)
---

### Integrated Validation

The system shall automatically perform validation when any command requires the parsed model, eliminating the need for a separate validate command.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [Two-Pass Validation Behavior](Behaviors.md#two-pass-validation-behavior)

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * refinedBy: [Integrated Validation Refinement Specification](Specifications.md#integrated-validation-refinement-specification)
  * satisfiedBy: [cli.rs](../../../cli/src/cli.rs)
---

### Internal Consistency Validator

The system shall implement a consistency validator that verifies logical coherence within the model, including checking for circular dependencies, orphaned elements, inconsistent relationship patterns, and element name uniqueness, with detailed error reporting.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * refinedBy: [Internal Consistency Validator Refinement Specification](Specifications.md#internal-consistency-validator-refinement-specification)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
---

### Cross-Section Duplicate Validation

The system shall detect when the same target appears in both the Relations and Attachments subsections of an element, treating this as a validation error.

#### Details
A constraint defines the detailed rules for cross-section duplicate detection.

This applies to all target types: element identifiers and file paths.

Within-section duplicates (same entry repeated within Relations OR within Attachments) are formatting issues handled by the format operation, not validation errors.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Internal Consistency Validator](#internal-consistency-validator)
  * refinedBy: [Cross-Section Duplicate Constraint](Constraints.md#cross-section-duplicate-constraint)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Cross-Section Duplicate Validation Test](Verifications/ValidationVerifications.md#cross-section-duplicate-validation-test)
---

### Relation Type Validation

The system shall validate relation types against a defined vocabulary and provide clear error messages for unsupported relation types, including suggestions for the correct relation types.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
  * verifiedBy: [Same-File Fragment Relations Test](Verifications/ValidationVerifications.md#same-file-fragment-relations-test)
---

### Single Root Hierarchy Ownership

The system shall enforce that each hierarchy element resolves to exactly one top root `user-requirement` through hierarchical relations.

#### Details
Validation details shall follow the associated hierarchy ownership constraint.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * refinedBy: [Single Root Hierarchy Ownership Constraint](Constraints.md#single-root-hierarchy-ownership-constraint)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * verifiedBy: [Single Root Hierarchy Ownership Validation Test](Verifications/ValidationVerifications.md#single-root-hierarchy-ownership-validation-test)
---

### Two-Pass Validation Strategy

The system shall execute model validation in two phases: element collection and graph validation.

#### Details
The system shall define two-pass validation behavior.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * refinedBy: [Two-Pass Validation Behavior](Behaviors.md#two-pass-validation-behavior)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

### Type Validation Error Requirement

The system shall display all valid type options when type validation fails.

#### Details
- Invalid element types shall show list of valid element types including custom type pattern
- Invalid relation types shall show list of valid relation types
- Element type list format: "type1, type2, ... For custom types use: other-TYPENAME"
- Relation type list format: "type1, type2, ..." (alphabetically sorted)

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * refinedBy: [Type Validation Error Behavior](Behaviors.md#type-validation-error-behavior)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
  * verifiedBy: [Type Validation Errors Test](Verifications/ValidationVerifications.md#type-validation-errors-test)
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

#### Metadata
  * type: requirement

#### Attachments
  * [Error Message Format Specification](../Output/Specifications.md#error-message-format-specification)

#### Relations
  * derivedFrom: [Validate Internal Consistency](#validate-internal-consistency)
  * refinedBy: [Validation Error Reporting Behavior](Behaviors.md#validation-error-reporting-behavior)
  * satisfiedBy: [error.rs](../../../core/src/error.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
---

### Validate Markdown Structure

The system shall validate the Markdown structure of system model to ensure compliance with formatting standards.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Markdown Structure Validator](#markdown-structure-validator)
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
---

### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces Reqvire's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Markdown Structure](#validate-markdown-structure)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Invalid Header Structure Test](Verifications/ValidationVerifications.md#invalid-header-structure-test)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
---

### Validate Relation Types

The system shall validate relation types and allow only supported types.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Relation Element Type Validator](#relation-element-type-validator)
  * derivedFrom: [Validating Structures](../../UserStories.md#validating-structures)
---

### Relation Element Type Validator

The system shall implement validation that verifies relation endpoints have appropriate element types based on the relation type, following the Element Type Relation Compatibility matrix.

#### Details
Implementation details shall follow the associated refinement specifications.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Validate Relation Types](#validate-relation-types)
  * refinedBy: [Relation Element Type Validator Refinement Specification](Specifications.md#relation-element-type-validator-refinement-specification)
  * satisfiedBy: [graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Document Refinement Validation Test](Verifications/ValidationVerifications.md#document-refinement-validation-test)
  * verifiedBy: [Element Type Relation Compatibility Test](Verifications/ValidationVerifications.md#element-type-relation-compatibility-test)
  * verifiedBy: [Invalid Relations Test](Verifications/ValidationVerifications.md#invalid-relations-test)
---
