import type { KnowledgeGraphProjection } from "@ds";
export const fixture = {
  "title": "KnowledgeGraphCanvas",
  "graph": {
    "nodes": [
      {
        "id": "system-model/Specifications.md#example-capability",
        "identifier": "system-model/Specifications.md#example-capability",
        "label": "Example Capability",
        "type": "capability",
        "node_type": "capability",
        "element_type": "capability",
        "file_path": "system-model/Specifications.md",
        "line_number": 1,
        "link": "#/content/system-model/Specifications.md#example-capability",
        "description": "Capability root for the fixture project graph.",
        "metadata": [
          {
            "name": "type",
            "value": "capability",
            "link": "",
            "kind": "metadata"
          }
        ],
        "governance": [
          {
            "name": "status",
            "value": "draft",
            "link": "",
            "kind": "governance"
          }
        ],
        "outgoing": [],
        "incoming": [
          {
            "name": "specifiedBy",
            "value": "Example Requirement",
            "link": "",
            "kind": "authored"
          },
          {
            "name": "specifiedBy",
            "value": "Unverified Fixture Requirement",
            "link": "",
            "kind": "authored"
          }
        ],
        "contract_bindings": [],
        "concept_references": []
      },
      {
        "id": "system-model/Specifications.md#example-requirement",
        "identifier": "system-model/Specifications.md#example-requirement",
        "label": "Example Requirement",
        "type": "requirement",
        "node_type": "requirement",
        "element_type": "requirement",
        "file_path": "system-model/Specifications.md",
        "line_number": 3,
        "link": "#/content/system-model/Specifications.md#example-requirement",
        "description": "The system shall demonstrate the Explorer shell with fixture data.",
        "metadata": [
          {
            "name": "type",
            "value": "requirement",
            "link": "",
            "kind": "metadata"
          }
        ],
        "governance": [
          {
            "name": "status",
            "value": "draft",
            "link": "",
            "kind": "governance"
          }
        ],
        "outgoing": [
          {
            "name": "specify",
            "value": "Example Capability",
            "link": "",
            "kind": "authored"
          },
          {
            "name": "definedBy",
            "value": "Fixture Interface Contract",
            "link": "",
            "kind": "authored"
          }
        ],
        "incoming": [
          {
            "name": "verify",
            "value": "Example Verification",
            "link": "",
            "kind": "authored"
          },
          {
            "name": "references",
            "value": "Service Endpoint",
            "link": "",
            "kind": "concept-reference"
          }
        ],
        "contract_bindings": [],
        "concept_references": [
          {
            "name": "Service Endpoint",
            "value": "urn:reqvire:test:concepts#ServiceEndpoint",
            "link": "",
            "kind": "concept-reference"
          }
        ]
      },
      {
        "id": "system-model/Specifications.md#example-unverified-requirement",
        "identifier": "system-model/Specifications.md#example-unverified-requirement",
        "label": "Unverified Fixture Requirement",
        "type": "requirement",
        "node_type": "requirement",
        "element_type": "requirement",
        "file_path": "system-model/Specifications.md",
        "line_number": 12,
        "link": "#/content/system-model/Specifications.md#example-unverified-requirement",
        "description": "Requirement branch with no verifying evidence.",
        "metadata": [
          {
            "name": "type",
            "value": "requirement",
            "link": "",
            "kind": "metadata"
          }
        ],
        "governance": [
          {
            "name": "status",
            "value": "draft",
            "link": "",
            "kind": "governance"
          }
        ],
        "outgoing": [
          {
            "name": "specify",
            "value": "Example Capability",
            "link": "",
            "kind": "authored"
          }
        ],
        "incoming": [],
        "contract_bindings": [],
        "concept_references": []
      },
      {
        "id": "system-model/Specifications.md#fixture-interface-contract",
        "identifier": "system-model/Specifications.md#fixture-interface-contract",
        "label": "Fixture Interface Contract",
        "type": "specification",
        "node_type": "specification",
        "element_type": "specification",
        "file_path": "system-model/Specifications.md",
        "line_number": 16,
        "link": "#/content/system-model/Specifications.md#fixture-interface-contract",
        "description": "Requirement-owned fixture contract used as a neighboring graph branch.",
        "metadata": [
          {
            "name": "type",
            "value": "specification",
            "link": "",
            "kind": "metadata"
          }
        ],
        "governance": [],
        "outgoing": [
          {
            "name": "define",
            "value": "Example Requirement",
            "link": "",
            "kind": "authored"
          }
        ],
        "incoming": [],
        "contract_bindings": [],
        "concept_references": []
      },
      {
        "id": "system-model/Specifications.md#fixture-verification-objective",
        "identifier": "system-model/Specifications.md#fixture-verification-objective",
        "label": "Fixture Verification Objective",
        "type": "verification-objective",
        "node_type": "verification-objective",
        "element_type": "verification-objective",
        "file_path": "system-model/Specifications.md",
        "line_number": 18,
        "link": "#/content/system-model/Specifications.md#fixture-verification-objective",
        "description": "Verification objective that groups fixture verification work.",
        "metadata": [
          {
            "name": "type",
            "value": "verification-objective",
            "link": "",
            "kind": "metadata"
          }
        ],
        "governance": [],
        "outgoing": [],
        "incoming": [
          {
            "name": "derive",
            "value": "Example Verification",
            "link": "",
            "kind": "authored"
          },
          {
            "name": "derive",
            "value": "Unsatisfied Fixture Verification",
            "link": "",
            "kind": "authored"
          }
        ],
        "contract_bindings": [],
        "concept_references": []
      },
      {
        "id": "system-model/Specifications.md#example-verification",
        "identifier": "system-model/Specifications.md#example-verification",
        "label": "Example Verification",
        "type": "verification",
        "node_type": "verification",
        "element_type": "test-verification",
        "file_path": "system-model/Specifications.md",
        "line_number": 20,
        "link": "#/content/system-model/Specifications.md#example-verification",
        "description": "Verifies the Example Requirement via a test.",
        "metadata": [
          {
            "name": "type",
            "value": "test-verification",
            "link": "",
            "kind": "metadata"
          }
        ],
        "governance": [],
        "outgoing": [
          {
            "name": "verify",
            "value": "Example Requirement",
            "link": "",
            "kind": "authored"
          },
          {
            "name": "derivedFrom",
            "value": "Fixture Verification Objective",
            "link": "",
            "kind": "authored"
          },
          {
            "name": "satisfiedBy",
            "value": "lib.rs",
            "link": "",
            "kind": "authored"
          }
        ],
        "incoming": [],
        "contract_bindings": [],
        "concept_references": []
      },
      {
        "id": "system-model/Specifications.md#example-unsatisfied-verification",
        "identifier": "system-model/Specifications.md#example-unsatisfied-verification",
        "label": "Unsatisfied Fixture Verification",
        "type": "verification",
        "node_type": "verification",
        "element_type": "test-verification",
        "file_path": "system-model/Specifications.md",
        "line_number": 28,
        "link": "#/content/system-model/Specifications.md#example-unsatisfied-verification",
        "description": "Concrete verification branch with no evidence.",
        "metadata": [
          {
            "name": "type",
            "value": "test-verification",
            "link": "",
            "kind": "metadata"
          }
        ],
        "governance": [],
        "outgoing": [
          {
            "name": "derivedFrom",
            "value": "Fixture Verification Objective",
            "link": "",
            "kind": "authored"
          }
        ],
        "incoming": [],
        "contract_bindings": [],
        "concept_references": []
      },
      {
        "id": "resource:crates/reqvire-core/src/lib.rs",
        "identifier": "resource:crates/reqvire-core/src/lib.rs",
        "label": "lib.rs",
        "type": "resource",
        "node_type": "resource",
        "element_type": "resource",
        "file_path": "crates/reqvire-core/src/lib.rs",
        "line_number": 1,
        "link": "#/content/crates/reqvire-core/src/lib.rs",
        "description": "Fixture implementation evidence resource.",
        "metadata": [
          {
            "name": "kind",
            "value": "local-file",
            "link": "",
            "kind": "metadata"
          }
        ],
        "governance": [],
        "outgoing": [],
        "incoming": [
          {
            "name": "satisfiedBy",
            "value": "Example Verification",
            "link": "",
            "kind": "authored"
          }
        ],
        "contract_bindings": [],
        "concept_references": []
      }
    ],
    "edges": [
      {
        "source": "system-model/Specifications.md#example-requirement",
        "target": "system-model/Specifications.md#example-capability",
        "label": "specify",
        "kind": "authored",
        "authored": true
      },
      {
        "source": "system-model/Specifications.md#example-unverified-requirement",
        "target": "system-model/Specifications.md#example-capability",
        "label": "specify",
        "kind": "authored",
        "authored": true
      },
      {
        "source": "system-model/Specifications.md#fixture-interface-contract",
        "target": "system-model/Specifications.md#example-requirement",
        "label": "define",
        "kind": "authored",
        "authored": true
      },
      {
        "source": "system-model/Specifications.md#example-verification",
        "target": "system-model/Specifications.md#fixture-verification-objective",
        "label": "derivedFrom",
        "kind": "authored",
        "authored": true
      },
      {
        "source": "system-model/Specifications.md#example-verification",
        "target": "system-model/Specifications.md#example-requirement",
        "label": "verify",
        "kind": "authored",
        "authored": true
      },
      {
        "source": "system-model/Specifications.md#example-unsatisfied-verification",
        "target": "system-model/Specifications.md#fixture-verification-objective",
        "label": "derivedFrom",
        "kind": "authored",
        "authored": true
      },
      {
        "source": "system-model/Specifications.md#example-verification",
        "target": "resource:crates/reqvire-core/src/lib.rs",
        "label": "satisfiedBy",
        "kind": "authored",
        "authored": true
      }
    ],
    "submodels": [
      {
        "root_id": "system-model/Specifications.md#example-requirement",
        "root_name": "Example Requirement",
        "root_type": "requirement",
        "requirement_count": 1
      }
    ],
    "summary": {
      "elements": 16,
      "relations": 19,
      "contract_bindings": 0,
      "concept_references": 1,
      "resources": 3,
      "submodels": 1
    }
  },
  "types": [
    "verification-objective",
    "specification",
    "requirement",
    "capability",
    "test-verification",
    "resource"
  ]
} satisfies {title:string; graph:KnowledgeGraphProjection; types:string[]};
