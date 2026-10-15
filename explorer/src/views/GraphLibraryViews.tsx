import { KnowledgeGraphCanvas } from "@ds";
import { useStore } from "../store/StoreContext";
import { useExplorerUiState } from "../state/ExplorerUiState";
import { ForceAtlasLayoutOwner } from "../workers/forceAtlasLayoutOwner";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";

const createLayoutOwner = () => new ForceAtlasLayoutOwner();

export function KnowledgeGraphView({ frameTestId = "model-graph", embedded = false, onOpenElement }: {
  frameTestId?: string; embedded?: boolean; onOpenElement?: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store } = useStore();
  const ui = useExplorerUiState();
  return <KnowledgeGraphCanvas graphData={store.knowledge_graph}
    contextKey={JSON.stringify([store.project.workspace_root, store.project.worktree_id])}
    activeTypes={ui.modelTypes} activeOverlays={ui.modelOverlays}
    selectedId={ui.knowledgeGraphSelectionId} onSelectNode={ui.setKnowledgeGraphSelectionId}
    frameTestId={frameTestId} embedded={embedded} onOpenElement={onOpenElement} createLayoutOwner={createLayoutOwner} />;
}
