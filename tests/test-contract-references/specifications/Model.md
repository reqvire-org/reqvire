# Elements

### Contract Provider

#### Metadata
  * type: capability
---

### Contract Owner

The system SHALL provide the error-response contract.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Error Response Specification](#error-response-specification)
  * specify: [Contract Provider](#contract-provider)
---

### Error Response Specification

Error responses contain a code and message.

#### Metadata
  * type: specification
---

### Other Contract Owner

The system SHALL provide the other contract.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Other Specification](#other-specification)
  * specify: [Contract Provider](#contract-provider)
---

### Other Specification

Other contract content.

#### Metadata
  * type: specification
---

### Contract Consumer

#### Metadata
  * type: capability
---

### Documentation

The system SHALL document error responses.

#### Metadata
  * type: requirement

#### Contract References
  * [Error Response Specification](#error-response-specification)

#### Relations
  * satisfiedBy: [documentation.txt](../evidence/documentation.txt)
  * specify: [Contract Consumer](#contract-consumer)
---

### Endpoint Implementation

The system SHALL implement endpoint errors.

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [endpoint.txt](../evidence/endpoint.txt)
  * specify: [Contract Consumer](#contract-consumer)
---

### Documentation Verification Objective

#### Metadata
  * type: verification-objective
---

### Documentation Test

Check documented errors.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Documentation Verification Objective](#documentation-verification-objective)
  * satisfiedBy: [test.txt](../evidence/test.txt)
  * verify: [Documentation](#documentation)
---
