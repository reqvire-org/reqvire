import { useEffect, useMemo, useState } from "react";
import { css, cx } from "@linaria/atomic";
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

const routeBaseUX = css`
  box-sizing: border-box;
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-left: var(--ex-current-left-width);
  padding-right: 0;

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }
`;

const routeSingleUX = css`
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
`;

const routeSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);
`;

const documentPanelBaseUX = css`
  position: relative;
  box-sizing: border-box;
  min-width: 0;
  min-height: 0;
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }
`;

const documentPanelSkinX = css`
  border-left: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  border-right: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  background: var(--bg-surface);

  .ex-app & {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }
`;

const fileShellBaseUX = css`
  --ex-file-toolbar-actions-min-w: 280px;
  --ex-file-crumb-max-w: 190px;
  --ex-file-crumb-wide-max-w: 240px;
  --ex-file-table-min-w: 780px;
  --ex-file-path-max-w: 360px;
  --ex-file-tile-min-w: 230px;
  --ex-file-tile-min-h-compact: 112px;
  --ex-file-row-card-min-h: 78px;
  --ex-file-hover-lift: -1px;
  display: flex;
  flex-direction: column;
  gap: var(--space-7);
  overflow: visible;

  .ex-app & {
    overflow: auto;
  }
`;

const fileShellSkinX = css`
  color: var(--text-body);
`;

const fileMissingMessageBaseUX = css`
  font-size: var(--text-sm);
`;

const fileMissingMessageSkinX = css`
  color: var(--text-muted);
`;

const fileMissingPathBaseUX = css`
  padding: var(--space-1) var(--space-2);
  font-size: var(--text-xs);
`;

const fileMissingPathSkinX = css`
  border-radius: var(--radius-xs);
  background: var(--bg-sunken);
`;

const fileToolbarBaseUX = css`
  display: flex;
  min-height: var(--space-24);
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
  padding: 0 var(--space-2) var(--space-7);

  .ex-file-toolbar-actions {
    display: flex;
    min-width: min(100%, var(--ex-file-toolbar-actions-min-w));
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-5);
    flex-wrap: wrap;
  }

  .ex-file-breadcrumbs {
    display: flex;
    min-width: 0;
    flex: 1 1 auto;
    align-items: center;
    gap: var(--space-1);
    overflow: hidden;
    font-size: var(--text-sm);
  }

  .ex-file-crumb {
    display: inline-flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-1);
  }

  .ex-file-crumb button {
    max-width: var(--ex-file-crumb-max-w);
    overflow: hidden;
    border: 0;
    background: transparent;
    text-overflow: ellipsis;
    white-space: nowrap;
    cursor: pointer;
  }

  .ex-file-crumb-current span:last-child {
    display: inline-block;
    max-width: var(--ex-file-crumb-wide-max-w);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @media (max-width: 900px) {
    align-items: stretch;
    flex-direction: column;

    .ex-file-toolbar-actions {
      width: 100%;
      min-width: 0;
    }
  }
`;

const fileToolbarSkinX = css`
  border-bottom: var(--border-w) solid var(--border-default);
  background: var(--bg-surface);

  .ex-file-breadcrumbs {
    color: var(--text-muted);
  }

  .ex-file-crumb button {
    color: var(--text-body);
  }

  .ex-file-crumb button:hover {
    text-decoration: underline;
  }

  .ex-file-crumb-current span:last-child {
    color: var(--text-strong);
    font-weight: var(--weight-medium);
  }

  .ex-file-crumb-separator {
    color: color-mix(in srgb, var(--text-muted) 70%, transparent);
  }

  .ex-browser__count {
    color: var(--text-muted);
    font-size: var(--text-caption);
    line-height: 1.4;
  }
`;

const fileTableBaseUX = css`
  min-height: 0;
  overflow: visible;
  box-shadow: none;
  --rq-tablewrap-border: 0;
  --rq-tablewrap-radius: 0;
  --rq-tablewrap-bg: transparent;
  --rq-table-min-w: var(--ex-file-table-min-w);
  --rq-table-td-p: var(--space-4) var(--space-6);

  th {
    font-weight: var(--weight-bold);
  }

  .ex-file-name-cell {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-width: 0;
  }

  .ex-file-path {
    max-width: var(--ex-file-path-max-w);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
`;

const fileTableSkinX = css`
  --rq-table-th-bg: transparent;
  --rq-table-th-border: transparent;
  --rq-table-th-fw: var(--weight-bold);
  --rq-table-td-border: transparent;
  --rq-table-row-hover-bg: transparent;
  --rq-table-sel-bg: transparent;

  th {
    background: transparent;
  }

  .ex-file-path {
    color: var(--text-muted);
  }

  .ex-file-name-cell .ex-file-item-action:hover,
  .ex-file-name-cell .ex-file-item-action:focus-visible,
  .ex-file-name-cell .ex-file-item-action.is-selected {
    border-color: transparent;
    background: color-mix(in srgb, var(--accent) 6%, transparent);
    box-shadow: none;
    outline: none;
  }

  .ex-file-name-cell .ex-file-item-action.is-selected {
    background: color-mix(in srgb, var(--accent) 10%, transparent);
  }
`;

const fileGridBaseUX = css`
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(var(--ex-file-tile-min-w), 1fr));
  gap: var(--space-8);

  .ex-file-card {
    position: relative;
    display: flex;
    flex-direction: column;
    min-height: var(--ex-file-tile-min-h-compact);
    gap: var(--space-5);
    box-sizing: border-box;
    padding: var(--space-7);
  }

  .ex-file-card.is-empty-file {
    min-height: var(--ex-file-tile-min-h-compact);
  }

  .ex-file-card > .ex-file-item-action {
    border: 0;
    padding: 0;
    font-size: var(--text-base);
    font-weight: var(--weight-semibold);
  }

  .ex-file-card > .ex-file-open-link {
    position: absolute;
    top: var(--space-5);
    right: var(--space-5);
  }

  .ex-file-card-path {
    display: -webkit-box;
    min-width: 0;
    overflow: hidden;
    overflow-wrap: anywhere;
    font-size: var(--text-caption);
    line-height: 1.35;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }

  .ex-file-count-badge {
    display: inline-flex;
    width: fit-content;
    align-items: center;
    padding: var(--space-1) var(--space-4);
    font-size: var(--text-caption);
    font-weight: var(--weight-bold);
    line-height: 1.2;
  }
`;

const fileGridSkinX = css`
  .ex-file-card {
    border-color: var(--border-default);
    border-radius: var(--radius-sm);
    background: var(--bg-raised);
    box-shadow: var(--shadow-xs);
  }

  .ex-file-card:hover {
    border-color: var(--border-default);
    background: color-mix(in srgb, var(--accent) 5%, var(--bg-raised));
    box-shadow: var(--shadow-xs);
    transform: translateY(var(--ex-file-hover-lift));
  }

  .ex-file-card.is-selected,
  .ex-file-card.is-selected:hover {
    border-color: var(--border-default);
    background: color-mix(in srgb, var(--accent) 10%, var(--bg-raised));
    box-shadow: var(--shadow-xs);
  }

  .ex-file-card > .ex-file-item-action {
    border-radius: var(--radius-sm);
  }

  .ex-file-card > .ex-file-item-action:hover,
  .ex-file-card > .ex-file-item-action.is-selected {
    border-color: transparent;
    background: transparent;
  }

  .ex-file-card-path {
    color: var(--text-muted);
  }

  .ex-file-count-badge {
    border-radius: var(--radius-pill);
    background: var(--bg-sunken);
    color: var(--text-muted);
  }
`;

const fileItemActionBaseUX = css`
  display: flex;
  width: 100%;
  min-width: 0;
  align-items: center;
  gap: var(--space-4);
  box-sizing: border-box;
  padding: var(--space-3) var(--space-5);
  text-align: left;
  text-decoration: none;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  cursor: pointer;
`;

const fileItemActionSkinX = css`
  border: var(--border-w) solid transparent;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-body);

  &:hover {
    border-color: var(--border-subtle);
    background: var(--bg-hover);
  }
`;

const fileItemNameBaseUX = css`
  min-width: 0;
  flex: 1 1 auto;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;

const fileItemIconBaseUX = css`
  width: var(--icon-sm);
  height: var(--icon-sm);
  flex: 0 0 auto;
`;

const fileOpenLinkBaseUX = css`
  display: inline-flex;
  flex: 0 0 auto;
  width: var(--space-12);
  height: var(--space-12);
  align-items: center;
  justify-content: center;
  text-decoration: none;
  opacity: 0.64;
  transition:
    color var(--dur-fast) var(--ease-standard),
    opacity var(--dur-fast) var(--ease-standard),
    transform var(--dur-fast) var(--ease-standard);

  svg {
    display: block;
    flex: 0 0 auto;
    width: var(--space-7);
    height: var(--space-7);
  }
`;

const fileOpenLinkSkinX = css`
  border: 0;
  border-radius: var(--radius-xs);
  background: transparent;
  color: var(--text-muted);

  &:hover,
  &:focus-visible {
    background: transparent;
    color: var(--text-strong);
    opacity: 1;
    outline: none;
  }

  &:focus-visible {
    box-shadow: var(--ring-focus);
  }
`;

const fileElementsBaseUX = css`
  padding-top: var(--space-10);

  .modeled-elements-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin-top: var(--space-6);
  }

  .modeled-elements-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--ex-file-tile-min-w), 1fr));
    gap: var(--space-7);
    margin-top: var(--space-7);
  }

  .modeled-element-card {
    display: grid;
    grid-template-columns: var(--control-md) minmax(0, 1fr);
    align-items: start;
    gap: var(--space-5);
    min-height: var(--ex-file-row-card-min-h);
    box-sizing: border-box;
    padding: var(--space-6);
    text-align: left;
    cursor: pointer;
    transition:
      border-color var(--dur-fast) var(--ease-standard),
      background var(--dur-fast) var(--ease-standard),
      box-shadow var(--dur-fast) var(--ease-standard),
      transform var(--dur-fast) var(--ease-standard);
  }

  .modeled-element-card-main {
    display: grid;
    min-width: 0;
    gap: var(--space-3);
  }

  .modeled-element-card-title {
    display: -webkit-box;
    overflow: hidden;
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    line-height: 1.3;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }

  .ex-list-row {
    display: flex;
    width: 100%;
    min-width: 0;
    align-items: center;
    gap: var(--space-4);
    box-sizing: border-box;
    padding: var(--space-4) var(--space-5);
    text-align: left;
  }
`;

const fileElementsSkinX = css`
  border-top: var(--border-w) solid var(--border-default);

  .modeled-element-card {
    border: var(--border-w) solid var(--border-default);
    border-radius: var(--radius-sm);
    background: var(--bg-surface);
    color: var(--text-body);
  }

  .modeled-element-card:hover {
    border-color: var(--border-default);
    background: color-mix(in srgb, var(--accent) 5%, var(--bg-surface));
    transform: translateY(var(--ex-file-hover-lift));
  }

  .modeled-element-card-title {
    color: var(--text-body);
  }

  .ex-list-row {
    border: var(--border-w) solid transparent;
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--text-body);
  }

  .ex-list-row:hover {
    border-color: transparent;
    background: color-mix(in srgb, var(--accent) 5%, transparent);
  }

  .ex-list-row.is-selected {
    border-color: transparent;
    background: color-mix(in srgb, var(--accent) 10%, transparent);
    color: var(--text-body);
  }
`;

const emptyBaseUX = css`
  font-size: var(--text-sm);
  font-style: italic;
  line-height: 1.45;
`;

const emptySkinX = css`
  color: var(--text-muted);
`;

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
      <div className={cx(routeBaseUX, routeSingleUX, routeSkinX)}>
        <div className={cx(documentPanelBaseUX, documentPanelSkinX, fileShellBaseUX, fileShellSkinX)}>
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
            <span className={cx(fileMissingMessageBaseUX, fileMissingMessageSkinX)}>
              No file container for <code className={cx(fileMissingPathBaseUX, fileMissingPathSkinX)}>{path}</code>.
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
    <div className={cx(fileToolbarBaseUX, fileToolbarSkinX)}>
      <div className={cx("ex-file-breadcrumbs")} aria-label="File breadcrumbs">
        {crumbs.map((crumb, index) => (
          <span key={crumb.path} className={cx("ex-file-crumb")}>
            {index > 0 && <span className={cx("ex-file-crumb-separator")}>/</span>}
            <button type="button" onClick={() => onOpenFolder(crumb.path)}>
              {crumb.label}
            </button>
          </span>
        ))}
        {selectedFile && (
          <span className={cx("ex-file-crumb", "ex-file-crumb-current")}>
            <span className={cx("ex-file-crumb-separator")}>/</span>
            <span title={selectedFile.display_path || selectedFile.path}>
              {displayName(selectedFile.display_path || selectedFile.path)}
            </span>
          </span>
        )}
      </div>
      <div className={cx("ex-file-toolbar-actions")}>
        <span className={cx("ex-browser__count")}>
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
    <TableViewport className={cx(fileTableBaseUX, fileTableSkinX)}>
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
          {items.map((item) => {
            const selected = isSelectedFileItem(item, selectedFile);
            return (
              <TableRow key={item.id} selected={selected}>
                <TableCell>
                  <div className={cx("ex-file-name-cell")}>
                    <ItemAction item={item} selected={selected} onOpenFolder={onOpenFolder} onOpenFile={onOpenFile} />
                    <FileContentLink item={item} />
                  </div>
                </TableCell>
                <TableCell>
                  <TypeBadge type={item.kind} family={item.kind}>{item.kind}</TypeBadge>
                </TableCell>
                <TableCell>{item.elementCount}</TableCell>
                <TableCell className={cx("ex-file-path")}>{item.displayPath}</TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
      {items.length === 0 && <span className={cx(emptyBaseUX, emptySkinX)}>No files or folders match the current filter.</span>}
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
    <div className={cx(fileGridBaseUX, fileGridSkinX)}>
      {items.map((item) => (
        <Card
          key={item.id}
          interactive
          selected={isSelectedFileItem(item, selectedFile)}
          className={cx(
            "ex-file-card",
            isSelectedFileItem(item, selectedFile) ? "is-selected" : "",
            item.kind === "file" && item.elementCount === 0 ? "is-empty-file" : "",
          )}
        >
          <ItemAction item={item} onOpenFolder={onOpenFolder} onOpenFile={onOpenFile} />
          {(item.kind === "folder" || item.elementCount > 0) && (
            <span className={cx("ex-file-count-badge")}>
              {item.kind === "folder" ? `${item.childCount} children` : `${item.elementCount} elements`}
            </span>
          )}
          <span className={cx("ex-file-card-path")}>{item.displayPath}</span>
        </Card>
      ))}
      {items.length === 0 && <span className={cx(emptyBaseUX, emptySkinX)}>No files or folders match the current filter.</span>}
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
      className={cx("ex-file-open-link", fileOpenLinkBaseUX, fileOpenLinkSkinX)}
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
  selected = false,
  onOpenFolder,
  onOpenFile,
}: {
  item: FileManagerItem;
  selected?: boolean;
  onOpenFolder: (path: string) => void;
  onOpenFile?: (path: string) => void;
}) {
  const content = (
    <>
      {item.kind === "folder" ? (
        <Icon name="folder" className={cx("file-kind-folder", "ex-file-item-icon", fileItemIconBaseUX)} />
      ) : (
        <Icon name="file" className={cx("file-kind-file", "ex-file-item-icon", fileItemIconBaseUX)} />
      )}
      <span className={cx("ex-file-item-name", fileItemNameBaseUX)}>{item.name}</span>
    </>
  );
  if (item.folder) {
    return (
      <button
        type="button"
        className={cx("ex-file-item-action", fileItemActionBaseUX, fileItemActionSkinX, selected ? "is-selected" : "")}
        onClick={() => onOpenFolder(item.folder?.path ?? ROOT_FOLDER)}
      >
        {content}
      </button>
    );
  }
  if (onOpenFile) {
    return (
      <button
        type="button"
        className={cx("ex-file-item-action", fileItemActionBaseUX, fileItemActionSkinX, selected ? "is-selected" : "")}
        onClick={() => onOpenFile(item.path)}
      >
        {content}
      </button>
    );
  }
  return (
    <a
      href={`#/files/${item.path}`}
      className={cx("ex-file-item-action", fileItemActionBaseUX, fileItemActionSkinX, selected ? "is-selected" : "")}
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
      <div className={cx(fileElementsBaseUX, fileElementsSkinX)}>
        <span className={cx(emptyBaseUX, emptySkinX)}>Select a file row to inspect its modeled elements.</span>
      </div>
    );
  }
  if (file.element_ids.length === 0) {
    return (
      <div className={cx(fileElementsBaseUX, fileElementsSkinX)}>
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
    <div className={cx(fileElementsBaseUX, fileElementsSkinX)}>
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
              className={cx("ex-list-row")}
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
