# Elements

### Defining Model Structure

As a **System Engineer**, I want a well-defined Reqvire model structure and addressing language, so that I can organize capabilities, requirements, contracts, semantic contracts, verifications, and traceability links consistently across a repository.

#### Details
Defining model structure is the capability root for the Reqvire model vocabulary. It defines how elements use explicit concept references for element semantics, capability abilities, requirement obligations, reusable semantic contracts, contracts, governance metadata, and traceability relations.

This capability answers how Reqvire structures system models in Markdown, which concepts are part of the modeling language, and which ontology elements define the language before implementation-facing requirements are written.
This capability also includes ontology rebasing support for the owned ontology context: when `ontology_base` or `ontology_prefix` changes, the dependent ontology boundary chain must be rewritten atomically.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Relations
  * specifiedBy: [Coexistence of Structured and Unstructured Documents](Configuration.md#coexistence-of-structured-and-unstructured-documents)
  * specifiedBy: [Git Repository as Project Root](ModelManagement.md#git-repository-as-project-root)
  * specifiedBy: [Specification File Identification](StructureAndParsing.md#specification-file-identification)
  * specifiedBy: [Structure and Addressing in Markdown Documents](StructureAndParsing.md#structure-and-addressing-in-markdown-documents)
---

### Defining Model Structure Source

The Reqvire model structure distinguishes implementation-independent capabilities from implementation-facing obligations.

Capability elements answer:
- What coherent operational, product, business, regulatory, or system ability is this?
- Why does this area exist in the product model?
- What stakeholder need, feature context, operational context, regulatory driver, mission objective, service context, AI context, source context, and ontology define its meaning?
- Which requirements specify this capability?
- Which verification evidence directly verifies this capability when capability-level evidence is appropriate?

Requirement elements answer:
- What must the system do?
- Under what condition, interface, state, or scope?
- What implementation or evidence can satisfy it?
- What verification proves it?

Ontology elements hold stable domain and model-language meaning. They live in the dedicated `requirements/Ontologies` folder so the ontology plane stays orthogonal to capability-root subgraphs. The top ontology element defines the ontology document base and canonical prefix; child ontology elements in the same base contribute terms to that same generated `owl:Ontology` document. Non-ontology elements bind their prose and model intent to ontology terms through `#### Concept References`. Semantic contracts define reusable SHACL shape profiles, explicitly `use` ontology vocabulary, and `constrain` zero or more requirements when formal closed-world rules are needed.
Ontology elements are first-class model-language content: they define concepts, relations, and rules for the modeling language, and their base or prefix rebasing must preserve the full dependency chain so the model stays valid.

Requirements should not duplicate ontology content and should not reuse ontology directly. A requirement that needs shared meaning should declare `#### Concept References` to the ontology terms it uses. Capabilities, contracts, verification objectives, and concrete verifications may do the same when their prose needs ontology-backed terms. If a requirement needs closed-world constraints, it should be linked to a semantic contract with `constrainedBy`/`constrain`; the semantic contract must use the ontology terms it references through explicit `use`/`usedBy` relations and must not author concept references.

The Reqvire model is intentionally split into separate capability-root subgraphs. A top capability should be used when requirements truly specify the same capability. Child capabilities should be used only when the capability has independently verifiable operational, product, interface, stakeholder, regulatory, or domain slices that need separate ownership, lifecycle, architecture impact, or collection. Shared ontology lives in `requirements/Ontologies`; elements reference ontology terms directly through concept references instead of becoming children of another capability just to reuse vocabulary. Requirements specify their local capability unless they are actually implementing the shared capability.

Shared terms for capability, requirement, contract, and verification prose come from explicit `#### Concept References`, not from one universal capability hierarchy and not from ontology reused_contract_context. Semantic-contract SHACL terms come from explicit semantic-contract `use` relations to ontology elements. This keeps each model concern independently collectible, reviewable, and impact-analyzable while preserving auditable semantic dependencies.

#### Metadata
  * type: specification

#### Relations
  * define: [Ontology and Semantic Contract Model](ModelManagement.md#ontology-and-semantic-contract-model)
---
