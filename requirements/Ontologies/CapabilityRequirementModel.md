# Elements

### Reqvire Capability Ontology

The Reqvire capability ontology defines capability elements as first-class operational, product, business, regulatory, or system abilities.

Capabilities decompose into child capabilities, attach ontology elements from the ontology plane, derive implementation-facing requirements through `specifiedBy`/`specify`, and may be directly verified. Requirements specify capabilities and own subordinate refinement details/contracts; they do not replace capability ownership of operational meaning, ontology attachment context, or direct verification context.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:Source a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedRefinement ;
  rdfs:comment "Requirement-owned source refinement for stakeholder, regulatory, contractual, policy, or other source context that explains why a requirement exists." .

reqvire:sourceType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "source" ;
  reqvire:elementTypeCategory "refinement" ;
  rdfs:comment "Requirement-owned source context such as stakeholder statements, regulations, policies, standards, contracts, or external obligations." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Requirement Ontology

The Reqvire requirement ontology defines requirement obligations and requirement-owned refinement types.

Requirements are implementation-facing obligations. They can own source context, specifications, constraints, behavior descriptions, state contracts, input-output contracts, shapes-only semantic contracts, and query-backed semantic contracts. Capabilities attach ontology, derive child capabilities, are specified by requirements, and may be verified; capabilities must not own refinement elements through `refinedBy`/`refine`. Requirements are verified by verification elements and may be satisfied by implementation or evidence artifacts.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

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
  rdfs:domain reqvire:Refinement ;
  rdfs:range reqvire:Requirement ;
  rdfs:comment "Links a requirement-owned refinement to the requirement that owns it." .
reqvire:refinementPurpose a owl:DatatypeProperty ;
  rdfs:domain reqvire:Refinement ;
  rdfs:range xsd:string ;
  rdfs:comment "Purpose or intent of a refinement in the requirement model." .
reqvire:allowedRefinementRelation a owl:DatatypeProperty ;
  rdfs:domain reqvire:Refinement ;
  rdfs:range xsd:string ;
  rdfs:comment "Allowed relation used to connect a requirement to a compatible refinement." .
reqvire:requirementObligationText a owl:DatatypeProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range xsd:string ;
  rdfs:comment "Normative obligation text associated with a requirement." .

reqvire:specificationType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "specification" ;
  reqvire:elementTypeCategory "refinement" ;
  rdfs:comment "Requirement-owned detailed specification or technical description." ;
  reqvire:defaultElementType false .
reqvire:constraintType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "constraint" ;
  reqvire:elementTypeCategory "refinement" ;
  rdfs:comment "Requirement-owned limit or boundary on valid system behavior." ;
  reqvire:defaultElementType false .
reqvire:behaviorType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "behavior" ;
  reqvire:elementTypeCategory "refinement" ;
  rdfs:comment "Requirement-owned behavior detail, operational rule, or scenario-specific behavior." ;
  reqvire:defaultElementType false .
reqvire:stateType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "state" ;
  reqvire:elementTypeCategory "refinement" ;
  rdfs:comment "Requirement-owned lifecycle state, state machine, transition, terminal state, or state-dependent contract behavior." ;
  reqvire:defaultElementType false .
reqvire:inputOutputType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "input-output" ;
  reqvire:elementTypeCategory "refinement" ;
  rdfs:comment "Requirement-owned payload, message, document, schema, fixture, or data contract crossing a system or component boundary." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Semantic Contract Ontology

The Reqvire semantic contract ontology defines semantic-contract and semantic-query-contract elements as first-class requirement-owned semantic refinements.

Ontology elements define reusable vocabulary and contain `#### Ontology`. Semantic contracts define SHACL profiles without local `#### Ontology`. Semantic query contracts define declarative graph queries in a generic `#### Query` subsection with one fenced `sparql` block, without local `#### Ontology`, `#### Shapes`, or query-kind classification. The semantic contract element IRI is derived from the Reqvire element id as `urn:reqvire:semantic-contract:<element.id>`. The semantic query contract element IRI is derived from the Reqvire element id as `urn:reqvire:semantic-query-contract:<element.id>`.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:SemanticContract a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedRefinement ;
  rdfs:comment "Requirement-owned refinement that defines SHACL profiles over reachable ontology terms." .
reqvire:Ontology a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "First-class ontology element that defines reusable RDF/OWL vocabulary." .
reqvire:ShapeContract a owl:Class ;
  rdfs:subClassOf reqvire:SemanticContract ;
  rdfs:comment "Requirement-owned semantic contract that defines SHACL shapes over reachable ontology terms." .
reqvire:SemanticQueryContract a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedRefinement ;
  rdfs:comment "Requirement-owned semantic refinement that carries a graph query contract over reachable semantic model context." .
reqvire:semanticContractIri a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticContract ;
  rdfs:range xsd:anyURI ;
  rdfs:comment "Stable RDF IRI assigned to a semantic-contract element." .
reqvire:semanticContractKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticContract ;
  rdfs:range xsd:string ;
  rdfs:comment "Semantic profile category for semantic-contract elements." .
reqvire:ontologyText a owl:DatatypeProperty ;
  rdfs:domain reqvire:Ontology ;
  rdfs:range xsd:string ;
  rdfs:comment "Inline Turtle ontology text carried by an ontology element." .
reqvire:shapesText a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticContract ;
  rdfs:range xsd:string ;
  rdfs:comment "Inline Turtle SHACL shape text carried by a semantic contract." .
reqvire:queryContractIri a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticQueryContract ;
  rdfs:range xsd:anyURI ;
  rdfs:comment "Stable RDF IRI assigned to a semantic-query-contract element." .
reqvire:queryLanguage a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticQueryContract ;
  rdfs:range xsd:string ;
  rdfs:comment "Query language token declared by the Query subsection fenced code info string, such as sparql." .
reqvire:queryText a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticQueryContract ;
  rdfs:range xsd:string ;
  rdfs:comment "Raw query text carried by a semantic query contract." .

reqvire:semanticContractType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "semantic-contract" ;
  reqvire:elementTypeCategory "refinement" ;
  rdfs:comment "Requirement-owned semantic refinement that carries a SHACL shape profile over reachable ontology." ;
  reqvire:defaultElementType false .
reqvire:semanticQueryContractType a reqvire:RefinementElementType ;
  reqvire:elementTypeName "semantic-query-contract" ;
  reqvire:elementTypeCategory "refinement" ;
  rdfs:comment "Requirement-owned semantic refinement that carries a declarative query in a Query subsection over reachable semantic model context." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Requirement Ontology](CapabilityRequirementModel.md#reqvire-requirement-ontology)
---
