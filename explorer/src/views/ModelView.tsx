import { useExplorerUiState, type ModelMode } from "../state/ExplorerUiState";
import { Icon, RouteLayout, SegmentedControl, WorkspaceShell } from "@ds";
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

  return (
    <ViewFrame testId="model">
      <RouteLayout>
        <WorkspaceShell
          rootLabel="Model"
          currentLabel="Graph"
          controls={<ModelModeSelector value={ui.modelMode} onChange={ui.setModelMode} />}
          breadcrumbLabel="Model graph breadcrumbs"
          onRootClick={() => ui.setModelMode("grid")}
        >
          <KnowledgeGraphView embedded frameTestId="model" onOpenElement={onOpenElement} />
        </WorkspaceShell>
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
