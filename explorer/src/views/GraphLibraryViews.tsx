import { useEffect, useMemo, useRef, useState } from "react";
import { css, cx } from "@linaria/atomic";
import Graph from "graphology";
import Sigma from "sigma";
import forceAtlas2 from "graphology-layout-forceatlas2";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { useExplorerUiState } from "../components/ExplorerUiState";
import type { KnowledgeGraphNode, KnowledgeGraphProjection } from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { cssVar, roleColorValue } from "@ds";

type GraphEdge = NonNullable<KnowledgeGraphProjection["edges"]>[number] & {
  relCategory?: RelationCategory;
};

type RelationCategory =
  | "derive"
  | "specify"
  | "refine"
  | "verify"
  | "satisfy"
  | "attach"
  | "concept-reference"
  | "trace";

type OverlayKey = "cross" | "verification" | "trace";

const STRUCTURAL_RELATIONS = new Set<RelationCategory>([
  "derive",
  "specify",
  "refine",
  "verify",
  "satisfy",
]);

const graphRouteUX = css`
  box-sizing: border-box;
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-left: var(--ex-current-left-width);
  padding-right: 0;

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }
`;

const graphRouteSkinX = css`
  background: var(--bg-surface);
  color: var(--text-body);
`;

const graphCanvasWrapUX = css`
  position: relative;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
`;

const graphCanvasWrapSkinX = css`
  background: var(--bg-canvas);
`;

const graphCanvasUX = css`
  --ex-graph-diagram-min-h: 520px;
  width: 100%;
  height: 100%;
  min-height: var(--ex-graph-diagram-min-h);
`;

const graphNoticeUX = css`
  position: absolute;
  top: 50%;
  left: 50%;
  font-size: var(--text-base);
  font-style: italic;
  transform: translate(-50%, -50%);
`;

const graphNoticeSkinX = css`
  color: var(--text-muted);
`;

function nodeKind(node: KnowledgeGraphNode): string {
  return node.element_type || node.node_type || node.type || "other";
}

function roleColor(kind: string) {
  return {
    fill: roleColorValue(kind),
    border: roleColorValue(kind, "ink"),
  };
}

function relationCategory(edge: Pick<GraphEdge, "label" | "kind">): RelationCategory {
  const label = String(edge.label || "").toLowerCase();
  const kind = String(edge.kind || "").toLowerCase();
  if (kind === "attachment" || label === "attaches") return "attach";
  if (kind === "concept-reference" || label === "conceptref") return "concept-reference";
  if (label.includes("derive")) return "derive";
  if (label.includes("specif")) return "specify";
  if (label.includes("refine")) return "refine";
  if (label.includes("verif")) return "verify";
  if (label.includes("satisf")) return "satisfy";
  if (label.includes("trace")) return "trace";
  return "trace";
}

function overlayVisible(category: RelationCategory, activeOverlays: Set<OverlayKey>) {
  if (category === "attach" || category === "concept-reference") {
    return activeOverlays.has("cross");
  }
  if (category === "verify" || category === "satisfy") {
    return activeOverlays.has("verification");
  }
  if (category === "trace") return activeOverlays.has("trace");
  return true;
}

function edgeParticipatesInLayout(edge: GraphEdge) {
  return STRUCTURAL_RELATIONS.has(edge.relCategory ?? relationCategory(edge));
}

function truncate(value: string | undefined, max: number) {
  const text = value ?? "";
  return text.length > max ? `${text.slice(0, Math.max(1, max - 1))}...` : text;
}

function nodeLabelLimit(node: KnowledgeGraphNode) {
  return ["capability", "requirement", "ontology"].includes(nodeKind(node)) ? 26 : 34;
}

function nodeSize(node: KnowledgeGraphNode, edges: GraphEdge[]) {
  const degree = edges.filter((edge) => edge.source === node.id || edge.target === node.id).length;
  return Math.min(16, 4 + Math.sqrt(degree + 1) * 1.6);
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

function mutedHexColor() {
  return cssVar("--text-muted");
}

function isOpenableGraphNode(node: KnowledgeGraphNode | null | undefined): node is KnowledgeGraphNode {
  if (!node?.identifier) return false;
  const kind = nodeKind(node);
  return kind !== "resource" && kind !== "concept";
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
  return { nodes, nodeById, edges };
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
    refinement: [3, 2],
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
  const { nodes, nodeById, edges } = useMemo(
    () => buildGraphData(store.knowledge_graph),
    [store.knowledge_graph],
  );
  const containerRef = useRef<HTMLDivElement | null>(null);
  const graphRef = useRef<Graph | null>(null);
  const rendererRef = useRef<Sigma | null>(null);
  const selectedRef = useRef<string | null>(null);
  const hoveredRef = useRef<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  const visibleNode = (node: KnowledgeGraphNode) =>
    activeTypes.has(nodeKind(node));
  const visibleEdge = (edge: GraphEdge) => {
    const source = nodeById.get(edge.source);
    const target = nodeById.get(edge.target);
    return (
      Boolean(source && target) &&
      Boolean(source && visibleNode(source)) &&
      Boolean(target && visibleNode(target)) &&
      overlayVisible(edge.relCategory ?? relationCategory(edge), activeOverlays)
    );
  };
  useEffect(() => {
    selectedRef.current = selectedId;
    rendererRef.current?.refresh();
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
          size: nodeSize(node, edges),
          color: roleColor(kind).fill,
          hidden: !visibleNode(node),
        });
      });
      edges.forEach((edge, index) => {
        if (!edgeParticipatesInLayout(edge)) return;
        graph?.addDirectedEdgeWithKey(`e${index}`, edge.source, edge.target, {
          ...edge,
          type: "arrow",
          label: edge.label,
          size: edge.kind === "attachment" || edge.kind === "concept-reference" ? 0.8 : 1.1,
          color: mutedHexColor(),
          hidden: !visibleEdge(edge),
        });
      });
      try {
        const settings = forceAtlas2.inferSettings(graph);
        forceAtlas2.assign(graph, {
          iterations: graph.order > 650 ? 260 : 180,
          settings: {
            ...settings,
            adjustSizes: true,
            barnesHutOptimize: true,
            gravity: 1.6,
            scalingRatio: 18,
            slowDown: 2,
          },
        });
      } catch (error) {
        console.warn("[Reqvire KG] ForceAtlas2 layout failed", error);
      }
      edges.forEach((edge, index) => {
        const key = `e${index}`;
        if (graph?.hasEdge(key)) return;
        graph?.addDirectedEdgeWithKey(key, edge.source, edge.target, {
          ...edge,
          type: "arrow",
          label: edge.label,
          size: edge.kind === "attachment" || edge.kind === "concept-reference" ? 0.8 : 1.1,
          color: mutedHexColor(),
          hidden: !visibleEdge(edge),
        });
      });

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
          const selectionIds = [selectedRef.current].filter(
            (id): id is string => Boolean(id),
          );
          const hoverIds = [hoveredRef.current].filter(
            (id): id is string => Boolean(id),
          );
          const dragIds = [draggedNodeId].filter((id): id is string => Boolean(id));
          const focusIds = [...selectionIds, ...hoverIds, ...dragIds];
          const hasFocus = focusIds.length > 0;
          const inFocusNeighborhood = focusIds.some(
            (focusId) =>
              node === focusId ||
              edges.some(
                (edge) =>
                  visibleEdge(edge) &&
                  ((edge.source === focusId && edge.target === node) ||
                    (edge.target === focusId && edge.source === node)),
              ),
          );
          const dragged = draggedNodeId === node;
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
          const focusIds = [selectedRef.current, hoveredRef.current, draggedNodeId].filter(
            (id): id is string => Boolean(id),
          );
          if (focusIds.length === 0 || attributes.hidden) {
            result.hidden = true;
            return result;
          }
          if (
            !focusIds.some(
              (focusId) => attributes.source === focusId || attributes.target === focusId,
            )
          ) {
            result.hidden = true;
            result.label = "";
          } else {
            result.hidden = false;
            result.color = cssVar("--edge-default");
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
        if (isOpenableGraphNode(node)) onOpenElement?.(node.identifier);
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
        setGraphCursor("pointer");
        renderer?.refresh();
      });
      renderer.on("leaveNode", (event) => {
        if (hoveredRef.current === event.node) hoveredRef.current = null;
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

    return () => {
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
      setGraphCursor("");
      renderer?.kill();
      graphRef.current = null;
      rendererRef.current = null;
      graph = null;
      renderer = null;
    };
  }, [edges, nodeById, nodes, onOpenElement, setSelectedId]);

  const graph = (
    <div className={cx("graph-route", graphRouteUX, graphRouteSkinX)}>
      <div className={cx("graph-canvas-wrap", graphCanvasWrapUX, graphCanvasWrapSkinX)}>
        <div
          ref={containerRef}
          data-testid="kg-sigma-canvas"
          role="img"
          aria-label="Actual project elements and facts graph"
          className={cx("graph-library-canvas", graphCanvasUX)}
        />
        {notice && <div className={cx("graph-render-notice", graphNoticeUX, graphNoticeSkinX)}>{notice}</div>}
      </div>
    </div>
  );

  if (embedded) return graph;

  return (
    <ViewFrame testId={frameTestId}>
      {graph}
    </ViewFrame>
  );
}
