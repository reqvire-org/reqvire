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
- Regenerate runtime artifacts from the real Reqvire repository workspace into a temporary directory by running separate namespace-scoped `reqvire semantic export --layer ontologies` and `reqvire semantic export --layer shapes` commands.
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
  * verify: [Runtime Reqvire Ontology Artifact](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-ontology-artifact)
  * verify: [Runtime Reqvire Ontology Synchronization](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-ontology-synchronization)
  * verify: [Runtime Reqvire SHACL Artifact](../../Semantics/SemanticModelRequirements.md#runtime-reqvire-shacl-artifact)
---

### Managed Semantic Query Verification Objective

Establish that managed query authoring, portable vocabulary validation, discovery, deterministic export, and mutation preserve the agreed model contract.

#### Metadata
  * type: verification-objective
---

### Semantic Query Authoring Verification

Verify the managed semantic query authoring contract through accepted and rejected inputs.

#### Details
Author SELECT, ASK, CONSTRUCT and DESCRIBE; preserve prose, Query and Produces through collect/add/override/format/move/rename. Reject missing/duplicate/empty/wrong-language sections, unsupported Update and unsupported metadata. Compare persisted files after rejected edits.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Managed Semantic Query Verification Objective](#managed-semantic-query-verification-objective)
  * verify: [Semantic Query Authoring](../../Semantics/SemanticModelRequirements.md#semantic-query-authoring)
  * satisfiedBy: [test.sh](../../../tests/test-semantic-queries/test.sh)
---

### Semantic Query Context Validation Verification

Verify the managed semantic query context validation contract through accepted and rejected inputs.

#### Details
Validate explicit prefixes and BASE, schema roles in nested queries/templates/paths/literal datatypes, ontology ancestors and local/built-in sources. Reject undeclared/wrong-role/unreachable vocabulary, missing use, invalid Produces and ambiguous prefixes. Accept data instance IRIs, graphs, SERVICE/SILENT, datasets and custom functions without execution. Verify that terms used only by a query or Produces appear in external-used export while unused external terms stay excluded.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Managed Semantic Query Verification Objective](#managed-semantic-query-verification-objective)
  * verify: [Semantic Query Context Validation](../../Semantics/SemanticModelRequirements.md#semantic-query-context-validation)
  * satisfiedBy: [test.sh](../../../tests/test-semantic-queries/test.sh)
---

### Semantic Query Discovery Verification

Verify the managed semantic query discovery contract through accepted and rejected inputs.

#### Details
Compare CLI/MCP native provenance, ordering, names/IRIs, context namespace filters and query RDF exports. Imported same-name queries stay RDF-only. Verify malformed candidates retain diagnostics, ontology changes impact query users through usedBy, query changes do not propagate back to ontologies, and metadata-only changes affect revisions and impact while leaving artifact hash unchanged.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [semantic_queries.rs](../../../crates/reqvire-core/tests/semantic_queries.rs)
  * verify: [MCP Managed Query Artifacts](../../Interfaces/MCP/ManagedQueries.md#mcp-managed-query-artifacts)
  * verify: [CLI Managed Query Artifacts](../../Interfaces/CLI/ManagedQueries.md#cli-managed-query-artifacts)
  * derivedFrom: [Managed Semantic Query Verification Objective](#managed-semantic-query-verification-objective)
  * verify: [Semantic Query Discovery](../../Semantics/SemanticModelRequirements.md#semantic-query-discovery)
  * satisfiedBy: [test.sh](../../../tests/test-semantic-queries/test.sh)
---

### Semantic Query Artifact Export Verification

Verify the managed semantic query artifact export contract through accepted and rejected inputs.

#### Details
Compare exact emitted bytes and SHA-256, blank-line trimming, comments, multiline literal CRLF, repeated exports, JSON envelopes and atomic file replacement. Reject invalid selection and conflicting output modes without overwriting files.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Managed Semantic Query Verification Objective](#managed-semantic-query-verification-objective)
  * verify: [Semantic Query Artifact Export](../../Semantics/SemanticModelRequirements.md#semantic-query-artifact-export)
  * satisfiedBy: [test.sh](../../../tests/test-semantic-queries/test.sh)
---

### Semantic Query Artifact Drift Check Verification

Verify the managed semantic query artifact drift check contract through accepted and rejected inputs.

#### Details
Check matching, stale, missing, and unreadable artifacts. Assert exit status and expected/actual hashes and compare the unchanged artifact bytes.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Managed Semantic Query Verification Objective](#managed-semantic-query-verification-objective)
  * verify: [Semantic Query Artifact Drift Check](../../Semantics/SemanticModelRequirements.md#semantic-query-artifact-drift-check)
  * satisfiedBy: [test.sh](../../../tests/test-semantic-queries/test.sh)
---
