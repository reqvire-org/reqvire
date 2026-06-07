# Elements

### Defining Model Structure

As a **System Engineer**, I want a well-defined Reqvire model structure and addressing language, so that I can organize capabilities, requirements, refinements, semantic contracts, semantic query contracts, verifications, and traceability links consistently across a repository.

#### Details
Defining model structure is the capability root for the Reqvire model vocabulary. It attaches the authoritative ontology set for elements, capability abilities, requirement obligations, requirement-owned semantic contracts, requirement-owned semantic query contracts, refinements, governance metadata, and traceability relations.

This capability answers how Reqvire structures system models in Markdown, which concepts are part of the modeling language, and which ontology elements define the language before implementation-facing requirements are written.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: high
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Capability Ontology](Ontologies/CapabilityRequirementModel.md#reqvire-capability-ontology)
  * [Reqvire Requirement Ontology](Ontologies/CapabilityRequirementModel.md#reqvire-requirement-ontology)
  * [Reqvire Semantic Contract Ontology](Ontologies/CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
  * [Reqvire Relation Ontology](Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)
  * [Reqvire Verification Ontology](Ontologies/Verification.md#reqvire-verification-ontology)
  * [Reqvire Governance Ontology](Ontologies/Governance.md#reqvire-governance-ontology)

#### Relations
  * specifiedBy: [Coexistence of Structured and Unstructured Documents](Functional/Core/Configuration.md#coexistence-of-structured-and-unstructured-documents)
  * specifiedBy: [Git Repository as Project Root](Functional/Core/ModelManagement.md#git-repository-as-project-root)
  * specifiedBy: [Specification File Identification](Functional/Core/StructureAndParsing.md#specification-file-identification)
  * specifiedBy: [Structure and Addressing in Markdown Documents](Functional/Core/StructureAndParsing.md#structure-and-addressing-in-markdown-documents)
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

Ontology elements hold stable domain and model-language meaning. They live in the dedicated `requirements/Ontologies` folder so the ontology plane stays orthogonal to capability-root subgraphs. Capability elements attach ontology to define the semantic context for the capability and for requirements that specify the capability. Requirement-owned semantic contracts define SHACL shape profiles that apply already reachable capability ontology to one obligation. Requirement-owned semantic query contracts define declarative semantic queries over the same reachable context without a query-kind classification.

Requirements should not duplicate ontology content and should not attach ontology directly. A requirement that needs shared meaning should specify the capability that attaches the relevant ontology; child requirements inherit that ontology context through the owning requirement and capability hierarchy. If the requirement needs closed-world constraints for its own obligation, it may own a shapes-only semantic contract.

The Reqvire model is intentionally split into separate capability-root subgraphs. A top capability should be used when requirements truly specify the same capability. Child capabilities should be used only when the capability has independently verifiable operational, product, interface, stakeholder, regulatory, or domain slices that need separate ownership, lifecycle, architecture impact, or collection. Shared ontology lives in `requirements/Ontologies`; capability roots attach the ontology they consume instead of becoming children of another capability just to reuse vocabulary. Requirements specify their local capability unless they are actually implementing the shared capability.

Shared terms come from explicit capability-level ontology attachments, not from one universal capability hierarchy and not from direct requirement ontology attachments. This keeps each model concern independently collectible, reviewable, and impact-analyzable while preserving auditable dependencies between contracts.

#### Metadata
  * type: specification

#### Relations
  * refine: [Ontology and Semantic Contract Model](Functional/Core/ModelManagement.md#ontology-and-semantic-contract-model)
---

### Reqvire Core Element Ontology Shape Profile

Defines SHACL constraints for shared Reqvire element identity, type, file location, and structural metadata.

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
  ] ;
  sh:property [
    sh:path reqvire:id ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:filePath ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:fragment ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:content ;
    sh:datatype xsd:string ;
  ] .

reqvire:ArtifactShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Artifact ;
  sh:property [
    sh:path reqvire:externalUrl ;
    sh:maxCount 1 ;
    sh:datatype xsd:anyURI ;
  ] .

reqvire:ElementTypeShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ElementType ;
  sh:property [
    sh:path reqvire:elementTypeName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:elementTypeCategory ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:defaultElementType ;
    sh:maxCount 1 ;
    sh:datatype xsd:boolean ;
  ] .

reqvire:CustomElementTypeShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CustomElementType ;
  sh:property [
    sh:path reqvire:customElementTypePattern ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] .

reqvire:ReservedSubsectionShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ReservedSubsection ;
  sh:property [
    sh:path reqvire:subsectionName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] .

reqvire:ElementIdentityShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ElementIdentity ;
  sh:property [
    sh:path reqvire:identitySource ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:identityStability ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] .

reqvire:ReferenceTargetKindShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ReferenceTargetKind ;
  sh:property [
    sh:path reqvire:referenceTargetKindName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Core Element Semantic Contract](Functional/Core/ModelManagement.md#core-element-semantic-contract)
---
