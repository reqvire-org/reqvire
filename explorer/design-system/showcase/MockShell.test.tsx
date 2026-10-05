import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { MockShell } from "./MockShell";
import { ProductPatternsPage } from "./pages/ProductPatternsPage";

// Coverage uses the real application. JSDOM only substitutes unavailable WebGL and worker APIs.
vi.mock("../../src/views/GraphLibraryViews", () => ({ KnowledgeGraphView: () => null }));
vi.mock("../../src/lib/ontologyGraphRenderer", () => ({ mountOntologyGraph: vi.fn() }));

function selectWorktree(branch: string) {
  fireEvent.click(screen.getByRole("combobox", { name: "Branch" }));
  fireEvent.click(screen.getByRole("option", { name: new RegExp(`^${branch} `) }));
}

function expectBranch(branch: string) {
  expect(screen.getByRole("combobox", { name: "Branch" }).textContent).toBe(branch);
}

function expand(name: string) {
  fireEvent.click(screen.getByRole("button", { name: `Expand ${name}` }));
}

describe("showcase application coverage", () => {
  beforeEach(() => {
    localStorage.clear();
    Element.prototype.scrollIntoView = vi.fn();
    vi.stubGlobal("Worker", class { postMessage() {} terminate() {} });
  });
  afterEach(() => vi.unstubAllGlobals());

  it("previews branch labels and unavailable choices in Patterns", () => {
    const view = render(<ProductPatternsPage />);
    try {
      const selector = screen.getByRole("combobox", { name: "Branch" });
      expectBranch("main");
      fireEvent.click(selector);
      expect(screen.getByRole("option", { name: /archived.*Unavailable/ }).getAttribute("aria-disabled")).toBe("true");
      fireEvent.keyDown(selector, { key: "Escape" });
      selectWorktree("coverage-review");
      expectBranch("coverage-review");
    } finally { view.unmount(); }
  });

  it("switches fixture worktrees in the real Explorer shell and closes old details", async () => {
    window.history.replaceState(null, "", "/?tab=mocks#/coverage");
    localStorage.setItem("reqvire-explorer-theme", "light");
    let view = render(<MockShell />);
    try {
      expand("Example Capability");
      const requirement = screen.getByRole("article", { name: "Example Requirement" });
      fireEvent.click(within(requirement).getByRole("link", { name: "requirement Example Requirement" }));
      await screen.findByRole("dialog");
      selectWorktree("coverage-review");
      await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
      expectBranch("coverage-review");
      expect(screen.getByRole("article", { name: "Alpha Root" })).toBeTruthy();
      expect(screen.queryByRole("article", { name: "Example Capability" })).toBeNull();
      expect(window.location.hash).toBe("#/coverage");
      expect(new URLSearchParams(window.location.search).get("worktree_id")).toBe("showcase-coverage");
      expect(screen.getByRole("tab", { name: "Coverage" }).getAttribute("aria-selected")).toBe("true");
      view.unmount();
      view = render(<MockShell />);
      expectBranch("coverage-review");
      expect(screen.getByRole("article", { name: "Alpha Root" })).toBeTruthy();
      selectWorktree("main");
      expectBranch("main");
      expect(screen.getByRole("article", { name: "Example Capability" })).toBeTruthy();
      window.history.back();
      await waitFor(() => expectBranch("coverage-review"));
      expect(screen.getByRole("article", { name: "Alpha Root" })).toBeTruthy();
      window.history.forward();
      await waitFor(() => expectBranch("main"));
    } finally {
      view.unmount();
      window.history.replaceState(null, "", "/");
    }
  });

  it("uses the real Explorer app and detail navigation in the showcase mock", async () => {
    window.history.replaceState(null, "", "/?tab=mocks&example=coverage#/coverage");
    localStorage.setItem("reqvire-explorer-theme", "light");
    const seed = window.reqvireProjectStore;
    const view = render(<MockShell example="coverage" />);
    try {
      expect(screen.getByRole("heading", { name: "Coverage" })).toBeTruthy();
      const scopeId = "specifications/Capabilities.md#alpha-root";
      fireEvent.change(screen.getByRole("combobox", { name: "Scope" }), { target: { value: scopeId } });
      expand("Shared Branch");
      expand("Alpha Parent");
      const parent = screen.getByRole("article", { name: "Alpha Parent" });
      fireEvent.click(within(parent).getByRole("link", { name: "requirement Alpha Parent" }));
      const dialog = await screen.findByRole("dialog");
      expect(within(dialog).getByText("Alpha Parent", { exact: true })).toBeTruthy();
      fireEvent.click(within(dialog).getAllByRole("button", { name: "Close" })[0]);
      expand("Alpha Middle");
      const terminal = screen.getByRole("article", { name: "Alpha Implemented" });
      expect(within(terminal).queryByRole("button")).toBeNull();
      expect(screen.queryByRole("list", { name: /^Implementation evidence for/ })).toBeNull();
      fireEvent.click(within(terminal).getByRole("link", { name: "requirement Alpha Implemented" }));
      const detail = await screen.findByRole("dialog");
      expect(within(detail).getAllByText("satisfiedBy").length).toBeGreaterThan(0);
      const artifact = within(detail).getByRole("link", { name: /alpha.txt/ });
      expect(artifact.getAttribute("href")).toBe("#/content/evidence/alpha.txt");
      fireEvent.click(artifact);
      expect(await screen.findByText(/Synthetic implementation artifact for the alpha fixture/)).toBeTruthy();
      expect(window.location.hash).toBe("#/content/evidence/alpha.txt");
      expect(screen.queryByRole("dialog")).toBeNull();
      fireEvent.click(screen.getByRole("tab", { name: "Coverage" }));
      await waitFor(() => expect((screen.getByRole("combobox", { name: "Scope" }) as HTMLSelectElement).value).toBe(scopeId));
      expect(screen.getByText("1 / 3 terminal requirements covered")).toBeTruthy();
    } finally {
      view.unmount();
      window.reqvireProjectStore = seed;
      window.history.replaceState(null, "", "/");
    }
  });

  it("nests both default fixture requirements under their actual capability", () => {
    window.history.replaceState(null, "", "/?tab=mocks#/coverage");
    localStorage.setItem("reqvire-explorer-theme", "light");
    const seed = window.reqvireProjectStore;
    const view = render(<MockShell />);
    try {
      expect(screen.queryByRole("article", { name: "Example Requirement" })).toBeNull();
      expect(screen.queryByRole("article", { name: "Unverified Fixture Requirement" })).toBeNull();
      expand("Example Capability");
      const capability = screen.getByRole("article", { name: "Example Capability" });
      expect(within(capability).getByRole("article", { name: "Example Requirement" })).toBeTruthy();
      const gap = within(capability).getByRole("article", { name: "Unverified Fixture Requirement" });
      expect(within(gap).queryByRole("button")).toBeNull();
      expect(within(gap).getByText("Not verified", { exact: true })).toBeTruthy();
      expect(within(gap).getByRole("link", { name: "requirement Unverified Fixture Requirement" })).toBeTruthy();
      const terminal = within(capability).getByRole("article", { name: "Example Requirement" });
      expect(within(terminal).queryByRole("button")).toBeNull();
      expect(within(terminal).getByRole("link", { name: "requirement Example Requirement" })).toBeTruthy();
    } finally {
      view.unmount();
      window.reqvireProjectStore = seed;
      window.history.replaceState(null, "", "/");
    }
  });
});
