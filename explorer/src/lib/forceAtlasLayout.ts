import Graph from "graphology";
import forceAtlas2 from "graphology-layout-forceatlas2";

export interface ForceAtlasInput {
  nodes: { id: string; x: number; y: number; size: number }[];
  edges: { id: string; source: string; target: string }[];
}
export type ForceAtlasPositions = { id: string; x: number; y: number }[];

/** The existing bounded profile, shared by the two renderer projections. */
export function forceAtlasProfile(nodeCount: number, edgeCount: number, averageNodeSize: number) {
  const clamp = (value: number, min: number, max: number) => Math.max(min, Math.min(max, value));
  const nodes = Math.max(1, nodeCount);
  const density = edgeCount / nodes;
  const pressure = clamp((averageNodeSize - 6) / 10, 0, 1.4);
  return {
    iterations: nodes > 650 ? 170 : nodes > 350 ? 180 : 200,
    gravity: clamp(1.45 + Math.log10(Math.max(10, nodes)) * 0.48 + Math.min(density, 8) * 0.04, 1.5, 3.2),
    scalingRatio: clamp(5 + Math.sqrt(nodes) * 0.14 + pressure * 1.5 - Math.min(density, 8) * 0.35, 5, 13),
    slowDown: nodes > 650 ? 2.3 : 2,
  };
}

/** Runs only inside the dedicated worker (or algorithm verification). */
export function runForceAtlasLayout(input: ForceAtlasInput): ForceAtlasPositions {
  const graph = new Graph({ type: "directed", multi: true, allowSelfLoops: true });
  let totalSize = 0;
  for (const node of input.nodes) {
    if (typeof node.id !== "string" || ![node.x, node.y, node.size].every(Number.isFinite) || node.size < 0) {
      throw new Error("Invalid ForceAtlas node");
    }
    graph.addNode(node.id, { x: node.x, y: node.y, size: node.size });
    totalSize += node.size;
  }
  for (const edge of input.edges) graph.addDirectedEdgeWithKey(edge.id, edge.source, edge.target);
  const profile = forceAtlasProfile(graph.order, graph.size, graph.order ? totalSize / graph.order : 0);
  forceAtlas2.assign(graph, {
    iterations: profile.iterations,
    settings: { ...forceAtlas2.inferSettings(graph), adjustSizes: true, barnesHutOptimize: true,
      gravity: profile.gravity, scalingRatio: profile.scalingRatio, slowDown: profile.slowDown },
  });
  return graph.mapNodes((id, { x, y }) => {
    if (!Number.isFinite(x) || !Number.isFinite(y)) throw new Error("Non-finite ForceAtlas result");
    return { id, x, y };
  });
}
