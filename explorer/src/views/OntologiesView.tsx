import { useEffect, useRef } from "react";
import { css, cx } from "@linaria/atomic";
import { useExplorerUiState } from "../components/ExplorerUiState";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { mountOntologyGraph, type OntologyGraphRendererHandle } from "../lib/ontologyGraphRenderer";
import { useStore } from "../store/StoreContext";
import type { OntologyGraphData, OntologyGraphNode } from "../store/types";
import { ViewFrame } from "./ViewFrame";

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

const ontologyGraphCanvasUX = css`
  position: relative;
  display: block;
  width: 100%;
  height: 100%;
  min-height: 0;
`;

const ontologyGraphCanvasSkinX = css`
  background: var(--bg-canvas);
`;

const ontologyGraphContainerUX = css`
  display: block;
  width: 100%;
  height: 100%;
`;

export function OntologiesView(_: Partial<ExplorerViewProps> = {}) {
  const { store } = useStore();
  const ui = useExplorerUiState();
  const graphData = store.ontology?.graph_data;

  if (graphData && (graphData.nodes?.length ?? 0) > 0) {
    return (
      <OntologyGraphRenderer
        graphData={graphData}
        activeFilters={[...ui.ontologyFilters]}
      />
    );
  }

  return <MissingCanonicalOntologyGraph />;
}

function MissingCanonicalOntologyGraph() {
  return (
    <ViewFrame testId="ontologies">
      <div className={cx("graph-route", graphRouteUX, graphRouteSkinX)}>
        <div className={cx("graph-canvas-wrap", graphCanvasWrapUX, graphCanvasWrapSkinX)}>
          <div className={cx("graph-render-notice", graphNoticeUX, graphNoticeSkinX)}>Ontology graph data was not exported.</div>
        </div>
      </div>
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
  const { setOntologySelectionId } = useExplorerUiState();

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return undefined;
    const renderer = mountOntologyGraph(container, graphData, {
      onSelect: (node: OntologyGraphNode | null) => setOntologySelectionId(node?.id ?? null),
    });
    rendererRef.current = renderer;
    return () => {
      renderer.destroy();
      rendererRef.current = null;
    };
  }, [graphData, setOntologySelectionId]);

  useEffect(() => {
    window.syncOntologyGraphFilters?.(activeFilters);
  }, [activeFilters]);

  return (
    <ViewFrame testId="ontologies">
      <div className={cx("ontology-page", "graph-route", graphRouteUX, graphRouteSkinX)}>
        <section className={cx("ontology-graph-panel", "graph-canvas-wrap", graphCanvasWrapUX, graphCanvasWrapSkinX)} aria-label="Ontology graph explorer">
          <div className={cx("ontology-graph-canvas", ontologyGraphCanvasUX, ontologyGraphCanvasSkinX)}>
            <div
              ref={containerRef}
              id="ontology-graph-container"
              className={cx("ex-ontology-graph-container", ontologyGraphContainerUX)}
              role="img"
              aria-label="Ontology and SHACL relationship graph"
            />
          </div>
        </section>
      </div>
    </ViewFrame>
  );
}
