import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { FilesView } from "./FilesView";

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

describe("FilesView", () => {
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
