# Elements

### Test Feature

A feature for type validation error tests.

#### Metadata
  * type: feature

#### Relations
  * specifiedBy: [Test System Requirement](#test-system-requirement)
  * refinedBy: [Test Source Refinement](#test-source-refinement)

#### Attachments
  * [Test Ontology](#test-ontology)
---

### Test System Requirement

A system requirement specified by the feature.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Test Feature](#test-feature)
  * refinedBy: [Test Semantic Contract](#test-semantic-contract)
---

### Test State Refinement

A state refinement for positive type validation.

#### Metadata
  * type: state

#### Relations
  * refine: [Test System Requirement](#test-system-requirement)
---

### Test Input Output Refinement

An input-output refinement for positive type validation.

#### Metadata
  * type: input-output

#### Relations
  * refine: [Test System Requirement](#test-system-requirement)
---

### Test Formal Proof Verification

A formal proof verification for positive type validation.

#### Metadata
  * type: formal-proof-verification

#### Relations
  * verify: [Test System Requirement](#test-system-requirement)
  * satisfiedBy: [proof-report.txt](proof-report.txt)
---

### Test Source Refinement

A source refinement for positive type validation.

#### Metadata
  * type: source

#### Relations
  * refine: [Test Feature](#test-feature)
---

### Test Semantic Contract

A semantic contract for positive type validation.

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Test System Requirement](#test-system-requirement)

#### Shapes
```turtle
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

#### Ontology
```turtle
@prefix reqvire: <urn:reqvire:test#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

reqvire:TypeValidationContract a owl:Class .
```
---
