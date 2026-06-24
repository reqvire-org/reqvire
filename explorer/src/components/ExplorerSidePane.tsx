import { useEffect, useMemo, useState, type FormEvent } from "react";
import {
  Button,
  ElementIcon,
  Icon,
  PaneActionRow,
  PaneChromeHeader,
  PaneControlSection,
  PaneFilterGroup,
  PaneFilterGrid,
  PaneFilterNavList,
  PaneFilterNavRow,
  PaneGhostLink,
  PaneLegend,
  PaneNotationLegend,
  PaneSearchForm,
  PaneSelection,
  PaneSummary,
  PaneTree,
  PaneTreeNode,
  ReqvireRailMark,
  SidePaneFrame,
  TokenSwatch,
  ToggleRow,
  TreeItem,
  type DesignSystemColorToken,
} from "@ds";
import { useStore } from "../store/StoreContext";
import { routeForContent, VIEW_TITLES, type ViewId } from "../router/routes";
import type {
  ExplorerProjectStore,
  KnowledgeGraphNode,
  OntologyGraphNode,
  ProjectStoreElement,
  ProjectStoreFile,
} from "../store/types";
import { ONTOLOGY_LAYER_FILTERS, useExplorerUiState, type CoverageSectionId } from "../state/ExplorerUiState";
import { SEARCH_KINDS, type SearchKind } from "../search/searchKinds";
import { buildTraceFiles, type TraceFileNode } from "../lib/traces";

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

interface TreeFolder {
  path: string;
  name: string;
  folders: TreeFolder[];
  files: ProjectStoreFile[];
}

const ROOT_PATH = "__root__";

interface TracePaneVerification {
  id: string;
  name: string;
  type?: string;
}

interface TracePaneFile {
  path: string;
  name: string;
  verifications: TracePaneVerification[];
}

interface TracePaneFolder {
  path: string;
  name: string;
  folders: TracePaneFolder[];
  files: TracePaneFile[];
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
  const { store, elementById } = useStore();
  const ui = useExplorerUiState();
  const projectRootLabel = projectTreeRootLabel(store);
  const tree = useMemo(() => buildFileTree(store.files, projectRootLabel), [projectRootLabel, store.files]);
  const filteredTree = useMemo(
    () => filterFileTree(tree, ui.modelTreeQuery, elementById),
    [elementById, tree, ui.modelTreeQuery],
  );
  const traceFiles = useMemo(() => buildTraceFiles(store), [store]);
  const traceTree = useMemo(() => buildTraceFileTree(traceFiles, projectRootLabel), [projectRootLabel, traceFiles]);
  const filteredTraceTree = useMemo(
    () => filterTraceFileTree(traceTree, ui.traceTreeQuery),
    [traceTree, ui.traceTreeQuery],
  );
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
      {activeView === "ontologies" && <OntologyGraphSearch />}
      {activeView === "thesaurus" && <ThesaurusSearch />}
      {showProjectTree && <ProjectTreeSearch />}
      {activeView === "traces" && <TraceTreeSearch />}
      <ExplorerViewControls
        activeView={activeView}
        onOpenElement={onOpenElement}
        onOpenOntologyNode={onOpenOntologyNode}
      />
      {activeView === "traces" && (
        <PaneTree aria-label="Verification trace tree" id="trace-tree">
          <TraceTreeFolderNode folder={filteredTraceTree} depth={0} query={ui.traceTreeQuery} />
        </PaneTree>
      )}
      {activeView === "traces" ? <TraceTreeSummary files={traceFiles} /> : null}
      {showProjectTree && (
        <PaneTree aria-label="Project tree">
          <TreeFolderNode
            folder={filteredTree}
            activeView={activeView}
            elementById={elementById}
            onNavigate={onNavigate}
            onOpenElement={onOpenElement}
            sourceBrowsing={sourceBrowsing}
            onOpenSourceRoute={onOpenSourceRoute}
            depth={0}
            query={ui.modelTreeQuery}
          />
        </PaneTree>
      )}
      {showProjectTree && activeView === "model" ? <ModelTreeSummary /> : null}
    </SidePaneFrame>
  );
}

function ModelTreeSummary() {
  const { store } = useStore();
  const summary = store.knowledge_graph.summary;
  const items = [
    { label: "Elements", value: formatSummaryValue(summary?.elements ?? store.elements.length) },
    { label: "Relations", value: formatSummaryValue(summary?.relations ?? store.relations.length) },
    { label: "Files", value: formatSummaryValue(store.files.length) },
    { label: "Resources", value: formatSummaryValue(store.resources.length) },
  ];

  return <PaneSummary items={items} placement="footer" />;
}

function TraceTreeSummary({ files }: { files: TraceFileNode[] }) {
  const totals = files.reduce(
    (current, file) => {
      current.files += 1;
      current.verifications += file.verifications.length;
      for (const verification of file.verifications) {
        current.directRequirements += verification.directCount;
        current.rollupRequirements += verification.totalCount;
      }
      return current;
    },
    {
      files: 0,
      verifications: 0,
      directRequirements: 0,
      rollupRequirements: 0,
    },
  );
  const items = [
    { label: "Files", value: formatSummaryValue(totals.files) },
    { label: "Verifications", value: formatSummaryValue(totals.verifications) },
    { label: "Direct reqs", value: formatSummaryValue(totals.directRequirements) },
    { label: "Rollup reqs", value: formatSummaryValue(totals.rollupRequirements) },
  ];

  return <PaneSummary items={items} placement="footer" />;
}

function OntologyGraphSearch() {
  const [query, setQuery] = useState("");

  useEffect(() => {
    window.filterOntologyGraph?.(query);
  }, [query]);

  function submitSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    window.filterOntologyGraph?.(query);
  }

  return (
    <PaneSearchForm
      searchInputId="ontology-graph-search"
      inputLabel="Search Explorer"
      placeholder="Search ontology graph..."
      value={query}
      resultsId="ontology-graph-results"
      onQueryChange={setQuery}
      onSubmit={submitSearch}
    />
  );
}

function ThesaurusSearch() {
  const ui = useExplorerUiState();

  return (
    <PaneSearchForm
      searchInputId="thesaurus-search"
      inputLabel="Filter concepts"
      placeholder="Filter concepts..."
      value={ui.thesaurusQuery}
      resultsId="thesaurus-tree"
      onQueryChange={ui.setThesaurusQuery}
      onSubmit={(event) => event.preventDefault()}
    />
  );
}

function ProjectTreeSearch() {
  const ui = useExplorerUiState();

  return (
    <PaneSearchForm
      searchInputId="project-tree-search"
      inputLabel="Filter project tree"
      placeholder="Filter model tree..."
      value={ui.modelTreeQuery}
      resultsId="project-tree"
      onQueryChange={ui.setModelTreeQuery}
      onSubmit={(event) => event.preventDefault()}
    />
  );
}

function TraceTreeSearch() {
  const ui = useExplorerUiState();

  return (
    <PaneSearchForm
      searchInputId="trace-tree-search"
      inputLabel="Filter trace tree"
      placeholder="Filter trace tree..."
      value={ui.traceTreeQuery}
      resultsId="trace-tree"
      onQueryChange={ui.setTraceTreeQuery}
      onSubmit={(event) => event.preventDefault()}
    />
  );
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
  const ui = useExplorerUiState();
  const { store, elementById } = useStore();

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

    return (
      <>
        <PaneControlSection aria-label="Graph controls">
          <KnowledgeGraphSelectedElementLink
            selectedNodeId={ui.knowledgeGraphSelectionId}
            nodes={store.knowledge_graph.nodes ?? []}
            elementById={elementById}
            onOpenElement={onOpenElement}
            onClear={() => ui.setKnowledgeGraphSelectionId(null)}
          />
          <PaneFilterGroup label="Show">
            <Button size="sm" onClick={ui.resetModelTypes}>
              Reset filters
            </Button>
            {graphTypeOptions.map((option) => (
              <ToggleRow
                key={option.type}
                label={humanize(option.type)}
                on={ui.modelTypes.has(option.type)}
                variant="filter"
                icon={<ElementIcon type={option.type} family={option.family} size="sm" />}
                meta={formatCompactCount(option.count)}
                onToggle={() => ui.toggleModelType(option.type)}
              />
            ))}
          </PaneFilterGroup>
        </PaneControlSection>
        <PaneSummary items={graphSummaryItems} placement="footer" />
      </>
    );
  }

  if (activeView === "traces") return null;

  if (activeView === "thesaurus") {
    const thesaurusTree = buildThesaurusPaneTree(store);
    const filteredTree = filterThesaurusPaneTree(thesaurusTree, ui.thesaurusQuery);
    const conceptCount = thesaurusTree.reduce((total, scheme) => total + scheme.concepts.length, 0);
    const summaryItems = [
      { label: "Schemes", value: formatSummaryValue(thesaurusTree.length) },
      { label: "Concepts", value: formatSummaryValue(conceptCount) },
    ];
    return (
      <>
        <PaneTree aria-label="Concept hierarchy" id="thesaurus-tree">
          {filteredTree.map((scheme) => (
            <ThesaurusSchemeTreeNode
              key={scheme.id}
              scheme={scheme}
              selectedId={ui.thesaurusSelectionId}
              query={ui.thesaurusQuery}
              onSelectConcept={ui.setThesaurusSelectionId}
            />
          ))}
        </PaneTree>
        <PaneSummary items={summaryItems} placement="footer" />
      </>
    );
  }

  if (activeView === "coverage") {
    const coverageItems = buildCoveragePaneItems(store);
    const coverage = isPlainRecord(store.coverage) ? store.coverage : {};
    const summary = isPlainRecord(coverage.summary) ? coverage.summary : {};
    const coverageSummaryItems = [
      { label: "Requirements", value: formatSummaryValue(readNumber(summary.total_requirements_in_scope)) },
      { label: "Leaf reqs", value: formatSummaryValue(readNumber(summary.total_leaf_requirements)) },
      { label: "Verifications", value: formatSummaryValue(readNumber(summary.total_verifications)) },
    ];
    return (
      <>
        <PaneControlSection aria-label="Coverage explorer">
          <PaneFilterGroup label="Coverage">
            <PaneFilterNavList>
              {coverageItems.map((item) => (
                <PaneFilterNavRow
                  key={item.id}
                  icon={item.icon}
                  label={item.label}
                  count={formatCompactCount(item.count)}
                  selected={ui.coverageSectionId === item.id}
                  onClick={() => {
                    ui.setCoverageSectionId(item.id);
                    navigateCoverageSection(item.id);
                  }}
                />
              ))}
            </PaneFilterNavList>
          </PaneFilterGroup>
        </PaneControlSection>
        <PaneSummary items={coverageSummaryItems} placement="footer" />
      </>
    );
  }

  if (activeView === "search") {
    return (
      <PaneControlSection aria-label="Search controls" title="Filter by">
        <Button size="sm" onClick={ui.resetSearchKinds}>
          Reset filters
        </Button>
        <PaneFilterGroup label="Result types">
          {SEARCH_KINDS.map((kind) => (
            <ToggleRow
              key={kind}
              label={searchKindLabel(kind)}
              on={ui.searchKinds.has(kind)}
              variant="filter"
              colorToken={searchKindColorToken(kind)}
              meta={formatCompactCount(searchKindCounts[kind] ?? 0)}
              onToggle={() => ui.toggleSearchKind(kind)}
            />
          ))}
        </PaneFilterGroup>
        {searchElementTypeOptions.length > 0 ? (
          <PaneFilterGroup label="Element types">
            {searchElementTypeOptions.map((option) => (
              <ToggleRow
                key={option.type}
                label={humanize(option.type)}
                on={ui.searchElementTypes.has(option.type)}
                variant="filter"
                icon={<ElementIcon type={option.type} family={option.family} size="sm" />}
                meta={formatCompactCount(option.count)}
                onToggle={() => ui.toggleSearchElementType(option.type)}
              />
            ))}
          </PaneFilterGroup>
        ) : null}
      </PaneControlSection>
    );
  }

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
    const ontologyLayerCounts = new Map<string, number>();
    for (const node of store.ontology.graph_data?.nodes ?? []) {
      ontologyLayerCounts.set(node.layer, (ontologyLayerCounts.get(node.layer) ?? 0) + 1);
    }
    return (
      <>
        <PaneControlSection aria-label="Ontology controls">
          <OntologySelectedNodeLink
            selectedNodeId={ui.ontologySelectionId}
            nodes={store.ontology.graph_data?.nodes ?? []}
            onOpenOntologyNode={onOpenOntologyNode}
            onClear={() => {
              ui.setOntologySelectionId(null);
              window.clearOntologySelection?.();
            }}
          />
          <PaneFilterGroup label="Graph">
            <PaneActionRow>
              {store.ontology.ttl_href ? (
                <PaneGhostLink
                  href={store.ontology.ttl_href}
                  title="Download the exported ontology as Turtle (ontologies.ttl)"
                >
                  <Icon name="download" />
                  Download .ttl
                </PaneGhostLink>
              ) : null}
              <Button
                tone="ghost"
                size="sm"
                iconLeft={<Icon name="rotate-ccw" />}
                onClick={() =>
                  (window as typeof window & { resetOntologyGraphLayout?: () => void })
                    .resetOntologyGraphLayout?.()
                }
              >
                Reset layout
              </Button>
            </PaneActionRow>
          </PaneFilterGroup>
          <PaneFilterGroup label="Overlays">
            {ONTOLOGY_LAYER_FILTERS.map(([value, label]) => {
              const layer = value.replace("layer-", "");
              const count = formatCompactCount(ontologyLayerCounts.get(layer) ?? 0);
              return (
                <ToggleRow
                  key={value}
                  label={label}
                  colorToken={ontologyLayerColorToken(value)}
                  on={ui.ontologyFilters.has(value)}
                  variant="filter"
                  meta={count}
                  title={ontologyLayerDescription(value)}
                  onToggle={() => ui.toggleOntologyFilter(value)}
                />
              );
            })}
          </PaneFilterGroup>
          <PaneFilterGrid columns="two">
            <div>
              <PaneFilterGroup label="Types">
                <PaneLegend
                  rows={[
                    { id: "class", label: "Class", colorToken: ontologyColorToken("class") },
                    { id: "skos-concept", label: "Concept", colorToken: ontologyColorToken("skos-concept") },
                    { id: "skos-concept-scheme", label: "Concept Scheme", colorToken: ontologyColorToken("skos-concept-scheme") },
                    { id: "named-individual", label: "Individual", colorToken: ontologyColorToken("named-individual") },
                    { id: "datatype", label: "Datatype", colorToken: ontologyColorToken("datatype") },
                    { id: "class-expression", label: "Class expr.", colorToken: ontologyColorToken("class-expression") },
                    { id: "node-shape", label: "Node shape", colorToken: ontologyColorToken("node-shape") },
                    { id: "property-shape", label: "Property shape", colorToken: ontologyColorToken("property-shape") },
                    { id: "resource", label: "Resource", colorToken: ontologyColorToken("resource") },
                  ]}
                />
                <PaneLegend rows={[{ id: "relation", label: "Relation", colorToken: "--text-muted", line: true }]} />
              </PaneFilterGroup>
            </div>
            <div>
              <PaneFilterGroup label="Notation">
                <PaneNotationLegend
                  rows={[
                    { symbol: "D/R", label: "Domain/range" },
                    { symbol: "⊆", label: "Subclass" },
                    { symbol: "∈", label: "Membership" },
                    { symbol: "⟂", label: "Disjoint" },
                    { symbol: "⇔", label: "Equivalence" },
                    { symbol: "⟲", label: "Inverse" },
                    { symbol: "∘", label: "Property chain" },
                    { symbol: "∩", label: "Class expr." },
                    { symbol: "SH", label: "SHACL overlay" },
                  ]}
                />
              </PaneFilterGroup>
            </div>
          </PaneFilterGrid>
        </PaneControlSection>
        <PaneSummary items={ontologySummaryItems} placement="footer" />
      </>
    );
  }

  return null;
}

function KnowledgeGraphSelectedElementLink({
  selectedNodeId,
  nodes,
  elementById,
  onOpenElement,
  onClear,
}: {
  selectedNodeId: string | null;
  nodes: KnowledgeGraphNode[];
  elementById: (id: string) => ProjectStoreElement | undefined;
  onOpenElement: (id: string) => void;
  onClear: () => void;
}) {
  const node = selectedNodeId
    ? nodes.find((candidate) => candidate.id === selectedNodeId)
    : undefined;
  const element = node?.identifier ? elementById(node.identifier) : undefined;
  const kind = element?.element_type || node?.element_type || node?.node_type || node?.type || "element";

  return (
    <PaneSelection
      ariaLabel="Selected graph element"
      emptyMessage="Select a graph node to inspect its details."
      selection={
        element
          ? {
              icon: (
                <ElementIcon
                  type={element.element_type}
                  family={element.type_family}
                  title={element.element_type}
                  size="sm"
                />
              ),
              name: element.name,
              kind,
            }
          : undefined
      }
      openTitle="Open element details"
      onOpen={element ? () => onOpenElement(element.id) : undefined}
      onClear={onClear}
    />
  );
}

function OntologySelectedNodeLink({
  selectedNodeId,
  nodes,
  onOpenOntologyNode,
  onClear,
}: {
  selectedNodeId: string | null;
  nodes: OntologyGraphNode[];
  onOpenOntologyNode: (id: string) => void;
  onClear: () => void;
}) {
  const node = selectedNodeId
    ? nodes.find((candidate) => candidate.id === selectedNodeId)
    : undefined;
  const kind = node ? node.semantic_type || node.node_type || node.type || "resource" : "";

  return (
    <PaneSelection
      ariaLabel="Selected ontology node"
      emptyMessage="Select a graph node to inspect its details."
      selection={
        node
          ? {
              icon: <TokenSwatch colorToken={ontologyColorToken(kind)} title={kind} />,
              name: node.label || node.id,
              kind,
            }
          : undefined
      }
      openTitle="Open node details"
      onOpen={node ? () => onOpenOntologyNode(node.id) : undefined}
      onClear={onClear}
    />
  );
}

function TreeFolderNode({
  folder,
  activeView,
  elementById,
  onNavigate,
  onOpenElement,
  sourceBrowsing,
  onOpenSourceRoute,
  depth,
  query,
}: {
  folder: TreeFolder;
  activeView: ViewId;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
  sourceBrowsing: boolean;
  onOpenSourceRoute?: (hash: string) => void;
  depth: number;
  query: string;
}) {
  const [open, setOpen] = useState(depth === 0);
  const ui = useExplorerUiState();
  const selectionId = folder.path === ROOT_PATH ? "__root__" : `folder:${folder.path}`;
  const expanded = Boolean(query.trim()) || open;

  function selectFolder() {
    if (folder.files.length + folder.folders.length > 0) {
      setOpen((value) => !value);
    }
    ui.setModelSelectionId(selectionId);
    if (activeView === "files") onNavigate("model");
  }

  return (
    <PaneTreeNode>
      <TreeItem
        kind="folder"
        label={folder.name}
        icon={expanded ? <Icon name="folder-open" className="file-kind-folder" /> : <Icon name="folder" className="file-kind-folder" />}
        count={folder.files.length + folder.folders.length}
        depth={depth}
        open={expanded}
        expandable={folder.files.length + folder.folders.length > 0}
        selected={ui.modelSelectionId === selectionId}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFolder}
      />
      {expanded && (
        <>
          {folder.folders.map((child) => (
            <TreeFolderNode
              key={child.path}
              folder={child}
              activeView={activeView}
              elementById={elementById}
              onNavigate={onNavigate}
              onOpenElement={onOpenElement}
              sourceBrowsing={sourceBrowsing}
              onOpenSourceRoute={onOpenSourceRoute}
              depth={depth + 1}
              query={query}
            />
          ))}
          {folder.files.map((file) => (
            <TreeFileNode
              key={file.path}
              file={file}
              activeView={activeView}
              elementById={elementById}
              onNavigate={onNavigate}
              onOpenElement={onOpenElement}
              sourceBrowsing={sourceBrowsing}
              onOpenSourceRoute={onOpenSourceRoute}
              depth={depth + 1}
              query={query}
            />
          ))}
        </>
      )}
    </PaneTreeNode>
  );
}

function TraceTreeFolderNode({
  folder,
  depth,
  query,
}: {
  folder: TracePaneFolder;
  depth: number;
  query: string;
}) {
  const ui = useExplorerUiState();
  const selectedPath = ui.traceFilePath;
  const hasSelectedDescendant = selectedPath
    ? traceFolderContainsPath(folder, selectedPath)
    : false;
  const [open, setOpen] = useState(depth < 2 || hasSelectedDescendant);
  const expanded = Boolean(query.trim()) || open;

  useEffect(() => {
    if (hasSelectedDescendant) setOpen(true);
  }, [hasSelectedDescendant]);

  return (
    <PaneTreeNode>
      <TreeItem
        kind="folder"
        label={folder.name}
        icon={expanded ? <Icon name="folder-open" className="file-kind-folder" /> : <Icon name="folder" className="file-kind-folder" />}
        count={traceFolderVerificationCount(folder)}
        depth={depth}
        open={expanded}
        expandable={folder.files.length + folder.folders.length > 0}
        selected={folder.path === ROOT_PATH && !selectedPath}
        onToggle={() => setOpen((value) => !value)}
        onSelect={() => {
          ui.setTraceFilePath(null);
          ui.setTraceSelectionId(null);
        }}
      />
      {expanded && (
        <>
          {folder.folders.map((child) => (
            <TraceTreeFolderNode key={child.path} folder={child} depth={depth + 1} query={query} />
          ))}
          {folder.files.map((file) => (
            <TraceTreeFileNode key={file.path} file={file} depth={depth + 1} query={query} />
          ))}
        </>
      )}
    </PaneTreeNode>
  );
}

function TraceTreeFileNode({
  file,
  depth,
  query,
}: {
  file: TracePaneFile;
  depth: number;
  query: string;
}) {
  const ui = useExplorerUiState();
  const selectedFile = ui.traceFilePath === file.path;
  const selectedVerification = selectedFile ? ui.traceSelectionId : null;
  const [open, setOpen] = useState(true);
  const expanded = Boolean(query.trim()) || open;

  function selectFile() {
    ui.setTraceFilePath(file.path);
    ui.setTraceSelectionId(null);
  }

  function selectVerification(id: string) {
    ui.setTraceFilePath(file.path);
    ui.setTraceSelectionId(id);
  }

  return (
    <PaneTreeNode>
      <TreeItem
        kind="file"
        label={file.name}
        icon={<Icon name="file" className="file-kind-file" />}
        count={file.verifications.length}
        depth={depth}
        open={expanded}
        expandable={file.verifications.length > 0}
        selected={selectedFile && !selectedVerification}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFile}
      />
      {expanded && file.verifications.map((verification) => (
        <TreeItem
          key={verification.id}
          kind="element"
          label={verification.name}
          icon={<ElementIcon type={verification.type ?? "verification"} family="verification" size="sm" />}
          depth={depth + 1}
          selected={selectedVerification === verification.id}
          onSelect={() => selectVerification(verification.id)}
        />
      ))}
    </PaneTreeNode>
  );
}

function TreeFileNode({
  file,
  activeView,
  elementById,
  onNavigate,
  onOpenElement,
  sourceBrowsing,
  onOpenSourceRoute,
  depth,
  query,
}: {
  file: ProjectStoreFile;
  activeView: ViewId;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
  sourceBrowsing: boolean;
  onOpenSourceRoute?: (hash: string) => void;
  depth: number;
  query: string;
}) {
  const ui = useExplorerUiState();
  const elements = file.element_ids.map(elementById).filter(Boolean) as ProjectStoreElement[];
  const showElementChildren = elements.length > 0;
  const [open, setOpen] = useState(showElementChildren);
  const expanded = Boolean(query.trim()) || open;
  const selectionId = `file:${file.path}`;

  function selectFile() {
    if (showElementChildren) {
      setOpen((value) => !value);
    }
    ui.setModelSelectionId(selectionId);
    if (sourceBrowsing) {
      onOpenSourceRoute?.(routeForContent(file.path));
      return;
    }
    if (activeView === "files") onNavigate("model");
  }

  function selectElement(elementId: string) {
    ui.setModelSelectionId(elementId);
    if (sourceBrowsing) {
      const element = elementById(elementId);
      if (element) {
        onOpenSourceRoute?.(sourceRouteForElement(element));
      }
      return;
    }
    if (activeView === "files") onNavigate("model");
    onOpenElement(elementId);
  }

  return (
    <PaneTreeNode>
      <TreeItem
        kind="file"
        label={displayName(file.display_path || file.path)}
        icon={<Icon name="file" className="file-kind-file" />}
        count={elements.length > 0 ? elements.length : undefined}
        depth={depth}
        open={expanded}
        expandable={showElementChildren}
        selected={ui.modelSelectionId === selectionId}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFile}
      />
      {expanded && showElementChildren && elements.map((element) => (
        <TreeItem
          key={element.id}
          kind="element"
          label={element.name}
          icon={(
            <ElementIcon
              type={element.element_type}
              family={element.type_family}
              title={element.element_type}
              size="sm"
            />
          )}
          depth={depth + 1}
          selected={ui.modelSelectionId === element.id}
          onSelect={() => selectElement(element.id)}
        />
      ))}
    </PaneTreeNode>
  );
}

function sourceRouteForElement(element: ProjectStoreElement) {
  if (element.source_anchor.startsWith("#/content/")) return element.source_anchor;
  if (element.source_anchor.startsWith("#")) {
    return `${routeForContent(element.file_path)}${element.source_anchor}`;
  }
  return element.source_anchor;
}

interface ThesaurusPaneConcept {
  id: string;
  label: string;
  parentId: string | null;
  schemeId: string;
  description: string;
}

interface ThesaurusPaneScheme {
  id: string;
  label: string;
  concepts: ThesaurusPaneConcept[];
}

function ThesaurusSchemeTreeNode({
  scheme,
  selectedId,
  query,
  onSelectConcept,
}: {
  scheme: ThesaurusPaneScheme;
  selectedId: string | null;
  query: string;
  onSelectConcept: (id: string | null) => void;
}) {
  const hasSelectedDescendant = selectedId ? scheme.concepts.some((concept) => concept.id === selectedId) : false;
  const [open, setOpen] = useState(hasSelectedDescendant || scheme.concepts.length <= 8);

  useEffect(() => {
    if (hasSelectedDescendant) setOpen(true);
  }, [hasSelectedDescendant]);

  const children = thesaurusTopLevelConcepts(scheme.concepts);
  const expanded = Boolean(query.trim()) || open;

  return (
    <PaneTreeNode>
      <TreeItem
        kind="element"
        label={scheme.label}
        icon={<ElementIcon type="concept-scheme" size="sm" />}
        count={scheme.concepts.length}
        depth={0}
        open={expanded}
        expandable={children.length > 0}
        selected={false}
        onToggle={() => setOpen((value) => !value)}
        onSelect={() => {
          const first = children[0] ?? scheme.concepts[0];
          if (first) onSelectConcept(first.id);
        }}
      />
      {expanded && children.map((concept) => (
        <ThesaurusConceptTreeNode
          key={concept.id}
          concept={concept}
          concepts={scheme.concepts}
          selectedId={selectedId}
          query={query}
          onSelectConcept={onSelectConcept}
          depth={1}
        />
      ))}
    </PaneTreeNode>
  );
}

function ThesaurusConceptTreeNode({
  concept,
  concepts,
  selectedId,
  query,
  onSelectConcept,
  depth,
}: {
  concept: ThesaurusPaneConcept;
  concepts: readonly ThesaurusPaneConcept[];
  selectedId: string | null;
  query: string;
  onSelectConcept: (id: string | null) => void;
  depth: number;
}) {
  const children = concepts.filter((candidate) => candidate.parentId === concept.id);
  const hasSelectedDescendant = selectedId ? conceptTreeContains(concepts, concept.id, selectedId) : false;
  const [open, setOpen] = useState(depth < 2 || hasSelectedDescendant);
  const expanded = Boolean(query.trim()) || open;

  useEffect(() => {
    if (hasSelectedDescendant) setOpen(true);
  }, [hasSelectedDescendant]);

  return (
    <PaneTreeNode>
      <TreeItem
        kind="element"
        label={concept.label}
        icon={<ElementIcon type="concept" size="sm" />}
        count={children.length > 0 ? children.length : undefined}
        depth={depth}
        open={expanded}
        expandable={children.length > 0}
        selected={selectedId === concept.id}
        onToggle={() => setOpen((value) => !value)}
        onSelect={() => onSelectConcept(concept.id)}
        title={concept.description || concept.label}
      />
      {expanded && children.map((child) => (
        <ThesaurusConceptTreeNode
          key={child.id}
          concept={child}
          concepts={concepts}
          selectedId={selectedId}
          query={query}
          onSelectConcept={onSelectConcept}
          depth={depth + 1}
        />
      ))}
    </PaneTreeNode>
  );
}

function formatSummaryValue(value: string | number) {
  return typeof value === "number" ? value.toLocaleString() : value;
}

function buildFileTree(files: ProjectStoreFile[], rootLabel: string): TreeFolder {
  const root: TreeFolder = { path: ROOT_PATH, name: rootLabel, folders: [], files: [] };
  const byPath = new Map<string, TreeFolder>([[ROOT_PATH, root]]);

  for (const file of files) {
    const folderPath = file.parent_folder || ROOT_PATH;
    ensureFolder(folderPath, byPath, root);
    byPath.get(folderPath)?.files.push(file);
  }

  for (const folder of byPath.values()) {
    folder.folders.sort((a, b) => a.name.localeCompare(b.name));
    folder.files.sort((a, b) =>
      displayName(a.display_path || a.path).localeCompare(displayName(b.display_path || b.path)),
    );
  }

  return root;
}

function filterFileTree(
  folder: TreeFolder,
  query: string,
  elementById: (id: string) => ProjectStoreElement | undefined,
): TreeFolder {
  const normalized = query.trim().toLowerCase();
  if (!normalized) return folder;

  const filtered = filterFileTreeNode(folder, normalized, elementById, true);
  return filtered ?? { ...folder, folders: [], files: [] };
}

function filterFileTreeNode(
  folder: TreeFolder,
  query: string,
  elementById: (id: string) => ProjectStoreElement | undefined,
  isRoot = false,
): TreeFolder | null {
  const folderMatches = !isRoot && textMatches(query, folder.name, folder.path);
  if (folderMatches) return folder;

  const folders = folder.folders
    .map((child) => filterFileTreeNode(child, query, elementById))
    .filter(Boolean) as TreeFolder[];
  const files = folder.files
    .map((file) => filterProjectFile(file, query, elementById))
    .filter(Boolean) as ProjectStoreFile[];

  if (folders.length === 0 && files.length === 0 && !isRoot) return null;
  return { ...folder, folders, files };
}

function filterProjectFile(
  file: ProjectStoreFile,
  query: string,
  elementById: (id: string) => ProjectStoreElement | undefined,
): ProjectStoreFile | null {
  if (textMatches(query, file.display_path, file.path, file.markdown_content)) return file;

  const elementIds = file.element_ids.filter((id) => {
    const element = elementById(id);
    return element
      ? textMatches(query, element.name, element.element_type, element.type_family, element.content)
      : id.toLowerCase().includes(query);
  });

  return elementIds.length > 0 ? { ...file, element_ids: elementIds } : null;
}

function filterTraceFileTree(folder: TracePaneFolder, query: string): TracePaneFolder {
  const normalized = query.trim().toLowerCase();
  if (!normalized) return folder;

  const filtered = filterTraceFileTreeNode(folder, normalized, true);
  return filtered ?? { ...folder, folders: [], files: [] };
}

function filterTraceFileTreeNode(
  folder: TracePaneFolder,
  query: string,
  isRoot = false,
): TracePaneFolder | null {
  const folderMatches = !isRoot && textMatches(query, folder.name, folder.path);
  if (folderMatches) return folder;

  const folders = folder.folders
    .map((child) => filterTraceFileTreeNode(child, query))
    .filter(Boolean) as TracePaneFolder[];
  const files = folder.files
    .map((file) => filterTraceFile(file, query))
    .filter(Boolean) as TracePaneFile[];

  if (folders.length === 0 && files.length === 0 && !isRoot) return null;
  return { ...folder, folders, files };
}

function filterTraceFile(file: TracePaneFile, query: string): TracePaneFile | null {
  if (textMatches(query, file.name, file.path)) return file;

  const verifications = file.verifications.filter((verification) =>
    textMatches(query, verification.name, verification.type, verification.id),
  );

  return verifications.length > 0 ? { ...file, verifications } : null;
}

function buildTraceFileTree(files: TraceFileNode[], rootLabel: string): TracePaneFolder {
  const root: TracePaneFolder = { path: ROOT_PATH, name: rootLabel, folders: [], files: [] };
  const byPath = new Map<string, TracePaneFolder>([[ROOT_PATH, root]]);

  for (const entry of files) {
    const path = entry.file;
    const folderPath = dirname(path) || ROOT_PATH;
    const folder = ensureTraceFolder(folderPath, byPath, root);
    folder.files.push({
      path,
      name: displayName(path),
      verifications: (entry.verifications ?? [])
        .map((verification) => ({
          id: verification.id,
          name: verification.name,
          type: verification.verificationType,
        }))
        .sort((a, b) => a.name.localeCompare(b.name)),
    });
  }

  for (const folder of byPath.values()) {
    folder.folders.sort((a, b) => a.name.localeCompare(b.name));
    folder.files.sort((a, b) => a.name.localeCompare(b.name));
  }

  return root;
}

function projectTreeRootLabel(store: ExplorerProjectStore) {
  const repository = store.project.repository?.trim();
  const branch = store.project.branch?.trim();
  if (repository && branch) return `${repository} @ ${branch}`;
  return store.project.root_label || repository || "Project";
}

function ensureTraceFolder(
  path: string,
  byPath: Map<string, TracePaneFolder>,
  root: TracePaneFolder,
): TracePaneFolder {
  if (path === ROOT_PATH || path === "") return root;
  const existing = byPath.get(path);
  if (existing) return existing;
  const parentPath = dirname(path) || ROOT_PATH;
  const parent = ensureTraceFolder(parentPath, byPath, root);
  const folder: TracePaneFolder = {
    path,
    name: displayName(path),
    folders: [],
    files: [],
  };
  byPath.set(path, folder);
  parent.folders.push(folder);
  return folder;
}

function traceFolderVerificationCount(folder: TracePaneFolder): number {
  return folder.files.reduce((sum, file) => sum + file.verifications.length, 0) +
    folder.folders.reduce((sum, child) => sum + traceFolderVerificationCount(child), 0);
}

function traceFolderContainsPath(folder: TracePaneFolder, path: string): boolean {
  return folder.files.some((file) => file.path === path) ||
    folder.folders.some((child) => traceFolderContainsPath(child, path));
}

function ensureFolder(path: string, byPath: Map<string, TreeFolder>, root: TreeFolder): TreeFolder {
  if (path === ROOT_PATH || path === "") return root;
  const existing = byPath.get(path);
  if (existing) return existing;
  const parentPath = dirname(path) || ROOT_PATH;
  const parent = ensureFolder(parentPath, byPath, root);
  const folder: TreeFolder = {
    path,
    name: displayName(path),
    folders: [],
    files: [],
  };
  byPath.set(path, folder);
  parent.folders.push(folder);
  return folder;
}

function displayName(path: string) {
  const normalized = path.replace(/\\/g, "/").replace(/\/$/, "");
  return normalized.split("/").pop() || normalized || "Project";
}

function textMatches(query: string, ...values: Array<string | null | undefined>) {
  return values.some((value) => value?.toLowerCase().includes(query));
}

function dirname(path: string) {
  const normalized = path.replace(/\\/g, "/").replace(/\/$/, "");
  const index = normalized.lastIndexOf("/");
  return index > 0 ? normalized.slice(0, index) : "";
}

function humanize(value: string) {
  return value.replace(/-/g, " ").replace(/\b\w/g, (match) => match.toUpperCase());
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

function buildCoveragePaneItems(store: ExplorerProjectStore): Array<{
  id: CoverageSectionId;
  label: string;
  count: number;
  icon: "pie-chart" | "box" | "file" | "activity" | "x" | "help-circle";
}> {
  const coverage = isPlainRecord(store.coverage) ? store.coverage : {};
  const summary = isPlainRecord(coverage.summary) ? coverage.summary : {};
  return [
    {
      id: "overview",
      label: "Overview",
      count: readNumber(summary.total_requirements_in_scope, store.elements.length),
      icon: "pie-chart",
    },
    {
      id: "capability-coverage",
      label: "Capability coverage",
      count: coverageCapabilityCount(coverage.capability_coverage),
      icon: "box",
    },
    {
      id: "unverified-requirements",
      label: "Unverified requirements",
      count: coverageSectionCount(coverage.unverified_leaf_requirements),
      icon: "file",
    },
    {
      id: "unimplemented-requirements",
      label: "Unimplemented requirements",
      count: coverageSectionCount(coverage.uncovered_requirements),
      icon: "activity",
    },
    {
      id: "unsatisfied-verifications",
      label: "Unsatisfied verifications",
      count: coverageSectionCount(coverage.unsatisfied_test_verifications),
      icon: "x",
    },
    {
      id: "orphaned-verifications",
      label: "Orphaned verifications",
      count: coverageSectionCount(coverage.orphaned_verifications),
      icon: "help-circle",
    },
  ];
}

function navigateCoverageSection(section: CoverageSectionId) {
  window.dispatchEvent(new CustomEvent("reqvire:coverage-navigate", { detail: { section } }));
}

function coverageSectionCount(section: unknown): number {
  if (!isPlainRecord(section) || !isPlainRecord(section.files)) return 0;
  return Object.values(section.files).reduce<number>((count, value) => {
    return count + (Array.isArray(value) ? value.length : 0);
  }, 0);
}

function coverageCapabilityCount(section: unknown): number {
  if (!isPlainRecord(section) || !Array.isArray(section.capabilities)) return 0;
  return section.capabilities.length;
}

function readNumber(value: unknown, fallback = 0): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function isPlainRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
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

function buildThesaurusPaneTree(store: ExplorerProjectStore): ThesaurusPaneScheme[] {
  const graphNodes = store.ontology.graph_data?.nodes ?? [];
  const graphEdges = store.ontology.graph_data?.edges ?? [];
  const conceptNodes = graphNodes
    .filter(isConceptGraphNode)
    .sort((left, right) => conceptGraphLabel(left).localeCompare(conceptGraphLabel(right)));
  const schemeIds = new Set(graphNodes.filter(isConceptSchemeGraphNode).map((node) => node.id));

  const conceptIds = new Set(conceptNodes.map((node) => node.id));
  const parentByConcept = new Map<string, string>();
  for (const edge of graphEdges) {
    if (edge.label === "broader" && conceptIds.has(edge.source) && conceptIds.has(edge.target)) {
      parentByConcept.set(edge.source, edge.target);
    }
  }

  const schemeById = new Map<string, ThesaurusPaneScheme>();
  for (const node of conceptNodes) {
    const schemeId = node.scheme_iri;
    const schemeLabel = node.scheme_label;
    if (!schemeId || !schemeLabel || !schemeIds.has(schemeId)) continue;
    const scheme = ensureThesaurusPaneScheme(schemeById, schemeId, schemeLabel);
    scheme.concepts.push({
      id: node.id,
      label: conceptGraphLabel(node),
      parentId: parentByConcept.get(node.id) ?? null,
      schemeId,
      description: conceptGraphDescription(node),
    });
  }

  return sortThesaurusPaneSchemes(Array.from(schemeById.values()).filter((scheme) => scheme.concepts.length > 0));
}

function filterThesaurusPaneTree(
  schemes: readonly ThesaurusPaneScheme[],
  query: string,
): ThesaurusPaneScheme[] {
  const normalized = query.trim().toLowerCase();
  if (!normalized) return [...schemes];

  return schemes
    .map((scheme) => {
      const included = new Set<string>();
      const byId = new Map(scheme.concepts.map((concept) => [concept.id, concept]));
      for (const concept of scheme.concepts) {
        if (!thesaurusConceptMatches(concept, normalized)) continue;
        included.add(concept.id);
        let parentId = concept.parentId;
        while (parentId) {
          included.add(parentId);
          parentId = byId.get(parentId)?.parentId ?? null;
        }
      }
      return {
        ...scheme,
        concepts: scheme.concepts.filter((concept) => included.has(concept.id)),
      };
    })
    .filter((scheme) => scheme.concepts.length > 0);
}

function ensureThesaurusPaneScheme(
  schemeById: Map<string, ThesaurusPaneScheme>,
  id: string,
  label: string,
): ThesaurusPaneScheme {
  const existing = schemeById.get(id);
  if (existing) return existing;
  const scheme = { id, label, concepts: [] };
  schemeById.set(id, scheme);
  return scheme;
}

function sortThesaurusPaneSchemes(schemes: ThesaurusPaneScheme[]) {
  return schemes
    .map((scheme) => ({
      ...scheme,
      concepts: [...scheme.concepts].sort((left, right) => {
        const leftDepth = thesaurusConceptDepth(scheme.concepts, left.id);
        const rightDepth = thesaurusConceptDepth(scheme.concepts, right.id);
        return leftDepth - rightDepth || left.label.localeCompare(right.label);
      }),
    }))
    .sort((left, right) => left.label.localeCompare(right.label));
}

function isConceptGraphNode(node: OntologyGraphNode) {
  return node.semantic_type === "skos-concept";
}

function isConceptSchemeGraphNode(node: OntologyGraphNode) {
  return node.semantic_type === "skos-concept-scheme";
}

function conceptGraphLabel(node: OntologyGraphNode) {
  return firstConceptLiteralValue(node, "prefLabel") || node.label;
}

function conceptGraphDescription(node: OntologyGraphNode) {
  return firstConceptLiteralValue(node, "definition") || firstConceptLiteralValue(node, "scopeNote") || node.comment;
}

function firstConceptLiteralValue(node: OntologyGraphNode, predicateSuffix: string) {
  return (node.literal_values ?? []).find((value) => value.predicate.endsWith(predicateSuffix))?.value ?? "";
}

function thesaurusConceptDepth(concepts: readonly ThesaurusPaneConcept[], id: string) {
  const parentById = new Map(concepts.map((concept) => [concept.id, concept.parentId]));
  let depth = 0;
  let current = parentById.get(id);
  const seen = new Set<string>([id]);
  while (current && !seen.has(current)) {
    seen.add(current);
    depth += 1;
    current = parentById.get(current) ?? null;
  }
  return depth;
}

function conceptTreeContains(concepts: readonly ThesaurusPaneConcept[], rootId: string, targetId: string): boolean {
  if (rootId === targetId) return true;
  const children = concepts.filter((concept) => concept.parentId === rootId);
  return children.some((child) => conceptTreeContains(concepts, child.id, targetId));
}

function thesaurusConceptMatches(concept: ThesaurusPaneConcept, query: string) {
  return concept.label.toLowerCase().includes(query) || concept.description.toLowerCase().includes(query);
}

function thesaurusTopLevelConcepts(concepts: readonly ThesaurusPaneConcept[]) {
  const ids = new Set(concepts.map((concept) => concept.id));
  return concepts.filter((concept) => concept.parentId === null || !ids.has(concept.parentId));
}

function elementTypeRank(type: string, family: string) {
  const direct = ELEMENT_TYPE_ORDER.indexOf(type);
  if (direct >= 0) return direct;
  const familyRank = ELEMENT_TYPE_ORDER.indexOf(family);
  return familyRank >= 0 ? familyRank + 0.5 : ELEMENT_TYPE_ORDER.length;
}

function formatCompactCount(value: number): string {
  if (value >= 1_000_000) return `${trimCompactNumber(value / 1_000_000)}M`;
  if (value >= 1_000) return `${trimCompactNumber(value / 1_000)}K`;
  return value.toLocaleString();
}

function trimCompactNumber(value: number): string {
  return value >= 10 ? Math.round(value).toString() : value.toFixed(1).replace(/\.0$/, "");
}

function searchKindLabel(kind: SearchKind) {
  const labels: Record<SearchKind, string> = {
    file: "Files",
    element: "Elements",
    resource: "Resources",
    ontology: "Ontology terms",
  };
  return labels[kind];
}

function searchKindColorToken(kind: SearchKind): DesignSystemColorToken {
  const colors: Record<SearchKind, DesignSystemColorToken> = {
    file: "--resource",
    element: "--requirement",
    resource: "--ontology",
    ontology: "--rdf-resource",
  };
  return colors[kind];
}

function ontologyColorToken(value: string): DesignSystemColorToken {
  const colors: Record<string, DesignSystemColorToken> = {
    class: "--rdf-class",
    "object-property": "--rdf-objprop",
    "datatype-property": "--rdf-dtprop",
    "rdf-property": "--rdf-rdfprop",
    property: "--rdf-objprop",
    "named-individual": "--rdf-individual",
    "skos-concept": "--rdf-concept",
    "skos-concept-scheme": "--rdf-concept-scheme",
    datatype: "--rdf-datatype",
    restriction: "--rdf-restriction",
    "class-expression": "--rdf-classexpr",
    "node-shape": "--rdf-nodeshape",
    "property-shape": "--rdf-propshape",
    resource: "--rdf-resource",
    relation: "--edge-default",
  };
  return colors[value] ?? colors.resource;
}

function ontologyLayerColorToken(value: string): DesignSystemColorToken {
  const colors: Record<string, DesignSystemColorToken> = {
    "layer-authored": "--ontology",
    "layer-concepts": "--rdf-concept",
    "layer-reqvire-context": "--info",
    "layer-external-source": "--other",
  };
  return colors[value] ?? "--text-muted";
}

function ontologyLayerDescription(value: string): string {
  const descriptions: Record<string, string> = {
    "layer-authored": "Authored OWL/RDFS/SHACL structural ontology nodes and projection edges.",
    "layer-concepts": "Curated SKOS concept nodes, concept taxonomy edges, and mapsToConcept bridge edges.",
    "layer-reqvire-context": "Semantic context: model elements that declare or reference ontology terms.",
    "layer-external-source": "Used external ontology subset triples derived from declared external sources.",
  };
  return descriptions[value] ?? "";
}
