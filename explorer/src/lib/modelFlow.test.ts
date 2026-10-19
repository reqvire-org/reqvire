import { describe, expect, it } from "vitest";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore, ProjectStoreElement, ProjectStoreRelation } from "../store/types";
import { buildModelFlow, prepareModelFlow, selectModelFlow } from "./modelFlow";

const element = (id: string, file = "model/A.md", type = "requirement"): ProjectStoreElement => ({
  id, name: id, file_path: file, element_type: type, type_family: type, line_number: 1,
  source_anchor: `#/content/${file}#${id}`, content: id, metadata: {}, governance: {},
});
const relation = (source: string, target: string, type = "derive", inverse = type === "derive" ? "derivedFrom" : undefined): ProjectStoreRelation => ({
  id: `${source}-${type}-${target}`, source_id: source, target_id: target, target_kind: "element",
  relation_type: type, canonical_relation_type: type, source_relation_types: [type],
  inverse_relation_type: inverse,
  authored: true, generated_opposite: false, resource_id: null,
});
const fixture: ExplorerProjectStore = { ...devFixture,
  elements: [element("a"), element("b"), element("merge", "other/B.md"), element("root", "other/B.md", "capability"), element("isolated", "separate/C.md")],
  relations: [{ ...relation("merge", "a"), generated_opposite: true }, relation("merge", "b"), relation("root", "merge", "specifiedBy", "specify"),
    { ...relation("merge", "a"), id: "same-canonical-edge", source_relation_types: ["derivedFrom", "derive"], generated_opposite: true }],
  contract_bindings: [], contract_references: [], concept_refs: [], resources: [],
};

describe("Model Flow projection", () => {
  it("retains directed relations and shared nodes while collapsing generated inverses", () => {
    const data = buildModelFlow(fixture, "__root__");
    expect(data.nodes.map(node => node.element.id).sort()).toEqual(["a", "b", "merge", "root"]);
    expect(data.edges.map(edge => [edge.source, edge.target, edge.label])).toEqual([
      ["merge", "a", "derive"], ["merge", "b", "derive"], ["root", "merge", "specifiedBy"],
    ]);
    expect(data.nodes.find(node => node.element.id === "root")?.type).toBe("capability");
    expect(data.nodes.find(node => node.element.id === "root")?.root).toBe(true);
    const suppliedVocabulary = buildModelFlow({ ...fixture,
      relations: [relation("root", "a", "customForward", "customInverse")],
    }, "a", "upstream");
    expect(suppliedVocabulary.edges).toEqual([
      expect.objectContaining({ source: "root", target: "a", label: "customInverse", reversed: true }),
    ]);
  });
  it("starts with capabilities and descends to requirements, verification, contracts, and evidence", () => {
    const store = { ...fixture, elements: [element("cap", "C.md", "capability"), element("child-cap", "C.md", "capability"),
      element("req"), element("child"), element("check", "V.md", "test-verification"), element("contract", "S.md", "specification")],
      relations: [relation("cap", "child-cap"), relation("child-cap", "req", "specifiedBy", "specify"), relation("req", "child"),
        relation("child", "check", "verifiedBy", "verify"), relation("req", "contract", "definedBy", "define")],
    };
    const snapshot = JSON.stringify(store);
    const data = buildModelFlow(store, "__root__");
    expect(data.edges.map(edge => [edge.source, edge.target, edge.label])).toEqual([
      ["cap", "child-cap", "derive"], ["child-cap", "req", "specifiedBy"], ["req", "child", "derive"],
      ["child", "check", "verifiedBy"], ["req", "contract", "definedBy"],
    ]);
    expect(data.nodes.filter(node => node.root).map(node => node.element.id)).toEqual(["cap"]);
    expect(JSON.stringify(store)).toBe(snapshot);
  });
  it("follows one horizon from element and page seeds, merging shared paths without adding siblings", () => {
    const store = { ...fixture, elements: [...fixture.elements, element("shared", "other/C.md"), element("leaf", "other/D.md")],
      relations: [...fixture.relations, relation("a", "shared"), relation("b", "shared"), relation("shared", "leaf")],
    };
    const snapshot = JSON.stringify(store);
    const index = prepareModelFlow(store);
    const ids = (selection: string, scope: "downstream" | "upstream" = "downstream") =>
      selectModelFlow(index, selection, "Model", scope).nodes.map(node => node.element.id).sort();
    for (const selection of ["folder:model", "file:model/A.md"]) {
      expect(ids(selection)).toEqual(["a", "b", "leaf", "shared"]);
      expect(ids(selection, "upstream")).toEqual(["a", "b", "merge", "root"]);
    }
    expect(ids("a")).toEqual(["a", "leaf", "shared"]);
    expect(ids("a", "upstream")).toEqual(["a", "merge", "root"]);
    // Related seeds retain their edge rather than being forced into one rank.
    expect(selectModelFlow(index, "file:other/B.md", "Model", "upstream").edges).toEqual([
      expect.objectContaining({ source: "root", target: "merge", label: "specify", reversed: true }),
    ]);
    expect(ids("isolated")).toEqual(["isolated"]);
    expect(ids("isolated", "upstream")).toEqual(["isolated"]);
    expect(ids("folder:empty")).toEqual([]);
    expect(ids("missing", "upstream")).toEqual([]);
    expect(JSON.stringify(store)).toBe(snapshot);
  });
  it("preserves distinct contracts, concept references, and evidence targets", () => {
    const store: ExplorerProjectStore = { ...fixture,
      elements: [...fixture.elements, element("contract", "S.md", "specification"), element("concept", "Concepts.md", "concept")],
      contract_bindings: [{ id: "binding", source_id: "a", target: "contract", target_kind: "element", resource_id: null, content_hash: null }],
      contract_references: [{ id: "reference", source_id: "b", target: "contract", target_kind: "element", resource_id: null, content_hash: null }],
      concept_refs: [{ id: "concept-ref", source_id: "a", target_element_id: "concept", label: "Concept", iri: "https://example.org/concept", line_number: 1 }],
      resources: [{ id: "evidence", kind: "file", target: "code.rs", display: "code.rs", file_path: "code.rs", external_url: null, referring_element_ids: ["a"], relation_types: ["satisfiedBy"] }],
      relations: [...fixture.relations, { ...relation("a", "code.rs", "satisfiedBy", "satisfy"), target_kind: "resource", resource_id: "evidence" }],
    };
    const data = buildModelFlow(store, "__root__");
    expect(data.edges).toEqual(expect.arrayContaining([
      expect.objectContaining({ source: "a", target: "contract", label: "Contract binding" }),
      expect.objectContaining({ source: "b", target: "contract", label: "Contract reference" }),
      expect.objectContaining({ source: "a", target: "concept", label: "Concept reference" }),
      expect.objectContaining({ source: "a", target: "evidence", label: "satisfiedBy" }),
    ]));
    expect(data.nodes.filter(node => node.element.id === "contract")).toHaveLength(1);
    expect(data.nodes.find(node => node.element.id === "evidence")?.element.href).toBe("#/resources/evidence");
    expect(buildModelFlow(store, "evidence", "upstream").edges).toContainEqual(
      expect.objectContaining({ source: "a", target: "evidence", label: "satisfy", reversed: true }));
    expect(buildModelFlow(store, "contract", "upstream").edges).toContainEqual(
      expect.objectContaining({ source: "a", target: "contract", label: "Contract binding" }));
  });
  it("traverses non-hierarchical cycles safely without changing edge direction", () => {
    const store = { ...fixture, relations: [...fixture.relations, relation("a", "root", "trace")] };
    const data = buildModelFlow(store, "a");
    expect(data.nodes.map(node => node.element.id).sort()).toEqual(["a", "b", "merge", "root"]);
    expect(data.edges.some(edge => edge.source === "a" && edge.target === "root" && edge.label === "trace")).toBe(true);
    const upstream = buildModelFlow(store, "a", "upstream");
    expect(upstream.nodes.map(node => node.element.id).sort()).toEqual(["a", "merge", "root"]);
    expect(upstream.edges).toContainEqual(expect.objectContaining({ source: "a", target: "root", label: "trace" }));
  });
});
