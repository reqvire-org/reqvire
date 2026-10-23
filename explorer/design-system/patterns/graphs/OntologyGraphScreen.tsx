import { useEffect, useRef, useState } from "preact/hooks";
import { RouteFrame } from "../shell/RouteFrame";
import { GraphCanvasFrame, GraphCanvasNotice, GraphCanvasSurface, GraphRoute } from "../content/GraphCanvasFrame";
import { Button } from "../../components/core/Button";
import { Spinner } from "../../components/core/Spinner";
import { useLatestRef } from "../../hooks/useLatestRef";
import { mountOntologyGraph, type OntologyGraphRendererHandle } from "./ontologyGraphRenderer";
import type { OntologyGraphData, OntologyGraphNode } from "./types";

declare global {
  interface Window {
    filterOntologyGraph?: (query: string) => void;
    focusOntologyNode?: (nodeId: string) => void;
    clearOntologySelection?: () => void;
    fitOntologyGraph?: () => void;
    resetOntologyGraphLayout?: () => void;
    setOntologyGraphFilter?: (category: string, value: string, active: boolean) => void;
    syncOntologyGraphFilters?: (activeValues: string[]) => void;
  }
}

export interface OntologyGraphScreenProps {
  graphData?: OntologyGraphData;
  activeFilters: string[];
  selectedId: string | null;
  onSelectNode: (id: string | null) => void;
  contextKey?: string;
  mountGraph?: typeof mountOntologyGraph;
}

export function OntologyGraphScreen({ graphData, activeFilters, selectedId,
  onSelectNode, contextKey, mountGraph = mountOntologyGraph }: OntologyGraphScreenProps) {
  if (!graphData?.nodes?.length) return <MissingCanonicalOntologyGraph />;
  return <OntologyGraphRenderer key={contextKey} graphData={graphData} activeFilters={activeFilters}
    ontologySelectionId={selectedId} setOntologySelectionId={onSelectNode} mountGraph={mountGraph} />;
}

function MissingCanonicalOntologyGraph() {
  return (
    <RouteFrame viewId="ontologies">
      <GraphRoute>
        <GraphCanvasFrame>
          <GraphCanvasNotice>Ontology graph data was not exported.</GraphCanvasNotice>
        </GraphCanvasFrame>
      </GraphRoute>
    </RouteFrame>
  );
}

function OntologyGraphRenderer({
  graphData,
  activeFilters, ontologySelectionId, setOntologySelectionId, mountGraph,
}: {
  graphData: OntologyGraphData;
  activeFilters: string[];
  ontologySelectionId: string | null;
  setOntologySelectionId: (id: string | null) => void;
  mountGraph: typeof mountOntologyGraph;
}) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<OntologyGraphRendererHandle | null>(null);
  const rendererSelectionRef = useRef<string | null>(null);
  const activeFiltersRef = useRef(activeFilters);
  const selectionRef = useLatestRef(ontologySelectionId);
  const setOntologySelectionIdRef = useLatestRef(setOntologySelectionId);
  const [notice, setNotice] = useState<string | null>("Loading ontology graph...");

  useEffect(() => {
    activeFiltersRef.current = activeFilters;
  }, [activeFilters]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return undefined;
    setNotice("Loading ontology graph...");
    let cancelled = false;
    const mounting = new AbortController();
    let buildTimer: number | null = null;
    const frameId = window.requestAnimationFrame(() => {
      buildTimer = window.setTimeout(async () => {
      try {
        const renderer = await mountGraph(container, graphData, {
          signal: mounting.signal,
          onSelect: (node: OntologyGraphNode | null) => {
            rendererSelectionRef.current = node?.id ?? null;
            setOntologySelectionIdRef.current(node?.id ?? null);
          },
          initialFilters: activeFiltersRef.current,
          onLayoutState: state => setNotice(state === "pending" ? "Laying out ontology..." : state === "failed" ? "Ontology layout failed." : null),
        });
        if (cancelled) { renderer.destroy(); return; }
        rendererRef.current = renderer;
        window.syncOntologyGraphFilters?.(activeFiltersRef.current);
        if (selectionRef.current) renderer.focusNode(selectionRef.current);
      } catch (error) {
        if (cancelled) return;
        console.error("[Reqvire Ontologies] Sigma/Graphology renderer failed", error);
        setNotice("Ontology graph renderer failed. Check the browser console for details.");
      }
      }, 0);
    });
    return () => {
      cancelled = true;
      mounting.abort();
      window.cancelAnimationFrame(frameId);
      if (buildTimer !== null) {
        window.clearTimeout(buildTimer);
        buildTimer = null;
      }
      rendererRef.current?.destroy();
      rendererRef.current = null;
      rendererSelectionRef.current = null;
    };
  }, [graphData, mountGraph, setOntologySelectionIdRef, selectionRef]);

  useEffect(() => {
    window.syncOntologyGraphFilters?.(activeFilters);
  }, [activeFilters]);

  useEffect(() => {
    if (rendererSelectionRef.current === ontologySelectionId) return;
    if (ontologySelectionId) rendererRef.current?.focusNode(ontologySelectionId);
    else rendererRef.current?.clearSelection();
  }, [ontologySelectionId]);

  return (
    <RouteFrame viewId="ontologies">
      <GraphRoute>
        <GraphCanvasFrame aria-label="Ontology graph explorer" aria-busy={notice === "Loading ontology graph..." || notice === "Laying out ontology..."}>
          <GraphCanvasSurface
            ref={containerRef}
            id="ontology-graph-container"
            variant="ontology"
            role="img"
            aria-label="Ontology and SHACL relationship graph"
          />
          {notice ? (
            <GraphCanvasNotice>
              {notice === "Loading ontology graph..." || notice === "Laying out ontology..." ? <Spinner label={notice} /> : null}
              <span>{notice}</span>
              {notice === "Ontology layout failed." ? <Button size="sm" onClick={() => rendererRef.current?.resetLayout()}>Retry layout</Button> : null}
            </GraphCanvasNotice>
          ) : null}
        </GraphCanvasFrame>
      </GraphRoute>
    </RouteFrame>
  );
}
