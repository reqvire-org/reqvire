import { Theme } from "@radix-ui/themes";
import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { FilesView } from "./FilesView";

function renderFiles(path: string | null = null) {
  const onOpenElement = vi.fn();
  const rendered = render(
    <Theme>
      <StoreProvider store={devFixture} schemaMismatch={null}>
        <FilesView path={path} onOpenElement={onOpenElement} />
      </StoreProvider>
    </Theme>,
  );
  return { ...rendered, onOpenElement };
}

describe("FilesView", () => {
  it("renders a native navigable file manager with search and no embedded widget", () => {
    const { container } = renderFiles();

    expect(screen.getByLabelText("Search files")).toBeTruthy();
    expect(screen.getAllByText("source file").length).toBeGreaterThan(0);
    expect(container.querySelector("iframe")).toBeNull();
    expect(container.querySelector(".explorer-left-panel")).toBeNull();
    expect(container.querySelector(".explorer-workspace-toolbar")).toBeNull();

    fireEvent.change(screen.getByLabelText("Search files"), {
      target: { value: "Specifications" },
    });

    expect(screen.getAllByText("requirements/Specifications.md").length).toBeGreaterThan(0);
  });

  it("preserves file selection and modeled element detail routing", () => {
    const { onOpenElement } = renderFiles("requirements/Specifications.md");

    expect(screen.getByRole("heading", { name: "Modeled elements" })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: /Example Requirement/ }));

    expect(onOpenElement).toHaveBeenCalledWith(
      "requirements/Specifications.md#example-requirement",
    );
  });
});
