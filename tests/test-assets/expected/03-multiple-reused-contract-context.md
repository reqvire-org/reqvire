# Elements

### System Requirements

Top-level container for test requirements.

#### Metadata
  * type: capability

#### Relations
  * derive: [Performance Requirement](#performance-requirement)
  * derive: [No Reused Contract Context Requirement](#no-reused-contract-context-requirement)
---


### Performance Requirement

The system shall meet defined performance criteria.

#### Details
This requirement has an reused SLA document.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
  * derive: [Implementation Detail](#implementation-detail)

#### Reused Contract Context
* [../docs/SLA.txt](../docs/SLA.txt)
* [../docs/benchmarks.txt](../docs/benchmarks.txt)
---

### Implementation Detail

The implementation shall follow the SLA guidelines.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Performance Requirement](#performance-requirement)
---

### No Reused Contract Context Requirement

This requirement has no reused_contract_context.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [System Requirements](#system-requirements)
---

### Contract Target Requirement

A separate requirement that owns contracts (outside main hierarchy).

#### Metadata
  * type: capability

#### Relations
  * definedBy: [Test Constraint Element](#test-constraint-element)
  * definedBy: [Test Behavior Element](#test-behavior-element)
---

### Test Constraint Element

This is a constraint Contract element for testing element reused_contract_context.

#### Details
This constraint defines limits on system behavior.

#### Metadata
  * type: constraint

---

### Test Behavior Element

This is a behavior Contract element for testing element reused_contract_context.

#### Details
This behavior defines expected system operation.

#### Metadata
  * type: behavior

---

