import { useMemo, type ReactNode } from "react";
import { Badge, Text, Tooltip } from "@radix-ui/themes";
import {
  ActivityLogIcon,
  ArchiveIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
  CubeIcon,
  FileIcon,
  GridIcon,
  ListBulletIcon,
  PieChartIcon,
  RowsIcon,
  TableIcon,
} from "@radix-ui/react-icons";
import { useStore } from "../store/StoreContext";
import type { ViewId } from "../router/routes";
import type { ProjectStoreElement, ProjectStoreFile } from "../store/types";
import {
  MODEL_ROLE_TYPES,
  KN2_RELATIONS,
  ONTOLOGY_SHOW_FILTERS,
  SEARCH_KINDS,
  useExplorerUiState,
  type GraphOverlayKey,
  type Kn2ClusterMode,
  type Kn2LayoutMode,
  type Kn2RelationCategory,
  type ModelMode,
  type SearchKind,
  type TraceMode,
} from "./ExplorerUiState";

interface ExplorerSidePaneProps {
  activeView: ViewId;
  open: boolean;
  onToggle: () => void;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
}

interface TreeFolder {
  path: string;
  name: string;
  folders: TreeFolder[];
  files: ProjectStoreFile[];
}

const ROOT_PATH = "__root__";

export function ExplorerSidePane({
  activeView,
  open,
  onToggle,
  onNavigate,
  onOpenElement,
}: ExplorerSidePaneProps) {
  const { store, elementById } = useStore();
  const tree = useMemo(() => buildFileTree(store.files), [store.files]);
  const showProjectTree = activeView === "model" || activeView === "files";

  return (
    <aside
      className={["explorer-side-pane", open ? "" : "is-collapsed"].join(" ")}
      aria-label="Explorer navigation"
    >
      <div className="explorer-side-content">
        <ExplorerViewControls activeView={activeView} onNavigate={onNavigate} />
        {showProjectTree && (
          <div className="explorer-tree" aria-label="Project tree">
            <TreeFolderNode
              folder={tree}
              elementById={elementById}
              onOpenElement={onOpenElement}
              depth={0}
            />
          </div>
        )}
      </div>
      <button
        type="button"
        className="explorer-tree-tab"
        aria-label={open ? "Collapse explorer pane" : "Expand explorer pane"}
        aria-expanded={open}
        onClick={onToggle}
      >
        <span className="explorer-tree-tab-label">Explorer</span>
        <span className="explorer-tree-tab-toggle" aria-hidden="true">
          {open ? <ChevronLeftIcon /> : <ChevronRightIcon />}
        </span>
      </button>
    </aside>
  );
}

function ExplorerViewControls({
  activeView,
  onNavigate,
}: {
  activeView: ViewId;
  onNavigate: (view: ViewId) => void;
}) {
  const ui = useExplorerUiState();

  if (activeView === "model" || activeView === "files") {
    function selectModelMode(value: string) {
      ui.setModelMode(value as ModelMode);
      if (activeView === "files") {
        onNavigate("model");
      }
    }

    return (
      <section className="explorer-pane-controls" aria-label="Model controls">
        <PaneTitle title="Model" />
        <ModeIconGroup
          items={[
            ["list", "List", <ListBulletIcon />],
            ["grid", "Grid", <GridIcon />],
            ["sunburst", "Sunburst", <PieChartIcon />],
            ["icicle", "Icicle", <RowsIcon />],
          ]}
          active={ui.modelMode}
          onSelect={selectModelMode}
        />
      </section>
    );
  }

  if (activeView === "knowledge-graph") {
    return (
      <section className="explorer-pane-controls" aria-label="Knowledge Graph controls">
        <PaneTitle title="Knowledge Graph" />
        <PaneSectionLabel label="Show" />
        {MODEL_ROLE_TYPES.map((type) => (
          <PaneToggle
            key={type}
            label={roleLabel(type)}
            active={ui.modelTypes.has(type)}
            color={roleColor(type)}
            onClick={() => ui.toggleModelType(type)}
          />
        ))}
        <PaneSectionLabel label="Overlays" />
        {[
          ["cross", "Attachments / concepts"],
          ["verification", "Verification / satisfy"],
          ["trace", "Trace"],
        ].map(([key, label]) => (
          <PaneToggle
            key={key}
            label={label}
            active={ui.modelOverlays.has(key as GraphOverlayKey)}
            line
            onClick={() => ui.toggleModelOverlay(key as GraphOverlayKey)}
          />
        ))}
      </section>
    );
  }

  if (activeView === "traces") {
    return (
      <section className="explorer-pane-controls" aria-label="Trace controls">
        <PaneTitle title="Traces" />
        <ModeIconGroup
          items={[
            ["flow", "Flow", <ActivityLogIcon />],
            ["rows", "Rows", <TableIcon />],
          ]}
          active={ui.traceMode}
          onSelect={(value) => ui.setTraceMode(value as TraceMode)}
        />
        <PaneLegend
          title="Legend"
          rows={[
            ["files", "#00897b"],
            ["verifications", "#4caf50"],
            ["requirements", "#673ab7"],
          ]}
        />
      </section>
    );
  }

  if (activeView === "search") {
    return (
      <section className="explorer-pane-controls" aria-label="Search controls">
        <PaneTitle title="Search" />
        <button
          type="button"
          className="explorer-mode-link"
          onClick={ui.resetSearchKinds}
        >
          Reset filters
        </button>
        <PaneSectionLabel label="Result types" />
        {SEARCH_KINDS.map((kind) => (
          <PaneToggle
            key={kind}
            label={searchKindLabel(kind)}
            active={ui.searchKinds.has(kind)}
            color={searchKindColor(kind)}
            onClick={() => ui.toggleSearchKind(kind)}
          />
        ))}
        <PaneLegend
          title="Legend"
          rows={SEARCH_KINDS.map((kind) => [searchKindLabel(kind), searchKindColor(kind)])}
        />
      </section>
    );
  }

  if (activeView === "ontologies") {
    return (
      <section className="explorer-pane-controls" aria-label="Ontology controls">
        <PaneTitle title="Ontologies" />
        <button
          type="button"
          className="explorer-mode-link"
          onClick={() =>
            (window as typeof window & { resetOntologyGraphLayout?: () => void })
              .resetOntologyGraphLayout?.()
          }
        >
          Reset layout
        </button>
        <PaneSectionLabel label="Show" />
        {ONTOLOGY_SHOW_FILTERS.map(([category, value, label, swatch]) => (
          <OntologyFilterToggle
            key={value}
            category={category}
            value={value}
            label={label}
            active={ui.ontologyFilters.has(value)}
            swatch={swatch}
            onClick={() => ui.toggleOntologyFilter(value)}
          />
        ))}
        <PaneSectionLabel label="Types" />
        <PaneVisualLegend
          rows={[
            ["class", "Class"],
            ["named-individual", "Individual"],
            ["datatype", "Datatype"],
            ["restriction", "Restriction"],
            ["class-expression", "Class expr."],
            ["node-shape", "Node shape"],
            ["property-shape", "Property shape"],
            ["resource", "Resource"],
          ]}
        />
        <div className="explorer-pane-legend-row">
          <span className="graph-line-swatch" />
          <Text size="1">Relation</Text>
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
            ["∀", "Restriction"],
            ["∩", "Class expr."],
            ["SH", "SHACL overlay"],
          ]}
        />
      </section>
    );
  }

  if (activeView === "kn2") {
    return (
      <section className="explorer-pane-controls" aria-label="KN2 controls">
        <PaneTitle title="KN2" />
        <PaneSectionLabel label="Layout" />
        <ModeIconGroup
          items={[
            ["structural", "CoSE structural", <ActivityLogIcon />],
            ["concentric", "Concentric", <PieChartIcon />],
            ["breadthfirst", "Breadthfirst", <RowsIcon />],
            ["circle", "Circle", <PieChartIcon />],
            ["grid", "Grid", <GridIcon />],
          ]}
          active={ui.kn2LayoutMode}
          onSelect={(value) => ui.setKn2LayoutMode(value as Kn2LayoutMode)}
        />
        <PaneSectionLabel label="Clusters" />
        {[
          ["structural", "Structural islands"],
          ["modularity", "Modularity-style"],
        ].map(([value, label]) => (
          <PaneToggle
            key={value}
            label={label}
            active={ui.kn2ClusterMode === value}
            onClick={() => ui.setKn2ClusterMode(value as Kn2ClusterMode)}
          />
        ))}
        <PaneSectionLabel label="Focus" />
        <label className="graph-slider-control">
          <span>
            Selection radius <strong>{ui.kn2FocusRadius}</strong>
          </span>
          <input
            type="range"
            min="1"
            max="4"
            step="1"
            value={ui.kn2FocusRadius}
            onChange={(event) => ui.setKn2FocusRadius(Number(event.target.value))}
          />
        </label>
        <label className="graph-check-control">
          <input
            type="checkbox"
            checked={ui.kn2FocusOnly}
            onChange={(event) => ui.setKn2FocusOnly(event.target.checked)}
          />
          Show focus only
        </label>
        <PaneSectionLabel label="Relations" />
        {KN2_RELATIONS.map((relation) => (
          <label key={relation} className="graph-check-control">
            <input
              type="checkbox"
              checked={ui.kn2Relations.has(relation)}
              onChange={() => ui.toggleKn2Relation(relation as Kn2RelationCategory)}
            />
            {relation}
          </label>
        ))}
        <PaneSectionLabel label="Overlays" />
        {[
          ["cross", "Cross-subgraph overlays"],
          ["verification", "Verification / satisfy"],
          ["trace", "Trace"],
        ].map(([overlay, label]) => (
          <label key={overlay} className="graph-check-control">
            <input
              id={`kn2-${overlay === "cross" ? "cross-subgraph" : overlay}-overlay`}
              type="checkbox"
              checked={ui.kn2Overlays.has(overlay as GraphOverlayKey)}
              onChange={() => ui.toggleKn2Overlay(overlay as GraphOverlayKey)}
            />
            {label}
          </label>
        ))}
        <PaneSectionLabel label="Display" />
        <label className="graph-check-control">
          <input
            type="checkbox"
            checked={ui.kn2LabelsEnabled}
            onChange={(event) => ui.setKn2LabelsEnabled(event.target.checked)}
          />
          Labels
        </label>
      </section>
    );
  }

  return null;
}

function TreeFolderNode({
  folder,
  elementById,
  onOpenElement,
  depth,
}: {
  folder: TreeFolder;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onOpenElement: (id: string) => void;
  depth: number;
}) {
  const open = depth < 2;
  return (
    <details className="explorer-tree-node" open={open}>
      <summary className="explorer-tree-row" style={{ paddingLeft: 8 + depth * 14 }}>
        <ArchiveIcon className="explorer-tree-icon file-kind-folder" />
        <span className="explorer-tree-label">{folder.name}</span>
        <Badge color="gray">{folder.files.length + folder.folders.length}</Badge>
      </summary>
      {folder.folders.map((child) => (
        <TreeFolderNode
          key={child.path}
          folder={child}
          elementById={elementById}
          onOpenElement={onOpenElement}
          depth={depth + 1}
        />
      ))}
      {folder.files.map((file) => (
        <TreeFileNode
          key={file.path}
          file={file}
          elementById={elementById}
          onOpenElement={onOpenElement}
          depth={depth + 1}
        />
      ))}
    </details>
  );
}

function TreeFileNode({
  file,
  elementById,
  onOpenElement,
  depth,
}: {
  file: ProjectStoreFile;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onOpenElement: (id: string) => void;
  depth: number;
}) {
  const elements = file.element_ids.map(elementById).filter(Boolean) as ProjectStoreElement[];
  const showElementChildren = elements.length > 1;

  return (
    <details className="explorer-tree-node" open={showElementChildren}>
      <summary className="explorer-tree-row" style={{ paddingLeft: 8 + depth * 14 }}>
        <FileIcon className="explorer-tree-icon file-kind-file" />
        <a className="explorer-tree-link" href={`#/files/${file.path}`}>
          {displayName(file.display_path || file.path)}
        </a>
        {elements.length > 0 && <Badge color="gray">{elements.length}</Badge>}
      </summary>
      {showElementChildren &&
        elements.map((element) => (
          <button
            key={element.id}
            type="button"
            className="explorer-tree-row explorer-tree-element-row"
            style={{ paddingLeft: 8 + (depth + 1) * 14 }}
            onClick={() => onOpenElement(element.id)}
          >
            <CubeIcon className="explorer-tree-icon file-kind-element" />
            <span className="explorer-tree-label">{element.name}</span>
          </button>
        ))}
    </details>
  );
}

function PaneTitle({ title }: { title: string }) {
  return (
    <Text size="2" weight="bold">
      {title}
    </Text>
  );
}

function PaneSectionLabel({ label }: { label: string }) {
  return (
    <Text size="1" color="gray" weight="bold" className="explorer-pane-section-label">
      {label}
    </Text>
  );
}

function ModeIconGroup<T extends string>({
  items,
  active,
  onSelect,
}: {
  items: [T, string, ReactNode][];
  active: T;
  onSelect: (value: T) => void;
}) {
  return (
    <div className="explorer-mode-icon-strip" role="toolbar" aria-label="View mode">
      {items.map(([value, label, icon]) => (
        <Tooltip key={value} content={label} side="bottom">
          <button
            type="button"
            className={["explorer-mode-icon-button", active === value ? "is-active" : ""].join(" ")}
            aria-label={label}
            aria-pressed={active === value}
            onClick={() => onSelect(value)}
          >
            {icon}
          </button>
        </Tooltip>
      ))}
    </div>
  );
}

function PaneToggle({
  label,
  active,
  color,
  line,
  onClick,
}: {
  label: string;
  active: boolean;
  color?: string;
  line?: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      aria-pressed={active}
      className={["explorer-mode-link", active ? "is-active" : ""].join(" ")}
      onClick={onClick}
    >
      <span
        className={line ? "graph-line-swatch" : "graph-control-swatch"}
        style={color ? { backgroundColor: color, borderColor: color } : undefined}
      />
      {label}
    </button>
  );
}

function OntologyFilterToggle({
  category,
  value,
  label,
  active,
  swatch,
  onClick,
}: {
  category: "role" | "relation";
  value: string;
  label: string;
  active: boolean;
  swatch: string;
  onClick: () => void;
}) {
  const usesLine = category === "relation" && !swatch.includes("property");
  return (
    <button
      type="button"
      aria-pressed={active}
      data-filter-category={category}
      data-filter-value={value}
      className={[
        "explorer-mode-link",
        "ontology-filter-toggle",
        active ? "is-active" : "",
      ].join(" ")}
      onClick={onClick}
    >
      {usesLine ? (
        <span className="graph-line-swatch" />
      ) : (
        <span
          className="graph-control-swatch"
          style={{
            backgroundColor: ontologyColor(swatch),
            borderColor: ontologyColor(swatch),
          }}
        />
      )}
      {label}
    </button>
  );
}

function PaneLegend({ title, rows }: { title: string; rows: [string, string][] }) {
  return (
    <div className="explorer-pane-legend">
      <PaneSectionLabel label={title} />
      {rows.map(([label, color]) => (
        <div key={label} className="explorer-pane-legend-row">
          <span className="graph-control-swatch" style={{ backgroundColor: color, borderColor: color }} />
          <Text size="1">{label}</Text>
        </div>
      ))}
    </div>
  );
}

function PaneVisualLegend({ rows }: { rows: [string, string][] }) {
  return (
    <div className="explorer-pane-legend">
      {rows.map(([kind, label]) => (
        <div key={kind} className="explorer-pane-legend-row">
          <span
            className="graph-control-swatch"
            style={{
              backgroundColor: ontologyColor(kind),
              borderColor: ontologyColor(kind),
            }}
          />
          <Text size="1">{label}</Text>
        </div>
      ))}
    </div>
  );
}

function PaneNotationLegend({ rows }: { rows: [string, string][] }) {
  return (
    <div className="explorer-pane-legend">
      {rows.map(([symbol, label]) => (
        <div key={symbol} className="explorer-pane-legend-row">
          <span className="explorer-pane-symbol">{symbol}</span>
          <Text size="1">{label}</Text>
        </div>
      ))}
    </div>
  );
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

function roleLabel(value: string) {
  return humanize(value);
}

function roleColor(value: string) {
  const colors: Record<string, string> = {
    capability: "#1976D2",
    requirement: "#673AB7",
    refinement: "#673AB7",
    verification: "#4CAF50",
    ontology: "#B08A00",
    resource: "#FFCA28",
    other: "#424242",
  };
  return colors[value] ?? colors.other;
}

function searchKindLabel(kind: SearchKind) {
  const labels: Record<SearchKind, string> = {
    file: "Files",
    element: "Elements",
    resource: "Resources",
    ontology: "Ontology",
    trace: "Traces",
    coverage: "Coverage",
  };
  return labels[kind];
}

function searchKindColor(kind: SearchKind) {
  const colors: Record<SearchKind, string> = {
    file: "#52605b",
    element: "#673ab7",
    resource: "#b08a00",
    ontology: "#00897b",
    trace: "#6d6258",
    coverage: "#4caf50",
  };
  return colors[kind];
}

function ontologyColor(value: string) {
  const colors: Record<string, string> = {
    class: "#2f6fa7",
    "object-property": "#65745f",
    "datatype-property": "#8f7a22",
    property: "#6f786a",
    "named-individual": "#76579a",
    datatype: "#b89422",
    restriction: "#4d7f88",
    "class-expression": "#5f6a9a",
    "node-shape": "#a24b4b",
    "property-shape": "#b25f54",
    resource: "#8b8f84",
  };
  return colors[value] ?? colors.resource;
}
