import { forwardRef } from "preact/compat";
import { AppShell, type AppShellProps, type ShellNavigationItem } from "./AppShell";
import { WorktreeSelector, type WorktreeSelectorProps } from "../../components/controls/WorktreeSelector";
import { SidePaneFrame } from "../side-pane/SidePaneFrame";
import { ThesaurusExplorer, type ThesaurusExplorerProps } from "../thesaurus/ThesaurusExplorer";
import { ThesaurusPane, type ThesaurusPaneProps } from "../thesaurus/ThesaurusPane";
import { FileBrowserScreen, type FileBrowserScreenProps } from "../files/FileBrowserScreen";
import { ModelFlowScreen, ModelGraphScreen, type ModelFlowScreenProps, type ModelGraphScreenProps } from "./ModelScreen";
import { TracesScreen, type TracesScreenProps } from "../reports/TracesScreen";
import { TraceNavigation, type TraceNavigationProps } from "../reports/TraceNavigation";
import { CoverageReportScreen, type CoverageReportScreenProps } from "../reports/CoverageReportScreen";
import { CoverageNavigationPane, type CoverageNavigationPaneProps } from "../reports/CoverageNavigation";
import { OntologyGraphScreen, type OntologyGraphScreenProps } from "../graphs/OntologyGraphScreen";
import { OntologyControlsPane, ModelGraphControls, type OntologyControlsPaneProps, type ModelGraphControlsProps } from "../graphs/GraphControlsPane";
import { FileNavigation, type FileNavigationProps } from "../files/FileNavigation";
import { SearchControlsPane, type SearchControlsPaneProps } from "../search/SearchControlsPane";
import { SearchScreen, type SearchScreenProps } from "../search/SearchScreen";

export const EXPLORER_NAVIGATION: ShellNavigationItem[] = [
  { value: "thesaurus", label: "Thesaurus", icon: "tags" },
  { value: "model", label: "Model", icon: "folder" },
  { value: "ontologies", label: "Ontologies", icon: "globe" },
  { value: "traces", label: "Traces", icon: "activity" },
  { value: "coverage", label: "Coverage", icon: "pie-chart" },
];

export interface ExplorerWorkspaceProps extends Omit<AppShellProps,
  "navigationItems" | "headerActions" | "headerContext"> {
  worktree?: Omit<WorktreeSelectorProps, "density" | "menuAlign">;
  isDark?: boolean;
  onToggleTheme?: () => void;
  onSearch?: () => void;
  onHelp?: () => void;
  onRefreshHead?: () => void;
  refreshingHead?: boolean;
  /** One controller supplies the matching production concept pane and graph. */
  thesaurus?: ThesaurusExplorerProps & Pick<ThesaurusPaneProps, "query" | "onQueryChange">;
  files?: FileBrowserScreenProps;
  fileNavigation?: FileNavigationProps;
  modelFlow?: ModelFlowScreenProps;
  modelGraph?: ModelGraphScreenProps;
  modelGraphControls?: ModelGraphControlsProps;
  traces?: TracesScreenProps;
  traceNavigation?: TraceNavigationProps;
  coverage?: CoverageReportScreenProps;
  coverageNavigation?: CoverageNavigationPaneProps;
  ontology?: OntologyGraphScreenProps;
  ontologyControls?: OntologyControlsPaneProps;
  search?: SearchScreenProps;
  searchControls?: SearchControlsPaneProps;
}

/** The production workspace chrome and screen composition, also used by fixtures. */
export const ExplorerWorkspace = forwardRef<HTMLDivElement, ExplorerWorkspaceProps>(
  function ExplorerWorkspace({ worktree, isDark, onToggleTheme, onSearch, onHelp, onRefreshHead, refreshingHead,
    thesaurus, files, fileNavigation, modelFlow, modelGraph, modelGraphControls, traces, traceNavigation,
    coverage, coverageNavigation, ontology, ontologyControls, search, searchControls,
    leftPaneOpen = true, onToggleLeftPane = () => {}, sidePane, main, ...props }, ref) {
    const conceptScreen = props.activeNavigationValue === "thesaurus" && thesaurus;
    let screen = main;
    let navigation = sidePane;
    const view = props.activeNavigationValue;
    if (view === "model") {
      screen = modelGraph ? <ModelGraphScreen {...modelGraph} /> : modelFlow ? <ModelFlowScreen {...modelFlow} /> : files ? <FileBrowserScreen {...files} /> : main;
      if (modelGraph && modelGraphControls) navigation = <ModelGraphControls {...modelGraphControls} />;
      else if (fileNavigation) navigation = <FileNavigation {...fileNavigation} />;
    } else if (view === "traces" && traces) {
      screen = <TracesScreen {...traces} />;
      if (traceNavigation) navigation = <TraceNavigation {...traceNavigation} />;
    } else if (view === "coverage" && coverage) {
      screen = <CoverageReportScreen {...coverage} />;
      if (coverageNavigation) navigation = <CoverageNavigationPane {...coverageNavigation} />;
    } else if (view === "ontologies" && ontology) {
      screen = <OntologyGraphScreen {...ontology} />;
      if (ontologyControls) navigation = <OntologyControlsPane {...ontologyControls} />;
    } else if (view === "search" && search) {
      screen = <SearchScreen {...search} />;
      if (searchControls) navigation = <SearchControlsPane {...searchControls} />;
    }
    const presetPane = navigation !== sidePane;
    return <AppShell {...props} ref={ref} navigationItems={EXPLORER_NAVIGATION}
      headerContext={worktree && <WorktreeSelector {...worktree} density="compact" menuAlign="end" />}
      headerActions={[
        ...(onRefreshHead ? [{ id: "refresh-head", label: "Refresh HEAD", icon: "rotate-ccw" as const, onClick: onRefreshHead, disabled: refreshingHead }] : []),
        ...(onSearch ? [{ id: "search", label: "Search", icon: "search" as const, onClick: onSearch }] : []),
        ...(onToggleTheme ? [{ id: "theme", label: isDark ? "Switch to light mode" : "Switch to dark mode", icon: isDark ? "sun" as const : "moon" as const, onClick: onToggleTheme }] : []),
        ...(onHelp ? [{ id: "help", label: "Help", icon: "help-circle" as const, onClick: onHelp }] : []),
      ]}
      leftPaneOpen={leftPaneOpen} onToggleLeftPane={onToggleLeftPane}
      leftPaneCollapseLabel="Collapse explorer" leftPaneExpandLabel="Expand explorer"
      leftPaneResizeLabel="Resize explorer pane"
      sidePane={conceptScreen ? <SidePaneFrame open={leftPaneOpen} chrome="app"
        onToggle={onToggleLeftPane} aria-label="Explorer navigation">
        <ThesaurusPane concepts={conceptScreen.concepts} selectedId={conceptScreen.selectedId}
          query={conceptScreen.query} onQueryChange={conceptScreen.onQueryChange}
          onSelectConcept={id => { if (id) conceptScreen.onSelectConcept(id); }} />
      </SidePaneFrame> : presetPane ? <SidePaneFrame open={leftPaneOpen} chrome="app"
        onToggle={onToggleLeftPane} aria-label="Explorer navigation">{navigation}</SidePaneFrame> : sidePane}
      main={conceptScreen ? <ThesaurusExplorer {...conceptScreen} /> : screen} />;
  },
);
