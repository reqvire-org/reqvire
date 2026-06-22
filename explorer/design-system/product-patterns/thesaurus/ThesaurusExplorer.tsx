import { css, cx } from "@linaria/atomic";
import { useEffect, useMemo, useRef, useState } from "react";
import {
  BaseEdge,
  Handle,
  Position,
  ReactFlow,
  type Edge,
  type EdgeProps,
  type Node,
  type NodeProps,
  type ReactFlowInstance,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { RouteFrame, RouteLayout } from "../shell";
import { WorkspaceShell } from "../content/WorkspaceShell";

export interface ThesaurusConceptUsage {
  id?: string;
  label: string;
  type: string;
}

export interface ThesaurusConceptItem {
  id: string;
  label: string;
  schemeId: string;
  schemeLabel: string;
  schemeSourceElementId?: string | null;
  parentId: string | null;
  depth: number;
  definition: string;
  altLabels: readonly string[];
  scopeNote: string;
  relatedIds: readonly string[];
  usedBy: readonly ThesaurusConceptUsage[];
  mapsTo: readonly ThesaurusConceptUsage[];
  sourceElementId?: string | null;
  sourceHref?: string | null;
  sourceLabel?: string | null;
}

export interface ThesaurusExplorerProps {
  concepts: readonly ThesaurusConceptItem[];
  selectedId: string | null;
  onSelectConcept: (id: string) => void;
  onOpenConcept?: (id: string) => void;
}

const shellBaseUX = css`
  display: block;
  min-height: 0;
  height: 100%;

  .ux-thesaurus-detail {
    min-width: 0;
    min-height: 0;
  }

  .ux-thesaurus-empty {
    padding: var(--space-6) var(--space-4);
    font-size: var(--text-sm);
  }

  .ux-thesaurus-detail {
    display: grid;
    grid-template-rows: minmax(0, 1fr);
    overflow: hidden;
  }

  .ux-thesaurus-title-block {
    display: grid;
    min-width: 0;
    gap: var(--space-3);
    padding-bottom: var(--space-8);
  }

  .ux-thesaurus-title-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .ux-thesaurus-title {
    margin: 0;
    font-size: var(--text-xl);
    line-height: var(--leading-tight);
  }

  .ux-thesaurus-definition {
    max-width: var(--content-max);
    margin: 0;
    font-size: var(--text-base);
    line-height: var(--leading-relaxed);
  }

  .ux-thesaurus-field-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-4);
  }

  .ux-thesaurus-detail-body {
    display: grid;
    grid-template-columns: minmax(0, 0.42fr) minmax(0, 0.58fr);
    min-height: 0;
    gap: var(--space-10);
    overflow: hidden;
  }

  .ux-thesaurus-fields,
  .ux-thesaurus-map {
    min-height: 0;
    overflow: auto;
  }

  .ux-thesaurus-fields {
    display: flex;
    flex-direction: column;
  }

  .ux-thesaurus-field {
    display: grid;
    grid-template-columns: minmax(var(--space-20), 0.32fr) minmax(0, 1fr);
    gap: var(--space-5);
    padding: var(--space-4) 0;
  }

  .ux-thesaurus-field-label {
    font-size: var(--text-caption);
    font-weight: var(--weight-bold);
    text-transform: uppercase;
  }

  .ux-thesaurus-field-value {
    min-width: 0;
    font-size: var(--text-sm);
    line-height: var(--leading-relaxed);
  }

  .ux-thesaurus-chip-row,
  .ux-thesaurus-link-list {
    display: flex;
    min-width: 0;
    flex-wrap: wrap;
    gap: var(--space-3);
  }

  .ux-thesaurus-chip {
    display: inline-flex;
    align-items: center;
    min-height: var(--space-8);
    border-radius: var(--radius-sm);
    padding: 0 var(--space-4);
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
  }

  .ux-thesaurus-link {
    display: inline-flex;
    align-items: center;
    min-width: 0;
    min-height: var(--space-8);
    gap: var(--space-3);
    border: 0;
    border-radius: var(--radius-sm);
    padding: 0 var(--space-3);
    cursor: pointer;
    text-align: left;
  }

  .ux-thesaurus-link-label {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-thesaurus-map {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    align-content: start;
    gap: var(--space-6);
  }

  .ux-thesaurus-map-column {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: var(--space-4);
  }

  .ux-thesaurus-map-label {
    font-size: var(--text-caption);
    font-weight: var(--weight-bold);
    text-transform: uppercase;
  }

  .ux-thesaurus-node {
    display: flex;
    min-width: 0;
    min-height: var(--control-lg);
    align-items: center;
    gap: var(--space-4);
    border-radius: var(--radius-md);
    padding: var(--space-4);
    cursor: pointer;
    text-align: left;
  }

  .ux-thesaurus-node-selected {
    min-height: var(--space-20);
  }

  .ux-thesaurus-node-label {
    min-width: 0;
    overflow: hidden;
    font-size: var(--text-sm);
    font-weight: var(--weight-bold);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-thesaurus-map-empty {
    border-radius: var(--radius-md);
    padding: var(--space-5);
    font-size: var(--text-sm);
  }

  .ux-thesaurus-scheme-map {
    display: grid;
    height: 100%;
    min-height: 0;
    grid-template-rows: minmax(0, 1fr);
    overflow: hidden;
  }

  .ux-thesaurus-scheme-map-header {
    display: flex;
    align-items: end;
    justify-content: space-between;
    gap: var(--space-6);
  }

  .ux-thesaurus-scheme-map-title {
    margin: 0;
    font-size: var(--text-xl);
    line-height: var(--leading-tight);
  }

  .ux-thesaurus-scheme-map-subtitle {
    margin: var(--space-2) 0 0;
    font-size: var(--text-sm);
  }

  .ux-thesaurus-scheme-map-canvas {
    display: block;
    width: 100%;
    min-height: 0;
    height: 100%;
    border-radius: var(--radius-lg);
  }

  .ux-thesaurus-scheme-map-canvas .react-flow {
    width: 100%;
    height: 100%;
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__pane {
    cursor: grab;
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__pane:active {
    cursor: grabbing;
  }

  .ux-thesaurus-flow-node {
    display: grid;
    min-width: 0;
    gap: var(--stack-gap-compact);
    color: var(--text-strong);
    position: relative;
  }

  .ux-thesaurus-flow-node__label {
    overflow: hidden;
    font-size: var(--text-base);
    font-weight: var(--weight-bold);
    line-height: var(--leading-tight);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-thesaurus-flow-node__meta {
    color: var(--text-muted);
    font-size: var(--text-micro);
    font-weight: var(--weight-medium);
    letter-spacing: 0.04em;
    line-height: 1.1;
    text-transform: uppercase;
  }

  .ux-thesaurus-flow-node.is-scheme {
    min-width: var(--thesaurus-map-scheme-node-min-width);
    border: var(--border-w) solid var(--concept);
    border-radius: var(--radius-sm);
    padding: var(--space-4) var(--space-5);
    background: var(--thesaurus-map-scheme-node-bg);
  }

  .ux-thesaurus-flow-node.is-concept {
    min-width: var(--thesaurus-map-concept-node-min-width);
    border: var(--border-w) solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: var(--space-3) var(--space-4);
    background: var(--bg-sunken);
  }

  .ux-thesaurus-flow-node.is-concept::after {
    display: block;
    width: 100%;
    height: var(--border-w-thick);
    margin-top: var(--space-2);
    border-radius: var(--radius-pill);
    background: var(--concept);
    content: "";
  }

  .react-flow__node.branch-0 .ux-thesaurus-flow-node.is-concept::after {
    background: var(--concept);
  }

  .react-flow__node.branch-1 .ux-thesaurus-flow-node.is-concept::after {
    background: var(--semantic-contract);
  }

  .react-flow__node.branch-2 .ux-thesaurus-flow-node.is-concept::after {
    background: var(--ontology);
  }

  .react-flow__node.branch-3 .ux-thesaurus-flow-node.is-concept::after {
    background: var(--verification);
  }

  .react-flow__node.branch-4 .ux-thesaurus-flow-node.is-concept::after {
    background: var(--capability);
  }

  .ux-thesaurus-flow-node.is-selected .ux-thesaurus-flow-node__label {
    color: var(--text-strong);
  }

  .ux-thesaurus-flow-node.is-selected::after {
    background: var(--text-strong);
  }

  .ux-thesaurus-scheme-map-link {
    fill: none;
    stroke-width: 3;
    stroke-linecap: round;
  }

  .ux-thesaurus-scheme-map-related {
    fill: none;
    stroke-width: 2;
    stroke-linecap: round;
    stroke-dasharray: 6 6;
  }

  .ux-thesaurus-scheme-map-node {
    cursor: pointer;
  }

  .ux-thesaurus-scheme-map-node-box {
    rx: var(--radius-md);
    stroke-width: var(--border-w);
  }

  .ux-thesaurus-scheme-map-node-dot {
    stroke-width: var(--border-w);
  }

  .ux-thesaurus-scheme-map-label-line {
    stroke-width: 3;
    stroke-linecap: round;
  }

  .ux-thesaurus-scheme-map-node-label {
    font-size: var(--text-base);
    font-weight: var(--weight-bold);
    dominant-baseline: middle;
    pointer-events: none;
  }

  .ux-thesaurus-scheme-map-node-meta {
    font-size: var(--text-micro);
    font-weight: var(--weight-medium);
    dominant-baseline: middle;
    pointer-events: none;
    text-transform: uppercase;
  }

  @media (max-width: 1100px) {
    .ux-thesaurus-detail-body,
    .ux-thesaurus-map {
      grid-template-columns: minmax(0, 1fr);
    }
  }
`;

const shellSkinX = css`
  color: var(--text-body);

  .ux-thesaurus-empty,
  .ux-thesaurus-definition,
  .ux-thesaurus-field-label,
  .ux-thesaurus-map-label {
    color: var(--text-muted);
  }

  .ux-thesaurus-title {
    color: var(--text-strong);
  }

  .ux-thesaurus-field {
    border-bottom: var(--border-w) solid var(--border-subtle);
  }

  .ux-thesaurus-chip,
  .ux-thesaurus-node,
  .ux-thesaurus-map-empty {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-sunken);
    color: var(--text-body);
  }

  .ux-thesaurus-link {
    background: var(--bg-sunken);
    color: var(--text-body);
  }

  .ux-thesaurus-link:hover,
  .ux-thesaurus-node:hover {
    background: var(--bg-hover);
    color: var(--text-strong);
  }

  .ux-thesaurus-node-selected {
    border-color: var(--concept);
    background: var(--bg-selected);
  }

  .ux-thesaurus-scheme-map-title {
    color: var(--text-strong);
  }

  .ux-thesaurus-scheme-map-subtitle {
    color: var(--text-muted);
  }

  .ux-thesaurus-scheme-map-canvas {
    background: transparent;
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__edge-path {
    stroke: var(--concept);
    stroke-width: var(--border-w-heavy);
    stroke-linecap: round;
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__edge.related .react-flow__edge-path {
    stroke: var(--text-muted);
    stroke-dasharray: 6 6;
    stroke-width: var(--border-w-thick);
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__edge.branch-0 .react-flow__edge-path {
    stroke: var(--concept);
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__edge.branch-1 .react-flow__edge-path {
    stroke: var(--semantic-contract);
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__edge.branch-2 .react-flow__edge-path {
    stroke: var(--ontology);
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__edge.branch-3 .react-flow__edge-path {
    stroke: var(--verification);
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__edge.branch-4 .react-flow__edge-path {
    stroke: var(--capability);
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__edge.related.branch-0 .react-flow__edge-path,
  .ux-thesaurus-scheme-map-canvas .react-flow__edge.related.branch-1 .react-flow__edge-path,
  .ux-thesaurus-scheme-map-canvas .react-flow__edge.related.branch-2 .react-flow__edge-path,
  .ux-thesaurus-scheme-map-canvas .react-flow__edge.related.branch-3 .react-flow__edge-path,
  .ux-thesaurus-scheme-map-canvas .react-flow__edge.related.branch-4 .react-flow__edge-path {
    stroke: var(--text-muted);
  }

  .ux-thesaurus-scheme-map-canvas .react-flow__handle {
    opacity: 0;
  }

  .ux-thesaurus-scheme-map-link {
    stroke: var(--concept);
  }

  .ux-thesaurus-scheme-map-related {
    stroke: var(--text-muted);
  }

  .ux-thesaurus-scheme-map-node-box {
    fill: var(--bg-sunken);
    stroke: var(--concept);
  }

  .ux-thesaurus-scheme-map-node-dot {
    fill: var(--concept);
    stroke: var(--text-strong);
  }

  .ux-thesaurus-scheme-map-node-label {
    fill: var(--text-strong);
  }

  .ux-thesaurus-scheme-map-node-meta {
    fill: var(--text-muted);
  }

  .ux-thesaurus-scheme-map-label-line {
    stroke: var(--concept);
  }

  .ux-thesaurus-scheme-map-node.is-leaf .ux-thesaurus-scheme-map-node-label {
    font-size: var(--text-caption);
    font-weight: var(--weight-medium);
  }

  .ux-thesaurus-scheme-map-node.is-leaf .ux-thesaurus-scheme-map-label-line {
    stroke-width: 2;
  }

  .ux-thesaurus-scheme-map-node.is-selected .ux-thesaurus-scheme-map-node-box {
    fill: var(--bg-selected);
    stroke: var(--concept);
  }

  .ux-thesaurus-scheme-map-node.is-selected .ux-thesaurus-scheme-map-node-label {
    fill: var(--text-strong);
  }

  .ux-thesaurus-scheme-map-node.is-selected .ux-thesaurus-scheme-map-label-line {
    stroke: var(--text-strong);
  }

  .ux-thesaurus-scheme-map-node.is-scheme .ux-thesaurus-scheme-map-node-box {
    fill: var(--bg-surface);
    stroke: var(--concept);
  }
`;

export function ThesaurusExplorer({
  concepts,
  selectedId,
  onSelectConcept,
  onOpenConcept,
}: ThesaurusExplorerProps) {
  const conceptById = new Map(concepts.map((concept) => [concept.id, concept]));
  const selected = selectedId ? conceptById.get(selectedId) : concepts[0];
  const activeSchemeId = selected?.schemeId ?? concepts[0]?.schemeId ?? "";
  const activeSchemeLabel = selected?.schemeLabel ?? concepts[0]?.schemeLabel ?? "Thesaurus";
  const schemeConcepts = concepts.filter((concept) => concept.schemeId === activeSchemeId);

  if (!selected) {
    return (
      <RouteFrame viewId="thesaurus">
        <RouteLayout>
          <WorkspaceShell
            rootLabel="Thesaurus"
            breadcrumbLabel="Thesaurus breadcrumbs"
            tone="canvas"
            showDivider={false}
          >
            <div className={cx("ux-thesaurus", shellBaseUX, shellSkinX)}>
              <div className="ux-thesaurus-empty">No SKOS concepts were exported for this model.</div>
            </div>
          </WorkspaceShell>
        </RouteLayout>
      </RouteFrame>
    );
  }

  return (
    <RouteFrame viewId="thesaurus">
      <RouteLayout>
        <WorkspaceShell
          rootLabel="Thesaurus"
          breadcrumbLabel="Thesaurus breadcrumbs"
          tone="canvas"
          showDivider={false}
        >
          <div className={cx("ux-thesaurus", shellBaseUX, shellSkinX)}>
            <ThesaurusSchemeMap
              concepts={schemeConcepts}
              schemeLabel={activeSchemeLabel}
              selectedId={selected.id}
              onSelect={onSelectConcept}
              onOpenConcept={onOpenConcept}
            />
          </div>
        </WorkspaceShell>
      </RouteLayout>
    </RouteFrame>
  );
}

function ThesaurusSchemeMap({
  concepts,
  schemeLabel,
  selectedId,
  onSelect,
  onOpenConcept,
}: {
  concepts: readonly ThesaurusConceptItem[];
  schemeLabel: string;
  selectedId: string;
  onSelect: (id: string) => void;
  onOpenConcept?: (id: string) => void;
}) {
  const layout = useMemo(
    () => buildThesaurusFlowLayout(concepts, schemeLabel, selectedId),
    [concepts, schemeLabel, selectedId],
  );
  const layoutKey = useMemo(() => layout.nodes.map((node) => node.id).join("|"), [layout.nodes]);
  const [flowInstance, setFlowInstance] = useState<ReactFlowInstance<ThesaurusFlowNode, ThesaurusFlowEdge> | null>(
    null,
  );
  const didFitLayoutRef = useRef<string | null>(null);
  const centeredSelectionRef = useRef<string | null>(null);

  useEffect(() => {
    if (!flowInstance || didFitLayoutRef.current === layoutKey) return;
    didFitLayoutRef.current = layoutKey;
    window.requestAnimationFrame(() => {
      flowInstance.fitView({
        padding: 0.02,
        duration: 0,
      });
    });
  }, [flowInstance, layoutKey]);

  useEffect(() => {
    if (!flowInstance || centeredSelectionRef.current === selectedId) return;
    const selectedNode = flowInstance.getNode(selectedId) ?? layout.nodes.find((node) => node.id === selectedId);
    if (!selectedNode) return;

    centeredSelectionRef.current = selectedId;
    const viewport = flowInstance.getViewport();
    const width = selectedNode.measured?.width ?? selectedNode.width ?? THESAURUS_CONCEPT_NODE_CENTER_WIDTH;
    const height = selectedNode.measured?.height ?? selectedNode.height ?? THESAURUS_CONCEPT_NODE_CENTER_HEIGHT;
    const centerX = selectedNode.position.x + width / 2;
    const centerY = selectedNode.position.y + height / 2;
    window.requestAnimationFrame(() => {
      flowInstance.setCenter(centerX, centerY, {
        zoom: viewport.zoom,
        duration: THESAURUS_SELECTION_CENTER_DURATION,
      });
    });
  }, [flowInstance, layout.nodes, selectedId]);

  return (
    <main className="ux-thesaurus-scheme-map" aria-label="Concept scheme map">
      <div className="ux-thesaurus-scheme-map-canvas" role="img" aria-label={`${schemeLabel} concept map`}>
        <ReactFlow
          nodes={layout.nodes}
          edges={layout.edges}
          nodeTypes={THESAURUS_NODE_TYPES}
          edgeTypes={THESAURUS_EDGE_TYPES}
          nodesDraggable
          nodesConnectable={false}
          elementsSelectable
          proOptions={{ hideAttribution: true }}
          onInit={setFlowInstance}
          onNodeClick={(_event, node) => {
            const sourceElementId = readFlowNodeSourceElementId(node);
            if (sourceElementId) {
              onOpenConcept?.(sourceElementId);
              return;
            }
            if (node.type !== "concept") return;
            onSelect(String(node.id));
          }}
          onNodeDoubleClick={(_event, node) => {
            const sourceElementId = readFlowNodeSourceElementId(node);
            if (sourceElementId) onOpenConcept?.(sourceElementId);
          }}
        />
      </div>
    </main>
  );
}


type ThesaurusFlowNodeData = Record<string, unknown> & {
  label: string;
  meta: string;
  selected: boolean;
  sourceElementId: string | null;
};

type ThesaurusFlowNode = Node<ThesaurusFlowNodeData>;
type ThesaurusFlowEdge = Edge;
type ThesaurusFlowLayout = {
  nodes: ThesaurusFlowNode[];
  edges: ThesaurusFlowEdge[];
};

const THESAURUS_SCHEME_NODE_ID = "__scheme__";
const THESAURUS_NODE_TYPES = {
  scheme: ThesaurusFlowNodeComponent,
  concept: ThesaurusFlowNodeComponent,
};
const THESAURUS_EDGE_TYPES = {
  mindmap: ThesaurusMindMapEdge,
};
const MINDMAP_X_STEP = 240;
const MINDMAP_Y_STEP = 112;
const MINDMAP_SIDE_OFFSET = 56;
const THESAURUS_CONCEPT_NODE_CENTER_WIDTH = 180;
const THESAURUS_CONCEPT_NODE_CENTER_HEIGHT = 44;
const THESAURUS_SELECTION_CENTER_DURATION = 260;

function ThesaurusFlowNodeComponent({ data, type }: NodeProps<ThesaurusFlowNode>) {
  return (
    <div
      className={cx(
        "ux-thesaurus-flow-node",
        type === "scheme" ? "is-scheme" : "is-concept",
        data.selected && "is-selected",
      )}
    >
      <Handle type="target" position={Position.Left} />
      <Handle type="target" position={Position.Right} />
      <div className="ux-thesaurus-flow-node__label">{data.label}</div>
      {data.meta ? <div className="ux-thesaurus-flow-node__meta">{data.meta}</div> : null}
      <Handle type="source" position={Position.Left} />
      <Handle type="source" position={Position.Right} />
    </div>
  );
}

function ThesaurusMindMapEdge({ id, sourceX, sourceY, targetX, targetY }: EdgeProps) {
  const direction = targetX >= sourceX ? 1 : -1;
  const controlOffset = Math.max(Math.abs(targetX - sourceX) * 0.52, MINDMAP_X_STEP * 0.25);
  const path = [
    `M ${sourceX} ${sourceY}`,
    `C ${sourceX + controlOffset * direction} ${sourceY}`,
    `${targetX - controlOffset * direction} ${targetY}`,
    `${targetX} ${targetY}`,
  ].join(" ");
  return <BaseEdge id={id} path={path} />;
}

function buildThesaurusFlowLayout(
  concepts: readonly ThesaurusConceptItem[],
  schemeLabel: string,
  selectedId: string,
): ThesaurusFlowLayout {
  const conceptIds = new Set(concepts.map((concept) => concept.id));
  const conceptById = new Map(concepts.map((concept) => [concept.id, concept]));
  const topConceptIds = concepts
    .filter((concept) => !concept.parentId || !conceptIds.has(concept.parentId))
    .map((concept) => concept.id);
  const branchByTopConcept = new Map(topConceptIds.map((id, index) => [id, `branch-${index % 5}`]));
  const branchDirectionByTopConcept = new Map(topConceptIds.map((id, index) => [id, index % 2 === 0 ? 1 : -1]));
  const schemeSourceElementId = concepts.find((concept) => concept.schemeSourceElementId)?.schemeSourceElementId ?? null;
  const positionById = mapMindMapPositions(concepts, topConceptIds, conceptIds, branchDirectionByTopConcept);
  const nodes: ThesaurusFlowNode[] = [
    {
      id: THESAURUS_SCHEME_NODE_ID,
      type: "scheme",
      position: { x: 0, y: 0 },
      data: {
        label: schemeLabel,
        meta: "Concept scheme",
        selected: false,
        sourceElementId: schemeSourceElementId,
      },
      sourcePosition: Position.Right,
      targetPosition: Position.Left,
    },
  ];
  const edges: ThesaurusFlowEdge[] = [];

  for (const concept of concepts) {
    const isTopConcept = !concept.parentId || !conceptIds.has(concept.parentId);
    const topId = topConceptId(concept, conceptById, conceptIds);
    const branchClass = branchByTopConcept.get(topId) ?? "branch-0";
    const position = positionById.get(concept.id) ?? { x: MINDMAP_X_STEP, y: 0 };
    const direction = position.x >= 0 ? 1 : -1;
    nodes.push({
      id: concept.id,
      type: "concept",
      className: branchClass,
      position,
      data: {
        label: concept.label,
        meta: "",
        selected: concept.id === selectedId,
        sourceElementId: concept.sourceElementId ?? null,
      },
      sourcePosition: direction > 0 ? Position.Right : Position.Left,
      targetPosition: direction > 0 ? Position.Left : Position.Right,
    });
    edges.push({
      id: `${isTopConcept ? THESAURUS_SCHEME_NODE_ID : concept.parentId}-${concept.id}`,
      source: isTopConcept ? THESAURUS_SCHEME_NODE_ID : concept.parentId as string,
      target: concept.id,
      type: "mindmap",
      className: `taxonomy ${branchClass}`,
    });
  }

  return { nodes, edges };
}

function mapMindMapPositions(
  concepts: readonly ThesaurusConceptItem[],
  topConceptIds: readonly string[],
  conceptIds: ReadonlySet<string>,
  branchDirectionByTopConcept: ReadonlyMap<string, number>,
) {
  const childrenByParent = new Map<string, ThesaurusConceptItem[]>();
  const conceptById = new Map(concepts.map((concept) => [concept.id, concept]));
  for (const concept of concepts) {
    const parentId = concept.parentId && conceptIds.has(concept.parentId) ? concept.parentId : THESAURUS_SCHEME_NODE_ID;
    const siblings = childrenByParent.get(parentId) ?? [];
    siblings.push(concept);
    childrenByParent.set(parentId, siblings);
  }
  for (const siblings of childrenByParent.values()) {
    siblings.sort((left, right) => left.label.localeCompare(right.label));
  }

  const positionById = new Map<string, { x: number; y: number }>();
  const placeSide = (rootIds: readonly string[], direction: number) => {
    const totalLeaves = rootIds.reduce((sum, id) => sum + countConceptLeaves(id, childrenByParent), 0);
    const sideOffset = direction > 0 ? -MINDMAP_SIDE_OFFSET : MINDMAP_SIDE_OFFSET;
    let cursor = -((Math.max(totalLeaves, 1) - 1) * MINDMAP_Y_STEP) / 2 + sideOffset;
    for (const rootId of rootIds) {
      const root = conceptById.get(rootId);
      if (!root) continue;
      cursor = placeConceptSubtree(root, direction, 1, cursor, childrenByParent, positionById);
    }
  };
  placeSide(topConceptIds.filter((id) => branchDirectionByTopConcept.get(id) === -1), -1);
  placeSide(topConceptIds.filter((id) => branchDirectionByTopConcept.get(id) !== -1), 1);
  return positionById;
}

function placeConceptSubtree(
  concept: ThesaurusConceptItem,
  direction: number,
  depth: number,
  cursor: number,
  childrenByParent: ReadonlyMap<string, readonly ThesaurusConceptItem[]>,
  positionById: Map<string, { x: number; y: number }>,
) {
  const children = childrenByParent.get(concept.id) ?? [];
  if (children.length === 0) {
    positionById.set(concept.id, { x: direction * depth * MINDMAP_X_STEP, y: cursor });
    return cursor + MINDMAP_Y_STEP;
  }
  const start = cursor;
  let nextCursor = cursor;
  for (const child of children) {
    nextCursor = placeConceptSubtree(child, direction, depth + 1, nextCursor, childrenByParent, positionById);
  }
  const end = nextCursor - MINDMAP_Y_STEP;
  positionById.set(concept.id, { x: direction * depth * MINDMAP_X_STEP, y: (start + end) / 2 });
  return nextCursor;
}

function countConceptLeaves(id: string, childrenByParent: ReadonlyMap<string, readonly ThesaurusConceptItem[]>): number {
  const children = childrenByParent.get(id) ?? [];
  if (children.length === 0) return 1;
  return children.reduce((sum, child) => sum + countConceptLeaves(child.id, childrenByParent), 0);
}

function topConceptId(
  concept: ThesaurusConceptItem,
  conceptById: ReadonlyMap<string, ThesaurusConceptItem>,
  conceptIds: ReadonlySet<string>,
) {
  let current = concept;
  const seen = new Set<string>([concept.id]);
  while (current.parentId && conceptIds.has(current.parentId) && !seen.has(current.parentId)) {
    seen.add(current.parentId);
    current = conceptById.get(current.parentId) ?? current;
  }
  return current.id;
}

function readFlowNodeSourceElementId(node: Node): string | null {
  const value = (node.data as Partial<ThesaurusFlowNodeData> | undefined)?.sourceElementId;
  return typeof value === "string" && value.length > 0 ? value : null;
}
