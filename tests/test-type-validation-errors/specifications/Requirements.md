# Elements


### Verification Objective

Verification objective for concrete verification fixtures in this test model.

#### Metadata
  * type: verification-objective
---
### Test Capability

A capability for type validation error tests.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Test System Requirement](#test-system-requirement)

---

### Test System Requirement

A system requirement specified by the capability.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Capability](#test-capability)
  * definedBy: [Test Source Contract](#test-source-contract)
  * constrainedBy: [Test Semantic Contract](#test-semantic-contract)
---

### Test State Contract

A state contract for positive type validation.

#### Metadata
  * type: state

#### Relations
  * define: [Test System Requirement](#test-system-requirement)
---

### Test Input Output Contract

An input-output contract for positive type validation.

#### Metadata
  * type: input-output

#### Relations
  * define: [Test System Requirement](#test-system-requirement)
---

### Test Formal Proof Verification

A formal proof verification for positive type validation.

#### Metadata
  * type: formal-proof-verification

#### Relations
  * derivedFrom: [Verification Objective](#verification-objective)
  * verify: [Test System Requirement](#test-system-requirement)
  * satisfiedBy: [proof-report.txt](proof-report.txt)
---

### Test Source Contract

A source contract for positive type validation.

#### Metadata
  * type: source

#### Relations
  * define: [Test System Requirement](#test-system-requirement)
---

### Test Semantic Contract

A semantic contract for positive type validation.

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Test System Requirement](#test-system-requirement)
  * use: [Test Ontology](#test-ontology)

#### Shapes
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix reqvire: <urn:reqvire:test#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

reqvire:TypeValidationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:TypeValidationContract .
```
---

### Test Ontology

An ontology for positive type validation.

#### Metadata
  * type: ontology
  * ontology_base: https://example.test/ontology
  * ontology_prefix: testonto

#### Ontology
```turtle
@prefix testonto: <https://example.test/ontology#> .
@prefix reqvire: <urn:reqvire:test#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<https://example.test/ontology> a owl:Ontology .
reqvire:TypeValidationContract a owl:Class .
```
---
