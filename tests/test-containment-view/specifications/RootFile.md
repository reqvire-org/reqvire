# Elements



### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Test Capability Test Containment View Specifications Rootfile Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Root Capability

The system shall provide comprehensive functionality.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-containment-view-specifications-rootfile-md)
### Root System Requirement

The system shall implement core capabilities.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Capability](#root-capability)

### Test Verification Element

This test verifies the root requirement.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Root System Requirement](#root-system-requirement)

### Analysis Verification Element

This analysis verifies performance metrics.

#### Metadata
  * type: analysis-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Root System Requirement](#root-system-requirement)

### Inspection Verification Element

This inspection verifies code quality.

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Root System Requirement](#root-system-requirement)

### Demonstration Verification Element

This demonstration verifies usability.

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Root System Requirement](#root-system-requirement)

### Custom Type Element

This is a custom element type for testing.

#### Metadata
  * type: other-design-document
