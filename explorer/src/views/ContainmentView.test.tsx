import { Theme } from "@radix-ui/themes";
import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ExplorerSidePane } from "../components/ExplorerSidePane";
import { ExplorerUiStateProvider } from "../components/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { ModelView } from "./ModelView";

function renderModel(activeView: "model" | "files" = "model") {
  const onOpenElement = vi.fn();
  const onNavigate = vi.fn();
  const rendered = render(
    <Theme>
      <StoreProvider store={devFixture} schemaMismatch={null}>
        <ExplorerUiStateProvider>
          <ExplorerSidePane
            activeView={activeView}
            open
            onToggle={vi.fn()}
            onNavigate={onNavigate}
            onOpenElement={onOpenElement}
          />
          <ModelView onOpenElement={onOpenElement} />
        </ExplorerUiStateProvider>
      </StoreProvider>
    </Theme>,
  );
  return { ...rendered, onOpenElement, onNavigate };
}

function clickMode(label: string) {
  fireEvent.click(screen.getByRole("button", { name: label }));
}

describe("Model containment modes", () => {
  it("renders native sunburst and icicle visualizations without iframes", () => {
    const { container } = renderModel();

    clickMode("Sunburst");

    expect(screen.getByTestId("containment-sunburst")).toBeTruthy();
    expect(
      screen.getByRole("img", { name: "Containment sunburst visualization" }),
    ).toBeTruthy();
    expect(container.querySelector("iframe")).toBeNull();

    clickMode("Icicle");

    expect(screen.getByTestId("containment-icicle")).toBeTruthy();
    expect(
      screen.getByRole("img", { name: "Containment icicle visualization" }),
    ).toBeTruthy();
    expect(container.querySelector("iframe")).toBeNull();
  });

  it("keeps containment diagrams inside the Model mode selector", () => {
    renderModel();

    expect(screen.queryByText("Containment")).toBeNull();
    expect(screen.getByRole("button", { name: "List" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Grid" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Sunburst" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Icicle" })).toBeTruthy();
  });

  it("returns file deep links to the Model route when changing model mode", () => {
    const { onNavigate } = renderModel("files");

    clickMode("Sunburst");

    expect(onNavigate).toHaveBeenCalledWith("model");
    expect(screen.getByTestId("containment-sunburst")).toBeTruthy();
  });
});
