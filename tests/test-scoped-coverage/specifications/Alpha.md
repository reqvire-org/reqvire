# Elements

### Alpha Local

The system SHALL provide a local alpha service.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Alpha Root](Capabilities.md#alpha-root)
---

### Alpha Contract Owner

The system SHALL define the exchange format for alpha data.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Alpha Left](Capabilities.md#alpha-left)
  * definedBy: [Exchange Format](#exchange-format)
---

### Exchange Format

An exchange record contains a nonempty text value.

#### Metadata
  * type: specification
---

### Alpha Parent

The system SHALL provide the shared alpha service.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Shared Branch](Capabilities.md#shared-branch)
  * definedBy: [Shared Service Format](#shared-service-format)
---

### Shared Service Format

A shared service response contains a status value.

#### Metadata
  * type: specification
---

### Alpha Middle

The system SHALL process shared alpha requests.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Alpha Parent](#alpha-parent)
---

### Alpha Implemented

The system SHALL return a status for a shared alpha request.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Alpha Middle](#alpha-middle)
  * satisfiedBy: [Alpha implementation](../evidence/alpha.txt)
---

### Alpha Gap

The system SHALL record shared alpha failures.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Alpha Parent](#alpha-parent)
---
