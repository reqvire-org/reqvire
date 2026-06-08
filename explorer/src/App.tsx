import {
  useEffect,
  useMemo,
  useState,
  type CSSProperties,
  type KeyboardEvent as ReactKeyboardEvent,
  type PointerEvent as ReactPointerEvent,
} from "react";
import { loadStore } from "./store/loadStore";
import { devFixture } from "./store/devFixture";
import { StoreProvider } from "./store/StoreContext";
import { MissingStoreNotice } from "./components/MissingStoreNotice";
import { HelpModal } from "./components/HelpModal";
import { ElementDetailModal } from "./components/ElementDetailModal";
import { OntologyNodeDetailModal } from "./components/OntologyNodeDetailModal";
import { ExplorerSidePane } from "./components/ExplorerSidePane";
import { ExplorerUiStateProvider } from "./components/ExplorerUiState";
import { SearchIndexProvider } from "./components/SearchIndexContext";
import { useHashRoute } from "./router/useHashRoute";
import { VIEW_TITLES, type ViewId } from "./router/routes";
import { ResourcesView } from "./views/ResourcesView";
import { SearchView } from "./views/SearchView";
import { FilesView } from "./views/FilesView";
import { ModelView } from "./views/ModelView";
import {
  CoverageView,
  TracesView,
} from "./views/ReportViews";
import { OntologiesView } from "./views/OntologiesView";
import { ContentView } from "./components/ContentView";
import { useTheme } from "./hooks/useTheme";
import { ReqvireRailMark } from "./components/PaneChrome";
import { Icon, IconButton } from "@ds";

const LEFT_PANE_WIDTH_DEFAULT = 380;
const LEFT_PANE_WIDTH_MIN = 300;
const LEFT_PANE_WIDTH_MAX = 720;
const LEFT_PANE_WIDTH_STORAGE_KEY = "reqvire:explorer:left-pane-width";

export function App() {
  // Load once: the seed is an immutable generated snapshot for the served workspace.
  const result = useMemo(() => loadStore(devFixture), []);

  if (!result.ok) {
    return <MissingStoreNotice reason={result.reason} detail={result.detail} />;
  }

  return (
    <StoreProvider store={result.store} schemaMismatch={result.schemaMismatch}>
      <SearchIndexProvider>
        <ExplorerUiStateProvider>
          <ExplorerShell schemaMismatch={result.schemaMismatch} />
        </ExplorerUiStateProvider>
      </SearchIndexProvider>
    </StoreProvider>
  );
}

function ExplorerShell({ schemaMismatch }: { schemaMismatch: string | null }) {
  const { route, navigateView, openElement, closeElement } = useHashRoute();
  const [helpOpen, setHelpOpen] = useState(false);
  const [leftPaneOpen, setLeftPaneOpen] = useState(true);
  const [leftPaneWidth, setLeftPaneWidth] = useState(readStoredLeftPaneWidth);
  const [leftPaneResizing, setLeftPaneResizing] = useState(false);
  const [ontologyNodeId, setOntologyNodeId] = useState<string | null>(null);
  const { isDark, toggleTheme } = useTheme();
  const sidePaneView =
    route.view === "content" || (route.view === "resources" && route.param)
      ? "model"
      : route.view;

  // Route changes update the document title to match the active Explorer view.
  useEffect(() => {
    document.title = `Reqvire Explorer — ${VIEW_TITLES[route.view]}`;
  }, [route.view]);

  useEffect(() => {
    window.localStorage.setItem(LEFT_PANE_WIDTH_STORAGE_KEY, String(leftPaneWidth));
  }, [leftPaneWidth]);

  useEffect(() => {
    function handleResize() {
      setLeftPaneWidth((width) => clampLeftPaneWidth(width));
    }

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  function toggleLeftPane() {
    setLeftPaneOpen((open) => !open);
  }

  function handleLeftPaneResizePointerDown(event: ReactPointerEvent<HTMLDivElement>) {
    if (!leftPaneOpen || event.button !== 0) return;

    const startX = event.clientX;
    const startWidth = leftPaneWidth;
    setLeftPaneResizing(true);
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";

    function handlePointerMove(moveEvent: PointerEvent) {
      const delta = moveEvent.clientX - startX;
      setLeftPaneResizing(true);
      setLeftPaneWidth(clampLeftPaneWidth(startWidth + delta));
    }

    function finishPointerDrag() {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", finishPointerDrag);
      window.removeEventListener("pointercancel", finishPointerDrag);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
      setLeftPaneResizing(false);
    }

    window.addEventListener("pointermove", handlePointerMove);
    window.addEventListener("pointerup", finishPointerDrag);
    window.addEventListener("pointercancel", finishPointerDrag);
  }

  function handleLeftPaneResizeKeyDown(event: ReactKeyboardEvent<HTMLDivElement>) {
    if (!leftPaneOpen) return;

    if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
      event.preventDefault();
      const direction = event.key === "ArrowLeft" ? -1 : 1;
      const step = event.shiftKey ? 40 : 16;
      setLeftPaneWidth((width) => clampLeftPaneWidth(width + direction * step));
    }
  }

  const shellStyle = {
    "--ex-left-pane-width": `${leftPaneWidth}px`,
  } as CSSProperties;

  return (
    <div
      className={[
        "ex-app",
        "ex-app-shell",
        leftPaneOpen ? "" : "is-left-collapsed",
        leftPaneResizing ? "is-left-resizing" : "",
      ].join(" ")}
      style={shellStyle}
    >
      <ExplorerHeader
        activeView={route.view}
        isDark={isDark}
        onNavigate={navigateView}
        onOpenHelp={() => setHelpOpen(true)}
        onToggleTheme={toggleTheme}
      />
      <div className="ex-main">
        <ExplorerSidePane
          activeView={sidePaneView}
          open={leftPaneOpen}
          onToggle={toggleLeftPane}
          onNavigate={navigateView}
          onOpenElement={openElement}
          onOpenOntologyNode={setOntologyNodeId}
        />
        <button
          type="button"
          className={["ex-collapse", leftPaneOpen ? "" : "is-collapsed"].join(" ")}
          aria-label={leftPaneOpen ? "Collapse explorer" : "Expand explorer"}
          aria-expanded={leftPaneOpen}
          title={leftPaneOpen ? "Collapse explorer" : "Expand explorer"}
          onClick={toggleLeftPane}
        >
          {leftPaneOpen ? <Icon name="chevron-left" /> : <Icon name="chevron-right" />}
        </button>
        <div
          className="ex-pane-resizer"
          role="separator"
          aria-label="Resize explorer pane"
          aria-orientation="vertical"
          aria-valuemin={LEFT_PANE_WIDTH_MIN}
          aria-valuemax={LEFT_PANE_WIDTH_MAX}
          aria-valuenow={leftPaneWidth}
          tabIndex={leftPaneOpen ? 0 : -1}
          onPointerDown={handleLeftPaneResizePointerDown}
          onKeyDown={handleLeftPaneResizeKeyDown}
        />
        <div className="ex-content">
          {schemaMismatch && (
            <div className="ex-schema-warning">
              <div role="alert" className="schema-alert">
                <Icon name="alert-triangle" className="ex-icon-sm" />
                <span>Store schema mismatch: {schemaMismatch}</span>
              </div>
            </div>
          )}

          <ActiveView
            view={route.view}
            param={route.param}
            onNavigate={navigateView}
            onOpenElement={openElement}
          />
        </div>
      </div>

      <HelpModal open={helpOpen} onOpenChange={setHelpOpen} />

      <ElementDetailModal
        identifier={route.elementId}
        onClose={closeElement}
        onOpenElement={openElement}
      />
      <OntologyNodeDetailModal
        nodeId={ontologyNodeId}
        onClose={() => setOntologyNodeId(null)}
      />
    </div>
  );
}

function readStoredLeftPaneWidth() {
  if (typeof window === "undefined") return LEFT_PANE_WIDTH_DEFAULT;

  const stored = Number(window.localStorage.getItem(LEFT_PANE_WIDTH_STORAGE_KEY));
  return clampLeftPaneWidth(Number.isFinite(stored) ? stored : LEFT_PANE_WIDTH_DEFAULT);
}

function clampLeftPaneWidth(width: number) {
  const viewportMax =
    typeof window === "undefined"
      ? LEFT_PANE_WIDTH_MAX
      : Math.max(
          LEFT_PANE_WIDTH_MIN,
          Math.min(LEFT_PANE_WIDTH_MAX, window.innerWidth - 420),
        );

  return Math.round(
    Math.min(Math.max(width, LEFT_PANE_WIDTH_MIN), viewportMax),
  );
}

function ExplorerHeader({
  activeView,
  isDark,
  onNavigate,
  onOpenHelp,
  onToggleTheme,
}: {
  activeView: ViewId;
  isDark: boolean;
  onNavigate: (view: ViewId) => void;
  onOpenHelp: () => void;
  onToggleTheme: () => void;
}) {
  const items: { view: ViewId; label: string; icon: React.ReactNode; badge?: string }[] = [
    { view: "model", label: "Model", icon: <Icon name="folder" /> },
    { view: "ontologies", label: "Ontologies", icon: <Icon name="globe" /> },
    { view: "traces", label: "Traces", icon: <Icon name="activity" /> },
    { view: "coverage", label: "Coverage", icon: <Icon name="pie-chart" /> },
  ];
  const effectiveView = activeView === "files" || activeView === "content" || activeView === "resources"
    ? "model"
    : activeView;

  return (
    <header className="ex-header">
      <div className="ex-brand">
        <ReqvireRailMark className="ex-brand__mark" />
        <span className="ex-brand__name">REQVIRE</span>
      </div>
      <nav className="ex-header__tabs" aria-label="Explorer views">
        <div className="rq-tabs rq-tabs--underline">
          {items.map((item) => {
            const active = item.view === effectiveView;
            return (
              <button
                key={item.view}
                type="button"
                className={["rq-tab", active ? "is-active" : ""].filter(Boolean).join(" ")}
                aria-current={active ? "page" : undefined}
                onClick={() => onNavigate(item.view)}
              >
                {item.icon}
                <span>{item.label}</span>
                {item.badge ? <span className="rq-tab__badge">{item.badge}</span> : null}
              </button>
            );
          })}
        </div>
      </nav>
      <div className="ex-header__actions">
        <IconButton aria-label="Search" onClick={() => onNavigate("search")}>
          <Icon name="search" />
        </IconButton>
        <IconButton
          aria-label={isDark ? "Switch to light mode" : "Switch to dark mode"}
          onClick={onToggleTheme}
        >
          {isDark ? <Icon name="sun" /> : <Icon name="moon" />}
        </IconButton>
        <IconButton aria-label="Help" onClick={onOpenHelp}>
          <Icon name="help-circle" />
        </IconButton>
      </div>
    </header>
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
    case "traces":
      return <TracesView activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "ontologies":
      return <OntologiesView activeView={view} onNavigate={onNavigate} />;
    case "coverage":
      return <CoverageView activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "resources":
      return <ResourcesView resourceId={param} activeView={view} onNavigate={onNavigate} />;
    case "files":
      return <FilesView path={param} activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "content":
      return <ContentView path={param ?? ""} />;
    case "search":
      return <SearchView initialQuery={param} activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    default:
      return <ModelView onOpenElement={onOpenElement} />;
  }
}
