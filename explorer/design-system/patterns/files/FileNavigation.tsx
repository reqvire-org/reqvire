import { createContext } from "preact";
import { useContext, useMemo, useState } from "preact/hooks";
import { Icon } from "../../components/core/Icon";
import { ElementIcon } from "../../components/data/ElementIcon";
import { TreeItem } from "../../components/navigation/TreeItem";
import { PaneTree, PaneTreeNode } from "../side-pane/PaneTree";
import { PaneSearchForm } from "../side-pane/PaneControls";
import { PaneSummary, type PaneSummaryItem } from "../side-pane/PaneSummary";

const ROOT_PATH = "__root__", MODEL_ROOT_PATH = "__model__", RESOURCE_ROOT_PATH = "__resources__";
const displayName = (path: string) => path.split("/").pop() || path;
export interface FileNavigationFile { path: string; display_path: string; markdown_content: string; element_ids: string[]; }
export interface FileNavigationResource {
  id: string; kind: string; target: string; display: string; file_path: string | null;
  source_text?: string | null; relation_types: string[];
}
export interface FileNavigationElement {
  id: string; name: string; element_type: string; type_family: string; content: string;
  source_anchor: string; file_path: string;
}
export interface FileNavigationFolder {
  path: string; name: string; selectionId?: string;
  folders: FileNavigationFolder[]; files: FileNavigationFile[]; resources: FileNavigationResource[];
}
export interface FileNavigationProps {
  tree: FileNavigationFolder;
  elementById: (id: string) => FileNavigationElement | undefined;
  activeView: string;
  onNavigate: (view: "model") => void;
  onOpenElement: (id: string) => void;
  sourceBrowsing?: boolean;
  onOpenSourceRoute?: (hash: string) => void;
  modelMode: string;
  modelSelectionId: string;
  setModelSelectionId: (id: string) => void;
  modelTreeQuery: string;
  setModelTreeQuery: (query: string) => void;
  routeForContent: (path: string) => string;
  routeForResource: (id: string) => string;
  navigateHash: (hash: string) => void;
  summaryItems?: PaneSummaryItem[];
}
const Context = createContext<FileNavigationProps | null>(null);
const useFileNavigation = () => useContext(Context)!;

export function FileNavigation(props: FileNavigationProps) {
  const filtered = useMemo(() => filterFileTree(props.tree, props.modelTreeQuery, props.elementById),
    [props.tree, props.modelTreeQuery, props.elementById]);
  return <Context.Provider value={props}>
    <PaneSearchForm searchInputId="project-tree-search" inputLabel="Filter project tree"
      placeholder="Filter model tree..." value={props.modelTreeQuery} resultsId="project-tree"
      onQueryChange={props.setModelTreeQuery} onSubmit={event => event.preventDefault()} />
    <PaneTree aria-label="Project tree" id="project-tree">
      {filtered.folders.map(folder => <TreeFolderNode key={folder.path} folder={folder}
        activeView={props.activeView} elementById={props.elementById} onNavigate={props.onNavigate}
        onOpenElement={props.onOpenElement} sourceBrowsing={props.sourceBrowsing ?? false}
        onOpenSourceRoute={props.onOpenSourceRoute} depth={0} query={props.modelTreeQuery} />)}
    </PaneTree>
    {props.summaryItems && <PaneSummary items={props.summaryItems} placement="footer" />}
  </Context.Provider>;
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
  folder: FileNavigationFolder;
  activeView: string;
  elementById: (id: string) => FileNavigationElement | undefined;
  onNavigate: (view: "model") => void;
  onOpenElement: (id: string) => void;
  sourceBrowsing: boolean;
  onOpenSourceRoute?: (hash: string) => void;
  depth: number;
  query: string;
}) {
  const [open, setOpen] = useState(depth < 2);
  const ui = useFileNavigation();
  const selectionId = folder.selectionId ?? folderSelectionId(folder.path);
  const expanded = Boolean(query.trim()) || open;

  function selectFolder() {
    if (treeFolderChildCount(folder) > 0) {
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
        count={treeFolderChildCount(folder)}
        depth={depth}
        open={expanded}
        expandable={treeFolderChildCount(folder) > 0}
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
          {folder.resources.map((resource) => (
            <TreeResourceNode
              key={resource.id}
              resource={resource}
              activeView={activeView}
              sourceBrowsing={sourceBrowsing}
              onOpenSourceRoute={onOpenSourceRoute}
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


function TreeResourceNode({
  resource,
  activeView,
  sourceBrowsing,
  onOpenSourceRoute,
  depth,
}: {
  resource: FileNavigationResource;
  activeView: string;
  sourceBrowsing: boolean;
  onOpenSourceRoute?: (hash: string) => void;
  depth: number;
}) {
  const ui = useFileNavigation();
  const route = ui.routeForResource(resource.id);

  function selectResource() {
    ui.setModelSelectionId(resource.id);
    if (sourceBrowsing) {
      onOpenSourceRoute?.(route);
      return;
    }
    if (activeView === "model" && ui.modelMode === "flow") return;
    ui.navigateHash(route);
  }

  return (
    <TreeItem
      kind="element"
      label={resource.display || displayName(resource.target)}
      icon={<ElementIcon type="evidence-file" title={resource.kind} size="sm" />}
      count={resource.relation_types.length > 0 ? resource.relation_types.length : undefined}
      depth={depth}
      selected={ui.modelSelectionId === resource.id}
      onSelect={selectResource}
      title={resource.target}
    />
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
  file: FileNavigationFile;
  activeView: string;
  elementById: (id: string) => FileNavigationElement | undefined;
  onNavigate: (view: "model") => void;
  onOpenElement: (id: string) => void;
  sourceBrowsing: boolean;
  onOpenSourceRoute?: (hash: string) => void;
  depth: number;
  query: string;
}) {
  const ui = useFileNavigation();
  const elements = file.element_ids.map(elementById).filter(Boolean) as FileNavigationElement[];
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
      onOpenSourceRoute?.(ui.routeForContent(file.path));
      return;
    }
    if (activeView === "files") onNavigate("model");
  }

  function selectElement(elementId: string) {
    ui.setModelSelectionId(elementId);
    if (sourceBrowsing) {
      const element = elementById(elementId);
      if (element) {
        onOpenSourceRoute?.(sourceRouteForElement(element, ui.routeForContent));
      }
      return;
    }
    if (activeView === "files") onNavigate("model");
    if (ui.modelMode === "flow") return;
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


function sourceRouteForElement(element: FileNavigationElement, routeForContent: (path: string) => string) {
  if (element.source_anchor.startsWith("#/content/")) return routeForContent(element.source_anchor.slice("#/content/".length));
  if (element.source_anchor.startsWith("#")) {
    return routeForContent(`${element.file_path}${element.source_anchor}`);
  }
  return element.source_anchor;
}


function filterFileTree(
  folder: FileNavigationFolder,
  query: string,
  elementById: (id: string) => FileNavigationElement | undefined,
): FileNavigationFolder {
  const normalized = query.trim().toLowerCase();
  if (!normalized) return folder;

  const filtered = filterFileTreeNode(folder, normalized, elementById, true);
  return filtered ?? { ...folder, folders: [], files: [], resources: [] };
}


function filterFileTreeNode(
  folder: FileNavigationFolder,
  query: string,
  elementById: (id: string) => FileNavigationElement | undefined,
  isRoot = false,
): FileNavigationFolder | null {
  const folderMatches = !isRoot && textMatches(query, folder.name, folder.path);
  if (folderMatches) return folder;

  const folders = folder.folders
    .map((child) => filterFileTreeNode(child, query, elementById))
    .filter(Boolean) as FileNavigationFolder[];
  const files = folder.files
    .map((file) => filterProjectFile(file, query, elementById))
    .filter(Boolean) as FileNavigationFile[];
  const resources = folder.resources
    .filter((resource) => filterProjectResource(resource, query));

  if (folders.length === 0 && files.length === 0 && resources.length === 0 && !isRoot) return null;
  return { ...folder, folders, files, resources };
}


function filterProjectFile(
  file: FileNavigationFile,
  query: string,
  elementById: (id: string) => FileNavigationElement | undefined,
): FileNavigationFile | null {
  if (textMatches(query, file.display_path, file.path, file.markdown_content)) return file;

  const elementIds = file.element_ids.filter((id) => {
    const element = elementById(id);
    return element
      ? textMatches(query, element.name, element.element_type, element.type_family, element.content)
      : id.toLowerCase().includes(query);
  });

  return elementIds.length > 0 ? { ...file, element_ids: elementIds } : null;
}


function filterProjectResource(resource: FileNavigationResource, query: string) {
  return textMatches(
    query,
    resource.display,
    resource.target,
    resource.file_path,
    resource.kind,
    resource.source_text,
    ...resource.relation_types,
  );
}


function treeFolderChildCount(folder: FileNavigationFolder) {
  return folder.folders.length + folder.files.length + folder.resources.length;
}


function folderSelectionId(path: string) {
  if (path === ROOT_PATH || path === MODEL_ROOT_PATH) return "__root__";
  if (path === RESOURCE_ROOT_PATH) return "resource-root";
  if (path.startsWith(`${RESOURCE_ROOT_PATH}/`)) return `resource-folder:${path.slice(RESOURCE_ROOT_PATH.length + 1)}`;
  return `folder:${path}`;
}


function textMatches(query: string, ...values: Array<string | null | undefined>) {
  return values.some((value) => value?.toLowerCase().includes(query));
}
