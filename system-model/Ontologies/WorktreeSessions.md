# Elements

### Reqvire Worktree Session Ontology

Defines the structural vocabulary for independently addressed worktree contexts, accepted and observed commit provenance, accepted model snapshots, and the input dependencies governing a HEAD refresh.

#### Details
Competency questions:
- Which worktree and accepted model snapshot belong to a context?
- Which commit is accepted, and which commit is currently observed?
- Which input observations protect model content or source resolution?
- Which referenced artifacts have existence dependencies without frozen content?
- Which context is the subject of a HEAD refresh?

Command names, eligibility algorithms, blocker codes and publication transitions belong to requirement-owned contracts. The vocabulary describes those domain distinctions without imposing an operational state machine or claiming runtime session instances are already exported.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix concept: <https://www.reqvire.org/concepts#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:WorktreeContext a owl:Class ;
  reqvire:mapsToConcept concept:MCPWorktreeContext ;
  rdfs:comment "An independently addressed accepted-model session rooted in one Git worktree." .
reqvire:GitCommit a owl:Class ;
  rdfs:comment "A Git commit that can be accepted or observed by a worktree context." .
reqvire:AcceptedModelSnapshot a owl:Class ;
  rdfs:comment "A complete validated model retained as authoritative by a context." .
reqvire:ModelInputDependency a owl:Class ;
  rdfs:comment "A content, existence or source-selection observation on which an accepted model depends." .
reqvire:ProtectedModelInput a owl:Class ;
  rdfs:subClassOf reqvire:ModelInputDependency ;
  reqvire:mapsToConcept concept:ProtectedModelInput ;
  rdfs:comment "A currently eligible model source, or a semantic source or resolution observation with a currently eligible declaring owner, whose change requires full validated admission." .
reqvire:ReferenceTargetDependency a owl:Class ;
  rdfs:subClassOf reqvire:ModelInputDependency ;
  rdfs:comment "A declared local reference's dependency on target existence and eligibility, independent of the target's content role." .
reqvire:SourceResolutionDependency a owl:Class ;
  rdfs:subClassOf reqvire:ProtectedModelInput ;
  rdfs:comment "An observation of a candidate path, including absence, that determines which local semantic source is selected." .
reqvire:HeadRefresh a owl:Class ;
  reqvire:mapsToConcept concept:HEADRefresh ;
  rdfs:comment "A guarded reload of an observed committed branch tip through normal model loading." .

reqvire:contextWorktree a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ;
  rdfs:range reqvire:GitWorktree .
reqvire:acceptedHeadCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ;
  rdfs:range reqvire:GitCommit ;
  reqvire:mapsToConcept concept:AcceptedHEAD .
reqvire:observedHeadCommit a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ;
  rdfs:range reqvire:GitCommit ;
  reqvire:mapsToConcept concept:ObservedHEAD .
reqvire:acceptedModelSnapshot a owl:ObjectProperty ;
  rdfs:domain reqvire:WorktreeContext ;
  rdfs:range reqvire:AcceptedModelSnapshot .
reqvire:modelInputDependency a owl:ObjectProperty ;
  rdfs:domain reqvire:AcceptedModelSnapshot ;
  rdfs:range reqvire:ModelInputDependency .
reqvire:refreshContext a owl:ObjectProperty ;
  rdfs:domain reqvire:HeadRefresh ;
  rdfs:range reqvire:WorktreeContext .
reqvire:dependencyPath a owl:DatatypeProperty ;
  rdfs:domain reqvire:ModelInputDependency ;
  rdfs:range xsd:string .
reqvire:commitObjectId a owl:DatatypeProperty ;
  rdfs:domain reqvire:GitCommit ;
  rdfs:range xsd:string .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
