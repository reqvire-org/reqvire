# Elements

### Report Output Vocabulary Shape

Defines SHACL constraints for report kinds, search filters, collection sources, coverage sources, submodel traversal, and cross-submodel coupling records.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
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
    sh:in ( "file-path" "element-name" "element-type" "governance-metadata" "content" "relation-presence" "reused-contract-context-presence" ) ;
  ] .

reqvire:CollectSourceTypeShape
  a sh:NodeShape ;
  sh:targetClass reqvire:CollectSourceType ;
  sh:property [
    sh:path reqvire:sourceTypeName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ( "element" "defined_by_element" "reused_contract_context_element" ) ;
  ] .

reqvire:ImplementationCoverageSourceShape
  a sh:NodeShape ;
  sh:targetClass reqvire:ImplementationCoverageSource ;
  sh:property [
    sh:path reqvire:coverageSourceName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
    sh:in ( "direct_satisfied" "contract_satisfied_via_reused_contract_context" "contract_satisfied_via_child" "uncovered" ) ;
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
  * constrain: [Model Reports](../Reports/ModelReports/ReportingRequirements.md#model-reports)
  * use: [Reqvire Report Ontology](#reqvire-report-ontology)
---

### Reqvire Report Ontology

The Reqvire report ontology defines model report, traversal, filter, and evidence output semantics.

Reports expose model structure without changing the source model. This ontology defines report categories, traversal semantics, and output semantics.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:ReportContract a owl:Class .
reqvire:TraversalContract a owl:Class ;
  rdfs:subClassOf reqvire:ReportContract .
reqvire:FilterContract a owl:Class ;
  rdfs:subClassOf reqvire:ReportContract .
reqvire:OutputContract a owl:Class ;
  rdfs:subClassOf reqvire:ReportContract .
reqvire:ReportKind a owl:Class ;
  rdfs:subClassOf reqvire:ReportContract .
reqvire:SearchFilterKind a owl:Class ;
  rdfs:subClassOf reqvire:FilterContract .
reqvire:JsonOutputContract a owl:Class ;
  rdfs:subClassOf reqvire:OutputContract .
reqvire:CollectSourceType a owl:Class ;
  rdfs:subClassOf reqvire:ReportContract .
reqvire:ImplementationCoverageSource a owl:Class ;
  rdfs:subClassOf reqvire:ReportContract .
reqvire:Submodel a owl:Class ;
  rdfs:subClassOf reqvire:TraversalContract .
reqvire:CapabilityRootSubmodel a owl:Class ;
  rdfs:subClassOf reqvire:Submodel .
reqvire:ScopedSubmodel a owl:Class ;
  rdfs:subClassOf reqvire:Submodel .
reqvire:CrossSubmodelCoupling a owl:Class ;
  rdfs:subClassOf reqvire:TraversalContract .
reqvire:ResourceReference a owl:Class ;
  rdfs:subClassOf reqvire:ReportContract .

reqvire:submodelBoundaryRule a owl:DatatypeProperty ;
  rdfs:domain reqvire:TraversalContract ;
  rdfs:range xsd:string ;
  rdfs:comment "Traversal rule text that defines a report submodel boundary or coupling interpretation." .
reqvire:couplingSource a owl:ObjectProperty ;
  rdfs:domain reqvire:CrossSubmodelCoupling ;
  rdfs:range reqvire:Submodel .
reqvire:couplingTarget a owl:ObjectProperty ;
  rdfs:domain reqvire:CrossSubmodelCoupling ;
  rdfs:range reqvire:Submodel .
reqvire:couplingRelation a owl:ObjectProperty ;
  rdfs:domain reqvire:CrossSubmodelCoupling ;
  rdfs:range reqvire:RelationRule .

reqvire:reportKindName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ReportKind ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical report command or report-output kind token." .
reqvire:searchFilterName a owl:DatatypeProperty ;
  rdfs:domain reqvire:SearchFilterKind ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical search filter token used by search contracts and output." .
reqvire:sourceTypeName a owl:DatatypeProperty ;
  rdfs:domain reqvire:CollectSourceType ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical collect report source type token." .
reqvire:coverageSourceName a owl:DatatypeProperty ;
  rdfs:domain reqvire:ImplementationCoverageSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical implementation coverage source token used in coverage records." .

reqvire:searchReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "search" ;
  rdfs:comment "Report kind that filters and lists model elements with evidence and summary counters." .
reqvire:collectReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "collect" ;
  rdfs:comment "Report kind that gathers element context, contracts, reused_contract_context, and reachable semantic context." .
reqvire:coverageReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "coverage" ;
  rdfs:comment "Report kind that classifies implementation or verification coverage for requirements and capability roll-up." .
reqvire:submodelsReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "submodels" ;
  rdfs:comment "Report kind that exposes independent capability-rooted subgraphs and cross-submodel couplings." .
reqvire:resourcesReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "resources" ;
  rdfs:comment "Report kind that lists files and external resources referenced by model relations and reused_contract_context." .
reqvire:ontologiesReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "ontologies" ;
  rdfs:comment "Report kind that exports ontology and SHACL content with source citations." .
reqvire:tracesReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "traces" ;
  rdfs:comment "Report kind that projects relation paths for traceability review." .
reqvire:modelReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "model" ;
  rdfs:comment "Report kind that exposes parsed model structure and relation evidence." .
reqvire:containmentReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "containment" ;
  rdfs:comment "Report kind that exposes filesystem and element containment structure." .
reqvire:lintReportKind a reqvire:ReportKind ;
  reqvire:reportKindName "lint" ;
  rdfs:comment "Report kind that exposes model quality findings that are not validation errors." .

reqvire:filePathSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "file-path" ;
  rdfs:comment "Search filter over repository-relative file path patterns." .
reqvire:elementNameSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "element-name" ;
  rdfs:comment "Search filter over element display names." .
reqvire:elementTypeSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "element-type" ;
  rdfs:comment "Search filter over canonical element type tokens and type categories." .
reqvire:governanceMetadataSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "governance-metadata" ;
  rdfs:comment "Search filter over effective status, priority, risk, and owner metadata." .
reqvire:contentSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "content" ;
  rdfs:comment "Search filter over element or page textual content." .
reqvire:relationPresenceSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "relation-presence" ;
  rdfs:comment "Search filter based on required or excluded authored relation types." .
reqvire:reusedContractContextPresenceSearchFilterKind a reqvire:SearchFilterKind ;
  reqvire:searchFilterName "reused-contract-context-presence" ;
  rdfs:comment "Search filter based on required or excluded reused_contract_context." .

reqvire:elementCollectSource a reqvire:CollectSourceType ;
  reqvire:sourceTypeName "element" .
reqvire:definedByCollectSource a reqvire:CollectSourceType ;
  reqvire:sourceTypeName "defined_by_element" .
reqvire:reusedContractContextCollectSource a reqvire:CollectSourceType ;
  reqvire:sourceTypeName "reused_contract_context_element" .

reqvire:directSatisfiedCoverageSource a reqvire:ImplementationCoverageSource ;
  reqvire:coverageSourceName "direct_satisfied" ;
  rdfs:comment "A requirement has direct satisfiedBy implementation or evidence." .
reqvire:contractSatisfiedViaReusedContractContextCoverageSource a reqvire:ImplementationCoverageSource ;
  reqvire:coverageSourceName "contract_satisfied_via_reused_contract_context" ;
  rdfs:comment "A requirement is covered because a directly satisfied requirement reuses a contract owned by it." .
reqvire:contractContractSatisfiedViaChildCoverageSource a reqvire:ImplementationCoverageSource ;
  reqvire:coverageSourceName "contract_satisfied_via_child" ;
  rdfs:comment "A requirement is covered because a directly satisfied child requirement implements an owned contract." .
reqvire:uncoveredCoverageSource a reqvire:ImplementationCoverageSource ;
  reqvire:coverageSourceName "uncovered" ;
  rdfs:comment "A requirement has no implementation coverage evidence through direct satisfaction, reused_contract_context coverage, or child coverage." .

reqvire:capabilityRootSubmodelRule a reqvire:CapabilityRootSubmodel ;
  reqvire:submodelBoundaryRule "A capability with no capability parent relation is a capability-root submodel boundary." .
reqvire:crossSubmodelReusedContractContextDependencyRule a reqvire:CrossSubmodelCoupling ;
  reqvire:submodelBoundaryRule "Cross-submodel dependencies are explicit reused_contract_context contracts rather than hierarchy relations." .

```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Relation Ontology](RelationsAndImpact.md#reqvire-relation-ontology)
---

### Reqvire Semantic Export Ontology

The Reqvire semantic export ontology defines RDF export concepts for collected ontology and SHACL content.

Semantic exports preserve Markdown as the source of truth while exposing parsed ontology and shape content for downstream semantic tooling.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:GraphRegistry a owl:Class .
reqvire:RdfProjection a owl:Class .
reqvire:RdfTriple a owl:Class .
reqvire:SemanticExport a owl:Class .
reqvire:OntologyTerm a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection .
reqvire:OwlReservedVocabularyRegistry a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Internal fixed-list registry of RDF, RDFS, XSD, and OWL reserved vocabulary IRIs recognized by Reqvire without loading local external ontology sources." .
reqvire:OwlReservedVocabularyTerm a owl:Class ;
  rdfs:subClassOf reqvire:OntologyTerm ;
  rdfs:comment "Reserved vocabulary IRI recognized by Reqvire in valid semantic positions without external ontology source loading." .
reqvire:OwlBuiltInDatatype a owl:Class ;
  rdfs:subClassOf reqvire:OwlReservedVocabularyTerm ;
  rdfs:comment "Reserved vocabulary IRI valid in datatype positions." .
reqvire:OwlDatatypeFacet a owl:Class ;
  rdfs:subClassOf reqvire:OwlReservedVocabularyTerm ;
  rdfs:comment "Reserved vocabulary IRI valid as a datatype facet or constraint term, not as a datatype." .
reqvire:SemanticArtifactExport a owl:Class ;
  rdfs:subClassOf reqvire:SemanticExport .
reqvire:FullSemanticModelExport a owl:Class ;
  rdfs:subClassOf reqvire:SemanticExport .
reqvire:SemanticBlock a owl:Class .
reqvire:OntologyBlock a owl:Class ;
  rdfs:subClassOf reqvire:SemanticBlock ;
  owl:disjointWith reqvire:ShapeBlock .
reqvire:ShapeBlock a owl:Class ;
  rdfs:subClassOf reqvire:SemanticBlock .
reqvire:ExternalOntologySource a owl:Class ;
  rdfs:subClassOf reqvire:SemanticBlock ;
  rdfs:comment "Local pinned external ontology source declared by an ontology element through a repeatable External Ontology section." .
reqvire:ModelContextProjection a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection .
reqvire:OntologyProjectionGraph a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated subprojection within the existing in-memory RDF projection containing normalized ontology-view constructs derived from authored ontology and SHACL quads." .
reqvire:OntologyConstructProjection a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Projection record produced by the ontology construct projector for direct-authored OWL/RDFS/SHACL patterns." .
reqvire:OntologyConstruct a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated semantic construct used by ontology output and Explorer exploration, such as a restriction, class expression, equivalence group, property chain, or property characteristic." .
reqvire:OntologyConstructMember a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Ordered member record for property chains and class-expression lists in generated ontology projection facts." .
reqvire:SemanticConstructQuery a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Versioned SPARQL CONSTRUCT query specification that defines generated semantic projection facts independently of the current Rust materialization path." .
reqvire:RelationFamilyConstructQuery a owl:Class ;
  rdfs:subClassOf reqvire:SemanticConstructQuery ;
  rdfs:comment "SPARQL CONSTRUCT query specification for materializing canonical forward and inverse relation-family facts from authored Reqvire model relations." .
reqvire:OntologyProjectionProvenance a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated provenance record describing how an ontology construct projection fact was derived." .
reqvire:OntologyProjectionEvidence a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Source quad evidence record supporting an ontology construct projection fact." .
reqvire:OntologyProjectionSource a owl:Class ;
  rdfs:subClassOf reqvire:SemanticBlock ;
  rdfs:comment "Source block record used by generated ontology projection facts." .
reqvire:OntologySymbol a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Canonical symbol vocabulary entry used by ontology projection facts and ontology viewer badges." .

reqvire:registryElement a owl:ObjectProperty ;
  rdfs:domain reqvire:GraphRegistry ;
  rdfs:range reqvire:Element .
reqvire:projectionTriple a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:RdfTriple .
reqvire:exportSourceElement a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticExport ;
  rdfs:range reqvire:Element .
reqvire:declaresTerm a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticBlock ;
  rdfs:range reqvire:OntologyTerm .
reqvire:referencesTerm a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticBlock ;
  rdfs:range reqvire:OntologyTerm .
reqvire:externalOntologyResource a owl:ObjectProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range owl:Ontology ;
  rdfs:comment "Ontology document IRI declared by a local external ontology source." .
reqvire:recognizesReservedVocabularyTerm a owl:ObjectProperty ;
  rdfs:domain reqvire:OwlReservedVocabularyRegistry ;
  rdfs:range reqvire:OwlReservedVocabularyTerm ;
  rdfs:comment "Links the OWL reserved vocabulary registry to a recognized fixed-list reserved IRI." .
reqvire:relationTarget a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:Element .
reqvire:conceptReference a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm .
reqvire:projectedConstruct a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyConstruct .
reqvire:ontologyConstructProjection a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyProjectionGraph ;
  rdfs:range reqvire:OntologyConstructProjection .
reqvire:ontologySymbol a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyProjectionGraph ;
  rdfs:range reqvire:OntologySymbol .
reqvire:constructSourceBlock a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:SemanticBlock .
reqvire:constructSubject a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm .
reqvire:constructPredicate a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm .
reqvire:constructObject a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm .
reqvire:constructProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm .
reqvire:constructMember a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range reqvire:OntologyConstructMember .
reqvire:constructQueryMaterializesFamily a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticConstructQuery ;
  rdfs:range reqvire:RelationFamily ;
  rdfs:comment "Relation family whose normalized semantic facts are materialized by a construct-query specification." .
reqvire:constructQueryMaterializesProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticConstructQuery ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Normalized RDF property that can be emitted by a construct-query specification." .
reqvire:memberTerm a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyConstructMember ;
  rdfs:range reqvire:OntologyTerm .
reqvire:constructProvenance a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range reqvire:OntologyProjectionProvenance .
reqvire:provenanceSource a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyProjectionProvenance ;
  rdfs:range reqvire:OntologyProjectionSource .
reqvire:provenanceEvidence a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyProjectionProvenance ;
  rdfs:range reqvire:OntologyProjectionEvidence .
reqvire:constructSymbol a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range reqvire:OntologySymbol .
reqvire:elementName a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical model element display name emitted in full semantic export context." .
reqvire:elementIdentifier a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable model element identifier emitted in semantic export records." .
reqvire:elementId a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable model element id emitted in semantic export records." .
reqvire:lineNumber a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticBlock ;
  rdfs:range xsd:integer ;
  rdfs:comment "Source line number for a semantic block or semantic source projection record." .
reqvire:relationType a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical relation type token emitted by model-context RDF projection facts." .
reqvire:relationTargetIdentifier a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable identifier of a relation target emitted by model-context RDF projection facts." .
reqvire:reusedContractContextTargetIdentifier a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable identifier of an reused_contract_context target emitted by model-context RDF projection facts." .
reqvire:conceptLabel a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Human-readable concept reference or ontology term label emitted by RDF projection facts." .
reqvire:termKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyTerm ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical kind token for an ontology projection term, such as iri, blank-node, or literal." .
reqvire:termValue a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyTerm ;
  rdfs:range xsd:string ;
  rdfs:comment "Raw RDF term value emitted in ontology projection facts." .
reqvire:reservedVocabularyIri a owl:DatatypeProperty ;
  rdfs:domain reqvire:OwlReservedVocabularyTerm ;
  rdfs:range xsd:anyURI ;
  rdfs:comment "Full IRI of a reserved vocabulary term recognized without external ontology source loading." .
reqvire:sourceBlockId a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyProjectionSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable generated identifier for an authored ontology or SHACL source block." .
reqvire:sourceElementIdentifier a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyProjectionSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable Reqvire element identifier for the source element that owns a semantic block." .
reqvire:sourceName a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyProjectionSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Source element display name emitted by generated ontology projection source records." .
reqvire:blockKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyProjectionSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical semantic block kind token emitted by ontology projection source records." .
reqvire:referenceKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical concept-reference kind token emitted by model-context RDF projection facts." .
reqvire:externalOntologyPrefix a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:string ;
  rdfs:comment "Prefix label configured for a local external ontology source." .
reqvire:externalOntologyNamespace a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:anyURI ;
  rdfs:comment "Namespace IRI configured for a local external ontology source." .
reqvire:externalOntologySourcePath a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:string ;
  rdfs:comment "Local repository path to the pinned external ontology source file." .
reqvire:externalOntologyFormat a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:string ;
  rdfs:comment "Serialization format of a local external ontology source. Turtle is the initial supported value." .
reqvire:constructFamily a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical ontology construct family token used for filtering generated projection facts." .
reqvire:constructKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical ontology construct kind token used for filtering generated projection facts." .
reqvire:propertyCharacteristic a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical property-characteristic token for generated ontology construct facts." .
reqvire:restrictionKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical OWL restriction kind token for generated ontology construct facts." .
reqvire:classExpressionKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical class-expression kind token for generated ontology construct facts." .
reqvire:shapeOverlayKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical SHACL overlay kind token for generated ontology construct facts." .
reqvire:constructSequenceIndex a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:integer ;
  rdfs:comment "Ordered sequence index for generated ontology construct member facts." .
reqvire:projectionDerivationMode a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical derivation-mode token for generated projection facts." .
reqvire:constructQueryName a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticConstructQuery ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable construct-query specification token." .
reqvire:constructQueryText a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticConstructQuery ;
  rdfs:range xsd:string ;
  rdfs:comment "SPARQL CONSTRUCT text that specifies generated semantic projection facts." .
reqvire:constructQueryPurpose a owl:DatatypeProperty ;
  rdfs:domain reqvire:SemanticConstructQuery ;
  rdfs:range xsd:string ;
  rdfs:comment "Human-readable purpose and intended consumer behavior for a construct-query specification." .
reqvire:symbolConceptName a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologySymbol ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical ontology viewer symbol concept token used for projection badges and filtering." .
reqvire:rawUnicodeCodePoint a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologySymbol ;
  rdfs:range xsd:string ;
  rdfs:comment "Unicode code point token for an ontology viewer symbol." .
reqvire:renderedUnicodeCharacter a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologySymbol ;
  rdfs:range xsd:string ;
  rdfs:comment "Rendered Unicode character for an ontology viewer symbol." .
reqvire:symbolTooltip a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologySymbol ;
  rdfs:range xsd:string ;
  rdfs:comment "Tooltip text for an ontology viewer symbol." .
reqvire:accessibleLabel a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologySymbol ;
  rdfs:range xsd:string ;
  rdfs:comment "Accessible label for an ontology viewer symbol." .

reqvire:semanticArtifactExportMode a reqvire:SemanticArtifactExport ;
  rdfs:comment "Default semantic export mode that emits generated ontology document declarations plus authored ontology and SHACL blocks with source comments." .
reqvire:fullSemanticModelExportMode a reqvire:FullSemanticModelExport ;
  rdfs:comment "Semantic export mode that emits generated ontology document declarations, authored ontology and SHACL blocks, and RDF triples for Reqvire model elements, relations, reused_contract_context, concept references, ontology term declarations, shape references, and generated ontology projection facts." .
reqvire:directAuthoredProjectionMode a reqvire:OntologyConstructProjection ;
  reqvire:projectionDerivationMode "direct-authored" ;
  rdfs:comment "Projection mode for constructs materialized from authored triples without OWL reasoning." .
reqvire:relationFamilyNormalizedConstructQuery a reqvire:RelationFamilyConstructQuery ;
  reqvire:constructQueryName "relation-family-normalized-projection" ;
  reqvire:constructFamily "semantic-search" ;
  reqvire:constructKind "relation-family-normalized-projection" ;
  reqvire:projectionDerivationMode "construct-query-specified" ;
  reqvire:constructQueryPurpose "Materialize canonical forward and inverse relation-family predicates for every authored Reqvire model relation so semantic search can query relation meaning rather than raw Markdown relation tokens." ;
  reqvire:constructQueryText """
PREFIX reqvire: <https://www.reqvire.org/ontology#>

CONSTRUCT {
  ?canonicalSource ?forwardProperty ?canonicalTarget .
  ?canonicalTarget ?inverseProperty ?canonicalSource .
}
WHERE {
  ?relation a reqvire:ModelRelation ;
    reqvire:relationSource ?source ;
    reqvire:relationTarget ?target ;
    reqvire:relationType ?relationName .

  ?rule a reqvire:RelationRule ;
    reqvire:relationName ?relationName ;
    reqvire:relationDirection ?direction ;
    reqvire:normalizedForwardProperty ?forwardProperty ;
    reqvire:normalizedInverseProperty ?inverseProperty .

  BIND(IF(?direction = "inverse", ?target, ?source) AS ?canonicalSource)
  BIND(IF(?direction = "inverse", ?source, ?target) AS ?canonicalTarget)
}
""" .
```

#### Metadata
  * type: ontology

#### Relations
  * derivedFrom: [Reqvire Semantic Contract Ontology](CapabilityRequirementModel.md#reqvire-semantic-contract-ontology)
---

### Semantic Export Projection Shape

Defines SHACL constraints for semantic export records, RDF projections, semantic blocks, and ontology-term references.

#### Shapes
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
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
    sh:path reqvire:reusedContractContextTargetIdentifier ;
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

reqvire:SemanticConstructQueryShape
  a sh:NodeShape ;
  sh:targetClass reqvire:SemanticConstructQuery ;
  sh:property [
    sh:path reqvire:constructQueryName ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:constructQueryText ;
    sh:minCount 1 ;
    sh:maxCount 1 ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:constructQueryPurpose ;
    sh:datatype xsd:string ;
  ] ;
  sh:property [
    sh:path reqvire:constructQueryMaterializesProperty ;
    sh:nodeKind sh:IRI ;
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

reqvire:OwlReservedVocabularyRegistryShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OwlReservedVocabularyRegistry ;
  sh:property [
    sh:path reqvire:recognizesReservedVocabularyTerm ;
    sh:class reqvire:OwlReservedVocabularyTerm ;
  ] .

reqvire:OwlReservedVocabularyTermShape
  a sh:NodeShape ;
  sh:targetClass reqvire:OwlReservedVocabularyTerm ;
  sh:property [
    sh:path reqvire:reservedVocabularyIri ;
    sh:minCount 1 ;
    sh:datatype xsd:anyURI ;
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
  * constrain: [Ontology and Shapes Collection](../Reports/ModelReports/ReportingRequirements.md#ontology-and-shapes-collection)
  * use: [Reqvire Semantic Export Ontology](#reqvire-semantic-export-ontology)
---

