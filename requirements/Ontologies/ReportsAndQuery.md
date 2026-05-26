# Elements

### Reqvire Report Ontology

The Reqvire report ontology defines model report, traversal, filter, and evidence output semantics.

Reports expose model structure without changing the source model. This ontology defines report categories, traversal semantics, and output semantics.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:ReportContract a owl:Class .
reqvire:TraversalContract a owl:Class .
reqvire:FilterContract a owl:Class .
reqvire:OutputContract a owl:Class .
reqvire:ReportKind a owl:Class .
reqvire:SearchFilterKind a owl:Class .
reqvire:JsonOutputContract a owl:Class ;
  rdfs:subClassOf reqvire:OutputContract .
reqvire:CollectSourceType a owl:Class .
reqvire:ImplementationCoverageSource a owl:Class .
reqvire:Submodel a owl:Class .
reqvire:CapabilityRootSubmodel a owl:Class ;
  rdfs:subClassOf reqvire:Submodel .
reqvire:ScopedSubmodel a owl:Class ;
  rdfs:subClassOf reqvire:Submodel .
reqvire:CrossSubmodelCoupling a owl:Class .
reqvire:ResourceReference a owl:Class .

reqvire:reportKindName a owl:DatatypeProperty .
reqvire:reportKindMeaning a owl:DatatypeProperty .
reqvire:searchFilterName a owl:DatatypeProperty .
reqvire:searchFilterMeaning a owl:DatatypeProperty .
reqvire:sourceTypeName a owl:DatatypeProperty .
reqvire:coverageSourceName a owl:DatatypeProperty .
reqvire:coverageSourceMeaning a owl:DatatypeProperty .
reqvire:submodelBoundaryRule a owl:DatatypeProperty .
reqvire:couplingSource a owl:ObjectProperty .
reqvire:couplingTarget a owl:ObjectProperty .
reqvire:couplingRelation a owl:ObjectProperty .

reqvire:searchReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "search" ;
  reqvire:reportKindMeaning "Report kind that filters and lists model elements with evidence and summary counters." .
reqvire:collectReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "collect" ;
  reqvire:reportKindMeaning "Report kind that gathers element context, refinements, attachments, and reachable semantic context." .
reqvire:coverageReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "coverage" ;
  reqvire:reportKindMeaning "Report kind that classifies implementation or verification coverage for requirements and capability roll-up." .
reqvire:submodelsReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "submodels" ;
  reqvire:reportKindMeaning "Report kind that exposes independent capability-rooted subgraphs and cross-submodel couplings." .
reqvire:resourcesReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "resources" ;
  reqvire:reportKindMeaning "Report kind that lists files and external resources referenced by model relations and attachments." .
reqvire:ontologiesReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "ontologies" ;
  reqvire:reportKindMeaning "Report kind that exports ontology and SHACL content with source citations." .
reqvire:tracesReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "traces" ;
  reqvire:reportKindMeaning "Report kind that projects relation paths for traceability review." .
reqvire:modelReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "model" ;
  reqvire:reportKindMeaning "Report kind that exposes parsed model structure and relation evidence." .
reqvire:containmentReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "containment" ;
  reqvire:reportKindMeaning "Report kind that exposes filesystem and element containment structure." .
reqvire:lintReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "lint" ;
  reqvire:reportKindMeaning "Report kind that exposes model quality findings that are not validation errors." .

reqvire:filePathSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "file-path" ;
  reqvire:searchFilterMeaning "Search filter over repository-relative file path patterns." .
reqvire:elementNameSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "element-name" ;
  reqvire:searchFilterMeaning "Search filter over element display names." .
reqvire:elementTypeSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "element-type" ;
  reqvire:searchFilterMeaning "Search filter over canonical element type tokens and type categories." .
reqvire:governanceMetadataSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "governance-metadata" ;
  reqvire:searchFilterMeaning "Search filter over effective status, priority, risk, and owner metadata." .
reqvire:contentSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "content" ;
  reqvire:searchFilterMeaning "Search filter over element or page textual content." .
reqvire:relationPresenceSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "relation-presence" ;
  reqvire:searchFilterMeaning "Search filter based on required or excluded authored relation types." .
reqvire:attachmentPresenceSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "attachment-presence" ;
  reqvire:searchFilterMeaning "Search filter based on required or excluded attachments." .

reqvire:elementCollectSource a reqvire:CollectSourceType ;
  reqvire:sourceTypeName "element" .
reqvire:refinedByCollectSource a reqvire:CollectSourceType ;
  reqvire:sourceTypeName "refined_by_element" .
reqvire:attachmentCollectSource a reqvire:CollectSourceType ;
  reqvire:sourceTypeName "attachment_element" .

reqvire:directSatisfiedCoverageSource a reqvire:ImplementationCoverageSource ;
  reqvire:coverageSourceName "direct_satisfied" ;
  reqvire:coverageSourceMeaning "A requirement has direct satisfiedBy implementation or evidence." .
reqvire:refinementContractSatisfiedViaAttachmentCoverageSource a reqvire:ImplementationCoverageSource ;
  reqvire:coverageSourceName "refinement_contract_satisfied_via_attachment" ;
  reqvire:coverageSourceMeaning "A requirement is covered because a directly satisfied requirement attaches a refinement contract owned by it." .
reqvire:refinementContractSatisfiedViaChildCoverageSource a reqvire:ImplementationCoverageSource ;
  reqvire:coverageSourceName "refinement_contract_satisfied_via_child" ;
  reqvire:coverageSourceMeaning "A requirement is covered because a directly satisfied child requirement implements an owned refinement contract." .
reqvire:uncoveredCoverageSource a reqvire:ImplementationCoverageSource ;
  reqvire:coverageSourceName "uncovered" ;
  reqvire:coverageSourceMeaning "A requirement has no implementation coverage evidence through direct satisfaction, attachment coverage, or child coverage." .

reqvire:capabilityRootSubmodelRule a reqvire:CapabilityRootSubmodel ;
  reqvire:submodelBoundaryRule "A capability with no capability parent relation is a capability-root submodel boundary." .
reqvire:crossSubmodelAttachmentDependencyRule a reqvire:CrossSubmodelCoupling ;
  reqvire:submodelBoundaryRule "Cross-submodel dependencies are explicit attachment contracts rather than hierarchy relations." .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---

### Reqvire Semantic Export Ontology

The Reqvire semantic export ontology defines RDF export concepts for collected ontology and SHACL content.

Semantic exports preserve Markdown as the source of truth while exposing parsed ontology and shape content for downstream semantic tooling.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

reqvire:GraphRegistry a owl:Class .
reqvire:RdfProjection a owl:Class .
reqvire:RdfTriple a owl:Class .
reqvire:SemanticExport a owl:Class .
reqvire:SemanticArtifactExport a owl:Class ;
  rdfs:subClassOf reqvire:SemanticExport .
reqvire:FullSemanticModelExport a owl:Class ;
  rdfs:subClassOf reqvire:SemanticExport .
reqvire:SemanticBlock a owl:Class .
reqvire:OntologyBlock a owl:Class ;
  rdfs:subClassOf reqvire:SemanticBlock .
reqvire:ShapeBlock a owl:Class ;
  rdfs:subClassOf reqvire:SemanticBlock .
reqvire:ModelContextProjection a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection .

reqvire:registryElement a owl:ObjectProperty .
reqvire:projectionTriple a owl:ObjectProperty .
reqvire:exportSourceElement a owl:ObjectProperty .
reqvire:attaches a owl:ObjectProperty .
reqvire:declaresTerm a owl:ObjectProperty .
reqvire:referencesTerm a owl:ObjectProperty .
reqvire:relationTarget a owl:ObjectProperty .
reqvire:conceptReference a owl:ObjectProperty .
reqvire:elementName a owl:DatatypeProperty .
reqvire:elementIdentifier a owl:DatatypeProperty .
reqvire:elementId a owl:DatatypeProperty .
reqvire:lineNumber a owl:DatatypeProperty .
reqvire:relationType a owl:DatatypeProperty .
reqvire:relationTargetIdentifier a owl:DatatypeProperty .
reqvire:attachmentTargetIdentifier a owl:DatatypeProperty .
reqvire:conceptLabel a owl:DatatypeProperty .
reqvire:referenceKind a owl:DatatypeProperty .

reqvire:semanticArtifactExportMode a reqvire:SemanticArtifactExport ;
  rdfs:comment "Default semantic export mode that emits authored ontology and SHACL blocks with source comments." .
reqvire:fullSemanticModelExportMode a reqvire:FullSemanticModelExport ;
  rdfs:comment "Semantic export mode that emits authored ontology and SHACL blocks plus RDF triples for Reqvire model elements, relations, attachments, concept references, ontology declarations, and shape references." .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Core Element Ontology](Core.md#reqvire-core-element-ontology)
---
