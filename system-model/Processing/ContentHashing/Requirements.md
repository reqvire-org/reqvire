# Elements

### SHA-256 Content Fingerprinting

When a Reqvire consumer requests a content fingerprint, the system shall compute a deterministic SHA-256 digest over the exact supplied bytes through a shared hashing primitive.

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [hashing.rs](../../../crates/reqvire-core/src/hashing.rs)
  * specify: [Stable Content Hashing](ContentHashingFeature.md#stable-content-hashing)
---

### Stable Model Revision Fingerprinting

When a consumer requests a model revision, the system shall fingerprint the defined parsed-element state using a versioned canonical encoding and the shared SHA-256 primitive.

While the canonical inputs and encoding version remain unchanged, the system shall return the same model revision across machines and tool builds.

#### Concept References
  * [Element](../../Thesaurus/Thesaurus.md#element)
  * [Governance](../../Thesaurus/Thesaurus.md#governance)
  * [Model Relation](../../Thesaurus/Thesaurus.md#model-relation)
  * [Workspace Root](../../Thesaurus/Thesaurus.md#workspace-root)

#### Metadata
  * type: requirement

#### Contract Bindings
  * [SHA-256 Hash Encoding Specification](Specifications.md#sha-256-hash-encoding-specification)

#### Relations
  * satisfiedBy: [model_revision.rs](../../../crates/reqvire-core/src/model_revision.rs)
  * specify: [Stable Content Hashing](ContentHashingFeature.md#stable-content-hashing)
---
