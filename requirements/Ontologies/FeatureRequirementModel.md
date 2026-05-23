# Elements

### Reqvire Feature Ontology

The Reqvire feature ontology defines feature anchors and feature-owned source refinements.

Features own source refinements and attach ontology elements from the ontology plane. Requirements specify features; they do not replace feature ownership of source context or feature-level ontology attachment context.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:FeatureOwnedRefinement a owl:Class ;
  rdfs:subClassOf reqvire:Refinement ;
  rdfs:comment "Refinement class whose instances are owned by feature elements through refine/refinedBy." .
reqvire:Source a owl:Class ;
  rdfs:subClassOf reqvire:FeatureOwnedRefinement ;
  rdfs:comment "Feature-owned source refinement for stakeholder, regulatory, contractual, policy, or other source context that explains why a feature exists." .

reqvire:sourceType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "source" ;
  reqvire:elementTypeCategory "feature-refinement" ;
  reqvire:elementTypeDescription "Feature-owned source context such as stakeholder statements, regulations, policies, standards, contracts, or external obligations." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Requirement Ontology

The Reqvire requirement ontology defines requirement obligations and requirement-owned refinement types.

Requirements are implementation-facing obligations. They can own specifications, constraints, behavior descriptions, state contracts, input-output contracts, and shapes-only semantic contracts. They are verified by verification elements and may be satisfied by implementation or evidence artifacts.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:RequirementOwnedRefinement a owl:Class ;
  rdfs:subClassOf reqvire:Refinement ;
  rdfs:comment "Refinement class whose instances are owned by requirement elements through refine/refinedBy." .
reqvire:Specification a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedRefinement ;
  rdfs:comment "Requirement-owned refinement for detailed specifications and technical descriptions." .
reqvire:Constraint a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedRefinement ;
  rdfs:comment "Requirement-owned refinement that limits or bounds valid system behavior." .
reqvire:Behavior a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedRefinement ;
  rdfs:comment "Requirement-owned refinement that describes behavior details, operational rules, or scenario-specific behavior." .
reqvire:State a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedRefinement ;
  rdfs:comment "Requirement-owned refinement for lifecycle states, state machines, transitions, terminal states, and state-dependent contract behavior." .
reqvire:InputOutput a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedRefinement ;
  rdfs:comment "Requirement-owned refinement for payloads, messages, documents, schemas, fixtures, and data contracts crossing system or component boundaries." .

reqvire:ownedByRequirement a owl:ObjectProperty ;
  rdfs:comment "Links a requirement-owned refinement to the requirement that owns it." .
reqvire:refinementPurpose a owl:DatatypeProperty ;
  rdfs:comment "Purpose or intent of a refinement in the requirement model." .
reqvire:allowedRefinementRelation a owl:DatatypeProperty ;
  rdfs:comment "Allowed relation used to connect a requirement to a compatible refinement." .
reqvire:requirementObligationText a owl:DatatypeProperty ;
  rdfs:comment "Normative obligation text associated with a requirement." .

reqvire:specificationType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "specification" ;
  reqvire:elementTypeCategory "requirement-refinement" ;
  reqvire:elementTypeDescription "Requirement-owned detailed specification or technical description." ;
  reqvire:defaultElementType false .
reqvire:constraintType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "constraint" ;
  reqvire:elementTypeCategory "requirement-refinement" ;
  reqvire:elementTypeDescription "Requirement-owned limit or boundary on valid system behavior." ;
  reqvire:defaultElementType false .
reqvire:behaviorType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "behavior" ;
  reqvire:elementTypeCategory "requirement-refinement" ;
  reqvire:elementTypeDescription "Requirement-owned behavior detail, operational rule, or scenario-specific behavior." ;
  reqvire:defaultElementType false .
reqvire:stateType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "state" ;
  reqvire:elementTypeCategory "requirement-refinement" ;
  reqvire:elementTypeDescription "Requirement-owned lifecycle state, state machine, transition, terminal state, or state-dependent contract behavior." ;
  reqvire:defaultElementType false .
reqvire:inputOutputType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "input-output" ;
  reqvire:elementTypeCategory "requirement-refinement" ;
  reqvire:elementTypeDescription "Requirement-owned payload, message, document, schema, fixture, or data contract crossing a system or component boundary." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Semantic Contract Ontology

The Reqvire semantic contract ontology defines semantic-contract elements as first-class requirement-owned refinements.

Ontology elements define reusable vocabulary and contain `#### Ontology`. Requirement-owned semantic contracts define obligation-specific SHACL profiles without local `#### Ontology`. The semantic contract element IRI is derived from the Reqvire element id as `urn:reqvire:semantic-contract:<element.id>`.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:SemanticContract a owl:Class ;
  rdfs:subClassOf reqvire:Refinement ;
  rdfs:comment "Requirement-owned refinement that defines SHACL profiles over reachable ontology terms." .
reqvire:Ontology a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "First-class ontology element that defines reusable RDF/OWL vocabulary." .
reqvire:ShapeContract a owl:Class ;
  rdfs:subClassOf reqvire:SemanticContract ;
  rdfs:comment "Requirement-owned semantic contract that defines SHACL shapes over reachable ontology terms." .
reqvire:semanticContractIri a owl:DatatypeProperty ;
  rdfs:comment "Stable RDF IRI assigned to a semantic-contract element." .
reqvire:semanticContractKind a owl:DatatypeProperty ;
  rdfs:comment "Semantic profile category for requirement-owned semantic-contract elements." .
reqvire:ontologyText a owl:DatatypeProperty ;
  rdfs:comment "Inline Turtle ontology text carried by an ontology element." .
reqvire:shapesText a owl:DatatypeProperty ;
  rdfs:comment "Inline Turtle SHACL shape text carried by a semantic contract." .

reqvire:semanticContractType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "semantic-contract" ;
  reqvire:elementTypeCategory "requirement-refinement" ;
  reqvire:elementTypeDescription "Requirement-owned semantic refinement that carries a SHACL shape profile over reachable ontology." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
