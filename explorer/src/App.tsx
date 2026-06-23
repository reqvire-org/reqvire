import {
  useEffect,
  useMemo,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
  type PointerEvent as ReactPointerEvent,
} from "react";
import { AppShell, type ShellActionItem, type ShellNavigationItem } from "@ds";
import { loadStore } from "./store/loadStore";
import { devFixture } from "./store/devFixture";
import { StoreProvider } from "./store/StoreContext";
import { MissingStoreNotice } from "./components/MissingStoreNotice";
import { HelpModal } from "./components/HelpModal";
import { ElementDetailModal } from "./components/ElementDetailModal";
import { OntologyNodeDetailModal } from "./components/OntologyNodeDetailModal";
import { ExplorerSidePane } from "./components/ExplorerSidePane";
import { ExplorerUiStateProvider } from "./state/ExplorerUiState";
import { SearchIndexProvider } from "./search/SearchIndexContext";
import { useHashRoute } from "./router/useHashRoute";
import { VIEW_TITLES, type ViewId } from "./router/routes";
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
      mainWarning={schemaMismatch ? `Store schema mismatch: ${schemaMismatch}` : null}
      sidePane={
        <ExplorerSidePane
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
        <ActiveView
          view={route.view}
          param={route.param}
          onNavigate={navigateView}
          onOpenElement={handleOpenElement}
        />
      }
    >
      <HelpModal open={helpOpen} onOpenChange={setHelpOpen} />
      <ElementDetailModal
        identifier={route.elementId}
        onClose={handleCloseElementDetail}
        onOpenElement={handleOpenRelatedElement}
        onOpenOntologyNode={setOntologyNodeId}
        onNavigateBack={elementDetailHistory.length > 0 ? handleElementDetailBack : undefined}
        previousElementLabel={elementDetailHistory.at(-1)}
      />
      <OntologyNodeDetailModal
        nodeId={ontologyNodeId}
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
      return <ContentView path={param ?? ""} />;
    case "search":
      return <SearchView initialQuery={param} activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    default:
      return <ModelView onOpenElement={onOpenElement} />;
  }
}
