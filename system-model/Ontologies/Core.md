# Elements

### Element Identity and Metadata Shape

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
  * constrain: [Ontology and Semantic Contract Model](../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * use: [Reqvire Core Element Ontology](#reqvire-core-element-ontology)
---

### Reqvire Core Element Ontology

The Reqvire core element ontology defines the shared base vocabulary for all Reqvire model elements.

This is the foundation ontology used by the rest of the Reqvire ontology set. Other ontology elements may derive from this vocabulary, and semantic contracts may reference these terms through explicit `use`/`usedBy` ontology context.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix concept: <https://www.reqvire.org/concepts#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

<https://www.reqvire.org/ontology> a owl:Ontology ;
  rdfs:label "Reqvire ontology" ;
  rdfs:comment "Canonical ontology document for same-base Reqvire ontology elements; derived ontology elements contribute vocabulary to this document." .

reqvire:Element a owl:Class ;
  reqvire:mapsToConcept concept:Element ;
  rdfs:comment "Base class for addressable Reqvire model elements and related model artifacts. Full semantic model exports emit concrete parsed model elements as owl:NamedIndividual instances of this class and their more specific Reqvire element class." .
reqvire:Capability a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  reqvire:mapsToConcept concept:Capability ;
  rdfs:comment "Coherent operational, product, business, regulatory, or system ability that bridges ontology, requirements, and verification." .
reqvire:Requirement a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  reqvire:mapsToConcept concept:Requirement ;
  rdfs:comment "Implementation-facing obligation that can be verified and satisfied by implementation or evidence." .
reqvire:Contract a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  reqvire:mapsToConcept concept:RequirementContract ;
  rdfs:comment "Requirement-owned detail element that defines a requirement with source, behavioral, structural, query, or contract information." .
reqvire:ConceptScheme a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Markdown-native Reqvire element that authors a SKOS concept scheme or thesaurus boundary." .
reqvire:Concept a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Markdown-native Reqvire element that authors one curated SKOS concept." .
reqvire:Verification a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  reqvire:mapsToConcept concept:ConcreteVerification ;
  rdfs:comment "Evidence or method used to verify a requirement." .
reqvire:Artifact a owl:Class ;
  rdfs:comment "Referenced implementation, evidence, document, or external resource artifact. Full semantic model exports emit concrete referenced artifacts as owl:NamedIndividual instances of this class and any more specific artifact class such as reqvire:File." .
reqvire:RuntimeOntologyArtifact a owl:Class ;
  rdfs:subClassOf reqvire:Artifact ;
  reqvire:mapsToConcept concept:RuntimeOntologyArtifact ;
  rdfs:comment "Generated runtime ontology snapshot embedded by Reqvire core and derived from authored ontology source." .
reqvire:RuntimeOntologyNamespace a owl:Class ;
  reqvire:mapsToConcept concept:RuntimeOntologyNamespace ;
  rdfs:comment "Term namespace selected as the Reqvire runtime vocabulary boundary for generated runtime ontology artifacts." .
reqvire:NamespaceScopedOntologyExport a owl:Class ;
  reqvire:mapsToConcept concept:NamespaceScopedOntologyExport ;
  rdfs:comment "Clean authored semantic export filtered to one ontology base or term namespace." .
reqvire:OntologySourceOfTruth a owl:Class ;
  reqvire:mapsToConcept concept:OntologySourceOfTruth ;
  rdfs:comment "Authored ontology source model that governs generated runtime ontology artifacts." .
reqvire:OntologyDocument a owl:Class ;
  reqvire:mapsToConcept concept:OntologyDocument ;
  rdfs:comment "Generated or authored ontology document identified by ontology base IRI and containing contributed ontology terms." .
reqvire:File a owl:Class ;
  rdfs:subClassOf reqvire:Artifact ;
  rdfs:comment "Repository-internal file artifact referenced by Reqvire relations, contract_bindings, or evidence links." .
reqvire:CustomElement a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Custom model extension element whose authored metadata type follows other-TYPENAME and cannot author canonical semantic relations." .
reqvire:ElementType a owl:Class ;
  reqvire:mapsToConcept concept:Element ;
  rdfs:comment "Canonical metadata type value used to classify Reqvire elements." .
reqvire:CapabilityElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for capability elements." .
reqvire:RequirementElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for requirement obligations." .
reqvire:ContractElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for requirement-owned contracts." .
reqvire:ConceptElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for Markdown-native SKOS concept schemes and concepts." .
reqvire:SemanticContractElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for reusable semantic-contract SHACL profile elements." .
reqvire:VerificationElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for verification methods and evidence records." .
reqvire:CustomElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for custom other-TYPENAME extensions that cannot author canonical semantic relations." .
reqvire:ElementIdentity a owl:Class ;
  rdfs:comment "Stable identity concept used to track an element independently of its current file location." .
reqvire:ElementId a owl:Class ;
  rdfs:subClassOf reqvire:ElementIdentity ;
  rdfs:comment "Stable identifier derived from element name and used for identity tracking across relocations." .
reqvire:ElementIdentifier a owl:Class ;
  rdfs:subClassOf reqvire:ElementIdentity ;
  rdfs:comment "Location-based address that combines file path and fragment for authored references." .
reqvire:ElementLocation a owl:Class ;
  rdfs:comment "File and section location context for an element, separate from stable identity." .
reqvire:ReferenceTargetKind a owl:Class ;
  rdfs:comment "Stable semantic category for relation or contract_bindings target resolution." .
reqvire:ElementIdentifierTarget a owl:Class ;
  rdfs:subClassOf reqvire:ReferenceTargetKind ;
  rdfs:comment "Reference target category that resolves to a specific model element." .
reqvire:InternalPathTarget a owl:Class ;
  rdfs:subClassOf reqvire:ReferenceTargetKind ;
  rdfs:comment "Reference target category that resolves to a repository-internal file path." .
reqvire:ExternalUrlTarget a owl:Class ;
  rdfs:subClassOf reqvire:ReferenceTargetKind ;
  rdfs:comment "Reference target category that resolves to an external URL." .
reqvire:ReservedSubsection a owl:Class ;
  rdfs:comment "Reserved level-four subsection name with parser-recognized model meaning." .

reqvire:mapsToConcept a owl:AnnotationProperty ;
  rdfs:range skos:Concept ;
  rdfs:label "maps to concept" ;
  rdfs:comment "Links a structural ontology term or model resource to a curated SKOS concept entity without implying OWL equivalence, SKOS mapping semantics, or subject membership in skos:Concept." .

reqvire:id a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable local element identifier." .
reqvire:identifier a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Fully qualified Reqvire element identifier including file path and fragment." .
reqvire:name a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Human-readable element name." .
reqvire:elementType a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Reqvire element type value declared in element metadata." .
reqvire:filePath a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Repository-relative file path containing the element." .
reqvire:externalUrl a owl:DatatypeProperty ;
  rdfs:domain reqvire:Artifact ;
  rdfs:range xsd:anyURI ;
  rdfs:comment "External URL referenced by a Reqvire artifact relation target." .
reqvire:fragment a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Markdown fragment used to address the element within its file." .
reqvire:content a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Collected textual content for the element." .
reqvire:elementTypeName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ElementType ;
  rdfs:range xsd:string ;
  rdfs:comment "Metadata type token used in Markdown, such as capability, requirement, or semantic-contract." .
reqvire:elementTypeCategory a owl:DatatypeProperty ;
  rdfs:domain reqvire:ElementType ;
  rdfs:range xsd:string ;
  rdfs:comment "Broad family for a Reqvire element type." .
reqvire:elementTypeDescription a owl:DatatypeProperty ;
  rdfs:domain reqvire:ElementType ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable semantic meaning of an element type." .
reqvire:defaultElementType a owl:DatatypeProperty ;
  rdfs:domain reqvire:ElementType ;
  rdfs:range xsd:boolean ;
  rdfs:comment "Marks the element type assigned when metadata omits type." .
reqvire:customElementTypePattern a owl:DatatypeProperty ;
  rdfs:domain reqvire:CustomElementType ;
  rdfs:range xsd:string ;
  rdfs:comment "Pattern used to identify named custom element types." .
reqvire:subsectionName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ReservedSubsection ;
  rdfs:range xsd:string ;
  rdfs:comment "Reserved subsection heading token used in Reqvire Markdown." .
reqvire:identitySource a owl:DatatypeProperty ;
  rdfs:domain reqvire:ElementIdentity ;
  rdfs:range xsd:string ;
  rdfs:comment "Source from which an identity or address concept is derived." .
reqvire:identityStability a owl:DatatypeProperty ;
  rdfs:domain reqvire:ElementIdentity ;
  rdfs:range xsd:string ;
  rdfs:comment "Stability behavior of an identity or address concept under relocation." .
reqvire:referenceTargetKindName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ReferenceTargetKind ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable reference target category token used by parsers and model queries." .
reqvire:capabilityType a reqvire:CapabilityElementType ;
  reqvire:elementTypeName "capability" ;
  reqvire:elementTypeCategory "capability" ;
  rdfs:comment "Implementation-independent operational, product, business, regulatory, or system ability specified by requirements, connected to ontology through contract_bindings, and verified by evidence." ;
  reqvire:defaultElementType false .

reqvire:requirementType a reqvire:RequirementElementType ;
  reqvire:elementTypeName "requirement" ;
  reqvire:elementTypeCategory "requirement" ;
  rdfs:comment "Implementation-facing system obligation verified by verification elements and satisfied by implementation or evidence." ;
  reqvire:defaultElementType true .

reqvire:conceptSchemeType a reqvire:ConceptElementType ;
  reqvire:elementTypeName "concept-scheme" ;
  reqvire:elementTypeCategory "concept" ;
  rdfs:comment "Markdown-native SKOS concept scheme or thesaurus boundary." ;
  reqvire:defaultElementType false .

reqvire:conceptType a reqvire:ConceptElementType ;
  reqvire:elementTypeName "concept" ;
  reqvire:elementTypeCategory "concept" ;
  rdfs:comment "Markdown-native SKOS concept authored from Reqvire sections and relations." ;
  reqvire:defaultElementType false .

reqvire:otherType a reqvire:CustomElementType ;
  reqvire:elementTypeName "other" ;
  reqvire:elementTypeCategory "custom" ;
  reqvire:customElementTypePattern "other-TYPENAME" ;
  rdfs:comment "Trace-only custom element type family for model extensions outside the canonical Reqvire type set." ;
  reqvire:defaultElementType false .

reqvire:elementIdConcept a reqvire:ElementId ;
  reqvire:identitySource "element-name" ;
  reqvire:identityStability "stable-across-relocation" .
reqvire:elementIdentifierConcept a reqvire:ElementIdentifier ;
  reqvire:identitySource "file-path-and-fragment" ;
  reqvire:identityStability "changes-when-file-location-changes" .

reqvire:elementIdentifierReferenceTargetKind a reqvire:ElementIdentifierTarget ;
  reqvire:referenceTargetKindName "element-identifier" ;
  rdfs:comment "Target resolves to a specific Reqvire model element by location-based identifier." .
reqvire:internalPathReferenceTargetKind a reqvire:InternalPathTarget ;
  reqvire:referenceTargetKindName "internal-path" ;
  rdfs:comment "Target resolves to a repository-internal path without an element fragment." .
reqvire:externalUrlReferenceTargetKind a reqvire:ExternalUrlTarget ;
  reqvire:referenceTargetKindName "external-url" ;
  rdfs:comment "Target resolves to an external URL resource." .

reqvire:metadataSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Metadata" ;
  rdfs:comment "Element metadata, element type, governance metadata where valid, and custom metadata." .
reqvire:relationsSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Relations" ;
  rdfs:comment "Authored relation edges from one model element to another model element or artifact." .
reqvire:detailsSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Details" ;
  rdfs:comment "Narrative requirement context and clarification that remains part of the owning element." .
reqvire:contractBindingsSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Contract Bindings" ;
  rdfs:comment "Explicit cross-subgraph reuse of requirement-owned contract context." .
reqvire:conceptReferencesSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Concept References" ;
  reqvire:mapsToConcept concept:ConceptReference ;
  rdfs:comment "Human-readable bindings from non-ontology, non-semantic-contract element prose to generated native SKOS concepts." .
reqvire:ontologySubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Ontology" ;
  rdfs:comment "Inline Turtle ontology content for ontology elements." .
reqvire:shapesSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Shapes" ;
  rdfs:comment "Inline SHACL shape content for reusable semantic contracts." .
```

#### Metadata
  * type: ontology
  * ontology_base: https://www.reqvire.org/ontology
  * ontology_prefix: reqvire
---
