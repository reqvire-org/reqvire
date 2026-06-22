# Elements

### Sparse Concept Scheme

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.test/native-concepts
  * concept_prefix: concept
---

### Concept Taxonomy Parent

Parent concept used to verify generated SKOS taxonomy relations.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Sparse Concept Scheme](#sparse-concept-scheme)
  * narrower: [Detailed Concept](#detailed-concept)
---

### Concept Taxonomy Peer

Peer concept used to verify generated SKOS associative relations.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Sparse Concept Scheme](#sparse-concept-scheme)
  * related: [Detailed Concept](#detailed-concept)
---

### Detailed Concept

Detailed concept definition generated from the main element body.

#### Labels
  * altLabel: Detailed vocabulary term
  * hiddenLabel: Detaild Concept

#### Scope Note
Use this concept to verify Markdown-native SKOS concept payload extraction.

#### Examples
  * A concept author writes labels, examples, mappings, and taxonomy in Markdown.

#### Mappings
  * exactMatch: <https://external.example/concepts/DetailedConcept>
  * closeMatch: <https://external.example/concepts/Detail>

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Sparse Concept Scheme](#sparse-concept-scheme)
  * broader: [Concept Taxonomy Parent](#concept-taxonomy-parent)
  * related: [Concept Taxonomy Peer](#concept-taxonomy-peer)
---

### Sparse Concept

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Sparse Concept Scheme](#sparse-concept-scheme)
---

### Nested Sparse Concept

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Sparse Concept](#sparse-concept)
---
