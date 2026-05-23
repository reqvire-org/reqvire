# Elements

### System Requirements

Top-level container for test requirements.

#### Metadata
  * type: feature

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
* [Test Constraint Element](#test-constraint-element)
---

### Implementation Detail

The implementation shall follow the SLA guidelines.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Performance Requirement](#performance-requirement)

#### Attachments
* [../Test Constraint Element/data.txt](../Test Constraint Element/data.txt)
---

### No Attachments Requirement

This requirement has no attachments.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
---

### Refinement Target Requirement

A separate requirement that owns refinements (outside main hierarchy).

#### Metadata
  * type: feature

#### Relations
  * refinedBy: [Test Constraint Element](#test-constraint-element)
  * refinedBy: [Test Behavior Element](#test-behavior-element)

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

