# Two-Pass Validation Architecture

## Overview
This document specifies the two-pass validation architecture that integrates validation into all commands that require the model, eliminating the need for a separate validation command.

---

### Two-Pass Validation Strategy

The system shall implement a two-pass validation strategy that separates element collection from relation validation, enabling complete error reporting while maintaining existing error behavior.

#### Details
The validation process shall be split into two distinct passes:

**Pass 1: Element Collection and Local Validation**
- Parse all markdown files
- Extract elements with metadata
- Apply automatic semantic normalization during parsing:
  - Convert non-link identifiers to proper markdown links with display text
  - Normalize absolute paths to relative paths for portable references
- Perform local validation (element uniqueness, identifier format, metadata syntax)
- Store elements in ElementRegistry
- Defer relation validation to Pass 2
- If errors are found, report them and exit the process

**Pass 2: Graph Construction and Relation Validation**
- Build GraphRegistry from ElementRegistry
- Validate all relations (target existence, type compatibility)
- Generate missing opposite relations
- Perform cross-component validation
- If errors are found, report them and exit the process

Both passes maintain the existing behavior where validation errors cause process termination with appropriate error reporting.

#### Relations
  * derivedFrom: [Validate Internal Consistency](../../UserRequirements.md#validate-internal-consistency)
  * derivedFrom: [Requirements Processing](../Storage/Configuration.md#requirements-processing)
  * satisfiedBy: [../../../core/src/model.rs](../../../core/src/model.rs)
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
  * derivedFrom: [Provide Validation Reports](../../UserRequirements.md#provide-validation-reports)
  * derivedFrom: [CLI Interface Structure](../UserInterface/CLI.md#cli-interface-structure)
  * satisfiedBy: [../../../cli/src/cli.rs](../../../cli/src/cli.rs)
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
  * derivedFrom: [Detailed Error Handling and Logging](../UserInterface/CLI.md#detailed-error-handling-and-logging)
  * derivedFrom: [Validation Report Generator](Reports.md#validation-report-generator)
  * satisfiedBy: [../../../core/src/error.rs](../../../core/src/error.rs)
  * satisfiedBy: [../../../core/src/model.rs](../../../core/src/model.rs)
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
  * derivedFrom: [Requirements Processing](../Storage/Configuration.md#requirements-processing)
  * satisfiedBy: [../../../core/src/graph_registry.rs](../../../core/src/graph_registry.rs)
  * satisfiedBy: [../../../core/src/model.rs](../../../core/src/model.rs)
  * verifiedBy: [../../Verifications/ValidationTests.md#requirements-files-search-and-detection-test](../../Verifications/ValidationTests.md#requirements-files-search-and-detection-test)
---

## Validation Capabilities
### Markdown Structure Validator

The system shall implement a markdown structure validator that enforces Reqvire's requirements for header levels, element structure, relation formatting, and other markdown-specific syntax rules, reporting violations with line numbers and suggested fixes.

#### Relations
  * derivedFrom: [Validate Markdown Structure](../../UserRequirements.md#validate-markdown-structure)
  * satisfiedBy: [../../../core/src/model.rs](../../../core/src/model.rs)
  * satisfiedBy: [../../../core/src/parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [../../Verifications/ValidationTests.md#invalid-relations-test](../../Verifications/ValidationTests.md#invalid-relations-test)
---

### Internal Consistency Validator

The system shall implement a consistency validator that verifies logical coherence within the model, including checking for circular dependencies, orphaned elements, and inconsistent relationship patterns, with detailed error reporting.

#### Relations
  * derivedFrom: [Validate Internal Consistency](../../UserRequirements.md#validate-internal-consistency)
  * satisfiedBy: [../../../core/src/model.rs](../../../core/src/model.rs)
  * satisfiedBy: [../../../core/src/parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [../../Verifications/ValidationTests.md#invalid-relations-test](../../Verifications/ValidationTests.md#invalid-relations-test)
---

### Cross-Component Dependency Validator

The system shall implement a specialized validator that analyzes dependencies across different model components, ensuring proper alignment between architectural layers, requirement levels, and verification elements.

#### Relations
  * derivedFrom: [Validate Cross-Component Dependencies](../../UserRequirements.md#validate-cross-component-dependencies)
  * satisfiedBy: [../../../core/src/model.rs](../../../core/src/model.rs)
  * satisfiedBy: [../../../core/src/parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [../../Verifications/ValidationTests.md#invalid-relations-test](../../Verifications/ValidationTests.md#invalid-relations-test)
---

### Relation Element Type Validator

The system shall implement validation that verifies relation endpoints have appropriate element types based on the relation type.

#### Details
- For `verifiedBy`/`verify` relations, validate that one endpoint is a requirement element and the other is a verification element
- For `satisfiedBy`/`satisfy` relations, validate that one endpoint is a requirement or test-verification element and the other is an implementation element
- For verification elements with `satisfiedBy` relations, validate that only test-verification elements may use satisfiedBy (other verification types should not have satisfiedBy relations)
- `trace` relations are always allowed for any verification type
- Relations should only connect elements of appropriate types based on the RelationTypesRegistry definition
- Warnings should be issued when relation endpoints have incompatible element types

#### Relations
  * derivedFrom: [Validate Relation Types](../../UserRequirements.md#validate-relation-types)
  * derivedFrom: [Relation Types and behaviors](../../SpecificationsRequirements.md#relation-types-and-behaviors)
  * satisfiedBy: [../../../core/src/model.rs](../../../core/src/model.rs)
  * satisfiedBy: [../../../core/src/parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [../../Verifications/ValidationTests.md#invalid-relations-test](../../Verifications/ValidationTests.md#invalid-relations-test)
---

### Relation Type Validation

The system shall validate relation types against a defined vocabulary and provide clear error messages for unsupported relation types, including suggestions for the correct relation types.

#### Relations
  * derivedFrom: [Enhanced Validation Error Reporting](../../UserRequirements.md#enhanced-validation-error-reporting)
  * satisfiedBy: [../../../core/src/relation.rs](../../../core/src/relation.rs)
---

### Excluded File Relation Validation

The system shall properly validate relations targeting files matching excluded filename patterns, enabling references to excluded files while still respecting their exclusion from processing and formatting operations.

#### Details
The validation process for excluded files:
1. Files matching excluded patterns are registered in the element registry for relation validation only
2. Internal elements within excluded files are not processed or validated

#### Relations
  * derivedFrom: [File Pattern Exclusion for Format](Formatting.md#file-pattern-exclusion-for-format)
  * satisfiedBy: [../../../core/src/parser.rs](../../../core/src/parser.rs)
---