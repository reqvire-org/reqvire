import { prepareThesaurus } from "../lib/thesaurus";
import { ThesaurusPane, TraceNavigation, FileNavigation, ModelGraphControls, OntologyControlsPane, CoverageNavigationPane, SearchControlsPane } from "@ds";
import { useWorktreeUrl } from "../store/worktreeUrls";
import { useMemo } from "preact/hooks";
import {
  PaneChromeHeader,
  ReqvireRailMark,
  SidePaneFrame,
} from "@ds";
import { useStore } from "../store/StoreContext";
import { routeForContent, routeForResource, VIEW_TITLES, type ViewId } from "../router/routes";
import type {
  ExplorerProjectStore,
  KnowledgeGraphNode,
  ProjectStoreElement,
} from "../store/types";
import { useExplorerUiState } from "../state/ExplorerUiState";
import { type SearchKind } from "../search/searchKinds";
import { coverageScopeOptions } from "../lib/coverage";

interface ExplorerSidePaneProps {
  activeView: ViewId;
  open: boolean;
  chrome?: "standalone" | "app";
  onToggle: () => void;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
  sourceBrowsing?: boolean;
  onOpenSourceRoute?: (hash: string) => void;
  onOpenOntologyNode: (id: string) => void;
}

export function ExplorerSidePane({
  activeView,
  open,
  chrome = "standalone",
  onToggle,
  onNavigate,
  onOpenElement,
  sourceBrowsing = false,
  onOpenSourceRoute,
  onOpenOntologyNode,
}: ExplorerSidePaneProps) {
  const ui = useExplorerUiState();
  const graphModelActive = activeView === "model" && ui.modelMode === "graph";
  const showProjectTree = (activeView === "model" || activeView === "files") && !graphModelActive;
  const title = graphModelActive ? "Graph Explorer" : `${VIEW_TITLES[activeView]} Explorer`;

  return (
    <SidePaneFrame
      open={open}
      chrome={chrome}
      header={<PaneChromeHeader title={title} />}
      railMark={<ReqvireRailMark />}
      onToggle={onToggle}
      aria-label="Explorer navigation"
    >
      <ExplorerViewControls
        activeView={activeView}
        onOpenElement={onOpenElement}
        onOpenOntologyNode={onOpenOntologyNode}
      />
      {activeView === "traces" ? <TracePaneContents /> : null}
      {showProjectTree && <ProjectTreeContents activeView={activeView} onNavigate={onNavigate}
        onOpenElement={onOpenElement} sourceBrowsing={sourceBrowsing} onOpenSourceRoute={onOpenSourceRoute} />}
    </SidePaneFrame>
  );
}

function ProjectTreeContents({ activeView, onNavigate, onOpenElement, sourceBrowsing = false, onOpenSourceRoute }:
  Pick<ExplorerSidePaneProps, "activeView" | "onNavigate" | "onOpenElement" | "sourceBrowsing" | "onOpenSourceRoute">) {
  const { store, elementById, getProjectFileTree } = useStore();
  const ui = useExplorerUiState();
  const summary = store.knowledge_graph.summary;
  const summaryItems = activeView === "model" ? [
    { label: "Elements", value: formatSummaryValue(summary?.elements ?? store.elements.length) },
    { label: "Relations", value: formatSummaryValue(summary?.relations ?? store.relations.length) },
    { label: "Files", value: formatSummaryValue(store.files.length) },
    { label: "Resources", value: formatSummaryValue(store.resources.length) },
  ] : undefined;
  return <FileNavigation tree={getProjectFileTree()} elementById={elementById} activeView={activeView}
    onNavigate={onNavigate} onOpenElement={onOpenElement} sourceBrowsing={sourceBrowsing} onOpenSourceRoute={onOpenSourceRoute}
    modelMode={ui.modelMode} modelSelectionId={ui.modelSelectionId} setModelSelectionId={ui.setModelSelectionId}
    modelTreeQuery={ui.modelTreeQuery} setModelTreeQuery={ui.setModelTreeQuery}
    routeForContent={routeForContent} routeForResource={routeForResource}
    navigateHash={hash => { window.location.hash = hash; }} summaryItems={summaryItems} />;
}

function ExplorerViewControls({
  activeView,
  onOpenElement,
  onOpenOntologyNode,
}: {
  activeView: ViewId;
  onOpenElement: (id: string) => void;
  onOpenOntologyNode: (id: string) => void;
}) {
  const contextualUrl = useWorktreeUrl();
  const ui = useExplorerUiState();
  const { store, elementById } = useStore();

  const coverageScopes = useMemo(() => activeView === "coverage" ? coverageScopeOptions(store) : [],
    [activeView, store.coverage.scope_index, store.relations]);
  const thesaurus = useMemo(() => activeView === "thesaurus" ? prepareThesaurus(store.thesaurus) : null, [activeView, store.thesaurus]);

  const graphControlsActive = (activeView === "model" && ui.modelMode === "graph");
  const graphTypeOptions = useMemo(
    () => buildKnowledgeGraphTypeOptions(store.knowledge_graph.nodes ?? [], elementById),
    [elementById, store.knowledge_graph.nodes],
  );
  const searchElementTypeOptions = useMemo(
    () => buildSearchElementTypeOptions(store.elements),
    [store.elements],
  );
  const searchKindCounts = useMemo(
    () => buildSearchKindCounts(store),
    [store.elements.length, store.files.length, store.resources.length, store.ontology.graph_data?.nodes],
  );

  if ((activeView === "model" || activeView === "files") && !graphControlsActive) {
    return null;
  }

  if (graphControlsActive) {
    const graphSummaryItems = [
      {
        label: "Submodels",
        value: formatSummaryValue(
          store.knowledge_graph.summary?.submodels ?? store.knowledge_graph.submodels?.length ?? 0,
        ),
      },
      {
        label: "Elements",
        value: formatSummaryValue(store.knowledge_graph.summary?.elements ?? store.elements.length),
      },
      {
        label: "Relations",
        value: formatSummaryValue(store.knowledge_graph.summary?.relations ?? store.relations.length),
      },
      {
        label: "Contract Binding",
        value: formatSummaryValue(store.knowledge_graph.summary?.contract_bindings ?? store.contract_bindings.length),
      },
    ];

    return <ModelGraphControls selectedNodeId={ui.knowledgeGraphSelectionId}
      nodes={store.knowledge_graph.nodes ?? []} elementById={elementById} onOpenElement={onOpenElement}
      onClear={() => ui.setKnowledgeGraphSelectionId(null)} options={graphTypeOptions}
      activeTypes={ui.modelTypes} onResetFilters={ui.resetModelTypes} onToggleType={ui.toggleModelType}
      summaryItems={graphSummaryItems} />;
  }

  if (activeView === "thesaurus") {
    return <ThesaurusPane concepts={thesaurus?.concepts ?? []}
      selectedId={ui.thesaurusSelectionId} query={ui.thesaurusQuery}
      onQueryChange={ui.setThesaurusQuery} onSelectConcept={ui.setThesaurusSelectionId} />;
  }

  if (activeView === "coverage") {
    return <CoverageNavigationPane key={JSON.stringify([store.project.workspace_root, store.project.worktree_id])}
      scopes={coverageScopes} selectedId={ui.coverageScopeId} onSelect={ui.setCoverageScopeId}
      query={ui.coverageTreeQuery} onQueryChange={ui.setCoverageTreeQuery} />;
  }

  if (activeView === "search") return <SearchControlsPane kinds={ui.searchKinds} counts={searchKindCounts}
    elementTypes={ui.searchElementTypes} options={searchElementTypeOptions} onReset={ui.resetSearchKinds}
    onToggleKind={ui.toggleSearchKind} onToggleElementType={ui.toggleSearchElementType} />;

  if (activeView === "ontologies") {
    const summary = store.ontology.summary ?? {};
    const externalCounts = store.ontology.external_counts ?? {};
    const declaredExternalSources = readNumber(externalCounts.declared_external_source_count);
    const usedExternalSources = readNumber(externalCounts.used_external_source_count);
    const materializedExternalTerms = readNumber(externalCounts.materialized_external_term_count);
    const materializedExternalTriples = readNumber(externalCounts.materialized_external_triple_count);
    const ontologySummaryItems = [
      { label: "Ontologies", value: formatSummaryValue(summary.ontology_blocks ?? 0) },
      { label: "Shapes", value: formatSummaryValue(summary.shape_blocks ?? 0) },
      {
        label: "Quads",
        value: formatSummaryValue(summary.total_quads ?? 0),
        title: "RDF statements (subject-predicate-object, with graph context)",
      },
      {
        label: "Blocks",
        value: formatSummaryValue(summary.total_blocks ?? 0),
        title: "Ontology and shape source blocks discovered in the model",
      },
    ];
    if (declaredExternalSources > 0) {
      ontologySummaryItems.push(
        {
          label: "Ext Sources",
          value: formatSummaryValue(usedExternalSources),
          title: `${formatSummaryValue(declaredExternalSources)} declared external source(s); ${formatSummaryValue(usedExternalSources)} contribute to the used subset`,
        },
        {
          label: "Ext Terms",
          value: formatSummaryValue(materializedExternalTerms),
          title: "External terms materialized into the Explorer used subset",
        },
        {
          label: "Ext Triples",
          value: formatSummaryValue(materializedExternalTriples),
          title: "External triples materialized into the Explorer used subset",
        },
      );
    }
    return <OntologyControlsPane selectedNodeId={ui.ontologySelectionId}
      nodes={store.ontology.graph_data?.nodes ?? []} onOpenNode={onOpenOntologyNode}
      onClear={() => { ui.setOntologySelectionId(null); window.clearOntologySelection?.(); }}
      filters={ui.ontologyFilters} onToggleFilter={ui.toggleOntologyFilter}
      ttlHref={store.ontology.ttl_href ? contextualUrl(store.ontology.ttl_href) : undefined}
      onResetLayout={() => window.resetOntologyGraphLayout?.()} summaryItems={ontologySummaryItems} />;
  }

  return null;
}

function formatSummaryValue(value: string | number) {
  return typeof value === "number" ? value.toLocaleString() : value;
}

function TracePaneContents() {
  const { store, getTraceFiles } = useStore();
  const ui = useExplorerUiState();
  return <TraceNavigation files={getTraceFiles()} rootLabel={projectTreeRootLabel(store)}
    traceTreeQuery={ui.traceTreeQuery} setTraceTreeQuery={ui.setTraceTreeQuery}
    traceFilePath={ui.traceFilePath} setTraceFilePath={ui.setTraceFilePath}
    traceSelectionId={ui.traceSelectionId} setTraceSelectionId={ui.setTraceSelectionId} />;
}

function projectTreeRootLabel(store: ExplorerProjectStore) {
  const rootLabel = store.project.root_label?.trim();
  const repository = store.project.repository?.trim();
  return rootLabel || repository || "Workspace";
}

const ELEMENT_TYPE_ORDER = [
  "capability",
  "requirement",
  "contract",
  "verification-objective",
  "verification",
  "test-verification",
  "formal-proof-verification",
  "analysis-verification",
  "inspection-verification",
  "demonstration-verification",
  "specification",
  "semantic-contract",
  "semantic-query",
  "ontology",
  "concept-scheme",
  "concept",
  "resource",
  "other",
];

function buildKnowledgeGraphTypeOptions(
  nodes: readonly KnowledgeGraphNode[],
  elementById: (id: string) => ProjectStoreElement | undefined,
) {
  const byType = new Map<string, { type: string; family: string; count: number }>();
  for (const node of nodes) {
    const type = knowledgeGraphNodeKind(node);
    const element = node.identifier ? elementById(node.identifier) : undefined;
    const family = element?.type_family || node.node_type || node.type || type;
    const existing = byType.get(type);
    if (existing) {
      existing.count += 1;
    } else {
      byType.set(type, { type, family, count: 1 });
    }
  }

  return Array.from(byType.values()).sort((left, right) => {
    const leftRank = elementTypeRank(left.type, left.family);
    const rightRank = elementTypeRank(right.type, right.family);
    return leftRank - rightRank || left.type.localeCompare(right.type);
  });
}

function knowledgeGraphNodeKind(node: KnowledgeGraphNode) {
  return node.element_type || node.node_type || node.type || "other";
}

function buildSearchKindCounts(store: ExplorerProjectStore): Record<SearchKind, number> {
  return {
    file: store.files.length,
    element: store.elements.length,
    resource: store.resources.length,
    ontology: store.ontology.graph_data?.nodes?.length ?? 0,
  };
}

function readNumber(value: unknown, fallback = 0): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function buildSearchElementTypeOptions(elements: readonly ProjectStoreElement[]) {
  const byType = new Map<string, { type: string; family: string; count: number }>();
  for (const element of elements) {
    if (!element.element_type) continue;
    const existing = byType.get(element.element_type);
    if (existing) {
      existing.count += 1;
    } else {
      byType.set(element.element_type, {
        type: element.element_type,
        family: element.type_family || element.element_type,
        count: 1,
      });
    }
  }

  return Array.from(byType.values()).sort((left, right) => {
    const leftRank = elementTypeRank(left.type, left.family);
    const rightRank = elementTypeRank(right.type, right.family);
    return leftRank - rightRank || left.type.localeCompare(right.type);
  });
}

function elementTypeRank(type: string, family: string) {
  const direct = ELEMENT_TYPE_ORDER.indexOf(type);
  if (direct >= 0) return direct;
  const familyRank = ELEMENT_TYPE_ORDER.indexOf(family);
  return familyRank >= 0 ? familyRank + 0.5 : ELEMENT_TYPE_ORDER.length;
}
