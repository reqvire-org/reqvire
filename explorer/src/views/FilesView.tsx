import { useEffect, useMemo, useState } from "react";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import type {
  ExplorerProjectStore,
  ProjectStoreElement,
  ProjectStoreFile,
} from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { useOptionalExplorerUiState, type ModelMode } from "../state/ExplorerUiState";
import { SourceCodePreview } from "../rendering/SourceCodePreview";
import { routeForContent } from "../router/routes";
import {
  FileBrowserElementsPanel,
  FileBrowserEmptyState,
  FileBrowserFrame,
  FileBrowserGrid,
  FileBrowserList,
  FileBrowserMissingFile,
  FileBrowserModeledElement,
  FileBrowserModeledElements,
  FileBrowserToolbar,
  type FileBrowserItem,
  type FileBrowserLayout,
  type FileBrowserMode,
  type FileBrowserSortDirection,
  type FileBrowserSortKey,
} from "@ds";

const ROOT_FOLDER = "__root__";

type FileLayout = FileBrowserLayout;
type SortKey = FileBrowserSortKey;
type SortDirection = FileBrowserSortDirection;

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
  const selectedFileLabel = selectedFile
    ? displayName(selectedFile.display_path || selectedFile.path)
    : undefined;

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
  const browserItems = useMemo<FileBrowserItem[]>(
    () =>
      items.map((item) => ({
        kind: item.kind,
        id: item.id,
        name: item.name,
        path: item.path,
        displayPath: item.displayPath,
        elementCount: item.elementCount,
        childCount: item.childCount,
        selected: isSelectedFileItem(item, selectedFile),
        emptyFile: item.kind === "file" && item.elementCount === 0,
        href: item.kind === "file" ? `#/files/${item.path}` : undefined,
        contentHref: item.kind === "file" ? routeForContent(item.path) : undefined,
      })),
    [items, selectedFile],
  );

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

  function changeLayout(nextLayout: FileBrowserMode) {
    if (stateDriven && ui) {
      ui.setModelMode(nextLayout as ModelMode);
      return;
    }
    if (nextLayout !== "graph") {
      setLocalLayout(nextLayout);
    }
  }

  return (
    <ViewFrame testId="files">
      <FileBrowserFrame>
        <FileBrowserToolbar
          breadcrumbs={folderCrumbs(currentFolder, store.project.root_label || "Project root")}
          selectedFile={
            selectedFile && selectedFileLabel
              ? {
                  name: selectedFileLabel,
                  title: selectedFile.display_path || selectedFile.path,
                }
              : undefined
          }
          layout={layout}
          resultCount={items.length}
          onOpenFolder={openFolder}
          onLayoutChange={changeLayout}
        />

        {path && !selectedFile ? (
          <FileBrowserMissingFile path={path} />
        ) : (
          <>
            {layout === "list" ? (
              <FileBrowserList
                items={browserItems}
                sortKey={sortKey}
                sortDirection={sortDirection}
                onSort={updateSort}
                onOpenFolder={openFolder}
                onOpenFile={stateDriven ? openFile : undefined}
              />
            ) : (
              <FileBrowserGrid
                items={browserItems}
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
      </FileBrowserFrame>
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
      <FileBrowserElementsPanel>
        <FileBrowserEmptyState>Select a file row to inspect its modeled elements.</FileBrowserEmptyState>
      </FileBrowserElementsPanel>
    );
  }
  if (file.element_ids.length === 0) {
    return (
      <FileBrowserElementsPanel>
        <SourceCodePreview
          path={file.path}
          content={file.markdown_content}
          kind="source file"
          defaultExpanded
          showPath
        />
      </FileBrowserElementsPanel>
    );
  }
  return (
    <FileBrowserElementsPanel>
      <FileBrowserModeledElements layout={layout}>
        {file.element_ids.map((id) => {
          const element = elementById(id);
          return (
            <FileBrowserModeledElement
              key={id}
              layout={layout}
              name={element?.name ?? id}
              type={element?.element_type}
              family={element?.type_family}
              onOpen={() => onOpenElement(id)}
            />
          );
        })}
      </FileBrowserModeledElements>
    </FileBrowserElementsPanel>
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
