import { useEffect, useMemo, useRef, useState } from "react";
import Graph from "graphology";
import Sigma from "sigma";
import { animateNodes } from "sigma/utils";
import forceAtlas2 from "graphology-layout-forceatlas2";
import noverlap from "graphology-layout-noverlap";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import { useExplorerUiState } from "../state/ExplorerUiState";
import type { KnowledgeGraphNode, KnowledgeGraphProjection } from "../store/types";
import { ViewFrame } from "./ViewFrame";
import {
  cssVar,
  GraphCanvasFrame,
  GraphCanvasNotice,
  GraphCanvasSurface,
  GraphRoute,
  roleColorValue,
  Spinner,
  useLatestRef,
} from "@ds";

type GraphEdge = NonNullable<KnowledgeGraphProjection["edges"]>[number] & {
  relCategory?: RelationCategory;
};

type GraphData = {
  nodes: KnowledgeGraphNode[];
  nodeById: Map<string, KnowledgeGraphNode>;
  edges: GraphEdge[];
  degreeByNode: Map<string, number>;
  edgeAdjacency: Map<string, GraphEdge[]>;
};

type GraphNodeAttributes = {
  x?: unknown;
  y?: unknown;
  baseX?: unknown;
  baseY?: unknown;
  size?: unknown;
  label?: unknown;
  fullLabel?: unknown;
  hidden?: unknown;
};

type ForceAtlasProfile = {
  iterations: number;
  gravity: number;
  scalingRatio: number;
  slowDown: number;
};

type RelationCategory =
  | "derive"
  | "specify"
  | "define"
  | "verify"
  | "satisfy"
  | "bind"
  | "concept-reference"
  | "trace";

type OverlayKey = "cross" | "verification" | "trace";

function nodeKind(node: KnowledgeGraphNode): string {
  return node.element_type || node.node_type || node.type || "other";
}

function roleColor(kind: string) {
  return {
    fill: roleColorValue(kind),
    border: roleColorValue(kind, "ink"),
  };
}

function relationCategory(edge: { label?: unknown; kind?: unknown }): RelationCategory {
  const label = String(edge.label || "").toLowerCase();
  const kind = String(edge.kind || "").toLowerCase();
  if (kind === "contract_bindings" || label === "binds contract") return "bind";
  if (kind === "concept-reference" || label === "conceptref") return "concept-reference";
  if (label.includes("derive")) return "derive";
  if (label.includes("specif")) return "specify";
  if (label.includes("defin")) return "define";
  if (label.includes("verif")) return "verify";
  if (label.includes("satisf")) return "satisfy";
  if (label.includes("trace")) return "trace";
  return "trace";
}

function displayEdgeLabel(edge: Pick<GraphEdge, "label" | "kind">): string {
  const label = String(edge.label || "");
  const kind = String(edge.kind || "").toLowerCase();
  if (kind === "contract_bindings" || label.toLowerCase() === "binds contract") {
    return "contract binding";
  }
  return label;
}

function overlayVisible(category: RelationCategory, activeOverlays: Set<OverlayKey>) {
  if (category === "bind" || category === "concept-reference") {
    return activeOverlays.has("cross");
  }
  if (category === "verify" || category === "satisfy") {
    return activeOverlays.has("verification");
  }
  if (category === "trace") return activeOverlays.has("trace");
  return true;
}

function truncate(value: string | undefined, max: number) {
  const text = value ?? "";
  return text.length > max ? `${text.slice(0, Math.max(1, max - 1))}...` : text;
}

function nodeLabelLimit(node: KnowledgeGraphNode) {
  return ["capability", "requirement", "ontology"].includes(nodeKind(node)) ? 26 : 34;
}

function nodeSize(node: KnowledgeGraphNode, degreeByNode: Map<string, number>) {
  const degree = degreeByNode.get(node.id) ?? 0;
  return Math.min(16, 4 + Math.sqrt(degree + 1) * 1.6);
}

function clamp(value: number, minimum: number, maximum: number) {
  return Math.max(minimum, Math.min(maximum, value));
}

function forceAtlasProfile(nodeCount: number, edgeCount: number, averageNodeSize: number): ForceAtlasProfile {
  const nodes = Math.max(1, nodeCount);
  const density = edgeCount / nodes;
  const sizePressure = clamp((averageNodeSize - 6) / 10, 0, 1.4);
  return {
    iterations: nodes > 650 ? 170 : nodes > 350 ? 180 : 200,
    gravity: clamp(1.45 + Math.log10(Math.max(10, nodes)) * 0.48 + Math.min(density, 8) * 0.04, 1.5, 3.2),
    scalingRatio: clamp(5 + Math.sqrt(nodes) * 0.14 + sizePressure * 1.5 - Math.min(density, 8) * 0.35, 5, 13),
    slowDown: nodes > 650 ? 2.3 : 2,
  };
}

function dimNodeColor(color: string, alpha: number) {
  const foreground = parseHexColor(color);
  const background = parseHexColor(cssVar("--bg-canvas"));
  if (!foreground || !background) return color;
  const r = Math.round(foreground.r * alpha + background.r * (1 - alpha));
  const g = Math.round(foreground.g * alpha + background.g * (1 - alpha));
  const b = Math.round(foreground.b * alpha + background.b * (1 - alpha));
  return `#${[r, g, b].map((component) => component.toString(16).padStart(2, "0")).join("")}`;
}

function parseHexColor(color: string) {
  if (!color.startsWith("#")) return null;
  const hex = color.slice(1);
  const value =
    hex.length === 3
      ? hex
          .split("")
          .map((part) => part + part)
          .join("")
      : hex.padEnd(6, "0").slice(0, 6);
  const r = Number.parseInt(value.slice(0, 2), 16);
  const g = Number.parseInt(value.slice(2, 4), 16);
  const b = Number.parseInt(value.slice(4, 6), 16);
  return [r, g, b].some((component) => Number.isNaN(component)) ? null : { r, g, b };
}

function sigmaLabelSettings() {
  return {
    labelColor: { color: cssVar("--slate-950") },
    edgeLabelColor: { color: cssVar("--text-muted") },
    labelWeight: "600",
    edgeLabelWeight: "600",
  } as const;
}

function edgeColor(edge: { label?: unknown; kind?: unknown; relCategory?: unknown }) {
  const category = typeof edge.relCategory === "string"
    ? edge.relCategory as RelationCategory
    : relationCategory(edge);
  if (category === "concept-reference") return cssVar("--concept-reference");
  if (category === "bind") return cssVar("--edge-bind");
  if (category === "derive") return cssVar("--edge-derive");
  if (category === "satisfy" || category === "verify") return cssVar("--edge-satisfy");
  if (category === "trace") return cssVar("--edge-trace");
  return cssVar("--edge-default");
}

function isOpenableGraphNode(node: KnowledgeGraphNode | null | undefined): node is KnowledgeGraphNode {
  if (!node?.identifier) return false;
  const kind = nodeKind(node);
  return kind !== "resource";
}

function buildGraphData(projection: KnowledgeGraphProjection | undefined) {
  const rawNodes = projection?.nodes ?? [];
  const nodes = Array.from(
    new Map(rawNodes.map((node) => [node.id, { ...node, node_type: nodeKind(node) }])).values(),
  );
  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  const edges = (projection?.edges ?? [])
    .filter((edge) => nodeById.has(edge.source) && nodeById.has(edge.target))
    .map((edge) => ({ ...edge, relCategory: relationCategory(edge) }));
  const degreeByNode = new Map<string, number>();
  const edgeAdjacency = new Map<string, GraphEdge[]>();
  const addEdgeForNode = (nodeId: string, edge: GraphEdge) => {
    degreeByNode.set(nodeId, (degreeByNode.get(nodeId) ?? 0) + 1);
    edgeAdjacency.set(nodeId, [...(edgeAdjacency.get(nodeId) ?? []), edge]);
  };
  edges.forEach((edge) => {
    addEdgeForNode(edge.source, edge);
    if (edge.target !== edge.source) addEdgeForNode(edge.target, edge);
  });
  return { nodes, nodeById, edges, degreeByNode, edgeAdjacency } satisfies GraphData;
}

function assignInitialPositions(nodes: KnowledgeGraphNode[]) {
  const buckets = new Map<string, KnowledgeGraphNode[]>();
  nodes.forEach((node) => {
    const key = nodeKind(node);
    buckets.set(key, [...(buckets.get(key) ?? []), node]);
  });
  const centers: Record<string, [number, number]> = {
    capability: [-8, -5],
    requirement: [-1, -2],
    contract: [3, 2],
    verification: [8, 5],
    ontology: [-5, -8],
    resource: [10, 8],
    concept: [11, 9],
    other: [0, 0],
  };
  buckets.forEach((bucket, kind) => {
    const [cx, cy] = centers[kind] ?? centers.other;
    const radius = Math.max(2.5, Math.sqrt(bucket.length) * 0.9);
    bucket.forEach((node, index) => {
      const angle = (index / Math.max(bucket.length, 1)) * Math.PI * 2;
      const ring = radius * (0.45 + (index % 11) / 11);
      (node as KnowledgeGraphNode & { x: number; y: number }).x = cx + Math.cos(angle) * ring;
      (node as KnowledgeGraphNode & { x: number; y: number }).y = cy + Math.sin(angle) * ring;
    });
  });
}

function numericAttribute(value: unknown, fallback: number) {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function stableNodePosition(attributes: GraphNodeAttributes) {
  return {
    x: numericAttribute(attributes.baseX, numericAttribute(attributes.x, 0)),
    y: numericAttribute(attributes.baseY, numericAttribute(attributes.y, 0)),
  };
}

function stableNodeTarget(graph: Graph, nodeId: string) {
  if (!graph.hasNode(nodeId)) return null;
  return stableNodePosition(graph.getNodeAttributes(nodeId) as GraphNodeAttributes);
}

function animateGraphNodesToStablePositions(
  graph: Graph,
  nodeIds: Iterable<string>,
  onComplete?: () => void,
) {
  const targets: Record<string, { x: number; y: number }> = {};
  for (const id of nodeIds) {
    const target = stableNodeTarget(graph, id);
    if (target) targets[id] = target;
  }
  if (Object.keys(targets).length === 0) return null;
  return animateNodes(graph, targets, {
    duration: 250,
    easing: "quadraticOut",
  }, onComplete);
}

function graphNodeCollisionSize(attributes: GraphNodeAttributes) {
  const size = numericAttribute(attributes.size, 6);
  const label = String(attributes.fullLabel || attributes.label || "");
  return Math.max(3, Math.min(18, size * 0.45 + label.length * 0.14));
}

function focusedNeighborhoodIds(
  selectedId: string,
  graph: Graph,
  edgeAdjacency: Map<string, GraphEdge[]>,
  visibleEdge: (edge: GraphEdge) => boolean,
) {
  const ids = new Set<string>([selectedId]);
  for (const edge of edgeAdjacency.get(selectedId) ?? []) {
    if (!visibleEdge(edge)) continue;
    if (graph.hasNode(edge.source)) ids.add(edge.source);
    if (graph.hasNode(edge.target)) ids.add(edge.target);
  }
  return [...ids].filter((id) => {
    if (!graph.hasNode(id)) return false;
    const attributes = graph.getNodeAttributes(id) as GraphNodeAttributes;
    return attributes.hidden !== true;
  });
}

function relayoutFocusedNeighborhood(
  graph: Graph,
  selectedId: string,
  edgeAdjacency: Map<string, GraphEdge[]>,
  visibleEdge: (edge: GraphEdge) => boolean,
  restoreNodeIds: Iterable<string>,
  onComplete?: () => void,
) {
  const ids = focusedNeighborhoodIds(selectedId, graph, edgeAdjacency, visibleEdge);
  const focusedIds = new Set(ids);
  const targets: Record<string, { x: number; y: number }> = {};
  for (const id of restoreNodeIds) {
    if (focusedIds.has(id)) continue;
    const target = stableNodeTarget(graph, id);
    if (target) targets[id] = target;
  }
  const animateTargets = () => {
    if (Object.keys(targets).length === 0) return null;
    return animateNodes(graph, targets, {
      duration: 250,
      easing: "quadraticOut",
    }, onComplete);
  };
  if (ids.length < 2 || ids.length > 90) return animateTargets();

  const selectedAttributes = graph.getNodeAttributes(selectedId) as GraphNodeAttributes;
  const center = stableNodePosition(selectedAttributes);
  const focusGraph = new Graph({ type: "undirected", multi: false, allowSelfLoops: false });
  const neighbors = ids.filter((id) => id !== selectedId).sort();
  const densityStep = Math.min(3, Math.floor(neighbors.length / 12));
  const radius = Math.max(28, Math.sqrt(neighbors.length) * 12);

  focusGraph.addNode(selectedId, {
    x: center.x,
    y: center.y,
    size: graphNodeCollisionSize(selectedAttributes),
  });
  neighbors.forEach((id, index) => {
    const attributes = graph.getNodeAttributes(id) as GraphNodeAttributes;
    const angle = (index / Math.max(neighbors.length, 1)) * Math.PI * 2;
    const ring = radius * (1 + Math.floor(index / 10) * 0.75);
    focusGraph.addNode(id, {
      x: center.x + Math.cos(angle) * ring * 1.75,
      y: center.y + Math.sin(angle) * ring * 1.1,
      size: graphNodeCollisionSize(attributes),
    });
  });

  for (const edge of edgeAdjacency.get(selectedId) ?? []) {
    if (!visibleEdge(edge) || !focusGraph.hasNode(edge.source) || !focusGraph.hasNode(edge.target)) continue;
    if (edge.source === edge.target || focusGraph.hasEdge(edge.source, edge.target)) continue;
    focusGraph.addUndirectedEdge(edge.source, edge.target);
  }

  try {
    noverlap.assign(focusGraph, {
      maxIterations: 120,
      settings: {
        gridSize: Math.max(1, Math.ceil(Math.sqrt(ids.length))),
        margin: 2.4 + densityStep * 0.8,
        expansion: 1.45 + densityStep * 0.12,
        ratio: 1,
        speed: 2,
      },
    });
  } catch {
    return null;
  }

  const shiftedSelected = focusGraph.getNodeAttributes(selectedId) as { x: number; y: number };
  const shift = {
    x: center.x - shiftedSelected.x,
    y: center.y - shiftedSelected.y,
  };
  focusGraph.forEachNode((id, attributes) => {
    const position = attributes as { x: number; y: number };
    const x = position.x + shift.x;
    const y = position.y + shift.y;
    const edgeStretch = id === selectedId ? 1 : 1.35;
    targets[id] = {
      x: center.x + (x - center.x) * edgeStretch,
      y: center.y + (y - center.y) * edgeStretch,
    };
  });

  return animateTargets();
}

function centerCameraOnNode(renderer: Sigma, graph: Graph, nodeId: string) {
  if (!graph.hasNode(nodeId)) return;
  const display = renderer.getNodeDisplayData(nodeId);
  if (!display) return;
  const camera = renderer.getCamera();
  camera.animate({
    x: display.x,
    y: display.y,
    ratio: camera.getState().ratio,
  }, {
    duration: 250,
    easing: "quadraticOut",
  });
}

export function KnowledgeGraphView({
  frameTestId = "model-graph",
  embedded = false,
  onOpenElement,
}: {
  frameTestId?: string;
  embedded?: boolean;
  onOpenElement?: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store } = useStore();
  const {
    modelTypes: activeTypes,
    modelOverlays: activeOverlays,
    knowledgeGraphSelectionId: selectedId,
    setKnowledgeGraphSelectionId: setSelectedId,
  } = useExplorerUiState();
  const { nodes, nodeById, edges, degreeByNode, edgeAdjacency } = useMemo(
    () => buildGraphData(store.knowledge_graph),
    [store.knowledge_graph],
  );
  const containerRef = useRef<HTMLDivElement | null>(null);
  const graphRef = useRef<Graph | null>(null);
  const rendererRef = useRef<Sigma | null>(null);
  const selectedRef = useRef<string | null>(null);
  const hoveredRef = useRef<string | null>(null);
  const layoutAnimationRef = useRef<(() => void) | null>(null);
  const focusedLayoutNodeIdsRef = useRef<Set<string>>(new Set());
  const onOpenElementRef = useLatestRef(onOpenElement);
  const activeTypesRef = useRef(activeTypes);
  const activeOverlaysRef = useRef(activeOverlays);
  const graphFilterRevisionRef = useRef(0);
  const [notice, setNotice] = useState<string | null>(() => (
    nodes.length > 0 ? "Loading graph..." : null
  ));

  const visibleNode = (node: KnowledgeGraphNode) =>
    activeTypesRef.current.has(nodeKind(node));
  const visibleEdge = (edge: GraphEdge) => {
    const source = nodeById.get(edge.source);
    const target = nodeById.get(edge.target);
    return (
      Boolean(source && target) &&
      Boolean(source && visibleNode(source)) &&
      Boolean(target && visibleNode(target)) &&
      overlayVisible(edge.relCategory ?? relationCategory(edge), activeOverlaysRef.current)
    );
  };
  const cancelFocusedLayoutAnimation = () => {
    layoutAnimationRef.current?.();
    layoutAnimationRef.current = null;
  };
  const runFocusedLayout = ({ centerSelection = false }: { centerSelection?: boolean } = {}) => {
    const graph = graphRef.current;
    const renderer = rendererRef.current;
    const selected = selectedRef.current;
    cancelFocusedLayoutAnimation();
    if (!graph || !renderer || !selected || !graph.hasNode(selected)) {
      layoutAnimationRef.current = graph && renderer
        ? animateGraphNodesToStablePositions(
          graph,
          focusedLayoutNodeIdsRef.current,
          () => renderer.refresh(),
        )
        : null;
      focusedLayoutNodeIdsRef.current = new Set();
      renderer?.refresh();
      return;
    }
    const focusedNodeIds = new Set(focusedNeighborhoodIds(selected, graph, edgeAdjacency, visibleEdge));
    const onFocusedLayoutComplete = () => {
      renderer.refresh();
      if (centerSelection) centerCameraOnNode(renderer, graph, selected);
    };
    layoutAnimationRef.current = relayoutFocusedNeighborhood(
      graph,
      selected,
      edgeAdjacency,
      visibleEdge,
      focusedLayoutNodeIdsRef.current,
      onFocusedLayoutComplete,
    );
    focusedLayoutNodeIdsRef.current = focusedNodeIds.size >= 2 && focusedNodeIds.size <= 90
      ? focusedNodeIds
      : new Set();
    if (centerSelection) centerCameraOnNode(renderer, graph, selected);
    renderer.refresh();
  };
  useEffect(() => {
    activeTypesRef.current = activeTypes;
    activeOverlaysRef.current = activeOverlays;
    graphFilterRevisionRef.current += 1;
  }, [activeTypes, activeOverlays]);

  useEffect(() => {
    selectedRef.current = selectedId;
    runFocusedLayout({ centerSelection: Boolean(selectedId) });
  }, [selectedId]);

  useEffect(() => {
    const graph = graphRef.current;
    const renderer = rendererRef.current;
    if (!graph) return;
    nodes.forEach((node) => {
      if (graph.hasNode(node.id)) graph.setNodeAttribute(node.id, "hidden", !visibleNode(node));
    });
    edges.forEach((edge, index) => {
      const key = `e${index}`;
      if (graph.hasEdge(key)) graph.setEdgeAttribute(key, "hidden", !visibleEdge(edge));
    });
    runFocusedLayout();
    renderer?.refresh();
  }, [activeTypes, activeOverlays, edges, nodes]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container || nodes.length === 0) {
      setNotice(nodes.length === 0 ? "No project graph nodes were exported." : null);
      return undefined;
    }

    let graph: Graph | null = null;
    let renderer: Sigma | null = null;
    let suppressNextStageClear = false;
    let suppressNextNodeClick = false;
    let suppressStageClearTimer: number | null = null;
    let draggedNodeId: string | null = null;
    let isDraggingNode = false;
    let dragMovedNode = false;
    const neighborhoodCache = new Map<string, Set<string>>();
    const setGraphCursor = (cursor: "" | "pointer" | "grabbing") => {
      container.style.cursor = cursor;
      container.querySelectorAll("canvas").forEach((canvas) => {
        canvas.style.cursor = cursor;
      });
    };
    const armStageClearSuppression = () => {
      suppressNextStageClear = true;
      if (suppressStageClearTimer !== null) {
        window.clearTimeout(suppressStageClearTimer);
      }
      suppressStageClearTimer = window.setTimeout(() => {
        suppressNextStageClear = false;
        suppressStageClearTimer = null;
      }, 0);
    };
    setNotice("Loading graph...");
    let buildTimer: number | null = null;
    const frameId = window.requestAnimationFrame(() => {
    buildTimer = window.setTimeout(() => {
    try {
      const positionedNodes = nodes.map((node) => ({ ...node }));
      assignInitialPositions(positionedNodes);
      graph = new Graph({ type: "directed", multi: true, allowSelfLoops: true });
      positionedNodes.forEach((node) => {
        const kind = nodeKind(node);
        const positioned = node as KnowledgeGraphNode & { x: number; y: number };
        graph?.addNode(node.id, {
          ...node,
          type: "circle",
          reqvireType: kind,
          label: truncate(node.label, nodeLabelLimit(node)),
          fullLabel: node.label,
          x: positioned.x,
          y: positioned.y,
          size: nodeSize(node, degreeByNode),
          color: roleColor(kind).fill,
          hidden: !visibleNode(node),
        });
      });
      edges.forEach((edge, index) => {
        graph?.addDirectedEdgeWithKey(`e${index}`, edge.source, edge.target, {
          ...edge,
          type: "arrow",
          label: displayEdgeLabel(edge),
          size: edge.kind === "contract_bindings" || edge.kind === "concept-reference" ? 0.8 : 1.1,
          color: edgeColor(edge),
          hidden: !visibleEdge(edge),
        });
      });
      try {
        const layoutGraph = new Graph({ type: "directed", multi: true, allowSelfLoops: true });
        let totalNodeSize = 0;
        positionedNodes.forEach((node) => {
          if (!visibleNode(node)) return;
          const positioned = node as KnowledgeGraphNode & { x: number; y: number };
          const size = nodeSize(node, degreeByNode);
          totalNodeSize += size;
          layoutGraph.addNode(node.id, {
            x: positioned.x,
            y: positioned.y,
            size,
          });
        });
        edges.forEach((edge, index) => {
          if (!visibleEdge(edge) || !layoutGraph.hasNode(edge.source) || !layoutGraph.hasNode(edge.target)) {
            return;
          }
          layoutGraph.addDirectedEdgeWithKey(`e${index}`, edge.source, edge.target);
        });
        const profile = forceAtlasProfile(
          layoutGraph.order,
          layoutGraph.size,
          layoutGraph.order ? totalNodeSize / layoutGraph.order : 0,
        );
        const settings = forceAtlas2.inferSettings(layoutGraph);
        forceAtlas2.assign(layoutGraph, {
          iterations: profile.iterations,
          settings: {
            ...settings,
            adjustSizes: true,
            barnesHutOptimize: true,
            gravity: profile.gravity,
            scalingRatio: profile.scalingRatio,
            slowDown: profile.slowDown,
          },
        });
        layoutGraph.forEachNode((nodeId, attributes) => {
          graph?.mergeNodeAttributes(nodeId, {
            x: numericAttribute(attributes.x, 0),
            y: numericAttribute(attributes.y, 0),
          });
        });
      } catch (error) {
        console.warn("[Reqvire KG] ForceAtlas2 layout failed", error);
      }
      graph.forEachNode((id, attributes) => {
        graph?.mergeNodeAttributes(id, {
          baseX: numericAttribute(attributes.x, 0),
          baseY: numericAttribute(attributes.y, 0),
        });
      });
      const computeFocusNeighborhoodIds = (focusIds: readonly string[]) => {
        const cacheKey = `${graphFilterRevisionRef.current}|${[...focusIds].sort().join("\u001f")}`;
        const cached = neighborhoodCache.get(cacheKey);
        if (cached) return cached;
        const neighborhood = new Set<string>();
        focusIds.forEach((focusId) => {
          neighborhood.add(focusId);
          (edgeAdjacency.get(focusId) ?? []).forEach((edge) => {
            if (!visibleEdge(edge)) return;
            if (edge.source === focusId) neighborhood.add(edge.target);
            if (edge.target === focusId) neighborhood.add(edge.source);
          });
        });
        neighborhoodCache.set(cacheKey, neighborhood);
        return neighborhood;
      };
      const edgeInFocusNeighborhood = (
        attributes: { source?: unknown; target?: unknown },
        focusIds: readonly string[],
        neighborhoodIds: ReadonlySet<string>,
      ) => {
        const source = String(attributes.source ?? "");
        const target = String(attributes.target ?? "");
        return (
          neighborhoodIds.has(source) &&
          neighborhoodIds.has(target) &&
          focusIds.some((focusId) => source === focusId || target === focusId)
        );
      };

      renderer = new Sigma(graph, container, {
        allowInvalidContainer: true,
        defaultEdgeType: "arrow",
        renderEdgeLabels: true,
        zIndex: true,
        ...sigmaLabelSettings(),
        labelDensity: 0.12,
        labelGridCellSize: 80,
        labelRenderedSizeThreshold: 9,
        nodeReducer: (node, attributes) => {
          const result = { ...attributes };
          if (attributes.hidden) {
            result.label = "";
            result.highlighted = false;
            result.forceLabel = false;
            return result;
          }
          const selectedId = selectedRef.current;
          const hoveredId = hoveredRef.current;
          const dragged = draggedNodeId === node;
          const selectionNeighborhoodIds = selectedId
            ? computeFocusNeighborhoodIds([selectedId])
            : new Set<string>();
          const hoverRefinesSelection = Boolean(
            selectedId && hoveredId && selectionNeighborhoodIds.has(hoveredId),
          );
          const hoverNeighborhoodIds = hoveredId
            ? computeFocusNeighborhoodIds([hoveredId])
            : new Set<string>();

          if (selectedId) {
            const inSelectionTree = selectionNeighborhoodIds.has(node);
            const inHoverTree = hoverRefinesSelection && hoverNeighborhoodIds.has(node);
            const visibleByFocus = inSelectionTree || inHoverTree || dragged;
            if (!visibleByFocus) {
              result.hidden = true;
              result.label = "";
              result.highlighted = false;
              result.forceLabel = false;
              result.zIndex = 0;
              return result;
            }
            result.hidden = false;
            if (hoverRefinesSelection && inSelectionTree && !inHoverTree && !dragged) {
              result.color = dimNodeColor(String(attributes.color ?? cssVar("--text-faint")), 0.2);
              result.label = "";
              result.highlighted = false;
              result.forceLabel = false;
              result.zIndex = 0;
              return result;
            }
            result.label = attributes.fullLabel ?? attributes.label ?? "";
            result.highlighted = true;
            result.forceLabel = true;
            result.zIndex = dragged || node === selectedId || node === hoveredId ? 20 : 10;
            return result;
          }

          const dragIds = [draggedNodeId].filter((id): id is string => Boolean(id));
          const focusIds = [
            ...(hoveredId ? [hoveredId] : []),
            ...dragIds,
          ];
          const hasFocus = focusIds.length > 0;
          const focusNeighborhoodIds = computeFocusNeighborhoodIds(focusIds);
          const inFocusNeighborhood = focusNeighborhoodIds.has(node);
          if (!hasFocus) {
            result.label = "";
            result.highlighted = false;
            result.forceLabel = false;
          } else if (inFocusNeighborhood || dragged) {
            result.label = attributes.fullLabel ?? attributes.label ?? "";
            result.highlighted = true;
            result.forceLabel = true;
            result.zIndex = dragged ? 20 : 10;
          } else {
            result.color = dimNodeColor(String(attributes.color ?? cssVar("--text-faint")), 0.2);
            result.label = "";
            result.highlighted = false;
            result.forceLabel = false;
            result.zIndex = 0;
          }
          return result;
        },
        edgeReducer: (_edge, attributes) => {
          const result = { ...attributes };
          const selectedId = selectedRef.current;
          const hoveredId = hoveredRef.current;
          if (attributes.hidden) {
            result.hidden = true;
            return result;
          }
          if (selectedId) {
            const selectionNeighborhoodIds = computeFocusNeighborhoodIds([selectedId]);
            const hoverRefinesSelection = Boolean(
              hoveredId && selectionNeighborhoodIds.has(hoveredId),
            );
            const focusIds = hoverRefinesSelection && hoveredId ? [hoveredId] : [selectedId];
            const focusNeighborhoodIds = hoverRefinesSelection && hoveredId
              ? computeFocusNeighborhoodIds([hoveredId])
              : selectionNeighborhoodIds;
            if (!edgeInFocusNeighborhood(attributes, focusIds, focusNeighborhoodIds)) {
              result.hidden = true;
              result.label = "";
              return result;
            }
            result.hidden = false;
            result.color = edgeColor(attributes);
            result.size = Math.max(1.1, Number(attributes.size ?? 1) * 1.15);
            result.forceLabel = true;
            return result;
          }
          const focusIds = [hoveredId, draggedNodeId].filter((id): id is string => Boolean(id));
          if (
            focusIds.length === 0 ||
            !edgeInFocusNeighborhood(attributes, focusIds, computeFocusNeighborhoodIds(focusIds))
          ) {
            result.hidden = true;
            result.label = "";
          } else {
            result.hidden = false;
            result.color = edgeColor(attributes);
            result.size = Math.max(1.1, Number(attributes.size ?? 1) * 1.15);
            result.forceLabel = true;
          }
          return result;
        },
      });
      graphRef.current = graph;
      rendererRef.current = renderer;
      setNotice(null);

      renderer.on("clickNode", (event) => {
        if (suppressNextNodeClick) {
          suppressNextNodeClick = false;
          armStageClearSuppression();
          return;
        }
        armStageClearSuppression();
        setSelectedId(event.node);
      });
      renderer.on("doubleClickNode", (event) => {
        const node = nodeById.get(event.node);
        if (isOpenableGraphNode(node)) onOpenElementRef.current?.(node.identifier);
      });
      renderer.on("clickStage", () => {
        setGraphCursor("");
        if (suppressNextStageClear) {
          suppressNextStageClear = false;
          if (suppressStageClearTimer !== null) {
            window.clearTimeout(suppressStageClearTimer);
            suppressStageClearTimer = null;
          }
          return;
        }
        setSelectedId(null);
      });
      renderer.on("enterNode", (event) => {
        hoveredRef.current = event.node;
        neighborhoodCache.clear();
        setGraphCursor("pointer");
        renderer?.refresh();
      });
      renderer.on("leaveNode", (event) => {
        if (hoveredRef.current === event.node) hoveredRef.current = null;
        neighborhoodCache.clear();
        setGraphCursor("");
        renderer?.refresh();
      });
      renderer.on("downNode", (event) => {
        if (!graph?.hasNode(event.node) || graph.getNodeAttribute(event.node, "hidden")) {
          return;
        }
        setGraphCursor("grabbing");
        isDraggingNode = true;
        draggedNodeId = event.node;
        dragMovedNode = false;
        armStageClearSuppression();
        if (!renderer?.getCustomBBox()) {
          renderer?.setCustomBBox(renderer.getBBox());
        }
        renderer?.refresh();
      });
      renderer.on("moveBody", ({ event }) => {
        if (!isDraggingNode || !draggedNodeId || !graph?.hasNode(draggedNodeId)) {
          return;
        }
        const position = renderer?.viewportToGraph(event);
        if (!position) return;
        graph.mergeNodeAttributes(draggedNodeId, {
          x: position.x,
          y: position.y,
        });
        dragMovedNode = true;
        renderer?.refresh();
        const sigmaEvent = event as typeof event & {
          preventSigmaDefault?: () => void;
          original?: {
            preventDefault?: () => void;
            stopPropagation?: () => void;
          };
        };
        sigmaEvent.preventSigmaDefault?.();
        sigmaEvent.original?.preventDefault?.();
        sigmaEvent.original?.stopPropagation?.();
      });
      const handleNodeDragEnd = () => {
        if (!isDraggingNode && !draggedNodeId) return;
        if (dragMovedNode) {
          suppressNextNodeClick = true;
          armStageClearSuppression();
        }
        isDraggingNode = false;
        draggedNodeId = null;
        dragMovedNode = false;
        setGraphCursor(hoveredRef.current ? "pointer" : "");
        renderer?.refresh();
      };
      renderer.on("upNode", handleNodeDragEnd);
      renderer.on("upStage", handleNodeDragEnd);
      renderer.getCamera().animatedReset({ duration: 250 });
    } catch (error) {
      console.error("[Reqvire KG] Sigma/Graphology renderer failed", error);
      setNotice("Graph renderer failed. Check the browser console for details.");
    }
    }, 0);
    });

    return () => {
      window.cancelAnimationFrame(frameId);
      if (buildTimer !== null) {
        window.clearTimeout(buildTimer);
        buildTimer = null;
      }
      // Lose all WebGL contexts before removal so the GPU compositor immediately
      // drops the cached texture — prevents stale-frame bleed onto the next view.
      containerRef.current?.querySelectorAll("canvas").forEach((canvas) => {
        const gl =
          (canvas.getContext("webgl") as WebGLRenderingContext | null) ??
          (canvas.getContext("webgl2") as WebGL2RenderingContext | null);
        gl?.getExtension("WEBGL_lose_context")?.loseContext();
      });
      if (suppressStageClearTimer !== null) {
        window.clearTimeout(suppressStageClearTimer);
        suppressStageClearTimer = null;
      }
      cancelFocusedLayoutAnimation();
      setGraphCursor("");
      renderer?.kill();
      graphRef.current = null;
      rendererRef.current = null;
      graph = null;
      renderer = null;
    };
  }, [degreeByNode, edgeAdjacency, edges, nodeById, nodes, setSelectedId]);

  const graph = (
    <GraphRoute embedded={embedded}>
      <GraphCanvasFrame>
        <GraphCanvasSurface
          ref={containerRef}
          data-testid="kg-sigma-canvas"
          role="img"
          aria-label="Actual project elements and facts graph"
        />
        {notice ? (
          <GraphCanvasNotice>
            {notice === "Loading graph..." ? <Spinner label={notice} /> : null}
            <span>{notice}</span>
          </GraphCanvasNotice>
        ) : null}
      </GraphCanvasFrame>
    </GraphRoute>
  );

  if (embedded) return graph;

  return (
    <ViewFrame testId={frameTestId}>
      {graph}
    </ViewFrame>
  );
}
