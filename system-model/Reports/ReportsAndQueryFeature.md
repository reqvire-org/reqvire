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
Semantic model export is the capability for collecting ontology and SHACL content, exporting Turtle/JSON-LD semantic artifacts, optionally materializing the used subset of local external ontology sources, projecting Reqvire model context as RDF, materializing generated ontology projection facts, and keeping semantic exports traceable to their Reqvire source elements.

Local external ontology source materialization supports authored source declarations for Turtle/TTL, RDF/XML, and JSON-LD vocabulary files without promoting imported terms to authored Reqvire ontology terms. Full external source files are internal dependency inputs for validation and term resolution; public semantic export surfaces expose only external facts selected by the used external subset contract.

Semantic export contracts may define the intended graph patterns for generated projection facts. General-purpose query execution, query output, and inferred reasoning remain separate future capabilities unless a requirement explicitly scopes them.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Relations
  * specifiedBy: [Local External Ontology Sources](ModelReports/ReportingRequirements.md#local-external-ontology-sources)
  * specifiedBy: [Ontology and Shapes Collection](ModelReports/ReportingRequirements.md#ontology-and-shapes-collection)
  * specifiedBy: [OWL Reserved Vocabulary Recognition](ModelReports/ReportingRequirements.md#owl-reserved-vocabulary-recognition)
  * specifiedBy: [Used External Ontology Subset Projection](ModelReports/ReportingRequirements.md#used-external-ontology-subset-projection)
---
