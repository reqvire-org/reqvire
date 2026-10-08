import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { CoverageNavigation } from "./CoverageNavigation";

const scopes = [
  { identifier: "cap#root", name: "Root", depth: 0 },
  { identifier: "cap#child", name: "Child", depth: 1 },
  { identifier: "cap#nested", name: "Nested", depth: 2 },
  { identifier: "cap#other", name: "Other", depth: 0 },
];

describe("Coverage navigation", () => {
  it("filters names and identifiers with ancestors while preserving selection and disclosure", () => {
    const input = structuredClone(scopes);
    const select = vi.fn();
    const view = render(<CoverageNavigation scopes={input} selectedId="cap#other" onSelect={select} />);
    fireEvent.keyDown(screen.getByRole("treeitem", { name: "Root" }), { key: "ArrowLeft" });
    view.rerender(<CoverageNavigation scopes={input} selectedId="cap#other" onSelect={select} query="  nEsTeD  " />);
    expect(screen.getAllByRole("treeitem").map(row => row.getAttribute("aria-label")))
      .toEqual(["Whole Model", "Root", "Child", "Nested"]);
    expect(screen.getByRole("treeitem", { name: "Root" }).getAttribute("aria-expanded")).toBe("true");
    fireEvent.keyDown(screen.getByRole("treeitem", { name: "Child" }), { key: "ArrowLeft" });
    expect(screen.queryByRole("treeitem", { name: "Nested" })).toBeNull();
    view.rerender(<CoverageNavigation scopes={input} selectedId="cap#other" onSelect={select} query="CAP#NESTED" />);
    expect(screen.getByRole("treeitem", { name: "Nested" })).toBeTruthy();
    view.rerender(<CoverageNavigation scopes={input} selectedId="cap#other" onSelect={select} query="" />);
    expect(screen.queryByRole("treeitem", { name: "Child" })).toBeNull();
    expect(screen.getByRole("treeitem", { name: "Other" }).getAttribute("aria-selected")).toBe("true");
    expect(select).not.toHaveBeenCalled();
    expect(input).toEqual(scopes);
  });

  it("keeps Whole Model available for unmatched and empty filters without empty disclosures", () => {
    const select = vi.fn();
    const view = render(<CoverageNavigation scopes={scopes} selectedId="cap#nested" onSelect={select} query="missing" />);
    const whole = screen.getByRole("treeitem", { name: "Whole Model" });
    expect(screen.getAllByRole("treeitem")).toEqual([whole]);
    expect(whole.hasAttribute("aria-expanded")).toBe(false);
    fireEvent.click(whole);
    expect(select).toHaveBeenLastCalledWith(null);
    view.rerender(<CoverageNavigation scopes={scopes} selectedId={null} onSelect={select} query="Other" />);
    expect(screen.getByRole("treeitem", { name: "Other" }).hasAttribute("aria-expanded")).toBe(false);
    view.rerender(<CoverageNavigation scopes={[]} selectedId={null} onSelect={select} query="Nested" />);
    expect(screen.getAllByRole("treeitem")).toEqual([whole]);
  });

  it("selects canonical capabilities and identifies the selected item", () => {
    const select = vi.fn();
    const view = render(<CoverageNavigation scopes={scopes} selectedId={null} onSelect={select} />);
    fireEvent.click(screen.getByRole("treeitem", { name: "Nested" }));
    expect(select).toHaveBeenLastCalledWith("cap#nested");
    view.rerender(<CoverageNavigation scopes={scopes} selectedId="cap#nested" onSelect={select} />);
    expect(screen.getByRole("treeitem", { name: "Nested" }).getAttribute("aria-selected")).toBe("true");
    fireEvent.click(screen.getByRole("treeitem", { name: "Whole Model" }));
    expect(select).toHaveBeenLastCalledWith(null);
    view.rerender(<CoverageNavigation scopes={scopes} selectedId={null} onSelect={select} />);
    expect(screen.getByRole("treeitem", { name: "Whole Model" }).getAttribute("aria-selected")).toBe("true");
  });

  it("supports keyboard traversal and disclosures without changing scope", () => {
    const select = vi.fn();
    render(<CoverageNavigation scopes={scopes} selectedId={null} onSelect={select} />);
    const root = screen.getByRole("treeitem", { name: "Root" });
    root.focus();
    fireEvent.keyDown(root, { key: "ArrowRight" });
    const child = screen.getByRole("treeitem", { name: "Child" });
    expect(document.activeElement).toBe(child);
    fireEvent.keyDown(child, { key: "ArrowDown" });
    const nested = screen.getByRole("treeitem", { name: "Nested" });
    expect(document.activeElement).toBe(nested);
    fireEvent.keyDown(nested, { key: "ArrowLeft" });
    expect(document.activeElement).toBe(child);
    fireEvent.keyDown(child, { key: "ArrowLeft" });
    expect(screen.queryByRole("treeitem", { name: "Nested" })).toBeNull();
    expect(child.getAttribute("aria-expanded")).toBe("false");
    expect(select).not.toHaveBeenCalled();
    fireEvent.keyDown(child, { key: "ArrowRight" });
    expect(screen.getByRole("treeitem", { name: "Nested" }).getAttribute("aria-level")).toBe("4");
    fireEvent.keyDown(child, { key: "End" });
    const other = screen.getByRole("treeitem", { name: "Other" });
    expect(document.activeElement).toBe(other);
    fireEvent.keyDown(other, { key: " " });
    expect(select).toHaveBeenLastCalledWith("cap#other");
    fireEvent.keyDown(other, { key: "Home" });
    const whole = screen.getByRole("treeitem", { name: "Whole Model" });
    expect(document.activeElement).toBe(whole);
    fireEvent.keyDown(whole, { key: "Enter" });
    expect(select).toHaveBeenLastCalledWith(null);
    fireEvent.keyDown(whole, { key: "ArrowRight" });
    expect(document.activeElement).toBe(root);
    fireEvent.keyDown(root, { key: "Enter" });
    expect(select).toHaveBeenLastCalledWith("cap#root");
  });

  it("reveals ancestors after selection changes while keeping the input hierarchy immutable", () => {
    const input = structuredClone(scopes);
    const select = vi.fn();
    const view = render(<CoverageNavigation scopes={input} selectedId={null} onSelect={select} />);
    fireEvent.keyDown(screen.getByRole("treeitem", { name: "Root" }), { key: "ArrowLeft" });
    expect(within(screen.getByRole("tree")).getAllByRole("treeitem")).toHaveLength(3);
    view.rerender(<CoverageNavigation scopes={input} selectedId="cap#nested" onSelect={select} />);
    expect(screen.getByRole("treeitem", { name: "Root" }).getAttribute("aria-expanded")).toBe("true");
    expect(screen.getByRole("treeitem", { name: "Nested" }).getAttribute("aria-selected")).toBe("true");
    expect(input).toEqual(scopes);
    expect(select).not.toHaveBeenCalled();
  });

  it("nests all capabilities under a collapsible Whole Model scope without publishing a capability identity", () => {
    const select = vi.fn();
    const view = render(<CoverageNavigation scopes={scopes} selectedId={null} onSelect={select} />);
    const whole = screen.getByRole("treeitem", { name: "Whole Model" });
    expect(whole.getAttribute("aria-level")).toBe("1");
    expect(whole.hasAttribute("data-capability-id")).toBe(false);
    expect(screen.getByRole("treeitem", { name: "Root" }).getAttribute("aria-level")).toBe("2");
    fireEvent.keyDown(whole, { key: "ArrowLeft" });
    expect(within(screen.getByRole("tree")).getAllByRole("treeitem")).toEqual([whole]);
    expect(select).not.toHaveBeenCalled();
    view.rerender(<CoverageNavigation scopes={scopes} selectedId="cap#nested" onSelect={select} />);
    expect(whole.getAttribute("aria-expanded")).toBe("true");
    expect(screen.getByRole("treeitem", { name: "Nested" }).getAttribute("aria-selected")).toBe("true");
    fireEvent.keyDown(screen.getByRole("treeitem", { name: "Root" }), { key: "ArrowLeft" });
    fireEvent.keyDown(screen.getByRole("treeitem", { name: "Root" }), { key: "ArrowLeft" });
    expect(document.activeElement).toBe(whole);
    fireEvent.keyDown(whole, { key: " " });
    expect(select).toHaveBeenLastCalledWith(null);
    view.rerender(<CoverageNavigation scopes={[]} selectedId={null} onSelect={select} />);
    expect(whole.hasAttribute("aria-expanded")).toBe(false);
    expect(whole.getAttribute("aria-selected")).toBe("true");
  });
});
