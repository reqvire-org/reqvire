import { useEffect, useMemo, useState, type FormEvent } from "react";
import {
  Button,
  ElementIcon,
  Icon,
  IconButton,
  SearchInput,
  SidebarSection,
  Stat,
  StatRow,
  ToggleRow,
  TreeItem,
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
import {
  SEARCH_KINDS,
  useExplorerUiState,
  type SearchKind,
} from "./ExplorerUiState";
import { PaneChromeHeader, ReqvireRailMark } from "./PaneChrome";
import { buildTraceFiles, type TraceFileNode } from "../lib/traces";

interface ExplorerSidePaneProps {
  activeView: ViewId;
  open: boolean;
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
  onToggle,
  onNavigate,
  onOpenElement,
  onOpenOntologyNode,
}: ExplorerSidePaneProps) {
  const { store, elementById } = useStore();
  const ui = useExplorerUiState();
  const tree = useMemo(() => buildFileTree(store.files), [store.files]);
  const traceTree = useMemo(() => buildTraceFileTree(buildTraceFiles(store)), [store]);
  const graphModelActive = activeView === "model" && ui.modelMode === "graph";
  const showProjectTree = (activeView === "model" || activeView === "files") && !graphModelActive;
  const title = graphModelActive ? "Graph Explorer" : `${VIEW_TITLES[activeView]} Explorer`;

  return (
    <aside
      className={["ex-side-pane", open ? "" : "is-collapsed"].join(" ")}
      aria-label="Explorer navigation"
    >
      <div className="ex-side-content">
        <PaneChromeHeader title={title} />
        {activeView === "ontologies" && <OntologyGraphSearch />}
        <ExplorerViewControls
          activeView={activeView}
          onOpenElement={onOpenElement}
          onOpenOntologyNode={onOpenOntologyNode}
        />
        {activeView === "traces" && (
          <div className="ex-tree rq-tree" aria-label="Verification trace tree">
            <TraceTreeFolderNode folder={traceTree} depth={0} />
          </div>
        )}
        {showProjectTree && (
          <div className="ex-tree rq-tree" aria-label="Project tree">
            <TreeFolderNode
              folder={tree}
              activeView={activeView}
              elementById={elementById}
              onNavigate={onNavigate}
              onOpenElement={onOpenElement}
              depth={0}
            />
          </div>
        )}
      </div>
      <button
        type="button"
        className="ex-tree-tab"
        aria-label={open ? "Collapse explorer pane" : "Expand explorer pane"}
        aria-expanded={open}
        onClick={onToggle}
      >
        <ReqvireRailMark />
        <span className="ex-tree-tab-label">Explorer</span>
        <span className="ex-tree-tab-toggle" aria-hidden="true">
          {open ? <Icon name="chevron-left" /> : <Icon name="chevron-right" />}
        </span>
      </button>
    </aside>
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
    <form className="ex-global-search" role="search" onSubmit={submitSearch}>
      <SearchInput
        id="ontology-graph-search"
        className="ex-global-search-control"
        size="lg"
        aria-label="Search Explorer"
        type="search"
        placeholder="Search ontology graph..."
        value={query}
        onChange={(event) => setQuery(event.target.value)}
      />
      <ul id="ontology-graph-results" className="ontology-graph-results ex-global-search-results" />
    </form>
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
      <section className="ex-pane-controls" aria-label="Graph controls">
        <SidebarSection title="Summary" className="ex-pane-summary" aria-label="Summary">
          <StatRow className="ex-summary">
            <Stat label="Submodels" value={formatSummaryValue(store.knowledge_graph.summary?.submodels ?? store.knowledge_graph.submodels?.length ?? 0)} />
            <Stat label="Elements" value={formatSummaryValue(store.knowledge_graph.summary?.elements ?? store.elements.length)} />
            <Stat label="Relations" value={formatSummaryValue(store.knowledge_graph.summary?.relations ?? store.relations.length)} />
            <Stat label="Attachments" value={formatSummaryValue(store.knowledge_graph.summary?.attachments ?? store.attachments.length)} />
          </StatRow>
        </SidebarSection>
        <KnowledgeGraphSelectedElementLink
          selectedNodeId={ui.knowledgeGraphSelectionId}
          nodes={store.knowledge_graph.nodes ?? []}
          elementById={elementById}
          onOpenElement={onOpenElement}
        />
        <PaneSectionLabel label="Show" />
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
      </section>
    );
  }

  if (activeView === "traces") return null;

  if (activeView === "coverage") {
    const coverageItems = buildCoveragePaneItems(store);
    return (
      <section className="ex-pane-controls" aria-label="Coverage explorer">
        <PaneSectionLabel label="Coverage" />
        <div className="ex-pane-nav-list">
          {coverageItems.map((item) => (
            <button
              key={item.id}
              type="button"
              className="ex-pane-nav-row"
              onClick={() => navigateCoverageSection(item.id)}
            >
              <span className="ex-pane-nav-row__icon" aria-hidden="true">
                <Icon name={item.icon} />
              </span>
              <span className="ex-pane-nav-row__label">{item.label}</span>
              <span className="ex-pane-nav-row__count">{formatCompactCount(item.count)}</span>
            </button>
          ))}
        </div>
      </section>
    );
  }

  if (activeView === "search") {
    return (
      <section className="ex-pane-controls" aria-label="Search controls">
        <h2 className="ex-pane-controls-title">Filter by</h2>
        <Button size="sm" onClick={ui.resetSearchKinds}>
          Reset filters
        </Button>
        <PaneSectionLabel label="Result types" />
        {SEARCH_KINDS.map((kind) => (
          <ToggleRow
            key={kind}
            label={searchKindLabel(kind)}
            on={ui.searchKinds.has(kind)}
            color={searchKindColor(kind)}
            meta={formatCompactCount(searchKindCounts[kind] ?? 0)}
            onToggle={() => ui.toggleSearchKind(kind)}
          />
        ))}
        {searchElementTypeOptions.length > 0 ? (
          <>
            <PaneSectionLabel label="Element types" />
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
          </>
        ) : null}
      </section>
    );
  }

  if (activeView === "ontologies") {
    const summary = store.ontology.summary ?? {};
    return (
      <section className="ex-pane-controls" aria-label="Ontology controls">
        <SidebarSection title="Summary" className="ex-pane-summary" aria-label="Summary">
          <StatRow className="ex-summary">
            <Stat label="Ontologies" value={formatSummaryValue(summary.ontology_blocks ?? 0)} />
            <Stat label="Shapes" value={formatSummaryValue(summary.shape_blocks ?? 0)} />
            <Stat
              label="Quads"
              value={formatSummaryValue(summary.total_quads ?? 0)}
              title="RDF statements (subject-predicate-object, with graph context)"
            />
            <Stat
              label="Blocks"
              value={formatSummaryValue(summary.total_blocks ?? 0)}
              title="Ontology and shape source blocks discovered in the model"
            />
          </StatRow>
        </SidebarSection>
        <OntologySelectedNodeLink
          selectedNodeId={ui.ontologySelectionId}
          nodes={store.ontology.graph_data?.nodes ?? []}
          onOpenOntologyNode={onOpenOntologyNode}
          onClear={() => {
            ui.setOntologySelectionId(null);
            window.clearOntologySelection?.();
          }}
        />
        <PaneSectionLabel label="Graph" />
        <div className="ex-pane-action-row">
          {store.ontology.ttl_href && (
            <a
              href={store.ontology.ttl_href}
              className="rq-btn rq-btn--ghost rq-btn--sm"
              title="Download the exported ontology as Turtle (ontologies.ttl)"
            >
              <span className="rq-btn__icon" aria-hidden="true">
                <Icon name="download" />
              </span>
              Download .ttl
            </a>
          )}
          <button
            type="button"
            className="rq-btn rq-btn--ghost rq-btn--sm"
            onClick={() =>
              (window as typeof window & { resetOntologyGraphLayout?: () => void })
                .resetOntologyGraphLayout?.()
            }
          >
            <span className="rq-btn__icon" aria-hidden="true">
              <Icon name="rotate-ccw" />
            </span>
            Reset layout
          </button>
        </div>
        <PaneSectionLabel label="Types" />
        <PaneVisualLegend
          rows={[
            ["class", "Class"],
            ["named-individual", "Individual"],
            ["datatype", "Datatype"],
            ["class-expression", "Class expr."],
            ["node-shape", "Node shape"],
            ["property-shape", "Property shape"],
            ["resource", "Resource"],
          ]}
        />
        <div className="ex-pane-legend-row">
          <span className="graph-line-swatch" />
          <span className="ex-pane-legend-text">Relation</span>
        </div>
        <PaneSectionLabel label="Notation" />
        <PaneNotationLegend
          rows={[
            ["D/R", "Domain/range"],
            ["⊆", "Subclass"],
            ["∈", "Membership"],
            ["⟂", "Disjoint"],
            ["⇔", "Equivalence"],
            ["⟲", "Inverse"],
            ["∘", "Property chain"],
            ["∩", "Class expr."],
            ["SH", "SHACL overlay"],
          ]}
        />
      </section>
    );
  }

  return null;
}

function KnowledgeGraphSelectedElementLink({
  selectedNodeId,
  nodes,
  elementById,
  onOpenElement,
}: {
  selectedNodeId: string | null;
  nodes: KnowledgeGraphNode[];
  elementById: (id: string) => ProjectStoreElement | undefined;
  onOpenElement: (id: string) => void;
}) {
  if (!selectedNodeId) return null;
  const node = nodes.find((candidate) => candidate.id === selectedNodeId);
  if (!node?.identifier) return null;
  const element = elementById(node.identifier);
  if (!element) return null;

  return (
    <section className="ex-pane-selected-element" aria-label="Selected graph element">
      <PaneSectionLabel label="Element" />
      <button
        type="button"
        className="rq-relation__target ex-pane-selected-element-link"
        onClick={() => onOpenElement(element.id)}
      >
        <ElementTypeGlyph element={element} />
        <span>{element.name}</span>
      </button>
    </section>
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
    <section className="ex-pane-selected-element" aria-label="Selected ontology node">
      <PaneSectionLabel label="Selection" />
      {!node ? (
        <p className="ex-empty ex-pane-selection-hint">
          Select a graph node to inspect its details.
        </p>
      ) : (
        <div className="ex-pane-selection-row">
          <button
            type="button"
            className="rq-relation__target ex-pane-selected-element-link"
            onClick={() => onOpenOntologyNode(node.id)}
            title="Open node details"
          >
            <span
              className="graph-control-swatch"
              style={{ backgroundColor: ontologyColor(kind), borderColor: ontologyColor(kind) }}
            />
            <span className="ex-pane-selection-name">{node.label || node.id}</span>
            <span className="ex-pane-selection-kind">{kind}</span>
            <Icon name="arrow-up-right" size={13} className="ex-pane-selection-open" />
          </button>
          <IconButton size="sm" tone="ghost" aria-label="Clear selection" title="Clear selection" onClick={onClear}>
            <Icon name="x" />
          </IconButton>
        </div>
      )}
    </section>
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
    <div className="ex-tree-node">
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
    </div>
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
    <div className="ex-tree-node">
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
    </div>
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
    <div className="ex-tree-node">
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
    </div>
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
    <div className="ex-tree-node">
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
          icon={<ElementTypeGlyph element={element} />}
          depth={depth + 1}
          selected={ui.modelSelectionId === element.id}
          onSelect={() => selectElement(element.id)}
        />
      ))}
    </div>
  );
}

export function ElementTypeGlyph({ element }: { element: ProjectStoreElement }) {
  return (
    <ElementIcon
      type={element.element_type}
      family={element.type_family}
      title={element.element_type}
      size="sm"
    />
  );
}

function PaneSectionLabel({ label }: { label: string }) {
  return (
    <span className="ex-pane-section-label">
      {label}
    </span>
  );
}

function PaneVisualLegend({ rows }: { rows: [string, string][] }) {
  return (
    <div className="ex-pane-legend">
      {rows.map(([kind, label]) => (
        <div key={kind} className="rq-togglerow rq-togglerow--static ex-pane-legend-row">
          <span
            className="rq-togglerow__swatch"
            style={{
              backgroundColor: ontologyColor(kind),
              borderColor: ontologyColor(kind),
            }}
          />
          <span className="rq-togglerow__label">{label}</span>
        </div>
      ))}
    </div>
  );
}

function PaneNotationLegend({ rows }: { rows: [string, string][] }) {
  return (
    <div className="ex-pane-legend">
      {rows.map(([symbol, label]) => (
        <div key={symbol} className="ex-pane-legend-row">
          <span className="ex-pane-symbol">{symbol}</span>
          <span className="ex-pane-legend-text">{label}</span>
        </div>
      ))}
    </div>
  );
}

function formatSummaryValue(value: string | number) {
  return typeof value === "number" ? value.toLocaleString() : value;
}

function buildFileTree(files: ProjectStoreFile[]): TreeFolder {
  const root: TreeFolder = { path: ROOT_PATH, name: "Project", folders: [], files: [] };
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

function buildTraceFileTree(files: TraceFileNode[]): TracePaneFolder {
  const root: TracePaneFolder = { path: ROOT_PATH, name: "Project", folders: [], files: [] };
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
  "refinement",
  "verification",
  "test-verification",
  "analysis-verification",
  "inspection-verification",
  "demonstration-verification",
  "specification",
  "semantic-contract",
  "semantic-query-contract",
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

function searchKindColor(kind: SearchKind) {
  const colors: Record<SearchKind, string> = {
    file: "var(--resource)",
    element: "var(--requirement)",
    resource: "var(--ontology)",
    ontology: "var(--rdf-resource)",
  };
  return colors[kind];
}

function ontologyColor(value: string) {
  const colors: Record<string, string> = {
    class: "var(--rdf-class)",
    "object-property": "var(--rdf-objprop)",
    "datatype-property": "var(--rdf-dtprop)",
    "rdf-property": "var(--rdf-rdfprop)",
    property: "var(--rdf-objprop)",
    "named-individual": "var(--rdf-individual)",
    datatype: "var(--rdf-datatype)",
    restriction: "var(--rdf-restriction)",
    "class-expression": "var(--rdf-classexpr)",
    "node-shape": "var(--rdf-nodeshape)",
    "property-shape": "var(--rdf-propshape)",
    resource: "var(--rdf-resource)",
    relation: "var(--edge-default)",
  };
  return colors[value] ?? colors.resource;
}
