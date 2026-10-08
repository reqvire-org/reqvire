import { describe, expect, it } from "vitest";
import { devFixture } from "../store/devFixture";
import { routeForContent, routeForElement } from "../router/routes";
import { buildVerificationFlow } from "./traceFlow";
import type { TraceVerificationNode } from "./traces";

const trace: TraceVerificationNode = {
  id: "Checks & Tests.md#verification", name: "Check", file: "Checks & Tests.md",
  verificationType: "test-verification", directCount: 2, totalCount: 4, requirementIds: ["a", "root"],
  traceGraph: {
    nodes: ["a", "left", "right", "root", "capability", "capability-root"].map(id => ({
      id, name: id, type: id.startsWith("capability") ? "capability" : id === "root" ? "system-requirement" : "requirement",
      is_directly_verified: id === "a" || id === "root",
    })),
    edges: [
      ["a", "derivedFrom", "left"], ["a", "derivedFrom", "right"],
      ["left", "derivedFrom", "root"], ["right", "derivedFrom", "root"],
      ["root", "specify", "capability"], ["capability", "derivedFrom", "capability-root"],
      ["root", "specify", "capability"],
    ].map(([source, relation_type, target]) => ({ source, relation_type, target })),
  },
};

describe("published verification flow adapter", () => {
  it("preserves evaluated shared paths, capability roles, labels and requirement-only counts", () => {
    const original = JSON.stringify(trace);
    const data = buildVerificationFlow(trace, new Map());
    const graph = data.graph!;
    expect(graph.nodes.map(node => node.element.id)).toEqual([trace.id, "a", "left", "right", "root", "capability", "capability-root"]);
    expect(graph.edges).toHaveLength(8);
    expect(graph.edges.filter(edge => edge.target === "root")).toHaveLength(3);
    expect(graph.edges.find(edge => edge.source === "root" && edge.target === "capability")?.label).toBe("specify");
    expect(graph.nodes.find(node => node.element.id === "capability")?.type).toBe("capability");
    expect(graph.nodes.find(node => node.element.id === "root")?.type).toBe("system-requirement");
    expect(new Set(data.requirements.filter(node => node.directlyVerified).map(node => node.id)).size).toBe(2);
    expect(new Set(data.requirements.map(node => node.id)).size).toBe(4);
    expect(JSON.stringify(trace)).toBe(original);
  });
  it("uses published labels, current source files and canonical navigation routes", () => {
    const data = buildVerificationFlow(trace, new Map([["a", { ...devFixture.elements[0], name: "Unrelated", file_path: "Moved & New.md" }]]));
    const node = data.graph!.nodes.find(node => node.element.id === "a")!;
    expect(node.element.name).toBe("a");
    expect(node.element.file).toBe("Moved & New.md");
    expect(node.element.sourceHref).toBe(routeForContent("Moved & New.md"));
    expect(data.verification.href).toBe(routeForElement(trace.id));
    expect(data.verification.sourceHref).toBe(routeForContent(trace.file));
  });
  it("adapts older direct-only projections and keeps an unlinked verification visible", () => {
    const legacy = { ...trace, traceGraph: undefined, requirementIds: ["a", "a"] };
    const data = buildVerificationFlow(legacy, new Map());
    const graph = data.graph!;
    expect(graph.nodes.map(node => node.element.id)).toEqual([trace.id, "a"]);
    expect(graph.edges).toHaveLength(1);
    expect(new Set(data.requirements.map(node => node.id)).size).toBe(1);
    const empty = buildVerificationFlow({ ...legacy, requirementIds: [] }, new Map());
    expect(empty.graph!.nodes.map(node => node.element.id)).toEqual([trace.id]);
    expect(empty.graph!.edges).toEqual([]);
    expect(empty.requirements).toEqual([]);
  });
});
