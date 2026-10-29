import { useMemo, useState } from "preact/hooks";
import { useExplorerUiState } from "../state/ExplorerUiState";
import { Icon, ModelFlowScreen, ModelGraphScreen, SegmentedControl } from "@ds";
import { useStore } from "../store/StoreContext";
import { flowLayoutEngine } from "../workers/flowLayoutEngine";
import { selectModelFlow, type ModelFlowScope } from "../lib/modelFlow";
import { routeForContent, routeForResource } from "../router/routes";
import { FilesView } from "./FilesView";
import { KnowledgeGraphView } from "./GraphLibraryViews";

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
  const [scope, setScope] = useState<ModelFlowScope>("downstream");
  const data = useMemo(() => selectModelFlow(getModelFlowIndex(), ui.modelSelectionId, store.project.root_label, scope),
    [getModelFlowIndex, store.project.root_label, ui.modelSelectionId, scope]);
  return <ModelFlowScreen data={data} layoutEngine={flowLayoutEngine} mode={ui.modelMode}
    controls={ui.modelSelectionId !== "__root__" && <SegmentedControl<ModelFlowScope> ariaLabel="Flow scope" value={scope} onChange={setScope}
      items={[{ value: "downstream", icon: <Icon name="chevron-down" />, tooltip: "Downstream" },
        { value: "upstream", icon: <Icon name="chevron-up" />, tooltip: "Upstream" }]} />}
    onModeChange={ui.setModelMode} onRootClick={() => ui.setModelSelectionId("__root__")}
    onOpenElement={id => {
      if (elementById(id)) onOpenElement(id);
      else window.location.hash = routeForResource(id);
    }} onOpenSource={element => { window.location.hash = element.sourceHref ?? routeForContent(element.file); }} />;
}

function ModelGraphView({ onOpenElement }: { onOpenElement: (id: string) => void }) {
  const ui = useExplorerUiState();
  return <ModelGraphScreen mode={ui.modelMode} onModeChange={ui.setModelMode}
    onRootClick={() => ui.setModelMode("grid")}
    graph={<KnowledgeGraphView embedded frameTestId="model" onOpenElement={onOpenElement} />} />;
}
