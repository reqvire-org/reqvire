import { useMemo } from "react";
import { useExplorerUiState, type ModelMode } from "../state/ExplorerUiState";
import { ElementFlow, Icon, RouteLayout, SegmentedControl, WorkspaceShell } from "@ds";
import { useStore } from "../store/StoreContext";
import { flowLayoutEngine } from "../workers/flowLayoutEngine";
import { selectModelFlow } from "../lib/modelFlow";
import { routeForContent, routeForResource } from "../router/routes";
import { FilesView } from "./FilesView";
import { KnowledgeGraphView } from "./GraphLibraryViews";
import { ViewFrame } from "./ViewFrame";

export function ModelView({ onOpenElement }: { onOpenElement: (id: string) => void }) {
  const { modelMode } = useExplorerUiState();

  if (modelMode === "graph") {
    return <ModelGraphView onOpenElement={onOpenElement} />;
  }
  if (modelMode === "flow") return <ModelFlowView onOpenElement={onOpenElement} />;

  return (
    <FilesView
      path={null}
      forcedLayout={modelMode}
      onOpenElement={onOpenElement}
    />
  );
}

function ModelFlowView({ onOpenElement }: { onOpenElement: (id: string) => void }) {
  const ui = useExplorerUiState();
  const { store, elementById, getModelFlowIndex } = useStore();
  const data = useMemo(() => selectModelFlow(getModelFlowIndex(), ui.modelSelectionId, store.project.root_label),
    [getModelFlowIndex, store.project.root_label, ui.modelSelectionId]);
  return <ViewFrame testId="model">
    <RouteLayout>
      <WorkspaceShell rootLabel="Model" currentLabel="Flow" breadcrumbLabel="Model flow breadcrumbs"
        onRootClick={() => ui.setModelSelectionId("__root__")}
        controls={<ModelModeSelector value={ui.modelMode} onChange={ui.setModelMode} />}>
        <ElementFlow layoutEngine={flowLayoutEngine} data={data} onOpenElement={id => {
          if (elementById(id)) onOpenElement(id);
          else window.location.hash = routeForResource(id);
        }} onOpenSource={element => { window.location.hash = element.sourceHref ?? routeForContent(element.file); }} />
      </WorkspaceShell>
    </RouteLayout>
  </ViewFrame>;
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
        { value: "flow", label: "Flow", icon: <Icon name="network" /> },
      ]}
    />
  );
}
