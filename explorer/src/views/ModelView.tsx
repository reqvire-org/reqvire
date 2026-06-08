import { useExplorerUiState, type ModelMode } from "../components/ExplorerUiState";
import { Icon, SegmentedControl } from "@ds";
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
      <div className="ex-route ex-route-single">
        <div className="ex-document-panel ex-browser ex-file-shell ex-model-graph-shell">
          <div className="ex-browser__bar ex-file-toolbar">
            <div className="rq-crumbs ex-file-breadcrumbs" aria-label="Model graph breadcrumbs">
              <span className="rq-crumbs__item ex-file-crumb">
                <button type="button" onClick={() => ui.setModelMode("grid")}>
                  Model
                </button>
              </span>
              <span className="rq-crumbs__item ex-file-crumb">
                <span className="rq-crumbs__sep ex-file-crumb-separator">/</span>
                <button type="button">Graph</button>
              </span>
            </div>
            <div className="ex-file-toolbar-actions">
              <span className="ex-browser__count ex-panel-muted">
                {elementCount} elements
              </span>
              <ModelModeSelector value={ui.modelMode} onChange={ui.setModelMode} />
            </div>
          </div>
          <KnowledgeGraphView embedded frameTestId="model" onOpenElement={onOpenElement} />
        </div>
      </div>
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
