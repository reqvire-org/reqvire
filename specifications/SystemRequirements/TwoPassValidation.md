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
  * derivedFrom: [Validate Internal Consistency](../UserRequirements.md#validate-internal-consistency)
  * derivedFrom: [Requirements Processing](Requirements.md#requirements-processing)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

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
- lint: Fixes markdown formatting without validation
- shell: Interactive mode with optional validation

Commands in the first group shall automatically run the two-pass validation and exit if any errors are found. Commands in the second group shall skip validation to allow operation on potentially invalid documents.

#### Relations
  * derivedFrom: [Provide Validation Reports](../UserRequirements.md#provide-validation-reports)
  * derivedFrom: [CLI Interface Structure](Requirements.md#cli-interface-structure)
  * satisfiedBy: [cli.rs](../../cli/src/cli.rs)

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
  * derivedFrom: [Detailed Error Handling and Logging](Requirements.md#detailed-error-handling-and-logging)
  * derivedFrom: [Validation Report Generator](Requirements.md#validation-report-generator)
  * satisfiedBy: [error.rs](../../core/src/error.rs)
  * satisfiedBy: [model.rs](../../core/src/model.rs)

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
  * derivedFrom: [Requirements Processing](Requirements.md#requirements-processing)
  * satisfiedBy: [graph_registry.rs](../../core/src/graph_registry.rs)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * verifiedBy: [Requirements Files Search and Detection Test](../Verifications/ValidationTests.md#requirements-files-search-and-detection-test)

---
