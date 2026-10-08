import { useEffect, useMemo, useRef, useState } from "react";
import { useExplorerUiState } from "../state/ExplorerUiState";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import { mountOntologyGraph, type OntologyGraphRendererHandle } from "../lib/ontologyGraphRenderer";
import { useStore } from "../store/StoreContext";
import type { OntologyGraphData, OntologyGraphNode } from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { Button, GraphCanvasFrame, GraphCanvasNotice, GraphCanvasSurface, GraphRoute, Spinner, useLatestRef } from "@ds";

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

export function OntologiesView(_: Partial<ExplorerViewProps> = {}) {
  const { store } = useStore();
  const ui = useExplorerUiState();
  const graphData = store.ontology?.graph_data;
  const contextKey = JSON.stringify([store.project.workspace_root, store.project.worktree_id]);
  const activeFilters = useMemo(() => [...ui.ontologyFilters], [ui.ontologyFilters]);

  if (graphData && (graphData.nodes?.length ?? 0) > 0) {
    return (
      <OntologyGraphRenderer
        key={contextKey}
        graphData={graphData}
        activeFilters={activeFilters}
      />
    );
  }

  return <MissingCanonicalOntologyGraph />;
}

function MissingCanonicalOntologyGraph() {
  return (
    <ViewFrame testId="ontologies">
      <GraphRoute>
        <GraphCanvasFrame>
          <GraphCanvasNotice>Ontology graph data was not exported.</GraphCanvasNotice>
        </GraphCanvasFrame>
      </GraphRoute>
    </ViewFrame>
  );
}

function OntologyGraphRenderer({
  graphData,
  activeFilters,
}: {
  graphData: OntologyGraphData;
  activeFilters: string[];
}) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const rendererRef = useRef<OntologyGraphRendererHandle | null>(null);
  const rendererSelectionRef = useRef<string | null>(null);
  const activeFiltersRef = useRef(activeFilters);
  const { ontologySelectionId, setOntologySelectionId } = useExplorerUiState();
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
    let buildTimer: number | null = null;
    const frameId = window.requestAnimationFrame(() => {
      buildTimer = window.setTimeout(() => {
      try {
        const renderer = mountOntologyGraph(container, graphData, {
          onSelect: (node: OntologyGraphNode | null) => {
            rendererSelectionRef.current = node?.id ?? null;
            setOntologySelectionIdRef.current(node?.id ?? null);
          },
          initialFilters: activeFiltersRef.current,
          onLayoutState: state => setNotice(state === "pending" ? "Laying out ontology..." : state === "failed" ? "Ontology layout failed." : null),
        });
        rendererRef.current = renderer;
        window.syncOntologyGraphFilters?.(activeFiltersRef.current);
        if (selectionRef.current) renderer.focusNode(selectionRef.current);
      } catch (error) {
        console.error("[Reqvire Ontologies] Sigma/Graphology renderer failed", error);
        setNotice("Ontology graph renderer failed. Check the browser console for details.");
      }
      }, 0);
    });
    return () => {
      window.cancelAnimationFrame(frameId);
      if (buildTimer !== null) {
        window.clearTimeout(buildTimer);
        buildTimer = null;
      }
      rendererRef.current?.destroy();
      rendererRef.current = null;
      rendererSelectionRef.current = null;
    };
  }, [graphData, setOntologySelectionIdRef, selectionRef]);

  useEffect(() => {
    window.syncOntologyGraphFilters?.(activeFilters);
  }, [activeFilters]);

  useEffect(() => {
    if (rendererSelectionRef.current === ontologySelectionId) return;
    if (ontologySelectionId) rendererRef.current?.focusNode(ontologySelectionId);
    else rendererRef.current?.clearSelection();
  }, [ontologySelectionId]);

  return (
    <ViewFrame testId="ontologies">
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
    </ViewFrame>
  );
}
