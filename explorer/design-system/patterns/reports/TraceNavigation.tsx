import { createContext } from "preact";
import { useContext, useEffect, useMemo, useRef, useState } from "preact/hooks";
import { Icon } from "../../components/core/Icon";
import { ElementIcon } from "../../components/data/ElementIcon";
import { TreeItem } from "../../components/navigation/TreeItem";
import { PaneSearchForm } from "../side-pane/PaneControls";
import { PaneTree, PaneTreeNode } from "../side-pane/PaneTree";
import { PaneSummary } from "../side-pane/PaneSummary";
import type { TraceOverviewFile } from "./TracesScreen";

const ROOT_PATH = "__root__";
const dirname = (path: string) => path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : "";
const displayName = (path: string) => path.split("/").pop() || path;
const formatSummaryValue = (value: number) => value.toLocaleString();
const textMatches = (query: string, ...values: Array<string | null | undefined>) => values.some(value => value?.toLowerCase().includes(query));

export interface TraceNavigationProps {
  files: readonly TraceOverviewFile[];
  rootLabel: string;
  traceTreeQuery: string;
  traceFilePath: string | null;
  traceSelectionId: string | null;
  setTraceTreeQuery: (query: string) => void;
  setTraceFilePath: (path: string | null) => void;
  setTraceSelectionId: (id: string) => void;
}
const Context = createContext<TraceNavigationProps | null>(null);
const useTraceNavigation = () => useContext(Context)!;

export function TraceNavigation(props: TraceNavigationProps) {
  const tree = useMemo(() => buildTraceFileTree(props.files, props.rootLabel), [props.files, props.rootLabel]);
  const filtered = useMemo(() => filterTraceFileTree(tree, props.traceTreeQuery), [tree, props.traceTreeQuery]);
  return <Context.Provider value={props}>
    <PaneSearchForm searchInputId="trace-tree-search" inputLabel="Filter trace tree"
      placeholder="Filter trace tree..." value={props.traceTreeQuery} resultsId="trace-tree"
      onQueryChange={props.setTraceTreeQuery} onSubmit={event => event.preventDefault()} />
    <PaneTree aria-label="Verification trace tree" id="trace-tree">
      <TraceTreeFolderNode folder={filtered} depth={0} query={props.traceTreeQuery} />
    </PaneTree>
    <TraceTreeSummary files={props.files} />
  </Context.Provider>;
}

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

function TraceTreeSummary({ files }: { files: readonly TraceOverviewFile[] }) {
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


function TraceTreeFolderNode({
  folder,
  depth,
  query,
}: {
  folder: TracePaneFolder;
  depth: number;
  query: string;
}) {
  const ui = useTraceNavigation();
  const selectedPath = ui.traceFilePath;
  const hasSelectedDescendant = selectedPath
    ? traceFolderContainsPath(folder, selectedPath)
    : false;
  const [open, setOpen] = useState(depth < 2 || hasSelectedDescendant);
  const expanded = Boolean(query.trim()) || open;

  useOpenWhenSelectionEnters(hasSelectedDescendant, setOpen);

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
          if (folder.files.length + folder.folders.length > 0) {
            setOpen((value) => !value);
          }
          ui.setTraceFilePath(null);
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
  const ui = useTraceNavigation();
  const selectedFile = ui.traceFilePath === file.path;
  const selectedVerification = selectedFile ? ui.traceSelectionId : null;
  const [open, setOpen] = useState(true);
  const expanded = Boolean(query.trim()) || open;

  function selectFile() {
    if (file.verifications.length > 0) {
      setOpen((value) => !value);
    }
    ui.setTraceFilePath(file.path);
  }

  function selectVerification(id: string) {
    ui.setTraceSelectionId(id);
  }

  return (
    <PaneTreeNode>
      <TreeItem
        role="treeitem" tabIndex={0} aria-expanded={expanded} aria-selected={selectedFile && !selectedVerification}
        title={file.path}
        onKeyDown={event => {
          if (event.key === "Enter" || event.key === " ") { event.preventDefault(); selectFile(); }
          if ((event.key === "ArrowRight" && !expanded) || (event.key === "ArrowLeft" && expanded)) { event.preventDefault(); setOpen(value => !value); }
        }}
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
          role="treeitem" tabIndex={0} aria-selected={selectedVerification === verification.id}
          onKeyDown={event => {
            if (event.key === "Enter" || event.key === " ") { event.preventDefault(); selectVerification(verification.id); }
          }}
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


function buildTraceFileTree(files: readonly TraceOverviewFile[], rootLabel: string): TracePaneFolder {
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


function useOpenWhenSelectionEnters(
  hasSelectedDescendant: boolean,
  setOpen: (update: boolean) => void,
) {
  const previouslyHadSelectedDescendant = useRef(hasSelectedDescendant);

  useEffect(() => {
    if (hasSelectedDescendant && !previouslyHadSelectedDescendant.current) {
      setOpen(true);
    }
    previouslyHadSelectedDescendant.current = hasSelectedDescendant;
  }, [hasSelectedDescendant, setOpen]);
}
