export interface OntologyGraphData {
  nodes?: OntologyGraphNode[];
  edges?: OntologyGraphEdge[];
}

export interface OntologyGraphNode {
  id: string;
  label: string;
  type?: string;
  node_type?: string;
  semantic_type: string;
  layer: "authored" | "concepts" | "reqvire-context" | "external-source";
  source_kind: "ontology" | "shape" | "concepts" | "model-context" | "external-ontology" | "query";
  full_uri: string;
  ontology_document?: string;
  scheme_iri?: string;
  scheme_label?: string;
  comment: string;
  rdf_types: string[];
  type_evidence: OntologyGraphTypeEvidence[];
  sources: OntologyGraphSource[];
  constraints: { property: string; value: string }[];
  badges: { kind: string; symbol: string; code_point: string; label: string }[];
  equivalence_group: string;
  inverse_properties: string[];
  property_chains: { id: string; members: string[]; source: string }[];
  domain: { label: string; iri: string; kind: string }[];
  range: { label: string; iri: string; kind: string }[];
  literal_values: { predicate: string; value: string; source: OntologyGraphSource }[];
  slot_facets: OntologyGraphSlotFacet[];
  constructs: OntologyGraphConstructDetail[];
  query?: {
    form: string;
    text: string;
    ontologies: OntologyGraphSource[];
    vocabulary: { label: string; iri: string; kind: string }[];
    produces_properties: { label: string; iri: string; kind: string }[];
    produces_families: { label: string; iri: string; kind: string }[];
  };
}

export interface OntologyGraphEdge {
  source: string;
  target: string;
  label: string;
  layer: "authored" | "concepts" | "reqvire-context" | "external-source";
  source_kind: "ontology" | "shape" | "concepts" | "model-context" | "external-ontology" | "query";
}

export interface OntologyGraphSource {
  source: string;
  source_name: string;
  file_path: string;
  line_number: number;
  kind: string;
  link: string;
}

export interface OntologyGraphTypeEvidence {
  iri: string;
  label: string;
  source: OntologyGraphSource;
}

export interface OntologyGraphSlotFacet {
  slot_label: string;
  slot_iri: string;
  slot_kind: string;
  target_class_label: string;
  target_class_iri: string;
  source_shape_label: string;
  source_shape_iri: string;
  source: OntologyGraphSource;
  facets: { name: string; value: string }[];
}

export interface OntologyGraphConstructDetail {
  id: string;
  family: string;
  kind: string;
  label: string;
  subject: string;
  predicate: string;
  object: string;
  property: string;
  members: string[];
  source: OntologyGraphSource;
  badge?: { kind: string; symbol: string; code_point: string; label: string };
}

export interface KnowledgeGraphNode {
  id: string;
  identifier: string;
  label: string;
  type?: string;
  node_type?: string;
  element_type: string;
  file_path: string;
  line_number?: number;
  link?: string;
  description?: string;
  metadata?: KnowledgeGraphFact[];
  governance?: KnowledgeGraphFact[];
  outgoing?: KnowledgeGraphFact[];
  incoming?: KnowledgeGraphFact[];
  contract_bindings?: KnowledgeGraphFact[];
  concept_references?: KnowledgeGraphFact[];
}

export interface KnowledgeGraphFact {
  name: string;
  value: string;
  link?: string;
  kind?: string;
}

export interface KnowledgeGraphSubmodel {
  root_id: string;
  root_name: string;
  root_type: string;
  requirement_count: number;
}

export interface KnowledgeGraphProjection {
  summary?: {
    elements?: number;
    relations?: number;
    contract_bindings?: number;
    concept_references?: number;
    resources?: number;
    submodels?: number;
  };
  nodes?: KnowledgeGraphNode[];
  edges?: { source: string; target: string; label: string; kind: string; authored?: boolean }[];
  submodels?: KnowledgeGraphSubmodel[];
}
