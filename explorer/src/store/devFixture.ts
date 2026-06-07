/*
 * Minimal dev fixture seed.
 *
 * Used ONLY in `npm run dev` (import.meta.env.DEV) when no Rust-exported seed
 * is injected, so the component shell is browsable during frontend work. It is
 * intentionally tiny and is never shipped into a real export.
 */
import type { ExplorerProjectStore } from "./types";
import { EXPECTED_SCHEMA_VERSION } from "./types";

export const devFixture: ExplorerProjectStore = {
  schema_version: EXPECTED_SCHEMA_VERSION,
  project: { name: "Reqvire project (dev fixture)", root_label: "Reqvire root" },
  folders: [
    { path: "requirements", parent: null, children: ["requirements/Specifications.md"] },
  ],
  files: [
    {
      path: "requirements/Specifications.md",
      display_path: "requirements/Specifications.md",
      html_path: "requirements/Specifications.html",
      parent_folder: "requirements",
      element_ids: ["requirements/Specifications.md#example-requirement"],
      resource_ids: [],
    },
  ],
  resources: [
    {
      id: "resource:core/src/lib.rs",
      kind: "local-file",
      target: "core/src/lib.rs",
      display: "lib.rs",
      file_path: "core/src/lib.rs",
      external_url: null,
      referring_element_ids: ["requirements/Specifications.md#example-requirement"],
      relation_types: ["satisfiedBy"],
    },
  ],
  elements: [
    {
      id: "requirements/Specifications.md#example-requirement",
      name: "Example Requirement",
      element_type: "requirement",
      type_family: "requirement",
      file_path: "requirements/Specifications.md",
      line_number: 3,
      source_anchor: "requirements/Specifications.html#example-requirement",
      content: "The system shall demonstrate the Explorer shell with fixture data.",
      metadata: { type: "requirement" },
      governance: { status: "draft" },
    },
    {
      id: "requirements/Specifications.md#example-verification",
      name: "Example Verification",
      element_type: "test-verification",
      type_family: "verification",
      file_path: "requirements/Specifications.md",
      line_number: 20,
      source_anchor: "requirements/Specifications.html#example-verification",
      content: "Verifies the Example Requirement via a test.",
      metadata: { type: "test-verification" },
      governance: {},
    },
  ],
  relations: [
    {
      id: "rel:0",
      source_id: "requirements/Specifications.md#example-verification",
      target_id: "requirements/Specifications.md#example-requirement",
      target_kind: "element",
      relation_type: "verify",
      canonical_relation_type: "verify",
      source_relation_types: ["verify"],
      authored: true,
      generated_opposite: false,
      resource_id: null,
    },
  ],
  attachments: [],
  concept_refs: [],
  submodels: { submodels: [], cross_submodel_couplings: [], summary: {} },
  traces: { files: {} },
  coverage: {},
  ontology: {
    summary: {
      ontology_blocks: 1,
      shape_blocks: 1,
      total_blocks: 2,
      total_quads: 8,
    },
    declarations: {
      "urn:reqvire:test:api:ServiceEndpoint": [
        {
          iri: "urn:reqvire:test:api:ServiceEndpoint",
          role: "class",
          element_identifier: "requirements/Specifications.md#example-requirement",
        },
      ],
      "urn:reqvire:test:api:identifier": [
        {
          iri: "urn:reqvire:test:api:identifier",
          role: "datatype-property",
          element_identifier: "requirements/Specifications.md#example-requirement",
        },
      ],
    },
    blocks: [
      {
        file_path: "requirements/Specifications.md",
        kind: "ontology",
        line_number: 40,
      },
      {
        file_path: "requirements/Specifications.md",
        kind: "shapes",
        line_number: 58,
      },
    ],
    projection: {
      id: "urn:reqvire:ontology-projection",
      derivation_mode: "direct-authored",
      projections: [
        {
          id: "urn:reqvire:ontology-projection:property-domain-range",
          family: "property-domain-range",
          derivation_mode: "direct-authored",
          construct_ids: [
            "urn:reqvire:ontology-construct:identifier-domain",
            "urn:reqvire:ontology-construct:identifier-range",
          ],
        },
        {
          id: "urn:reqvire:ontology-projection:shape-overlay",
          family: "shape-overlay",
          derivation_mode: "direct-authored",
          construct_ids: ["urn:reqvire:ontology-construct:endpoint-shape"],
        },
      ],
      constructs: [
        {
          id: "urn:reqvire:ontology-construct:identifier-domain",
          family: "property-domain-range",
          kind: "property-domain",
          subject: {
            kind: "iri",
            value: "urn:reqvire:test:api:identifier",
            label: "identifier",
          },
          predicate: {
            kind: "iri",
            value: "http://www.w3.org/2000/01/rdf-schema#domain",
            label: "domain",
          },
          object: {
            kind: "iri",
            value: "urn:reqvire:test:api:ServiceEndpoint",
            label: "ServiceEndpoint",
          },
          provenance: {
            derivation_mode: "direct-authored",
            source: {
              source_block: "requirements/Specifications.md#ontology-1",
              source_element_identifier: "requirements/Specifications.md#example-requirement",
              source_name: "Example Requirement",
              file_path: "requirements/Specifications.md",
              line_number: 40,
              block_kind: "ontology",
            },
            evidence: [],
          },
        },
        {
          id: "urn:reqvire:ontology-construct:identifier-range",
          family: "property-domain-range",
          kind: "property-range",
          subject: {
            kind: "iri",
            value: "urn:reqvire:test:api:identifier",
            label: "identifier",
          },
          predicate: {
            kind: "iri",
            value: "http://www.w3.org/2000/01/rdf-schema#range",
            label: "range",
          },
          object: {
            kind: "iri",
            value: "http://www.w3.org/2001/XMLSchema#string",
            label: "string",
          },
          provenance: {
            derivation_mode: "direct-authored",
            source: {
              source_block: "requirements/Specifications.md#ontology-1",
              source_element_identifier: "requirements/Specifications.md#example-requirement",
              source_name: "Example Requirement",
              file_path: "requirements/Specifications.md",
              line_number: 41,
              block_kind: "ontology",
            },
            evidence: [],
          },
        },
        {
          id: "urn:reqvire:ontology-construct:endpoint-shape",
          family: "shape-overlay",
          kind: "shape-overlay",
          shape_overlay_kind: "node-shape",
          subject: {
            kind: "iri",
            value: "urn:reqvire:test:api:ServiceEndpointShape",
            label: "ServiceEndpointShape",
          },
          predicate: {
            kind: "iri",
            value: "http://www.w3.org/ns/shacl#targetClass",
            label: "targetClass",
          },
          object: {
            kind: "iri",
            value: "urn:reqvire:test:api:ServiceEndpoint",
            label: "ServiceEndpoint",
          },
          provenance: {
            derivation_mode: "direct-authored",
            source: {
              source_block: "requirements/Specifications.md#shapes-1",
              source_element_identifier: "requirements/Specifications.md#example-requirement",
              source_name: "Example Requirement",
              file_path: "requirements/Specifications.md",
              line_number: 58,
              block_kind: "shapes",
            },
            evidence: [],
          },
        },
      ],
      symbols: [],
    },
    graph_data: {
      nodes: [
        {
          id: "urn:reqvire:test:api:ServiceEndpoint",
          label: "ServiceEndpoint",
          type: "class",
          node_type: "class",
          semantic_type: "class",
          full_uri: "urn:reqvire:test:api:ServiceEndpoint",
          comment: "Example ontology class.",
          rdf_types: ["owl:Class"],
          type_evidence: [],
          sources: [
            {
              source: "requirements/Specifications.md#ontology-1",
              source_name: "Example Requirement",
              file_path: "requirements/Specifications.md",
              line_number: 40,
              kind: "ontology",
              link: "requirements/Specifications.html#example-requirement",
            },
          ],
          constraints: [],
          badges: [],
          equivalence_group: "",
          inverse_properties: [],
          property_chains: [],
          domain: [],
          range: [],
          literal_values: [],
          slot_facets: [],
          constructs: [],
        },
        {
          id: "urn:reqvire:test:api:identifier",
          label: "identifier",
          type: "datatype-property",
          node_type: "datatype-property",
          semantic_type: "datatype-property",
          full_uri: "urn:reqvire:test:api:identifier",
          comment: "Example datatype property.",
          rdf_types: ["owl:DatatypeProperty"],
          type_evidence: [],
          sources: [
            {
              source: "requirements/Specifications.md#ontology-1",
              source_name: "Example Requirement",
              file_path: "requirements/Specifications.md",
              line_number: 41,
              kind: "ontology",
              link: "requirements/Specifications.html#example-requirement",
            },
          ],
          constraints: [],
          badges: [],
          equivalence_group: "",
          inverse_properties: [],
          property_chains: [],
          domain: [{ label: "ServiceEndpoint", iri: "urn:reqvire:test:api:ServiceEndpoint", kind: "class" }],
          range: [{ label: "string", iri: "http://www.w3.org/2001/XMLSchema#string", kind: "datatype" }],
          literal_values: [],
          slot_facets: [],
          constructs: [],
        },
      ],
      edges: [
        {
          source: "urn:reqvire:test:api:identifier",
          target: "urn:reqvire:test:api:ServiceEndpoint",
          label: "domain",
        },
      ],
    },
    graph_renderer: {
      css: "#ontology-graph-container{width:100%;height:100%;min-height:320px}",
      js: `
const container = document.getElementById("ontology-graph-container");
if (container) {
  container.dataset.renderer = "committed";
  container.innerHTML = '<button type="button" data-node-id="urn:reqvire:test:api:identifier">identifier</button>';
}
window.filterOntologyGraph = function (query) {
  const results = document.getElementById("ontology-graph-results");
  if (!results) return;
  results.innerHTML = query ? "<li>identifier</li>" : "";
};
window.focusOntologyNode = function () {};
window.fitOntologyGraph = function () {};
window.resetOntologyGraphLayout = function () {};
window.clearOntologySelection = function () {
  const title = document.getElementById("ontology-inspector-title");
  if (title) title.textContent = "Node Inspector";
};
document.querySelectorAll(".ontology-filter-toggle").forEach((button) => {
  button.addEventListener("click", () => {
    button.classList.toggle("is-active");
    button.setAttribute("aria-pressed", button.classList.contains("is-active") ? "true" : "false");
  });
});
`,
    },
    ttl_href: "ontologies.ttl",
  },
  knowledge_graph: {
    nodes: [
      {
        id: "requirements/Specifications.md#example-requirement",
        identifier: "requirements/Specifications.md#example-requirement",
        label: "Example Requirement",
        type: "requirement",
        node_type: "requirement",
        element_type: "requirement",
        file_path: "requirements/Specifications.md",
        line_number: 3,
        link: "requirements/Specifications.html#example-requirement",
        description: "The system shall demonstrate the Explorer shell with fixture data.",
        metadata: [{ name: "type", value: "requirement", link: "", kind: "metadata" }],
        governance: [{ name: "status", value: "draft", link: "", kind: "governance" }],
        outgoing: [],
        incoming: [{ name: "verify", value: "Example Verification", link: "", kind: "authored" }],
        attachments: [],
        concept_references: [],
      },
      {
        id: "requirements/Specifications.md#example-verification",
        identifier: "requirements/Specifications.md#example-verification",
        label: "Example Verification",
        type: "verification",
        node_type: "verification",
        element_type: "test-verification",
        file_path: "requirements/Specifications.md",
        line_number: 20,
        link: "requirements/Specifications.html#example-verification",
        description: "Verifies the Example Requirement via a test.",
        metadata: [{ name: "type", value: "test-verification", link: "", kind: "metadata" }],
        governance: [],
        outgoing: [{ name: "verify", value: "Example Requirement", link: "", kind: "authored" }],
        incoming: [],
        attachments: [],
        concept_references: [],
      },
    ],
    edges: [
      {
        source: "requirements/Specifications.md#example-verification",
        target: "requirements/Specifications.md#example-requirement",
        label: "verify",
        kind: "authored",
        authored: true,
      },
    ],
    submodels: [
      {
        root_id: "requirements/Specifications.md#example-requirement",
        root_name: "Example Requirement",
        root_type: "requirement",
        requirement_count: 1,
      },
    ],
    summary: {
      elements: 2,
      relations: 1,
      attachments: 0,
      concept_references: 0,
      resources: 1,
      submodels: 1,
    },
  },
  search: [
    {
      id: "requirements/Specifications.md#example-requirement",
      kind: "element",
      title: "Example Requirement",
      route: "#/elements/requirements/Specifications.md#example-requirement",
      text: "Example Requirement requirement demonstrate Explorer shell",
    },
  ],
  summaries: {
    elements: 2,
    files: 1,
    folders: 1,
    resources: 1,
    relations: 1,
    attachments: 0,
    concept_refs: 0,
    ontology_blocks: 0,
    shape_blocks: 0,
  },
  routes: {
    canonical: [
      { id: "model", pattern: "#/model", title: "Model" },
      { id: "knowledge-graph", pattern: "#/knowledge-graph", title: "Knowledge Graph" },
      { id: "traces", pattern: "#/traces", title: "Traces" },
      { id: "ontologies", pattern: "#/ontologies", title: "Ontologies" },
      { id: "kn2", pattern: "#/kn2", title: "KN2" },
    ],
  },
};
