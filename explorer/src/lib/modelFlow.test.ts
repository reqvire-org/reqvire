import { describe, expect, it } from "vitest";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore, ProjectStoreElement, ProjectStoreRelation } from "../store/types";
import { buildModelFlow } from "./modelFlow";

const element = (id: string, file = "model/A.md", type = "requirement"): ProjectStoreElement => ({
  id, name: id, file_path: file, element_type: type, type_family: type, line_number: 1,
  source_anchor: `#/content/${file}#${id}`, content: id, metadata: {}, governance: {},
});
const relation = (source: string, target: string, type = "derivedFrom"): ProjectStoreRelation => ({
  id: `${source}-${type}-${target}`, source_id: source, target_id: target, target_kind: "element",
  relation_type: type, canonical_relation_type: type, source_relation_types: [type],
  authored: true, generated_opposite: false, resource_id: null,
});
const fixture: ExplorerProjectStore = { ...devFixture,
  elements: [element("a"), element("b"), element("merge", "other/B.md"), element("root", "other/B.md", "capability"), element("isolated", "separate/C.md")],
  relations: [{ ...relation("a", "merge"), generated_opposite: true }, relation("b", "merge"), relation("merge", "root", "specify"),
    { ...relation("a", "merge"), id: "same-canonical-edge", source_relation_types: ["derivedFrom", "derive"], generated_opposite: true }],
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
  });
  it("starts with capabilities and descends to requirements, verification, contracts, and evidence", () => {
    const store = { ...fixture, elements: [element("cap", "C.md", "capability"), element("child-cap", "C.md", "capability"),
      element("req"), element("child"), element("check", "V.md", "test-verification"), element("contract", "S.md", "specification")],
      relations: [relation("child-cap", "cap"), relation("req", "child-cap", "specify"), relation("child", "req"),
        relation("check", "child", "verify"), relation("contract", "req", "define")],
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
  it("scopes containers to their elements and immediate endpoints, and elements to full directed paths", () => {
    for (const selection of ["folder:model", "file:model/A.md"]) {
      expect(buildModelFlow(fixture, selection).nodes.map(node => node.element.id).sort()).toEqual(["a", "b", "merge", "root"]);
    }
    expect(buildModelFlow(fixture, "a").nodes.map(node => node.element.id).sort()).toEqual(["a", "merge", "root"]);
    expect(buildModelFlow(fixture, "merge").nodes.map(node => node.element.id).sort()).toEqual(["a", "b", "merge", "root"]);
    expect(buildModelFlow(fixture, "isolated").nodes.map(node => node.element.id)).toEqual(["isolated"]);
    expect(buildModelFlow(fixture, "folder:empty").nodes).toEqual([]);
  });
  it("preserves distinct contracts, concept references, and evidence targets", () => {
    const store: ExplorerProjectStore = { ...fixture,
      elements: [...fixture.elements, element("contract", "S.md", "specification"), element("concept", "Concepts.md", "concept")],
      contract_bindings: [{ id: "binding", source_id: "a", target: "contract", target_kind: "element", resource_id: null, content_hash: null }],
      contract_references: [{ id: "reference", source_id: "b", target: "contract", target_kind: "element", resource_id: null, content_hash: null }],
      concept_refs: [{ id: "concept-ref", source_id: "a", target_element_id: "concept", label: "Concept", iri: "https://example.org/concept", line_number: 1 }],
      resources: [{ id: "evidence", kind: "file", target: "code.rs", display: "code.rs", file_path: "code.rs", external_url: null, referring_element_ids: ["a"], relation_types: ["satisfiedBy"] }],
      relations: [...fixture.relations, { ...relation("a", "code.rs", "satisfiedBy"), target_kind: "resource", resource_id: "evidence" }],
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
  });
  it("traverses non-hierarchical cycles safely without changing edge direction", () => {
    const data = buildModelFlow({ ...fixture, relations: [...fixture.relations, relation("a", "root", "trace")] }, "a");
    expect(data.nodes.map(node => node.element.id).sort()).toEqual(["a", "b", "merge", "root"]);
    expect(data.edges.some(edge => edge.source === "a" && edge.target === "root" && edge.label === "trace")).toBe(true);
  });
});
