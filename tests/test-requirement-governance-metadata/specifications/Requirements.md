# Elements


### Test Capability Test Requirement Governance Metadata Specifications Requirements Md

Test capability root for migrated requirement fixtures.

#### Metadata
  * type: capability
---

### Root Requirement

The system shall provide a parent requirement with explicit governance metadata.

#### Metadata
  * type: requirement
  * status: approved
  * priority: high
  * risk: medium
  * owner: Platform Team

#### Relations
  * specify: [Test Capability](#test-capability-test-requirement-governance-metadata-specifications-requirements-md)
---

### Child Inherits Governance

The system shall provide a child requirement that inherits governance metadata.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Child Overrides Governance

The system shall provide a child requirement that overrides selected governance metadata.

#### Metadata
  * type: requirement
  * status: review
  * risk: critical

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Independent Requirement

The system shall provide a requirement that uses governance metadata defaults.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Default Governance Root](#default-governance-root)
---

### Default Governance Root

The system shall provide a root requirement without authored governance metadata.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability-test-requirement-governance-metadata-specifications-requirements-md)
---

### Refinement Contract

This refinement augments the root requirement and must not author governance metadata.

#### Metadata
  * type: specification

#### Relations
  * define: [Root Requirement](#root-requirement)
---
