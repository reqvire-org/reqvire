# Elements

### Provide Reports

As a **Manager** and **System Engineer**, I want Reqvire to generate structured model reports, so that I can inspect model content, traceability, coverage, resources, submodels, and semantic outputs without manually reconstructing the graph.

#### Details
Provide reports is the capability for search, collect, coverage, traces, model, containment, resources, ontologies, submodels, lint, and semantic output contracts.

Report requirements define traversal direction, output structure, filters, JSON evidence, graph projection, and diagram/report rendering behavior.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: high
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Relation Ontology](../Ontologies/RelationsAndImpact.md#reqvire-relation-ontology)
  * [Reqvire Verification Ontology](../Ontologies/Verification.md#reqvire-verification-ontology)
  * [Reqvire Verification Rollup Ontology](../Ontologies/Verification.md#reqvire-verification-rollup-ontology)
  * [Reqvire Report Ontology](../Ontologies/ReportsAndQuery.md#reqvire-report-ontology)

#### Relations
  * specifiedBy: [Model Reports](../Functional/Output/Reporting.md#model-reports)
---

### Reqvire Report Ontology Shape Profile

Defines SHACL constraints for report kinds, search filters, collection sources, coverage sources, submodel traversal, and cross-submodel coupling records.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ReportKindShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ReportKind ;
  sh:property [
    sh:path reqvire:reportKindName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ( "search" "collect" "coverage" "submodels" "resources" "ontologies" "traces" "model" "containment" "lint" ) ;
  ] .

reqvire:SearchFilterKindShape
  a sh:NodeShape ;
  sh:targetClass reqvire:SearchFilterKind ;
  sh:property [
    sh:path reqvire:searchFilterName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ( "file-path" "element-name" "element-type" "governance-metadata" "content" "relation-presence" "attachment-presence" ) ;
  ] .

reqvire:CollectSourceTypeShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CollectSourceType ;
  sh:property [
    sh:path reqvire:sourceTypeName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ( "element" "refined_by_element" "attachment_element" ) ;
  ] .

reqvire:ImplementationCoverageSourceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ImplementationCoverageSource ;
  sh:property [
    sh:path reqvire:coverageSourceName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ( "direct_satisfied" "refinement_contract_satisfied_via_attachment" "refinement_contract_satisfied_via_child" "uncovered" ) ;
  ] .

reqvire:TraversalContractShape
  a sh:NodeShape ;
  sh:targetClass reqvire:TraversalContract ;
  sh:property [
    sh:path reqvire:submodelBoundaryRule ;
    sh:datatype xsd:string ;
  ] .

reqvire:CrossSubmodelCouplingShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CrossSubmodelCoupling ;
  sh:property [
    sh:path reqvire:couplingSource ;
    sh:class reqvire:Submodel ;
  ] ;
  sh:property [
    sh:path reqvire:couplingTarget ;
    sh:class reqvire:Submodel ;
  ] ;
  sh:property [
    sh:path reqvire:couplingRelation ;
    sh:class reqvire:RelationRule ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Model Reports](../Functional/Output/Reporting.md#model-reports)
---

### Semantic Model Export

As a **System Engineer**, I want Reqvire to expose ontology and shape content as semantic artifacts, so that downstream tools can inspect and reuse the model's semantic vocabulary without parsing Markdown directly.

#### Details
Semantic model export is the capability for collecting ontology and SHACL content, exporting Turtle/JSON-LD semantic artifacts, projecting Reqvire model context as RDF, materializing generated ontology projection facts, and keeping semantic exports traceable to their Reqvire source elements.

Semantic query contracts may define the intended graph patterns for generated projection facts. General-purpose query execution, query output, and inferred reasoning remain separate future capabilities unless a requirement explicitly scopes them.

#### Metadata
  * type: capability
  * owner: syseng
  * priority: medium
  * risk: medium
  * status: approved

#### Attachments
  * [Reqvire Core Element Ontology](../Ontologies/Core.md#reqvire-core-element-ontology)
  * [Reqvire Semantic Contract Ontology](../Ontologies/CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
  * [Reqvire Semantic Export Ontology](../Ontologies/ReportsAndQuery.md#reqvire-semantic-export-ontology)

#### Relations
  * specifiedBy: [Ontology and Shapes Collection](../Functional/Output/Reporting.md#ontology-and-shapes-collection)
---

### Reqvire Semantic Export Ontology Shape Profile

Defines SHACL constraints for semantic export records, RDF projections, semantic blocks, and ontology-term references.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:GraphRegistryShape
  a sh:NodeShape ;
  sh:targetClass reqvire:GraphRegistry ;
  sh:property [
    sh:path reqvire:registryElement ;
    sh:class reqvire:Element ;
  ] .

reqvire:SemanticExportShape
  a sh:NodeShape ;
  sh:targetClass reqvire:SemanticExport ;
  sh:property [
    sh:path reqvire:exportSourceElement ;
    sh:class reqvire:Element ;
  ] ;
  sh:property [
    sh:path reqvire:elementName ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:elementIdentifier ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:elementId ;
    sh:datatype xsd:string ;
  ] .

reqvire:RdfProjectionShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RdfProjection ;
  sh:property [
    sh:path reqvire:projectionTriple ;
    sh:class reqvire:RdfTriple ;
  ] ;
  sh:property [
    sh:path reqvire:relationTarget ;
    sh:class reqvire:Element ;
  ] ;
  sh:property [
    sh:path reqvire:conceptReference ;
    sh:class reqvire:Element ;
  ] ;
  sh:property [
    sh:path reqvire:relationType ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:relationTargetIdentifier ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:attachmentTargetIdentifier ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:conceptLabel ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:referenceKind ;
    sh:datatype xsd:string ;
  ] .

reqvire:OntologyProjectionGraphShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologyProjectionGraph ;
  sh:property [
    sh:path reqvire:ontologyConstructProjection ;
    sh:class reqvire:OntologyConstructProjection ;
  ] ;
  sh:property [
    sh:path reqvire:projectedConstruct ;
    sh:class reqvire:OntologyConstruct ;
  ] ;
  sh:property [
    sh:path reqvire:ontologySymbol ;
    sh:class reqvire:OntologySymbol ;
  ] ;
  sh:property [
    sh:path reqvire:projectionDerivationMode ;
    sh:datatype xsd:string ;
  ] .

reqvire:OntologyConstructProjectionShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologyConstructProjection ;
  sh:property [
    sh:path reqvire:constructFamily ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:projectedConstruct ;
    sh:class reqvire:OntologyConstruct ;
  ] ;
  sh:property [
    sh:path reqvire:constructQueryContract ;
    sh:class reqvire:SemanticQueryContract ;
  ] ;
  sh:property [
    sh:path reqvire:projectionDerivationMode ;
    sh:datatype xsd:string ;
  ] .

reqvire:OntologyConstructShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologyConstruct ;
  sh:property [
    sh:path reqvire:constructFamily ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:constructKind ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:constructSourceBlock ;
    sh:class reqvire:SemanticBlock ;
  ] ;
  sh:property [
    sh:path reqvire:constructSubject ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:constructPredicate ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:constructObject ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:constructProperty ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:constructMember ;
    sh:class reqvire:OntologyConstructMember ;
  ] ;
  sh:property [
    sh:path reqvire:constructProvenance ;
    sh:class reqvire:OntologyProjectionProvenance ;
  ] ;
  sh:property [
    sh:path reqvire:constructSymbol ;
    sh:class reqvire:OntologySymbol ;
  ] ;
  sh:property [
    sh:path reqvire:propertyCharacteristic ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:restrictionKind ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:classExpressionKind ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:shapeOverlayKind ;
    sh:datatype xsd:string ;
  ] .

reqvire:OntologyConstructMemberShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologyConstructMember ;
  sh:property [
    sh:path reqvire:memberTerm ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:constructSourceBlock ;
    sh:class reqvire:SemanticBlock ;
  ] ;
  sh:property [
    sh:path reqvire:constructSequenceIndex ;
    sh:datatype xsd:integer ;
  ] .

reqvire:OntologyProjectionProvenanceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologyProjectionProvenance ;
  sh:property [
    sh:path reqvire:projectionDerivationMode ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:constructQueryContract ;
    sh:class reqvire:SemanticQueryContract ;
  ] ;
  sh:property [
    sh:path reqvire:provenanceSource ;
    sh:class reqvire:OntologyProjectionSource ;
  ] ;
  sh:property [
    sh:path reqvire:provenanceEvidence ;
    sh:class reqvire:OntologyProjectionEvidence ;
  ] .

reqvire:OntologyProjectionEvidenceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologyProjectionEvidence ;
  sh:property [
    sh:path reqvire:constructSourceBlock ;
    sh:class reqvire:SemanticBlock ;
  ] ;
  sh:property [
    sh:path reqvire:constructSubject ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:constructPredicate ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:constructObject ;
    sh:class reqvire:OntologyTerm ;
  ] .

reqvire:OntologyTermShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologyTerm ;
  sh:property [
    sh:path reqvire:termKind ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:termValue ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:conceptLabel ;
    sh:datatype xsd:string ;
  ] .

reqvire:OntologyProjectionSourceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologyProjectionSource ;
  sh:property [
    sh:path reqvire:sourceBlockId ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:sourceElementIdentifier ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:sourceName ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:filePath ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:lineNumber ;
    sh:datatype xsd:integer ;
  ] ;
  sh:property [
    sh:path reqvire:blockKind ;
    sh:datatype xsd:string ;
  ] .

reqvire:OntologySymbolShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OntologySymbol ;
  sh:property [
    sh:path reqvire:symbolConceptName ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:rawUnicodeCodePoint ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:renderedUnicodeCharacter ;
    sh:minCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:symbolTooltip ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:accessibleLabel ;
    sh:datatype xsd:string ;
  ] .

reqvire:SemanticBlockShape
  a sh:NodeShape ;
  sh:targetClass reqvire:SemanticBlock ;
  sh:property [
    sh:path reqvire:declaresTerm ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:referencesTerm ;
    sh:class reqvire:OntologyTerm ;
  ] ;
  sh:property [
    sh:path reqvire:lineNumber ;
    sh:datatype xsd:integer ;
  ] .
```

#### Metadata
  * type: semantic-contract

#### Relations
  * refine: [Ontology and Shapes Collection](../Functional/Output/Reporting.md#ontology-and-shapes-collection)
---
