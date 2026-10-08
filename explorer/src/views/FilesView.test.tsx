import { fireEvent, render, screen, within } from "@testing-library/react";
import { useEffect } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { FilesView } from "./FilesView";
import { ExplorerSidePane } from "../components/ExplorerSidePane";
import { ExplorerUiStateProvider, useExplorerUiState } from "../state/ExplorerUiState";
import type { ExplorerProjectStore } from "../store/types";
import * as fileTrees from "../lib/fileTrees";

afterEach(() => vi.restoreAllMocks());

function SelectedElement({ id }: { id: string }) {
  const { setModelSelectionId } = useExplorerUiState();
  useEffect(() => setModelSelectionId(id), [id, setModelSelectionId]);
  return null;
}

function fileScene(store: ExplorerProjectStore, view: "files" | "model" | "coverage") {
  return <StoreProvider store={store} schemaMismatch={null}>
    <ExplorerUiStateProvider>
      {view === "files" ? <FilesView path={null} onOpenElement={vi.fn()} /> :
        <ExplorerSidePane activeView={view} open onToggle={vi.fn()} onNavigate={vi.fn()}
          onOpenElement={vi.fn()} onOpenOntologyNode={vi.fn()} />}
    </ExplorerUiStateProvider>
  </StoreProvider>;
}

function renderFiles(path: string | null = null) {
  const onOpenElement = vi.fn();
  const rendered = render(
    <StoreProvider store={devFixture} schemaMismatch={null}>
      <FilesView path={path} onOpenElement={onOpenElement} />
    </StoreProvider>,
  );
  return { ...rendered, onOpenElement };
}

function renderFilesWithStore(store: typeof devFixture, path: string | null = null) {
  return render(
    <StoreProvider store={store} schemaMismatch={null}>
      <FilesView path={path} onOpenElement={vi.fn()} />
    </StoreProvider>,
  );
}

beforeEach(() => { localStorage.clear(); window.history.replaceState(null, "", "/#/model"); });

describe("FilesView", () => {
  it("keeps filtering and the selected element current without rebuilding file topology", () => {
    const sidebarBuild = vi.spyOn(fileTrees, "buildProjectFileTree");
    const managerBuild = vi.spyOn(fileTrees, "buildFileManagerModel");
    const onOpenElement = vi.fn();
    const original = { ...devFixture.elements[0], id: "model/A.md#target", name: "Original label",
      file_path: "model/A.md", element_type: "requirement", type_family: "requirement", content: "Original content" };
    const initial: ExplorerProjectStore = { ...devFixture, elements: [original], resources: [],
      files: [{ path: "model/A.md", display_path: "model/A.md", parent_folder: "model",
        element_ids: [original.id], resource_ids: [], markdown_content: "source" }],
      folders: [{ path: "model", parent: null, children: ["model/A.md"] }],
    };
    const scene = (store: ExplorerProjectStore) => <StoreProvider store={store} schemaMismatch={null}>
      <ExplorerUiStateProvider>
        <SelectedElement id={original.id} />
        <ExplorerSidePane activeView="model" open onToggle={vi.fn()} onNavigate={vi.fn()}
          onOpenElement={onOpenElement} onOpenOntologyNode={vi.fn()} />
        <FilesView path={null} forcedLayout="list" onOpenElement={onOpenElement} />
      </ExplorerUiStateProvider>
    </StoreProvider>;
    const { rerender } = render(scene(initial));
    expect(screen.getByRole("button", { name: /Original label/ })).toBeTruthy();
    fireEvent.change(screen.getByRole("searchbox", { name: "Filter project tree" }), { target: { value: "Renamed target" } });
    const tree = screen.getByRole("tree", { name: "Project tree" });
    expect(within(tree).queryByText("Original label")).toBeNull();
    const edited = { ...initial, elements: [{ ...original, name: "Renamed target", element_type: "capability", type_family: "capability" }] };
    rerender(scene(edited));
    expect(within(tree).getByText("Renamed target")).toBeTruthy();
    fireEvent.change(screen.getByRole("searchbox", { name: "Filter project tree" }), { target: { value: "capability" } });
    expect(within(tree).getByText("Renamed target")).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: /Renamed target/ }));
    expect(onOpenElement).toHaveBeenCalledWith(original.id);
    expect(sidebarBuild).toHaveBeenCalledTimes(1);
    expect(managerBuild).toHaveBeenCalledTimes(1);
  });

  it("refreshes selected file records and source links across moves and context switches", () => {
    const file = devFixture.files[0];
    const original = { ...devFixture.elements[0], id: "shared-id", name: "First branch", file_path: "old/A.md" };
    const initial = { ...devFixture, elements: [original],
      files: [{ ...file, path: original.file_path, display_path: original.file_path, parent_folder: "old", element_ids: [original.id] }],
      folders: [{ path: "old", parent: null, children: [original.file_path] }],
    };
    const scene = (store: ExplorerProjectStore) => <StoreProvider store={store} schemaMismatch={null}>
      <ExplorerUiStateProvider><SelectedElement id={original.id} />
        <FilesView path={null} forcedLayout="list" onOpenElement={vi.fn()} />
      </ExplorerUiStateProvider>
    </StoreProvider>;
    const { rerender } = render(scene(initial));
    expect(screen.getByRole("button", { name: /First branch/ })).toBeTruthy();
    const moved = { ...original, name: "Moved element", file_path: "new/B.md" };
    let edited = { ...initial, elements: [moved], files: [{ ...initial.files[0], path: moved.file_path,
      display_path: moved.file_path, parent_folder: "new" }], folders: [{ path: "new", parent: null, children: [moved.file_path] }] };
    rerender(scene(edited));
    expect(screen.getByRole("button", { name: /Moved element/ })).toBeTruthy();
    expect(screen.getByRole("link", { name: /open content for b\.md/i }).getAttribute("href")).toBe("#/content/new/B.md");
    expect(screen.queryByRole("link", { name: /open content for a\.md/i })).toBeNull();
    edited = { ...edited, project: { ...initial.project, worktree_id: "other-branch" }, elements: [{ ...moved, name: "Other branch" }] };
    rerender(scene(edited));
    expect(screen.getByRole("button", { name: /Other branch/ })).toBeTruthy();
    expect(screen.queryByRole("button", { name: /Moved element/ })).toBeNull();
  });

  it("does not prepare the project tree for an inactive view", () => {
    let parentReads = 0;
    const store = { ...devFixture, files: devFixture.files.map(file => ({ ...file,
      get parent_folder() { parentReads++; return file.parent_folder; },
    })) };
    const { rerender } = render(fileScene(store, "coverage"));
    expect(parentReads).toBe(0);
    rerender(fileScene(store, "model"));
    expect(parentReads).toBeGreaterThan(0);
    expect(screen.getByRole("tree", { name: "Project tree" })).toBeTruthy();
  });

  it.each(["files", "model"] as const)("reuses the %s hierarchy across report-only refreshes", view => {
    let hierarchyReads = 0;
    const store = { ...devFixture,
      files: devFixture.files.map(file => ({ ...file,
        get parent_folder() { if (view === "model") hierarchyReads++; return file.parent_folder; },
      })),
      folders: devFixture.folders.map(folder => ({ ...folder,
        get parent() { if (view === "files") hierarchyReads++; return folder.parent; },
      })),
    };
    const { rerender } = render(fileScene(store, view));
    expect(hierarchyReads).toBeGreaterThan(0);
    hierarchyReads = 0;
    rerender(fileScene({ ...store, coverage: { ...store.coverage } }, view));
    expect(hierarchyReads).toBe(0);
  });

  it("renders a native navigable file manager with layout controls and no embedded widget", () => {
    const { container } = renderFiles();

    expect(screen.getByRole("button", { name: "List" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Grid" })).toBeTruthy();
    expect(screen.queryByLabelText("File manager legend")).toBeNull();
    expect(container.querySelector("iframe")).toBeNull();
    expect(container.querySelector('[data-product-pattern="side-pane-frame"]')).toBeNull();
    expect(container.querySelector('[data-product-pattern="workspace-toolbar"]')).toBeNull();
  });

  it("preserves file selection and modeled element detail routing", () => {
    const { onOpenElement } = renderFiles("system-model/Specifications.md");

    expect(screen.queryByRole("heading", { name: "Modeled elements" })).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: /Example Requirement/ }));

    expect(onOpenElement).toHaveBeenCalledWith(
      "system-model/Specifications.md#example-requirement",
    );
  });

  it("opens source content from the file-row open icon", () => {
    renderFiles("system-model/Specifications.md");

    expect(screen.getByRole("link", { name: /open content for specifications\.md/i }).getAttribute("href")).toBe(
      "#/content/system-model/Specifications.md",
    );
  });

  it("does not render an empty folder path as a root child", () => {
    renderFilesWithStore({
      ...devFixture,
      folders: [{ path: "", parent: null, children: [] }, ...devFixture.folders],
    });

    expect(screen.getByText("1 items")).toBeTruthy();
    expect(screen.getByRole("button", { name: /system-model/ })).toBeTruthy();
  });

  it("renders empty source files as inline source previews", () => {
    renderFilesWithStore(
      {
        ...devFixture,
        folders: [
          {
            path: "evidence",
            parent: null,
            children: ["evidence/test-output.txt"],
          },
        ],
        files: [
          {
            path: "evidence/test-output.txt",
            display_path: "evidence/test-output.txt",
            markdown_content: "raw verification evidence\nline two\n",
            parent_folder: "evidence",
            element_ids: [],
            resource_ids: [],
          },
        ],
      },
      "evidence/test-output.txt",
    );

    expect(screen.getByRole("button", { name: /source file evidence\/test-output.txt/i })).toBeTruthy();
    expect(screen.getByText(/raw verification evidence/)).toBeTruthy();
    expect(screen.queryByText(/No modeled elements/)).toBeNull();
  });
});
