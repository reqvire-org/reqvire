import Graph from "graphology";
import forceAtlas2 from "graphology-layout-forceatlas2";
import { describe, expect, it } from "vitest";
import { forceAtlasProfile, runForceAtlasLayout, type ForceAtlasInput } from "./forceAtlasLayout";

describe("ForceAtlas algorithm compatibility", () => {
  it("retains the independently calculated bounded profile at both iteration thresholds", () => {
    expect(forceAtlasProfile(4, 5, 8)).toEqual({ iterations: 200, gravity: 1.98, scalingRatio: 5.1425, slowDown: 2 });
    expect([350, 351, 650, 651].map(n => forceAtlasProfile(n, 0, 6).iterations)).toEqual([200, 180, 180, 170]);
    expect(forceAtlasProfile(100000, 1000000, 100)).toEqual({ iterations: 170, gravity: 3.2, scalingRatio: 13, slowDown: 2.3 });
  });

  it("matches the original synchronous algorithm exactly for splits, joins, parallel relations and loops", () => {
    const input: ForceAtlasInput = {
      nodes: Array.from({ length: 4 }, (_, i) => ({ id: `n${i}`, x: Math.cos(i) * 3, y: Math.sin(i) * 3, size: 8 })),
      edges: [[0, 1], [0, 2], [1, 3], [2, 3], [3, 3]].map(([a, b], i) => ({ id: `e${i}`, source: `n${a}`, target: `n${b}` })),
    };
    input.edges[4] = { id: "e4", source: "n0", target: "n1" };
    // Keep a self-loop as well; six edges have density 1.5, giving fixed hand-calculated settings.
    input.edges.push({ id: "loop", source: "n3", target: "n3" });
    const baseline = new Graph({ type: "directed", multi: true, allowSelfLoops: true });
    input.nodes.forEach(({ id, ...attrs }) => baseline.addNode(id, attrs));
    input.edges.forEach(edge => baseline.addDirectedEdgeWithKey(edge.id, edge.source, edge.target));
    forceAtlas2.assign(baseline, { iterations: 200, settings: {
      ...forceAtlas2.inferSettings(baseline), adjustSizes: true, barnesHutOptimize: true,
      gravity: 1.99, scalingRatio: 5.055, slowDown: 2,
    } });
    const expected = baseline.mapNodes((id, { x, y }) => ({ id, x, y }));
    const saved = structuredClone(input);
    expect(runForceAtlasLayout(input)).toEqual(expected);
    expect(runForceAtlasLayout(input)).toEqual(expected);
    expect(input).toEqual(saved);
  });

  it("handles empty and isolated graphs and rejects malformed inputs", () => {
    expect(runForceAtlasLayout({ nodes: [], edges: [] })).toEqual([]);
    expect(runForceAtlasLayout({ nodes: [{ id: "alone", x: 0, y: 0, size: 6 }], edges: [] })).toHaveLength(1);
    expect(() => runForceAtlasLayout({ nodes: [{ id: "bad", x: NaN, y: 0, size: 8 }], edges: [] })).toThrow();
    expect(() => runForceAtlasLayout({ nodes: [], edges: [{ id: "bad", source: "missing", target: "missing" }] })).toThrow();
  });
});
