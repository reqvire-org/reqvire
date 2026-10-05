# Elements

### Service Verification Objective

This objective organizes verification of the alpha and beta services.

#### Metadata
  * type: verification-objective
---

### Shared Check

This test checks service responses across both roots.

#### Details
Check shared alpha failure records, shared alpha status, and beta exchange consumption.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Service Verification Objective](#service-verification-objective)
  * verify: [Alpha Gap](Alpha.md#alpha-gap)
  * verify: [Alpha Implemented](Alpha.md#alpha-implemented)
  * verify: [Beta Consumer](Beta.md#beta-consumer)
  * satisfiedBy: [Service test evidence](../evidence/check.txt)
---

### Gap Check

This test checks alpha failure recording.

#### Details
Check that rejected shared requests produce a failure record.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Service Verification Objective](#service-verification-objective)
  * verify: [Alpha Gap](Alpha.md#alpha-gap)
---

### Parent Review

This inspection checks the shared service definition.

#### Details
Inspect the shared service response format for an explicit status value.

#### Metadata
  * type: inspection-verification

#### Relations
  * derivedFrom: [Service Verification Objective](#service-verification-objective)
  * verify: [Alpha Parent](Alpha.md#alpha-parent)
---

### Shared Proof

This proof checks exchange validity and rejected-record handling across roots.

#### Details
Establish that an accepted exchange value is nonempty and rejected beta records are recorded.

#### Metadata
  * type: formal-proof-verification

#### Relations
  * derivedFrom: [Service Verification Objective](#service-verification-objective)
  * verify: [Alpha Contract Owner](Alpha.md#alpha-contract-owner)
  * verify: [Beta Gap](Beta.md#beta-gap)
  * satisfiedBy: [Exchange proof evidence](../evidence/proof.txt)
---

### Middle Demonstration

This demonstration checks shared request processing.

#### Details
Demonstrate processing one shared alpha request.

#### Metadata
  * type: demonstration-verification

#### Relations
  * derivedFrom: [Service Verification Objective](#service-verification-objective)
  * verify: [Alpha Middle](Alpha.md#alpha-middle)
---

### Beta Analysis

This analysis checks rejection recording.

#### Details
Analyze the outcomes of invalid beta records.

#### Metadata
  * type: analysis-verification

#### Relations
  * derivedFrom: [Service Verification Objective](#service-verification-objective)
  * verify: [Beta Gap](Beta.md#beta-gap)
---

### Orphan Check

This test is intentionally without a requirement target to exercise orphan diagnostics.

#### Details
The fixture preserves an untargeted test as whole-model diagnostic input.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Service Verification Objective](#service-verification-objective)
---
