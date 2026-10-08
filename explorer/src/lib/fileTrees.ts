import type { ProjectStoreFile, ProjectStoreFolder, ProjectStoreProject, ProjectStoreResource } from "../store/types";

// Sidebar grouping and file-manager folders intentionally retain separate projections.
export interface TreeFolder {
  path: string;
  name: string;
  selectionId?: string;
  folders: TreeFolder[];
  files: ProjectStoreFile[];
  resources: ProjectStoreResource[];
}

export const ROOT_PATH = "__root__";
export const MODEL_ROOT_PATH = "__model__";
export const RESOURCE_ROOT_PATH = "__resources__";
const MODEL_WORKTREE_ROOT = "__model_worktree__";
const RESOURCE_WORKTREE_ROOT = "__resource_worktree__";

export function buildProjectFileTree(files: ProjectStoreFile[], resources: ProjectStoreResource[], worktrees: ProjectWorktree[]): TreeFolder {
  const root: TreeFolder = { path: ROOT_PATH, name: "Workspace", folders: [], files: [], resources: [] };
  const modelRoot: TreeFolder = {
    path: MODEL_ROOT_PATH,
    name: "Model",
    selectionId: "__root__",
    folders: [],
    files: [],
    resources: [],
  };
  root.folders.push(modelRoot);
  const modelFolders = new Map<string, TreeFolder>([
    [ROOT_PATH, root],
    [MODEL_ROOT_PATH, modelRoot],
  ]);
  const modelWorktreeRoots = new Map(
    worktrees.map((worktree) => {
      const folder: TreeFolder = {
        path: `${MODEL_WORKTREE_ROOT}/${worktree.key}`,
        name: worktree.label,
        selectionId: worktree.prefix ? `folder:${worktree.prefix}` : "__root__",
        folders: [],
        files: [],
        resources: [],
      };
      modelRoot.folders.push(folder);
      modelFolders.set(folder.path, folder);
      return [worktree.key, folder] as const;
    }),
  );

  for (const file of files) {
    const worktree = worktreeForPath(worktrees, file.path);
    const worktreeRoot = modelWorktreeRoots.get(worktree.key) ?? modelRoot;
    const relativeFolder = stripWorktreePrefix(file.parent_folder || "", worktree.prefix);
    ensureVirtualFolder(relativeFolder, modelFolders, worktreeRoot, worktreeRoot.path, worktree.prefix, "model").files.push(file);
  }

  if (resources.length > 0) {
    const resourceRoot: TreeFolder = {
      path: RESOURCE_ROOT_PATH,
      name: "Resources",
      selectionId: "resource-root",
      folders: [],
      files: [],
      resources: [],
    };
    root.folders.push(resourceRoot);
    const resourceFolders = new Map<string, TreeFolder>([[RESOURCE_ROOT_PATH, resourceRoot]]);
    const resourceWorktreeRoots = new Map(
      worktrees.map((worktree) => {
        const folder: TreeFolder = {
          path: `${RESOURCE_WORKTREE_ROOT}/${worktree.key}`,
          name: worktree.label,
          selectionId: worktree.prefix ? `resource-folder:${worktree.prefix}` : "resource-root",
          folders: [],
          files: [],
          resources: [],
        };
        resourceRoot.folders.push(folder);
        resourceFolders.set(folder.path, folder);
        return [worktree.key, folder] as const;
      }),
    );
    for (const resource of resources) {
      if (!resource.file_path) {
        ensureVirtualFolder("External", resourceFolders, resourceRoot, RESOURCE_ROOT_PATH, "", "resource").resources.push(resource);
        continue;
      }
      const worktree = worktreeForPath(worktrees, resource.file_path);
      const worktreeRoot = resourceWorktreeRoots.get(worktree.key) ?? resourceRoot;
      const relativeFolder = stripWorktreePrefix(dirname(resource.file_path), worktree.prefix);
      ensureVirtualFolder(relativeFolder, resourceFolders, worktreeRoot, worktreeRoot.path, worktree.prefix, "resource").resources.push(resource);
    }
    for (const folder of resourceFolders.values()) {
      folder.folders.sort((a, b) => a.name.localeCompare(b.name));
      folder.resources.sort((a, b) => resourceLabel(a).localeCompare(resourceLabel(b)));
    }
  }

  for (const folder of modelFolders.values()) {
    folder.folders.sort((a, b) => a.name.localeCompare(b.name));
    folder.files.sort((a, b) =>
      displayName(a.display_path || a.path).localeCompare(displayName(b.display_path || b.path)),
    );
  }

  return root;
}

export interface ProjectWorktree {
  key: string;
  label: string;
  prefix: string;
}

export function projectWorktrees(project: ProjectStoreProject): ProjectWorktree[] {
  const source = project.eligible_git_worktrees.length > 0
    ? project.eligible_git_worktrees
    : [{ workspace_relative_root: "." }];
  const worktrees = source.map((worktree, index) => {
    const prefix = normalizeWorkspaceRelativeRoot(worktree.workspace_relative_root);
    const fallbackName = index === 0
      ? project.repository || basename(project.workspace_root) || project.name
      : "";
    const label = prefix ? displayName(prefix) : fallbackName || "workspace";
    const key = prefix || label || `worktree-${index + 1}`;
    return { key, label, prefix };
  });

  return worktrees.sort((a, b) => b.prefix.length - a.prefix.length || a.label.localeCompare(b.label));
}

function normalizeWorkspaceRelativeRoot(path: string | null | undefined) {
  const normalized = (path || "").replaceAll("\\", "/").replace(/^\/+|\/+$/g, "");
  return normalized === "." ? "" : normalized;
}

function basename(path: string | null | undefined) {
  const normalized = normalizeWorkspaceRelativeRoot(path);
  if (!normalized) return "";
  return displayName(normalized);
}

function worktreeForPath(worktrees: ProjectWorktree[], path: string): ProjectWorktree {
  const normalized = normalizeWorkspaceRelativeRoot(path);
  return worktrees.find((worktree) =>
    worktree.prefix === "" ||
    normalized === worktree.prefix ||
    normalized.startsWith(`${worktree.prefix}/`)
  ) ?? worktrees[0];
}

function stripWorktreePrefix(path: string, prefix: string) {
  const normalized = normalizeWorkspaceRelativeRoot(path);
  if (!prefix) return normalized;
  if (normalized === prefix) return "";
  return normalized.startsWith(`${prefix}/`) ? normalized.slice(prefix.length + 1) : normalized;
}

function ensureVirtualFolder(
  relativePath: string,
  byPath: Map<string, TreeFolder>,
  root: TreeFolder,
  rootPath: string,
  actualPrefix: string,
  kind: "model" | "resource",
): TreeFolder {
  const normalized = normalizeWorkspaceRelativeRoot(relativePath);
  if (!normalized) return root;
  const path = `${rootPath}/${normalized}`;
  const existing = byPath.get(path);
  if (existing) return existing;
  const parentPath = dirname(normalized);
  const parent = ensureVirtualFolder(parentPath, byPath, root, rootPath, actualPrefix, kind);
  const actualPath = [actualPrefix, normalized].filter(Boolean).join("/");
  const folder: TreeFolder = {
    path,
    name: displayName(normalized),
    selectionId: kind === "model" ? `folder:${actualPath}` : `resource-folder:${actualPath}`,
    folders: [],
    files: [],
    resources: [],
  };
  byPath.set(path, folder);
  parent.folders.push(folder);
  return folder;
}

function resourceLabel(resource: ProjectStoreResource) {
  return resource.display || displayName(resource.file_path || resource.target);
}

export function displayName(path: string) {
  const normalized = path.replace(/\\/g, "/").replace(/\/$/, "");
  return normalized.split("/").pop() || normalized || "Project";
}

export function dirname(path: string) {
  const normalized = path.replace(/\\/g, "/").replace(/\/$/, "");
  const index = normalized.lastIndexOf("/");
  return index > 0 ? normalized.slice(0, index) : "";
}

const ROOT_FOLDER = ROOT_PATH;

export interface FolderNode {
  kind: "folder";
  id: string;
  path: string;
  name: string;
  parent: string | null;
  folders: FolderNode[];
  files: ProjectStoreFile[];
}

export interface FileManagerModel {
  root: FolderNode;
  folderByPath: Map<string, FolderNode>;
  fileByPath: Map<string, ProjectStoreFile>;
  folderElementCounts: Map<string, number>;
}

export function buildFileManagerModel(files: ProjectStoreFile[], folders: ProjectStoreFolder[], rootLabel: string): FileManagerModel {
  const root: FolderNode = {
    kind: "folder",
    id: "folder:__root__",
    path: ROOT_FOLDER,
    name: rootLabel || "Project root",
    parent: null,
    folders: [],
    files: [],
  };
  const folderByPath = new Map<string, FolderNode>([[ROOT_FOLDER, root]]);

  for (const folder of folders) {
    const folderPath = normalizeFolderPath(folder.path);
    if (folderPath === ROOT_FOLDER) continue;
    folderByPath.set(folderPath, {
      kind: "folder",
      id: `folder:${folderPath}`,
      path: folderPath,
      name: fileDisplayName(folderPath),
      parent: normalizeFolderPath(folder.parent),
      folders: [],
      files: [],
    });
  }

  for (const folder of folders) {
    const folderPath = normalizeFolderPath(folder.path);
    if (folderPath === ROOT_FOLDER) continue;
    const node = folderByPath.get(folderPath);
    if (!node) continue;
    const parentPath = normalizeFolderPath(folder.parent);
    const parent = parentPath === ROOT_FOLDER ? root : folderByPath.get(parentPath);
    (parent ?? root).folders.push(node);
  }

  const fileByPath = new Map<string, ProjectStoreFile>();
  for (const file of files) {
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

export function fileDisplayName(path: string): string {
  return path.split("/").filter(Boolean).at(-1) ?? path;
}
