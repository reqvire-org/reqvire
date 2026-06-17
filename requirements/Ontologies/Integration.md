# Elements

### Reqvire AI Assistance Ontology

The Reqvire AI assistance ontology defines the vocabulary for assistant-facing modeling guidance.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:AiAssistanceContract a owl:Class .
reqvire:SkillInstructionContract a owl:Class ;
  rdfs:subClassOf reqvire:AiAssistanceContract .
reqvire:AssistantArtifactContract a owl:Class ;
  rdfs:subClassOf reqvire:AiAssistanceContract .
reqvire:SkillWorkflowContract a owl:Class ;
  rdfs:subClassOf reqvire:SkillInstructionContract .
reqvire:SkillBoundaryContract a owl:Class ;
  rdfs:subClassOf reqvire:SkillInstructionContract .
reqvire:SkillVerificationContract a owl:Class ;
  rdfs:subClassOf reqvire:SkillInstructionContract .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Code Traceability Ontology

The Reqvire code traceability ontology defines the vocabulary for implementation evidence markers in source files.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:CodeTraceabilityContract a owl:Class ;
  rdfs:comment "Contract vocabulary for source-level implementation evidence markers and supported marker parsing categories." .
reqvire:TraceabilityMarkerContract a owl:Class ;
  rdfs:subClassOf reqvire:CodeTraceabilityContract ;
  rdfs:comment "Contract vocabulary for markers that connect source content to model requirements or trace evidence." .
reqvire:CommentStyleContract a owl:Class ;
  rdfs:subClassOf reqvire:CodeTraceabilityContract ;
  rdfs:comment "Contract vocabulary for source comment styles that can carry traceability markers." .
reqvire:TraceabilityRelationKind a owl:Class ;
  rdfs:subClassOf reqvire:TraceabilityMarkerContract ;
  rdfs:comment "Controlled vocabulary entry for a code marker relation token." .
reqvire:CommentStyleKind a owl:Class ;
  rdfs:subClassOf reqvire:CommentStyleContract ;
  rdfs:comment "Controlled vocabulary entry for a supported source comment style token." .

reqvire:traceabilityRelationKindName a owl:DatatypeProperty ;
  rdfs:domain reqvire:TraceabilityRelationKind ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical code marker relation token consumed by source marker parsing and reporting contracts." .
reqvire:commentStyleName a owl:DatatypeProperty ;
  rdfs:domain reqvire:CommentStyleKind ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical supported comment style token consumed by source marker parsing contracts." .

reqvire:satisfiesTraceabilityRelationKind a reqvire:TraceabilityRelationKind ;
  reqvire:traceabilityRelationKindName "satisfies" ;
  rdfs:comment "Code marker relation kind that links implementation content to a requirement it satisfies." .
reqvire:traceTraceabilityRelationKind a reqvire:TraceabilityRelationKind ;
  reqvire:traceabilityRelationKindName "trace" ;
  rdfs:comment "Code marker relation kind that creates documentation traceability without satisfaction semantics." .

reqvire:lineHashCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "hash-line" ;
  rdfs:comment "Single-line hash comment style for traceability markers." .
reqvire:lineSlashCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "slash-line" ;
  rdfs:comment "Single-line slash comment style for traceability markers." .
reqvire:lineDashCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "dash-line" ;
  rdfs:comment "Single-line dash comment style for traceability markers." .
reqvire:batchLineCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "batch-line" ;
  rdfs:comment "Batch-style line comment category for traceability markers." .
reqvire:blockCssCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "css-block" ;
  rdfs:comment "CSS-style block comment wrapper for traceability markers." .
reqvire:blockXmlCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "xml-block" ;
  rdfs:comment "XML/HTML-style block comment wrapper for traceability markers." .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire GitHub Workflow Ontology

The Reqvire GitHub workflow ontology defines the vocabulary for hosted repository automation behavior.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:RepositoryWorkflowContract a owl:Class .
reqvire:ExplorerServeContract a owl:Class ;
  rdfs:subClassOf reqvire:RepositoryWorkflowContract .
reqvire:PullRequestValidationContract a owl:Class ;
  rdfs:subClassOf reqvire:RepositoryWorkflowContract .
reqvire:ChangeLogContract a owl:Class ;
  rdfs:subClassOf reqvire:RepositoryWorkflowContract .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Source Marker Traceability Shape

Defines SHACL constraints for code traceability relation kinds and supported comment style vocabulary.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:TraceabilityRelationKindShape
  a sh:NodeShape ;
  sh:targetClass reqvire:TraceabilityRelationKind ;
  sh:property [
    sh:path reqvire:traceabilityRelationKindName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("satisfies" "trace") ;
    sh:message "Traceability relation kinds must use a supported code marker relation token." ;
  ] .

reqvire:CommentStyleKindShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CommentStyleKind ;
  sh:property [
    sh:path reqvire:commentStyleName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ("hash-line" "slash-line" "dash-line" "batch-line" "css-block" "xml-block") ;
    sh:message "Comment style kinds must use a supported source marker comment style token." ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * constrain: [Code Traceability](../Integration/CodeAlignment/CodeAlignmentRequirements.md#code-traceability)
  * use: [Reqvire Code Traceability Ontology](#reqvire-code-traceability-ontology)
---
