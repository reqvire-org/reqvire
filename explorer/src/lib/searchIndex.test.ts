import { describe, expect, it } from "vitest";
import {
  createProjectSearchIndex,
  searchProjectDocuments,
  tokenizeSearchText,
  type ProjectSearchDocument,
} from "./searchIndex";
import type { SearchKind } from "../search/searchKinds";

const allKinds = new Set<SearchKind>(["element", "file", "resource", "ontology"]);

const documents: ProjectSearchDocument[] = [
  {
    id: "requirements/Capabilities.md#defining-model-structure",
    kind: "element",
    displayKind: "element",
    elementType: "capability",
    title: "Defining Model Structure",
    route: "#/elements/requirements/Capabilities.md#defining-model-structure",
    text: "A capability root for the Reqvire model vocabulary.",
  },
  {
    id: "requirements/Capabilities.md#reqvire-core-element-ontology-shape-profile",
    kind: "element",
    displayKind: "element",
    elementType: "semantic-contract",
    title: "Reqvire Core Element Ontology Shape Profile",
    route: "#/elements/requirements/Capabilities.md#reqvire-core-element-ontology-shape-profile",
    text: "Defines SHACL constraints for core ontology terms.",
  },
  {
    id: "requirements/Functional/Output/Reporting.md#cli-json-file-output-option-contract-specification",
    kind: "element",
    displayKind: "element",
    elementType: "specification",
    title: "CLI JSON File Output Option Contract Specification",
    route: "#/elements/requirements/Functional/Output/Reporting.md#cli-json-file-output-option-contract-specification",
    text: "The --output option writes JSON content to the specified file path.",
  },
  {
    id: "requirements/Functional/Core/Verifications/ReusedContractContextVerifications.md#reuse-command-verification",
    kind: "element",
    displayKind: "element",
    elementType: "test-verification",
    title: "Reuse Command Verification",
    route: "#/elements/requirements/Functional/Core/Verifications/ReusedContractContextVerifications.md#reuse-command-verification",
    text: "test-verification for reused_contract_context behavior.",
  },
  {
    id: "requirements/Ontologies/Core.md",
    kind: "file",
    displayKind: "file",
    title: "Core.md",
    route: "#/content/requirements/Ontologies/Core.md",
    text: "Ontology declarations and class hierarchy.",
  },
  {
    id: "resource:resource:claude-plugins/commands/analyze-coverage.md",
    kind: "file",
    displayKind: "resource",
    title: "claude-plugins/commands/analyze-coverage.md",
    route: "#/content/claude-plugins/commands/analyze-coverage.md",
    text: "Analyze coverage command documentation.",
  },
];

describe("project search index", () => {
  it("splits paths, separators, and camel case into searchable tokens", () => {
    expect(tokenizeSearchText("Capabilities.md#defining-model-structure CLIJsonOutput")).toEqual([
      "capabilities",
      "md",
      "defining",
      "model",
      "structure",
      "cli",
      "json",
      "output",
    ]);
  });

  it("ranks title matches before weaker body/path matches", () => {
    const index = createProjectSearchIndex(documents);
    const results = searchProjectDocuments(index, "ontology shape", allKinds);

    expect(results[0]?.title).toBe("Reqvire Core Element Ontology Shape Profile");
    expect(results.some((result) => result.title === "Core.md")).toBe(true);
  });

  it("matches path tokens and body text for implementation-style queries", () => {
    const index = createProjectSearchIndex(documents);
    const results = searchProjectDocuments(index, "json output", allKinds);

    expect(results[0]?.title).toBe("CLI JSON File Output Option Contract Specification");
  });

  it("supports fuzzy and prefix matching", () => {
    const index = createProjectSearchIndex(documents);
    const results = searchProjectDocuments(index, "verificaton reuse", allKinds);

    expect(results[0]?.title).toBe("Reuse Command Verification");
  });

  it("filters by visible search kind", () => {
    const index = createProjectSearchIndex(documents);
    const results = searchProjectDocuments(index, "coverage", new Set<SearchKind>(["resource"]));

    expect(results).toHaveLength(1);
    expect(results[0]?.title).toBe("claude-plugins/commands/analyze-coverage.md");
  });

  it("keeps empty search in store order and respects kind filters", () => {
    const index = createProjectSearchIndex(documents);
    const results = searchProjectDocuments(index, "", new Set<SearchKind>(["file"]));

    expect(results.map((result) => result.title)).toEqual(["Core.md"]);
  });

  it("filters element results by concrete element type", () => {
    const index = createProjectSearchIndex(documents);
    const results = searchProjectDocuments(
      index,
      "ontology",
      allKinds,
      new Set(["semantic-contract"]),
    );

    const titles = results.map((result) => result.title);
    expect(titles).toContain("Reqvire Core Element Ontology Shape Profile");
    expect(titles).toContain("Core.md");
    expect(titles).not.toContain("Defining Model Structure");
  });
});
