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

### Refinement Target Requirement

A separate requirement that refinements can satisfy (outside main hierarchy).

#### Metadata
  * type: user-requirement

---

### Test Constraint Element

This is a constraint Refinement element for testing element attachments.

#### Details
This constraint defines limits on system behavior.

#### Metadata
  * type: constraint

#### Relations
  * satisfy: [Refinement Target Requirement](#refinement-target-requirement)
---

### Test Behavior Element

This is a behavior Refinement element for testing element attachments.

#### Details
This behavior defines expected system operation.

#### Metadata
  * type: behavior

#### Relations
  * satisfy: [Refinement Target Requirement](#refinement-target-requirement)
---

