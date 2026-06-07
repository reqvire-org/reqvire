import { useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import {
  Box,
  Flex,
  Grid,
  Heading,
  Text,
  TextField,
} from "@radix-ui/themes";
import { MagnifyingGlassIcon } from "@radix-ui/react-icons";
import Graph from "graphology";
import Sigma from "sigma";
import forceAtlas2 from "graphology-layout-forceatlas2";
import cytoscape, { type Core, type ElementDefinition } from "cytoscape";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { useExplorerUiState } from "../components/ExplorerUiState";
import type {
  KnowledgeGraphFact,
  KnowledgeGraphNode,
  KnowledgeGraphProjection,
} from "../store/types";
import { REQVIRE_SURFACE_BASE } from "../theme";
import { ViewFrame } from "./ViewFrame";

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

type LayoutMode = "structural" | "concentric" | "breadthfirst" | "circle" | "grid";
type ClusterMode = "structural" | "modularity";
type OverlayKey = "cross" | "verification" | "trace";

const ROLE_COLORS: Record<string, { fill: string; border: string }> = {
  capability: { fill: "#1976D2", border: "#0f4d8a" },
  requirement: { fill: "#673AB7", border: "#452480" },
  refinement: { fill: "#673AB7", border: "#452480" },
  verification: { fill: "#4CAF50", border: "#2f6f32" },
  ontology: { fill: "#B08A00", border: "#775d00" },
  resource: { fill: "#FFCA28", border: "#b88c00" },
  concept: { fill: "#8D6E63", border: "#5f493f" },
  other: { fill: "#424242", border: "#232323" },
};

const STRUCTURAL_RELATIONS = new Set<RelationCategory>([
  "derive",
  "specify",
  "refine",
]);

const COMMUNITY_COLORS = [
  "#1976D2",
  "#673AB7",
  "#4CAF50",
  "#B08A00",
  "#8D6E63",
  "#00838F",
  "#C62828",
  "#5D4037",
  "#7B1FA2",
  "#2E7D32",
];

function nodeKind(node: KnowledgeGraphNode): string {
  return node.node_type ?? node.type ?? node.element_type ?? "other";
}

function roleColor(kind: string) {
  return ROLE_COLORS[kind] ?? ROLE_COLORS.other;
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

function graphSearchCorpus(node: KnowledgeGraphNode) {
  const facts = [
    ...(node.metadata ?? []),
    ...(node.governance ?? []),
    ...(node.outgoing ?? []),
    ...(node.incoming ?? []),
    ...(node.attachments ?? []),
    ...(node.concept_references ?? []),
  ];
  return [
    node.label,
    node.element_type,
    node.identifier,
    node.file_path,
    node.description,
    ...facts.flatMap((fact) => [fact.name, fact.value, fact.kind]),
  ]
    .join(" ")
    .toLowerCase();
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

function cytoscapeNodeSize(node: KnowledgeGraphNode, edges: GraphEdge[]) {
  const degree = edges.filter((edge) => edge.source === node.id || edge.target === node.id).length;
  return Math.min(34, 11 + Math.sqrt(degree + 1) * 3.8);
}

function dimNodeColor(color: string, alpha: number) {
  const foreground = parseHexColor(color);
  const background = parseHexColor(REQVIRE_SURFACE_BASE);
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

function readableTextColor(background: string) {
  const parsed = parseHexColor(background);
  if (!parsed) return "#172027";
  const luminance = (0.299 * parsed.r + 0.587 * parsed.g + 0.114 * parsed.b) / 255;
  return luminance > 0.55 ? "#172027" : "#ffffff";
}

function factRows(facts: KnowledgeGraphFact[] | undefined) {
  return (facts ?? []).map((fact, index) => (
    <div key={`${fact.name}-${fact.value}-${index}`} className="graph-fact-row">
      <span className="graph-fact-name">{fact.name}</span>
      {fact.link ? (
        <a className="graph-fact-value" href={fact.link}>
          {fact.value}
        </a>
      ) : (
        <span className="graph-fact-value">{fact.value}</span>
      )}
    </div>
  ));
}

function InspectorSection({
  title,
  children,
}: {
  title: string;
  children: ReactNode;
}) {
  return (
    <section className="graph-inspector-section">
      <Text as="div" size="1" weight="bold" className="uppercase">
        {title}
      </Text>
      {children}
    </section>
  );
}

function NodeInspector({
  title,
  node,
  empty,
  onOpenElement,
}: {
  title: string;
  node: KnowledgeGraphNode | null;
  empty: string;
  onOpenElement: (id: string) => void;
}) {
  return (
    <aside className="graph-right-inspector">
      <div className="graph-inspector-header">
        <Heading as="h2" size="3">
          {node?.label ?? title}
        </Heading>
      </div>
      <div className="graph-inspector-body">
        {node ? (
          <Flex direction="column" gap="3">
            <button
              type="button"
              onClick={() => onOpenElement(node.identifier)}
              className="graph-dark-command"
            >
              Open element detail
            </button>
            <InspectorSection title="Kind">
              <span
                className="graph-kind-pill"
                style={{
                  backgroundColor: roleColor(nodeKind(node)).fill,
                  borderColor: roleColor(nodeKind(node)).border,
                  color: readableTextColor(roleColor(nodeKind(node)).fill),
                }}
              >
                {node.element_type || nodeKind(node)}
              </span>
            </InspectorSection>
            <InspectorSection title="Identifier">
              <div className="graph-fact-row">
                <span className="graph-fact-name">id</span>
                <span className="graph-fact-value graph-mono">{node.identifier}</span>
              </div>
            </InspectorSection>
            {node.file_path && (
              <InspectorSection title="Source">
                <div className="graph-fact-row">
                  <span className="graph-fact-name">file</span>
                  {node.link ? (
                    <a className="graph-fact-value" href={node.link}>
                      {node.file_path}
                      {node.line_number ? `:${node.line_number}` : ""}
                    </a>
                  ) : (
                    <span className="graph-fact-value">
                      {node.file_path}
                      {node.line_number ? `:${node.line_number}` : ""}
                    </span>
                  )}
                </div>
              </InspectorSection>
            )}
            <InspectorSection title="Description">
              <Text size="2">{node.description || "None specified."}</Text>
            </InspectorSection>
            {(node.governance ?? []).length > 0 && (
              <InspectorSection title="Governance">{factRows(node.governance)}</InspectorSection>
            )}
            {(node.metadata ?? []).length > 0 && (
              <InspectorSection title="Metadata">{factRows(node.metadata)}</InspectorSection>
            )}
            {(node.outgoing ?? []).length > 0 && (
              <InspectorSection title="Outgoing Facts">{factRows(node.outgoing)}</InspectorSection>
            )}
            {(node.incoming ?? []).length > 0 && (
              <InspectorSection title="Incoming Facts">{factRows(node.incoming)}</InspectorSection>
            )}
            {(node.attachments ?? []).length > 0 && (
              <InspectorSection title="Attachments">{factRows(node.attachments)}</InspectorSection>
            )}
            {(node.concept_references ?? []).length > 0 && (
              <InspectorSection title="Concept References">
                {factRows(node.concept_references)}
              </InspectorSection>
            )}
          </Flex>
        ) : (
          <Text size="2" color="gray" className="italic">
            {empty}
          </Text>
        )}
      </div>
    </aside>
  );
}

function buildGraphData(projection: KnowledgeGraphProjection | undefined) {
  const rawNodes = projection?.nodes ?? [];
  const nodes = rawNodes.map((node) => ({ ...node, node_type: nodeKind(node) }));
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
  frameTestId = "knowledge-graph",
  onOpenElement,
}: {
  frameTestId?: string;
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store } = useStore();
  const { modelTypes: activeTypes, modelOverlays: activeOverlays } = useExplorerUiState();
  const { nodes, nodeById, edges } = useMemo(
    () => buildGraphData(store.knowledge_graph),
    [store.knowledge_graph],
  );
  const containerRef = useRef<HTMLDivElement | null>(null);
  const graphRef = useRef<Graph | null>(null);
  const rendererRef = useRef<Sigma | null>(null);
  const selectedRef = useRef<string | null>(null);
  const hoveredRef = useRef<string | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [query, setQuery] = useState("");
  const [notice, setNotice] = useState<string | null>(null);

  const selected = selectedId ? nodeById.get(selectedId) ?? null : null;
  const searchTerm = query.trim().toLowerCase();
  const visibleNode = (node: KnowledgeGraphNode) =>
    activeTypes.has(nodeKind(node)) &&
    (!searchTerm || graphSearchCorpus(node).includes(searchTerm));
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
  const results = useMemo(
    () =>
      searchTerm
        ? nodes.filter(visibleNode).slice(0, 20)
        : [],
    [activeTypes, activeOverlays, nodes, query],
  );

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
  }, [activeTypes, activeOverlays, edges, nodes, query]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container || nodes.length === 0) {
      setNotice(nodes.length === 0 ? "No project graph nodes were exported." : null);
      return undefined;
    }

    let disposed = false;
    let graph: Graph | null = null;
    let renderer: Sigma | null = null;
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
          color: "#6d7b83",
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
          color: "#6d7b83",
          hidden: !visibleEdge(edge),
        });
      });

      renderer = new Sigma(graph, container, {
        allowInvalidContainer: true,
        defaultEdgeType: "arrow",
        renderEdgeLabels: true,
        labelDensity: 0.12,
        labelGridCellSize: 80,
        labelRenderedSizeThreshold: 9,
        nodeReducer: (node, attributes) => {
          const result = { ...attributes };
          const focusIds = [selectedRef.current, hoveredRef.current].filter(
            (id): id is string => Boolean(id),
          );
          const inFocus =
            focusIds.length === 0 ||
            focusIds.some(
              (focusId) =>
                node === focusId ||
                edges.some(
                  (edge) =>
                    visibleEdge(edge) &&
                    ((edge.source === focusId && edge.target === node) ||
                      (edge.target === focusId && edge.source === node)),
                ),
            );
          if (focusIds.includes(String(node))) {
            result.label = attributes.fullLabel ?? attributes.label ?? "";
            result.forceLabel = true;
          } else if (!inFocus) {
            result.color = dimNodeColor(String(attributes.color ?? "#8da0ae"), 0.2);
            result.label = "";
            result.forceLabel = false;
          } else if (focusIds.length > 0) {
            result.forceLabel = true;
          }
          return result;
        },
        edgeReducer: (_edge, attributes) => {
          const result = { ...attributes };
          const focusIds = [selectedRef.current, hoveredRef.current].filter(
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
            result.color = "#53636b";
            result.size = Math.max(1.1, Number(attributes.size ?? 1) * 1.15);
            result.forceLabel = true;
          }
          return result;
        },
      });
      graphRef.current = graph;
      rendererRef.current = renderer;
      setNotice(null);

      renderer.on("clickNode", (event) => setSelectedId(event.node));
      renderer.on("clickStage", () => setSelectedId(null));
      renderer.on("enterNode", (event) => {
        hoveredRef.current = event.node;
        renderer?.refresh();
      });
      renderer.on("leaveNode", (event) => {
        if (hoveredRef.current === event.node) hoveredRef.current = null;
        renderer?.refresh();
      });
      renderer.getCamera().animatedReset({ duration: 250 });
    } catch (error) {
      console.error("[Reqvire KG] Sigma/Graphology renderer failed", error);
      setNotice("Knowledge Graph renderer failed. Check the browser console for details.");
    }

    return () => {
      disposed = true;
      if (!disposed) return;
      renderer?.kill();
      graphRef.current = null;
      rendererRef.current = null;
      graph = null;
      renderer = null;
    };
  }, [edges, nodes]);

  function focusResult(node: KnowledgeGraphNode) {
    setSelectedId(node.id);
    const renderer = rendererRef.current;
    const graph = graphRef.current;
    if (!renderer || !graph || !graph.hasNode(node.id)) return;
    const display = renderer.getNodeDisplayData(node.id);
    if (!display) return;
    const camera = renderer.getCamera();
    const state = camera.getState();
    camera.animate({ x: display.x, y: display.y, ratio: Math.min(state.ratio, 0.9) }, { duration: 280 });
  }

  return (
    <ViewFrame testId={frameTestId}>
      <Grid columns={{ initial: "1fr", lg: "minmax(0, 1fr) 390px" }} className="graph-route">
        <Box className="graph-canvas-wrap">
          <div
            ref={containerRef}
            data-testid="kg-sigma-canvas"
            role="img"
            aria-label="Actual project elements and facts graph"
            className="graph-library-canvas"
          />
          {notice && <div className="graph-render-notice">{notice}</div>}
        </Box>
        <Box className="graph-sidebar">
          <div className="graph-search-panel">
            <TextField.Root
              placeholder="Search elements, facts, files, relations, concept references"
              value={query}
              onChange={(event) => setQuery(event.target.value)}
            >
              <TextField.Slot>
                <MagnifyingGlassIcon />
              </TextField.Slot>
            </TextField.Root>
            {results.length > 0 && (
              <ul className="graph-results">
                {results.map((node) => (
                  <li key={node.id}>
                    <button type="button" onClick={() => focusResult(node)}>
                      <span
                        className="graph-result-swatch"
                        style={{ backgroundColor: roleColor(nodeKind(node)).fill }}
                      />
                      <span>{node.label}</span>
                    </button>
                  </li>
                ))}
              </ul>
            )}
          </div>
          <NodeInspector
            title="Fact Inspector"
            node={selected}
            empty="Search or select a node to inspect actual project facts: element type, relations, attachments, governance, concept references, and source location."
            onOpenElement={onOpenElement}
          />
          <div className="graph-summary-strip">
            <span>
              Submodels <strong>{store.knowledge_graph.summary?.submodels ?? store.knowledge_graph.submodels?.length ?? 0}</strong>
            </span>
            <span>
              Elements <strong>{store.knowledge_graph.summary?.elements ?? nodes.length}</strong>
            </span>
            <span>
              Relations <strong>{store.knowledge_graph.summary?.relations ?? edges.length}</strong>
            </span>
            <span>
              Attachments <strong>{store.knowledge_graph.summary?.attachments ?? 0}</strong>
            </span>
          </div>
        </Box>
      </Grid>
    </ViewFrame>
  );
}

function cytoscapeStyle(labelsEnabled: boolean): cytoscape.Stylesheet[] {
  return [
    {
      selector: "node",
      style: {
        "background-color": "data(color)",
        "border-color": "data(borderColor)",
        "border-width": 2,
        width: "data(size)",
        height: "data(size)",
        label: labelsEnabled ? "data(shortLabel)" : "",
        "font-size": 10,
        "font-weight": 600,
        color: "#111827",
        "text-background-color": "#ffffff",
        "text-background-opacity": 0.88,
        "text-background-padding": "2px",
        "text-border-color": "#111827",
        "text-border-width": 0.5,
        "text-border-opacity": 0.35,
        "text-valign": "center",
        "text-halign": "right",
        "text-margin-x": 5,
        "overlay-opacity": 0,
      },
    },
    {
      selector: 'node[node_type = "capability"], node[node_type = "requirement"]',
      style: { shape: "ellipse" },
    },
    {
      selector:
        'node[node_type = "refinement"], node[node_type = "ontology"], node[node_type = "verification"], node[node_type = "resource"], node[node_type = "concept"], node[node_type = "other"]',
      style: { shape: "round-rectangle" },
    },
    {
      selector: "edge",
      style: {
        "curve-style": "bezier",
        "target-arrow-shape": "triangle",
        "target-arrow-color": "#4b5563",
        "line-color": "#4b5563",
        width: "data(width)",
        label: labelsEnabled ? "data(label)" : "",
        "font-size": 7,
        color: "#334155",
        "text-background-color": REQVIRE_SURFACE_BASE,
        "text-background-opacity": 0.74,
        "text-background-padding": "1px",
        "arrow-scale": 0.72,
      },
    },
    { selector: ".faded", style: { opacity: 0.18, "text-opacity": 0 } },
    {
      selector: ".focused",
      style: { opacity: 1, "text-opacity": 1, "border-width": 4, "z-index": 10 },
    },
    {
      selector: ".community-colored",
      style: { "border-width": 5, "border-color": "data(communityColor)" },
    },
  ];
}

function layoutFor(mode: LayoutMode): cytoscape.LayoutOptions {
  if (mode === "concentric") {
    return {
      name: "concentric",
      animate: false,
      fit: true,
      padding: 80,
      minNodeSpacing: 14,
      concentric: (node) => node.degree(false),
      levelWidth: (nodes) => Math.max(1, nodes.maxDegree(false) / 7),
    };
  }
  if (mode === "breadthfirst") {
    return {
      name: "breadthfirst",
      animate: false,
      fit: true,
      padding: 80,
      directed: true,
      spacingFactor: 1.15,
      avoidOverlap: true,
    };
  }
  if (mode === "circle") return { name: "circle", animate: false, fit: true, padding: 80, avoidOverlap: true };
  if (mode === "grid") return { name: "grid", animate: false, fit: true, padding: 80, avoidOverlap: true };
  return {
    name: "cose",
    animate: false,
    fit: true,
    padding: 80,
    nodeRepulsion: () => 7800,
    idealEdgeLength: () => 90,
    edgeElasticity: () => 120,
    nestingFactor: 1.25,
    gravity: 0.65,
    numIter: 1100,
  };
}

function showCytoscapeElement(element: unknown) {
  (element as { show: () => void }).show();
}

function hideCytoscapeElement(element: unknown) {
  (element as { hide: () => void }).hide();
}

export function Kn2View({
  onOpenElement,
}: {
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store } = useStore();
  const { nodes, nodeById, edges } = useMemo(
    () => buildGraphData(store.knowledge_graph),
    [store.knowledge_graph],
  );
  const containerRef = useRef<HTMLDivElement | null>(null);
  const cyRef = useRef<Core | null>(null);
  const selectedRef = useRef<string | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [query, setQuery] = useState("");
  const ui = useExplorerUiState();
  const layoutMode: LayoutMode = ui.kn2LayoutMode;
  const clusterMode: ClusterMode = ui.kn2ClusterMode;
  const focusRadius = ui.kn2FocusRadius;
  const focusOnly = ui.kn2FocusOnly;
  const labelsEnabled = ui.kn2LabelsEnabled;
  const activeRelations = useMemo(
    () => new Set<RelationCategory>(ui.kn2Relations),
    [ui.kn2Relations],
  );
  const activeOverlays = ui.kn2Overlays;
  const [status, setStatus] = useState("Cytoscape is initializing.");
  const [visibleCounts, setVisibleCounts] = useState({ nodes: 0, edges: 0, overlays: 0 });
  const [communityMap, setCommunityMap] = useState<Map<string, number>>(() => new Map());
  const selected = selectedId ? nodeById.get(selectedId) ?? null : null;
  const searchTerm = query.trim().toLowerCase();
  const results = useMemo(
    () => (searchTerm ? nodes.filter((node) => graphSearchCorpus(node).includes(searchTerm)).slice(0, 24) : []),
    [nodes, searchTerm],
  );

  useEffect(() => {
    selectedRef.current = selectedId;
  }, [selectedId]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container || nodes.length === 0) {
      setStatus(nodes.length === 0 ? "No project graph nodes were exported." : "Cytoscape container is unavailable.");
      return undefined;
    }
    let cy: Core | null = null;
    try {
      const elements: ElementDefinition[] = [
        ...nodes.map((node) => {
          const kind = nodeKind(node);
          return {
            group: "nodes" as const,
            data: {
              ...node,
              node_type: kind,
              color: roleColor(kind).fill,
              borderColor: roleColor(kind).border,
              size: cytoscapeNodeSize(node, edges),
              shortLabel: truncate(node.label, nodeLabelLimit(node)),
            },
          };
        }),
        ...edges.map((edge, index) => ({
          group: "edges" as const,
          data: {
            ...edge,
            id: `e${index}`,
            relCategory: edge.relCategory ?? relationCategory(edge),
            width: edge.kind === "attachment" || edge.kind === "concept-reference" ? 1 : 1.4,
          },
        })),
      ];
      cy = cytoscape({
        container,
        elements,
        minZoom: 0.08,
        maxZoom: 4,
        wheelSensitivity: 0.18,
        style: cytoscapeStyle(labelsEnabled),
        layout: { name: "grid", fit: false },
      });
      cyRef.current = cy;
      cy.on("tap", "node", (event) => {
        const id = event.target.id();
        setSelectedId(id);
        event.target.animate({ style: { "border-width": 4 } }, { duration: 120 });
        event.target.cy().animate({ center: { eles: event.target }, zoom: Math.min(1.4, Math.max(event.target.cy().zoom(), 0.75)) }, { duration: 260 });
      });
      cy.on("tap", (event) => {
        if (event.target === cy) setSelectedId(null);
      });
      cy.on("mouseover", "node", (event) => {
        applyCytoscapeFilters(event.target.id());
      });
      cy.on("mouseout", "node", () => {
        applyCytoscapeFilters();
      });
      cy.elements().layout(layoutFor("structural")).run();
      setCommunityMap(detectCommunities(cy, clusterMode, store.knowledge_graph.submodels ?? [], activeRelations));
      setStatus(`Cytoscape loaded ${nodes.length} nodes / ${edges.length} edges. Structural subgraphs ignore cross-subgraph overlays.`);
    } catch (error) {
      console.error("[Reqvire KN2] Cytoscape renderer failed", error);
      setStatus("Cytoscape renderer failed. Check the browser console for details.");
    }

    return () => {
      cy?.destroy();
      cyRef.current = null;
    };
  }, [edges, nodes]);

  useEffect(() => {
    const cy = cyRef.current;
    if (!cy) return;
    cy.style(cytoscapeStyle(labelsEnabled));
    cy.resize();
  }, [labelsEnabled]);

  useEffect(() => {
    const cy = cyRef.current;
    if (!cy) return;
    const nextMap = detectCommunities(cy, clusterMode, store.knowledge_graph.submodels ?? [], activeRelations);
    setCommunityMap(nextMap);
    applyCommunityColors(cy, nextMap);
    structuralCollection(cy, activeRelations).layout(layoutFor(layoutMode)).run();
    applyCytoscapeFilters();
    const source =
      clusterMode === "modularity"
        ? "modularity-style clusters"
        : store.knowledge_graph.submodels?.length
          ? "Reqvire root submodels"
          : "structural islands";
    setStatus(`Colored ${new Set(nextMap.values()).size} ${source}. Attachments, verification, satisfaction, trace, and concept overlays are ignored for cluster detection.`);
  }, [activeRelations, clusterMode, layoutMode]);

  useEffect(() => {
    applyCytoscapeFilters();
  }, [activeOverlays, focusOnly, focusRadius, selectedId]);

  function applyCytoscapeFilters(hoveredId?: string) {
    const cy = cyRef.current;
    if (!cy) return;
    showCytoscapeElement(cy.elements());
    cy.elements().removeClass("faded focused");

    cy.edges().forEach((edge) => {
      const category = edge.data("relCategory") as RelationCategory;
      if (STRUCTURAL_RELATIONS.has(category) && !activeRelations.has(category)) {
        hideCytoscapeElement(edge);
      }
      if (!STRUCTURAL_RELATIONS.has(category) && !overlayVisible(category, activeOverlays)) {
        hideCytoscapeElement(edge);
      }
    });
    if (!activeOverlays.has("cross")) hideCytoscapeElement(cy.nodes('[node_type = "concept"]'));

    const focusId = hoveredId ?? selectedRef.current;
    if (focusId) {
      const root = cy.getElementById(focusId);
      if (root.nonempty() && !root.hidden()) {
        const focus = egoCollection(root, focusRadius, activeRelations);
        if (focusOnly) {
          cy.nodes().forEach((node) => {
            if (!focus.contains(node)) hideCytoscapeElement(node);
          });
        } else {
          cy.elements().addClass("faded");
          focus.removeClass("faded").addClass("focused");
        }
      }
    }
    cy.edges().forEach((edge) => {
      if (edge.source().hidden() || edge.target().hidden()) hideCytoscapeElement(edge);
    });
    setVisibleCounts({
      nodes: cy.nodes(":visible").length,
      edges: cy.edges(":visible").length,
      overlays: cy.edges(":visible").filter((edge) => {
        const category = edge.data("relCategory") as RelationCategory;
        return category === "attach" || category === "concept-reference";
      }).length,
    });
    const overlays = [
      activeOverlays.has("cross") ? "cross-subgraph" : null,
      activeOverlays.has("verification") ? "verification" : null,
      activeOverlays.has("trace") ? "trace" : null,
    ].filter(Boolean);
    setStatus(
      `View: ${cy.nodes(":visible").length} visible nodes / ${cy.edges(":visible").length} visible edges; focus ${
        focusId ? `r${focusRadius}${focusOnly ? " only" : ""}` : "all"
      }; structural relations ${Array.from(activeRelations).join(", ") || "none"}; overlays ${
        overlays.join(", ") || "off"
      }.`,
    );
  }

  function focusSearchResult(node: KnowledgeGraphNode) {
    const cy = cyRef.current;
    setSelectedId(node.id);
    if (!cy) return;
    const cyNode = cy.getElementById(node.id);
    if (cyNode.nonempty()) {
      cy.animate({ center: { eles: cyNode }, zoom: Math.min(1.4, Math.max(cy.zoom(), 0.75)) }, { duration: 260 });
    }
  }

  return (
    <ViewFrame testId="kn2">
      <Grid columns={{ initial: "1fr", lg: "minmax(0, 1fr) 390px" }} className="graph-route">
        <Box className="graph-canvas-wrap">
          <div
            ref={containerRef}
            data-testid="kn2-cytoscape-canvas"
            role="img"
            aria-label="Cytoscape project graph POC"
            className="graph-library-canvas"
          />
        </Box>
        <Box className="graph-sidebar">
          <div className="graph-search-panel">
            <TextField.Root placeholder="Search KN2 graph, facts, overlays" value={query} onChange={(event) => setQuery(event.target.value)}>
              <TextField.Slot>
                <MagnifyingGlassIcon />
              </TextField.Slot>
            </TextField.Root>
            {results.length > 0 && (
              <ul className="graph-results">
                {results.map((node) => (
                  <li key={node.id}>
                    <button type="button" onClick={() => focusSearchResult(node)}>
                      <span
                        className="graph-result-swatch"
                        style={{ backgroundColor: roleColor(nodeKind(node)).fill }}
                      />
                      <span>{node.label}</span>
                    </button>
                  </li>
                ))}
              </ul>
            )}
          </div>
          <NodeInspector
            title="Cytoscape Inspector"
            node={selected}
            empty="Select a node to inspect project facts. Structural subgraphs ignore cross-subgraph overlays."
            onOpenElement={onOpenElement}
          />
          <div className="graph-summary-strip">
            <span className="graph-summary-status" title={status}>
              Status <strong>{status}</strong>
            </span>
            <span>
              Nodes <strong>{visibleCounts.nodes}</strong>
            </span>
            <span>
              Edges <strong>{visibleCounts.edges}</strong>
            </span>
            <span>
              Overlays <strong>{visibleCounts.overlays}</strong>
            </span>
            <span>
              Focus <strong>{selectedId ? `r${focusRadius}${focusOnly ? " only" : ""}` : "all"}</strong>
            </span>
            <span>
              Clusters <strong>{new Set(communityMap.values()).size}</strong>
            </span>
          </div>
        </Box>
      </Grid>
    </ViewFrame>
  );
}

function structuralCollection(cy: Core, activeRelations: Set<RelationCategory>) {
  const nodes = cy.nodes().filter((node) => node.data("node_type") !== "concept");
  const edges = cy.edges().filter((edge) => {
    const category = edge.data("relCategory") as RelationCategory;
    return STRUCTURAL_RELATIONS.has(category) && activeRelations.has(category);
  });
  return nodes.union(edges);
}

function egoCollection(root: cytoscape.NodeSingular, radius: number, activeRelations: Set<RelationCategory>) {
  let keep = root as unknown as cytoscape.CollectionReturnValue;
  let frontier = [root];
  const seen = new Set([root.id()]);
  for (let depth = 0; depth < radius; depth += 1) {
    const nextFrontier: cytoscape.NodeSingular[] = [];
    frontier.forEach((node) => {
      node.connectedEdges().forEach((edge) => {
        const category = edge.data("relCategory") as RelationCategory;
        if (!STRUCTURAL_RELATIONS.has(category) || !activeRelations.has(category) || edge.hidden()) return;
        const other = edge.connectedNodes().difference(node)[0];
        if (!other || other.hidden()) return;
        keep = keep.union(edge).union(other);
        if (!seen.has(other.id())) {
          seen.add(other.id());
          nextFrontier.push(other);
        }
      });
    });
    frontier = nextFrontier;
  }
  return keep;
}

function detectCommunities(
  cy: Core,
  clusterMode: ClusterMode,
  submodels: NonNullable<KnowledgeGraphProjection["submodels"]>,
  activeRelations: Set<RelationCategory>,
) {
  if (clusterMode === "modularity") {
    return detectModularityStyleCommunities(cy, activeRelations);
  }
  const result = new Map<string, number>();
  const seen = new Set<string>();
  let community = 0;
  const roots = submodels.length > 0 ? submodels.map((submodel) => submodel.root_id) : cy.nodes().map((node) => node.id());
  roots.forEach((id) => {
    const root = cy.getElementById(id);
    if (root.empty() || seen.has(root.id()) || root.data("node_type") === "concept") return;
    community = visitStructuralComponent(root, result, seen, community, activeRelations);
  });
  cy.nodes().forEach((node) => {
    if (seen.has(node.id()) || node.data("node_type") === "concept") return;
    community = visitStructuralComponent(node, result, seen, community, activeRelations);
  });
  return result;
}

function visitStructuralComponent(
  start: cytoscape.NodeSingular,
  result: Map<string, number>,
  seen: Set<string>,
  community: number,
  activeRelations: Set<RelationCategory>,
) {
  const queue = [start];
  seen.add(start.id());
  while (queue.length) {
    const node = queue.shift();
    if (!node) break;
    result.set(node.id(), community);
    node.connectedEdges().forEach((edge) => {
      const category = edge.data("relCategory") as RelationCategory;
      if (!STRUCTURAL_RELATIONS.has(category) || !activeRelations.has(category)) return;
      const other = edge.connectedNodes().difference(node)[0];
      if (!other || seen.has(other.id()) || other.data("node_type") === "concept") return;
      seen.add(other.id());
      queue.push(other);
    });
  }
  return community + 1;
}

function detectModularityStyleCommunities(cy: Core, activeRelations: Set<RelationCategory>) {
  const nodes = cy.nodes().filter((node) => node.data("node_type") !== "concept");
  const labels = new Map(nodes.map((node) => [node.id(), node.id()]));
  for (let iteration = 0; iteration < 24; iteration += 1) {
    let changed = false;
    nodes.forEach((node) => {
      const counts = new Map<string, number>();
      node.connectedEdges().forEach((edge) => {
        const category = edge.data("relCategory") as RelationCategory;
        if (!STRUCTURAL_RELATIONS.has(category) || !activeRelations.has(category)) return;
        const other = edge.connectedNodes().difference(node)[0];
        if (!other || other.data("node_type") === "concept") return;
        const label = labels.get(other.id());
        if (!label) return;
        counts.set(label, (counts.get(label) ?? 0) + 1);
      });
      if (counts.size === 0) return;
      const next = Array.from(counts.entries()).sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))[0][0];
      if (next !== labels.get(node.id())) {
        labels.set(node.id(), next);
        changed = true;
      }
    });
    if (!changed) break;
  }
  const compressed = new Map<string, number>();
  const result = new Map<string, number>();
  let community = 0;
  nodes.forEach((node) => {
    const label = labels.get(node.id()) ?? node.id();
    if (!compressed.has(label)) {
      compressed.set(label, community);
      community += 1;
    }
    result.set(node.id(), compressed.get(label) ?? 0);
  });
  return result;
}

function applyCommunityColors(cy: Core, communityMap: Map<string, number>) {
  cy.nodes().forEach((node) => {
    const community = communityMap.get(node.id());
    if (community === undefined) {
      node.removeClass("community-colored");
      node.removeData("communityColor");
      return;
    }
    node.data("communityColor", COMMUNITY_COLORS[community % COMMUNITY_COLORS.length]);
    node.addClass("community-colored");
  });
}
