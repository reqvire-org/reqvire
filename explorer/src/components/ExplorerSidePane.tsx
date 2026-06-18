import { useEffect, useMemo, useState, type FormEvent } from "react";
import {
  Button,
  ElementIcon,
  Icon,
  PaneActionRow,
  PaneChromeHeader,
  PaneFilterGroup,
  PaneFilterNavList,
  PaneFilterNavRow,
  PaneFilterSection,
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
  ToggleRow,
  TreeItem,
  type DesignSystemColorToken,
} from "@ds";
import { useStore } from "../store/StoreContext";
import { VIEW_TITLES, type ViewId } from "../router/routes";
import type {
  ExplorerProjectStore,
  KnowledgeGraphNode,
  OntologyGraphNode,
  ProjectStoreElement,
  ProjectStoreFile,
} from "../store/types";
import { useExplorerUiState } from "../state/ExplorerUiState";
import { SEARCH_KINDS, type SearchKind } from "../search/searchKinds";
import { buildTraceFiles, type TraceFileNode } from "../lib/traces";

interface ExplorerSidePaneProps {
  activeView: ViewId;
  open: boolean;
  chrome?: "standalone" | "app";
  onToggle: () => void;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
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

type CoverageSectionId =
  | "overview"
  | "capability-coverage"
  | "unverified-requirements"
  | "unimplemented-requirements"
  | "unsatisfied-verifications"
  | "orphaned-verifications";

export function ExplorerSidePane({
  activeView,
  open,
  chrome = "standalone",
  onToggle,
  onNavigate,
  onOpenElement,
  onOpenOntologyNode,
}: ExplorerSidePaneProps) {
  const { store, elementById } = useStore();
  const ui = useExplorerUiState();
  const projectRootLabel = projectTreeRootLabel(store);
  const tree = useMemo(() => buildFileTree(store.files, projectRootLabel), [projectRootLabel, store.files]);
  const traceTree = useMemo(() => buildTraceFileTree(buildTraceFiles(store), projectRootLabel), [projectRootLabel, store]);
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
      <ExplorerViewControls
        activeView={activeView}
        onOpenElement={onOpenElement}
        onOpenOntologyNode={onOpenOntologyNode}
      />
      {activeView === "traces" && (
        <PaneTree aria-label="Verification trace tree">
          <TraceTreeFolderNode folder={traceTree} depth={0} />
        </PaneTree>
      )}
      {showProjectTree && (
        <PaneTree aria-label="Project tree">
          <TreeFolderNode
            folder={tree}
            activeView={activeView}
            elementById={elementById}
            onNavigate={onNavigate}
            onOpenElement={onOpenElement}
            depth={0}
          />
        </PaneTree>
      )}
    </SidePaneFrame>
  );
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
    return (
      <PaneFilterSection aria-label="Graph controls">
        <PaneSummary
          items={[
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
              label: "Attachments",
              value: formatSummaryValue(store.knowledge_graph.summary?.attachments ?? store.attachments.length),
            },
          ]}
        />
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
              icon={<ElementIcon type={option.type} family={option.family} size="sm" />}
              meta={formatCompactCount(option.count)}
              onToggle={() => ui.toggleModelType(option.type)}
            />
          ))}
        </PaneFilterGroup>
      </PaneFilterSection>
    );
  }

  if (activeView === "traces") return null;

  if (activeView === "coverage") {
    const coverageItems = buildCoveragePaneItems(store);
    return (
      <PaneFilterSection aria-label="Coverage explorer">
        <PaneFilterGroup label="Coverage">
          <PaneFilterNavList>
            {coverageItems.map((item) => (
              <PaneFilterNavRow
                key={item.id}
                icon={item.icon}
                label={item.label}
                count={formatCompactCount(item.count)}
                onClick={() => navigateCoverageSection(item.id)}
              />
            ))}
          </PaneFilterNavList>
        </PaneFilterGroup>
      </PaneFilterSection>
    );
  }

  if (activeView === "search") {
    return (
      <PaneFilterSection aria-label="Search controls" title="Filter by">
        <Button size="sm" onClick={ui.resetSearchKinds}>
          Reset filters
        </Button>
        <PaneFilterGroup label="Result types">
          {SEARCH_KINDS.map((kind) => (
            <ToggleRow
              key={kind}
              label={searchKindLabel(kind)}
              on={ui.searchKinds.has(kind)}
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
                icon={<ElementIcon type={option.type} family={option.family} size="sm" />}
                meta={formatCompactCount(option.count)}
                onToggle={() => ui.toggleSearchElementType(option.type)}
              />
            ))}
          </PaneFilterGroup>
        ) : null}
      </PaneFilterSection>
    );
  }

  if (activeView === "ontologies") {
    const summary = store.ontology.summary ?? {};
    return (
      <PaneFilterSection aria-label="Ontology controls">
        <PaneSummary
          items={[
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
          ]}
        />
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
        <PaneFilterGroup label="Types">
          <PaneLegend
            rows={[
              { id: "class", label: "Class", colorToken: ontologyColorToken("class") },
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
      </PaneFilterSection>
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
              icon: <ElementIcon type={kind} size="sm" />,
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
  depth,
}: {
  folder: TreeFolder;
  activeView: ViewId;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
  depth: number;
}) {
  const [open, setOpen] = useState(depth < 2);
  const ui = useExplorerUiState();
  const selectionId = folder.path === ROOT_PATH ? "__root__" : `folder:${folder.path}`;

  function selectFolder() {
    ui.setModelSelectionId(selectionId);
    if (activeView === "files") onNavigate("model");
  }

  return (
    <PaneTreeNode>
      <TreeItem
        kind="folder"
        label={folder.name}
        icon={open ? <Icon name="folder-open" className="file-kind-folder" /> : <Icon name="folder" className="file-kind-folder" />}
        count={folder.files.length + folder.folders.length}
        depth={depth}
        open={open}
        expandable={folder.files.length + folder.folders.length > 0}
        selected={ui.modelSelectionId === selectionId}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFolder}
      />
      {open && (
        <>
          {folder.folders.map((child) => (
            <TreeFolderNode
              key={child.path}
              folder={child}
              activeView={activeView}
              elementById={elementById}
              onNavigate={onNavigate}
              onOpenElement={onOpenElement}
              depth={depth + 1}
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
              depth={depth + 1}
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
}: {
  folder: TracePaneFolder;
  depth: number;
}) {
  const ui = useExplorerUiState();
  const selectedPath = ui.traceFilePath;
  const hasSelectedDescendant = selectedPath
    ? traceFolderContainsPath(folder, selectedPath)
    : false;
  const [open, setOpen] = useState(depth < 2 || hasSelectedDescendant);

  useEffect(() => {
    if (hasSelectedDescendant) setOpen(true);
  }, [hasSelectedDescendant]);

  return (
    <PaneTreeNode>
      <TreeItem
        kind="folder"
        label={folder.name}
        icon={open ? <Icon name="folder-open" className="file-kind-folder" /> : <Icon name="folder" className="file-kind-folder" />}
        count={traceFolderVerificationCount(folder)}
        depth={depth}
        open={open}
        expandable={folder.files.length + folder.folders.length > 0}
        selected={folder.path === ROOT_PATH && !selectedPath}
        onToggle={() => setOpen((value) => !value)}
        onSelect={() => {
          ui.setTraceFilePath(null);
          ui.setTraceSelectionId(null);
        }}
      />
      {open && (
        <>
          {folder.folders.map((child) => (
            <TraceTreeFolderNode key={child.path} folder={child} depth={depth + 1} />
          ))}
          {folder.files.map((file) => (
            <TraceTreeFileNode key={file.path} file={file} depth={depth + 1} />
          ))}
        </>
      )}
    </PaneTreeNode>
  );
}

function TraceTreeFileNode({
  file,
  depth,
}: {
  file: TracePaneFile;
  depth: number;
}) {
  const ui = useExplorerUiState();
  const selectedFile = ui.traceFilePath === file.path;
  const selectedVerification = selectedFile ? ui.traceSelectionId : null;
  const [open, setOpen] = useState(true);

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
        open={open}
        expandable={file.verifications.length > 0}
        selected={selectedFile && !selectedVerification}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFile}
      />
      {open && file.verifications.map((verification) => (
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
  depth,
}: {
  file: ProjectStoreFile;
  activeView: ViewId;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
  depth: number;
}) {
  const ui = useExplorerUiState();
  const elements = file.element_ids.map(elementById).filter(Boolean) as ProjectStoreElement[];
  const showElementChildren = elements.length > 0;
  const [open, setOpen] = useState(showElementChildren);
  const selectionId = `file:${file.path}`;

  function selectFile() {
    ui.setModelSelectionId(selectionId);
    if (activeView === "files") onNavigate("model");
  }

  function selectElement(elementId: string) {
    ui.setModelSelectionId(elementId);
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
        open={open}
        expandable={showElementChildren}
        selected={ui.modelSelectionId === selectionId}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFile}
      />
      {open && showElementChildren && elements.map((element) => (
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
