# Elements

### Public Documentation Website

The system shall publish a public documentation website that explains Reqvire concepts, workflows, interfaces, ontology authoring, verification, implementation coverage, and AI-assistant integration using terminology aligned with the validated system model.

#### Details
The website is the public-facing documentation surface for `www.reqvire.org`. Page source files are implementation artifacts for website documentation requirements so change impact can identify which pages need review when model terminology, workflow contracts, or interface behavior changes.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Public Documentation Website Source Context Specification](WebsiteSpecifications.md#public-documentation-website-source-context-specification)
  * derive: [Website Assistant Integration Documentation](#website-assistant-integration-documentation)
  * derive: [Website Command and Workflow Documentation](#website-command-and-workflow-documentation)
  * derive: [Website Home Messaging](#website-home-messaging)
  * derive: [Website Implementation Coverage Documentation](#website-implementation-coverage-documentation)
  * derive: [Website Modeling Language Documentation](#website-modeling-language-documentation)
  * derive: [Website Ontology Documentation](#website-ontology-documentation)
  * derive: [Website Requirements and Contracts Documentation](#website-requirements-and-contracts-documentation)
  * derive: [Website Semantic Model Documentation](#website-semantic-model-documentation)
  * derive: [Website Strategic Positioning Documentation](#website-strategic-positioning-documentation)
  * derive: [Website Verification Documentation](#website-verification-documentation)
  * satisfiedBy: [App.tsx](../../../website/src/App.tsx)
  * satisfiedBy: [AppLayout.tsx](../../../website/src/components/AppLayout.tsx)
  * satisfiedBy: [Sidebar.tsx](../../../website/src/components/Sidebar.tsx)
  * specify: [Public Documentation Website Interface](../InterfacesFeature.md#public-documentation-website-interface)
---

### Website Home Messaging

The system shall present Reqvire's public homepage as a concise semantic-engineering overview with links to major conceptual and workflow pages.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Home Messaging Specification](WebsiteSpecifications.md#website-home-messaging-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [Home.tsx](../../../website/src/pages/Home.tsx)
  * satisfiedBy: [graph-hierarchy.svg](../../../website/public/images/graph-hierarchy.svg)
---

### Website Strategic Positioning Documentation

The system shall explain Reqvire as a semantic engineering framework for AI-assisted, traceable, verifiable software engineering.

#### Reused Contract Context
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Strategic Positioning Documentation Specification](WebsiteSpecifications.md#website-strategic-positioning-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [StrategicVision.tsx](../../../website/src/pages/StrategicVision.tsx)
---

### Website Semantic Model Documentation

The system shall document the Reqvire semantic model, including element types, relation semantics, ownership, reused contract context, concept references, verification links, and implementation evidence.

#### Reused Contract Context
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Semantic Model Documentation Specification](WebsiteSpecifications.md#website-semantic-model-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [SemanticModel.tsx](../../../website/src/pages/SemanticModel.tsx)
  * satisfiedBy: [Submodels.tsx](../../../website/src/pages/Submodels.tsx)
---

### Website Requirements and Contracts Documentation

The system shall document capability, requirement, contract, semantic-contract, concept-reference, reused-contract-context, and governance rules using current Reqvire terminology.

#### Reused Contract Context
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Requirements and Contracts Documentation Specification](WebsiteSpecifications.md#website-requirements-and-contracts-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [RequirementsCapabilities.tsx](../../../website/src/pages/RequirementsCapabilities.tsx)
---

### Website Ontology Documentation

The system shall document ontology authoring, external ontology sources, built-in reserved vocabulary behavior, semantic contracts, validation, and ontology export modes.

#### Reused Contract Context
  * [Local External Ontology Source Specification](../../Reports/ModelReports/Specifications.md#local-external-ontology-source-specification)
  * [Ontology Collection Output Specification](../../Reports/ModelReports/Specifications.md#ontology-collection-output-specification)
  * [Ontology Projection Subgraph Materialization Specification](../../Reports/ModelReports/Specifications.md#ontology-projection-subgraph-materialization-specification)
  * [OWL Reserved Vocabulary Recognition Specification](../../Reports/ModelReports/Specifications.md#owl-reserved-vocabulary-recognition-specification)
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Ontology Documentation Specification](WebsiteSpecifications.md#website-ontology-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [Ontologies.tsx](../../../website/src/pages/Ontologies.tsx)
---

### Website Verification Documentation

The system shall document verification objectives, concrete verification types, evidence-backed verification behavior, coverage, traces, and verification roll-up semantics.

#### Reused Contract Context
  * [Verification Coverage Specification](../../Reports/ModelReports/Specifications.md#verification-coverage-specification)
  * [Verification Roll-up Specification](../../Verification/Traceability/Specifications.md#verification-roll-up-specification)
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Verification Documentation Specification](WebsiteSpecifications.md#website-verification-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [Verifications.tsx](../../../website/src/pages/Verifications.tsx)
---

### Website Implementation Coverage Documentation

The system shall document requirement implementation coverage, verification coverage, traceability, and how implementation/evidence artifacts relate to model elements.

#### Reused Contract Context
  * [Requirement Implementation Coverage Logic Specification](../../Reports/ModelReports/Specifications.md#requirement-implementation-coverage-logic-specification)
  * [Verification Coverage Specification](../../Reports/ModelReports/Specifications.md#verification-coverage-specification)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Implementation Coverage Documentation Specification](WebsiteSpecifications.md#website-implementation-coverage-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [ImplementationCoverage.tsx](../../../website/src/pages/ImplementationCoverage.tsx)
---

### Website Command and Workflow Documentation

The system shall document everyday CLI workflows, model commands, report commands, mutation commands, change impact, and advanced workflow patterns.

#### Reused Contract Context
  * [CLI Interface Structure Contract Specification](../CLI/Specifications.md#cli-interface-structure-contract-specification)
  * [Collect Content Specification](../../Reports/ModelReports/Specifications.md#collect-content-specification)
  * [Report Command Catalog Specification](../../Reports/ModelReports/Specifications.md#report-command-catalog-specification)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Command and Workflow Documentation Specification](WebsiteSpecifications.md#website-command-and-workflow-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [Advanced.tsx](../../../website/src/pages/Advanced.tsx)
  * satisfiedBy: [UserGuide.tsx](../../../website/src/pages/UserGuide.tsx)
---

### Website Assistant Integration Documentation

The system shall document MCP, coding-assistant integrations, prompt workflows, and assistant-scoped Reqvire context using current protocol and model-tool terminology.

#### Reused Contract Context
  * [MCP Prompt Guidance Specification](../MCP/Specifications.md#mcp-prompt-guidance-specification)
  * [MCP Protocol Standard Conformance Specification](../MCP/Specifications.md#mcp-protocol-standard-conformance-specification)
  * [MCP Semantic Query Tools Specification](../MCP/Specifications.md#mcp-semantic-query-tools-specification)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Assistant Integration Documentation Specification](WebsiteSpecifications.md#website-assistant-integration-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [CodingAssistants.tsx](../../../website/src/pages/CodingAssistants.tsx)
  * satisfiedBy: [Integrations.tsx](../../../website/src/pages/Integrations.tsx)
  * satisfiedBy: [McpServer.tsx](../../../website/src/pages/McpServer.tsx)
---

### Website Modeling Language Documentation

The system shall document Reqvire Markdown element syntax, relation syntax, model file structure, and user-facing modeling-language examples.

#### Reused Contract Context
  * [Specification File Identification Contract Specification](../../ModelStructure/Specifications.md#specification-file-identification-contract-specification)
  * [Structure and Addressing in Markdown Documents Contract Specification](../../ModelStructure/Specifications.md#structure-and-addressing-in-markdown-documents-contract-specification)

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Modeling Language Documentation Specification](WebsiteSpecifications.md#website-modeling-language-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [ModelingLanguage.tsx](../../../website/src/pages/ModelingLanguage.tsx)
---
