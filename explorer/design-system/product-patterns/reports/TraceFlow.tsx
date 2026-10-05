import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { css, cx } from "@linaria/atomic";
import {
  BaseEdge, Handle, MarkerType, Position, ReactFlow, getViewportForBounds,
  type Edge, type EdgeProps, type Node, type NodeProps, type ReactFlowInstance,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { Button } from "../../components/core/Button";
import { Icon } from "../../components/core/Icon";
import { IconButton } from "../../components/core/IconButton";
import { ElementIcon } from "../../components/data/ElementIcon";
import { SegmentedControl } from "../../components/controls/SegmentedControl";
import { Spinner } from "../../components/core/Spinner";
import { ExpandableViewport } from "../../components/core/ExpandableViewport";
import {
  buildTraceFlowTopology, traceFlowNeighborhood, TRACE_NODE_HEIGHT, TRACE_NODE_WIDTH,
  type TraceFlowData, type TraceFlowElement, type TraceFlowDirection, type TraceFlowTopology, type ElementFlowData, type FlowLayoutEngine,
} from "./traceFlowLayout";
import { useTraceFlowLayout } from "./useTraceFlowLayout";

export interface TraceFlowProps {
  trace: TraceFlowData;
  layoutEngine: FlowLayoutEngine;
  onOpenElement: (id: string) => void;
  onOpenSource: (element: TraceFlowElement) => void;
}

export interface ElementFlowProps {
  data: ElementFlowData;
  layoutEngine: FlowLayoutEngine;
  onOpenElement: TraceFlowProps["onOpenElement"];
  onOpenSource: TraceFlowProps["onOpenSource"];
}

const baseUX = css`
  --ux-trace-node-width: ${TRACE_NODE_WIDTH}px;
  --ux-trace-node-height: ${TRACE_NODE_HEIGHT}px;
  position: relative;
  display: flex;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
  height: 100%;

  .ux-trace-flow-header {
    display: flex;
    flex-wrap: wrap;
    align-items: start;
    justify-content: space-between;
    gap: var(--space-8);
    padding: var(--space-12) var(--space-16);
  }
  .ux-trace-flow-heading { flex: 1 1 60%; min-width: 0; }
  .ux-trace-flow-heading h2 {
    margin: 0 0 var(--space-4);
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    line-height: var(--leading-tight);
    overflow-wrap: anywhere;
  }
  .ux-trace-flow-summary { margin: 0; font-size: var(--text-sm); }
  .ux-trace-flow-tools { display: flex; flex-wrap: wrap; align-items: center; gap: var(--space-2); }
  .ux-trace-flow-focus {
    position: absolute;
    bottom: var(--space-8);
    left: 50%;
    transform: translateX(-50%);
    z-index: var(--z-sticky);
    display: flex;
    align-items: center;
    gap: var(--space-4);
    min-width: 0;
    max-width: calc(100% - var(--space-16));
    padding: var(--space-3) var(--space-6);
    border: var(--border-w) solid var(--border-default);
    border-radius: var(--radius-pill);
    font-size: var(--text-sm);
  }
  .ux-trace-flow-focus span { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .ux-trace-flow-canvas {
    position: relative;
    flex: 1 1 auto;
    min-width: 0;
    min-height: 0;
  }
  .ux-trace-flow-status {
    position: absolute;
    top: var(--space-8);
    left: 50%;
    transform: translateX(-50%);
    z-index: var(--z-sticky);
    display: flex;
    align-items: center;
    gap: var(--space-6);
    padding: var(--space-4) var(--space-8);
    max-width: calc(100% - var(--space-16));
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
  }
  .ux-trace-flow-node {
    display: flex;
    cursor: pointer;
    flex-direction: column;
    box-sizing: border-box;
    width: var(--ux-trace-node-width);
    height: var(--ux-trace-node-height);
    padding: var(--space-6);
    gap: var(--space-4);
    border: var(--border-w) solid;
    border-radius: var(--radius-lg);
  }
  .ux-trace-flow-node-identity {
    display: flex;
    align-items: start;
    gap: var(--space-4);
    min-width: 0;
    text-decoration: none;
  }
  .ux-trace-flow-node-kind { font-size: var(--text-caption); }
  .ux-trace-flow-node-title {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    min-width: 0;
    text-align: left;
    font: inherit;
    font-size: var(--text-base);
    font-weight: var(--weight-semibold);
    line-height: var(--leading-snug);
  }
  .ux-trace-flow-node-source {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-width: 0;
    flex: 1;
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    text-decoration: none;
  }
  .ux-trace-flow-node-source span { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .ux-trace-flow-node-source svg { flex: none; }
  .ux-trace-flow-node-footer {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-top: auto;
    min-height: var(--control-sm);
  }
  .ux-trace-flow-node-footer > button { flex: none; }
  .react-flow__handle { visibility: hidden; }
  .react-flow__edge-text { font-size: var(--text-caption); font-family: var(--font-mono); }
`;

const skinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);
  .ux-trace-flow-header { border-bottom: var(--border-w) solid var(--border-subtle); }
  .ux-trace-flow-heading h2 { color: var(--text-strong); }
  .ux-trace-flow-focus { background: var(--bg-surface); box-shadow: var(--shadow-sm); }
  .ux-trace-flow-status { background: var(--bg-surface); color: var(--text-muted); box-shadow: var(--shadow-sm); }
  .ux-trace-flow-summary, .ux-trace-flow-node-kind { color: var(--text-muted); }
  .ux-trace-flow-node {
    background: var(--bg-surface);
    border-color: var(--border-default);
    box-shadow: var(--shadow-xs);
    transition: opacity var(--dur-fast) var(--ease-standard), border-color var(--dur-fast) var(--ease-standard);
  }
  .ux-trace-flow-node:hover { border-color: var(--border-strong); }
  .ux-trace-flow-node:focus-within { border-color: var(--text-strong); }
  .ux-trace-flow-node[data-focused="true"] { border-color: var(--text-strong); background: var(--bg-selected); }
  .ux-trace-flow-node[data-dimmed="true"] { opacity: 0.3; }
  .ux-trace-flow-node-identity { color: var(--text-strong); }
  .ux-trace-flow-node-identity:hover { text-decoration: underline; }
  .ux-trace-flow-node-source { color: var(--text-muted); }
  .ux-trace-flow-node-source:hover { color: var(--text-strong); }
  .ux-trace-flow-node-identity:focus-visible, .ux-trace-flow-node-source:focus-visible {
    outline: none;
    border-radius: var(--radius-xs);
    box-shadow: var(--ring-focus);
  }
  .react-flow__edge-path { stroke: var(--edge-trace); stroke-width: var(--border-w-2); }
  .react-flow__edge { transition: opacity var(--dur-fast) var(--ease-standard); }
  .react-flow__edge.ux-trace-edge-dimmed { opacity: 0.15; }
  .react-flow__edge.ux-trace-edge-highlighted .react-flow__edge-path { stroke: var(--accent); stroke-width: var(--border-w-thick); }
  .ux-trace-edge-label {
    fill: var(--text-muted);
    stroke: var(--bg-canvas);
    stroke-width: var(--space-2);
    stroke-linejoin: round;
    paint-order: stroke;
  }
  .react-flow__attribution { background: var(--bg-canvas); color: var(--text-muted); }
  @media (prefers-reduced-motion: reduce) {
    .ux-trace-flow-node, .react-flow__edge { transition: none; }
  }
`;

type FlowNodeData = Record<string, unknown> & {
  element: TraceFlowElement;
  elementType: string;
  direction: TraceFlowDirection;
  context: string;
  parentCount: number;
  collapsed: boolean;
  focused: boolean;
  dimmed: boolean;
  onToggle: () => void;
  onPreview: (active: boolean) => void;
  onFocusPath: () => void;
  onOpenElement: TraceFlowProps["onOpenElement"];
  onOpenSource: TraceFlowProps["onOpenSource"];
};
type FlowNode = Node<FlowNodeData>;
type FlowEdge = Edge<{ path: string; labelPosition: { x: number; y: number } }>;

function TraceEdge({ id, data, label, markerEnd }: EdgeProps<FlowEdge>) {
  if (!data) return null;
  return <>
    <BaseEdge id={id} path={data.path} markerEnd={markerEnd} />
    {/* Anchor text directly in graph coordinates. Cached getBBox measurements at tiny
        overview scales can produce oversized background rectangles after zooming. */}
    <text className="react-flow__edge-text ux-trace-edge-label"
      x={data.labelPosition.x} y={data.labelPosition.y} textAnchor="middle" dominantBaseline="central">{label}</text>
  </>;
}
const EDGE_TYPES = { trace: TraceEdge };

function TraceNode({ data }: NodeProps<FlowNode>) {
  const { element } = data;
  return <article className="ux-trace-flow-node nodrag nopan" aria-label={element.name}
    data-focused={data.focused} data-dimmed={data.dimmed}
    onMouseEnter={() => data.onPreview(true)} onMouseLeave={() => data.onPreview(false)}
    onFocusCapture={() => data.onPreview(true)}
    onBlurCapture={event => { if (!event.currentTarget.contains(event.relatedTarget)) data.onPreview(false); }}>
    <Handle type="target" position={data.direction === "DOWN" ? Position.Top : Position.Left} isConnectable={false} />
    <a className="ux-trace-flow-node-identity nodrag nopan" href={element.href ?? `#/elements/${encodeURI(element.id)}`}
      aria-label={element.name} title={element.name}
      onClick={event => { event.preventDefault(); data.onOpenElement(element.id); }}>
      <ElementIcon type={data.elementType} size="sm" />
      <span className="ux-trace-flow-node-title">{element.name}</span>
    </a>
    <div className="ux-trace-flow-node-kind">{data.context}</div>
    <div className="ux-trace-flow-node-footer nodrag nopan">
      <a className="ux-trace-flow-node-source" title={element.file}
      aria-label={`Open source for ${element.name}`}
      href={element.sourceHref ?? `#/content/${element.file}`}
      onClick={event => { event.preventDefault(); data.onOpenSource(element); }}>
      <Icon name="file-text" size={13} /><span>{element.file.split("/").pop()}</span>
      </a>
      <IconButton size="sm" tone="ghost" aria-label={`Focus path through ${element.name}`} title="Focus path"
        aria-pressed={data.focused} onClick={data.onFocusPath}><Icon name="target" /></IconButton>
      {data.parentCount > 0 && <Button size="sm" tone="ghost" aria-expanded={!data.collapsed}
        aria-label={`${data.collapsed ? "Expand" : "Collapse"} ancestors of ${element.name}`}
        iconLeft={<Icon name={data.collapsed ? "chevron-right" : "chevron-down"} size={13} />}
        onClick={data.onToggle}>
        {data.parentCount} {data.parentCount === 1 ? "parent" : "parents"}
      </Button>}
    </div>
    <Handle type="source" position={data.direction === "DOWN" ? Position.Bottom : Position.Right} isConnectable={false} />
  </article>;
}
const NODE_TYPES = { trace: TraceNode };

/** Fills a bounded container; callers own evaluated trace data and navigation. */
export function TraceFlow(props: TraceFlowProps) {
  return <TraceFlowContent key={props.trace.verification.id} {...props} />;
}

function TraceFlowContent({ trace, ...navigation }: TraceFlowProps) {
  const [collapsed, setCollapsed] = useState<Set<string>>(new Set());
  const topology = useMemo(() => buildTraceFlowTopology(trace, collapsed), [trace, collapsed]);
  const verificationType = trace.verification.type.replace(/-verification$/, "").replaceAll("-", " ");
  return <FlowCanvas {...navigation} kind="trace" title={trace.verification.name} topology={topology}
    summary={`${verificationType[0]?.toUpperCase()}${verificationType.slice(1)} · ${topology.directCount} directly verified · ${topology.totalCount} requirements in trace`}
    collapsed={collapsed} onExpandAll={() => setCollapsed(new Set())}
    onToggle={id => setCollapsed(previous => {
      const next = new Set(previous);
      if (next.has(id)) next.delete(id); else next.add(id);
      return next;
    })} />;
}

/** The Model adapter supplies store relations; cards and interactions stay shared with Traces. */
export function ElementFlow({ data, ...navigation }: ElementFlowProps) {
  const topology = useMemo<TraceFlowTopology>(() => ({
    nodes: data.nodes.map(node => ({ ...node, id: node.element.id, parentCount: 0 })),
    edges: [...data.edges], totalCount: data.nodes.length,
  }), [data]);
  return <FlowCanvas {...navigation} kind="model" title={data.title} topology={topology}
    summary={`${data.nodes.length} elements · ${data.edges.length} relations`} />;
}

function FlowCanvas({ layoutEngine, topology, title, summary, kind, collapsed, onToggle, onExpandAll, onOpenElement, onOpenSource }: {
  topology: TraceFlowTopology;
  title: string;
  summary: string;
  kind: "trace" | "model";
  collapsed?: ReadonlySet<string>;
  onToggle?: (id: string) => void;
  onExpandAll?: () => void;
} & Omit<TraceFlowProps, "trace">) {
  const [direction, setDirection] = useState<TraceFlowDirection>(kind === "model" ? "DOWN" : "RIGHT");
  const [expanded, setExpanded] = useState(false);
  const [focusedId, setFocusedId] = useState<string | null>(null);
  const [hoveredId, setHoveredId] = useState<string | null>(null);
  const [initializedCanvas, setInitializedCanvas] = useState<{
    generation: object;
    flow: ReactFlowInstance<FlowNode, FlowEdge>;
  } | null>(null);
  const canvas = useRef<HTMLDivElement>(null);
  const [size, setSize] = useState({ width: 0, height: 0 });
  const { graph, pending, failed, retry } = useTraceFlowLayout(topology, direction, layoutEngine);
  // React Flow's deferred onInit can arrive after its canvas is unmounted.
  // A generation distinguishes DOWN → RIGHT → DOWN, even with equal directions.
  const canvasGeneration = useMemo(() => ({}), [graph?.direction]);
  const activeCanvas = useRef<object | null>(null);
  useLayoutEffect(() => {
    activeCanvas.current = canvasGeneration;
    return () => { activeCanvas.current = null; };
  }, [canvasGeneration]);
  const initializeCanvas = useCallback((instance: ReactFlowInstance<FlowNode, FlowEdge>) => {
    if (activeCanvas.current === canvasGeneration) {
      setInitializedCanvas({ generation: canvasGeneration, flow: instance });
    }
  }, [canvasGeneration]);
  const flow = initializedCanvas?.generation === canvasGeneration ? initializedCanvas.flow : null;
  const waitingForCanvas = Boolean(graph && !flow);
  const busy = pending || waitingForCanvas;
  const nodeById = useMemo(() => new Map(graph?.nodes.map(node => [node.id, node])), [graph]);
  useEffect(() => {
    const element = canvas.current;
    if (!element) return;
    const measure = () => {
      const width = element.offsetWidth;
      const height = element.offsetHeight;
      setSize(previous => previous.width === width && previous.height === height ? previous : { width, height });
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(element);
    return () => observer.disconnect();
  }, []);
  const fittedViewport = useMemo(() => graph?.nodes.length && size.width > 0 && size.height > 0
    ? getViewportForBounds(graph.bounds, size.width, size.height, 0, 1, 0.12) : undefined,
  [graph, size]);
  // Retain room to zoom out beyond the complete overview, regardless of graph size.
  const minZoom = fittedViewport ? Math.min(0.2, fittedViewport.zoom / 2) : 0.2;
  const fitFlow = useCallback(() => {
    if (fittedViewport) void flow?.setViewport(fittedViewport);
  }, [flow, fittedViewport]);
  useLayoutEffect(fitFlow, [fitFlow]);
  const activeId = [hoveredId, focusedId].find(id => id && nodeById.has(id)) ?? null;
  const neighborhood = useMemo(() => graph && activeId ? traceFlowNeighborhood(graph.edges, activeId) : null, [graph, activeId]);
  const focusedNode = focusedId ? nodeById.get(focusedId) : undefined;
  // Retain node objects during viewport navigation so React Flow keeps measured handles.
  const nodes = useMemo<FlowNode[]>(() => (graph?.nodes ?? []).map(node => ({
    id: node.id, type: "trace", position: node.position,
    width: TRACE_NODE_WIDTH, height: TRACE_NODE_HEIGHT,
    data: {
      element: node.element, elementType: node.type,
      direction: graph?.direction ?? direction,
      context: node.context,
      parentCount: node.parentCount, collapsed: collapsed?.has(node.id) ?? false, onOpenElement, onOpenSource,
      focused: focusedId === node.id, dimmed: neighborhood ? !neighborhood.nodeIds.has(node.id) : false,
      onPreview: active => setHoveredId(active ? node.id : null),
      onFocusPath: () => setFocusedId(node.id),
      onToggle: () => onToggle?.(node.id),
    },
  })), [graph, direction, collapsed, onOpenElement, onOpenSource, focusedId, neighborhood, onToggle]);
  const edges: FlowEdge[] = (graph?.edges ?? []).map(edge => ({ id: edge.id, source: edge.source, target: edge.target, label: edge.label, type: "trace",
    className: neighborhood ? neighborhood.edgeIds.has(edge.id) ? "ux-trace-edge-highlighted" : "ux-trace-edge-dimmed" : undefined,
    data: { path: edge.path, labelPosition: edge.labelPosition },
    markerEnd: { type: MarkerType.ArrowClosed, color: neighborhood?.edgeIds.has(edge.id) ? "var(--accent)" : "var(--edge-trace)" },
    ariaLabel: `${nodeById.get(edge.source)?.element.name} ${edge.label} ${nodeById.get(edge.target)?.element.name}`,
  }));
  return <ExpandableViewport expanded={expanded} onExpandedChange={setExpanded}
    label={kind === "trace" ? "Expanded verification trace flow" : "Expanded model flow"}>
    <section className={cx(baseUX, skinX)} data-product-pattern="trace-flow" aria-label={kind === "trace" ? "Verification trace flow" : "Model flow"} aria-busy={busy}
    onKeyDown={event => { if (event.key === "Escape" && !expanded) { setFocusedId(null); setHoveredId(null); } }}>
    <header className="ux-trace-flow-header">
      <div className="ux-trace-flow-heading">
        <h2>{title}</h2>
        <p className="ux-trace-flow-summary">{summary}</p>
        {focusedNode && <div className="ux-trace-flow-focus" aria-live="polite">
          <span>{focusedNode.element.name}</span>
          <Button size="sm" tone="link" onClick={() => onOpenElement(focusedNode.id)}>Open details</Button>
          <IconButton size="sm" aria-label="Clear path focus" title="Show full trace" onClick={() => { setFocusedId(null); setHoveredId(null); }}><Icon name="x" /></IconButton>
        </div>}
      </div>
      <div className="ux-trace-flow-tools" role="group" aria-label={kind === "trace" ? "Trace navigation" : "Flow navigation"}>
        <SegmentedControl ariaLabel="Flow direction" value={direction} onChange={setDirection} items={[
          { value: "RIGHT", label: "Left to right" }, { value: "DOWN", label: "Top to bottom" },
        ]} />
        {Boolean(collapsed?.size) && <Button tone="ghost" size="sm" onClick={onExpandAll}>Expand all</Button>}
        <IconButton aria-label="Zoom out" title="Zoom out" disabled={!flow} onClick={() => void flow?.zoomOut()}><Icon name="minus" /></IconButton>
        <IconButton aria-label="Zoom in" title="Zoom in" disabled={!flow} onClick={() => void flow?.zoomIn()}><Icon name="plus" /></IconButton>
        <Button tone="secondary" size="sm" disabled={!flow} onClick={fitFlow}>{kind === "trace" ? "Fit trace" : "Fit flow"}</Button>
        <Button tone="ghost" size="sm" disabled={!flow} onClick={() => void flow?.zoomTo(1)}>100%</Button>
        <IconButton aria-label={expanded ? "Close expanded flow" : "Expand flow"}
          title={expanded ? "Close expanded flow" : "Expand flow"} aria-expanded={expanded}
          data-expanded-close={expanded || undefined} onClick={() => setExpanded(value => !value)}>
          <Icon name={expanded ? "x" : "maximize"} />
        </IconButton>
      </div>
    </header>
    <div className="ux-trace-flow-canvas" ref={canvas}>
      {graph && <ReactFlow key={graph.direction} nodes={nodes} edges={edges} nodeTypes={NODE_TYPES} edgeTypes={EDGE_TYPES} onInit={initializeCanvas}
        onNodeClick={(event, node) => {
          // Register with React Flow so read-only nodes retain browser pointer events.
          // Embedded links and controls handle their own actions.
          if (event.target instanceof Element && !event.target.closest("a, button, [role=button]")) onOpenElement(node.id);
        }}
        defaultViewport={fittedViewport}
        minZoom={minZoom} maxZoom={1.5} nodesDraggable={false} nodesConnectable={false}
        nodesFocusable={false} edgesFocusable elementsSelectable={false}
        zoomOnDoubleClick={false} onPaneClick={() => { setFocusedId(null); setHoveredId(null); }} />}
      {busy && <div className="ux-trace-flow-status" role="status"><Spinner aria-hidden="true" />{graph ? "Updating flow…" : "Loading flow…"}</div>}
      {!pending && !failed && topology.nodes.length === 0 && <div className="ux-trace-flow-status" role="status">No elements in this selection.</div>}
      {failed && <div className="ux-trace-flow-status" role="alert">
        <span>Couldn’t lay out this flow.</span><Button tone="secondary" size="sm" onClick={retry}>Retry</Button>
      </div>}
    </div>
  </section>
  </ExpandableViewport>;
}
