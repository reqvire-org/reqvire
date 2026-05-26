# Elements


### Test Capability Test Implementation Coverage Report Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Root Requirement

Top-level requirement used to exercise implementation coverage roll-up.

#### Metadata
  * type: requirement
#### Relations
  * specify: [Test Capability](#test-capability-test-implementation-coverage-report-specifications-requirements-md)
  * derive: [Direct Implemented](#direct-implemented)
  * derive: [Derived Parent](#derived-parent)
  * derive: [Contract Owner](#contract-owner)
---

### Direct Implemented

Requirement directly implemented in code.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
  * satisfiedBy: [src/direct.rs](src/direct.rs)
---

### Derived Parent

Parent requirement covered through a derived requirement implementation.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Derived Intermediate](#derived-intermediate)
  * derivedFrom: [Root Requirement](#root-requirement)
  * refinedBy: [Derived Parent Contract Specification](#derived-parent-contract-specification)
---

### Derived Intermediate

Intermediate descendant requirement in transitive child path.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Derived Child Implemented](#derived-child-implemented)
  * derivedFrom: [Derived Parent](#derived-parent)
---

### Derived Parent Contract Specification

Refinement contract owned by Derived Parent.

#### Metadata
  * type: specification

#### Relations
  * refine: [Derived Parent](#derived-parent)
---

### Derived Child Implemented

Derived requirement directly implemented in code.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Derived Intermediate](#derived-intermediate)
  * satisfiedBy: [src/derived_child.rs](src/derived_child.rs)
---

### Contract Owner

Requirement that owns a refinement contract and has no direct implementation.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
  * refinedBy: [Contract Specification](#contract-specification)
---

### Contract Specification

Refinement contract owned by Contract Owner.

#### Metadata
  * type: specification

#### Relations
  * refine: [Contract Owner](#contract-owner)
---

### Contract Consumer Implemented

Requirement that consumes contract specification and is directly implemented.

#### Metadata
  * type: requirement

#### Attachments
  * [Contract Specification](#contract-specification)

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
  * satisfiedBy: [src/contract_consumer.rs](src/contract_consumer.rs)
---

### Uncovered Requirement

Requirement intentionally left without implementation evidence.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---
