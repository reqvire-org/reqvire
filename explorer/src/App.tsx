import { useEffect, useMemo, useState } from "react";
import { Callout } from "@radix-ui/themes";
import { ChevronLeftIcon, ChevronRightIcon, ExclamationTriangleIcon } from "@radix-ui/react-icons";
import { loadStore } from "./store/loadStore";
import { devFixture } from "./store/devFixture";
import { StoreProvider } from "./store/StoreContext";
import { MissingStoreNotice } from "./components/MissingStoreNotice";
import { HelpModal } from "./components/HelpModal";
import { ElementDetailModal } from "./components/ElementDetailModal";
import { ExplorerSidePane } from "./components/ExplorerSidePane";
import { ExplorerToolRail } from "./components/ExplorerToolRail";
import { ExplorerUiStateProvider } from "./components/ExplorerUiState";
import { useHashRoute } from "./router/useHashRoute";
import { VIEW_TITLES } from "./router/routes";
import { ResourcesView } from "./views/ResourcesView";
import { SearchView } from "./views/SearchView";
import { FilesView } from "./views/FilesView";
import { ModelView } from "./views/ModelView";
import {
  CoverageView,
  TracesView,
} from "./views/ReportViews";
import { OntologiesView } from "./views/OntologiesView";
import { Kn2View, KnowledgeGraphView } from "./views/GraphLibraryViews";

export function App() {
  // Load once: the seed is an immutable generated snapshot for the export.
  const result = useMemo(() => loadStore(devFixture), []);

  if (!result.ok) {
    return <MissingStoreNotice reason={result.reason} detail={result.detail} />;
  }

  return (
    <StoreProvider store={result.store} schemaMismatch={result.schemaMismatch}>
      <ExplorerUiStateProvider>
        <ExplorerShell schemaMismatch={result.schemaMismatch} />
      </ExplorerUiStateProvider>
    </StoreProvider>
  );
}

function ExplorerShell({ schemaMismatch }: { schemaMismatch: string | null }) {
  const { route, navigateView, openElement, closeElement } = useHashRoute();
  const [helpOpen, setHelpOpen] = useState(false);
  const [leftPaneOpen, setLeftPaneOpen] = useState(true);
  const [rightPaneOpen, setRightPaneOpen] = useState(true);
  const hasRightInspector = route.view !== "search";

  // Route changes update the document title to match the active Explorer view.
  useEffect(() => {
    document.title = `Reqvire Explorer — ${VIEW_TITLES[route.view]}`;
  }, [route.view]);

  return (
    <div
      className={[
        "explorer-app-shell",
        leftPaneOpen ? "" : "is-left-collapsed",
        rightPaneOpen ? "" : "is-right-collapsed",
        hasRightInspector ? "has-right-inspector" : "",
      ].join(" ")}
    >
      <ExplorerSidePane
        activeView={route.view}
        open={leftPaneOpen}
        onToggle={() => setLeftPaneOpen((open) => !open)}
        onNavigate={navigateView}
        onOpenElement={openElement}
      />
      <ExplorerToolRail
        onNavigate={navigateView}
        onOpenHelp={() => setHelpOpen(true)}
      />
      {hasRightInspector && (
        <button
          type="button"
          className="explorer-inspector-tab"
          aria-label={rightPaneOpen ? "Collapse inspector pane" : "Expand inspector pane"}
          aria-expanded={rightPaneOpen}
          onClick={() => setRightPaneOpen((open) => !open)}
        >
          <span className="explorer-inspector-tab-label">Inspector</span>
          <span className="explorer-inspector-tab-toggle" aria-hidden="true">
            {rightPaneOpen ? <ChevronRightIcon /> : <ChevronLeftIcon />}
          </span>
        </button>
      )}

      {schemaMismatch && (
        <div className="explorer-schema-warning">
          <Callout.Root color="amber" role="alert" m="2">
            <Callout.Icon>
              <ExclamationTriangleIcon />
            </Callout.Icon>
            <Callout.Text>Store schema mismatch: {schemaMismatch}</Callout.Text>
          </Callout.Root>
        </div>
      )}

      <ActiveView
        view={route.view}
        param={route.param}
        onNavigate={navigateView}
        onOpenElement={openElement}
      />

      <HelpModal open={helpOpen} onOpenChange={setHelpOpen} />

      <ElementDetailModal
        identifier={route.elementId}
        onClose={closeElement}
        onOpenElement={openElement}
      />
    </div>
  );
}

function ActiveView({
  view,
  param,
  onNavigate,
  onOpenElement,
}: {
  view: ReturnType<typeof useHashRoute>["route"]["view"];
  param: string | null;
  onNavigate: (view: ReturnType<typeof useHashRoute>["route"]["view"]) => void;
  onOpenElement: (id: string) => void;
}) {
  switch (view) {
    case "model":
      return <ModelView onOpenElement={onOpenElement} />;
    case "knowledge-graph":
      return <KnowledgeGraphView frameTestId={view} onOpenElement={onOpenElement} />;
    case "kn2":
      return <Kn2View activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "traces":
      return <TracesView activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "ontologies":
      return <OntologiesView activeView={view} onNavigate={onNavigate} />;
    case "coverage":
      return <CoverageView activeView={view} onNavigate={onNavigate} />;
    case "resources":
      return <ResourcesView activeView={view} onNavigate={onNavigate} />;
    case "files":
      return <FilesView path={param} activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "search":
      return <SearchView initialQuery={param} activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    default:
      return <ModelView onOpenElement={onOpenElement} />;
  }
}
