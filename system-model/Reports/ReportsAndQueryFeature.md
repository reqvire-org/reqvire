# Elements

### Provide Reports

As a **Manager** and **System Engineer**, I want Reqvire to generate structured model reports, so that I can inspect model content, traceability, coverage, resources, submodels, and semantic outputs without manually reconstructing the graph.

#### Details
Provide reports is the capability for search, collect, coverage, traces, model, containment, resources, ontologies, submodels, lint, and semantic output contracts.

Report requirements define traversal direction, output structure, filters, JSON evidence, graph projection, and diagram/report rendering behavior.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: medium
  * status: approved

#### Relations
  * specifiedBy: [Model Reports](ModelReports/ReportingRequirements.md#model-reports)
---

### Semantic Model Export

As a **System Engineer**, I want Reqvire to expose ontology and shape content as semantic artifacts, so that downstream tools can inspect and reuse the model's semantic vocabulary without parsing Markdown directly.

#### Details
Semantic model export is the capability for collecting ontology vocabulary, SHACL shapes, SKOS concepts, and combined semantic graph content from the semantic model core, exporting Turtle/JSON-LD semantic artifacts, optionally materializing the used external ontology subset, projecting Reqvire model context as RDF, materializing generated ontology projection facts, and keeping semantic exports traceable to their Reqvire source elements.

External ontology source parsing, built-in external source resolution, term validation, and used-subset construction are owned by [Semantic Model Core](../Semantics/SemanticModelFeature.md#semantic-model-core). Reporting owns the public export surface and exposure policy for those semantic facts.

Semantic export contracts may define the intended graph patterns for generated projection facts. General-purpose query execution, query output, and inferred reasoning remain separate future capabilities unless a requirement explicitly scopes them.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * derivedFrom: [Semantic Model Core](../Semantics/SemanticModelFeature.md#semantic-model-core)
  * specifiedBy: [External Vocabulary Exposure Policy](ModelReports/ReportingRequirements.md#external-vocabulary-exposure-policy)
  * specifiedBy: [Ontology Collection Output](ModelReports/ReportingRequirements.md#ontology-collection-output)
---
