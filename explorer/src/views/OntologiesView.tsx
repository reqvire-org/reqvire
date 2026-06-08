import { useEffect, useRef } from "react";
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
      <div className="graph-route">
        <div className="graph-canvas-wrap">
          <div className="graph-render-notice">Ontology graph data was not exported.</div>
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
      <div className="ontology-page graph-route">
        <section className="ontology-graph-panel graph-canvas-wrap" aria-label="Ontology graph explorer">
          <div className="ontology-graph-canvas">
            <div
              ref={containerRef}
              id="ontology-graph-container"
              role="img"
              aria-label="Ontology and SHACL relationship graph"
            />
          </div>
        </section>
      </div>
    </ViewFrame>
  );
}
