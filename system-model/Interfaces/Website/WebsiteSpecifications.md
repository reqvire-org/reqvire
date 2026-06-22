# Elements

### Public Documentation Website Source Context Specification

#### Details
The public documentation website shall treat React page modules, route configuration, layout, sidebar navigation, and static public images as implementation artifacts for website documentation requirements.

Each website page requirement shall link to the page source files that implement its user-facing wording. When a Reqvire model contract, relation rule, ontology workflow, verification rule, or CLI/MCP interface contract changes, change-impact review shall be able to reach the website requirement and then the concrete website page source through `satisfiedBy`.

The website shall not be modeled as the Explorer UI. It is a separate public documentation surface for `www.reqvire.org`, while the Explorer is the model-browsing application served by Reqvire.

#### Metadata
  * type: specification

#### Relations
  * define: [Public Documentation Website](WebsiteRequirements.md#public-documentation-website)
---

### Website Home Messaging Specification

#### Details
The home page shall provide concise public positioning for Reqvire, introduce the semantic engineering framework, summarize the major knowledge categories, and route readers to conceptual and workflow pages.

Homepage wording shall stay aligned with current model vocabulary for ontologies, capabilities, requirements, contracts, verification, implementation artifacts, change impact, and assistant context.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Home Messaging](WebsiteRequirements.md#website-home-messaging)
---

### Website Strategic Positioning Documentation Specification

#### Details
The strategic positioning page shall explain why Reqvire separates ontologies, capabilities, requirements, contracts, verifications, implementation artifacts, and evidence. It shall describe Reqvire as a semantic engineering framework that supports traceability, verification, change-impact workflows, and AI-assisted implementation context.

When relation semantics, contract terminology, semantic-contract rules, or AI-context workflows change, this page shall be reviewed for wording drift.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Strategic Positioning Documentation](WebsiteRequirements.md#website-strategic-positioning-documentation)
---

### Website Semantic Model Documentation Specification

#### Details
The semantic model page shall explain Reqvire element families, relation semantics, ownership rules, submodel boundaries, concept references, reused contract context, verification links, and implementation evidence.

This page shall distinguish non-semantic requirement-owned contracts from semantic contracts and shall keep relation vocabulary aligned with the canonical relation families.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Semantic Model Documentation](WebsiteRequirements.md#website-semantic-model-documentation)
---

### Website Requirements and Contracts Documentation Specification

#### Details
The requirements and capabilities page shall explain capability scope, requirement obligations, requirement-owned contracts, semantic contracts, governance metadata, concept references, and reused contract context.

Contract wording shall use the canonical phrasing: contracts define requirements in precise terms, including source basis, specifications, constraints, behavior, state, interfaces, and input/output semantics. Semantic contracts shall be presented as ontology-plane SHACL profiles that constrain requirements through `constrainedBy`/`constrain` and use ontology through `use`/`usedBy`.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Requirements and Contracts Documentation](WebsiteRequirements.md#website-requirements-and-contracts-documentation)
---

### Website Ontology Documentation Specification

#### Details
The ontology page shall document ontology elements, concept references, external ontology sources, built-in reserved vocabulary behavior, semantic contracts, validation rules, export modes, and semantic tooling.

The page shall distinguish authored ontology/SHACL output, full semantic model context, external ontology source inclusion, and Explorer ontology visualization behavior without implying hidden Turtle prefix injection or hidden semantic triples.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Ontology Documentation](WebsiteRequirements.md#website-ontology-documentation)
---

### Website Verification Documentation Specification

#### Details
The verification page shall explain verification objectives, concrete verification types, evidence-backed verification satisfaction, requirement verification links, and capability coverage rollup philosophy.

The page shall keep the verification-objective model current: objectives organize verification work and concrete verification elements provide verification evidence through `verify`/`verifiedBy`; evidence-backed concrete verifications use `satisfiedBy` for test or proof artifacts.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Verification Documentation](WebsiteRequirements.md#website-verification-documentation)
---

### Website Implementation Coverage Documentation Specification

#### Details
The implementation coverage page shall explain how requirements and evidence-backed verifications use `satisfiedBy`, how implementation coverage and verification coverage differ, and how reports identify missing implementation or evidence links.

The page shall remain aligned with implementation coverage report semantics and verification coverage semantics.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Implementation Coverage Documentation](WebsiteRequirements.md#website-implementation-coverage-documentation)
---

### Website Command and Workflow Documentation Specification

#### Details
The workflow documentation pages shall describe CLI commands, report commands, model mutation commands, change-impact analysis, collection, coverage, traces, and advanced workflows using current command names and options.

Workflow examples shall be treated as source context for CLI and report contract changes.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Command and Workflow Documentation](WebsiteRequirements.md#website-command-and-workflow-documentation)
---

### Website Assistant Integration Documentation Specification

#### Details
The assistant integration pages shall explain MCP, coding-assistant plugins, Codex skills, prompt workflows, MCP endpoints, protocol behavior, and assistant-facing Reqvire context.

These pages shall be reviewed when MCP tool schemas, prompt guidance, protocol support, or assistant integration workflows change.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Assistant Integration Documentation](WebsiteRequirements.md#website-assistant-integration-documentation)
---

### Website Modeling Language Documentation Specification

#### Details
The modeling-language page shall document Reqvire Markdown structure, element metadata, relation syntax, reserved subsections, and examples using current parser and model-structure contracts.

The page shall be reviewed when structured Markdown parsing, file identification, reserved subsections, relation syntax, or element addressing changes.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Modeling Language Documentation](WebsiteRequirements.md#website-modeling-language-documentation)
---
