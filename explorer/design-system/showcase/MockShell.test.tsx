import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { MockShell } from "./MockShell";
import { ProductPatternsPage } from "./pages/ProductPatternsPage";

// Coverage uses the real application. JSDOM only substitutes unavailable WebGL and worker APIs.
vi.mock("../../src/views/GraphLibraryViews", () => ({ KnowledgeGraphView: () => null }));
vi.mock("../../src/lib/ontologyGraphRenderer", () => ({ mountOntologyGraph: vi.fn() }));

function selectWorktree(branch: string) {
  fireEvent.click(screen.getAllByRole("combobox", { name: "Branch" })[0]);
  fireEvent.click(screen.getByRole("option", { name: new RegExp(`^${branch} `) }));
}

function expectBranch(branch: string) {
  for (const control of screen.getAllByRole("combobox", { name: "Branch" })) expect(control.textContent).toBe(branch);
}

function selectCapability(name: string) {
  fireEvent.click(screen.getByRole("treeitem", { name: "Whole Model" }));
  fireEvent.click(screen.getByRole("treeitem", { name }));
}
function expand(name: string) {
  if (!screen.queryByRole("button", { name: `Expand ${name}` })) selectCapability(name);
  fireEvent.click(screen.getByRole("button", { name: `Expand ${name}` }));
}

describe("showcase application coverage", () => {
  beforeEach(() => {
    localStorage.clear();
    localStorage.setItem("reqvire-explorer-theme", "light");
    Element.prototype.scrollIntoView = vi.fn();
    vi.stubGlobal("Worker", class { postMessage() {} terminate() {} });
  });
  afterEach(() => vi.unstubAllGlobals());

  it("uses the pane-aligned shared header while resizing and collapsing the real Explorer mock", () => {
    history.replaceState(null, "", "/?tab=mocks&example=coverage#/coverage?scope=");
    localStorage.setItem("reqvire:explorer:left-pane-width", "380");
    const view = render(<MockShell example="coverage" />);
    try {
      const shell = view.container.querySelector<HTMLElement>('[data-product-pattern="app-shell"]')!;
      const leading = shell.querySelector('[data-product-pattern-slot="header-leading"]')!;
      const picker = screen.getByRole("combobox", { name: "Branch" });
      const navigation = screen.getByRole("navigation", { name: "Explorer views" });
      expect(leading.contains(picker)).toBe(true);
      expect(leading.nextElementSibling).toBe(navigation);
      const resizer = screen.getByRole("separator", { name: "Resize explorer pane" });
      const url = location.href;
      fireEvent.keyDown(resizer, { key: "ArrowRight", shiftKey: true });
      expect(resizer.getAttribute("aria-valuenow")).toBe("420");
      expect(shell.getAttribute("style")).toContain("--ux-left-pane-width: 420px");
      expect(localStorage.getItem("reqvire:explorer:left-pane-width")).toBe("420");
      expect(location.href).toBe(url);
      fireEvent.click(screen.getByRole("button", { name: "Collapse explorer" }));
      expect(screen.getByRole("combobox", { name: "Branch" })).toBe(picker);
      expect(leading.contains(picker)).toBe(true);
      expect(screen.getByRole("button", { name: "Search" })).toBeTruthy();
      expect(screen.getByRole("button", { name: "Help" })).toBeTruthy();
      fireEvent.click(screen.getByRole("button", { name: "Expand explorer" }));
      expect(resizer.getAttribute("aria-valuenow")).toBe("420");
      expect(location.href).toBe(url);
    } finally { view.unmount(); history.replaceState(null, "", "/"); }
  });

  it("uses the Model quick-filter pattern for capability navigation without changing scope or report", () => {
    history.replaceState(null, "", "/?tab=mocks&example=coverage#/coverage?scope=");
    const view = render(<MockShell example="coverage" />);
    try {
      selectCapability("Alpha Root");
      const explorer = screen.getByRole("complementary", { name: "Explorer navigation" });
      expect(within(explorer).queryByText("Capabilities", { exact: true })).toBeNull();
      const search = within(explorer).getByRole("searchbox", { name: "Filter capability tree" });
      const url = location.href;
      const heading = screen.getByRole("heading", { name: "Alpha Root" });
      const metric = screen.getByText("1 / 3 terminal requirements covered");
      fireEvent.change(search, { target: { value: "shared" } });
      const names = () => within(screen.getByRole("tree", { name: "Coverage capabilities" })).getAllByRole("treeitem")
        .map(row => row.getAttribute("aria-label"));
      expect(names()).toEqual(["Whole Model", "Alpha Root", "Alpha Left", "Shared Branch"]);
      expect(screen.getByRole("treeitem", { name: "Alpha Root" }).getAttribute("aria-selected")).toBe("true");
      expect(location.href).toBe(url);
      expect(screen.getByRole("heading", { name: "Alpha Root" })).toBe(heading);
      expect(screen.getByText("1 / 3 terminal requirements covered")).toBe(metric);
      fireEvent.change(search, { target: { value: "no match" } });
      expect(names()).toEqual(["Whole Model"]);
      expect(location.href).toBe(url);
      fireEvent.change(search, { target: { value: "" } });
      expect(names()).toContain("Beta Root");
      expect(location.href).toBe(url);
    } finally { view.unmount(); history.replaceState(null, "", "/"); }
  });

  it("previews branch labels and unavailable choices in Patterns", () => {
    const view = render(<ProductPatternsPage />);
    try {
      const selector = screen.getAllByRole("combobox", { name: "Branch" })[0];
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
      expect(screen.getByRole("tree", { name: "Coverage capabilities" })).toBeTruthy();
      selectCapability("Example Capability");
      expect(screen.getByRole("treeitem", { name: "Example Capability" })).toBeTruthy();
      expect(screen.queryByRole("treeitem", { name: "Shared Branch" })).toBeNull();
      expand("Example Capability");
      const requirement = screen.getByRole("article", { name: "Example Requirement" });
      fireEvent.click(within(requirement).getByRole("link", { name: "requirement Example Requirement" }));
      await screen.findByRole("dialog");
      selectWorktree("coverage-review");
      await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
      expectBranch("coverage-review");
      expect(screen.getByRole("heading", { name: "Whole Model" })).toBeTruthy();
      selectCapability("Alpha Root");
      expect(screen.getByRole("treeitem", { name: "Shared Branch" })).toBeTruthy();
      expect(screen.queryByRole("treeitem", { name: "Example Capability" })).toBeNull();
      expect(screen.getByRole("article", { name: "Alpha Root" })).toBeTruthy();
      expect(screen.queryByRole("article", { name: "Example Capability" })).toBeNull();
      expect(window.location.hash).toBe("#/coverage?scope=specifications%2FCapabilities.md%23alpha-root");
      expect(new URLSearchParams(window.location.search).get("worktree_id")).toBe("showcase-coverage");
      expect(screen.getByRole("tab", { name: "Coverage" }).getAttribute("aria-selected")).toBe("true");
      view.unmount();
      view = render(<MockShell />);
      expectBranch("coverage-review");
      expect(screen.getByRole("article", { name: "Alpha Root" })).toBeTruthy();
      selectWorktree("main");
      expectBranch("main");
      expect(screen.getByRole("treeitem", { name: "Example Capability" })).toBeTruthy();
      expect(screen.queryByRole("treeitem", { name: "Shared Branch" })).toBeNull();
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
      expect(screen.getByRole("heading", { name: "Whole Model" })).toBeTruthy();
      fireEvent.click(screen.getByRole("treeitem", { name: "Whole Model" }));
      const scopeOptions = () => within(screen.getByRole("tree", { name: "Coverage capabilities" })).getAllByRole("treeitem")
        .map(row => [row.getAttribute("data-capability-id"), row.getAttribute("aria-label"), row.getAttribute("aria-level")]);
      const expectedScopes = [
        [null, "Whole Model", "1"],
        ["specifications/Capabilities.md#alpha-root", "Alpha Root", "2"],
        ["specifications/Capabilities.md#alpha-left", "Alpha Left", "3"],
        ["specifications/Capabilities.md#shared-branch", "Shared Branch", "4"],
        ["specifications/Capabilities.md#alpha-right", "Alpha Right", "3"],
        ["specifications/Capabilities.md#empty-branch", "Empty Branch", "3"],
        ["specifications/Capabilities.md#beta-root", "Beta Root", "2"],
      ];
      expect(scopeOptions()).toEqual(expectedScopes);
      expect(screen.queryByRole("combobox", { name: "Scope" })).toBeNull();
      fireEvent.click(screen.getByRole("treeitem", { name: "Shared Branch" }));
      expect(screen.getByText("1 / 2 terminal requirements covered")).toBeTruthy();
      expect(scopeOptions()).toEqual(expectedScopes);
      const scopeId = "specifications/Capabilities.md#alpha-root";
      fireEvent.click(screen.getByRole("treeitem", { name: "Alpha Root" }));
      expand("Shared Branch");
      expand("Alpha Parent");
      const parent = screen.getByRole("article", { name: "Alpha Parent" });
      fireEvent.click(within(parent).getByRole("link", { name: "requirement Alpha Parent" }));
      const dialog = await screen.findByRole("dialog");
      expect(within(dialog).getByText("Alpha Parent", { exact: true })).toBeTruthy();
      fireEvent.click(within(dialog).getAllByRole("button", { name: "Close" })[0]);
      expect(window.location.hash).toBe(`#/coverage?scope=${encodeURIComponent(scopeId)}`);
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
      await waitFor(() => expect(screen.getByRole("treeitem", { name: "Alpha Root" }).getAttribute("aria-selected")).toBe("true"));
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
      expect(within(gap).queryByText("Not verified", { exact: true })).toBeNull();
      expect(within(gap).getByRole("group", { name: /^Verification: Not verified;/ })).toBeTruthy();
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
