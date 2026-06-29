use oxigraph::model::{NamedOrBlankNode, Term};

pub mod reserved;

pub use reserved::{OWL_NS, RDFS_NS, RDF_NS, SHACL_NS, XSD_NS};

pub const MODULE: &str = "vocab";

pub const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
pub const RDF_FIRST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
pub const RDF_REST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
pub const RDF_NIL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";
pub const RDF_PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
pub const RDF_LIST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#List";

pub const RDFS_SUBCLASS_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
pub const RDFS_DOMAIN: &str = "http://www.w3.org/2000/01/rdf-schema#domain";
pub const RDFS_RANGE: &str = "http://www.w3.org/2000/01/rdf-schema#range";
pub const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
pub const RDFS_COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";
pub const RDFS_IS_DEFINED_BY: &str = "http://www.w3.org/2000/01/rdf-schema#isDefinedBy";
pub const RDFS_CLASS: &str = "http://www.w3.org/2000/01/rdf-schema#Class";
pub const RDFS_DATATYPE: &str = "http://www.w3.org/2000/01/rdf-schema#Datatype";
pub const RDFS_LITERAL: &str = "http://www.w3.org/2000/01/rdf-schema#Literal";
pub const RDFS_RESOURCE: &str = "http://www.w3.org/2000/01/rdf-schema#Resource";

pub const OWL_CLASS: &str = "http://www.w3.org/2002/07/owl#Class";
pub const OWL_NAMED_INDIVIDUAL: &str = "http://www.w3.org/2002/07/owl#NamedIndividual";
pub const OWL_ONTOLOGY: &str = "http://www.w3.org/2002/07/owl#Ontology";
pub const OWL_THING: &str = "http://www.w3.org/2002/07/owl#Thing";
pub const OWL_IMPORTS: &str = "http://www.w3.org/2002/07/owl#imports";
pub const OWL_ANNOTATION_PROPERTY: &str = "http://www.w3.org/2002/07/owl#AnnotationProperty";
pub const OWL_OBJECT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ObjectProperty";
pub const OWL_DATATYPE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#DatatypeProperty";
pub const OWL_RESTRICTION: &str = "http://www.w3.org/2002/07/owl#Restriction";
pub const OWL_DISJOINT_WITH: &str = "http://www.w3.org/2002/07/owl#disjointWith";
pub const OWL_EQUIVALENT_CLASS: &str = "http://www.w3.org/2002/07/owl#equivalentClass";
pub const OWL_EQUIVALENT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#equivalentProperty";
pub const OWL_SAME_AS: &str = "http://www.w3.org/2002/07/owl#sameAs";
pub const OWL_INVERSE_OF: &str = "http://www.w3.org/2002/07/owl#inverseOf";
pub const OWL_PROPERTY_CHAIN_AXIOM: &str = "http://www.w3.org/2002/07/owl#propertyChainAxiom";
pub const OWL_FUNCTIONAL_PROPERTY: &str = "http://www.w3.org/2002/07/owl#FunctionalProperty";
pub const OWL_INVERSE_FUNCTIONAL_PROPERTY: &str =
    "http://www.w3.org/2002/07/owl#InverseFunctionalProperty";
pub const OWL_TRANSITIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#TransitiveProperty";
pub const OWL_SYMMETRIC_PROPERTY: &str = "http://www.w3.org/2002/07/owl#SymmetricProperty";
pub const OWL_ASYMMETRIC_PROPERTY: &str = "http://www.w3.org/2002/07/owl#AsymmetricProperty";
pub const OWL_REFLEXIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#ReflexiveProperty";
pub const OWL_IRREFLEXIVE_PROPERTY: &str = "http://www.w3.org/2002/07/owl#IrreflexiveProperty";
pub const OWL_ON_PROPERTY: &str = "http://www.w3.org/2002/07/owl#onProperty";
pub const OWL_ALL_VALUES_FROM: &str = "http://www.w3.org/2002/07/owl#allValuesFrom";
pub const OWL_SOME_VALUES_FROM: &str = "http://www.w3.org/2002/07/owl#someValuesFrom";
pub const OWL_HAS_VALUE: &str = "http://www.w3.org/2002/07/owl#hasValue";
pub const OWL_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#cardinality";
pub const OWL_MIN_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#minCardinality";
pub const OWL_MAX_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#maxCardinality";
pub const OWL_QUALIFIED_CARDINALITY: &str = "http://www.w3.org/2002/07/owl#qualifiedCardinality";
pub const OWL_MIN_QUALIFIED_CARDINALITY: &str =
    "http://www.w3.org/2002/07/owl#minQualifiedCardinality";
pub const OWL_MAX_QUALIFIED_CARDINALITY: &str =
    "http://www.w3.org/2002/07/owl#maxQualifiedCardinality";
pub const OWL_ON_CLASS: &str = "http://www.w3.org/2002/07/owl#onClass";
pub const OWL_ON_DATA_RANGE: &str = "http://www.w3.org/2002/07/owl#onDataRange";
pub const OWL_INTERSECTION_OF: &str = "http://www.w3.org/2002/07/owl#intersectionOf";
pub const OWL_UNION_OF: &str = "http://www.w3.org/2002/07/owl#unionOf";
pub const OWL_COMPLEMENT_OF: &str = "http://www.w3.org/2002/07/owl#complementOf";

pub const SHACL_PREFIX: &str = "http://www.w3.org/ns/shacl#";
pub const SH_SHAPE: &str = "http://www.w3.org/ns/shacl#Shape";
pub const SH_NODE_SHAPE: &str = "http://www.w3.org/ns/shacl#NodeShape";
pub const SH_PROPERTY_SHAPE: &str = "http://www.w3.org/ns/shacl#PropertyShape";
pub const SH_TARGET_CLASS: &str = "http://www.w3.org/ns/shacl#targetClass";
pub const SH_TARGET_NODE: &str = "http://www.w3.org/ns/shacl#targetNode";
pub const SH_TARGET_SUBJECTS_OF: &str = "http://www.w3.org/ns/shacl#targetSubjectsOf";
pub const SH_TARGET_OBJECTS_OF: &str = "http://www.w3.org/ns/shacl#targetObjectsOf";
pub const SH_TARGET: &str = "http://www.w3.org/ns/shacl#target";
pub const SH_PROPERTY: &str = "http://www.w3.org/ns/shacl#property";
pub const SH_PATH: &str = "http://www.w3.org/ns/shacl#path";
pub const SH_INVERSE_PATH: &str = "http://www.w3.org/ns/shacl#inversePath";
pub const SH_ALTERNATIVE_PATH: &str = "http://www.w3.org/ns/shacl#alternativePath";
pub const SH_ZERO_OR_MORE_PATH: &str = "http://www.w3.org/ns/shacl#zeroOrMorePath";
pub const SH_ONE_OR_MORE_PATH: &str = "http://www.w3.org/ns/shacl#oneOrMorePath";
pub const SH_ZERO_OR_ONE_PATH: &str = "http://www.w3.org/ns/shacl#zeroOrOnePath";
pub const SH_CLASS: &str = "http://www.w3.org/ns/shacl#class";
pub const SH_DATATYPE: &str = "http://www.w3.org/ns/shacl#datatype";
pub const SH_NODE_KIND: &str = "http://www.w3.org/ns/shacl#nodeKind";
pub const SH_MIN_COUNT: &str = "http://www.w3.org/ns/shacl#minCount";
pub const SH_MAX_COUNT: &str = "http://www.w3.org/ns/shacl#maxCount";
pub const SH_MIN_EXCLUSIVE: &str = "http://www.w3.org/ns/shacl#minExclusive";
pub const SH_MIN_INCLUSIVE: &str = "http://www.w3.org/ns/shacl#minInclusive";
pub const SH_MAX_EXCLUSIVE: &str = "http://www.w3.org/ns/shacl#maxExclusive";
pub const SH_MAX_INCLUSIVE: &str = "http://www.w3.org/ns/shacl#maxInclusive";
pub const SH_MIN_LENGTH: &str = "http://www.w3.org/ns/shacl#minLength";
pub const SH_MAX_LENGTH: &str = "http://www.w3.org/ns/shacl#maxLength";
pub const SH_PATTERN: &str = "http://www.w3.org/ns/shacl#pattern";
pub const SH_FLAGS: &str = "http://www.w3.org/ns/shacl#flags";
pub const SH_LANGUAGE_IN: &str = "http://www.w3.org/ns/shacl#languageIn";
pub const SH_UNIQUE_LANG: &str = "http://www.w3.org/ns/shacl#uniqueLang";
pub const SH_EQUALS: &str = "http://www.w3.org/ns/shacl#equals";
pub const SH_DISJOINT: &str = "http://www.w3.org/ns/shacl#disjoint";
pub const SH_LESS_THAN: &str = "http://www.w3.org/ns/shacl#lessThan";
pub const SH_LESS_THAN_OR_EQUALS: &str = "http://www.w3.org/ns/shacl#lessThanOrEquals";
pub const SH_AND: &str = "http://www.w3.org/ns/shacl#and";
pub const SH_OR: &str = "http://www.w3.org/ns/shacl#or";
pub const SH_NOT: &str = "http://www.w3.org/ns/shacl#not";
pub const SH_XONE: &str = "http://www.w3.org/ns/shacl#xone";
pub const SH_NODE: &str = "http://www.w3.org/ns/shacl#node";
pub const SH_QUALIFIED_VALUE_SHAPE: &str = "http://www.w3.org/ns/shacl#qualifiedValueShape";
pub const SH_QUALIFIED_MIN_COUNT: &str = "http://www.w3.org/ns/shacl#qualifiedMinCount";
pub const SH_QUALIFIED_MAX_COUNT: &str = "http://www.w3.org/ns/shacl#qualifiedMaxCount";
pub const SH_HAS_VALUE: &str = "http://www.w3.org/ns/shacl#hasValue";
pub const SH_IN: &str = "http://www.w3.org/ns/shacl#in";
pub const SH_SPARQL: &str = "http://www.w3.org/ns/shacl#sparql";
pub const SH_SELECT: &str = "http://www.w3.org/ns/shacl#select";
pub const SH_CLOSED: &str = "http://www.w3.org/ns/shacl#closed";
pub const SH_IGNORED_PROPERTIES: &str = "http://www.w3.org/ns/shacl#ignoredProperties";

pub const SKOS_NS: &str = "http://www.w3.org/2004/02/skos/core#";
pub const SKOS_ONTOLOGY: &str = "http://www.w3.org/2004/02/skos/core";
pub const SKOS_CONCEPT: &str = "http://www.w3.org/2004/02/skos/core#Concept";
pub const SKOS_CONCEPT_SCHEME: &str = "http://www.w3.org/2004/02/skos/core#ConceptScheme";
pub const SKOS_PREF_LABEL: &str = "http://www.w3.org/2004/02/skos/core#prefLabel";
pub const SKOS_DEFINITION: &str = "http://www.w3.org/2004/02/skos/core#definition";
pub const SKOS_SCOPE_NOTE: &str = "http://www.w3.org/2004/02/skos/core#scopeNote";
pub const SKOS_IN_SCHEME: &str = "http://www.w3.org/2004/02/skos/core#inScheme";
pub const SKOS_HAS_TOP_CONCEPT: &str = "http://www.w3.org/2004/02/skos/core#hasTopConcept";
pub const SKOS_TOP_CONCEPT_OF: &str = "http://www.w3.org/2004/02/skos/core#topConceptOf";
pub const SKOS_BROADER: &str = "http://www.w3.org/2004/02/skos/core#broader";
pub const SKOS_NARROWER: &str = "http://www.w3.org/2004/02/skos/core#narrower";
pub const SKOS_RELATED: &str = "http://www.w3.org/2004/02/skos/core#related";
pub const SKOS_EXACT_MATCH: &str = "http://www.w3.org/2004/02/skos/core#exactMatch";
pub const SKOS_CLOSE_MATCH: &str = "http://www.w3.org/2004/02/skos/core#closeMatch";
pub const SKOS_BROAD_MATCH: &str = "http://www.w3.org/2004/02/skos/core#broadMatch";
pub const SKOS_NARROW_MATCH: &str = "http://www.w3.org/2004/02/skos/core#narrowMatch";
pub const SKOS_RELATED_MATCH: &str = "http://www.w3.org/2004/02/skos/core#relatedMatch";
pub const SKOS_ALT_LABEL: &str = "http://www.w3.org/2004/02/skos/core#altLabel";
pub const SKOS_HIDDEN_LABEL: &str = "http://www.w3.org/2004/02/skos/core#hiddenLabel";
pub const SKOS_EXAMPLE: &str = "http://www.w3.org/2004/02/skos/core#example";

pub fn subject_iri(subject: &NamedOrBlankNode) -> Option<&str> {
    match subject {
        NamedOrBlankNode::NamedNode(node) => Some(node.as_str()),
        NamedOrBlankNode::BlankNode(_) => None,
    }
}

pub fn term_iri(term: &Term) -> Option<&str> {
    match term {
        Term::NamedNode(node) => Some(node.as_str()),
        _ => None,
    }
}
