import {
  useEffect,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
  type PointerEvent as ReactPointerEvent,
  type ReactNode,
} from "react";
import { AppShell, WorktreeSelector, type ShellActionItem, type ShellNavigationItem } from "@ds";
import { WorktreeUrlContext } from "./store/worktreeUrls";
import { useLiveStore } from "./store/useLiveStore";
import { StoreProvider } from "./store/StoreContext";
import { MissingStoreNotice } from "./components/MissingStoreNotice";
import { HelpModal } from "./components/HelpModal";
import { ElementDetailModal } from "./components/ElementDetailModal";
import { OntologyNodeDetailModal } from "./components/OntologyNodeDetailModal";
import { ExplorerSidePane } from "./components/ExplorerSidePane";
import { ExplorerUiStateProvider, useExplorerUiState } from "./state/ExplorerUiState";
import { SearchIndexProvider } from "./search/SearchIndexContext";
import { useHashRoute } from "./router/useHashRoute";
import { VIEW_TITLES, routeForContent, type ViewId } from "./router/routes";
import { ResourcesView } from "./views/ResourcesView";
import { SearchView } from "./views/SearchView";
import { FilesView } from "./views/FilesView";
import { ModelView } from "./views/ModelView";
import { ThesaurusView } from "./views/ThesaurusView";
import {
  CoverageView,
  TracesView,
} from "./views/ReportViews";
import { OntologiesView } from "./views/OntologiesView";
import { ContentView } from "./components/ContentView";
import { useTheme } from "./hooks/useTheme";

const LEFT_PANE_WIDTH_DEFAULT = 380;
const LEFT_PANE_WIDTH_MIN = 300;
const LEFT_PANE_WIDTH_MAX = 720;
const LEFT_PANE_WIDTH_STORAGE_KEY = "reqvire:explorer:left-pane-width";

const SHELL_NAVIGATION_ITEMS: ShellNavigationItem[] = [
  { value: "thesaurus", label: "Thesaurus", icon: "tags" },
  { value: "model", label: "Model", icon: "folder" },
  { value: "ontologies", label: "Ontologies", icon: "globe" },
  { value: "traces", label: "Traces", icon: "activity" },
  { value: "coverage", label: "Coverage", icon: "pie-chart" },
];

/** Route composition slots let the showcase use the application shell and router. */
export interface ExplorerViewSlots {
  main: (props: { onOpenElement: (id: string) => void; onOpenSource: (file: string) => void }) => ReactNode;
  sidePane: (props: { open: boolean; onToggle: () => void }) => ReactNode;
}

interface AppProps {
  viewOverrides?: Partial<Record<ViewId, ExplorerViewSlots>>;
}

export function App({ viewOverrides }: AppProps = {}) {
  const live = useLiveStore();
  return <ExplorerApplication live={live} viewOverrides={viewOverrides} />;
}

/** Shared application composition; the showcase supplies fixture snapshots in place of HTTP. */
export function ExplorerApplication({ live, viewOverrides }: AppProps & { live: ReturnType<typeof useLiveStore> }) {
  const { result, refreshError } = live;
  const toolbar = live.worktreeRouting ? <WorktreeSelector
    value={live.selectedWorktree}
    displayedValue={result.ok ? result.store.project.worktree_id : undefined}
    choices={live.worktrees.map(item => ({ id: item.worktree_id, branch: item.branch, root: item.workspace_root, available: item.available && (item.explorer_available || item.owned === false) }))}
    branch={result.ok ? result.store.project.branch : undefined}
    pending={live.switching && !refreshError}
    onChange={live.selectWorktree}
    onOpen={() => { void live.refreshWorktrees(); }}
  /> : undefined;

  if (!result.ok) {
    return <>{toolbar}<MissingStoreNotice reason={result.reason} detail={refreshError ?? result.detail} /></>;
  }

  return (
    <WorktreeUrlContext.Provider value={result.store.project.worktree_id}>
    <StoreProvider store={result.store} schemaMismatch={result.schemaMismatch}>
      <SearchIndexProvider>
        <ExplorerUiStateProvider>
          <ExplorerShell
            viewOverrides={viewOverrides}
            schemaMismatch={result.schemaMismatch}
            refreshError={refreshError}
            automaticRefresh={live.automaticRefresh}
            toolbar={toolbar}
            worktreeId={result.store.project.worktree_id}
          />
        </ExplorerUiStateProvider>
      </SearchIndexProvider>
    </StoreProvider>
    </WorktreeUrlContext.Provider>
  );
}

function ExplorerShell({ schemaMismatch, refreshError, automaticRefresh, viewOverrides, toolbar, worktreeId }: AppProps & {
  schemaMismatch: string | null;
  refreshError: string | null;
  automaticRefresh: boolean;
  toolbar?: ReactNode;
  worktreeId?: string;
}) {
  const { route, navigateView, openElement, closeElement } = useHashRoute();
  const { navigationNotice } = useExplorerUiState();
  const previousContext = useRef(worktreeId);
  const contextChanged = previousContext.current !== worktreeId;
  useEffect(() => {
    if (previousContext.current === worktreeId) return;
    previousContext.current = worktreeId;
    if (route.elementId) closeElement(true);
    setElementDetailHistory([]);
    setOntologyNodeId(null);
  }, [worktreeId, route.elementId, closeElement]);
  const viewOverride = viewOverrides?.[route.view];
  const [helpOpen, setHelpOpen] = useState(false);
  const [leftPaneOpen, setLeftPaneOpen] = useState(true);
  const [leftPaneResizing, setLeftPaneResizing] = useState(false);
  const [leftPaneWidth, setLeftPaneWidth] = useState(readStoredLeftPaneWidth);
  const [elementDetailHistory, setElementDetailHistory] = useState<string[]>([]);
  const [ontologyNodeId, setOntologyNodeId] = useState<string | null>(null);
  const shellRef = useRef<HTMLDivElement | null>(null);
  const leftPaneWidthRef = useRef(leftPaneWidth);
  const { isDark, toggleTheme } = useTheme();
  const sidePaneView =
    route.view === "content" || (route.view === "resources" && route.param)
      ? "model"
      : route.view;
  const effectiveHeaderView: ViewId =
    route.view === "files" || route.view === "content" || route.view === "resources"
      ? "model"
      : route.view;

  // Route changes update the document title to match the active Explorer view.
  useEffect(() => {
    document.title = `Reqvire Explorer — ${VIEW_TITLES[route.view]}`;
  }, [route.view]);

  useEffect(() => {
    if (!route.elementId) setElementDetailHistory([]);
  }, [route.elementId]);

  useEffect(() => {
    leftPaneWidthRef.current = leftPaneWidth;
    shellRef.current?.style.setProperty("--ux-left-pane-width", `${leftPaneWidth}px`);
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

  function handleShellNavigate(value: string) {
    navigateView(value as ViewId);
  }

  function handleOpenElement(identifier: string) {
    setElementDetailHistory([]);
    openElement(identifier);
  }

  function handleOpenRelatedElement(identifier: string) {
    if (identifier === route.elementId) return;
    setElementDetailHistory((history) => (route.elementId ? [...history, route.elementId] : history));
    openElement(identifier);
  }

  function handleElementDetailBack() {
    const previous = elementDetailHistory.at(-1);
    if (!previous) return;
    setElementDetailHistory((history) => history.slice(0, -1));
    openElement(previous);
  }

  function handleCloseElementDetail() {
    setElementDetailHistory([]);
    closeElement();
  }

  function handleLeftPaneResizePointerDown(event: ReactPointerEvent<HTMLDivElement>) {
    if (!leftPaneOpen || event.button !== 0) return;

    const startX = event.clientX;
    const startWidth = leftPaneWidthRef.current;
    let nextWidth = startWidth;
    setLeftPaneResizing(true);
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";

    function handlePointerMove(moveEvent: PointerEvent) {
      const delta = moveEvent.clientX - startX;
      nextWidth = clampLeftPaneWidth(startWidth + delta);
      shellRef.current?.style.setProperty("--ux-left-pane-width", `${nextWidth}px`);
    }

    function finishPointerDrag() {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", finishPointerDrag);
      window.removeEventListener("pointercancel", finishPointerDrag);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
      setLeftPaneResizing(false);
      leftPaneWidthRef.current = nextWidth;
      setLeftPaneWidth(nextWidth);
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

  const headerActions: ShellActionItem[] = [
    {
      id: "search",
      label: "Search",
      icon: "search",
      onClick: () => navigateView("search"),
    },
    {
      id: "theme",
      label: isDark ? "Switch to light mode" : "Switch to dark mode",
      icon: isDark ? "sun" : "moon",
      onClick: toggleTheme,
    },
    {
      id: "help",
      label: "Help",
      icon: "help-circle",
      onClick: () => setHelpOpen(true),
    },
  ];

  return (
    <AppShell
      ref={shellRef}
      navigationItems={SHELL_NAVIGATION_ITEMS}
      activeNavigationValue={effectiveHeaderView}
      headerActions={headerActions}
      sidePaneHeader={toolbar}
      leftPaneOpen={leftPaneOpen}
      leftPaneResizing={leftPaneResizing}
      leftPaneWidth={leftPaneWidth}
      leftPaneMinWidth={LEFT_PANE_WIDTH_MIN}
      leftPaneMaxWidth={LEFT_PANE_WIDTH_MAX}
      leftPaneCollapseLabel="Collapse explorer"
      leftPaneExpandLabel="Expand explorer"
      leftPaneResizeLabel="Resize explorer pane"
      onNavigate={handleShellNavigate}
      onToggleLeftPane={toggleLeftPane}
      onLeftPaneResizePointerDown={handleLeftPaneResizePointerDown}
      onLeftPaneResizeKeyDown={handleLeftPaneResizeKeyDown}
      mainWarning={refreshError
        ? `Refresh failed: ${refreshError}. Keeping the last valid view. ${automaticRefresh ? "Will retry automatically." : "Select a branch to retry."}`
        : schemaMismatch ? `Store schema mismatch: ${schemaMismatch}` : navigationNotice}
      sidePane={
        viewOverride ? viewOverride.sidePane({ open: leftPaneOpen, onToggle: toggleLeftPane }) : <ExplorerSidePane
          activeView={sidePaneView}
          open={leftPaneOpen}
          chrome="app"
          onToggle={toggleLeftPane}
          onNavigate={navigateView}
          onOpenElement={handleOpenElement}
          sourceBrowsing={route.view === "content"}
          onOpenSourceRoute={(hash) => {
            window.location.hash = hash;
          }}
          onOpenOntologyNode={setOntologyNodeId}
        />
      }
      main={
        viewOverride ? viewOverride.main({
          onOpenElement: handleOpenElement,
          onOpenSource: (file) => { window.location.hash = routeForContent(file); },
        }) : <ActiveView
          view={route.view}
          param={route.param}
          onNavigate={navigateView}
          onOpenElement={handleOpenElement}
        />
      }
    >
      <HelpModal open={helpOpen} onOpenChange={setHelpOpen} />
      <ElementDetailModal
        identifier={contextChanged ? null : route.elementId}
        onClose={handleCloseElementDetail}
        onOpenElement={handleOpenRelatedElement}
        onOpenOntologyNode={setOntologyNodeId}
        onNavigateBack={elementDetailHistory.length > 0 ? handleElementDetailBack : undefined}
        previousElementLabel={elementDetailHistory.at(-1)}
      />
      <OntologyNodeDetailModal
        nodeId={contextChanged ? null : ontologyNodeId}
        onClose={() => setOntologyNodeId(null)}
      />
    </AppShell>
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
    case "thesaurus":
      return <ThesaurusView onOpenElement={onOpenElement} />;
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
      return <ContentView path={param ?? ""} onOpenElement={onOpenElement} />;
    case "search":
      return <SearchView initialQuery={param} activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    default:
      return <ModelView onOpenElement={onOpenElement} />;
  }
}
