# Elements

### Defining Model Structure

As a **System Engineer**, I want a well-defined Reqvire model structure and addressing language, so that I can organize feature roots, requirements, refinements, semantic contracts, verifications, and traceability links consistently across a repository.

#### Details
Defining model structure is the feature root for the Reqvire model vocabulary. It attaches the authoritative ontology set for elements, feature anchors, requirement obligations, requirement-owned semantic contracts, requirement-owned refinements, governance metadata, and traceability relations.

This feature answers how Reqvire structures system models in Markdown, which concepts are part of the modeling language, and which ontology elements define the language before implementation-facing requirements are written.

#### Metadata
  * type: feature
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Feature Ontology](Ontologies/FeatureRequirementModel.md#reqvire-feature-ontology)
  * [Reqvire Requirement Ontology](Ontologies/FeatureRequirementModel.md#reqvire-requirement-ontology)
  * [Reqvire Semantic Contract Ontology](Ontologies/FeatureRequirementModel.md#reqvire-semantic-contract-ontology)
  * [Reqvire Relation Ontology](Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)
  * [Reqvire Governance Ontology](Ontologies/Governance.md#reqvire-governance-ontology)

#### Relations
  * refinedBy: [Defining Model Structure Source](#defining-model-structure-source)
  * specifiedBy: [Coexistence of Structured and Unstructured Documents](Functional/Core/Configuration.md#coexistence-of-structured-and-unstructured-documents)
  * specifiedBy: [Git Repository as Project Root](Functional/Core/ModelManagement.md#git-repository-as-project-root)
  * specifiedBy: [Specification File Identification](Functional/Core/StructureAndParsing.md#specification-file-identification)
  * specifiedBy: [Structure and Addressing in Markdown Documents](Functional/Core/StructureAndParsing.md#structure-and-addressing-in-markdown-documents)
---

### Defining Model Structure Source

The Reqvire model structure distinguishes product/capability anchors from implementation-facing obligations.

Feature elements answer:
- What capability, product area, stakeholder need, regulatory area, external obligation, or domain slice is this?
- Why does this area exist in the product model?
- What source context and ontology define the domain language?
- Which requirements belong under this capability?

Requirement elements answer:
- What must the system do?
- Under what condition, interface, state, or scope?
- What implementation or evidence can satisfy it?
- What verification proves it?

Ontology elements hold stable domain and model-language meaning. They live in the dedicated `requirements/Ontologies` folder so the ontology plane stays orthogonal to feature-root subgraphs. Feature elements attach ontology to define the semantic context for the feature and for requirements that specify the feature. Requirement-owned semantic contracts define SHACL shape profiles that apply already reachable feature ontology to one obligation.

Requirements should not duplicate ontology content and should not attach ontology directly. A requirement that needs shared meaning should specify the feature that attaches the relevant ontology; child requirements inherit that ontology context through the owning requirement and feature hierarchy. If the requirement needs closed-world constraints for its own obligation, it may own a shapes-only semantic contract.

The Reqvire model is intentionally split into separate feature-root subgraphs. A top feature should be used when requirements truly specify the same capability. Subfeatures should be used only when the capability has meaningful product, interface, stakeholder, regulatory, or domain slices that need independent ownership or collection. Shared ontology lives in `requirements/Ontologies`; feature roots attach the ontology they consume instead of becoming children of another feature just to reuse vocabulary. Requirements specify their local feature unless they are actually implementing the shared capability.

Shared terms come from explicit feature-level ontology attachments, not from one universal feature hierarchy and not from direct requirement ontology attachments. This keeps each model concern independently collectible, reviewable, and impact-analyzable while preserving auditable dependencies between contracts.

#### Metadata
  * type: source

#### Relations
  * refine: [Defining Model Structure](#defining-model-structure)
---

### Reqvire Core Element Ontology Shape Profile

SHACL profile split from Reqvire Core Element Ontology so ontology vocabulary remains first-class and requirement-owned semantic contracts carry closed-world constraints.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Element ;
  sh:property [
    sh:path reqvire:identifier ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:name ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:elementType ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Ontology and Semantic Contract Model](Functional/Core/ModelManagement.md#ontology-and-semantic-contract-model)
---
