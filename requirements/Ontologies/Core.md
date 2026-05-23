# Elements

### Reqvire Core Element Ontology

The Reqvire core element ontology defines the shared base vocabulary for all Reqvire model elements.

This is the foundation ontology used by the rest of the Reqvire ontology set. Other ontology elements and semantic contracts may reference these terms through reachable ontology context.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:Element a owl:Class ;
  rdfs:comment "Base class for addressable Reqvire model elements and related model artifacts." .
reqvire:Feature a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Capability, stakeholder, regulatory, product-area, or domain anchor specified by requirements." .
reqvire:Requirement a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Implementation-facing obligation that can be verified and satisfied by implementation or evidence." .
reqvire:Refinement a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Detail element that refines a feature or requirement with source, semantic, behavioral, structural, or contract information." .
reqvire:Verification a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Evidence or method used to verify a requirement." .
reqvire:Artifact a owl:Class ;
  rdfs:comment "Referenced implementation, evidence, document, or external resource artifact." .
reqvire:ElementType a owl:Class ;
  rdfs:comment "Canonical metadata type value used to classify Reqvire elements." .
reqvire:FeatureElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for feature anchors." .
reqvire:RequirementElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for requirement obligations." .
reqvire:RefinementElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for feature-owned source refinements and requirement-owned refinements." .
reqvire:VerificationElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for verification methods and evidence records." .
reqvire:CustomElementType a owl:Class ;
  rdfs:subClassOf reqvire:ElementType ;
  rdfs:comment "Element type category for custom other-TYPENAME extensions that are trace-only." .
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
  rdfs:comment "Stable semantic category for relation or attachment target resolution." .
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

reqvire:id a owl:DatatypeProperty ;
  rdfs:comment "Stable local element identifier." .
reqvire:identifier a owl:DatatypeProperty ;
  rdfs:comment "Fully qualified Reqvire element identifier including file path and fragment." .
reqvire:name a owl:DatatypeProperty ;
  rdfs:comment "Human-readable element name." .
reqvire:elementType a owl:DatatypeProperty ;
  rdfs:comment "Reqvire element type value declared in element metadata." .
reqvire:filePath a owl:DatatypeProperty ;
  rdfs:comment "Repository-relative file path containing the element." .
reqvire:fragment a owl:DatatypeProperty ;
  rdfs:comment "Markdown fragment used to address the element within its file." .
reqvire:content a owl:DatatypeProperty ;
  rdfs:comment "Collected textual content for the element." .
reqvire:elementTypeName a owl:DatatypeProperty ;
  rdfs:comment "Metadata type token used in Markdown, such as feature, requirement, or semantic-contract." .
reqvire:elementTypeCategory a owl:DatatypeProperty ;
  rdfs:comment "Broad family for a Reqvire element type." .
reqvire:elementTypeDescription a owl:DatatypeProperty ;
  rdfs:comment "Stable semantic meaning of an element type." .
reqvire:defaultElementType a owl:DatatypeProperty ;
  rdfs:comment "Marks the element type assigned when metadata omits type." .
reqvire:customElementTypePattern a owl:DatatypeProperty ;
  rdfs:comment "Pattern used to identify named custom element types." .
reqvire:subsectionName a owl:DatatypeProperty ;
  rdfs:comment "Reserved subsection heading token used in Reqvire Markdown." .
reqvire:subsectionPurpose a owl:DatatypeProperty ;
  rdfs:comment "Stable model purpose of a reserved subsection." .
reqvire:identitySource a owl:DatatypeProperty ;
  rdfs:comment "Source from which an identity or address concept is derived." .
reqvire:identityStability a owl:DatatypeProperty ;
  rdfs:comment "Stability behavior of an identity or address concept under relocation." .
reqvire:referenceTargetKindName a owl:DatatypeProperty ;
  rdfs:comment "Stable reference target category token used by parsers and reports." .
reqvire:referenceTargetKindMeaning a owl:DatatypeProperty ;
  rdfs:comment "Stable meaning of a reference target category." .

reqvire:featureType a reqvire:FeatureElementType ;
  reqvire:elementTypeName "feature" ;
  reqvire:elementTypeCategory "feature" ;
  reqvire:elementTypeDescription "Product, capability, stakeholder, regulatory, external, or domain anchor specified by requirements." ;
  reqvire:defaultElementType false .

reqvire:requirementType a reqvire:RequirementElementType ;
  reqvire:elementTypeName "requirement" ;
  reqvire:elementTypeCategory "requirement" ;
  reqvire:elementTypeDescription "Implementation-facing system obligation verified by verification elements and satisfied by implementation or evidence." ;
  reqvire:defaultElementType true .

reqvire:otherType a reqvire:CustomElementType ;
  reqvire:elementTypeName "other" ;
  reqvire:elementTypeCategory "custom" ;
  reqvire:customElementTypePattern "other-TYPENAME" ;
  reqvire:elementTypeDescription "Trace-only custom element type family for model extensions outside the canonical Reqvire type set." ;
  reqvire:defaultElementType false .

reqvire:elementIdConcept a reqvire:ElementId ;
  reqvire:identitySource "element-name" ;
  reqvire:identityStability "stable-across-relocation" .
reqvire:elementIdentifierConcept a reqvire:ElementIdentifier ;
  reqvire:identitySource "file-path-and-fragment" ;
  reqvire:identityStability "changes-when-file-location-changes" .

reqvire:elementIdentifierReferenceTargetKind a reqvire:ElementIdentifierTarget ;
  reqvire:referenceTargetKindName "element-identifier" ;
  reqvire:referenceTargetKindMeaning "Target resolves to a specific Reqvire model element by location-based identifier." .
reqvire:internalPathReferenceTargetKind a reqvire:InternalPathTarget ;
  reqvire:referenceTargetKindName "internal-path" ;
  reqvire:referenceTargetKindMeaning "Target resolves to a repository-internal path without an element fragment." .
reqvire:externalUrlReferenceTargetKind a reqvire:ExternalUrlTarget ;
  reqvire:referenceTargetKindName "external-url" ;
  reqvire:referenceTargetKindMeaning "Target resolves to an external URL resource." .

reqvire:metadataSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Metadata" ;
  reqvire:subsectionPurpose "Element metadata, element type, governance metadata where valid, and custom metadata." .
reqvire:relationsSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Relations" ;
  reqvire:subsectionPurpose "Authored relation edges from one model element to another model element or artifact." .
reqvire:detailsSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Details" ;
  reqvire:subsectionPurpose "Narrative requirement context and clarification that remains part of the owning element." .
reqvire:attachmentsSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Attachments" ;
  reqvire:subsectionPurpose "Explicit attached ontology or requirement-owned contract dependencies." .
reqvire:conceptReferencesSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Concept References" ;
  reqvire:subsectionPurpose "Human-readable bindings from element prose to reachable ontology terms." .
reqvire:ontologySubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Ontology" ;
  reqvire:subsectionPurpose "Inline Turtle ontology content for ontology elements." .
reqvire:shapesSubsection a reqvire:ReservedSubsection ;
  reqvire:subsectionName "Shapes" ;
  reqvire:subsectionPurpose "Inline SHACL shape content for requirement-owned semantic contracts." .
```

#### Metadata
  * type: ontology
---
