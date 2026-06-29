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
- Regenerate runtime artifacts from the real repository root into a temporary directory by running separate namespace-scoped `reqvire semantic export --layer ontologies` and `reqvire semantic export --layer shapes` commands.
- Compare the regenerated temporary output with `crates/reqvire-core/src/runtime_ontology/reqvire.ttl` and `crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl` after deterministic blank-node label normalization.
- Verify regenerated runtime Turtle artifacts include deterministic prefix declarations, compact built-in and Reqvire prefixed names where safe, and remain parseable RDF/Turtle.
- Fail the dedicated runtime ontology artifact test when either embedded runtime artifact is stale or when ontology and SHACL blocks are mixed.
- Keep the check out of copied fixture workspaces so it verifies the actual Reqvire authored ontology model.
- Exercise the same export commands documented for the artifact update script while ensuring the verification itself does not replace checked-in source artifacts.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Semantic Model Runtime Verification Objective](#semantic-model-runtime-verification-objective)
  * satisfiedBy: [test.sh](../../../tests/test-runtime-ontology-artifact/test.sh)
  * verify: [Namespace-Scoped Ontology Export](../../Semantics/SemanticModelRequirements.md#namespace-scoped-ontology-export)
  * verify: [Prefixed Turtle Semantic Export](../../Semantics/SemanticModelRequirements.md#prefixed-turtle-semantic-export)
  * verify: [Runtime Reqvire Ontology Artifact](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-ontology-artifact)
  * verify: [Runtime Reqvire Ontology Synchronization](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-ontology-synchronization)
  * verify: [Runtime Reqvire SHACL Artifact](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-shacl-artifact)
---
