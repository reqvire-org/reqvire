import { fileDisplayName as displayName, type FolderNode, type FileManagerModel } from "../lib/fileTrees";
import { useEffect, useMemo, useState } from "react";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import type {
  ProjectStoreElement,
  ProjectStoreFile,
} from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { useOptionalExplorerUiState, type ModelMode } from "../state/ExplorerUiState";
import { SourceCodePreview } from "../rendering/SourceCodePreview";
import { routeForContent, routeForFile } from "../router/routes";
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
  const { store, elementById, getFileManagerModel } = useStore();
  const ui = useOptionalExplorerUiState();
  const model = getFileManagerModel();
  const stateDriven = Boolean(forcedLayout && ui);
  const modelSelectionId = ui?.modelSelectionId ?? "__root__";
  const selectedFile = stateDriven
    ? selectedFileFromModelSelection(modelSelectionId, model, elementById)
    : path
      ? model.fileByPath.get(path)
      : undefined;
  const [currentFolderPath, setCurrentFolderPath] = useState(ROOT_FOLDER);
  const [localLayout, setLocalLayout] = useState<FileLayout>("list");
  const [sortKey, setSortKey] = useState<SortKey>("name");
  const [sortDirection, setSortDirection] = useState<SortDirection>("asc");

  useEffect(() => {
    if (stateDriven) {
      const folderPath = folderPathFromModelSelection(modelSelectionId, model, elementById);
      setCurrentFolderPath(folderPath);
      return;
    }
    if (!selectedFile) return;
    const nextFolder = selectedFile.parent_folder || ROOT_FOLDER;
    setCurrentFolderPath(model.folderByPath.has(nextFolder) ? nextFolder : ROOT_FOLDER);
  }, [model, modelSelectionId, selectedFile, stateDriven, elementById]);

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
        href: item.kind === "file" ? routeForFile(item.path) : undefined,
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
    if (nextLayout === "graph" || nextLayout === "flow") {
      if (ui) {
        ui.setModelSelectionId(selectedFile ? `file:${selectedFile.path}` : currentFolderPath === ROOT_FOLDER ? "__root__" : `folder:${currentFolderPath}`);
        ui.setModelMode(nextLayout);
        window.location.hash = "#/model";
      }
    } else {
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
          showResultCount={!stateDriven}
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

function selectedFileFromModelSelection(
  selectionId: string,
  model: FileManagerModel,
  elementById: (id: string) => ProjectStoreElement | undefined,
): ProjectStoreFile | undefined {
  if (selectionId.startsWith("file:")) {
    return model.fileByPath.get(selectionId.slice("file:".length));
  }
  const element = elementById(selectionId);
  return element ? model.fileByPath.get(element.file_path) : undefined;
}

function folderPathFromModelSelection(
  selectionId: string,
  model: FileManagerModel,
  elementById: (id: string) => ProjectStoreElement | undefined,
): string {
  if (selectionId === "__root__") return ROOT_FOLDER;
  if (selectionId.startsWith("folder:")) {
    const folderPath = selectionId.slice("folder:".length);
    return model.folderByPath.has(folderPath) ? folderPath : ROOT_FOLDER;
  }
  const selectedFile = selectedFileFromModelSelection(selectionId, model, elementById);
  if (selectedFile) {
    const folderPath = selectedFile.parent_folder || ROOT_FOLDER;
    return model.folderByPath.has(folderPath) ? folderPath : ROOT_FOLDER;
  }
  return ROOT_FOLDER;
}
