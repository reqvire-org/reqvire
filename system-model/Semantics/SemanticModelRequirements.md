# Elements

### Local External Ontology Sources

The system shall allow ontology elements to declare local external ontology source files as internal RDF dependency inputs.

#### Details
Detailed declaration grammar, supported formats, path resolution, parser ownership, internal graph handling, and reserved-vocabulary boundaries shall follow the associated specification.

#### Concept References
  * [External ontology source](../Thesaurus/Thesaurus.md#external-ontology-source)
  * [External ontology prefix](../Thesaurus/Thesaurus.md#external-ontology-prefix)
  * [External ontology namespace](../Thesaurus/Thesaurus.md#external-ontology-namespace)
  * [External ontology resource](../Thesaurus/Thesaurus.md#external-ontology-resource)
  * [External ontology source path](../Thesaurus/Thesaurus.md#external-ontology-source-path)
  * [External ontology format](../Thesaurus/Thesaurus.md#external-ontology-format)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Local External Ontology Source Specification](SemanticModelSpecifications.md#local-external-ontology-source-specification)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * satisfiedBy: [mod.rs](../../crates/reqvire-core/src/semantic_contract/mod.rs)
  * specify: [External Ontology Source Management](SemanticModelFeature.md#external-ontology-source-management)
---

### Built-In External Ontology Source Resolution

The system shall provide Reqvire-shipped external ontology sources that are available to ontology and semantic-contract validation without per-project local `#### External Ontology` declarations.

#### Details
Built-in external ontology sources shall behave like imported external ontology sources for term resolution, SHACL alignment, concept-reference prefix resolution, and used-subset export materialization, while remaining clearly marked as built-in dependencies.

The initial built-in external ontology source shall be SKOS:

- prefix: `skos`
- namespace: `http://www.w3.org/2004/02/skos/core#`
- resource: `http://www.w3.org/2004/02/skos/core`
- source: `builtin:skos.rdf`
- format: RDF/XML

SKOS enables Reqvire models to separate conceptual and structural semantic layers. Native `concept-scheme` and `concept` elements generate `skos:ConceptScheme`, `skos:Concept`, `skos:prefLabel`, `skos:altLabel`, `skos:definition`, `skos:broader`, and `skos:related` facts for the conceptual layer, while structural ontology terms can point to those generated conceptual anchors through project-owned properties.

Built-in external ontology sources shall not be treated as RDF/OWL/SHACL language built-ins. They are parsed RDF source graphs owned by Reqvire core external-source policy.

#### Concept References
  * [External ontology source](../Thesaurus/Thesaurus.md#external-ontology-source)
  * [Built-in external ontology source](../Thesaurus/Thesaurus.md#built-in-external-ontology-source)
  * [Used external ontology subset](../Thesaurus/Thesaurus.md#used-external-ontology-subset)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Built-In External Ontology Source Specification](SemanticModelSpecifications.md#built-in-external-ontology-source-specification)
  * derive: [External Vocabulary Reference Resolution](#external-vocabulary-reference-resolution)
  * derivedFrom: [Local External Ontology Sources](#local-external-ontology-sources)
  * specify: [Built-In External Ontology Sources](SemanticModelFeature.md#built-in-external-ontology-sources)
---

### External Vocabulary Reference Resolution

The system shall resolve references to imported external vocabulary terms through local external ontology sources declared by ontology context and built-in external ontology sources shipped by Reqvire.

#### Details
Detailed local/built-in visibility, non-promotion, concept-reference, and duplicate-term rules shall follow the associated specification.

#### Concept References
  * [External ontology source](../Thesaurus/Thesaurus.md#external-ontology-source)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [External Vocabulary Reference Resolution Specification](SemanticModelSpecifications.md#external-vocabulary-reference-resolution-specification)
  * derive: [Used External Vocabulary Selection](#used-external-vocabulary-selection)
  * derivedFrom: [Built-In External Ontology Source Resolution](#built-in-external-ontology-source-resolution)
  * specify: [External Ontology Source Management](SemanticModelFeature.md#external-ontology-source-management)
---

### Used External Vocabulary Selection

The system shall derive the set of used external vocabulary terms from external namespace references found in authored ontology, authored model, and generated semantic content.

#### Details
Selection shall seed used external terms from authored ontology RDF, SHACL RDF, concept-reference facts in the authored-model graph, and generated semantic facts that reference declared external namespaces.

The selection contract is intentionally separate from external source declaration and public exposure policy. It identifies which external terms are used; it does not define the public output surface or the support facts needed to describe the terms.

#### Concept References
  * [Used external ontology subset](../Thesaurus/Thesaurus.md#used-external-ontology-subset)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Used External Vocabulary Selection Specification](SemanticModelSpecifications.md#used-external-vocabulary-selection-specification)
  * derive: [External Vocabulary Description Construction](#external-vocabulary-description-construction)
  * derivedFrom: [External Vocabulary Reference Resolution](#external-vocabulary-reference-resolution)
  * specify: [External Ontology Source Management](SemanticModelFeature.md#external-ontology-source-management)
---

### External Vocabulary Description Construction

The system shall apply the o-kernel referenced graph subset construction service to ontology graphs of interest and internal raw external dependency graphs.

#### Details
Detailed graph-role identification, o-kernel subset construction, metadata preservation, authored-source boundary, and exposure handoff rules shall follow the associated specification.

#### Concept References
  * [External ontology subset construct query](../Thesaurus/Thesaurus.md#external-ontology-subset-construct-query)
  * [Raw external ontology graph](../Thesaurus/Thesaurus.md#raw-external-ontology-graph)

#### Metadata
  * type: requirement

#### Contract References
  * [Referenced Graph Subset Construction Specification](../Architecture/OntologyKernelSpecifications.md#referenced-graph-subset-construction-specification)
  * [RDF Term Description Construction Specification](../Architecture/OntologyKernelSpecifications.md#rdf-term-description-construction-specification)

#### Relations
  * definedBy: [External Vocabulary Description Construction Specification](SemanticModelSpecifications.md#external-vocabulary-description-construction-specification)
  * satisfiedBy: [semantic_store.rs](../../crates/reqvire-core/src/semantic_store.rs)
  * derivedFrom: [Used External Vocabulary Selection](#used-external-vocabulary-selection)
  * specify: [External Ontology Source Management](SemanticModelFeature.md#external-ontology-source-management)
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### Ontology and Shapes Collection

The system shall collect ontology `#### Ontology` and semantic-contract `#### Shapes` RDF blocks from the graph registry into a reusable semantic context, and shall optionally project Reqvire model context and generated ontology construct facts into the same RDF graph.

#### Details
The system SHALL follow its owned specification and referenced contracts for block collection, model-layer projection, provenance, parser ownership, and clean-vs-full context.

#### Metadata
  * type: requirement

#### Contract References
  * [Semantic Contract Structure Specification](../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Ontology Kernel RDF Native Boundary Specification](../Architecture/OntologyKernelSpecifications.md#ontology-kernel-rdf-native-boundary-specification)
  * [Ontology Construct Classification Specification](../Architecture/OntologyKernelSpecifications.md#ontology-construct-classification-specification)

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Ontology and Shapes Collection Specification](SemanticModelSpecifications.md#ontology-and-shapes-collection-specification)
  * derive: [Ontology Term Definition Link Materialization](#ontology-term-definition-link-materialization)
  * derive: [OWL Reserved Vocabulary Recognition](#owl-reserved-vocabulary-recognition)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * satisfiedBy: [index.rs](../../crates/reqvire-core/src/semantic_contract/index.rs)
  * satisfiedBy: [prefixes.rs](../../crates/reqvire-core/src/semantic_contract/prefixes.rs)
  * satisfiedBy: [vocabulary.rs](../../crates/reqvire-core/src/semantic_contract/vocabulary.rs)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
---

### OWL Reserved Vocabulary Recognition

The system shall apply the o-kernel standards reserved vocabulary registry when validating and exporting Reqvire ontology and semantic-contract RDF.

#### Details
The system SHALL follow its owned specification and referenced o-kernel contract for validation, export, datatype-position handling, registry delegation, and non-reserved IRI handling.

#### Concept References
  * [OWL reserved vocabulary registry](../Thesaurus/Thesaurus.md#owl-reserved-vocabulary-registry)
  * [OWL reserved vocabulary term](../Thesaurus/Thesaurus.md#owl-reserved-vocabulary-term)
  * [OWL built-in datatype](../Thesaurus/Thesaurus.md#owl-built-in-datatype)

#### Metadata
  * type: requirement

#### Contract References
  * [Standards Reserved Vocabulary Recognition Specification](../Architecture/OntologyKernelSpecifications.md#standards-reserved-vocabulary-recognition-specification)

#### Relations
  * satisfiedBy: [validation.rs](../../crates/reqvire-core/src/graph_registry/validation.rs)
  * satisfiedBy: [owl_reserved.rs](../../crates/o-kernel/src/owl_reserved.rs)
  * definedBy: [OWL Reserved Vocabulary Recognition Specification](SemanticModelSpecifications.md#owl-reserved-vocabulary-recognition-specification)
  * derivedFrom: [Ontology and Shapes Collection](#ontology-and-shapes-collection)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### Ontology Term Definition Link Materialization

The system shall materialize standard `rdfs:isDefinedBy` links from authored named ontology resources to the generated ontology document IRI resolved from the owning Reqvire ontology element metadata.

#### Details
Reqvire already owns the generated `owl:Ontology` document declaration for authored ontology elements through `ontology_base` and `ontology_prefix`. Runtime semantic context shall therefore add `rdfs:isDefinedBy <ontology_base>` facts for authored named ontology resources in `#### Ontology` blocks without requiring authors to repeat that statement manually.

The intermediate generated `rdfs:isDefinedBy` section may declare the prefixes it uses, including `rdfs:` for the ownership predicate and `owl:` when ontology-document declarations are present, so the generated fragment remains parseable before final export assembly. Final Turtle artifacts shall still use the shared top-level prefixed Turtle export contract.

The materialization shall:
- Apply to authored named subjects collected from `#### Ontology` blocks when the subject IRI is inside the generated ontology document term namespace.
- Use the generated ontology document IRI resolved from the ontology element's `ontology_base`.
- Appear in Turtle, JSON-LD, semantic ontology API output, full semantic export, and the model-owned semantic store used by semantic query APIs.
- Be available to semantic tooling as ontology-document ownership metadata for query filtering, vocabulary grouping, and consumer evidence views.
- Avoid writing generated triples back into Markdown source.
- Deduplicate an authored matching `rdfs:isDefinedBy` triple when it is already present.
- Reject an authored named ontology resource whose explicit `rdfs:isDefinedBy` target conflicts with the generated ontology document IRI.
- Not generate `rdfs:isDefinedBy` links for imported external ontology terms; external terms use External Ontology source metadata for Reqvire ownership/provenance.

Imported external ontology terms remain governed by local and built-in external ontology source materialization and shall not be promoted to authored Reqvire ontology terms by this rule. If a used external source contains its own `rdfs:isDefinedBy` triples, those triples are source data rather than Reqvire-generated ownership facts.

Downstream ontology graph consumers shall be able to use the ownership metadata for grouping, filtering, search, and evidence while treating generated `rdfs:isDefinedBy` facts and generated `owl:Ontology` document IRIs as metadata rather than primary authored ontology relationships.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ontology Term Definition Link Materialization Specification](SemanticModelSpecifications.md#ontology-term-definition-link-materialization-specification)
  * derivedFrom: [Ontology and Shapes Collection](#ontology-and-shapes-collection)
  * satisfiedBy: [index.rs](../../crates/reqvire-core/src/semantic_contract/index.rs)
  * satisfiedBy: [prefixes.rs](../../crates/reqvire-core/src/semantic_contract/prefixes.rs)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### Prefixed Turtle Semantic Export

The system shall serialize semantic RDF graph exports as readable Turtle with deterministic `@prefix` declarations and compact prefixed names where this is syntactically valid.

#### Details
Detailed Turtle syntax, prefix-map construction, ontology-document preservation, RDF graph semantics, and JSON-LD separation rules shall follow the associated specification.

#### Concept References
  * [Namespace scoped ontology export](../Thesaurus/Thesaurus.md#namespace-scoped-ontology-export)
  * [External ontology prefix](../Thesaurus/Thesaurus.md#external-ontology-prefix)
  * [Ontology document](../Thesaurus/Thesaurus.md#ontology-document)

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Prefixed Turtle Semantic Export Specification](SemanticModelSpecifications.md#prefixed-turtle-semantic-export-specification)
  * derive: [Namespace-Scoped Ontology Export](#namespace-scoped-ontology-export)
  * derivedFrom: [Ontology and Shapes Collection](#ontology-and-shapes-collection)
  * satisfiedBy: [export.rs](../../crates/reqvire-core/src/semantic_contract/export.rs)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### Namespace-Scoped Ontology Export

The system shall filter clean authored ontology exports to a requested ontology base or term namespace.

#### Details
Detailed namespace normalization, clean-export scope, runtime artifact use, layer compatibility, and interface behavior shall follow the associated specification.

#### Concept References
  * [Runtime ontology namespace](../Thesaurus/Thesaurus.md#runtime-ontology-namespace)
  * [Namespace-scoped ontology export](../Thesaurus/Thesaurus.md#namespace-scoped-ontology-export)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Namespace-Scoped Ontology Export Specification](SemanticModelSpecifications.md#namespace-scoped-ontology-export-specification)
  * derivedFrom: [Prefixed Turtle Semantic Export](#prefixed-turtle-semantic-export)
  * satisfiedBy: [export.rs](../../crates/reqvire-core/src/semantic_contract/export.rs)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
---

### Runtime Reqvire Ontology Artifact

The system shall provide an embedded runtime Reqvire ontology artifact generated from the authored Reqvire ontology model.

#### Details
The artifact is an implementation snapshot for runtime/bootstrap vocabulary needs; authored ontology elements remain the source of truth. Detailed artifact path, namespace-scoped export, inclusion/exclusion, prefix, concept-bridge metadata, and embedding rules shall follow the associated specification.

#### Concept References
  * [Runtime ontology artifact](../Thesaurus/Thesaurus.md#runtime-ontology-artifact)
  * [Ontology document](../Thesaurus/Thesaurus.md#ontology-document)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Runtime Reqvire Ontology Artifact Specification](SemanticModelSpecifications.md#runtime-reqvire-ontology-artifact-specification)
  * derive: [Runtime Reqvire Ontology Synchronization](#runtime-reqvire-ontology-synchronization)
  * derivedFrom: [Namespace-Scoped Ontology Export](#namespace-scoped-ontology-export)
  * satisfiedBy: [runtime_ontology.rs](../../crates/reqvire-core/src/runtime_ontology.rs)
  * satisfiedBy: [reqvire.ttl](../../crates/reqvire-core/src/runtime_ontology/reqvire.ttl)
  * specify: [Runtime Reqvire Ontology Vocabulary](SemanticModelFeature.md#runtime-reqvire-ontology-vocabulary)
  * verifiedBy: [Runtime Reqvire Ontology Artifact Verification](../Verifications/Semantics/SemanticModelVerifications.md#runtime-reqvire-ontology-artifact-verification)
---

### Runtime Reqvire Ontology Synchronization

The system shall detect when embedded runtime Reqvire semantic artifacts are stale relative to the authored ontology model.

#### Details
Detailed export commands, artifact split, deterministic comparison, validation-gate, and regeneration behavior shall follow the associated specification and verification evidence.

#### Concept References
  * [Runtime ontology artifact](../Thesaurus/Thesaurus.md#runtime-ontology-artifact)
  * [Ontology source of truth](../Thesaurus/Thesaurus.md#ontology-source-of-truth)

#### Metadata
  * type: requirement

#### Contract References
  * [Runtime Reqvire SHACL Artifact Specification](SemanticModelSpecifications.md#runtime-reqvire-shacl-artifact-specification)

#### Relations
  * definedBy: [Runtime Reqvire Ontology Synchronization Specification](SemanticModelSpecifications.md#runtime-reqvire-ontology-synchronization-specification)
  * derivedFrom: [Runtime Reqvire Ontology Artifact](#runtime-reqvire-ontology-artifact)
  * satisfiedBy: [update-runtime-ontology-artifacts.sh](../../scripts/update-runtime-ontology-artifacts.sh)
  * satisfiedBy: [test.sh](../../tests/test-runtime-ontology-artifact/test.sh)
  * specify: [Runtime Reqvire Ontology Vocabulary](SemanticModelFeature.md#runtime-reqvire-ontology-vocabulary)
  * verifiedBy: [Runtime Reqvire Ontology Artifact Verification](../Verifications/Semantics/SemanticModelVerifications.md#runtime-reqvire-ontology-artifact-verification)
---

### Runtime Reqvire SHACL Artifact

The system shall provide an embedded runtime Reqvire SHACL artifact generated from authored semantic-contract shape rules.

#### Details
The artifact is an implementation snapshot for runtime/bootstrap shape-rule needs; authored semantic-contract elements remain the source of truth. Detailed artifact path, namespace-scoped shape export, inclusion/exclusion, physical split, and embedding rules shall follow the associated specification.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Runtime Reqvire SHACL Artifact Specification](SemanticModelSpecifications.md#runtime-reqvire-shacl-artifact-specification)
  * derivedFrom: [Namespace-Scoped Ontology Export](#namespace-scoped-ontology-export)
  * satisfiedBy: [runtime_ontology.rs](../../crates/reqvire-core/src/runtime_ontology.rs)
  * satisfiedBy: [reqvire-shacl.ttl](../../crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl)
  * specify: [Runtime Reqvire Ontology Vocabulary](SemanticModelFeature.md#runtime-reqvire-ontology-vocabulary)
  * verifiedBy: [Runtime Reqvire Ontology Artifact Verification](../Verifications/Semantics/SemanticModelVerifications.md#runtime-reqvire-ontology-artifact-verification)
---

### Semantic Query Authoring

WHEN an author supplies a semantic-query element, the system SHALL preserve its query document and optional output declarations as structured model content.

#### Details
Authoring grammar, generated identity, source locations, and serialization follow the owned specification.

#### Concept References
  * [Semantic Query](../Thesaurus/Thesaurus.md#semantic-query)

#### Contract References
  * [Supported Element Types Specification](../ModelStructure/Specifications.md#supported-element-types-specification)
  * [Structure and Addressing in Markdown Documents Contract Specification](../ModelStructure/Specifications.md#structure-and-addressing-in-markdown-documents-contract-specification)

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [queries.rs](../../crates/reqvire-core/src/semantic_contract/queries.rs)
  * specify: [Managed SPARQL Query Artifacts](SemanticModelFeature.md#managed-sparql-query-artifacts)
  * definedBy: [Semantic Query Authoring Specification](SemanticQuerySpecifications.md#semantic-query-authoring-specification)
---

### Semantic Query Context Validation

WHEN validating a semantic query, the system SHALL resolve schema references against its explicitly used ontology context and report invalid query content.

#### Details
Validation uses parsed syntax roles and the existing model mutation validation gate. Downstream runtime features remain portable without execution.

#### Concept References
  * [Semantic Query](../Thesaurus/Thesaurus.md#semantic-query)

#### Contract References
  * [Relation Semantics Specification](../ModelStructure/Specifications.md#relation-semantics-specification)
  * [External Vocabulary Reference Resolution Specification](SemanticModelSpecifications.md#external-vocabulary-reference-resolution-specification)
  * [Built-In External Ontology Source Specification](SemanticModelSpecifications.md#built-in-external-ontology-source-specification)

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [queries.rs](../../crates/reqvire-core/src/semantic_contract/queries.rs)
  * specify: [Managed SPARQL Query Artifacts](SemanticModelFeature.md#managed-sparql-query-artifacts)
  * definedBy: [Semantic Query Context Validation Specification](SemanticQuerySpecifications.md#semantic-query-context-validation-specification)
---

### Semantic Query Discovery

The system SHALL expose native authored semantic queries with their metadata, source provenance, and used ontology namespaces.

#### Details
Discovery, namespace selection, RDF projection, and CLI/MCP consumers share the native semantic index.

#### Concept References
  * [Semantic Query](../Thesaurus/Thesaurus.md#semantic-query)

#### Contract References
  * [Namespace-Scoped Ontology Export Specification](SemanticModelSpecifications.md#namespace-scoped-ontology-export-specification)

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [queries.rs](../../crates/reqvire-core/src/semantic_contract/queries.rs)
  * specify: [Managed SPARQL Query Artifacts](SemanticModelFeature.md#managed-sparql-query-artifacts)
  * definedBy: [Semantic Query Discovery Specification](SemanticQuerySpecifications.md#semantic-query-discovery-specification)
---

### Semantic Query Artifact Export

WHEN exporting a selected semantic query, the system SHALL produce deterministic standalone SPARQL bytes and their SHA-256 digest.

#### Details
Selection and serialization follow the shared artifact contract.

#### Concept References
  * [Semantic Query](../Thesaurus/Thesaurus.md#semantic-query)

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [queries.rs](../../crates/reqvire-core/src/semantic_contract/queries.rs)
  * satisfiedBy: [cli.rs](../../crates/reqvire-cli/src/cli.rs)
  * specify: [Managed SPARQL Query Artifacts](SemanticModelFeature.md#managed-sparql-query-artifacts)
  * definedBy: [Semantic Query Artifact Export Specification](SemanticQuerySpecifications.md#semantic-query-artifact-export-specification)
---

### Semantic Query Artifact Drift Check

WHEN checking a query artifact, the system SHALL compare its bytes with the generated query document and report matching, stale, missing, or unreadable artifacts.

#### Details
Artifact checks preserve the checked file and report expected and available actual hashes.

#### Concept References
  * [Semantic Query](../Thesaurus/Thesaurus.md#semantic-query)

#### Contract References
  * [Semantic Query Artifact Export Specification](SemanticQuerySpecifications.md#semantic-query-artifact-export-specification)

#### Metadata
  * type: requirement

#### Relations
  * satisfiedBy: [queries.rs](../../crates/reqvire-core/src/semantic_contract/queries.rs)
  * satisfiedBy: [cli.rs](../../crates/reqvire-cli/src/cli.rs)
  * specify: [Managed SPARQL Query Artifacts](SemanticModelFeature.md#managed-sparql-query-artifacts)
  * definedBy: [Semantic Query Artifact Drift Check Specification](SemanticQuerySpecifications.md#semantic-query-artifact-drift-check-specification)
---
