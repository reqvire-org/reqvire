import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { ContentView } from "./ContentView";

function renderContent(path: string) {
  return render(
    <StoreProvider store={devFixture} schemaMismatch={null}>
      <ContentView path={path} />
    </StoreProvider>,
  );
}

describe("ContentView", () => {
  it("renders source pages with a back to model action", () => {
    renderContent("requirements/Specifications.md");

    expect(screen.getByText("Source page")).toBeTruthy();
    expect(screen.getByText("requirements/Specifications.md")).toBeTruthy();
    expect(screen.getByRole("link", { name: "Back to model" }).getAttribute("href")).toBe("#/model");
  });

  it("keeps the model back action when a source page is missing", () => {
    renderContent("missing.md");

    expect(screen.getByText("File not found: missing.md")).toBeTruthy();
    expect(screen.getByRole("link", { name: "Back to model" }).getAttribute("href")).toBe("#/model");
  });

  it("renders local resource source files in the middle pane", () => {
    renderContent("core/src/lib.rs");

    expect(screen.getByText("Source file")).toBeTruthy();
    expect(screen.getByText("core/src/lib.rs")).toBeTruthy();
    expect(screen.getByText(/fixture_source/)).toBeTruthy();
  });
});
