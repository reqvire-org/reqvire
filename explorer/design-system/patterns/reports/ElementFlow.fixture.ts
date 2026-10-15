export const fixture = {
  "title": "ElementFlow",
  "data": {
    "id": "__root__",
    "title": "reqvire workspace",
    "nodes": [
      {
        "type": "capability",
        "context": "capability",
        "element": {
          "id": "system-model/Capabilities/API.md#api-exploration-capability",
          "name": "API Exploration Capability",
          "file": "system-model/Capabilities/API.md",
          "href": "#/elements/system-model/Capabilities/API.md#api-exploration-capability",
          "sourceHref": "#/content/system-model/Capabilities/API.md#api-exploration-capability"
        },
        "root": true
      },
      {
        "type": "requirement",
        "context": "requirement",
        "element": {
          "id": "system-model/Requirements/API.md#api-surface-exploration",
          "name": "API Surface Exploration",
          "file": "system-model/Requirements/API.md",
          "href": "#/elements/system-model/Requirements/API.md#api-surface-exploration",
          "sourceHref": "#/content/system-model/Requirements/API.md#api-surface-exploration"
        },
        "root": false
      },
      {
        "type": "requirement",
        "context": "requirement",
        "element": {
          "id": "system-model/Requirements/API.md#api-evidence-traceability",
          "name": "API Evidence Traceability",
          "file": "system-model/Requirements/API.md",
          "href": "#/elements/system-model/Requirements/API.md#api-evidence-traceability",
          "sourceHref": "#/content/system-model/Requirements/API.md#api-evidence-traceability"
        },
        "root": false
      },
      {
        "type": "specification",
        "context": "specification",
        "element": {
          "id": "system-model/Contracts/API.md#api-response-contract",
          "name": "API Response Contract",
          "file": "system-model/Contracts/API.md",
          "href": "#/elements/system-model/Contracts/API.md#api-response-contract",
          "sourceHref": "#/content/system-model/Contracts/API.md#api-response-contract"
        },
        "root": false
      },
      {
        "type": "test-verification",
        "context": "test verification",
        "element": {
          "id": "system-model/Verifications/API.md#api-smoke-test-verification",
          "name": "API Smoke Test Verification",
          "file": "system-model/Verifications/API.md",
          "href": "#/elements/system-model/Verifications/API.md#api-smoke-test-verification",
          "sourceHref": "#/content/system-model/Verifications/API.md#api-smoke-test-verification"
        },
        "root": false
      },
      {
        "type": "analysis-verification",
        "context": "analysis verification",
        "element": {
          "id": "system-model/Verifications/API.md#api-contract-analysis",
          "name": "API Contract Analysis",
          "file": "system-model/Verifications/API.md",
          "href": "#/elements/system-model/Verifications/API.md#api-contract-analysis",
          "sourceHref": "#/content/system-model/Verifications/API.md#api-contract-analysis"
        },
        "root": false
      },
      {
        "type": "capability",
        "context": "capability",
        "element": {
          "id": "system-model/Specifications.md#example-capability",
          "name": "Example Capability",
          "file": "system-model/Specifications.md",
          "href": "#/elements/system-model/Specifications.md#example-capability",
          "sourceHref": "#/content/system-model/Specifications.md#example-capability"
        },
        "root": true
      },
      {
        "type": "requirement",
        "context": "requirement",
        "element": {
          "id": "system-model/Specifications.md#example-requirement",
          "name": "Example Requirement",
          "file": "system-model/Specifications.md",
          "href": "#/elements/system-model/Specifications.md#example-requirement",
          "sourceHref": "#/content/system-model/Specifications.md#example-requirement"
        },
        "root": false
      },
      {
        "type": "requirement",
        "context": "requirement",
        "element": {
          "id": "system-model/Specifications.md#example-unverified-requirement",
          "name": "Unverified Fixture Requirement",
          "file": "system-model/Specifications.md",
          "href": "#/elements/system-model/Specifications.md#example-unverified-requirement",
          "sourceHref": "#/content/system-model/Specifications.md#example-unverified-requirement"
        },
        "root": false
      },
      {
        "type": "test-verification",
        "context": "test verification",
        "element": {
          "id": "system-model/Specifications.md#example-verification",
          "name": "Example Verification",
          "file": "system-model/Specifications.md",
          "href": "#/elements/system-model/Specifications.md#example-verification",
          "sourceHref": "#/content/system-model/Specifications.md#example-verification"
        },
        "root": false
      },
      {
        "type": "resource",
        "context": "local file",
        "element": {
          "id": "resource:system-model/Evidence/api-smoke-report.json",
          "name": "api-smoke-report.json",
          "file": "system-model/Evidence/api-smoke-report.json",
          "href": "#/resources/resource:system-model/Evidence/api-smoke-report.json",
          "sourceHref": "#/resources/resource:system-model/Evidence/api-smoke-report.json"
        },
        "root": false
      },
      {
        "type": "resource",
        "context": "local file",
        "element": {
          "id": "resource:system-model/Evidence/openapi-fragment.yaml",
          "name": "openapi-fragment.yaml",
          "file": "system-model/Evidence/openapi-fragment.yaml",
          "href": "#/resources/resource:system-model/Evidence/openapi-fragment.yaml",
          "sourceHref": "#/resources/resource:system-model/Evidence/openapi-fragment.yaml"
        },
        "root": false
      }
    ],
    "edges": [
      {
        "id": "[\"system-model/Specifications.md#example-capability\",\"system-model/Specifications.md#example-requirement\",\"specifiedBy\"]",
        "source": "system-model/Specifications.md#example-capability",
        "target": "system-model/Specifications.md#example-requirement",
        "label": "specifiedBy"
      },
      {
        "id": "[\"system-model/Specifications.md#example-capability\",\"system-model/Specifications.md#example-unverified-requirement\",\"specifiedBy\"]",
        "source": "system-model/Specifications.md#example-capability",
        "target": "system-model/Specifications.md#example-unverified-requirement",
        "label": "specifiedBy"
      },
      {
        "id": "[\"system-model/Capabilities/API.md#api-exploration-capability\",\"system-model/Requirements/API.md#api-surface-exploration\",\"specifiedBy\"]",
        "source": "system-model/Capabilities/API.md#api-exploration-capability",
        "target": "system-model/Requirements/API.md#api-surface-exploration",
        "label": "specifiedBy"
      },
      {
        "id": "[\"system-model/Capabilities/API.md#api-exploration-capability\",\"system-model/Requirements/API.md#api-evidence-traceability\",\"specifiedBy\"]",
        "source": "system-model/Capabilities/API.md#api-exploration-capability",
        "target": "system-model/Requirements/API.md#api-evidence-traceability",
        "label": "specifiedBy"
      },
      {
        "id": "[\"system-model/Requirements/API.md#api-surface-exploration\",\"system-model/Contracts/API.md#api-response-contract\",\"definedBy\"]",
        "source": "system-model/Requirements/API.md#api-surface-exploration",
        "target": "system-model/Contracts/API.md#api-response-contract",
        "label": "definedBy"
      },
      {
        "id": "[\"system-model/Requirements/API.md#api-surface-exploration\",\"system-model/Verifications/API.md#api-smoke-test-verification\",\"verifiedBy\"]",
        "source": "system-model/Requirements/API.md#api-surface-exploration",
        "target": "system-model/Verifications/API.md#api-smoke-test-verification",
        "label": "verifiedBy"
      },
      {
        "id": "[\"system-model/Requirements/API.md#api-evidence-traceability\",\"system-model/Verifications/API.md#api-contract-analysis\",\"verifiedBy\"]",
        "source": "system-model/Requirements/API.md#api-evidence-traceability",
        "target": "system-model/Verifications/API.md#api-contract-analysis",
        "label": "verifiedBy"
      },
      {
        "id": "[\"system-model/Verifications/API.md#api-smoke-test-verification\",\"resource:system-model/Evidence/api-smoke-report.json\",\"satisfiedBy\"]",
        "source": "system-model/Verifications/API.md#api-smoke-test-verification",
        "target": "resource:system-model/Evidence/api-smoke-report.json",
        "label": "satisfiedBy"
      },
      {
        "id": "[\"system-model/Verifications/API.md#api-contract-analysis\",\"resource:system-model/Evidence/openapi-fragment.yaml\",\"satisfiedBy\"]",
        "source": "system-model/Verifications/API.md#api-contract-analysis",
        "target": "resource:system-model/Evidence/openapi-fragment.yaml",
        "label": "satisfiedBy"
      },
      {
        "id": "[\"system-model/Specifications.md#example-requirement\",\"system-model/Specifications.md#example-verification\",\"verifiedBy\"]",
        "source": "system-model/Specifications.md#example-requirement",
        "target": "system-model/Specifications.md#example-verification",
        "label": "verifiedBy"
      }
    ]
  }
};
