# Elements

### Semantic Model Runtime Verification Objective

This objective groups verification that Reqvire runtime semantic artifacts remain derived from and synchronized with the authored semantic model.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Runtime Reqvire Ontology Artifact Verification](#runtime-reqvire-ontology-artifact-verification)
---

### Runtime Reqvire Ontology Artifact Verification

This verification proves that embedded runtime Reqvire ontology and SHACL artifacts are reproducible from the authored ontology model.

#### Details
Expected checks:
- Regenerate the runtime namespace-scoped ontology export from the real repository root with `reqvire semantic graph --namespace-base https://www.reqvire.org/ontology# --output <temporary-file>`.
- Apply documented runtime-artifact curation rules to the regenerated output when the authored model includes concept-layer imports or structural-to-concept bridge usage triples that are not runtime bootstrap facts.
- Split the regenerated output into ontology vocabulary and SHACL rule artifacts using semantic block kind metadata.
- Compare the regenerated, curated, and split output with `crates/reqvire-core/src/runtime_ontology/reqvire.ttl` and `crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl` after deterministic blank-node label normalization.
- Fail the dedicated runtime ontology artifact test when either embedded runtime artifact is stale or when ontology and SHACL blocks are mixed.
- Keep the check out of copied fixture workspaces so it verifies the actual Reqvire authored ontology model.
- Exercise the namespace export boundary and curation pipeline used by the runtime artifact rather than the whole authored ontology export.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Semantic Model Runtime Verification Objective](#semantic-model-runtime-verification-objective)
  * satisfiedBy: [test.sh](../../../tests/test-runtime-ontology-artifact/test.sh)
  * verify: [Namespace-Scoped Ontology Export](../../Semantics/SemanticModelRequirements.md#namespace-scoped-ontology-export)
  * verify: [Runtime Reqvire Ontology Artifact](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-ontology-artifact)
  * verify: [Runtime Reqvire Ontology Synchronization](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-ontology-synchronization)
  * verify: [Runtime Reqvire SHACL Artifact](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-shacl-artifact)
---
