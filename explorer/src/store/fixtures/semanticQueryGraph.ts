import { devFixture } from "../devFixture";
import type { ExplorerProjectStore, OntologyGraphNode } from "../types";

export const queryId = "urn:reqvire:semantic-query:item-labels";
export const queryText = "PREFIX item: <https://example.org/items#>\nCONSTRUCT { ?s item:name ?name } WHERE { ?s a item:Item; item:name ?name }\n";
const source = {
  source: "system-model/Queries.md#item-labels", source_name: "Item labels",
  file_path: "system-model/Queries.md", line_number: 10, kind: "query",
  link: "#/content/system-model/Queries.md#item-labels",
};
const ontology = { ...source, source_name: "Item ontology", kind: "ontology", source: "system-model/Items.md#items", file_path: "system-model/Items.md", link: "#/content/system-model/Items.md#items" };
const term = { label: "name", iri: "https://example.org/items#name", kind: "datatype-property" };
const item = { label: "Item", iri: "https://example.org/items#Item", kind: "class" };
const base: OntologyGraphNode = {
  id: queryId, full_uri: queryId, label: "Item labels", semantic_type: "semantic-query",
  layer: "authored", source_kind: "query", comment: "Provides item labels.", rdf_types: ["SemanticQuery"],
  type_evidence: [], sources: [source], constraints: [], badges: [], equivalence_group: "",
  inverse_properties: [], property_chains: [], domain: [], range: [], literal_values: [], slot_facets: [], constructs: [],
  query: { form: "CONSTRUCT", text: queryText, ontologies: [ontology], vocabulary: [item, term], produces_properties: [term], produces_families: [] },
};
export const semanticQueryStore: ExplorerProjectStore = {
  ...devFixture,
  ontology: { ...devFixture.ontology, graph_data: {
    nodes: [base, ...[item, term].map(value => ({ ...base, query: undefined, id: value.iri, full_uri: value.iri, label: value.label, semantic_type: value.kind, source_kind: "ontology" as const, sources: [ontology], rdf_types: [], comment: "" }))],
    edges: [{ source: queryId, target: item.iri, label: "uses vocabulary", layer: "authored", source_kind: "query" }, { source: queryId, target: term.iri, label: "declares output", layer: "authored", source_kind: "query" }],
  } },
};
