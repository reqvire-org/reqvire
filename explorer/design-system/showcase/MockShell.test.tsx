import { fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { MockShell } from "./MockShell";

// Coverage uses the real application. JSDOM only substitutes unavailable WebGL and worker APIs.
vi.mock("../../src/views/GraphLibraryViews", () => ({ KnowledgeGraphView: () => null }));
vi.mock("../../src/lib/ontologyGraphRenderer", () => ({ mountOntologyGraph: vi.fn() }));

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

  it("uses the real Explorer app and detail navigation in the showcase mock", async () => {
    window.history.replaceState(null, "", "/?tab=mocks&example=coverage#/coverage");
    localStorage.setItem("reqvire-explorer-theme", "light");
    const seed = window.reqvireProjectStore;
    const view = render(<MockShell example="coverage" />);
    try {
      expect(screen.getByRole("heading", { name: "Coverage" })).toBeTruthy();
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
      fireEvent.click(within(detail).getByRole("link", { name: /alpha.txt/ }));
      expect(await screen.findByText(/Synthetic implementation artifact for the alpha fixture/)).toBeTruthy();
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
