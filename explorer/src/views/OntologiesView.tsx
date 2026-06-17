import { useEffect, useRef } from "react";
import { useExplorerUiState } from "../state/ExplorerUiState";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import { mountOntologyGraph, type OntologyGraphRendererHandle } from "../lib/ontologyGraphRenderer";
import { useStore } from "../store/StoreContext";
import type { OntologyGraphData, OntologyGraphNode } from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { GraphCanvasFrame, GraphCanvasNotice, GraphCanvasSurface, GraphRoute } from "@ds";

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
      <GraphRoute>
        <GraphCanvasFrame aria-label="Ontology graph explorer">
          <GraphCanvasSurface
            ref={containerRef}
            id="ontology-graph-container"
            variant="ontology"
            role="img"
            aria-label="Ontology and SHACL relationship graph"
          />
        </GraphCanvasFrame>
      </GraphRoute>
    </ViewFrame>
  );
}
