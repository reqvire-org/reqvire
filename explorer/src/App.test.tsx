import { fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";
import { devFixture } from "./store/devFixture";
import * as liveStore from "./store/useLiveStore";
import { WorktreeSelector } from "@ds";

vi.mock("./views/GraphLibraryViews", () => ({ KnowledgeGraphView: () => null }));
vi.mock("./lib/ontologyGraphRenderer", () => ({ mountOntologyGraph: vi.fn() }));

describe("Explorer worktree navigation", () => {
  beforeEach(() => {
    localStorage.clear();
    Element.prototype.scrollIntoView = vi.fn();
    vi.stubGlobal("Worker", class { postMessage() {} terminate() {} });
  });
  afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });

  it("supports keyboard choice, disabled worktrees, and dismissal without relabelling the displayed model", () => {
    const onChange = vi.fn();
    const onOpen = vi.fn();
    const choices = [
      { id: "a", branch: "main", root: "/repo/a", available: true },
      { id: "b", branch: "archived", root: "/repo/b", available: false },
      { id: "c", branch: "feature/long-branch-name", root: "/repo/c", available: true },
    ];
    const view = render(<WorktreeSelector choices={choices} value="a" branch="main" onChange={onChange} onOpen={onOpen} />);
    const trigger = screen.getByRole("combobox", { name: "Branch" });
    trigger.focus();
    fireEvent.keyDown(trigger, { key: "ArrowDown" });
    expect(onOpen).toHaveBeenCalledTimes(1);
    fireEvent.click(screen.getByRole("option", { name: /archived/ }));
    expect(onChange).not.toHaveBeenCalled();
    fireEvent.keyDown(trigger, { key: "ArrowDown" });
    expect(document.getElementById(trigger.getAttribute("aria-activedescendant")!)?.textContent).toContain("feature/");
    fireEvent.keyDown(trigger, { key: "Enter" });
    expect(onChange).toHaveBeenCalledWith("c");
    expect(trigger.textContent).toBe("main");
    expect(document.activeElement).toBe(trigger);
    fireEvent.keyDown(trigger, { key: "End" });
    fireEvent.keyDown(trigger, { key: "Home" });
    expect(document.getElementById(trigger.getAttribute("aria-activedescendant")!)?.textContent).toContain("/repo/a");
    fireEvent.keyDown(trigger, { key: "f" });
    expect(document.getElementById(trigger.getAttribute("aria-activedescendant")!)?.textContent).toContain("/repo/c");
    fireEvent.keyDown(trigger, { key: "Escape" });
    expect(screen.queryByRole("listbox")).toBeNull();
    fireEvent.keyDown(trigger, { key: " " });
    fireEvent.keyDown(trigger, { key: "Tab" });
    expect(screen.queryByRole("listbox")).toBeNull();
    fireEvent.click(trigger);
    fireEvent.pointerDown(document.body);
    expect(screen.queryByRole("listbox")).toBeNull();
    view.unmount();
  });

  it("keeps the adopted worktree identity through pending and failed switches to an identically named branch", () => {
    const props = {
      choices: [
        { id: "a", branch: "main", root: "/repo/a", available: true },
        { id: "b", branch: "main", root: "/repo/b", available: true },
      ],
      branch: "main", displayedValue: "a", value: "b", onChange: vi.fn(), onOpen: vi.fn(),
    };
    const view = render(<WorktreeSelector {...props} pending />);
    const trigger = screen.getByRole("combobox", { name: "Branch" });
    expect(trigger.title).toContain("/repo/a");
    expect(trigger.getAttribute("aria-busy")).toBe("true");
    fireEvent.click(trigger);
    expect(screen.getByRole("option", { name: /repo\/a/ }).getAttribute("aria-selected")).toBe("true");
    expect(screen.getByRole("option", { name: /repo\/b/ }).getAttribute("aria-selected")).toBe("false");
    view.rerender(<WorktreeSelector {...props} pending={false} />);
    expect(trigger.title).toContain("/repo/a");
    expect(screen.getByRole("option", { name: /repo\/a/ }).getAttribute("aria-selected")).toBe("true");
    view.rerender(<WorktreeSelector {...props} displayedValue="b" />);
    expect(trigger.title).toContain("/repo/b");
    expect(screen.getByRole("option", { name: /repo\/b/ }).getAttribute("aria-selected")).toBe("true");
    view.unmount();
  });

  it("places a compact branch picker inside the left pane and preserves navigation", () => {
    window.history.replaceState(null, "", "/?worktree_id=original#/model");
    localStorage.setItem("reqvire-explorer-theme", "light");
    const selectWorktree = vi.fn();
    vi.spyOn(liveStore, "useLiveStore").mockReturnValue({
      result: { ok: true, schemaMismatch: null, store: { ...devFixture,
        project: { ...devFixture.project, worktree_id: "original", branch: "main" },
      } },
      refreshError: null, automaticRefresh: true, worktreeRouting: true, selectedWorktree: "original", switching: false,
      worktrees: [
        { worktree_id: "original", branch: "main", workspace_root: "/repo", available: true, explorer_available: true },
        { worktree_id: "feature", branch: "feature", workspace_root: "/feature", available: true, explorer_available: true },
        { worktree_id: "repair", branch: "repair", workspace_root: "/repair", available: true, explorer_available: false, owned: false },
        { worktree_id: "stopped", branch: "stopped", workspace_root: "/stopped", available: false, explorer_available: false, owned: false },
      ],
      selectWorktree, refreshWorktrees: vi.fn().mockResolvedValue(undefined),
    });
    const view = render(<App />);
    try {
      const selector = screen.getByRole("combobox", { name: "Branch" });
      const navigation = screen.getByRole("navigation", { name: "Explorer views" });
      expect(selector.closest('[data-product-pattern-slot="start-pane"]')).toBeTruthy();
      expect(selector.textContent).toBe("main");
      expect(screen.queryByText("Viewing main")).toBeNull();
      expect(screen.queryByText("/repo")).toBeNull();
      for (const name of ["Thesaurus", "Model", "Ontologies", "Traces", "Coverage"]) {
        expect(within(navigation).getByRole("tab", { name })).toBeTruthy();
      }
      expect(screen.getByRole("button", { name: "Search" })).toBeTruthy();
      expect(screen.getByRole("button", { name: "Help" })).toBeTruthy();
      fireEvent.click(within(navigation).getByRole("tab", { name: "Coverage" }));
      expect(window.location.hash).toBe("#/coverage");
      expect(selector.textContent).toBe("main");
      fireEvent.click(selector);
      fireEvent.click(screen.getByRole("option", { name: /feature/ }));
      expect(selectWorktree).toHaveBeenCalledWith("feature");
      fireEvent.click(selector);
      // Read-only validation errors can recover on demand after the files are repaired.
      expect(screen.getByRole("option", { name: /stopped/ }).getAttribute("aria-disabled")).toBe("true");
      fireEvent.click(screen.getByRole("option", { name: /repair/ }));
      expect(selectWorktree).toHaveBeenCalledWith("repair");
    } finally {
      view.unmount();
      window.history.replaceState(null, "", "/");
    }
  });

});
