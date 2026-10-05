import type { ElkNode } from "elkjs/lib/elk-api";
import { describe, expect, it, vi } from "vitest";
import { buildTraceFlowGraph, TRACE_NODE_HEIGHT, TRACE_NODE_WIDTH, type TraceFlowDirection, type TraceFlowTopology } from "./traceFlowLayout";

const layout = vi.fn<(input: ElkNode) => Promise<ElkNode>>();

// Stop at the real engine boundary so preparation work is measured separately
// from ELK layout cost. The regular TraceFlow tests exercise the actual engine.
async function prepare(topology: TraceFlowTopology, direction: TraceFlowDirection) {
  const stop = new Error("captured layout input");
  layout.mockClear().mockRejectedValueOnce(stop);
  await expect(buildTraceFlowGraph(topology, direction, layout)).rejects.toBe(stop);
  expect(layout).toHaveBeenCalledTimes(1);
  return layout.mock.calls[0][0];
}

const portId = (id: string, side: "source" | "target") => JSON.stringify([id, side]);
const sorted = <T extends { id: string }>(rows: readonly T[]) => [...rows].sort((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : 0);
const nodes = (ids: string[]): TraceFlowTopology["nodes"] => ids.map(id => ({
  id, element: { id, name: id, file: "Model.md" }, type: "requirement", context: "requirement", parentCount: 0,
}));

// Deliberately retain the former scan as an independent parity oracle.
function scanInput(topology: TraceFlowTopology, direction: TraceFlowDirection): ElkNode {
  const edges = sorted(topology.edges);
  return {
    id: "trace-layout",
    layoutOptions: {
      "elk.algorithm": "layered", "elk.direction": direction, "elk.edgeRouting": "ORTHOGONAL",
      "elk.padding": "[top=24,left=24,bottom=24,right=24]", "elk.spacing.nodeNode": "48",
      "elk.layered.spacing.nodeNodeBetweenLayers": "88", "elk.spacing.edgeNode": "20",
      "elk.layered.spacing.edgeNodeBetweenLayers": "24", "elk.spacing.edgeEdge": "20",
      "elk.layered.spacing.edgeEdgeBetweenLayers": "16", "elk.randomSeed": "1",
    },
    children: sorted(topology.nodes).map(node => ({
      id: node.id, width: TRACE_NODE_WIDTH, height: TRACE_NODE_HEIGHT,
      layoutOptions: { "elk.portConstraints": "FIXED_SIDE", ...(node.root ? { "elk.layered.layering.layerConstraint": "FIRST" } : {}) },
      ports: edges.flatMap(edge => [
        ...(edge.source === node.id ? [{ id: portId(edge.id, "source"), width: 0, height: 0,
          layoutOptions: { "elk.port.side": direction === "DOWN" ? "SOUTH" : "EAST" } }] : []),
        ...(edge.target === node.id ? [{ id: portId(edge.id, "target"), width: 0, height: 0,
          layoutOptions: { "elk.port.side": direction === "DOWN" ? "NORTH" : "WEST" } }] : []),
      ]),
    })),
    edges: edges.map(edge => ({
      id: edge.id, sources: [portId(edge.id, "source")], targets: [portId(edge.id, "target")],
      labels: [{ text: edge.label, width: 100, height: 28, layoutOptions: { "elk.edgeLabels.placement": "CENTER" } }],
    })),
  };
}

const topology: TraceFlowTopology = {
  totalCount: 5,
  nodes: nodes(["root", "b", "a", "shared", 'isolated"[]']).map(node => ({ ...node, root: node.id === "root" })),
  edges: [
    { id: "split-b", source: "root", target: "b", label: "derive" },
    { id: "merge-b", source: "b", target: "shared", label: "derive" },
    { id: "split-a", source: "root", target: "a", label: "derive" },
    { id: "merge-a", source: "a", target: "shared", label: "derive" },
    { id: "parallel", source: "a", target: "shared", label: "trace" },
    { id: 'self"[source]', source: "shared", target: "shared", label: "related" },
  ],
};

describe("Flow layout input preparation", () => {
  it.each(["RIGHT", "DOWN"] as const)("preserves every ordered port and layout option (%s)", async direction => {
    const input = await prepare(topology, direction);
    expect(input).toEqual(scanInput(topology, direction));
    expect(input.children?.flatMap(node => node.ports ?? [])).toHaveLength(2 * topology.edges.length);
    expect(input.children?.find(node => node.id === 'isolated"[]')?.ports).toEqual([]);
    const self = input.edges?.find(edge => edge.id === 'self"[source]');
    expect(self?.sources).not.toEqual(self?.targets);
    expect(await prepare({ ...topology, nodes: [...topology.nodes].reverse(), edges: [...topology.edges].reverse() }, direction)).toEqual(input);
  });

  it.each(["RIGHT", "DOWN"] as const)("preserves empty graphs and unconnected cards (%s)", async direction => {
    for (const ids of [[], ["only"]]) {
      const data = { totalCount: ids.length, nodes: nodes(ids), edges: [] };
      expect(await prepare(data, direction)).toEqual(scanInput(data, direction));
    }
  });

  it("prepares fresh ports for changed topology and direction without mutating inputs", async () => {
    const before = JSON.stringify(topology);
    const horizontal = await prepare(topology, "RIGHT");
    const retainedInput = JSON.stringify(horizontal);
    const changed = { ...topology, edges: topology.edges.slice(1) };
    const vertical = await prepare(changed, "DOWN");
    expect(vertical).toEqual(scanInput(changed, "DOWN"));
    expect(JSON.stringify(horizontal)).toBe(retainedInput);
    expect(JSON.stringify(topology)).toBe(before);
    expect(await prepare(topology, "RIGHT")).toEqual(horizontal);
  });

  it.each([64, 128, 256])("reads each relation endpoint once for %i nodes", async size => {
    let endpointReads = 0;
    const data: TraceFlowTopology = {
      totalCount: size,
      nodes: nodes(Array.from({ length: size }, (_, index) => `node-${index}`)),
      edges: Array.from({ length: size - 1 }, (_, index) => ({
        id: `edge-${index}`, label: "derive",
        get source() { endpointReads++; return `node-${index}`; },
        get target() { endpointReads++; return `node-${index + 1}`; },
      })),
    };
    const input = await prepare(data, "RIGHT");
    expect(input.children?.flatMap(node => node.ports ?? [])).toHaveLength(2 * data.edges.length);
    expect(endpointReads).toBe(2 * data.edges.length);
  });
});
