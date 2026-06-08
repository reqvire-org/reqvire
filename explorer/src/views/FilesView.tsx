import { useEffect, useMemo, useState } from "react";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import type {
  ExplorerProjectStore,
  ProjectStoreElement,
  ProjectStoreFile,
} from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { useOptionalExplorerUiState, type ModelMode } from "../components/ExplorerUiState";
import { ElementTypeGlyph } from "../components/ExplorerSidePane";
import { SourceCodePreview } from "../components/SourceCodePreview";
import { routeForContent } from "../router/routes";
import { Card, ElementIcon, Icon, SegmentedControl } from "@ds";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  TableSortButton,
  TableViewport,
} from "@ds";
import { TypeBadge } from "@ds";

const ROOT_FOLDER = "__root__";

type FileLayout = "list" | "grid";
type SortKey = "name" | "type" | "elements" | "path";
type SortDirection = "asc" | "desc";

type FileManagerKind = "folder" | "file";

interface FolderNode {
  kind: "folder";
  id: string;
  path: string;
  name: string;
  parent: string | null;
  folders: FolderNode[];
  files: ProjectStoreFile[];
}

interface FileManagerItem {
  kind: FileManagerKind;
  id: string;
  name: string;
  path: string;
  displayPath: string;
  elementCount: number;
  childCount: number;
  file?: ProjectStoreFile;
  folder?: FolderNode;
}

interface FileManagerModel {
  root: FolderNode;
  folderByPath: Map<string, FolderNode>;
  fileByPath: Map<string, ProjectStoreFile>;
  folderElementCounts: Map<string, number>;
}

/*
 * Files view (`#/files/<path>`). It is a read-only Reqvire file manager:
 * folders and source files are navigable Project Store containers, while
 * modeled elements stay available through the shared element detail modal.
 */
export function FilesView({
  path,
  forcedLayout,
  onOpenElement,
}: {
  path: string | null;
  forcedLayout?: FileLayout;
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store, elementById } = useStore();
  const ui = useOptionalExplorerUiState();
  const model = useMemo(() => buildFileManagerModel(store), [store]);
  const stateDriven = Boolean(forcedLayout && ui);
  const modelSelectionId = ui?.modelSelectionId ?? "__root__";
  const selectedFile = stateDriven
    ? selectedFileFromModelSelection(modelSelectionId, model, store)
    : path
      ? model.fileByPath.get(path)
      : undefined;
  const [currentFolderPath, setCurrentFolderPath] = useState(ROOT_FOLDER);
  const [localLayout, setLocalLayout] = useState<FileLayout>("list");
  const [sortKey, setSortKey] = useState<SortKey>("name");
  const [sortDirection, setSortDirection] = useState<SortDirection>("asc");

  useEffect(() => {
    if (stateDriven) {
      const folderPath = folderPathFromModelSelection(modelSelectionId, model, store);
      setCurrentFolderPath(folderPath);
      return;
    }
    if (!selectedFile) return;
    const nextFolder = selectedFile.parent_folder || ROOT_FOLDER;
    setCurrentFolderPath(model.folderByPath.has(nextFolder) ? nextFolder : ROOT_FOLDER);
  }, [model, model.folderByPath, modelSelectionId, selectedFile, stateDriven, store]);

  const currentFolder = model.folderByPath.get(currentFolderPath) ?? model.root;
  const layout = forcedLayout ?? localLayout;

  const items = useMemo(() => {
    const folderToItem = (folder: FolderNode): FileManagerItem => ({
      kind: "folder",
      id: folder.id,
      name: folder.name,
      path: folder.path,
      displayPath: folder.path === ROOT_FOLDER ? store.project.root_label || "Project root" : folder.path,
      elementCount: model.folderElementCounts.get(folder.path) ?? 0,
      childCount: folder.folders.length + folder.files.length,
      folder,
    });

    const fileToItem = (file: ProjectStoreFile): FileManagerItem => ({
      kind: "file",
      id: `file:${file.path}`,
      name: displayName(file.display_path || file.path),
      path: file.path,
      displayPath: file.display_path || file.path,
      elementCount: file.element_ids.length,
      childCount: file.element_ids.length,
      file,
    });

    return [
      ...currentFolder.folders.map(folderToItem),
      ...currentFolder.files.map(fileToItem),
    ].sort((a, b) => compareItems(a, b, sortKey, sortDirection));
  }, [
    currentFolder,
    model.folderElementCounts,
    sortDirection,
    sortKey,
    store.project.root_label,
  ]);

  function updateSort(nextKey: SortKey) {
    if (nextKey === sortKey) {
      setSortDirection((current) => (current === "asc" ? "desc" : "asc"));
    } else {
      setSortKey(nextKey);
      setSortDirection("asc");
    }
  }

  function openFolder(folderPath: string) {
    if (stateDriven && ui) {
      ui.setModelSelectionId(folderPath === ROOT_FOLDER ? "__root__" : `folder:${folderPath}`);
    }
    setCurrentFolderPath(folderPath);
  }

  function openFile(filePath: string) {
    if (stateDriven && ui) {
      ui.setModelSelectionId(`file:${filePath}`);
    }
  }

  function changeLayout(nextLayout: ModelMode) {
    if (stateDriven && ui) {
      ui.setModelMode(nextLayout);
      return;
    }
    if (nextLayout !== "graph") {
      setLocalLayout(nextLayout);
    }
  }

  return (
    <ViewFrame testId="files">
      <div className="ex-route ex-route-single">
        <div className="ex-document-panel ex-browser ex-file-shell">
          <FileManagerToolbar
            currentFolder={currentFolder}
            selectedFile={selectedFile}
            rootLabel={store.project.root_label || "Project root"}
            layout={layout}
            resultCount={items.length}
            onOpenFolder={openFolder}
            onLayoutChange={changeLayout}
          />

          {path && !selectedFile ? (
            <span className="file-missing-message">
              No file container for <code className="file-missing-path">{path}</code>.
            </span>
          ) : (
            <>
              {layout === "list" ? (
                <FileManagerList
                  items={items}
                  selectedFile={selectedFile}
                  sortKey={sortKey}
                  sortDirection={sortDirection}
                  onSort={updateSort}
                  onOpenFolder={openFolder}
                  onOpenFile={stateDriven ? openFile : undefined}
                />
              ) : (
                <FileManagerGrid
                  items={items}
                  selectedFile={selectedFile}
                  onOpenFolder={openFolder}
                  onOpenFile={stateDriven ? openFile : undefined}
                />
              )}

              <SelectedFileElements
                file={selectedFile}
                layout={layout}
                onOpenElement={onOpenElement}
                elementById={elementById}
              />
            </>
          )}
        </div>
      </div>
    </ViewFrame>
  );
}

function buildFileManagerModel(store: ExplorerProjectStore): FileManagerModel {
  const root: FolderNode = {
    kind: "folder",
    id: "folder:__root__",
    path: ROOT_FOLDER,
    name: store.project.root_label || "Project root",
    parent: null,
    folders: [],
    files: [],
  };
  const folderByPath = new Map<string, FolderNode>([[ROOT_FOLDER, root]]);

  for (const folder of store.folders) {
    const folderPath = normalizeFolderPath(folder.path);
    if (folderPath === ROOT_FOLDER) continue;
    folderByPath.set(folderPath, {
      kind: "folder",
      id: `folder:${folderPath}`,
      path: folderPath,
      name: displayName(folderPath),
      parent: normalizeFolderPath(folder.parent),
      folders: [],
      files: [],
    });
  }

  for (const folder of store.folders) {
    const folderPath = normalizeFolderPath(folder.path);
    if (folderPath === ROOT_FOLDER) continue;
    const node = folderByPath.get(folderPath);
    if (!node) continue;
    const parentPath = normalizeFolderPath(folder.parent);
    const parent = parentPath === ROOT_FOLDER ? root : folderByPath.get(parentPath);
    (parent ?? root).folders.push(node);
  }

  const fileByPath = new Map<string, ProjectStoreFile>();
  for (const file of store.files) {
    fileByPath.set(file.path, file);
    const parentPath = normalizeFolderPath(file.parent_folder);
    const parent = folderByPath.get(parentPath) ?? root;
    parent.files.push(file);
  }

  for (const folder of folderByPath.values()) {
    folder.folders.sort((a, b) => a.name.localeCompare(b.name));
    folder.files.sort((a, b) => a.display_path.localeCompare(b.display_path));
  }

  const folderElementCounts = new Map<string, number>();
  function countElements(folder: FolderNode): number {
    const direct = folder.files.reduce((count, file) => count + file.element_ids.length, 0);
    const nested = folder.folders.reduce((count, child) => count + countElements(child), 0);
    const total = direct + nested;
    folderElementCounts.set(folder.path, total);
    return total;
  }
  countElements(root);

  return { root, folderByPath, fileByPath, folderElementCounts };
}

function normalizeFolderPath(path: string | null | undefined): string {
  const normalized = (path ?? "").replace(/^\/+|\/+$/g, "");
  return normalized || ROOT_FOLDER;
}

function FileManagerToolbar({
  currentFolder,
  selectedFile,
  rootLabel,
  layout,
  resultCount,
  onOpenFolder,
  onLayoutChange,
}: {
  currentFolder: FolderNode;
  selectedFile: ProjectStoreFile | undefined;
  rootLabel: string;
  layout: FileLayout;
  resultCount: number;
  onOpenFolder: (path: string) => void;
  onLayoutChange: (layout: ModelMode) => void;
}) {
  const crumbs = folderCrumbs(currentFolder, rootLabel);
  return (
    <div className="ex-browser__bar ex-file-toolbar">
      <div className="rq-crumbs ex-file-breadcrumbs" aria-label="File breadcrumbs">
        {crumbs.map((crumb, index) => (
          <span key={crumb.path} className="rq-crumbs__item ex-file-crumb">
            {index > 0 && <span className="rq-crumbs__sep ex-file-crumb-separator">/</span>}
            <button type="button" onClick={() => onOpenFolder(crumb.path)}>
              {crumb.label}
            </button>
          </span>
        ))}
        {selectedFile && (
          <span className="rq-crumbs__item ex-file-crumb ex-file-crumb-current">
            <span className="rq-crumbs__sep ex-file-crumb-separator">/</span>
            <span title={selectedFile.display_path || selectedFile.path}>
              {displayName(selectedFile.display_path || selectedFile.path)}
            </span>
          </span>
        )}
      </div>
      <div className="ex-file-toolbar-actions">
        <span className="ex-browser__count ex-panel-muted">
          {resultCount} items
        </span>
        <SegmentedControl<ModelMode>
          ariaLabel="Model layout"
          value={layout}
          onChange={onLayoutChange}
          items={[
            { value: "list", label: "List", icon: <Icon name="list" /> },
            { value: "grid", label: "Grid", icon: <Icon name="layout-grid" /> },
            { value: "graph", label: "Graph", icon: <Icon name="git-branch" /> },
          ]}
        />
      </div>
    </div>
  );
}

function FileManagerList({
  items,
  selectedFile,
  sortKey,
  sortDirection,
  onSort,
  onOpenFolder,
  onOpenFile,
}: {
  items: FileManagerItem[];
  selectedFile: ProjectStoreFile | undefined;
  sortKey: SortKey;
  sortDirection: SortDirection;
  onSort: (key: SortKey) => void;
  onOpenFolder: (path: string) => void;
  onOpenFile?: (path: string) => void;
}) {
  return (
    <TableViewport className="ex-file-table-wrap">
      <Table>
        <TableHeader>
          <TableRow>
            <SortableHeader label="Name" sortKey="name" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Type" sortKey="type" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Elements" sortKey="elements" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Path" sortKey="path" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
          </TableRow>
        </TableHeader>
        <TableBody>
          {items.map((item) => (
            <TableRow key={item.id} selected={isSelectedFileItem(item, selectedFile)}>
              <TableCell>
                <div className="ex-file-name-cell">
                  <ItemAction item={item} onOpenFolder={onOpenFolder} onOpenFile={onOpenFile} />
                  <FileContentLink item={item} />
                </div>
              </TableCell>
              <TableCell>
                <span className="rq-typebadge">{item.kind}</span>
              </TableCell>
              <TableCell>{item.elementCount}</TableCell>
              <TableCell className="ex-file-path">{item.displayPath}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      {items.length === 0 && <span className="ex-empty">No files or folders match the current filter.</span>}
    </TableViewport>
  );
}

function SortableHeader({
  label,
  sortKey,
  activeKey,
  direction,
  onSort,
}: {
  label: string;
  sortKey: SortKey;
  activeKey: SortKey;
  direction: SortDirection;
  onSort: (key: SortKey) => void;
}) {
  const active = sortKey === activeKey;
  return (
    <TableHead>
      <TableSortButton direction={active ? direction : undefined} onClick={() => onSort(sortKey)}>
        {label}
      </TableSortButton>
    </TableHead>
  );
}

function FileManagerGrid({
  items,
  selectedFile,
  onOpenFolder,
  onOpenFile,
}: {
  items: FileManagerItem[];
  selectedFile: ProjectStoreFile | undefined;
  onOpenFolder: (path: string) => void;
  onOpenFile?: (path: string) => void;
}) {
  return (
    <div className="ex-grid ex-file-grid">
      {items.map((item) => (
        <Card
          key={item.id}
          interactive
          selected={isSelectedFileItem(item, selectedFile)}
          className={[
            "ex-tile",
            "ex-file-card",
            isSelectedFileItem(item, selectedFile) ? "is-selected" : "",
            item.kind === "file" && item.elementCount === 0 ? "is-empty-file" : "",
          ].join(" ")}
        >
          <ItemAction item={item} onOpenFolder={onOpenFolder} onOpenFile={onOpenFile} />
          {(item.kind === "folder" || item.elementCount > 0) && (
            <span className="ex-file-count-badge">
              {item.kind === "folder" ? `${item.childCount} children` : `${item.elementCount} elements`}
            </span>
          )}
          <span className="ex-file-card-path">{item.displayPath}</span>
        </Card>
      ))}
      {items.length === 0 && <span className="ex-empty">No files or folders match the current filter.</span>}
    </div>
  );
}

function FileContentLink({
  item,
}: {
  item: FileManagerItem;
}) {
  if (!item.file) return null;
  return (
    <a
      href={routeForContent(item.path)}
      className="ex-file-open-link"
      aria-label={`Open content for ${item.name}`}
      title="Open content"
      onClick={(event) => event.stopPropagation()}
    >
      <Icon name="external-link" />
    </a>
  );
}

function ItemAction({
  item,
  onOpenFolder,
  onOpenFile,
}: {
  item: FileManagerItem;
  onOpenFolder: (path: string) => void;
  onOpenFile?: (path: string) => void;
}) {
  const content = (
    <>
      {item.kind === "folder" ? (
        <Icon name="folder" className="file-kind-folder ex-file-item-icon" />
      ) : (
        <Icon name="file" className="file-kind-file ex-file-item-icon" />
      )}
      <span className="ex-file-item-name">{item.name}</span>
    </>
  );
  if (item.folder) {
    return (
      <button type="button" className="ex-file-item-action" onClick={() => onOpenFolder(item.folder?.path ?? ROOT_FOLDER)}>
        {content}
      </button>
    );
  }
  if (onOpenFile) {
    return (
      <button
        type="button"
        className="ex-file-item-action"
        onClick={() => onOpenFile(item.path)}
      >
        {content}
      </button>
    );
  }
  return (
    <a
      href={`#/files/${item.path}`}
      className="ex-file-item-action"
    >
      {content}
    </a>
  );
}

function isSelectedFileItem(item: FileManagerItem, selectedFile: ProjectStoreFile | undefined) {
  return Boolean(item.file && selectedFile && item.file.path === selectedFile.path);
}

function SelectedFileElements({
  file,
  layout,
  onOpenElement,
  elementById,
}: {
  file: ProjectStoreFile | undefined;
  layout: FileLayout;
  onOpenElement: (id: string) => void;
  elementById: (id: string) => ProjectStoreElement | undefined;
}) {
  if (!file) {
    return (
      <div className="ex-file-elements">
        <span className="ex-empty">Select a file row to inspect its modeled elements.</span>
      </div>
    );
  }
  if (file.element_ids.length === 0) {
    return (
      <div className="ex-file-elements">
        <SourceCodePreview
          path={file.path}
          content={file.markdown_content}
          kind="source file"
          defaultExpanded
          showPath
        />
      </div>
    );
  }
  return (
    <div className="ex-file-elements">
      <div className={layout === "grid" ? "modeled-elements-grid" : "modeled-elements-list"}>
        {file.element_ids.map((id) => {
          const element = elementById(id);
          if (layout === "grid") {
            return (
              <button
                key={id}
                type="button"
                onClick={() => onOpenElement(id)}
                className="modeled-element-card"
              >
                {element ? (
                  <ElementTypeGlyph element={element} />
                ) : (
                  <ElementIcon type="other" size="sm" />
                )}
                <span className="modeled-element-card-main">
                  <span className="modeled-element-card-title">{element?.name ?? id}</span>
                  {element?.element_type && (
                    <TypeBadge type={element.element_type} family={element.type_family} tinted>
                      {element.element_type}
                    </TypeBadge>
                  )}
                </span>
              </button>
            );
          }
          return (
            <button
              key={id}
              type="button"
              onClick={() => onOpenElement(id)}
              className="ex-list-row"
            >
              {element ? (
                <ElementTypeGlyph element={element} />
              ) : (
                <ElementIcon type="other" size="sm" />
              )}
              <span className="modeled-element-list-title">{element?.name ?? id}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}

function folderCrumbs(folder: FolderNode, rootLabel: string): { path: string; label: string }[] {
  if (folder.path === ROOT_FOLDER) return [{ path: ROOT_FOLDER, label: rootLabel }];
  const segments = folder.path.split("/").filter(Boolean);
  const crumbs = [{ path: ROOT_FOLDER, label: rootLabel }];
  let path = "";
  for (const segment of segments) {
    path = path ? `${path}/${segment}` : segment;
    crumbs.push({ path, label: segment });
  }
  return crumbs;
}

function compareItems(
  a: FileManagerItem,
  b: FileManagerItem,
  sortKey: SortKey,
  direction: SortDirection,
): number {
  if (a.kind !== b.kind && sortKey !== "type") {
    return a.kind === "folder" ? -1 : 1;
  }

  let result = 0;
  if (sortKey === "elements") {
    result = a.elementCount - b.elementCount;
  } else if (sortKey === "type") {
    result = a.kind.localeCompare(b.kind) || a.name.localeCompare(b.name);
  } else if (sortKey === "path") {
    result = a.displayPath.localeCompare(b.displayPath);
  } else {
    result = a.name.localeCompare(b.name);
  }
  return direction === "asc" ? result : -result;
}

function displayName(path: string): string {
  return path.split("/").filter(Boolean).at(-1) ?? path;
}

function selectedFileFromModelSelection(
  selectionId: string,
  model: FileManagerModel,
  store: ExplorerProjectStore,
): ProjectStoreFile | undefined {
  if (selectionId.startsWith("file:")) {
    return model.fileByPath.get(selectionId.slice("file:".length));
  }
  const element = store.elements.find((item) => item.id === selectionId);
  return element ? model.fileByPath.get(element.file_path) : undefined;
}

function folderPathFromModelSelection(
  selectionId: string,
  model: FileManagerModel,
  store: ExplorerProjectStore,
): string {
  if (selectionId === "__root__") return ROOT_FOLDER;
  if (selectionId.startsWith("folder:")) {
    const folderPath = selectionId.slice("folder:".length);
    return model.folderByPath.has(folderPath) ? folderPath : ROOT_FOLDER;
  }
  const selectedFile = selectedFileFromModelSelection(selectionId, model, store);
  if (selectedFile) {
    const folderPath = selectedFile.parent_folder || ROOT_FOLDER;
    return model.folderByPath.has(folderPath) ? folderPath : ROOT_FOLDER;
  }
  return ROOT_FOLDER;
}
