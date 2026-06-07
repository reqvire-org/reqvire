import { useEffect, useMemo, useState } from "react";
import type { ReactNode } from "react";
import {
  Badge,
  Box,
  Code,
  Flex,
  Grid,
  Heading,
  Link,
  SegmentedControl,
  Text,
  TextField,
} from "@radix-ui/themes";
import {
  ArchiveIcon,
  CubeIcon,
  FileIcon,
  MagnifyingGlassIcon,
} from "@radix-ui/react-icons";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import type {
  ExplorerProjectStore,
  ProjectStoreFile,
} from "../store/types";
import { ViewFrame } from "./ViewFrame";

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
  const model = useMemo(() => buildFileManagerModel(store), [store]);
  const selectedFile = path ? model.fileByPath.get(path) : undefined;
  const [currentFolderPath, setCurrentFolderPath] = useState(ROOT_FOLDER);
  const [localLayout, setLocalLayout] = useState<FileLayout>("list");
  const [sortKey, setSortKey] = useState<SortKey>("name");
  const [sortDirection, setSortDirection] = useState<SortDirection>("asc");
  const [query, setQuery] = useState("");

  useEffect(() => {
    if (!selectedFile) return;
    const nextFolder = selectedFile.parent_folder || ROOT_FOLDER;
    setCurrentFolderPath(model.folderByPath.has(nextFolder) ? nextFolder : ROOT_FOLDER);
  }, [model.folderByPath, selectedFile]);

  const currentFolder = model.folderByPath.get(currentFolderPath) ?? model.root;
  const layout = forcedLayout ?? localLayout;
  const normalizedQuery = query.trim().toLowerCase();

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

    const directItems = [
      ...currentFolder.folders.map(folderToItem),
      ...currentFolder.files.map(fileToItem),
    ];

    const allItems = [
      ...Array.from(model.folderByPath.values())
        .filter((folder) => folder.path !== ROOT_FOLDER)
        .map(folderToItem),
      ...store.files.map(fileToItem),
    ];

    const source = normalizedQuery ? allItems : directItems;
    return source
      .filter((item) => {
        if (!normalizedQuery) return true;
        if (
          item.name.toLowerCase().includes(normalizedQuery) ||
          item.displayPath.toLowerCase().includes(normalizedQuery)
        ) {
          return true;
        }
        return item.file?.element_ids.some((id) => {
          const element = elementById(id);
          return (
            element?.name.toLowerCase().includes(normalizedQuery) ||
            id.toLowerCase().includes(normalizedQuery)
          );
        }) ?? false;
      })
      .sort((a, b) => compareItems(a, b, sortKey, sortDirection));
  }, [
    currentFolder,
    elementById,
    model.folderByPath,
    model.folderElementCounts,
    normalizedQuery,
    sortDirection,
    sortKey,
    store.files,
    store.project.root_label,
  ]);

  const searchResults = useMemo(
    () => (normalizedQuery ? items.slice(0, 12) : []),
    [items, normalizedQuery],
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
    setCurrentFolderPath(folderPath);
    setQuery("");
  }

  return (
    <ViewFrame testId="files">
      <Grid columns={{ initial: "1fr", lg: "minmax(0, 1fr) 390px" }} className="explorer-route">
        <Box className="explorer-document-panel file-manager-shell">
          <FileManagerToolbar
            currentFolder={currentFolder}
            rootLabel={store.project.root_label || "Project root"}
            query={query}
            layout={layout}
            resultCount={items.length}
            onOpenFolder={openFolder}
            onLayoutChange={forcedLayout ? undefined : setLocalLayout}
          />

          {path && !selectedFile ? (
            <Text color="gray">
              No file container for <Code>{path}</Code>.
            </Text>
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
                />
              ) : (
                <FileManagerGrid
                  items={items}
                  selectedFile={selectedFile}
                  onOpenFolder={openFolder}
                />
              )}

              <SelectedFileElements
                file={selectedFile}
                onOpenElement={onOpenElement}
                elementName={(id) => elementById(id)?.name ?? id}
              />
            </>
          )}
        </Box>

        <FileInspector
          file={selectedFile}
          folder={currentFolder}
          query={query}
          searchResults={searchResults}
          rootLabel={store.project.root_label || "Project root"}
          onQueryChange={setQuery}
          onOpenFolder={openFolder}
        />
      </Grid>
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
    folderByPath.set(folder.path, {
      kind: "folder",
      id: `folder:${folder.path}`,
      path: folder.path,
      name: displayName(folder.path),
      parent: folder.parent,
      folders: [],
      files: [],
    });
  }

  for (const folder of store.folders) {
    const node = folderByPath.get(folder.path);
    if (!node) continue;
    const parent = folder.parent ? folderByPath.get(folder.parent) : root;
    (parent ?? root).folders.push(node);
  }

  const fileByPath = new Map<string, ProjectStoreFile>();
  for (const file of store.files) {
    fileByPath.set(file.path, file);
    const parentPath = file.parent_folder || ROOT_FOLDER;
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

function FileManagerToolbar({
  currentFolder,
  rootLabel,
  query,
  layout,
  resultCount,
  onOpenFolder,
  onLayoutChange,
}: {
  currentFolder: FolderNode;
  rootLabel: string;
  query: string;
  layout: FileLayout;
  resultCount: number;
  onOpenFolder: (path: string) => void;
  onLayoutChange?: (layout: FileLayout) => void;
}) {
  const crumbs = folderCrumbs(currentFolder, rootLabel);
  return (
    <div className="file-manager-toolbar">
      <div className="file-manager-breadcrumbs" aria-label="File breadcrumbs">
        {crumbs.map((crumb, index) => (
          <span key={crumb.path} className="file-manager-crumb">
            {index > 0 && <span className="file-manager-crumb-separator">/</span>}
            <button type="button" onClick={() => onOpenFolder(crumb.path)}>
              {crumb.label}
            </button>
          </span>
        ))}
      </div>
      <Flex align="center" gap="3" wrap="wrap">
        <Text className="explorer-panel-muted">
          {query ? `${resultCount} search results` : `${resultCount} items`}
        </Text>
        {onLayoutChange && (
          <SegmentedControl.Root
            value={layout}
            onValueChange={(value) => onLayoutChange(value as FileLayout)}
          >
            <SegmentedControl.Item value="list">List</SegmentedControl.Item>
            <SegmentedControl.Item value="grid">Grid</SegmentedControl.Item>
          </SegmentedControl.Root>
        )}
      </Flex>
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
}: {
  items: FileManagerItem[];
  selectedFile: ProjectStoreFile | undefined;
  sortKey: SortKey;
  sortDirection: SortDirection;
  onSort: (key: SortKey) => void;
  onOpenFolder: (path: string) => void;
}) {
  return (
    <div className="file-manager-table-wrap">
      <table className="file-manager-table">
        <thead>
          <tr>
            <SortableHeader label="Name" sortKey="name" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Type" sortKey="type" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Elements" sortKey="elements" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Path" sortKey="path" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
          </tr>
        </thead>
        <tbody>
          {items.map((item) => (
            <tr key={item.id} className={isSelectedFileItem(item, selectedFile) ? "is-selected" : ""}>
              <td>
                <ItemAction item={item} onOpenFolder={onOpenFolder} selectedFile={selectedFile} />
              </td>
              <td>
                <Badge color="gray">{item.kind}</Badge>
              </td>
              <td>{item.elementCount}</td>
              <td className="file-manager-path">{item.displayPath}</td>
            </tr>
          ))}
        </tbody>
      </table>
      {items.length === 0 && <Text className="explorer-empty">No files or folders match the current filter.</Text>}
    </div>
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
    <th>
      <button type="button" onClick={() => onSort(sortKey)}>
        <span>{label}</span>
        {active && <span className="file-manager-sort">{direction}</span>}
      </button>
    </th>
  );
}

function FileManagerGrid({
  items,
  selectedFile,
  onOpenFolder,
}: {
  items: FileManagerItem[];
  selectedFile: ProjectStoreFile | undefined;
  onOpenFolder: (path: string) => void;
}) {
  return (
    <div className="file-manager-grid">
      {items.map((item) => (
        <div key={item.id} className={["file-manager-card", isSelectedFileItem(item, selectedFile) ? "is-selected" : ""].join(" ")}>
          <ItemAction item={item} onOpenFolder={onOpenFolder} selectedFile={selectedFile} />
          <Text className="explorer-panel-muted">
            {item.kind === "folder" ? `${item.childCount} children` : `${item.elementCount} elements`}
          </Text>
          <Text className="file-manager-card-path">{item.displayPath}</Text>
        </div>
      ))}
      {items.length === 0 && <Text className="explorer-empty">No files or folders match the current filter.</Text>}
    </div>
  );
}

function ItemAction({
  item,
  selectedFile,
  onOpenFolder,
}: {
  item: FileManagerItem;
  selectedFile: ProjectStoreFile | undefined;
  onOpenFolder: (path: string) => void;
}) {
  const content = (
    <>
      <span className={["explorer-icon-swatch", item.kind === "folder" ? "file-kind-folder" : "file-kind-file"].join(" ")}>
        {item.kind === "folder" ? <ArchiveIcon /> : <FileIcon />}
      </span>
      <span className="min-w-0 flex-1 truncate">{item.name}</span>
      {item.kind === "file" && <Code>{item.elementCount}</Code>}
    </>
  );
  if (item.folder) {
    return (
      <button type="button" className="file-manager-item-action" onClick={() => onOpenFolder(item.folder?.path ?? ROOT_FOLDER)}>
        {content}
      </button>
    );
  }
  return (
    <Link
      href={`#/files/${item.path}`}
      className={["file-manager-item-action", isSelectedFileItem(item, selectedFile) ? "is-selected" : ""].join(" ")}
    >
      {content}
    </Link>
  );
}

function isSelectedFileItem(item: FileManagerItem, selectedFile: ProjectStoreFile | undefined) {
  return Boolean(item.file && selectedFile && item.file.path === selectedFile.path);
}

function SelectedFileElements({
  file,
  onOpenElement,
  elementName,
}: {
  file: ProjectStoreFile | undefined;
  onOpenElement: (id: string) => void;
  elementName: (id: string) => string;
}) {
  if (!file) {
    return (
      <div className="file-manager-elements">
        <Text className="explorer-empty">Select a file row to inspect its modeled elements.</Text>
      </div>
    );
  }
  return (
    <div className="file-manager-elements">
      <Flex align="center" justify="between" gap="3" wrap="wrap">
        <Heading as="h2" size="3" className="explorer-panel-title">
          Modeled elements
        </Heading>
        <Link href={file.html_path} target="_blank" rel="noreferrer">
          Open exported source page
        </Link>
      </Flex>
      <Flex direction="column" gap="1" mt="3">
        {file.element_ids.map((id) => (
          <button
            key={id}
            type="button"
            onClick={() => onOpenElement(id)}
            className="explorer-list-row"
          >
            <span className="explorer-icon-swatch file-kind-element">
              <CubeIcon />
            </span>
            <Text size="2">{elementName(id)}</Text>
          </button>
        ))}
      </Flex>
      {file.element_ids.length === 0 && <Text className="explorer-empty">No modeled elements are attached to this file.</Text>}
    </div>
  );
}

function FileInspector({
  file,
  folder,
  query,
  searchResults,
  rootLabel,
  onQueryChange,
  onOpenFolder,
}: {
  file: ProjectStoreFile | undefined;
  folder: FolderNode;
  query: string;
  searchResults: FileManagerItem[];
  rootLabel: string;
  onQueryChange: (value: string) => void;
  onOpenFolder: (path: string) => void;
}) {
  return (
    <Box className="graph-sidebar">
      <div className="graph-search-panel">
        <TextField.Root
          aria-label="Search files"
          placeholder="Search files, folders, elements"
          value={query}
          onChange={(event) => onQueryChange(event.target.value)}
        >
          <TextField.Slot>
            <MagnifyingGlassIcon />
          </TextField.Slot>
        </TextField.Root>
        {searchResults.length > 0 && (
          <ul className="graph-results">
            {searchResults.map((item) => (
              <li key={item.id}>
                {item.folder ? (
                  <button type="button" onClick={() => onOpenFolder(item.folder?.path ?? ROOT_FOLDER)}>
                    <span className="graph-result-swatch file-result-folder" />
                    <span>{item.displayPath}</span>
                  </button>
                ) : (
                  <a href={`#/files/${item.path}`}>
                    <span className="graph-result-swatch file-result-file" />
                    <span>{item.displayPath}</span>
                  </a>
                )}
              </li>
            ))}
          </ul>
        )}
      </div>
      <div className="graph-inspector-header">
        <Heading as="h2" size="3">
          File Inspector
        </Heading>
      </div>
      <div className="graph-inspector-body">
        {file ? (
          <Flex direction="column" gap="3">
            <Box>
              <Heading as="h2" size="3" mb="2">
                {displayName(file.display_path || file.path)}
              </Heading>
              <Flex gap="2" wrap="wrap">
                <Badge color="gray">source file</Badge>
                <Code>{file.element_ids.length} elements</Code>
              </Flex>
            </Box>
            <Box>
              <Text size="1" color="gray" weight="bold">
                Path
              </Text>
              <Code className="block whitespace-normal break-words">{file.path}</Code>
            </Box>
            <Link href={file.html_path} target="_blank" rel="noreferrer">
              Open exported source page
            </Link>
          </Flex>
        ) : (
          <Flex direction="column" gap="3">
            <Box>
              <Heading as="h2" size="3" mb="2">
                {folder.path === ROOT_FOLDER ? rootLabel : folder.name}
              </Heading>
              <Flex gap="2" wrap="wrap">
                <Badge color="gray">folder</Badge>
                <Code>{folder.files.length} files</Code>
                <Code>{folder.folders.length} folders</Code>
              </Flex>
            </Box>
            <Text className="explorer-empty">Select a file to inspect its source path and modeled elements.</Text>
          </Flex>
        )}

        <Box mt="4">
          <Text size="1" color="gray" weight="bold">
            Legend
          </Text>
          <Flex direction="column" gap="2" mt="2">
            <LegendRow className="file-kind-folder" label="folder" icon={<ArchiveIcon />} />
            <LegendRow className="file-kind-file" label="source file" icon={<FileIcon />} />
            <LegendRow className="file-kind-element" label="modeled element" icon={<CubeIcon />} />
          </Flex>
        </Box>
      </div>
    </Box>
  );
}

function LegendRow({
  className,
  label,
  icon,
}: {
  className: string;
  label: string;
  icon: ReactNode;
}) {
  return (
    <Flex align="center" gap="2">
      <span className={["explorer-icon-swatch", className].join(" ")}>{icon}</span>
      <Text size="1">{label}</Text>
    </Flex>
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
