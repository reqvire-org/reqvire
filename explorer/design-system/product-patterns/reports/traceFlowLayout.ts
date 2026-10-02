import type { ELK as ElkEngine, ElkNode } from "elkjs/lib/elk-api";

export interface TraceFlowElement {
  id: string;
  name: string;
  file: string;
  sourceHref?: string;
  href?: string;
}

export interface TraceFlowRequirement extends TraceFlowElement {
  directlyVerified: boolean;
  /** Upstream requirement ancestors in the trace projection. */
  parentIds: readonly string[];
}

export interface TraceFlowData {
  verification: TraceFlowElement & { type: string };
  requirements: readonly TraceFlowRequirement[];
}

export interface ElementFlowData {
  id: string;
  title: string;
  nodes: readonly { element: TraceFlowElement; type: string; context: string; root?: boolean }[];
  edges: readonly { id: string; source: string; target: string; label: string }[];
}

export interface TraceFlowTopology {
  nodes: { id: string; element: TraceFlowElement; type: string; context: string; parentCount: number; root?: boolean }[];
  edges: { id: string; source: string; target: string; label: string }[];
  totalCount: number;
  directCount?: number;
}

// Shared with the card's CSS custom properties; coordinates are canvas units.
export const TRACE_NODE_WIDTH = 280;
export const TRACE_NODE_HEIGHT = 164;
export interface TraceFlowPoint { x: number; y: number }

/** Rounded routed polyline; routing itself belongs to the graph layout. */
export function traceFlowEdgePath(points: readonly TraceFlowPoint[]): string {
  if (!points.length) return "";
  let path = `M ${points[0].x} ${points[0].y}`;
  for (let index = 1; index < points.length - 1; index++) {
    const previous = points[index - 1];
    const point = points[index];
    const next = points[index + 1];
    const beforeLength = Math.hypot(point.x - previous.x, point.y - previous.y);
    const afterLength = Math.hypot(next.x - point.x, next.y - point.y);
    const radius = Math.min(12, beforeLength / 2, afterLength / 2);
    const before = beforeLength ? radius / beforeLength : 0;
    const after = afterLength ? radius / afterLength : 0;
    path += ` L ${point.x + (previous.x - point.x) * before} ${point.y + (previous.y - point.y) * before}`;
    path += ` Q ${point.x} ${point.y} ${point.x + (next.x - point.x) * after} ${point.y + (next.y - point.y) * after}`;
  }
  const last = points[points.length - 1];
  return `${path} L ${last.x} ${last.y}`;
}

/** All directed paths into and out of a node, excluding sibling-only links. */
export function traceFlowNeighborhood(edges: readonly { id: string; source: string; target: string }[], id: string) {
  const nodeIds = new Set([id]);
  const edgeIds = new Set<string>();
  for (const direction of ["incoming", "outgoing"] as const) {
    const adjacency = new Map<string, typeof edges[number][]>();
    for (const edge of edges) {
      const from = direction === "incoming" ? edge.target : edge.source;
      const links = adjacency.get(from) ?? [];
      links.push(edge);
      adjacency.set(from, links);
    }
    const visited = new Set<string>();
    const queue = [id];
    for (let index = 0; index < queue.length; index++) {
      const current = queue[index];
      if (visited.has(current)) continue;
      visited.add(current);
      for (const edge of adjacency.get(current) ?? []) {
        const next = direction === "incoming" ? edge.source : edge.target;
        edgeIds.add(edge.id);
        nodeIds.add(next);
        queue.push(next);
      }
    }
  }
  return { nodeIds, edgeIds };
}

/** The caller supplies the evaluated trace; disclosure preserves shared paths. */
export function buildTraceFlowTopology(trace: TraceFlowData, collapsed: ReadonlySet<string> = new Set()) {
  const requirements = new Map<string, TraceFlowRequirement>();
  for (const requirement of trace.requirements) {
    const previous = requirements.get(requirement.id);
    requirements.set(requirement.id, previous ? {
      ...previous,
      directlyVerified: previous.directlyVerified || requirement.directlyVerified,
      parentIds: [...new Set([...previous.parentIds, ...requirement.parentIds])],
    } : requirement);
  }
  const direct = [...requirements.values()].filter(node => node.directlyVerified);
  const links: { id: string; source: string; target: string; label: "verifies" | "derivedFrom" }[] = [];
  const addLink = (source: string, target: string, label: "verifies" | "derivedFrom") => {
    links.push({ id: JSON.stringify([source, target]), source, target, label });
  };
  direct.forEach(node => addLink(trace.verification.id, node.id, "verifies"));
  const visible = new Set([trace.verification.id]);
  const queue = direct.map(node => node.id);
  for (let index = 0; index < queue.length; index++) {
    const id = queue[index];
    if (visible.has(id)) continue;
    visible.add(id);
    if (collapsed.has(id)) continue;
    for (const parentId of new Set(requirements.get(id)?.parentIds)) {
      if (!requirements.has(parentId)) continue;
      addLink(id, parentId, "derivedFrom");
      queue.push(parentId);
    }
  }

  const nodes = [...visible].map(id => ({
    id,
    element: id === trace.verification.id ? trace.verification : requirements.get(id)!,
    type: id === trace.verification.id ? trace.verification.type : "requirement",
    context: id === trace.verification.id ? "Verification" : requirements.get(id)?.directlyVerified ? "Directly verified" : "Requirement ancestor",
    parentCount: new Set(requirements.get(id)?.parentIds).size,
  }));
  return { nodes, edges: links, directCount: direct.length, totalCount: requirements.size };
}

export type TraceFlowDirection = "RIGHT" | "DOWN";
export type TraceFlowGraph = Awaited<ReturnType<typeof buildTraceFlowGraph>>;

let engine: Promise<ElkEngine> | undefined;
function getEngine() {
  // Load the layout engine only when a trace map is opened.
  return engine ??= import("elkjs/lib/elk.bundled.js")
    .then(({ default: ELK }) => new ELK({ algorithms: ["layered"] }))
    .catch(error => { engine = undefined; throw error; });
}

const portId = (edgeId: string, side: "source" | "target") => JSON.stringify([edgeId, side]);
const coordinate = (value: number | undefined) => {
  if (value === undefined || !Number.isFinite(value)) throw new Error("Incomplete trace layout");
  return value;
};

/** ELK owns card placement, ports, orthogonal routing, and relation-label space. */
export async function buildTraceFlowGraph(topology: TraceFlowTopology, direction: TraceFlowDirection = "RIGHT") {
  // Canonical insertion order keeps layout independent of model serialization.
  const edges = [...topology.edges].sort((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : 0);
  const input: ElkNode = {
    id: "trace-layout",
    layoutOptions: {
      "elk.algorithm": "layered",
      "elk.direction": direction,
      "elk.edgeRouting": "ORTHOGONAL",
      "elk.padding": "[top=24,left=24,bottom=24,right=24]",
      "elk.spacing.nodeNode": "48",
      "elk.layered.spacing.nodeNodeBetweenLayers": "88",
      "elk.spacing.edgeNode": "20",
      "elk.layered.spacing.edgeNodeBetweenLayers": "24",
      "elk.spacing.edgeEdge": "20",
      "elk.layered.spacing.edgeEdgeBetweenLayers": "16",
      "elk.randomSeed": "1",
    },
    children: [...topology.nodes].sort((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : 0).map(node => ({
      id: node.id, width: TRACE_NODE_WIDTH, height: TRACE_NODE_HEIGHT,
      layoutOptions: { "elk.portConstraints": "FIXED_SIDE", ...(node.root ? { "elk.layered.layering.layerConstraint": "FIRST" } : {}) },
      ports: edges.flatMap(edge => {
        return [
          ...(edge.source === node.id ? [{ id: portId(edge.id, "source"), width: 0, height: 0, layoutOptions: { "elk.port.side": direction === "DOWN" ? "SOUTH" : "EAST" } }] : []),
          ...(edge.target === node.id ? [{ id: portId(edge.id, "target"), width: 0, height: 0, layoutOptions: { "elk.port.side": direction === "DOWN" ? "NORTH" : "WEST" } }] : []),
        ];
      }),
    })),
    edges: edges.map(edge => ({
      id: edge.id,
      sources: [portId(edge.id, "source")], targets: [portId(edge.id, "target")],
      labels: [{ text: edge.label, width: 100, height: 28, layoutOptions: { "elk.edgeLabels.placement": "CENTER" } }],
    })),
  };
  const result = await (await getEngine()).layout(input);
  const positions = new Map(result.children?.map(node => [node.id, { x: coordinate(node.x), y: coordinate(node.y) }]));
  const routes = new Map(result.edges?.map(edge => [edge.id, edge]));
  return {
    ...topology,
    direction,
    // ELK's complete extent includes detouring connections and relation labels.
    bounds: { x: 0, y: 0, width: coordinate(result.width), height: coordinate(result.height) },
    nodes: topology.nodes.map(node => {
      const position = positions.get(node.id);
      if (!position) throw new Error("Missing trace node layout");
      return { ...node, position };
    }),
    edges: topology.edges.map(edge => {
      const route = routes.get(edge.id);
      const section = route?.sections?.[0];
      const label = route?.labels?.[0];
      if (!section || route?.sections?.length !== 1 || !label) throw new Error("Missing trace edge layout");
      const points = [section.startPoint, ...section.bendPoints ?? [], section.endPoint]
        .map(point => ({ x: coordinate(point.x), y: coordinate(point.y) }));
      const labelBounds = { x: coordinate(label.x), y: coordinate(label.y), width: coordinate(label.width), height: coordinate(label.height) };
      return { ...edge, points, path: traceFlowEdgePath(points), labelBounds,
        labelPosition: { x: labelBounds.x + labelBounds.width / 2, y: labelBounds.y + labelBounds.height / 2 },
      };
    }),
  };
}
