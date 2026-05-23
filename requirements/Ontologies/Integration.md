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

reqvire:CodeTraceabilityContract a owl:Class .
reqvire:TraceabilityMarkerContract a owl:Class .
reqvire:CommentStyleContract a owl:Class .
reqvire:TraceabilityRelationKind a owl:Class .
reqvire:CommentStyleKind a owl:Class .

reqvire:traceabilityRelationKindName a owl:DatatypeProperty .
reqvire:traceabilityRelationKindMeaning a owl:DatatypeProperty .
reqvire:commentStyleName a owl:DatatypeProperty .
reqvire:commentStyleMeaning a owl:DatatypeProperty .

reqvire:satisfiesTraceabilityRelationKind a reqvire:TraceabilityRelationKind ;
  reqvire:traceabilityRelationKindName "satisfies" ;
  reqvire:traceabilityRelationKindMeaning "Code marker relation kind that links implementation content to a requirement it satisfies." .
reqvire:traceTraceabilityRelationKind a reqvire:TraceabilityRelationKind ;
  reqvire:traceabilityRelationKindName "trace" ;
  reqvire:traceabilityRelationKindMeaning "Code marker relation kind that creates documentation traceability without satisfaction semantics." .

reqvire:lineHashCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "hash-line" ;
  reqvire:commentStyleMeaning "Single-line hash comment style for traceability markers." .
reqvire:lineSlashCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "slash-line" ;
  reqvire:commentStyleMeaning "Single-line slash comment style for traceability markers." .
reqvire:lineDashCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "dash-line" ;
  reqvire:commentStyleMeaning "Single-line dash comment style for traceability markers." .
reqvire:batchLineCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "batch-line" ;
  reqvire:commentStyleMeaning "Batch-style line comment category for traceability markers." .
reqvire:blockCssCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "css-block" ;
  reqvire:commentStyleMeaning "CSS-style block comment wrapper for traceability markers." .
reqvire:blockXmlCommentStyle a reqvire:CommentStyleKind ;
  reqvire:commentStyleName "xml-block" ;
  reqvire:commentStyleMeaning "XML/HTML-style block comment wrapper for traceability markers." .

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
reqvire:DocumentationExportContract a owl:Class ;
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
