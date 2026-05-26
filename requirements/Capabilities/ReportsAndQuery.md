# Elements

### Provide Reports

As a **Manager** and **System Engineer**, I want Reqvire to generate structured model reports, so that I can inspect model content, traceability, coverage, resources, submodels, and exported documentation without manually reconstructing the graph.

#### Details
Provide reports is the capability for search, collect, coverage, traces, model, containment, resources, ontologies, submodels, lint, and export report contracts.

Report requirements define traversal direction, output structure, filters, JSON evidence, graph projection, and diagram/report rendering behavior.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Relation Ontology](../Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)
  * [Reqvire Verification Ontology](../Ontologies/Verification.md#reqvire-verification-ontology)
  * [Reqvire Verification Rollup Ontology](../Ontologies/Verification.md#reqvire-verification-rollup-ontology)
  * [Reqvire Report Ontology](../Ontologies/ReportsAndQuery.md#reqvire-report-ontology)

#### Relations
  * specifiedBy: [Model Reports](../Functional/Output/Reporting.md#model-reports)
---

### Semantic Model Export

As a **System Engineer**, I want Reqvire to expose ontology and shape content as semantic artifacts, so that downstream tools can inspect and reuse the model's semantic vocabulary without parsing Markdown directly.

#### Details
Semantic model export is the capability for collecting ontology and SHACL content, exporting Turtle/JSON-LD semantic artifacts, projecting Reqvire model context as RDF, and keeping semantic exports traceable to their Reqvire source elements.

Query behavior over RDF projections should be added later as a separate requirement or subcapability when the implementation actually supports it.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Semantic Contract Ontology](../Ontologies/CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
  * [Reqvire Semantic Export Ontology](../Ontologies/ReportsAndQuery.md#reqvire-semantic-export-ontology)

#### Relations
  * specifiedBy: [Ontology and Shapes Collection](../Functional/Output/Reporting.md#ontology-and-shapes-collection)
---
