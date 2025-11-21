# Test Requirements

## Root

### System Requirements

Top-level container for test requirements.

#### Metadata
  * type: root-requirement

#### Relations
  * derive: [Performance Requirement](#performance-requirement)
  * derive: [No Attachments Requirement](#no-attachments-requirement)
---

## Requirements

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

