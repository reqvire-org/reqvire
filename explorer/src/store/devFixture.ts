/*
 * Minimal dev fixture seed.
 *
 * Used ONLY in `npm run dev` (import.meta.env.DEV) when no served Project Store
 * seed is available, so the component shell is browsable during frontend work.
 * It is intentionally tiny and is never shipped in release assets.
 */
import type { ExplorerProjectStore } from "./types";
import { EXPECTED_SCHEMA_VERSION } from "./types";

export const devFixture: ExplorerProjectStore = {
  schema_version: EXPECTED_SCHEMA_VERSION,
  project: {
    name: "reqvire",
    root_label: "reqvire @ dev-fixture",
    repository: "reqvire",
    branch: "dev-fixture",
  },
  folders: [
    { path: "system-model", parent: null, children: ["system-model/Specifications.md"] },
  ],
  files: [
    {
      path: "system-model/Specifications.md",
      display_path: "system-model/Specifications.md",
      markdown_content: [
        "# Elements",
        "",
        "### Example Requirement",
        "",
        "The dev fixture shall render Markdown inside the Explorer.",
        "",
        "#### Metadata",
        "  * type: requirement",
      ].join("\n"),
      parent_folder: "system-model",
      element_ids: [
        "system-model/Specifications.md#example-capability",
        "system-model/Specifications.md#example-requirement",
        "system-model/Specifications.md#example-unverified-requirement",
        "system-model/Specifications.md#example-verification",
        "system-model/Specifications.md#example-unsatisfied-verification",
      ],
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
      source_text: "pub fn fixture_source() -> &'static str {\n    \"ok\"\n}\n",
      external_url: null,
      referring_element_ids: ["system-model/Specifications.md#example-requirement"],
      relation_types: ["satisfiedBy"],
    },
  ],
  elements: [
    {
      id: "system-model/Specifications.md#example-capability",
      name: "Example Capability",
      element_type: "capability",
      type_family: "capability",
      file_path: "system-model/Specifications.md",
      line_number: 1,
      source_anchor: "#/content/system-model/Specifications.md#example-capability",
      content: "The dev fixture shall expose a capability with mixed verification and implementation coverage.",
      metadata: { type: "capability" },
      governance: { status: "draft" },
    },
    {
      id: "system-model/Specifications.md#example-requirement",
      name: "Example Requirement",
      element_type: "requirement",
      type_family: "requirement",
      file_path: "system-model/Specifications.md",
      line_number: 3,
      source_anchor: "#/content/system-model/Specifications.md#example-requirement",
      content: "The system shall demonstrate the Explorer shell with fixture data.",
      metadata: { type: "requirement" },
      governance: { status: "draft" },
    },
    {
      id: "system-model/Specifications.md#example-unverified-requirement",
      name: "Unverified Fixture Requirement",
      element_type: "requirement",
      type_family: "requirement",
      file_path: "system-model/Specifications.md",
      line_number: 12,
      source_anchor: "#/content/system-model/Specifications.md#example-unverified-requirement",
      content: "The system shall demonstrate an uncovered requirement row in the Coverage view.",
      metadata: { type: "requirement" },
      governance: { status: "draft" },
    },
    {
      id: "system-model/Specifications.md#example-verification",
      name: "Example Verification",
      element_type: "test-verification",
      type_family: "verification",
      file_path: "system-model/Specifications.md",
      line_number: 20,
      source_anchor: "#/content/system-model/Specifications.md#example-verification",
      content: "Verifies the Example Requirement via a test.",
      metadata: { type: "test-verification" },
      governance: {},
    },
    {
      id: "system-model/Specifications.md#example-unsatisfied-verification",
      name: "Unsatisfied Fixture Verification",
      element_type: "test-verification",
      type_family: "verification",
      file_path: "system-model/Specifications.md",
      line_number: 28,
      source_anchor: "#/content/system-model/Specifications.md#example-unsatisfied-verification",
      content: "Demonstrates a verification with no satisfiedBy evidence and no verified target.",
      metadata: { type: "test-verification" },
      governance: {},
    },
  ],
  relations: [
    {
      id: "rel:0",
      source_id: "system-model/Specifications.md#example-verification",
      target_id: "system-model/Specifications.md#example-requirement",
      target_kind: "element",
      relation_type: "verify",
      canonical_relation_type: "verify",
      source_relation_types: ["verify"],
      authored: true,
      generated_opposite: false,
      resource_id: null,
    },
  ],
  reused_contract_context: [],
  concept_refs: [],
  submodels: { submodels: [], cross_submodel_couplings: [], summary: {} },
  traces: { files: {} },
  coverage: {
    summary: {
      total_leaf_requirements: 2,
      verified_leaf_requirements: 1,
      unverified_leaf_requirements: 1,
      leaf_requirements_coverage_percentage: 50,
      total_test_verifications: 2,
      satisfied_test_verifications: 1,
      unsatisfied_test_verifications: 1,
      test_verifications_satisfaction_percentage: 50,
      total_verifications: 2,
      orphaned_verifications: 1,
      orphaned_verifications_percentage: 50,
      verification_types: {
        test: 2,
        formal_proof: 0,
        analysis: 0,
        inspection: 0,
        demonstration: 0,
      },
      total_requirements_in_scope: 2,
      covered_requirements: 1,
      uncovered_requirements: 1,
      implementation_coverage_percentage: 50,
      coverage_sources: {
        direct_satisfied: 1,
        contract_satisfied_via_reused_contract_context: 0,
        contract_satisfied_via_child: 0,
      },
    },
    verified_leaf_requirements: {
      files: {
        "system-model/Specifications.md": [
          {
            identifier: "system-model/Specifications.md#example-requirement",
            name: "Example Requirement",
            verified_by: ["system-model/Specifications.md#example-verification"],
          },
        ],
      },
    },
    unverified_leaf_requirements: {
      files: {
        "system-model/Specifications.md": [
          {
            identifier: "system-model/Specifications.md#example-unverified-requirement",
            name: "Unverified Fixture Requirement",
            verified_by: [],
          },
        ],
      },
    },
    satisfied_test_verifications: {
      files: {
        "system-model/Specifications.md": [
          {
            identifier: "system-model/Specifications.md#example-verification",
            name: "Example Verification",
            verification_type: "test-verification",
            satisfied_by: ["resource:core/src/lib.rs"],
          },
        ],
      },
    },
    unsatisfied_test_verifications: {
      files: {
        "system-model/Specifications.md": [
          {
            identifier: "system-model/Specifications.md#example-unsatisfied-verification",
            name: "Unsatisfied Fixture Verification",
            verification_type: "test-verification",
            satisfied_by: [],
          },
        ],
      },
    },
    orphaned_verifications: {
      files: {
        "system-model/Specifications.md": [
          {
            identifier: "system-model/Specifications.md#example-unsatisfied-verification",
            name: "Unsatisfied Fixture Verification",
            verification_type: "test-verification",
            satisfied_by: [],
          },
        ],
      },
    },
    covered_requirements: {
      files: {
        "system-model/Specifications.md": [
          {
            identifier: "system-model/Specifications.md#example-requirement",
            name: "Example Requirement",
            coverage_source: "direct_satisfied",
            evidence: ["resource:core/src/lib.rs"],
          },
        ],
      },
    },
    uncovered_requirements: {
      files: {
        "system-model/Specifications.md": [
          {
            identifier: "system-model/Specifications.md#example-unverified-requirement",
            name: "Unverified Fixture Requirement",
          },
        ],
      },
    },
    capability_coverage: {
      capabilities: [
        {
          identifier: "system-model/Specifications.md#example-capability",
          name: "Example Capability",
          local_leaf_requirements: 2,
          local_verified_leaf_requirements: 1,
          aggregate_leaf_requirements: 2,
          aggregate_verified_leaf_requirements: 1,
          verification_coverage_percentage: 50,
          local_requirements: 2,
          local_covered_requirements: 1,
          aggregate_requirements: 2,
          aggregate_covered_requirements: 1,
          implementation_coverage_percentage: 50,
          mark: "partial",
        },
      ],
    },
  },
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
          element_identifier: "system-model/Specifications.md#example-requirement",
        },
      ],
      "urn:reqvire:test:api:identifier": [
        {
          iri: "urn:reqvire:test:api:identifier",
          role: "datatype-property",
          element_identifier: "system-model/Specifications.md#example-requirement",
        },
      ],
    },
    blocks: [
      {
        file_path: "system-model/Specifications.md",
        kind: "ontology",
        line_number: 40,
      },
      {
        file_path: "system-model/Specifications.md",
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
              source_block: "system-model/Specifications.md#ontology-1",
              source_element_identifier: "system-model/Specifications.md#example-requirement",
              source_name: "Example Requirement",
              file_path: "system-model/Specifications.md",
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
              source_block: "system-model/Specifications.md#ontology-1",
              source_element_identifier: "system-model/Specifications.md#example-requirement",
              source_name: "Example Requirement",
              file_path: "system-model/Specifications.md",
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
              source_block: "system-model/Specifications.md#shapes-1",
              source_element_identifier: "system-model/Specifications.md#example-requirement",
              source_name: "Example Requirement",
              file_path: "system-model/Specifications.md",
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
          layer: "authored",
          source_kind: "ontology",
          full_uri: "urn:reqvire:test:api:ServiceEndpoint",
          comment: "Example ontology class.",
          rdf_types: ["owl:Class"],
          type_evidence: [],
          sources: [
            {
              source: "system-model/Specifications.md#ontology-1",
              source_name: "Example Requirement",
              file_path: "system-model/Specifications.md",
              line_number: 40,
              kind: "ontology",
              link: "#/content/system-model/Specifications.md#example-requirement",
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
          layer: "authored",
          source_kind: "ontology",
          full_uri: "urn:reqvire:test:api:identifier",
          comment: "Example datatype property.",
          rdf_types: ["owl:DatatypeProperty"],
          type_evidence: [],
          sources: [
            {
              source: "system-model/Specifications.md#ontology-1",
              source_name: "Example Requirement",
              file_path: "system-model/Specifications.md",
              line_number: 41,
              kind: "ontology",
              link: "#/content/system-model/Specifications.md#example-requirement",
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
          layer: "authored",
          source_kind: "ontology",
        },
      ],
    },
    ttl_href: "ontologies.ttl",
  },
  knowledge_graph: {
    nodes: [
      {
        id: "system-model/Specifications.md#example-requirement",
        identifier: "system-model/Specifications.md#example-requirement",
        label: "Example Requirement",
        type: "requirement",
        node_type: "requirement",
        element_type: "requirement",
        file_path: "system-model/Specifications.md",
        line_number: 3,
        link: "#/content/system-model/Specifications.md#example-requirement",
        description: "The system shall demonstrate the Explorer shell with fixture data.",
        metadata: [{ name: "type", value: "requirement", link: "", kind: "metadata" }],
        governance: [{ name: "status", value: "draft", link: "", kind: "governance" }],
        outgoing: [],
        incoming: [{ name: "verify", value: "Example Verification", link: "", kind: "authored" }],
        reused_contract_context: [],
        concept_references: [],
      },
      {
        id: "system-model/Specifications.md#example-verification",
        identifier: "system-model/Specifications.md#example-verification",
        label: "Example Verification",
        type: "verification",
        node_type: "verification",
        element_type: "test-verification",
        file_path: "system-model/Specifications.md",
        line_number: 20,
        link: "#/content/system-model/Specifications.md#example-verification",
        description: "Verifies the Example Requirement via a test.",
        metadata: [{ name: "type", value: "test-verification", link: "", kind: "metadata" }],
        governance: [],
        outgoing: [{ name: "verify", value: "Example Requirement", link: "", kind: "authored" }],
        incoming: [],
        reused_contract_context: [],
        concept_references: [],
      },
    ],
    edges: [
      {
        source: "system-model/Specifications.md#example-verification",
        target: "system-model/Specifications.md#example-requirement",
        label: "verify",
        kind: "authored",
        authored: true,
      },
    ],
    submodels: [
      {
        root_id: "system-model/Specifications.md#example-requirement",
        root_name: "Example Requirement",
        root_type: "requirement",
        requirement_count: 1,
      },
    ],
    summary: {
      elements: 2,
      relations: 1,
      reused_contract_context: 0,
      concept_references: 0,
      resources: 1,
      submodels: 1,
    },
  },
  search: [
    {
      id: "system-model/Specifications.md#example-requirement",
      kind: "element",
      title: "Example Requirement",
      route: "#/elements/system-model/Specifications.md#example-requirement",
      text: "Example Requirement requirement demonstrate Explorer shell",
    },
  ],
  summaries: {
    elements: 2,
    files: 1,
    folders: 1,
    resources: 1,
    relations: 1,
    reused_contract_context: 0,
    concept_refs: 0,
    ontology_blocks: 0,
    shape_blocks: 0,
  },
  routes: {
    canonical: [
      { id: "model", pattern: "#/model", title: "Model" },
      { id: "traces", pattern: "#/traces", title: "Traces" },
      { id: "ontologies", pattern: "#/ontologies", title: "Ontologies" },
    ],
  },
};
