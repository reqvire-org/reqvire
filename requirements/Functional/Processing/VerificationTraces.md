# Elements

### Verification Upward Traceability

The system shall provide upward traceability from verifications to root requirements, showing the complete requirement hierarchy and verification coverage.

#### Details
When analyzing verification coverage, the system shall trace from verification elements upward through the requirement hierarchy to root requirements.

When generating trace reports, the system shall indicate which requirements are directly verified versus transitively covered.

When detecting model quality issues, the system shall identify redundant verification relations where a verification directly verifies both a leaf requirement and its ancestor.

#### Metadata
  * type: user-requirement

#### Attachments
  * [Traceability Reporting Specification](../../Refinements.md#traceability-reporting-specification)

#### Relations
  * derive: [Verification Roll-up Strategy](#verification-roll-up-strategy)
  * derive: [Verification Trace Builder](#verification-trace-builder)
  * derivedFrom: [Verification Traceability](../../UserStories.md#verification-traceability)
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

### Verification Trace Builder

The system shall provide functionality to build upward trace trees from verification elements by traversing all upward parent relations to reach root requirements, merging all paths into a single tree structure with marked directly-verified requirements.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Verification Upward Traceability](#verification-upward-traceability)
  * refinedBy: [Verification Trace Tree Construction](Specifications.md#verification-trace-tree-construction)
---
