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
    expect(parseHash("#/knowledge-graph", "model").view).toBe("model");
    expect(parseHash("#/ontologies", "model").view).toBe("ontologies");
  });

  it("treats element routes as overlays over the previous view", () => {
    const r = parseHash(
      "#/elements/system-model/Specifications.md#example-requirement",
      "model",
    );
    expect(r.view).toBe("model");
    expect(r.elementId).toBe("system-model/Specifications.md#example-requirement");
  });

  it("parses file routes with their path param", () => {
    const r = parseHash("#/files/system-model/Specifications.md", "model");
    expect(r.view).toBe("files");
    expect(r.param).toBe("system-model/Specifications.md");
  });

  it("parses resource routes with their id param", () => {
    const r = parseHash("#/resources/resource:crates/reqvire-core/src/lib.rs", "model");
    expect(r.view).toBe("resources");
    expect(r.param).toBe("resource:crates/reqvire-core/src/lib.rs");
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
    expect(routeForElement("a/b.md#c")).toBe("#/elements/a/b.md#c");
    expect(parseHash(routeForElement("a/b.md#c"), "model").elementId).toBe("a/b.md#c");
  });
});
