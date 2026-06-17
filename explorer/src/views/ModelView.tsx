import { useExplorerUiState, type ModelMode } from "../state/ExplorerUiState";
import { Icon, ModelGraphShell, RouteLayout, SegmentedControl } from "@ds";
import { useStore } from "../store/StoreContext";
import { FilesView } from "./FilesView";
import { KnowledgeGraphView } from "./GraphLibraryViews";
import { ViewFrame } from "./ViewFrame";

export function ModelView({ onOpenElement }: { onOpenElement: (id: string) => void }) {
  const { modelMode } = useExplorerUiState();

  if (modelMode === "graph") {
    return <ModelGraphView onOpenElement={onOpenElement} />;
  }

  return (
    <FilesView
      path={null}
      forcedLayout={modelMode}
      onOpenElement={onOpenElement}
    />
  );
}

function ModelGraphView({ onOpenElement }: { onOpenElement: (id: string) => void }) {
  const ui = useExplorerUiState();
  const { store } = useStore();
  const summary = store.knowledge_graph.summary;
  const elementCount = summary?.elements ?? store.elements.length;

  return (
    <ViewFrame testId="model">
      <RouteLayout>
        <ModelGraphShell
          rootLabel="Model"
          currentLabel="Graph"
          countLabel={`${elementCount} elements`}
          controls={<ModelModeSelector value={ui.modelMode} onChange={ui.setModelMode} />}
          onRootClick={() => ui.setModelMode("grid")}
        >
          <KnowledgeGraphView embedded frameTestId="model" onOpenElement={onOpenElement} />
        </ModelGraphShell>
      </RouteLayout>
    </ViewFrame>
  );
}

function ModelModeSelector({
  value,
  onChange,
}: {
  value: ModelMode;
  onChange: (mode: ModelMode) => void;
}) {
  return (
    <SegmentedControl<ModelMode>
      ariaLabel="Model layout"
      value={value}
      onChange={onChange}
      items={[
        { value: "list", label: "List", icon: <Icon name="list" /> },
        { value: "grid", label: "Grid", icon: <Icon name="layout-grid" /> },
        { value: "graph", label: "Graph", icon: <Icon name="git-branch" /> },
      ]}
    />
  );
}
