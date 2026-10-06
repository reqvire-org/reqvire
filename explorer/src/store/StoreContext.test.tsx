import { renderHook } from "@testing-library/react";
import type { ReactNode } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as modelFlow from "../lib/modelFlow";
import * as traces from "../lib/traces";
import * as fileTrees from "../lib/fileTrees";
import { StoreProvider, useStore } from "./StoreContext";
import { devFixture } from "./devFixture";
import type { ExplorerProjectStore } from "./types";

afterEach(() => vi.restoreAllMocks());

function mount(initial: ExplorerProjectStore) {
  let store = initial;
  const hook = renderHook(useStore, { wrapper: ({ children }: { children: ReactNode }) =>
    <StoreProvider store={store} schemaMismatch={null}>{children}</StoreProvider>,
  });
  return { result: hook.result, update(next: ExplorerProjectStore) { store = next; hook.rerender(); } };
}

describe("derived store inputs", () => {
  it("shares lazy file projections across consumers, report updates, and Git metadata changes", () => {
    const sidebarBuild = vi.spyOn(fileTrees, "buildProjectFileTree");
    const managerBuild = vi.spyOn(fileTrees, "buildFileManagerModel");
    const { result, update } = mount(devFixture);
    expect(sidebarBuild).not.toHaveBeenCalled();
    expect(managerBuild).not.toHaveBeenCalled();
    const sidebar = result.current.getProjectFileTree();
    expect(managerBuild).not.toHaveBeenCalled();
    const manager = result.current.getFileManagerModel();
    expect(result.current.getProjectFileTree()).toBe(sidebar);
    expect(result.current.getFileManagerModel()).toBe(manager);
    update({ ...devFixture, coverage: { ...devFixture.coverage },
      project: { ...devFixture.project, branch: "renamed-branch", eligible_git_worktrees:
        devFixture.project.eligible_git_worktrees.map(worktree => ({ ...worktree, head: "new-head", dirty: true })) },
    });
    expect(result.current.getProjectFileTree()).toBe(sidebar);
    expect(result.current.getFileManagerModel()).toBe(manager);
    expect(sidebarBuild).toHaveBeenCalledTimes(1);
    expect(managerBuild).toHaveBeenCalledTimes(1);
  });

  it("invalidates only the affected file projection for folder, resource, and label changes", () => {
    const { result, update } = mount(devFixture);
    const sidebar = result.current.getProjectFileTree();
    const manager = result.current.getFileManagerModel();
    let edited = { ...devFixture, folders: [...devFixture.folders, { path: "empty", parent: null, children: [] }] };
    update(edited);
    const withFolder = result.current.getFileManagerModel();
    expect(withFolder.folderByPath.get("empty")?.files).toEqual([]);
    expect(withFolder.folderElementCounts.get("empty")).toBe(0);
    expect(withFolder.root.folders.some(folder => folder.path === "empty")).toBe(true);
    expect(withFolder).not.toBe(manager);
    expect(result.current.getProjectFileTree()).toBe(sidebar);
    edited = { ...edited, resources: [] };
    update(edited);
    const withoutResources = result.current.getProjectFileTree();
    expect(withoutResources.folders.map(folder => folder.name)).toEqual(["Model"]);
    expect(result.current.getFileManagerModel()).toBe(withFolder);
    edited = { ...edited, project: { ...edited.project, repository: "renamed-repository" } };
    update(edited);
    expect(result.current.getProjectFileTree().folders[0].folders[0].name).toBe("renamed-repository");
    expect(result.current.getFileManagerModel()).toBe(withFolder);
    const renamedSidebar = result.current.getProjectFileTree();
    edited = { ...edited, project: { ...edited.project, root_label: "New title" } };
    update(edited);
    expect(result.current.getFileManagerModel().root.name).toBe("New title");
    expect(result.current.getProjectFileTree()).toBe(renamedSidebar);
    expect(sidebar.folders.some(folder => folder.name === "Resources")).toBe(true);
    expect(manager.folderByPath.has("empty")).toBe(false);
  });

  it("adopts file additions, moves, source changes and deletions with current membership and counts", () => {
    const original = devFixture.files[0];
    const { result, update } = mount({ ...devFixture, files: [original] });
    const prior = result.current.getFileManagerModel();
    const moved = { ...original, path: "moved/New.md", display_path: "moved/New.md", parent_folder: "moved",
      markdown_content: "updated source", element_ids: ["new-id", "second-id"] };
    let edited = { ...devFixture, files: [moved], folders: [{ path: "moved", parent: null, children: [moved.path] }] };
    update(edited);
    const manager = result.current.getFileManagerModel();
    expect(manager.fileByPath.get(original.path)).toBeUndefined();
    expect(manager.folderByPath.get("moved")?.files).toEqual([moved]);
    expect(manager.fileByPath.get(moved.path)?.markdown_content).toBe("updated source");
    expect(manager.folderElementCounts.get("moved")).toBe(2);
    expect(manager.folderElementCounts.get("__root__")).toBe(2);
    const worktree = result.current.getProjectFileTree().folders[0].folders[0];
    expect(worktree.folders[0]).toMatchObject({ name: "moved", selectionId: "folder:moved", files: [moved] });
    edited = { ...edited, files: [] };
    update(edited);
    expect(result.current.getFileManagerModel().folderElementCounts.get("__root__")).toBe(0);
    expect(result.current.getFileManagerModel().fileByPath.size).toBe(0);
    expect(result.current.getProjectFileTree().folders[0].folders[0].folders).toEqual([]);
    expect(prior.fileByPath.get(original.path)).toBe(original);
  });

  it("preserves nested worktree grouping, prefix boundaries, and external resources when roots change", () => {
    const file = devFixture.files[0];
    const paths = ["top.md", "nested/A.md", "nested/child/B.md", "nested-other/C.md"];
    const initial: ExplorerProjectStore = { ...devFixture, files: paths.map(path => ({ ...file, path, display_path: path,
      parent_folder: path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : "" })),
      resources: [
        { ...devFixture.resources[0], id: "local", file_path: "nested/child/code.rs", target: "nested/child/code.rs" },
        { ...devFixture.resources[0], id: "external", file_path: null, target: "https://example.org/evidence" },
      ],
    };
    const { result, update } = mount(initial);
    const original = result.current.getProjectFileTree();
    update({ ...initial, project: { ...initial.project, eligible_git_worktrees: [".", "nested", "nested/child"].map(root => ({
      root: `/workspace/${root}`, workspace_relative_root: root, dirty: false,
    })) } });
    const tree = result.current.getProjectFileTree();
    expect(tree).not.toBe(original);
    const groups = tree.folders[0].folders;
    expect(groups.find(group => group.name === "child")?.files.map(entry => entry.path)).toEqual(["nested/child/B.md"]);
    expect(groups.find(group => group.name === "nested")?.files.map(entry => entry.path)).toEqual(["nested/A.md"]);
    const root = groups.find(group => group.selectionId === "__root__")!;
    expect(root.files.map(entry => entry.path)).toEqual(["top.md"]);
    expect(root.folders[0]).toMatchObject({ name: "nested-other", selectionId: "folder:nested-other" });
    const resources = tree.folders[1].folders;
    expect(resources.find(group => group.name === "child")).toMatchObject({ selectionId: "resource-folder:nested/child",
      resources: [expect.objectContaining({ id: "local" })] });
    expect(resources.find(group => group.name === "External")?.resources[0].id).toBe("external");
  });

  it.each(["workspace_root", "worktree_id"] as const)("isolates file projections when %s changes", key => {
    const { result, update } = mount(devFixture);
    const sidebar = result.current.getProjectFileTree();
    const manager = result.current.getFileManagerModel();
    update({ ...devFixture, project: { ...devFixture.project, [key]: "other-context" } });
    expect(result.current.getProjectFileTree()).not.toBe(sidebar);
    expect(result.current.getFileManagerModel()).not.toBe(manager);
  });

  it("retains the element lookup across report-only refreshes", () => {
    const { result, update } = mount(devFixture);
    const lookup = result.current.elementById;
    update({ ...devFixture, coverage: { ...devFixture.coverage } });
    expect(result.current.elementById).toBe(lookup);
    expect(result.current.elementById(devFixture.elements[0].id)).toBe(devFixture.elements[0]);
  });

  it("adopts edited and removed elements without changing the prior snapshot", () => {
    const { result, update } = mount(devFixture);
    const prior = result.current.elementById;
    const original = devFixture.elements[0];
    const edited = { ...original, name: "Renamed", file_path: "moved/File.md", element_type: "capability" };
    update({ ...devFixture, elements: [edited] });
    expect(result.current.elementById(original.id)).toBe(edited);
    expect(result.current.elementById(devFixture.elements[1].id)).toBeUndefined();
    expect(prior(original.id)).toBe(original);
  });

  it.each(["workspace_root", "worktree_id"] as const)("isolates the lookup when %s changes", key => {
    const { result, update } = mount(devFixture);
    const prior = result.current.elementById;
    update({ ...devFixture, project: { ...devFixture.project, [key]: "another-context" } });
    expect(result.current.elementById).not.toBe(prior);
    expect(result.current.elementById(devFixture.elements[0].id)).toBe(devFixture.elements[0]);
  });

  it("prepares trace grouping once on demand and reuses it across consumers and report changes", () => {
    const build = vi.spyOn(traces, "buildTraceFiles");
    const initial: ExplorerProjectStore = { ...devFixture, traces: { files: { "Checks.md": {
      verifications: [{ identifier: "Checks.md#check", name: "Check", file: "Checks.md",
        directly_verified_count: 1, total_requirements_in_tree: 1 }],
    } } } };
    const { result, update } = mount(initial);
    expect(build).not.toHaveBeenCalled();
    const grouped = result.current.getTraceFiles();
    expect(result.current.getTraceFiles()).toBe(grouped);
    update({ ...initial, coverage: { ...initial.coverage }, elements: [...initial.elements] });
    expect(result.current.getTraceFiles()).toBe(grouped);
    expect(build).toHaveBeenCalledTimes(1);
    const file = "Checks.md";
    const original = initial.traces.files[file];
    update({ ...initial, traces: { files: { [file]: { ...original,
      verifications: [{ ...original.verifications[0], name: "Updated verification" }],
    } } } });
    expect(result.current.getTraceFiles()[0].verifications[0].name).toBe("Updated verification");
    expect(result.current.getTraceFiles()).not.toBe(grouped);
    expect(build).toHaveBeenCalledTimes(2);
  });

  it("invalidates fallback trace grouping when element or relation inputs change", () => {
    const initial = { ...devFixture, traces: { files: {} } };
    const { result, update } = mount(initial);
    const grouped = result.current.getTraceFiles();
    const verification = initial.elements.find(element => traces.isVerification(element))!;
    const edited = { ...initial, elements: initial.elements.map(element => element.id === verification.id
      ? { ...element, name: "Moved check", file_path: "checks/Moved.md" } : element) };
    update(edited);
    const moved = result.current.getTraceFiles().find(file => file.file === "checks/Moved.md")!;
    expect(moved.verifications[0].name).toBe("Moved check");
    expect(result.current.getTraceFiles()).not.toBe(grouped);
    update({ ...edited, relations: [] });
    expect(result.current.getTraceFiles().flatMap(file => file.verifications).every(check => check.directCount === 0)).toBe(true);
    expect(result.current.getTraceFiles().find(file => file.file === "checks/Moved.md")).not.toBe(moved);
  });

  it("prepares Flow once for repeated scopes and ignores unrelated report and root-label changes", () => {
    const prepare = vi.spyOn(modelFlow, "prepareModelFlow");
    let relationReads = 0;
    const initial = { ...devFixture, relations: devFixture.relations.map(relation => ({
      ...relation, get source_id() { relationReads++; return relation.source_id; },
    })) };
    const { result, update } = mount(initial);
    expect(prepare).not.toHaveBeenCalled();
    const index = result.current.getModelFlowIndex();
    const preparedReads = relationReads;
    expect(preparedReads).toBeGreaterThan(0);
    for (const selection of ["__root__", "folder:system-model", ...devFixture.elements.map(element => element.id)]) {
      const selected = modelFlow.selectModelFlow(result.current.getModelFlowIndex(), selection, initial.project.root_label);
      expect(selected.id).toBe(selection);
      if (initial.elements.some(element => element.id === selection)) {
        expect(selected.nodes.some(node => node.element.id === selection)).toBe(true);
      }
    }
    expect(relationReads).toBe(preparedReads);
    expect(prepare).toHaveBeenCalledTimes(1);
    update({ ...initial, project: { ...initial.project, root_label: "Renamed workspace" },
      traces: { ...initial.traces }, coverage: { ...initial.coverage } });
    expect(result.current.getModelFlowIndex()).toBe(index);
    expect(modelFlow.selectModelFlow(index, "__root__", result.current.store.project.root_label).title).toBe("Renamed workspace");
    expect(prepare).toHaveBeenCalledTimes(1);
  });

  it("updates Flow labels, types, source files and root membership while leaving old projections intact", () => {
    const { result, update } = mount(devFixture);
    const original = devFixture.elements[0];
    const before = modelFlow.selectModelFlow(result.current.getModelFlowIndex(), original.id, devFixture.project.root_label);
    update({ ...devFixture, elements: [{ ...original, name: "New label", element_type: "capability",
      file_path: "moved/Changed.md", source_anchor: "" }], relations: [], resources: [],
      contract_bindings: [], contract_references: [], concept_refs: [],
    });
    const data = modelFlow.selectModelFlow(result.current.getModelFlowIndex(), "__root__", devFixture.project.root_label);
    expect(data.nodes).toHaveLength(1);
    expect(data.nodes[0]).toMatchObject({ type: "capability", root: true,
      element: { id: original.id, name: "New label", file: "moved/Changed.md", sourceHref: "#/content/moved/Changed.md" },
    });
    expect(data.edges).toEqual([]);
    expect(before.nodes.find(node => node.element.id === original.id)?.element.name).toBe(original.name);
  });

  it.each(["elements", "resources", "relations", "contract_bindings", "contract_references", "concept_refs"] as const)(
    "rebuilds Flow for a changed %s section", section => {
      const { result, update } = mount(devFixture);
      const index = result.current.getModelFlowIndex();
      const edited = { ...devFixture, [section]: [] };
      update(edited);
      expect(result.current.getModelFlowIndex()).not.toBe(index);
    },
  );

  it.each(["workspace_root", "worktree_id"] as const)("isolates shared derived data across %s changes", key => {
    const { result, update } = mount(devFixture);
    const index = result.current.getModelFlowIndex();
    const grouped = result.current.getTraceFiles();
    update({ ...devFixture, project: { ...devFixture.project, [key]: "another-context" } });
    expect(result.current.getModelFlowIndex()).not.toBe(index);
    expect(result.current.getTraceFiles()).not.toBe(grouped);
  });
});
