import { css, cx } from "@linaria/atomic";
import { useExplorerUiState, type ModelMode } from "../components/ExplorerUiState";
import { Icon, SegmentedControl } from "@ds";
import { useStore } from "../store/StoreContext";
import { FilesView } from "./FilesView";
import { KnowledgeGraphView } from "./GraphLibraryViews";
import { ViewFrame } from "./ViewFrame";

const modelRouteBaseUX = css`
  box-sizing: border-box;
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-left: var(--ex-current-left-width);
  padding-right: 0;

  &.ex-route-single {
    grid-template-columns: minmax(0, 1fr) !important;
    column-gap: 0;
  }

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }
`;

const modelRouteSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);

  .ex-app & {
    background: var(--bg-canvas);
  }
`;

const modelGraphShellBaseUX = css`
  --ex-model-toolbar-actions-min-w: 280px;
  --ex-model-crumb-max-w: 190px;
  --ex-model-crumb-wide-max-w: 240px;
  position: relative;
  box-sizing: border-box;
  display: flex;
  min-width: 0;
  min-height: 0;
  height: 100%;
  flex-direction: column;
  gap: var(--space-7);
  overflow: hidden;
  padding: var(--space-14) var(--space-16);

  .ex-file-toolbar {
    display: flex;
    min-height: var(--space-24);
    align-items: center;
    justify-content: space-between;
    gap: var(--space-6);
    padding: 0 var(--space-2) var(--space-7);
  }

  .ex-file-toolbar-actions {
    display: flex;
    min-width: min(100%, var(--ex-model-toolbar-actions-min-w));
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-5);
    flex-wrap: wrap;
  }

  .ex-file-breadcrumbs {
    display: flex;
    min-width: 0;
    flex: 1 1 auto;
    align-items: center;
    gap: var(--space-1);
    overflow: hidden;
    font-size: var(--text-sm);
  }

  .ex-file-crumb {
    display: inline-flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-1);
  }

  .ex-file-crumb button {
    max-width: var(--ex-model-crumb-max-w);
    overflow: hidden;
    border: 0;
    background: transparent;
    text-overflow: ellipsis;
    white-space: nowrap;
    cursor: pointer;
  }

  .ex-file-crumb-current span:last-child {
    display: inline-block;
    max-width: var(--ex-model-crumb-wide-max-w);
    overflow: hidden;
    font-weight: var(--weight-medium);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .graph-route {
    flex: 1 1 auto;
    height: auto;
  }

  .graph-canvas-wrap {
    min-height: 0;
  }

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }

  @media (max-width: 900px) {
    .ex-file-toolbar {
      align-items: stretch;
      flex-direction: column;
    }

    .ex-file-toolbar-actions {
      width: 100%;
      min-width: 0;
    }
  }
`;

const modelGraphShellSkinX = css`
  border-left: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  border-right: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  background: var(--bg-surface);
  color: var(--text-body);

  .ex-file-toolbar {
    border-bottom: var(--border-w) solid var(--border-default);
    background: var(--bg-surface);
  }

  .ex-file-breadcrumbs,
  .ex-panel-muted {
    color: var(--text-muted);
  }

  .ex-file-crumb button {
    color: var(--text-body);
  }

  .ex-file-crumb button:hover {
    text-decoration: underline;
  }

  .ex-file-crumb-current span:last-child {
    color: var(--text-strong);
  }

  .ex-file-crumb-separator {
    color: color-mix(in srgb, var(--text-muted) 70%, transparent);
  }

  .ex-panel-muted {
    font-size: var(--text-caption);
    line-height: 1.4;
  }

  .ex-app & {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }
`;

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
      <div className={cx("ex-route", "ex-route-single", modelRouteBaseUX, modelRouteSkinX)}>
        <div
          className={cx(
            "ex-document-panel",
            "ex-browser",
            "ex-file-shell",
            "ex-model-graph-shell",
            modelGraphShellBaseUX,
            modelGraphShellSkinX,
          )}
        >
          <div className={cx("ex-browser__bar", "ex-file-toolbar")}>
            <div className={cx("ex-file-breadcrumbs")} aria-label="Model graph breadcrumbs">
              <span className={cx("ex-file-crumb")}>
                <button type="button" onClick={() => ui.setModelMode("grid")}>
                  Model
                </button>
              </span>
              <span className={cx("ex-file-crumb")}>
                <span className={cx("ex-file-crumb-separator")}>/</span>
                <button type="button">Graph</button>
              </span>
            </div>
            <div className={cx("ex-file-toolbar-actions")}>
              <span className={cx("ex-browser__count", "ex-panel-muted")}>
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
