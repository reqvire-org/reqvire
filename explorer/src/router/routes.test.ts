import { describe, expect, it } from "vitest";
import { parseHash, routeForCoverage, routeForSelection, routeForElement, routeForResource, routeForView, worktreeUrl } from "./routes";

describe("parseHash", () => {
  it("round-trips view selections with reserved characters and keeps them under overlays", () => {
    const id = "model/Scope ü +&?%.md#child";
    for (const view of ["model", "traces", "thesaurus", "ontologies"] as const) {
      const hash = routeForSelection(view, id, { mode: view === "model" ? "flow" : undefined });
      const parsed = parseHash(hash, "model");
      expect(new URLSearchParams(parsed.param!).get("selected")).toBe(id);
      expect(parseHash(routeForElement("other.md#element"), parsed).param).toBe(parsed.param);
      expect(worktreeUrl(`https://example.test/?worktree_id=a${hash}`, "b").hash).toBe(`#/${view}`);
    }
    expect(new URLSearchParams(parseHash(routeForSelection("traces", null, { file: id }), "model").param!).get("file")).toBe(id);
  });


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

  it("round-trips explicit coverage scopes, including Whole model and reserved characters", () => {
    const id = "specifications/Scope ü +&?%.md#child";
    expect(routeForCoverage(id)).toBe(`#/coverage?scope=${encodeURIComponent(id)}`);
    expect(new URLSearchParams(parseHash(routeForCoverage(id), "model").param!).get("scope")).toBe(id);
    expect(new URLSearchParams(parseHash(routeForCoverage(null), "model").param!).get("scope")).toBe("");
    expect(parseHash("#/coverage", "model").param).toBeNull();
    expect(parseHash("#/coverage?other=value", "model").param).toBe("other=value");
  });

  it("retains coverage scope beneath element overlays", () => {
    expect(parseHash(routeForElement("a.md#element"), { view: "coverage", param: "scope=b.md%23scope&mode=issues&issue=unimplemented-requirements" }))
      .toEqual({ view: "coverage", param: "scope=b.md%23scope&mode=issues&issue=unimplemented-requirements", elementId: "a.md#element" });
  });

  it("resumes the target worktree scope without carrying the previous scope", () => {
    const url = worktreeUrl(`https://example.test/export/index.html?theme=dark&worktree_id=a${routeForCoverage("a.md#capability")}`, "b");
    expect(url.pathname).toBe("/export/index.html");
    expect(url.searchParams.get("theme")).toBe("dark");
    expect(url.searchParams.get("worktree_id")).toBe("b");
    expect(url.hash).toBe("#/coverage");
    expect(worktreeUrl("https://example.test/#/content/a.md", "b").hash).toBe("#/content/a.md");
  });

  it("treats element routes as overlays over the previous view", () => {
    const r = parseHash(
      "#/elements/system-model/Specifications.md#example-requirement",
      "model",
    );
    expect(r.view).toBe("model");
    expect(r.elementId).toBe("system-model/Specifications.md#example-requirement");
  });

  it("preserves previous route params for element overlays", () => {
    const r = parseHash(
      "#/elements/system-model/Thesaurus/Thesaurus.md#access-token",
      { view: "content", param: "system-model/Specifications.md" },
    );
    expect(r.view).toBe("content");
    expect(r.param).toBe("system-model/Specifications.md");
    expect(r.elementId).toBe("system-model/Thesaurus/Thesaurus.md#access-token");
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
    expect(routeForResource("resource:a/b.txt")).toBe("#/resources/resource:a/b.txt");
    expect(parseHash(routeForElement("a/b.md#c"), "model").elementId).toBe("a/b.md#c");
  });
});
