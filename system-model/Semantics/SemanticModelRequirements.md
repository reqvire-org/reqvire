# Elements

### Local External Ontology Sources

The system shall allow ontology elements to declare local external ontology source files as internal RDF dependency inputs.

#### Details
External ontology sources shall be declared with repeatable `#### External Ontology` sections on ontology elements. Each section shall define `prefix`, `namespace`, `resource`, `source`, and an optional `format` value. Supported source formats shall include Turtle/TTL, RDF/XML, and JSON-LD.

The Markdown grammar for `#### External Ontology` sections shall be parsed by the model parser. Semantic model construction shall consume parsed external-source records with source line numbers rather than rescanning element Markdown content.

The `source` path shall be local and resolved like a model path; Reqvire shall not fetch network ontology sources during validation or export.

External ontology source files shall be parsed as internal dependency inputs before validating ontology and semantic-contract references. Raw full external source graphs shall remain internal dependency inputs and shall not be public semantic output by this requirement.

Turtle blocks remain explicit. External source sections do not inject prefixes, ontology declarations, imports, or semantic triples into authored ontology or SHACL blocks.

Standard OWL reserved vocabulary and built-in datatype IRIs remain recognized by the o-kernel standards reserved vocabulary registry and do not require `#### External Ontology` declarations.

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
  * derive: [External Vocabulary Reference Resolution](#external-vocabulary-reference-resolution)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * satisfiedBy: [semantic_contract.rs](../../crates/reqvire-core/src/semantic_contract.rs)
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
  * derive: [Used External Vocabulary Selection](#used-external-vocabulary-selection)
  * derivedFrom: [Local External Ontology Sources](#local-external-ontology-sources)
  * specify: [Built-In External Ontology Sources](SemanticModelFeature.md#built-in-external-ontology-sources)
  * verifiedBy: [Ontology Semantic Export Verification](../Verifications/Reports/ModelReports/ReportingVerifications.md#ontology-semantic-export-verification)
---

### External Vocabulary Reference Resolution

The system shall resolve references to imported external vocabulary terms through local external ontology sources declared by ontology context and built-in external ontology sources shipped by Reqvire.

#### Details
Terms declared by the local source shall be available to the declaring ontology element, its ontology descendants, and semantic contracts that use that ontology context.

Terms declared by built-in external ontology sources shall be available to ontology elements and semantic contracts without requiring local source declarations. Non-ontology concept references shall resolve to generated native SKOS concept resources, not to arbitrary built-in vocabulary terms.

Imported terms shall remain marked as external declarations and shall not be promoted to authored Reqvire ontology terms. Imported terms shall not count as project-owned declarations for authored duplicate-term validation.

#### Concept References
  * [External ontology source](../Thesaurus/Thesaurus.md#external-ontology-source)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [External Vocabulary Reference Resolution Specification](SemanticModelSpecifications.md#external-vocabulary-reference-resolution-specification)
  * derive: [Used External Vocabulary Selection](#used-external-vocabulary-selection)
  * derivedFrom: [Built-In External Ontology Source Resolution](#built-in-external-ontology-source-resolution)
  * derivedFrom: [Local External Ontology Sources](#local-external-ontology-sources)
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
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### External Vocabulary Description Construction

The system shall apply the o-kernel referenced graph subset construction service to ontology graphs of interest and internal raw external dependency graphs.

#### Details
Reqvire shall identify which parsed RDF graphs are ontology graphs of interest and which parsed RDF graphs are external dependency graphs, then pass those graph roles to the o-kernel service. The o-kernel service shall own reference extraction, support context, annotation context, RDF list closure, and bounded expansion for the standard external ontology dependency subset profile. Reqvire shall preserve external source metadata, keep constructed triples out of authored Markdown blocks, and pass constructed used external vocabulary content to the external vocabulary exposure policy.

#### Concept References
  * [External ontology subset construct query](../Thesaurus/Thesaurus.md#external-ontology-subset-construct-query)
  * [Raw external ontology graph](../Thesaurus/Thesaurus.md#raw-external-ontology-graph)

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Referenced Graph Subset Construction Specification](../Architecture/OntologyKernelSpecifications.md#referenced-graph-subset-construction-specification)
  * [RDF Term Description Construction Specification](../Architecture/OntologyKernelSpecifications.md#rdf-term-description-construction-specification)

#### Relations
  * definedBy: [External Vocabulary Description Construction Specification](SemanticModelSpecifications.md#external-vocabulary-description-construction-specification)
  * derivedFrom: [Used External Vocabulary Selection](#used-external-vocabulary-selection)
  * specify: [External Ontology Source Management](SemanticModelFeature.md#external-ontology-source-management)
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### Ontology and Shapes Collection

The system shall collect ontology `#### Ontology` and semantic-contract `#### Shapes` RDF blocks from the graph registry into a reusable semantic context, and shall optionally project Reqvire model context and generated ontology construct facts into the same RDF graph.

#### Details
The default semantic context shall expose authored ontology RDF content and semantic-contract SHACL RDF content without changing the Markdown model as the source of truth.

When semantic model layer is requested, the collection shall also emit RDF triples for Reqvire model elements, element metadata, relation-family projection facts, requirement-to-capability specification relations, requirement-to-semantic-contract constraint relations, semantic-contract-to-ontology use relations, ontology hierarchy relations, concept references, ontology term declarations, semantic-contract shape references, and generated ontology projection facts derived from o-kernel construct classifications over direct-authored OWL/RDFS/SHACL RDF. Concrete parsed model elements and referenced artifacts in the model layer shall be emitted as `owl:NamedIndividual` instances of their Reqvire classes so OWL/RDF consumers can distinguish model ABox instances from schema vocabulary.

The collection shall preserve source element identifiers, source file paths, section kind, and line numbers so validation, reports, Explorer rendering, and downstream semantic tooling can cite the model source of each RDF block.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Semantic Contract Structure Specification](../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Ontology Kernel RDF Native Boundary Specification](../Architecture/OntologyKernelSpecifications.md#ontology-kernel-rdf-native-boundary-specification)
  * [Ontology Construct Classification Specification](../Architecture/OntologyKernelSpecifications.md#ontology-construct-classification-specification)

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Ontology and Shapes Collection Specification](SemanticModelSpecifications.md#ontology-and-shapes-collection-specification)
  * derive: [Ontology Term Definition Link Materialization](#ontology-term-definition-link-materialization)
  * derive: [OWL Reserved Vocabulary Recognition](#owl-reserved-vocabulary-recognition)
  * satisfiedBy: [explorer_runtime.rs](../../crates/reqvire-core/src/explorer_runtime.rs)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * satisfiedBy: [semantic_contract.rs](../../crates/reqvire-core/src/semantic_contract.rs)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
---

### Prefixed Turtle Semantic Export

The system shall serialize semantic RDF graph exports as readable Turtle with deterministic `@prefix` declarations and compact prefixed names where this is syntactically valid.

#### Details
Reqvire semantic exports remain RDF graphs. Turtle is the default RDF syntax for CLI, MCP, served ontology artifacts, runtime ontology artifacts, and downstream ontology tooling. OWL, RDFS, SHACL, SKOS, and Reqvire vocabulary terms are RDF terms in that graph; the exporter shall not treat "OWL Turtle" and "RDF Turtle" as separate data models.

The Turtle serializer shall build a canonical prefix map from Reqvire built-ins, authored ontology namespace metadata, native concept-scheme namespace metadata, and included external ontology source prefixes. The serializer shall emit one stable top-level `@prefix` declaration block and compact an IRI only when the IRI starts with a registered namespace and the remaining local name is valid Turtle local-name syntax.

The serializer shall preserve multiple authored `owl:Ontology` document subjects, authored `owl:imports` triples, generated ontology document declarations, generated `rdfs:isDefinedBy` links, and authored ontology-document metadata as RDF graph facts. Exact duplicate RDF triples may be deduplicated because RDF graph semantics do not preserve duplicate statements.

JSON-LD output remains a separate RDF serialization surface. It shall use JSON-LD context behavior rather than Turtle `@prefix` declarations.

#### Concept References
  * [Namespace scoped ontology export](../Thesaurus/Thesaurus.md#namespace-scoped-ontology-export)
  * [External ontology prefix](../Thesaurus/Thesaurus.md#external-ontology-prefix)
  * [Ontology document](../Thesaurus/Thesaurus.md#ontology-document)

#### Metadata
  * type: requirement

#### Relations
  * constrainedBy: [Semantic Export Projection Shape](../Ontologies/SemanticExport.md#semantic-export-projection-shape)
  * definedBy: [Prefixed Turtle Semantic Export Specification](SemanticModelSpecifications.md#prefixed-turtle-semantic-export-specification)
  * derivedFrom: [Ontology and Shapes Collection](#ontology-and-shapes-collection)
  * derive: [Namespace-Scoped Ontology Export](#namespace-scoped-ontology-export)
  * satisfiedBy: [semantic_contract.rs](../../crates/reqvire-core/src/semantic_contract.rs)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---

### Namespace-Scoped Ontology Export

The system shall filter clean authored ontology exports to a requested ontology base or term namespace.

#### Details
The namespace filter shall let Reqvire generate a deterministic runtime ontology artifact from the runtime Reqvire namespace while leaving non-runtime authored ontology content in the system model.

The filter shall accept either an ontology document base IRI such as `https://www.reqvire.org/ontology` or a term namespace IRI such as `https://www.reqvire.org/ontology#`, normalize both forms to the term namespace, and serialize only the clean authored ontology or graph export for that namespace.

The filter shall apply to clean semantic exports only. It shall not be combined with the `model` layer because authored-model facts and generated facts have a different graph scope from clean authored ontology and shape exports.

#### Concept References
  * [Runtime ontology namespace](../Thesaurus/Thesaurus.md#runtime-ontology-namespace)
  * [Namespace-scoped ontology export](../Thesaurus/Thesaurus.md#namespace-scoped-ontology-export)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Namespace-Scoped Ontology Export Specification](SemanticModelSpecifications.md#namespace-scoped-ontology-export-specification)
  * derivedFrom: [Prefixed Turtle Semantic Export](#prefixed-turtle-semantic-export)
  * satisfiedBy: [semantic_contract.rs](../../crates/reqvire-core/src/semantic_contract.rs)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
  * verifiedBy: [Runtime Reqvire Ontology Artifact Verification](../Verifications/Semantics/SemanticModelVerifications.md#runtime-reqvire-ontology-artifact-verification)
---

### Runtime Reqvire Ontology Artifact

The system shall provide an embedded runtime Reqvire ontology artifact generated from the authored Reqvire ontology model.

#### Details
The runtime artifact shall be `crates/reqvire-core/src/runtime_ontology/reqvire.ttl`, embedded through a stable Rust module entry point. It is an implementation artifact that satisfies runtime/bootstrap vocabulary needs; it is not the authored source of truth.

Authored ontology elements under `system-model/Ontologies` remain the source model. The runtime ontology artifact shall contain the namespace-scoped ontology export for the runtime Reqvire term namespace `https://www.reqvire.org/ontology#`: generated ontology document declarations, generated term definition links, and authored Reqvire runtime ontology RDF, without authored semantic-contract SHACL blocks, full model projection facts, or raw external source dumps.

The final runtime Turtle artifact shall use the shared prefixed Turtle export contract with one deterministic top-level prefix declaration block. Intermediate generated sections may be self-contained before final artifact assembly, but the committed artifact shall not depend on repeated in-section prefix declarations.

When the authored model maps structural ontology terms to standalone native concepts, namespace-scoped exports may contain authored `reqvire:mapsToConcept` bridge vocabulary and usages. The embedded runtime artifact is a bootstrap vocabulary snapshot for Reqvire core, so it shall be curated to exclude concept-scheme imports, `reqvire:mapsToConcept` declarations, and concept-bridge usage triples. The authored source remains responsible for those concept links; the runtime artifact carries only the structural runtime vocabulary needed by Reqvire core.

The artifact shall give runtime code a deterministic vocabulary snapshot while keeping ontology authoring, validation, and change impact anchored in the authored model.

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
The runtime ontology and SHACL artifacts shall be reproducible from the current authored model by running `reqvire semantic export --layer ontologies --layer shapes --namespace-base https://www.reqvire.org/ontology#`, applying the documented runtime-artifact curation and split step, and comparing the result with the embedded `reqvire.ttl` and `reqvire-shacl.ttl` artifacts after deterministic blank-node label normalization.

When an ontology, semantic contract, semantic export rule, or documented runtime curation/split rule changes runtime artifact output, validation through the test suite shall fail until the runtime artifacts are regenerated. This makes change impact from authored runtime ontology and SHACL changes reach implementation artifacts and verification evidence.

#### Concept References
  * [Runtime ontology artifact](../Thesaurus/Thesaurus.md#runtime-ontology-artifact)
  * [Ontology source of truth](../Thesaurus/Thesaurus.md#ontology-source-of-truth)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Runtime Reqvire Ontology Synchronization Specification](SemanticModelSpecifications.md#runtime-reqvire-ontology-synchronization-specification)
  * derivedFrom: [Namespace-Scoped Ontology Export](#namespace-scoped-ontology-export)
  * derivedFrom: [Runtime Reqvire Ontology Artifact](#runtime-reqvire-ontology-artifact)
  * derivedFrom: [Runtime Reqvire SHACL Artifact](#runtime-reqvire-shacl-artifact)
  * satisfiedBy: [test.sh](../../tests/test-runtime-ontology-artifact/test.sh)
  * specify: [Runtime Reqvire Ontology Vocabulary](SemanticModelFeature.md#runtime-reqvire-ontology-vocabulary)
  * verifiedBy: [Runtime Reqvire Ontology Artifact Verification](../Verifications/Semantics/SemanticModelVerifications.md#runtime-reqvire-ontology-artifact-verification)
---

### Runtime Reqvire SHACL Artifact

The system shall provide an embedded runtime Reqvire SHACL artifact generated from authored semantic-contract shape rules.

#### Details
The runtime SHACL artifact shall be `crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl`, embedded through the same stable Rust module entry point as the runtime ontology artifact. It is an implementation artifact that satisfies runtime/bootstrap shape-rule needs; it is not the authored source of truth.

Authored semantic-contract elements under `system-model/Ontologies` remain the source model. The runtime SHACL artifact shall contain namespace-scoped authored SHACL shape blocks whose declared shape subjects are in the runtime Reqvire term namespace `https://www.reqvire.org/ontology#`, without authored ontology vocabulary blocks, full model projection facts, generated ontology projection facts, or raw external source dumps.

The runtime ontology artifact and runtime SHACL artifact shall stay physically separate. Runtime code may load both artifacts together when it needs vocabulary plus validation rules, but neither artifact shall require mixing ontology vocabulary and SHACL rules in the same checked-in Turtle file.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Runtime Reqvire SHACL Artifact Specification](SemanticModelSpecifications.md#runtime-reqvire-shacl-artifact-specification)
  * derive: [Runtime Reqvire Ontology Synchronization](#runtime-reqvire-ontology-synchronization)
  * derivedFrom: [Namespace-Scoped Ontology Export](#namespace-scoped-ontology-export)
  * satisfiedBy: [runtime_ontology.rs](../../crates/reqvire-core/src/runtime_ontology.rs)
  * satisfiedBy: [reqvire-shacl.ttl](../../crates/reqvire-core/src/runtime_ontology/reqvire-shacl.ttl)
  * specify: [Runtime Reqvire Ontology Vocabulary](SemanticModelFeature.md#runtime-reqvire-ontology-vocabulary)
  * verifiedBy: [Runtime Reqvire Ontology Artifact Verification](../Verifications/Semantics/SemanticModelVerifications.md#runtime-reqvire-ontology-artifact-verification)
---

### OWL Reserved Vocabulary Recognition

The system shall apply the o-kernel standards reserved vocabulary registry when validating and exporting Reqvire ontology and semantic-contract RDF.

#### Details
Reqvire shall treat o-kernel reserved vocabulary IRIs as model-valid references in positions where their reserved role is valid without requiring `#### External Ontology` sections for those namespaces.

Semantic-contract validation shall accept standard RDF/RDFS/OWL/XSD/SHACL reserved IRIs, including `rdf:type` when used as a `sh:path`, without requiring an authored ontology declaration or local external ontology source. Reserved vocabulary recognition shall use the o-kernel registry rather than ad hoc Reqvire-side allowlists.

Built-in datatype IRIs from the kernel registry shall be accepted in datatype positions such as ontology datatype property ranges and SHACL `sh:datatype` values. The Reqvire adapter shall preserve the kernel distinction between datatype IRIs, datatype facet IRIs, annotation vocabulary, reserved classes, reserved properties, and SHACL syntax vocabulary.

Custom IRIs outside the reserved vocabulary registry remain subject to normal authored or external ontology resolution when term existence validation applies.

#### Concept References
  * [OWL reserved vocabulary registry](../Thesaurus/Thesaurus.md#owl-reserved-vocabulary-registry)
  * [OWL reserved vocabulary term](../Thesaurus/Thesaurus.md#owl-reserved-vocabulary-term)
  * [OWL built-in datatype](../Thesaurus/Thesaurus.md#owl-built-in-datatype)

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Standards Reserved Vocabulary Recognition Specification](../Architecture/OntologyKernelSpecifications.md#standards-reserved-vocabulary-recognition-specification)

#### Relations
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
- Appear in Turtle, JSON-LD, MCP semantic ontology output, full semantic export, and the model-owned semantic store used by MCP SPARQL.
- Be available to semantic tooling as ontology-document ownership metadata for query filtering, vocabulary grouping, and Explorer modal evidence.
- Avoid writing generated triples back into Markdown source.
- Deduplicate an authored matching `rdfs:isDefinedBy` triple when it is already present.
- Reject an authored named ontology resource whose explicit `rdfs:isDefinedBy` target conflicts with the generated ontology document IRI.
- Not generate `rdfs:isDefinedBy` links for imported external ontology terms; external terms use External Ontology source metadata for Reqvire ownership/provenance.

Imported external ontology terms remain governed by local and built-in external ontology source materialization and shall not be promoted to authored Reqvire ontology terms by this rule. If a used external source contains its own `rdfs:isDefinedBy` triples, those triples are source data rather than Reqvire-generated ownership facts.

Explorer ontology graph rendering shall use the ownership metadata for grouping, filtering, search, and modal evidence, and shall not render generated `rdfs:isDefinedBy` facts as canvas edges or generated `owl:Ontology` document IRIs as primary ontology graph nodes.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Ontology Term Definition Link Materialization Specification](SemanticModelSpecifications.md#ontology-term-definition-link-materialization-specification)
  * derivedFrom: [Ontology and Shapes Collection](#ontology-and-shapes-collection)
  * satisfiedBy: [semantic_contract.rs](../../crates/reqvire-core/src/semantic_contract.rs)
  * specify: [Semantic Model Core](SemanticModelFeature.md#semantic-model-core)
  * verifiedBy: [CLI Ontologies Command Verification](../Verifications/Interfaces/CLI/CLIVerifications.md#cli-ontologies-command-verification)
  * verifiedBy: [MCP Model Evidence Tools Verification](../Verifications/Interfaces/MCP/MCPVerifications.md#mcp-model-evidence-tools-verification)
---
