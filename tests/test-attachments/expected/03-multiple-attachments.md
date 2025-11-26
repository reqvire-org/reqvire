# Elements

### System Requirements

Top-level container for test requirements.

#### Metadata
  * type: user-requirement

#### Relations
  * derive: [Performance Requirement](#performance-requirement)
  * derive: [No Attachments Requirement](#no-attachments-requirement)
---


### Performance Requirement

The system shall meet defined performance criteria.

#### Details
This requirement has an attached SLA document.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
  * derive: [Implementation Detail](#implementation-detail)

#### Attachments
* [docs/SLA.txt](docs/SLA.txt)
* [docs/benchmarks.txt](docs/benchmarks.txt)
---

### Implementation Detail

The implementation shall follow the SLA guidelines.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Performance Requirement](#performance-requirement)
---

### No Attachments Requirement

This requirement has no attachments.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
---

### Test Constraint Element

This is a constraint Refinement element for testing element attachments.

#### Details
This constraint defines limits on system behavior.

#### Metadata
  * type: constraint
---

### Test Behavior Element

This is a behavior Refinement element for testing element attachments.

#### Details
This behavior defines expected system operation.

#### Metadata
  * type: behavior
---

