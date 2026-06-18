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
  * constrainedBy: [Verification Coverage Rollup Shape](../../Ontologies/Verification.md#verification-coverage-rollup-shape)
  * constrainedBy: [Verification Target and Evidence Shape](../../Ontologies/Verification.md#verification-target-and-evidence-shape)
  * derive: [Verification Roll-up Strategy](#verification-roll-up-strategy)
  * derive: [Verification Trace Builder](#verification-trace-builder)
  * specify: [Verification Traceability](../VerificationFeature.md#verification-traceability)
---

### Verification Roll-up Strategy

The system shall implement a verification roll-up strategy where parent requirements are considered verified based on the verification status of their child requirements.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Verification Roll-up Specification](Specifications.md#verification-roll-up-specification)
  * derivedFrom: [Verification Upward Traceability](#verification-upward-traceability)
  * satisfiedBy: [report_coverage.rs](../../../core/src/report_coverage.rs)
---

### Verification Trace Builder

The system shall provide functionality to build upward trace trees from verification elements by traversing all upward parent relations to reach owning capability roots, merging all paths into a single tree structure with marked directly verified capabilities or requirements.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Verification Trace Tree Construction](Specifications.md#verification-trace-tree-construction)
  * derivedFrom: [Verification Upward Traceability](#verification-upward-traceability)
---

