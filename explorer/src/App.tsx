import { useEffect, useRef, useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { Toaster, ExplorerWorkspace, useExplorerPane, WorktreeLoadDialog, type WorktreeSelectorProps } from "@ds";
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
import { useStore } from "./store/StoreContext";
import { prepareThesaurus } from "./lib/thesaurus";

/** Route composition slots let the application test harness use the application shell and router. */
export interface ExplorerViewSlots {
  main: (props: { onOpenElement: (id: string) => void; onOpenSource: (file: string) => void }) => ComponentChildren;
  sidePane: (props: { open: boolean; onToggle: () => void }) => ComponentChildren;
}

interface AppProps {
  viewOverrides?: Partial<Record<ViewId, ExplorerViewSlots>>;
}

export function App({ viewOverrides }: AppProps = {}) {
  const live = useLiveStore();
  return <><Toaster /><ExplorerApplication live={live} viewOverrides={viewOverrides} /></>;
}

/** Shared application composition; the application test harness supplies fixture snapshots in place of HTTP. */
export function ExplorerApplication({ live, viewOverrides }: AppProps & { live: ReturnType<typeof useLiveStore> }) {
  const { result, refreshError } = live;
  const displayedWorktree = result.ok ? result.store.project.worktree_id : undefined;
  const [dismissedRefresh, setDismissedRefresh] = useState<{ context?: string; error: string } | null>(null);
  useEffect(() => {
    if (!refreshError || dismissedRefresh?.context !== displayedWorktree) setDismissedRefresh(null);
  }, [refreshError, displayedWorktree, dismissedRefresh?.context]);
  const selectingWorktree = Boolean(live.switching || live.worktreeSelectionError);
  const showRefreshError = Boolean(refreshError && !selectingWorktree
    && !(dismissedRefresh?.context === displayedWorktree && dismissedRefresh?.error === refreshError));
  const blocked = selectingWorktree || showRefreshError;
  const worktreeSelector: WorktreeSelectorProps | undefined = live.worktreeRouting ? {
    value: live.selectedWorktree,
    displayedValue: result.ok ? result.store.project.worktree_id : undefined,
    choices: live.worktrees.map(item => ({ id: item.worktree_id, branch: item.branch,
      root: item.workspace_root, available: item.available && (item.explorer_available || item.owned === false) })),
    branch: result.ok ? result.store.project.branch : undefined,
    pending: live.switching && !refreshError,
    onChange: live.selectWorktree,
    onOpen: () => { void live.refreshWorktrees(); },
  } : undefined;

  if (!result.ok) {
    return <ExplorerWorkspace worktree={worktreeSelector}
      main={<MissingStoreNotice reason={result.reason} detail={refreshError ?? result.detail} />} />;
  }

  return (
    <WorktreeUrlContext.Provider value={result.store.project.worktree_id}>
    <StoreProvider store={result.store} schemaMismatch={result.schemaMismatch}>
      <SearchIndexProvider>
        <ExplorerUiStateProvider worktreeRouting={live.worktreeRouting}>
          {(live.switching || live.worktreeSelectionError) && <WorktreeLoadDialog
            branch={live.worktrees.find(item => item.worktree_id === live.selectedWorktree)?.branch ?? live.selectedWorktree ?? "Selected worktree"}
            error={live.worktreeSelectionError} onDismiss={live.dismissWorktreeError} /> }
          {showRefreshError && <WorktreeLoadDialog operation="refresh"
            branch={result.store.project.branch ?? "Displayed model"} error={refreshError}
            retryAutomatically={live.automaticRefresh}
            onDismiss={() => { if (refreshError) setDismissedRefresh({ context: displayedWorktree, error: refreshError }); }} />}
          <ExplorerShell
            viewOverrides={viewOverrides}
            schemaMismatch={result.schemaMismatch}
            recoveryWarning={live.recoveryWarning}
            worktreeSelector={worktreeSelector}
            worktreeBlocked={blocked}
            worktreeId={result.store.project.worktree_id}
            onRefreshHead={live.refreshHead}
            refreshingHead={live.refreshingHead}
          />
        </ExplorerUiStateProvider>
      </SearchIndexProvider>
    </StoreProvider>
    </WorktreeUrlContext.Provider>
  );
}

function ExplorerShell({ schemaMismatch, recoveryWarning, viewOverrides, worktreeSelector, worktreeId, worktreeBlocked, onRefreshHead, refreshingHead }: AppProps & {
  recoveryWarning?: string | null;
  schemaMismatch: string | null;
  worktreeSelector?: WorktreeSelectorProps;
  worktreeId?: string;
  worktreeBlocked?: boolean;
  onRefreshHead?: () => void;
  refreshingHead?: boolean;
}) {
  const { route, navigateView, openElement, closeElement } = useHashRoute();
  const ui = useExplorerUiState();
  const { navigationNotice } = ui;
  const { store } = useStore();
  const previousContext = useRef(worktreeId);
  const contextChanged = previousContext.current !== worktreeId;
  useEffect(() => {
    if (previousContext.current === worktreeId) return;
    previousContext.current = worktreeId;
    if (route.elementId) closeElement(true, true);
    setElementDetailHistory([]);
    setOntologyNodeId(null);
  }, [worktreeId, route.elementId, closeElement]);
  const viewOverride = viewOverrides?.[route.view];
  const [helpOpen, setHelpOpen] = useState(false);
  const [elementDetailHistory, setElementDetailHistory] = useState<string[]>([]);
  const [ontologyNodeId, setOntologyNodeId] = useState<string | null>(null);
  const { isDark, toggleTheme } = useTheme();
  const { leftPaneOpen, leftPaneResizing, leftPaneWidth, shellRef, toggleLeftPane,
    handleLeftPaneResizePointerDown, handleLeftPaneResizeKeyDown, minWidth, maxWidth } = useExplorerPane();
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



  return (
    <ExplorerWorkspace
      inert={worktreeBlocked || undefined}
      ref={shellRef}
      activeNavigationValue={effectiveHeaderView}
      worktree={worktreeSelector}
      onRefreshHead={onRefreshHead}
      refreshingHead={refreshingHead}
      isDark={isDark} onToggleTheme={toggleTheme}
      onSearch={() => navigateView("search")} onHelp={() => setHelpOpen(true)}
      thesaurus={route.view === "thesaurus" && !viewOverride ? {
        concepts: prepareThesaurus(store.thesaurus).concepts,
        selectedId: ui.thesaurusSelectionId, onSelectConcept: ui.setThesaurusSelectionId,
        onOpenConcept: handleOpenElement, query: ui.thesaurusQuery, onQueryChange: ui.setThesaurusQuery,
      } : undefined}
      leftPaneOpen={leftPaneOpen}
      leftPaneResizing={leftPaneResizing}
      leftPaneWidth={leftPaneWidth}
      leftPaneMinWidth={minWidth}
      leftPaneMaxWidth={maxWidth}
      leftPaneCollapseLabel="Collapse explorer"
      leftPaneExpandLabel="Expand explorer"
      leftPaneResizeLabel="Resize explorer pane"
      onNavigate={handleShellNavigate}
      onToggleLeftPane={toggleLeftPane}
      onLeftPaneResizePointerDown={handleLeftPaneResizePointerDown}
      onLeftPaneResizeKeyDown={handleLeftPaneResizeKeyDown}
      mainWarning={[recoveryWarning, schemaMismatch ? `Store schema mismatch: ${schemaMismatch}` : navigationNotice]
        .filter(Boolean).join(" ") || null}
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
      <HelpModal open={helpOpen && !worktreeBlocked} onOpenChange={setHelpOpen} />
      <ElementDetailModal
        identifier={contextChanged || worktreeBlocked ? null : route.elementId}
        onClose={handleCloseElementDetail}
        onOpenElement={handleOpenRelatedElement}
        onOpenOntologyNode={setOntologyNodeId}
        onNavigateBack={elementDetailHistory.length > 0 ? handleElementDetailBack : undefined}
        previousElementLabel={elementDetailHistory.at(-1)}
      />
      <OntologyNodeDetailModal
        nodeId={contextChanged || worktreeBlocked ? null : ontologyNodeId}
        onClose={() => setOntologyNodeId(null)}
      />
    </ExplorerWorkspace>
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
      return <OntologiesView />;
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
