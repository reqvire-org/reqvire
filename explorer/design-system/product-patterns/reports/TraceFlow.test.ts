import { describe, expect, it } from "vitest";
import { buildTraceFlowGraph, buildTraceFlowTopology, traceFlowNeighborhood, TRACE_NODE_HEIGHT, TRACE_NODE_WIDTH, type TraceFlowData, type TraceFlowDirection } from "./traceFlowLayout";
import { SPLIT_MERGE_TRACE } from "../../showcase/fixtures/traces";

const trace: TraceFlowData = {
  verification: { id: "v", name: "Check", type: "test-verification", file: "Checks.md" },
  requirements: [
    { id: "a", name: "A", file: "Requirements.md", directlyVerified: true, parentIds: ["shared"] },
    { id: "b", name: "B", file: "Requirements.md", directlyVerified: true, parentIds: ["shared", "exclusive"] },
    { id: "shared", name: "Shared", file: "Requirements.md", directlyVerified: false, parentIds: ["root"] },
    { id: "exclusive", name: "Exclusive", file: "Requirements.md", directlyVerified: false, parentIds: [] },
    { id: "root", name: "Root", file: "Requirements.md", directlyVerified: false, parentIds: [] },
  ],
};

const layoutTrace = (data: TraceFlowData, collapsed?: ReadonlySet<string>, direction?: TraceFlowDirection) => buildTraceFlowGraph(buildTraceFlowTopology(data, collapsed), direction);

describe("native trace flow", () => {
  it("lays out a cyclic model context and reflexive relation without losing connections", async () => {
    const graph = await buildTraceFlowGraph({ totalCount: 2,
      nodes: ["a", "b"].map(id => ({ id, element: { id, name: id, file: "Model.md" }, type: "requirement", context: "requirement", parentCount: 0 })),
      edges: [{ id: "ab", source: "a", target: "b", label: "trace" },
        { id: "ba", source: "b", target: "a", label: "trace" }, { id: "aa", source: "a", target: "a", label: "related" }],
    }, "DOWN");
    expect(graph.edges).toHaveLength(3);
    expect(graph.edges.every(edge => edge.path.startsWith("M ") && !edge.path.includes("NaN"))).toBe(true);
  });
  it("highlights paths through a chosen node without leaking into sibling-only paths", async () => {
    const graph = await layoutTrace(SPLIT_MERGE_TRACE.trace);
    const selected = traceFlowNeighborhood(graph.edges, "source-context");
    expect([...selected.nodeIds].sort()).toEqual([
      "source-context", "input-b", "split-merge-test", "interactive-view", "presentation", "trace-root",
    ].sort());
    expect(selected.edgeIds.has(JSON.stringify(["input-a", "trace-root"]))).toBe(false);
    expect(selected.edgeIds.has(JSON.stringify(["normalize", "interactive-view"]))).toBe(false);
  });
  it.each(["RIGHT", "DOWN"] as const)("lays out repeated splits and merges independently of input ordering (%s)", async direction => {
    const data = SPLIT_MERGE_TRACE.trace;
    const first = await layoutTrace(data, undefined, direction);
    const second = await layoutTrace({ ...data,
      requirements: [...data.requirements].reverse().map(node => ({ ...node, parentIds: [...node.parentIds].reverse() })),
    }, undefined, direction);
    for (const node of first.nodes) {
      expect(second.nodes.find(item => item.id === node.id)?.position).toEqual(node.position);
    }
    expect(first.nodes).toHaveLength(9);
    expect(first.edges).toHaveLength(12);
    for (const edge of first.edges) {
      expect(second.edges.find(item => item.id === edge.id)?.points).toEqual(edge.points);
    }
  });

  it.each(["RIGHT", "DOWN"] as const)("routes shortcuts around cards and reserves relation label space (%s)", async direction => {
    const graph = await layoutTrace(SPLIT_MERGE_TRACE.trace, undefined, direction);
    const cards = graph.nodes.map(node => ({
      id: node.id, x: node.position.x, y: node.position.y, width: TRACE_NODE_WIDTH, height: TRACE_NODE_HEIGHT,
    }));
    const insideLayout = (rectangle: { x: number; y: number; width: number; height: number }) => {
      expect(rectangle.x).toBeGreaterThanOrEqual(graph.bounds.x);
      expect(rectangle.y).toBeGreaterThanOrEqual(graph.bounds.y);
      expect(rectangle.x + rectangle.width).toBeLessThanOrEqual(graph.bounds.x + graph.bounds.width);
      expect(rectangle.y + rectangle.height).toBeLessThanOrEqual(graph.bounds.y + graph.bounds.height);
    };
    for (const card of cards) insideLayout(card);
    const intersects = (a: { x: number; y: number; width: number; height: number }, b: typeof a) =>
      a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y;
    for (const card of cards) for (const other of cards) {
      if (card.id !== other.id) expect(intersects(card, other)).toBe(false);
    }
    for (const edge of graph.edges) {
      insideLayout(edge.labelBounds);
      for (const point of edge.points) insideLayout({ ...point, width: 0, height: 0 });
      expect(edge.points?.length).toBeGreaterThanOrEqual(2);
      const source = cards.find(card => card.id === edge.source)!;
      const target = cards.find(card => card.id === edge.target)!;
      const start = edge.points[0];
      const end = edge.points[edge.points.length - 1];
      if (direction === "RIGHT") {
        expect(start.x).toBeCloseTo(source.x + source.width);
        expect(end.x).toBeCloseTo(target.x);
        expect(start.y).toBeGreaterThanOrEqual(source.y);
        expect(start.y).toBeLessThanOrEqual(source.y + source.height);
        expect(end.y).toBeGreaterThanOrEqual(target.y);
        expect(end.y).toBeLessThanOrEqual(target.y + target.height);
        expect(target.x).toBeGreaterThan(source.x);
      } else {
        expect(start.y).toBeCloseTo(source.y + source.height);
        expect(end.y).toBeCloseTo(target.y);
        expect(start.x).toBeGreaterThanOrEqual(source.x);
        expect(start.x).toBeLessThanOrEqual(source.x + source.width);
        expect(end.x).toBeGreaterThanOrEqual(target.x);
        expect(end.x).toBeLessThanOrEqual(target.x + target.width);
        expect(target.y).toBeGreaterThan(source.y);
      }
      expect(edge.path.startsWith(`M ${start.x} ${start.y}`)).toBe(true);
      expect(edge.path.endsWith(`L ${end.x} ${end.y}`)).toBe(true);
      const label = edge.labelBounds;
      for (const card of cards) expect(intersects(label, card)).toBe(false);
      for (const other of graph.edges) {
        if (edge.id !== other.id) expect(intersects(label, other.labelBounds)).toBe(false);
      }
      // Sample the actual routed polyline, including skipped ranks, for card intrusion.
      for (let index = 1; index < edge.points.length; index++) {
        const from = edge.points[index - 1];
        const to = edge.points[index];
        expect(Math.min(Math.abs(from.x - to.x), Math.abs(from.y - to.y)),
          `ELK route ${edge.id} must be orthogonal: ${JSON.stringify([from, to])}`).toBeLessThan(0.000001);
        const steps = Math.ceil(Math.hypot(to.x - from.x, to.y - from.y));
        for (let step = 0; step <= steps; step++) {
          const t = steps ? step / steps : 0;
          const x = from.x + (to.x - from.x) * t;
          const y = from.y + (to.y - from.y) * t;
          for (const card of cards) {
            if (card.id === edge.source || card.id === edge.target) continue;
            expect(x > card.x && x < card.x + card.width && y > card.y && y < card.y + card.height,
              `${edge.source} → ${edge.target} crosses ${card.id} at ${x},${y}; route ${JSON.stringify(edge.points)}`).toBe(false);
          }
        }
      }
    }
  });

  it("merges shared ancestors, counts unique requirements, and keeps directed relations", async () => {
    const graph = await layoutTrace({ ...trace, requirements: [...trace.requirements, trace.requirements[2]] });
    expect(graph.nodes.map(node => node.id)).toEqual(["v", "a", "b", "shared", "exclusive", "root"]);
    expect(graph.directCount).toBe(2);
    expect(graph.totalCount).toBe(5);
    expect(graph.edges.map(({ source, target, label }) => [source, target, label])).toEqual([
      ["v", "a", "verifies"], ["v", "b", "verifies"],
      ["a", "shared", "derivedFrom"], ["b", "shared", "derivedFrom"],
      ["b", "exclusive", "derivedFrom"], ["shared", "root", "derivedFrom"],
    ]);
    for (const edge of graph.edges) {
      expect(graph.nodes.find(node => node.id === edge.target)!.position.x)
        .toBeGreaterThan(graph.nodes.find(node => node.id === edge.source)!.position.x);
    }
  });

  it("preserves a shared ancestor reachable through another expanded branch", async () => {
    const graph = await layoutTrace(trace, new Set(["b"]));
    expect(graph.nodes.map(node => node.id)).toEqual(["v", "a", "b", "shared", "root"]);
    expect(graph.edges.some(edge => edge.source === "b")).toBe(false);
    expect(graph.totalCount).toBe(5);
    expect((await layoutTrace(trace)).nodes.some(node => node.id === "exclusive")).toBe(true);
    const collapsed = await layoutTrace(trace, new Set(["a", "b"]));
    expect(collapsed.nodes.map(node => node.id)).toEqual(["v", "a", "b"]);
  });

  it("places an ancestor after every incoming path, including a direct verification link", async () => {
    const graph = await layoutTrace({ ...trace, requirements: trace.requirements.map(node => ({
      ...node, directlyVerified: node.directlyVerified || node.id === "shared",
    })) });
    expect(graph.edges.filter(edge => edge.target === "shared")).toHaveLength(3);
    expect(graph.nodes.find(node => node.id === "shared")!.position.x)
      .toBeGreaterThan(graph.nodes.find(node => node.id === "b")!.position.x);
  });

  it("keeps an unlinked verification visible with zero counts", async () => {
    const graph = await layoutTrace({ ...trace, requirements: [] });
    expect(graph.nodes.map(node => node.id)).toEqual(["v"]);
    expect(graph.edges).toEqual([]);
    expect(graph.directCount).toBe(0);
    expect(graph.totalCount).toBe(0);
  });
});
