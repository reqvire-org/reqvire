import { describe, expect, it } from "vitest";
import { parseHash, routeForElement, routeForView } from "./routes";

describe("parseHash", () => {
  it("defaults empty hash to model", () => {
    expect(parseHash("", "model")).toEqual({
      view: "model",
      param: null,
      elementId: null,
    });
    expect(parseHash("#/", "model")).toEqual({
      view: "model",
      param: null,
      elementId: null,
    });
  });

  it("parses primary view routes", () => {
    expect(parseHash("#/model", "model").view).toBe("model");
    expect(parseHash("#/knowledge-graph", "model").view).toBe("knowledge-graph");
    expect(parseHash("#/kn2", "model").view).toBe("kn2");
    expect(parseHash("#/ontologies", "model").view).toBe("ontologies");
  });

  it("treats element routes as overlays over the previous view", () => {
    const r = parseHash(
      "#/elements/requirements/Specifications.md#example-requirement",
      "model",
    );
    expect(r.view).toBe("model");
    expect(r.elementId).toBe("requirements/Specifications.md#example-requirement");
  });

  it("parses file routes with their path param", () => {
    const r = parseHash("#/files/requirements/Specifications.md", "model");
    expect(r.view).toBe("files");
    expect(r.param).toBe("requirements/Specifications.md");
  });

  it("parses search routes with query", () => {
    expect(parseHash("#/search", "model").view).toBe("search");
    expect(parseHash("#/search/requirement", "model").param).toBe("requirement");
  });

  it("defaults unknown routes to model", () => {
    expect(parseHash("#/nope", "model").view).toBe("model");
  });

  it("round-trips view and element route builders", () => {
    expect(routeForView("traces")).toBe("#/traces");
    expect(routeForView("knowledge-graph")).toBe("#/knowledge-graph");
    expect(routeForView("kn2")).toBe("#/kn2");
    expect(routeForElement("a/b.md#c")).toBe("#/elements/a/b.md#c");
    expect(parseHash(routeForElement("a/b.md#c"), "model").elementId).toBe("a/b.md#c");
  });
});
