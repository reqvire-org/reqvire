# Elements

### Capability Structure and Relation Shape

Defines SHACL constraints for capability structure, concept-reference semantics, specification, verification, and forbidden implementation satisfaction or contract ownership edges.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CapabilityShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Capability ;
  sh:property [
    sh:path reqvire:derive ;
    sh:class reqvire:Capability ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:class reqvire:Capability ;
  ] ;
  sh:property [
    sh:path reqvire:definedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:class reqvire:Verification ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:define ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specify ;
    sh:maxCount 0 ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Ontology and Semantic Contract Model](../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * use: [Reqvire Capability Ontology](#reqvire-capability-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### Contract Ownership Shape

Defines SHACL constraints for concrete contract subtype ownership.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .

reqvire:ContractShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Contract ;
  sh:property [
    sh:path reqvire:define ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] .

reqvire:SourceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Source ;
  sh:property [
    sh:path reqvire:define ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] .

reqvire:SpecificationShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Specification ;
  sh:property [
    sh:path reqvire:define ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] .

reqvire:ConstraintShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Constraint ;
  sh:property [
    sh:path reqvire:define ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] .

reqvire:BehaviorShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Behavior ;
  sh:property [
    sh:path reqvire:define ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] .

reqvire:StateShape
  a sh:NodeShape ;
  sh:targetClass reqvire:State ;
  sh:property [
    sh:path reqvire:define ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] .

reqvire:InputOutputShape
  a sh:NodeShape ;
  sh:targetClass reqvire:InputOutput ;
  sh:property [
    sh:path reqvire:define ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Contract Element Structure Constraints](../ModelStructure/ModelManagement.md#contract-element-structure-constraints)
  * use: [Reqvire Capability Ontology](#reqvire-capability-ontology)
  * use: [Reqvire Requirement Ontology](#reqvire-requirement-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### Custom Element Semantic Boundary Shape

Defines SHACL constraints for custom `other-*` elements as extension nodes that cannot author canonical semantic relations.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CustomElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CustomElement ;
  sh:property [
    sh:path reqvire:elementType ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:pattern "^other-.+" ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:define ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:definedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verify ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:reusesContract ;
    sh:maxCount 0 ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Contract Element Structure Constraints](../ModelStructure/ModelManagement.md#contract-element-structure-constraints)
  * use: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### Ontology Element Vocabulary Shape

Defines SHACL constraints for ontology elements as vocabulary-bearing graph nodes.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:OntologyElementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Ontology ;
  sh:property [
    sh:path reqvire:ontologyText ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:shapesText ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:reusesContract ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:class reqvire:Ontology ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:class reqvire:Ontology ;
  ] ;
  sh:property [
    sh:path reqvire:definedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:maxCount 0 ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Ontology and Semantic Contract Model](../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * use: [Reqvire Semantic Contract Ontology](#reqvire-semantic-contract-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### Requirement Ownership and Coverage Shape

Defines SHACL constraints for requirement ownership, hierarchy, contracts, verification, and capability specification.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:RequirementShape
  a sh:NodeShape ;
  sh:targetClass reqvire:Requirement ;
  sh:or (
    [
      sh:property [
        sh:path reqvire:specify ;
        sh:minCount 1 ;
        sh:class reqvire:Capability ;
      ]
    ]
    [
      sh:property [
        sh:path reqvire:derivedFrom ;
        sh:minCount 1 ;
        sh:class reqvire:Requirement ;
      ]
    ]
  ) ;
  sh:property [
    sh:path reqvire:specify ;
    sh:class reqvire:Capability ;
  ] ;
  sh:property [
    sh:path reqvire:derivedFrom ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:derive ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:definedBy ;
    sh:class reqvire:Contract ;
  ] ;
  sh:property [
    sh:path reqvire:verifiedBy ;
    sh:class reqvire:Verification ;
  ] ;
  sh:property [
    sh:path reqvire:satisfiedBy ;
    sh:class reqvire:Artifact ;
  ] ;
  sh:property [
    sh:path reqvire:requirementObligationText ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:satisfy ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:specifiedBy ;
    sh:maxCount 0 ;
  ] .

reqvire:RequirementOwnedContractShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RequirementOwnedContract ;
  sh:property [
    sh:path reqvire:define ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:class reqvire:Requirement ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Ontology and Semantic Contract Model](../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * use: [Reqvire Requirement Ontology](#reqvire-requirement-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### Reqvire Capability Ontology

The Reqvire capability ontology defines capability elements as first-class operational, product, business, regulatory, or system abilities.

Capabilities decompose into child capabilities, bind vocabulary through concept references, derive implementation-facing requirements through `specifiedBy`/`specify`, and may be directly verified. Requirements specify capabilities and own subordinate contract details/contracts; they do not replace capability ownership of operational meaning, concept-reference context, or direct verification context.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:Source a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedContract ;
  rdfs:comment "Requirement-owned source contract for stakeholder, regulatory, contractual, policy, or other source context that explains why a requirement exists." .

reqvire:sourceType a reqvire:ContractElementType ;
  reqvire:elementTypeName "source" ;
  reqvire:elementTypeCategory "contract" ;
  rdfs:comment "Requirement-owned source context such as stakeholder statements, regulations, policies, standards, contracts, or external obligations." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Requirement Ontology

The Reqvire requirement ontology defines requirement obligations and requirement-owned contract types.

Requirements are implementation-facing obligations. They can own source context, specifications, constraints, behavior descriptions, state contracts, and input-output contracts. Requirements may also be constrained by reusable shapes-only semantic contracts. Capabilities bind ontology terms through concept references, derive child capabilities, are specified by requirements, and may be verified; capabilities must not own contract elements through `definedBy`/`define`. Requirements are verified by verification elements and may be satisfied by implementation or evidence artifacts.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:RequirementOwnedContract a owl:Class ;
  rdfs:subClassOf reqvire:Contract ;
  rdfs:comment "Contract class whose instances are owned by requirement elements through define/definedBy." .
reqvire:Specification a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedContract ;
  rdfs:comment "Requirement-owned contract for detailed specifications and technical descriptions." .
reqvire:Constraint a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedContract ;
  rdfs:comment "Requirement-owned contract that limits or bounds valid system behavior." .
reqvire:Behavior a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedContract ;
  rdfs:comment "Requirement-owned contract that describes behavior details, operational rules, or scenario-specific behavior." .
reqvire:State a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedContract ;
  rdfs:comment "Requirement-owned contract for lifecycle states, state machines, transitions, terminal states, and state-dependent contract behavior." .
reqvire:InputOutput a owl:Class ;
  rdfs:subClassOf reqvire:RequirementOwnedContract ;
  rdfs:comment "Requirement-owned contract for payloads, messages, documents, schemas, fixtures, and data contracts crossing system or component boundaries." .

reqvire:ownedByRequirement a owl:ObjectProperty ;
  rdfs:domain reqvire:Contract ;
  rdfs:range reqvire:Requirement ;
  rdfs:comment "Links a requirement-owned contract to the requirement that owns it." .
reqvire:contractPurpose a owl:DatatypeProperty ;
  rdfs:domain reqvire:Contract ;
  rdfs:range xsd:string ;
  rdfs:comment "Purpose or intent of a contract in the requirement model." .
reqvire:allowedContractRelation a owl:DatatypeProperty ;
  rdfs:domain reqvire:Contract ;
  rdfs:range xsd:string ;
  rdfs:comment "Allowed relation used to connect a requirement to a compatible contract." .
reqvire:requirementObligationText a owl:DatatypeProperty ;
  rdfs:domain reqvire:Requirement ;
  rdfs:range xsd:string ;
  rdfs:comment "Normative obligation text associated with a requirement." .

reqvire:specificationType a reqvire:ContractElementType ;
  reqvire:elementTypeName "specification" ;
  reqvire:elementTypeCategory "contract" ;
  rdfs:comment "Requirement-owned detailed specification or technical description." ;
  reqvire:defaultElementType false .
reqvire:constraintType a reqvire:ContractElementType ;
  reqvire:elementTypeName "constraint" ;
  reqvire:elementTypeCategory "contract" ;
  rdfs:comment "Requirement-owned limit or boundary on valid system behavior." ;
  reqvire:defaultElementType false .
reqvire:behaviorType a reqvire:ContractElementType ;
  reqvire:elementTypeName "behavior" ;
  reqvire:elementTypeCategory "contract" ;
  rdfs:comment "Requirement-owned behavior detail, operational rule, or scenario-specific behavior." ;
  reqvire:defaultElementType false .
reqvire:stateType a reqvire:ContractElementType ;
  reqvire:elementTypeName "state" ;
  reqvire:elementTypeCategory "contract" ;
  rdfs:comment "Requirement-owned lifecycle state, state machine, transition, terminal state, or state-dependent contract behavior." ;
  reqvire:defaultElementType false .
reqvire:inputOutputType a reqvire:ContractElementType ;
  reqvire:elementTypeName "input-output" ;
  reqvire:elementTypeCategory "contract" ;
  rdfs:comment "Requirement-owned payload, message, document, schema, fixture, or data contract crossing a system or component boundary." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Semantic Contract Ontology

The Reqvire semantic contract ontology defines semantic-contract elements as reusable SHACL constraint contracts.

Ontology elements define reusable vocabulary and contain `#### Ontology`. Semantic contracts define SHACL profiles without local `#### Ontology`. The semantic contract element IRI is derived from the Reqvire element id as `urn:reqvire:semantic-contract:<element.id>`.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:SemanticContract a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "Reusable semantic contract that defines SHACL profiles over ontology terms reached through explicit use relations." .
reqvire:Ontology a owl:Class ;
  rdfs:subClassOf reqvire:Element ;
  rdfs:comment "First-class ontology element that defines reusable RDF/OWL vocabulary." .
reqvire:ShapeContract a owl:Class ;
  rdfs:subClassOf reqvire:SemanticContract ;
  rdfs:comment "Semantic contract that defines SHACL shapes over ontology terms reached through explicit use relations." .
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
reqvire:ontologyBase a owl:DatatypeProperty ;
  rdfs:domain owl:Ontology ;
  rdfs:range xsd:anyURI ;
  rdfs:comment "Canonical ontology document IRI base resolved from the root ontology element metadata." .
reqvire:ontologyPrefix a owl:DatatypeProperty ;
  rdfs:domain owl:Ontology ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical CURIE prefix label resolved from the root ontology element metadata." .
reqvire:termNamespace a owl:DatatypeProperty ;
  rdfs:domain owl:Ontology ;
  rdfs:range xsd:anyURI ;
  rdfs:comment "Hash namespace used for terms contributed to a generated ontology document." .
reqvire:ontologyElement a owl:ObjectProperty ;
  rdfs:domain owl:Ontology ;
  rdfs:range reqvire:Ontology ;
  rdfs:comment "Reqvire ontology element that contributes authored vocabulary to the generated ontology document." .
reqvire:shapesText a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticContract ;
  rdfs:range xsd:string ;
  rdfs:comment "Inline Turtle SHACL shape text carried by a semantic contract." .

reqvire:semanticContractType a reqvire:SemanticContractElementType ;
  reqvire:elementTypeName "semantic-contract" ;
  reqvire:elementTypeCategory "semantic-contract" ;
  rdfs:comment "Reusable semantic-contract element that carries a SHACL shape profile over explicitly used ontology." ;
  reqvire:defaultElementType false .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Requirement Ontology](#reqvire-requirement-ontology)
---

### Semantic Contract Structure Shape

Defines SHACL constraints for semantic-contract identity, profile kind, shape content, constrained requirements, and explicit ontology use.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:SemanticContractShape
  a sh:NodeShape ;
  sh:targetClass reqvire:SemanticContract ;
  sh:property [
    sh:path reqvire:semanticContractIri ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:anyURI ;
  ] ;
  sh:property [
    sh:path reqvire:semanticContractKind ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("semantic-contract") ;
  ] ;
  sh:property [
    sh:path reqvire:ontologyText ;
    sh:maxCount 0 ;
  ] ;
  sh:property [
    sh:path reqvire:shapesText ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:constrain ;
    sh:minCount 1 ;
    sh:class reqvire:Requirement ;
  ] ;
  sh:property [
    sh:path reqvire:use ;
    sh:minCount 1 ;
    sh:class reqvire:Ontology ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Ontology and Semantic Contract Model](../ModelStructure/ModelManagement.md#ontology-and-semantic-contract-model)
  * use: [Reqvire Semantic Contract Ontology](#reqvire-semantic-contract-ontology)
  * use: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

