import { useMemo } from "preact/hooks";
import { OntologyGraphScreen } from "@ds";
import { useExplorerUiState } from "../state/ExplorerUiState";
import { useStore } from "../store/StoreContext";
import { mountOntologyGraph } from "../lib/ontologyGraphRenderer";

export function OntologiesView() {
  const { store } = useStore();
  const ui = useExplorerUiState();
  const activeFilters = useMemo(() => [...ui.ontologyFilters], [ui.ontologyFilters]);
  return <OntologyGraphScreen graphData={store.ontology?.graph_data} activeFilters={activeFilters}
    contextKey={JSON.stringify([store.project.workspace_root, store.project.worktree_id])}
    selectedId={ui.ontologySelectionId} onSelectNode={ui.setOntologySelectionId} mountGraph={mountOntologyGraph} />;
}
