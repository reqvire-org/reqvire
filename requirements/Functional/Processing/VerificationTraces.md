# Elements

### Verification Upward Traceability

The system shall provide upward traceability from verifications to owning capability roots, showing directly verified capabilities, the complete requirement hierarchy, capability context, and verification coverage.

#### Details
When analyzing verification coverage, the system shall trace from verification elements upward through directly verified capabilities or through the requirement hierarchy and owning capability context.

When generating trace reports, the system shall indicate which capabilities or requirements are directly verified versus transitively covered.

When detecting model quality issues, the system shall identify redundant verification relations where a verification directly verifies both a leaf requirement and its ancestor.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Verification Element Semantic Contract](#verification-element-semantic-contract)
  * derive: [Verification Roll-up Strategy](#verification-roll-up-strategy)
  * derive: [Verification Rollup Semantic Contract](#verification-rollup-semantic-contract)
  * derive: [Verification Trace Builder](#verification-trace-builder)
  * specify: [Verification Traceability](../../Capabilities/Verification.md#verification-traceability)
---

### Verification Element Semantic Contract

The system shall define SHACL constraints for verification elements, verified capabilities or requirements, and evidence-backed verification satisfaction.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Verification Upward Traceability](#verification-upward-traceability)
---

### Verification Roll-up Strategy

The system shall implement a verification roll-up strategy where parent requirements are considered verified based on the verification status of their child requirements.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Verification Upward Traceability](#verification-upward-traceability)
  * refinedBy: [Verification Roll-up Specification](Specifications.md#verification-roll-up-specification)
  * satisfiedBy: [report_coverage.rs](../../../core/src/report_coverage.rs)
---

### Verification Rollup Semantic Contract

The system shall define SHACL constraints for verification rollup records, capability coverage records, requirement coverage records, and coverage state values.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Verification Upward Traceability](#verification-upward-traceability)
---

### Verification Trace Builder

The system shall provide functionality to build upward trace trees from verification elements by traversing all upward parent relations to reach owning capability roots, merging all paths into a single tree structure with marked directly verified capabilities or requirements.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Verification Upward Traceability](#verification-upward-traceability)
  * refinedBy: [Verification Trace Tree Construction](Specifications.md#verification-trace-tree-construction)
---
