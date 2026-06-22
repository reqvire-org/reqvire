# Elements

### Store Test Thesaurus

Fixture concept scheme used to verify the Project Store Thesaurus projection.

#### Metadata
  * type: concept-scheme
  * concept_base: https://example.test/thesaurus
  * concept_prefix: concept
---

### API Surface

Curated terminology for public API endpoints, operations, and payload boundaries.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Store Test Thesaurus](#store-test-thesaurus)
  * narrower: [Service Endpoint](#service-endpoint)
  * related: [Traceability](#traceability)
---

### Service Endpoint

A callable service boundary exposed by the system API.

#### Labels
  * altLabel: Endpoint

#### Scope Note
Use for the addressable API surface. Operation and payload structure stay in the structural ontology.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [API Surface](#api-surface)
  * broader: [API Surface](#api-surface)
---

### Traceability

The ability to follow model intent from capabilities through requirements, contracts, verification, and implementation evidence.

#### Metadata
  * type: concept

#### Relations
  * derivedFrom: [Store Test Thesaurus](#store-test-thesaurus)
  * related: [API Surface](#api-surface)
