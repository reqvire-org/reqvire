# Elements

### Reqvire Semantic Export Ontology


The Reqvire semantic export ontology defines RDF export concepts for collected ontology vocabulary, SHACL shapes, SKOS concepts, and combined semantic graph content.

Semantic exports preserve Markdown as the source of truth while exposing parsed ontology and shape content for downstream semantic tooling.

#### Ontology
```turtle
@prefix reqvire: <https://www.reqvire.org/ontology#> .
@prefix concept: <https://www.reqvire.org/concepts#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

reqvire:GraphRegistry a owl:Class .
reqvire:RdfProjection a owl:Class .
reqvire:RdfTriple a owl:Class .
reqvire:SemanticExport a owl:Class ;
  reqvire:mapsToConcept concept:SemanticExport .
reqvire:OntologyTerm a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection .
reqvire:OwlReservedVocabularyRegistry a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  reqvire:mapsToConcept concept:OwlReservedVocabularyRegistry ;
  rdfs:comment "Internal o-kernel registry of RDF, RDFS, XSD, OWL, and SHACL reserved vocabulary IRIs recognized by Reqvire without loading local external ontology sources. RDF, RDFS, OWL, and SHACL terms are derived from bundled standards vocabulary graphs; XSD datatype terms are kernel datatype policy." .
reqvire:OwlReservedVocabularyTerm a owl:Class ;
  rdfs:subClassOf reqvire:OntologyTerm ;
  reqvire:mapsToConcept concept:OwlReservedVocabularyTerm ;
  rdfs:comment "Reserved vocabulary IRI recognized by Reqvire in valid semantic positions without external ontology source loading." .
reqvire:OwlBuiltInDatatype a owl:Class ;
  rdfs:subClassOf reqvire:OwlReservedVocabularyTerm ;
  reqvire:mapsToConcept concept:OwlBuiltInDatatype ;
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
  reqvire:mapsToConcept concept:ExternalOntologySource ;
  rdfs:comment "Local external ontology source declared by an ontology element through a repeatable External Ontology section." .
reqvire:BuiltInExternalOntologySource a owl:Class ;
  rdfs:subClassOf reqvire:ExternalOntologySource ;
  reqvire:mapsToConcept concept:BuiltInExternalOntologySource ;
  rdfs:comment "Reqvire-shipped external ontology source available without project-local External Ontology declaration." .
reqvire:RawExternalOntologyGraph a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  reqvire:mapsToConcept concept:RawExternalOntologyGraph ;
  rdfs:comment "Internal raw graph parsed from a local external ontology dependency input; this graph is available for validation and subset construction but is not a public semantic export mode." .
reqvire:UsedExternalOntologySubset a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  reqvire:mapsToConcept concept:UsedExternalOntologySubset ;
  rdfs:comment "Only external ontology materialization that Reqvire exposes when include_external is requested; it contains used external terms plus selected support and annotation facts." .
reqvire:UsedExternalOntologyTerm a owl:Class ;
  rdfs:subClassOf reqvire:OntologyTerm ;
  rdfs:comment "External ontology term selected for exposure because authored Reqvire semantic content references it directly or through required support closure." .
reqvire:ModelContextProjection a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection .
reqvire:SemanticConstructQuery a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  reqvire:mapsToConcept concept:OntologyProjection ;
  rdfs:comment "Versioned SPARQL CONSTRUCT query specification that defines generated semantic projection facts independently of the current Rust materialization path." .
reqvire:RelationFamilyConstructQuery a owl:Class ;
  rdfs:subClassOf reqvire:SemanticConstructQuery ;
  reqvire:mapsToConcept concept:RelationFamilyConstructQuery ;
  rdfs:comment "SPARQL CONSTRUCT query specification for materializing canonical forward and inverse relation-family facts from authored Reqvire model relations." .
reqvire:ExternalOntologySubsetConstructQuery a owl:Class ;
  rdfs:subClassOf reqvire:SemanticConstructQuery ;
  reqvire:mapsToConcept concept:ExternalOntologySubsetConstructQuery ;
  rdfs:comment "SPARQL query specification for selecting the used external ontology subset from internal raw external dependency graphs." .
reqvire:OntologyProjectionGraph a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated graph-level projection record for ontology construct facts emitted in full semantic exports." .
reqvire:OntologyConstructProjection a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated projection pass record grouping ontology constructs by construct family." .
reqvire:OntologyConstruct a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated record for an extracted ontology construct such as a declaration, axiom, path member, restriction, or shape overlay." .
reqvire:OntologyConstructMember a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated ordered member record for RDF list and path constructs." .
reqvire:OntologyProjectionProvenance a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated provenance record that explains how an ontology construct was derived from authored semantic blocks." .
reqvire:OntologyProjectionEvidence a owl:Class ;
  rdfs:subClassOf reqvire:RdfProjection ;
  rdfs:comment "Generated evidence record linking a projected construct to source RDF terms." .
reqvire:OntologyProjectionSource a owl:Class ;
  rdfs:subClassOf reqvire:SemanticBlock ;
  rdfs:comment "Generated semantic-block source record used by ontology projection facts." .
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
  reqvire:mapsToConcept concept:ExternalOntologyResource ;
  rdfs:comment "Ontology document IRI declared by a local external ontology source." .
reqvire:externalUsedTerm a owl:ObjectProperty ;
  rdfs:domain reqvire:UsedExternalOntologySubset ;
  rdfs:range reqvire:UsedExternalOntologyTerm ;
  rdfs:comment "External term selected for public used-subset materialization." .
reqvire:externalSubsetSource a owl:ObjectProperty ;
  rdfs:domain reqvire:UsedExternalOntologySubset ;
  rdfs:range reqvire:ExternalOntologySource ;
  rdfs:comment "External ontology source declaration from which a used-subset materialization was derived." .
reqvire:externalSubsetGraph a owl:ObjectProperty ;
  rdfs:domain reqvire:UsedExternalOntologySubset ;
  rdfs:range reqvire:RawExternalOntologyGraph ;
  rdfs:comment "Internal raw external graph used as input to used-subset construction." .
reqvire:recognizesReservedVocabularyTerm a owl:ObjectProperty ;
  rdfs:domain reqvire:OwlReservedVocabularyRegistry ;
  rdfs:range reqvire:OwlReservedVocabularyTerm ;
  rdfs:comment "Links the OWL reserved vocabulary registry to a recognized reserved IRI derived from bundled standards vocabularies or kernel datatype policy." .
reqvire:relationTarget a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:Element .
reqvire:conceptReference a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm .
reqvire:constructQueryMaterializesFamily a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticConstructQuery ;
  rdfs:range reqvire:RelationFamily ;
  rdfs:comment "Relation family whose normalized semantic facts are materialized by a construct-query specification." .
reqvire:constructQueryMaterializesProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:SemanticConstructQuery ;
  rdfs:range owl:ObjectProperty ;
  rdfs:comment "Normalized RDF property that can be emitted by a construct-query specification." .
reqvire:projectedConstruct a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyConstruct ;
  rdfs:comment "Links an ontology projection graph or projection pass to a generated ontology construct record." .
reqvire:ontologyConstructProjection a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyProjectionGraph ;
  rdfs:range reqvire:OntologyConstructProjection ;
  rdfs:comment "Links the generated ontology projection graph to a construct-family projection pass." .
reqvire:constructSourceBlock a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:SemanticBlock ;
  rdfs:comment "Source semantic block from which a generated ontology projection record was derived." .
reqvire:constructSubject a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm ;
  rdfs:comment "Subject term captured for a generated ontology construct or projection evidence record." .
reqvire:constructPredicate a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm ;
  rdfs:comment "Predicate term captured for a generated ontology construct or projection evidence record." .
reqvire:constructObject a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm ;
  rdfs:comment "Object term captured for a generated ontology construct or projection evidence record." .
reqvire:constructProperty a owl:ObjectProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range reqvire:OntologyTerm ;
  rdfs:comment "Property term captured for property-centric generated ontology constructs." .
reqvire:constructMember a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range reqvire:OntologyConstructMember ;
  rdfs:comment "Ordered member of a generated ontology construct." .
reqvire:memberTerm a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyConstructMember ;
  rdfs:range reqvire:OntologyTerm ;
  rdfs:comment "Ontology term represented by an ordered construct member." .
reqvire:constructProvenance a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range reqvire:OntologyProjectionProvenance ;
  rdfs:comment "Provenance record for a generated ontology construct." .
reqvire:provenanceSource a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyProjectionProvenance ;
  rdfs:range reqvire:OntologyProjectionSource ;
  rdfs:comment "Generated source record associated with ontology construct provenance." .
reqvire:provenanceEvidence a owl:ObjectProperty ;
  rdfs:domain reqvire:OntologyProjectionProvenance ;
  rdfs:range reqvire:OntologyProjectionEvidence ;
  rdfs:comment "Evidence records associated with ontology construct provenance." .
reqvire:elementName a owl:DatatypeProperty ;
  rdfs:domain reqvire:Element ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical model element name emitted in full semantic export context." .
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
reqvire:referenceKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical concept-reference kind token emitted by model-context RDF projection facts." .
reqvire:externalOntologyPrefix a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:string ;
  reqvire:mapsToConcept concept:ExternalOntologyPrefix ;
  rdfs:comment "Prefix label configured for a local external ontology source." .
reqvire:externalOntologyNamespace a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:anyURI ;
  reqvire:mapsToConcept concept:ExternalOntologyNamespace ;
  rdfs:comment "Namespace IRI configured for a local external ontology source." .
reqvire:externalOntologySourcePath a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:string ;
  reqvire:mapsToConcept concept:ExternalOntologySourcePath ;
  rdfs:comment "Local repository path to the external ontology source file." .
reqvire:externalOntologyFormat a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:string ;
  reqvire:mapsToConcept concept:ExternalOntologyFormat ;
  rdfs:comment "Serialization format of a local external ontology source, such as Turtle/TTL, RDF/XML, or JSON-LD." .
reqvire:builtinExternalOntology a owl:DatatypeProperty ;
  rdfs:domain reqvire:ExternalOntologySource ;
  rdfs:range xsd:boolean ;
  rdfs:comment "True when an external ontology source is shipped by Reqvire rather than declared by a project-local External Ontology section." .
reqvire:externalMaterializationMode a owl:DatatypeProperty ;
  rdfs:domain reqvire:UsedExternalOntologySubset ;
  rdfs:range xsd:string ;
  rdfs:comment "External ontology materialization mode token; public Reqvire semantic exports use the used_subset mode and do not expose full raw external graphs." .
reqvire:externalMaterializedTripleCount a owl:DatatypeProperty ;
  rdfs:domain reqvire:UsedExternalOntologySubset ;
  rdfs:range xsd:integer ;
  rdfs:comment "Count of triples emitted in the used external ontology subset." .
reqvire:constructFamily a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical construct family token used by semantic construct query specifications and generated ontology projection facts." .
reqvire:constructKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical construct kind token used by semantic construct query specifications and generated ontology projection facts." .
reqvire:projectionDerivationMode a owl:DatatypeProperty ;
  rdfs:domain reqvire:RdfProjection ;
  rdfs:range xsd:string ;
  rdfs:comment "Canonical derivation-mode token for semantic construct query specifications and generated ontology projection facts." .
reqvire:propertyCharacteristic a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Property characteristic token captured for generated ontology property constructs." .
reqvire:restrictionKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Restriction kind token captured for generated ontology restriction constructs." .
reqvire:classExpressionKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Class-expression kind token captured for generated ontology class expression constructs." .
reqvire:shapeOverlayKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstruct ;
  rdfs:range xsd:string ;
  rdfs:comment "Shape-overlay kind token captured for generated SHACL projection constructs." .
reqvire:constructSequenceIndex a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyConstructMember ;
  rdfs:range xsd:integer ;
  rdfs:comment "Stable order of an RDF list or path member inside a generated ontology construct." .
reqvire:sourceBlockId a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyProjectionSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable source semantic-block identifier for a generated ontology projection source." .
reqvire:sourceElementIdentifier a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyProjectionSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Stable element identifier that owns a generated ontology projection source." .
reqvire:sourceName a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyProjectionSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Human-readable source name for a generated ontology projection source." .
reqvire:blockKind a owl:DatatypeProperty ;
  rdfs:domain reqvire:OntologyProjectionSource ;
  rdfs:range xsd:string ;
  rdfs:comment "Semantic block kind token for a generated ontology projection source." .
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

reqvire:semanticArtifactExportMode a reqvire:SemanticArtifactExport ;
  rdfs:comment "Semantic ontology export mode that emits generated ontology document declarations plus authored ontology vocabulary with source comments." .
reqvire:fullSemanticModelExportMode a reqvire:FullSemanticModelExport ;
  rdfs:comment "Semantic graph export mode that emits generated ontology document declarations, authored ontology and SHACL blocks, and RDF triples for Reqvire model elements, relations, reused_contract_context, concept references, ontology term declarations, shape references, and generated ontology projection facts." .
reqvire:externalUsedTermSeedQuery a reqvire:ExternalOntologySubsetConstructQuery ;
  reqvire:constructQueryName "external-used-term-seed-query" ;
  reqvire:constructFamily "external-used-subset" ;
  reqvire:constructKind "seed-query" ;
  reqvire:projectionDerivationMode "construct-query-specified" ;
  reqvire:constructQueryPurpose "Select external ontology terms referenced by authored ontology, SHACL, concept-reference, model-context, or generated semantic projection facts whose IRIs fall under declared external namespaces." ;
  reqvire:constructQueryText """
PREFIX reqvire: <https://www.reqvire.org/ontology#>

SELECT DISTINCT ?term
WHERE {
  ?source a reqvire:ExternalOntologySource ;
    reqvire:externalOntologyNamespace ?namespace .
  {
    ?block reqvire:referencesTerm ?term .
  }
  UNION {
    ?block reqvire:declaresTerm ?term .
  }
  UNION {
    ?projection reqvire:conceptReference ?term .
  }
  UNION {
    ?projection reqvire:constructSubject|reqvire:constructPredicate|reqvire:constructObject|reqvire:constructProperty ?term .
  }
  FILTER(isIRI(?term))
  FILTER(STRSTARTS(STR(?term), STR(?namespace)))
}
""" .
reqvire:externalUsedTermDirectDescriptionConstructQuery a reqvire:ExternalOntologySubsetConstructQuery ;
  reqvire:constructQueryName "external-used-term-direct-description-construct" ;
  reqvire:constructFamily "external-used-subset" ;
  reqvire:constructKind "direct-description-construct" ;
  reqvire:projectionDerivationMode "construct-query-specified" ;
  reqvire:constructQueryPurpose "Construct direct raw-external-graph description triples for seed external ontology terms only." ;
  reqvire:constructQueryText """
PREFIX reqvire: <https://www.reqvire.org/ontology#>

CONSTRUCT {
  ?term ?p ?o .
}
WHERE {
  ?subset a reqvire:UsedExternalOntologySubset ;
    reqvire:externalUsedTerm ?term ;
    reqvire:externalSubsetGraph ?rawExternalGraph .
  GRAPH ?rawExternalGraph {
    ?term ?p ?o .
  }
}
""" .
reqvire:externalUsedTermSupportClosureConstructQuery a reqvire:ExternalOntologySubsetConstructQuery ;
  reqvire:constructQueryName "external-used-term-support-closure-construct" ;
  reqvire:constructFamily "external-used-subset" ;
  reqvire:constructKind "support-closure-construct" ;
  reqvire:projectionDerivationMode "construct-query-specified" ;
  reqvire:constructQueryPurpose "Construct one-hop support facts for used external terms across selected RDF, RDFS, and OWL support predicates." ;
  reqvire:constructQueryText """
PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX reqvire: <https://www.reqvire.org/ontology#>

CONSTRUCT {
  ?support ?p ?supportObject .
}
WHERE {
  ?subset a reqvire:UsedExternalOntologySubset ;
    reqvire:externalUsedTerm ?term ;
    reqvire:externalSubsetGraph ?rawExternalGraph .
  GRAPH ?rawExternalGraph {
    ?term ?p ?support .
    FILTER(?p IN (rdf:type, rdfs:subClassOf, rdfs:subPropertyOf, rdfs:domain, rdfs:range, owl:equivalentClass, owl:equivalentProperty, owl:inverseOf, owl:onProperty, owl:someValuesFrom, owl:allValuesFrom, owl:hasValue))
    FILTER(isIRI(?support) || isBlank(?support))
    OPTIONAL {
      ?support ?p ?supportObject .
      FILTER(?p IN (rdf:type, rdfs:subClassOf, rdfs:subPropertyOf, rdfs:domain, rdfs:range, owl:equivalentClass, owl:equivalentProperty, owl:inverseOf, owl:onProperty, owl:someValuesFrom, owl:allValuesFrom, owl:hasValue))
    }
  }
}
""" .
reqvire:externalUsedTermAnnotationConstructQuery a reqvire:ExternalOntologySubsetConstructQuery ;
  reqvire:constructQueryName "external-used-term-annotation-construct" ;
  reqvire:constructFamily "external-used-subset" ;
  reqvire:constructKind "annotation-construct" ;
  reqvire:projectionDerivationMode "construct-query-specified" ;
  reqvire:constructQueryPurpose "Construct label, comment, preferred-label, definition, and description annotation triples for used external terms and support terms." ;
  reqvire:constructQueryText """
PREFIX dcterms: <http://purl.org/dc/terms/>
PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX reqvire: <https://www.reqvire.org/ontology#>
PREFIX skos: <http://www.w3.org/2004/02/skos/core#>

CONSTRUCT {
  ?describedTerm ?annotationProperty ?annotationValue .
}
WHERE {
  ?subset a reqvire:UsedExternalOntologySubset ;
    reqvire:externalSubsetGraph ?rawExternalGraph .
  {
    ?subset reqvire:externalUsedTerm ?describedTerm .
  }
  UNION {
    ?subset reqvire:externalUsedTerm ?term .
    GRAPH ?rawExternalGraph {
      ?term ?supportProperty ?describedTerm .
      FILTER(?supportProperty IN (rdfs:subClassOf, rdfs:subPropertyOf, rdfs:domain, rdfs:range, owl:equivalentClass, owl:equivalentProperty, owl:inverseOf, owl:onProperty, owl:someValuesFrom, owl:allValuesFrom, owl:hasValue))
    }
  }
  GRAPH ?rawExternalGraph {
    ?describedTerm ?annotationProperty ?annotationValue .
    FILTER(?annotationProperty IN (rdfs:label, rdfs:comment, skos:prefLabel, skos:definition, dcterms:description))
  }
}
""" .
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

reqvire:NamespaceScopedOntologyExportShape
  a sh:NodeShape ;
  sh:targetClass reqvire:NamespaceScopedOntologyExport .

reqvire:RuntimeOntologyArtifactShape
  a sh:NodeShape ;
  sh:targetClass reqvire:RuntimeOntologyArtifact .

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
  * constrain: [Ontology and Shapes Collection](../Semantics/SemanticModelRequirements.md#ontology-and-shapes-collection)
  * constrain: [Namespace-Scoped Ontology Export](../Semantics/SemanticModelRequirements.md#namespace-scoped-ontology-export)
  * constrain: [Runtime Reqvire Ontology Artifact](../Semantics/SemanticModelRequirements.md#runtime-reqvire-ontology-artifact)
  * constrain: [Runtime Reqvire Ontology Synchronization](../Semantics/SemanticModelRequirements.md#runtime-reqvire-ontology-synchronization)
  * use: [Reqvire Semantic Export Ontology](#reqvire-semantic-export-ontology)
---
