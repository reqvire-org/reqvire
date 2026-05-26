# Elements

### Product Capability

The top product capability for collect traversal.

#### Metadata
  * type: capability

#### Relations
  * derive: [Child Capability](#child-capability)
---

### Child Capability

The child product capability that specifies the root requirement.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Product Capability](#product-capability)
  * specifiedBy: [Root Requirement](#root-requirement)

#### Attachments
  * [Collect Ontology](#collect-ontology)
---

### Collect Ontology

Ontology content for collect traversal.

#### Metadata
  * type: ontology

#### Ontology
```turtle
@prefix ex: <urn:reqvire:test:collect:> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:CollectContract a owl:Class .
```
---

### Root Requirement

The root requirement for testing content collection.

#### Details
This is the top-level requirement that has no derivedFrom relations.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Child Capability](#child-capability)
---

### Mid-Level Requirement

The mid-level requirement derives from the root.

#### Details
This requirement sits in the middle of the hierarchy.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Root Requirement](#root-requirement)
---

### Leaf Requirement

The leaf requirement at the bottom of the hierarchy.

#### Details
This is the leaf requirement that derives from the mid-level.

#### Attachments
  * [Design Doc Specification](DesignDoc.md#design-doc-specification)

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Mid-Level Requirement](#mid-level-requirement)
---

### Design Owner Requirement

Owner requirement for design refinement specification.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Design Capability](#design-capability)
  * refinedBy: [Design Doc Specification](DesignDoc.md#design-doc-specification)
---

### Design Capability

Capability that owns reusable design documentation.

#### Metadata
  * type: capability

#### Relations
  * specifiedBy: [Design Owner Requirement](#design-owner-requirement)
---

### Test Verification

A verification element to test error handling.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Leaf Requirement](#leaf-requirement)
---
