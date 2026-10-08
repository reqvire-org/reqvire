import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { coverageRows } from "../lib/coverage";
import { routeForCoverage } from "../router/routes";
import { writeExplorerUrl } from "../router/location";
import whole from "../../../tests/test-scoped-coverage/expected/whole-model.json";
import scopes from "../../../tests/test-scoped-coverage/expected/scopes.json";
import showcase from "../store/fixtures/scopedCoverage.json";
import { loadStoreCandidate } from "../store/loadStore";
import { StoreProvider } from "../store/StoreContext";
import { ExplorerUiStateProvider } from "../state/ExplorerUiState";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore } from "../store/types";
import { ExplorerSidePane } from "../components/ExplorerSidePane";
import { CoverageView } from "./ReportViews";

const cap = (name: string) => `specifications/Capabilities.md#${name.toLowerCase().replaceAll(" ", "-")}`;
function fixture(): ExplorerProjectStore {
  const coverage = structuredClone(whole);
  const index = Object.fromEntries(Object.values(scopes).map(scope => [scope.capability_identifier, {
    scope,
    summary: { ...coverage.summary, total_requirements_in_scope: scope.requirement_ids.length,
      total_verifications: scope.verification_ids.length, orphaned_verifications: 0, orphaned_verifications_percentage: 0 },
  }]));
  Object.assign(index[cap("Alpha Root")].summary, { total_leaf_requirements: 4, verified_leaf_requirements: 3,
    leaf_requirements_coverage_percentage: 75,
    total_terminal_requirements: 3, covered_terminal_requirements: 1, uncovered_terminal_requirements: 2,
    implementation_coverage_percentage: 33.33, covered_requirements: 3, uncovered_requirements: 3,
    total_test_verifications: 3, satisfied_test_verifications: 2, unsatisfied_test_verifications: 1 });
  for (const key of Object.keys(index[cap("Empty Branch")].summary)) {
    if (typeof index[cap("Empty Branch")].summary[key as keyof typeof coverage.summary] === "number") {
      Object.assign(index[cap("Empty Branch")].summary, { [key]: 0 });
    }
  }
  const capabilities = coverage.capability_coverage.capabilities.map(row => ({ ...devFixture.elements[0],
    id: row.identifier, name: row.name, element_type: "capability", type_family: "capability" }));
  const requirements = [...Object.values(whole.covered_requirements.files).flat(), ...Object.values(whole.uncovered_requirements.files).flat()]
    .map(row => ({ ...devFixture.elements[0], id: row.identifier, name: row.name, element_type: "requirement", type_family: "requirement" }));
  const relations = structuredClone(showcase.relations);
  return { ...devFixture, elements: [...capabilities, ...requirements], relations,
    coverage: { ...coverage, scope_index: index } };
}

function generatedFixture(): ExplorerProjectStore {
  const loaded = loadStoreCandidate(structuredClone(showcase));
  if (!loaded.ok) throw new Error(loaded.reason);
  return loaded.store;
}

function shell(store: ExplorerProjectStore, onOpenElement = vi.fn(), worktreeRouting = false) {
  return <StoreProvider store={store} schemaMismatch={null}><ExplorerUiStateProvider worktreeRouting={worktreeRouting}>
    <ExplorerSidePane activeView="coverage" open onToggle={vi.fn()} onNavigate={vi.fn()}
      onOpenElement={onOpenElement} onOpenOntologyNode={vi.fn()} />
    <CoverageView onOpenElement={onOpenElement} />
  </ExplorerUiStateProvider></StoreProvider>;
}
function selectScope(name: string) {
  fireEvent.click(name ? screen.getByRole("treeitem", { name }) : screen.getByRole("treeitem", { name: "Whole Model" }));
}
function selectedScope() {
  return screen.queryByRole("tree", { name: "Coverage capabilities" })?.querySelector('[aria-selected="true"]')?.getAttribute("data-capability-id") ?? "";
}
function scopeRows() {
  return within(screen.getByRole("tree", { name: "Coverage capabilities" })).getAllByRole("treeitem").map(row => [
    row.getAttribute("data-capability-id"), row.getAttribute("aria-label"), row.getAttribute("aria-level"),
  ]);
}
function capabilityNames(container: HTMLElement) {
  return [...container.querySelectorAll('[data-kind="capability"][data-coverage-depth] a span:last-child')].map(node => node.textContent);
}

function expand(name: string) {
  fireEvent.click(screen.getByRole("button", { name: `Expand ${name}` }));
}
function openParent() {
  expand("Shared Branch");
  expand("Alpha Parent");
}

describe("scoped coverage views", () => {
  beforeEach(() => {
    localStorage.clear();
    window.history.replaceState(null, "", "/");
    Element.prototype.scrollIntoView = vi.fn();
  });
  afterEach(() => window.history.replaceState(null, "", "/"));

  it("shows the complete whole-model dashboard and scoped issues below each selected capability", () => {
    history.replaceState(null, "", "/#/coverage");
    const store = generatedFixture(); const snapshot = structuredClone(store);
    const { container } = render(shell(store));
    expect(screen.getByRole("heading", { name: "Whole Model" })).toBeTruthy();
    expect(screen.getByRole("treeitem", { name: "Whole Model" }).getAttribute("aria-selected")).toBe("true");
    expect(screen.queryByLabelText("Coverage mode")).toBeNull();
    expect(screen.queryByRole("button", { name: "View issues" })).toBeNull();
    expect(container.querySelectorAll(".coverage-kpi")).toHaveLength(4);
    expect(container.querySelectorAll(".coverage-gap-list")).toHaveLength(4);
    expect(capabilityNames(container)).toHaveLength(6);
    selectScope("Alpha Root");
    expect(screen.getByRole("heading", { name: "Alpha Root", level: 1 })).toBeTruthy();
    expect(screen.queryByRole("button", { name: "Report details" })).toBeNull();
    expect(screen.getByRole("heading", { name: "Verification types" })).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Implementation sources" })).toBeTruthy();
    const breakdown = screen.getByRole("region", { name: "Coverage breakdown" });
    expect(container.querySelector(".coverage-kpi-grid")!.compareDocumentPosition(breakdown) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    const legend = breakdown.querySelectorAll(".coverage-legend-row");
    expect(legend).toHaveLength(5);
    expect(breakdown.querySelectorAll('[data-meter="segmented"]')).toHaveLength(1);
    expect([...legend].map(row => [row.querySelector("span:nth-child(2)")?.textContent, row.querySelector("strong")?.textContent]))
      .toEqual([["Test", "2"], ["Formal proof", "1"], ["Analysis", "0"], ["Inspection", "1"], ["Demonstration", "1"]]);
    expect(container.querySelectorAll(".coverage-kpi")).toHaveLength(3);
    expect(container.querySelectorAll(".coverage-gap-list")).toHaveLength(3);
    const issues = container.querySelector("#coverage-section-unimplemented-requirements")!;
    expect([...issues.querySelectorAll(".coverage-gap-row__title")].map(node => node.textContent).sort())
      .toEqual(["Alpha Gap", "Alpha Local", "Alpha Parent"]);
    const drilldown = container.querySelector("#coverage-section-capability-coverage")!;
    expect(within(drilldown as HTMLElement).getAllByText("Verification", { exact: true })).toHaveLength(1);
    expect(within(drilldown as HTMLElement).getAllByText("Implementation", { exact: true })).toHaveLength(1);
    expect(within(drilldown as HTMLElement).getByRole("heading", { name: "Capability coverage", level: 2 })).toBeTruthy();
    expect(drilldown.compareDocumentPosition(issues) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(breakdown.compareDocumentPosition(drilldown) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
    expect(container.querySelector("#coverage-section-orphaned-verifications")).toBeNull();
    selectScope("");
    expect(selectedScope()).toBe("");
    expect(window.location.hash).toBe(routeForCoverage(null));
    expect(container.querySelectorAll(".coverage-gap-list")).toHaveLength(4);
    expect(store).toEqual(snapshot);
  });

  it("renders a full dashboard for an empty capability and resets it through the Whole Model root", () => {
    const { container } = render(shell(generatedFixture()));
    selectScope("Empty Branch");
    expect(screen.getByRole("heading", { name: "Empty Branch", level: 1 })).toBeTruthy();
    expect(screen.getByText("No requirements in this capability scope.")).toBeTruthy();
    expect(screen.queryByRole("heading", { name: "Select a capability" })).toBeNull();
    expect(container.querySelectorAll(".coverage-kpi")).toHaveLength(3);
    expect(container.querySelectorAll(".coverage-gap-list")).toHaveLength(3);
    expect(container.querySelectorAll(".coverage-gap-row")).toHaveLength(0);
    selectScope("");
    expect(screen.getByRole("heading", { name: "Whole Model" })).toBeTruthy();
    expect(screen.getByRole("treeitem", { name: "Whole Model" }).getAttribute("aria-selected")).toBe("true");
  });

  it("prefers explicit scope links over saved preferences and resumes legacy bare links", () => {
    const store = generatedFixture();
    const seed = render(shell(store));
    selectScope("Alpha Root");
    seed.unmount();
    window.history.replaceState(null, "", `/?theme=dark${routeForCoverage(cap("Shared Branch"))}`);
    const linked = render(shell(store));
    expect(selectedScope()).toBe(cap("Shared Branch"));
    expect(screen.getByText("1 / 2 terminal requirements covered")).toBeTruthy();
    expect(window.location.search).toBe("?theme=dark");
    linked.unmount();
    window.history.replaceState(null, "", "/#/coverage");
    const legacy = render(shell(store));
    expect(selectedScope()).toBe(cap("Shared Branch"));
    expect(window.location.hash).toBe(routeForCoverage(cap("Shared Branch")));
    legacy.unmount();
    window.history.replaceState(null, "", `/${routeForCoverage(null)}`);
    render(shell(store));
    expect(selectedScope()).toBe("");
  });

  it("restores scope and shared counts through history and fresh loading without preferences", async () => {
    const store = generatedFixture();
    window.history.replaceState(null, "", `/${routeForCoverage(null)}`);
    const view = render(shell(store));
    selectScope("Alpha Root");
    expect(window.location.hash).toBe(routeForCoverage(cap("Alpha Root")));
    selectScope("Shared Branch");
    expect(window.location.hash).toBe(routeForCoverage(cap("Shared Branch")));
    act(() => window.history.back());
    await waitFor(() => expect(window.location.hash).toBe(routeForCoverage(cap("Alpha Root"))));
    expect(screen.getByText("1 / 3 terminal requirements covered")).toBeTruthy();
    act(() => window.history.forward());
    await waitFor(() => expect(window.location.hash).toBe(routeForCoverage(cap("Shared Branch"))));
    expect(screen.getByText("1 / 2 terminal requirements covered")).toBeTruthy();
    view.unmount();
    localStorage.clear();
    render(shell(store));
    expect(selectedScope()).toBe(cap("Shared Branch"));
  });

  it.each(["specifications/Capabilities.md#missing", "specifications/Alpha.md#alpha-parent"])(
    "explains unavailable scope %s and replaces its URL with Whole model", id => {
      const store = generatedFixture();
      const snapshot = structuredClone(store);
      window.history.replaceState(null, "", `/${routeForCoverage(id)}`);
      render(shell(store));
      expect(selectedScope()).toBe("");
      expect(screen.getByText(/selected capability is no longer available/)).toBeTruthy();
      expect(window.location.hash).toBe(routeForCoverage(null));
      expect(store).toEqual(snapshot);
    });

  it("waits for the requested worktree snapshot before interpreting or rewriting its scope", () => {
    const a = generatedFixture();
    a.project = { ...a.project, worktree_id: "a" };
    const b = structuredClone(a);
    b.project = { ...b.project, worktree_id: "b" };
    window.history.replaceState(null, "", `/?worktree_id=a${routeForCoverage(cap("Alpha Root"))}`);
    const view = render(shell(a, vi.fn(), true));
    const target = new URL(`/?worktree_id=b${routeForCoverage(cap("Shared Branch"))}`, window.location.origin);
    act(() => writeExplorerUrl(target));
    expect(selectedScope()).toBe(cap("Alpha Root"));
    expect(window.location.href).toBe(target.href);
    view.rerender(shell(b, vi.fn(), true));
    expect(selectedScope()).toBe(cap("Shared Branch"));
    expect(window.location.href).toBe(target.href);
  });

  it("nests requirements while terminal names open element details", () => {
    const loaded = loadStoreCandidate(showcase);
    if (!loaded.ok) throw new Error(loaded.reason);
    render(shell(loaded.store));
    const header = screen.getByRole("heading", { name: "Whole Model" }).closest(".coverage-header")!;
    expect(within(header as HTMLElement).queryByRole("combobox")).toBeNull();
    expect(screen.getByRole("treeitem", { name: "Whole Model" }).getAttribute("aria-selected")).toBe("true");
    expect(screen.queryByRole("heading", { name: "Implementation evidence" })).toBeNull();
    expect(screen.queryByRole("article", { name: "Alpha Parent" })).toBeNull();
    expand("Shared Branch");
    const parent = screen.getByRole("article", { name: "Alpha Parent" });
    expect(within(parent).queryByRole("region")).toBeNull();
    fireEvent.click(within(parent).getByRole("button", { name: "Expand Alpha Parent" }));
    expect(within(parent).queryByRole("region")).toBeNull();
    expect(within(parent).getByRole("article", { name: "Alpha Gap" })).toBeTruthy();
    const middle = within(parent).getByRole("article", { name: "Alpha Middle" });
    expect(within(middle).getByRole("button", { name: "Expand Alpha Middle" }).getAttribute("aria-expanded")).toBe("false");
    expect(within(parent).queryByRole("article", { name: "Alpha Implemented" })).toBeNull();
    fireEvent.click(within(middle).getByRole("button", { name: "Expand Alpha Middle" }));
    const terminal = within(middle).getByRole("article", { name: "Alpha Implemented" });
    expect(within(terminal).queryByRole("button")).toBeNull();
    expect(within(terminal).queryByText("Verified", { exact: true })).toBeNull();
    expect(within(terminal).queryByText("Covered", { exact: true })).toBeNull();
    expect(within(terminal).getByText("100% · 1 / 1 leaves")).toBeTruthy();
    expect(within(terminal).getByText("100% · 1 / 1 terminal")).toBeTruthy();
    expect([...terminal.querySelectorAll("[data-color-token]")].map(bar => [
      bar.getAttribute("data-color-token"), bar.querySelector("rect")?.getAttribute("width"),
    ])).toEqual([["--verification", "100"], ["--resource", "100"]]);
    expect(within(terminal).queryByRole("button", { name: /Evidence for/ })).toBeNull();
    expect(within(terminal).queryByRole("list")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Collapse Shared Branch" }));
    expect(screen.queryByRole("article", { name: "Alpha Parent" })).toBeNull();
  });

  it("lists all scopes alphabetically in the capability navigation tree", () => {
    const store = generatedFixture();
    const snapshot = structuredClone(store);
    render(shell(store));
    fireEvent.click(screen.getByRole("treeitem", { name: "Whole Model" }));
    const expected = [
      [null, "Whole Model", "1"],
      [cap("Alpha Root"), "Alpha Root", "2"],
      [cap("Alpha Left"), "Alpha Left", "3"],
      [cap("Shared Branch"), "Shared Branch", "4"],
      [cap("Alpha Right"), "Alpha Right", "3"],
      [cap("Empty Branch"), "Empty Branch", "3"],
      [cap("Beta Root"), "Beta Root", "2"],
    ];
    expect(scopeRows()).toEqual(expected);
    selectScope("Shared Branch");
    expect(selectedScope()).toBe(cap("Shared Branch"));
    expect(screen.getByRole("heading", { name: "Shared Branch" })).toBeTruthy();
    expect(scopeRows()).toEqual(expected);
    expect(screen.getByText("1 / 2 terminal requirements covered")).toBeTruthy();
    selectScope("");
    expect(screen.getByRole("heading", { name: "Whole Model" })).toBeTruthy();
    expect(screen.getByRole("treeitem", { name: "Whole Model" }).getAttribute("aria-selected")).toBe("true");
    expect(store).toEqual(snapshot);
  });

  it("keeps scope ordering and identity stable across reordered records and inverse parent edges", () => {
    const store = generatedFixture();
    const view = render(shell(store));
    fireEvent.click(screen.getByRole("treeitem", { name: "Whole Model" }));
    const expected = scopeRows();
    selectScope("Shared Branch");
    const reordered = structuredClone(store);
    reordered.coverage.scope_index = Object.fromEntries(Object.entries(reordered.coverage.scope_index).reverse());
    reordered.coverage.capability_coverage.capabilities.reverse();
    reordered.relations = reordered.relations.flatMap(relation => relation.relation_type === "derive" ? [relation, {
      ...relation, id: relation.id + "-inverse", source_id: relation.target_id, target_id: relation.source_id,
      relation_type: "derivedFrom",
    }] : [relation]).reverse();
    const snapshot = structuredClone(reordered);
    view.rerender(shell(reordered));
    expect(scopeRows()).toEqual(expected);
    expect(selectedScope()).toBe(cap("Shared Branch"));
    expect(scopeRows().filter(([id]) => id === cap("Shared Branch"))).toHaveLength(1);
    expect(screen.getByText("1 / 2 terminal requirements covered")).toBeTruthy();
    expect(reordered).toEqual(snapshot);
  });

  it("discloses capabilities without changing scope and reveals a collapsed selection on history traversal", async () => {
    const store = generatedFixture();
    window.history.replaceState(null, "", `/${routeForCoverage(null)}`);
    render(shell(store));
    selectScope("Shared Branch");
    const root = screen.getByRole("treeitem", { name: "Alpha Root" });
    fireEvent.keyDown(root, { key: "ArrowLeft" });
    expect(screen.queryByRole("treeitem", { name: "Shared Branch" })).toBeNull();
    expect(window.location.hash).toBe(routeForCoverage(cap("Shared Branch")));
    selectScope("");
    act(() => window.history.back());
    await waitFor(() => expect(screen.getByRole("treeitem", { name: "Shared Branch" }).getAttribute("aria-selected")).toBe("true"));
    expect(screen.getByRole("treeitem", { name: "Alpha Root" }).getAttribute("aria-expanded")).toBe("true");
    expect(screen.getByText("1 / 2 terminal requirements covered")).toBeTruthy();
  });

  it("ranks roots and siblings within one hierarchy and preserves both parent paths", () => {
    const store = fixture();
    const { container } = render(shell(store));
    expect(coverageRows(store, store.coverage).map(row => row.name)).toEqual(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right", "Beta Root"]);
    selectScope("Alpha Root");
    expect(screen.queryByText("Partially verified", { exact: true })).toBeNull();
    expect(screen.getAllByRole("group", { name: /^Verification: Partially verified/ }).length).toBeGreaterThan(0);
    expect(screen.queryByRole("button", { name: "Root summaries" })).toBeNull();
    expect(screen.queryByRole("button", { name: "Hierarchy" })).toBeNull();
    expect(screen.queryByRole("button", { name: "Ranked capabilities" })).toBeNull();
    selectScope("Alpha Root");
    expect(screen.getByText("1 / 3 terminal requirements covered")).toBeTruthy();
    expect(capabilityNames(container)).toEqual(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right"]);
    expect([...container.querySelectorAll('[data-kind="capability"][data-coverage-depth]')].map(row => row.getAttribute("data-coverage-depth")))
      .toEqual(["0", "1", "1", "2", "1"]);
    expect(screen.queryByRole("button", { name: /Unimplemented requirements.*3/ })).toBeNull();
    expect(store.relations.filter(row => row.target_id === cap("Shared Branch"))).toHaveLength(2);
  });

  it("alternates capability and requirement rows together across changing disclosures", () => {
    const { container } = render(shell(fixture()));
    const expectTones = (names: string[]) => {
      const rows = [...container.querySelectorAll('[data-kind="requirement"][data-coverage-depth]')];
      expect(rows.map(row => row.closest("article")!.getAttribute("aria-label"))).toEqual(names);
      const allRows = [...container.querySelectorAll('[data-coverage-depth]')];
      expect(allRows.map(row => row.getAttribute("data-coverage-tone")))
        .toEqual(allRows.map((_, index) => index % 2 ? "alternate" : "base"));
    };
    expectTones([]);
    openParent();
    expectTones(["Alpha Parent", "Alpha Gap", "Alpha Middle"]);
    expand("Alpha Middle");
    expectTones(["Alpha Parent", "Alpha Gap", "Alpha Middle", "Alpha Implemented"]);
    expand("Alpha Left");
    expectTones(["Alpha Contract Owner", "Alpha Parent", "Alpha Gap", "Alpha Middle", "Alpha Implemented"]);
    fireEvent.click(screen.getByRole("button", { name: "Collapse Alpha Middle" }));
    expectTones(["Alpha Contract Owner", "Alpha Parent", "Alpha Gap", "Alpha Middle"]);
    fireEvent.click(screen.getByRole("button", { name: "Collapse Shared Branch" }));
    expectTones(["Alpha Contract Owner"]);
    expand("Shared Branch");
    expectTones(["Alpha Contract Owner", "Alpha Parent"]);
  });

  it("keeps deterministic hierarchy when coverage ties and stored display choices are stale", () => {
    const store = fixture();
    const projectKey = `reqvire:coverage:${JSON.stringify([store.project.workspace_root, store.project.repository, store.project.name])}`;
    localStorage.setItem(projectKey, JSON.stringify({ scopeId: cap("Alpha Root"), display: "roots" }));
    const rows = store.coverage.capability_coverage!.capabilities!;
    for (const row of rows) {
      row.verification_coverage_percentage = 50;
      row.implementation_coverage_percentage = 50;
    }
    rows.reverse();
    store.relations.reverse();
    const { container, rerender } = render(shell(store));
    expect(capabilityNames(container)).toEqual(["Alpha Root", "Alpha Left", "Shared Branch", "Alpha Right", "Empty Branch"]);
    const tied = structuredClone(store);
    tied.coverage.capability_coverage!.capabilities!.find(row => row.name === "Alpha Right")!.name = "Alpha Left";
    // Equal names use identifiers; the shared child remains below the left parent.
    rerender(shell(tied));
    expect(capabilityNames(container)).toEqual(["Alpha Root", "Alpha Left", "Shared Branch", "Alpha Left", "Empty Branch"]);
  });

  it("opens external binding consumers without changing scope and offers whole-model orphan navigation", () => {
    const open = vi.fn();
    render(shell(fixture(), open));
    selectScope("Alpha Root");
    expand("Alpha Left");
    expand("Alpha Contract Owner");
    const owner = screen.getByRole("article", { name: "Alpha Contract Owner" });
    expect(within(owner).getByRole("region", { name: "Binding consumers for Alpha Contract Owner" })).toBeTruthy();
    expect(within(owner).queryByText(/All required contract consumers/)).toBeNull();
    expect(within(owner).getByText("Outside scope")).toBeTruthy();
    expect(within(owner).getByRole("list", { name: "Binding consumers" }).textContent).toContain("Covered");
    fireEvent.click(screen.getByRole("link", { name: "requirement Beta Consumer" }));
    expect(open).toHaveBeenCalledWith("specifications/Beta.md#beta-consumer");
    expect(selectedScope()).toBe(cap("Alpha Root"));
    expect(screen.queryByText("0 / 5 orphaned")).toBeNull();
    selectScope("");
    expect(selectedScope()).toBe("");
    expect(screen.getByRole("heading", { name: "Whole Model" })).toBeTruthy();
    expect(within(document.getElementById("coverage-section-orphaned-verifications")!).getByRole("button", { name: /Orphan Check/ })).toBeTruthy();
    selectScope("Alpha Root");
    expect(selectedScope()).toBe(cap("Alpha Root"));
    expect(screen.getByText("1 / 3 terminal requirements covered")).toBeTruthy();
  });

  it("keeps direct and inherited artifacts in the report without repeating them in hierarchy rows", () => {
    const loaded = loadStoreCandidate(showcase);
    if (!loaded.ok) throw new Error(loaded.reason);
    const snapshot = structuredClone(loaded.store.coverage);
    const open = vi.fn();
    render(shell(loaded.store, open));
    selectScope("Alpha Root");
    openParent();
    expand("Alpha Middle");
    expect(screen.queryByRole("list", { name: /^Implementation evidence for/ })).toBeNull();
    expect(screen.queryByText("Direct", { exact: true })).toBeNull();
    expect(screen.queryByText("Via dependencies", { exact: true })).toBeNull();
    expect(screen.queryByRole("link", { name: /alpha.txt|parent.txt|SKILL.md/ })).toBeNull();
    const terminal = screen.getByRole("article", { name: "Alpha Implemented" });
    expect(within(terminal).queryByRole("button")).toBeNull();
    fireEvent.click(within(terminal).getByRole("link", { name: "requirement Alpha Implemented" }));
    expect(open).toHaveBeenCalledWith("specifications/Alpha.md#alpha-implemented");
    expect(loaded.store.coverage).toEqual(snapshot);
    expect(screen.getByText("1 / 3 terminal requirements covered")).toBeTruthy();
  });

  it("persists selections per project and uses one snapshot after refresh", () => {
    const store = fixture();
    const first = render(shell(store));
    selectScope("Alpha Root");
    first.unmount();
    const second = render(shell(store));
    expect(selectedScope()).toBe(cap("Alpha Root"));
    expect(capabilityNames(second.container)).toEqual(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right"]);
    second.rerender(shell({ ...store, project: { ...store.project, workspace_root: "/different-project" } }));
    expect(selectedScope()).toBe("");
    expect(capabilityNames(second.container)).toHaveLength(6);
    expect(screen.getByRole("heading", { name: "Whole Model" })).toBeTruthy();
  });

  it("shows parent blockers and aggregate metrics alongside child requirements", () => {
    const loaded = loadStoreCandidate(showcase);
    if (!loaded.ok) throw new Error(loaded.reason);
    const open = vi.fn();
    render(shell(loaded.store, open));
    expect(screen.getByRole("heading", { name: "Whole Model" })).toBeTruthy();
    selectScope("Alpha Root");
    expect(screen.getByText("1 / 3 terminal requirements covered")).toBeTruthy();
    openParent();
    expect(screen.queryByText(/Implementation depends on/)).toBeNull();
    const branch = screen.getByRole("article", { name: "Alpha Parent" });
    expect(within(branch).getByText("Blocked · 1 requirement", { exact: true })).toBeTruthy();
    const parentRow = branch.querySelector("[data-coverage-depth]")!;
    expect(parentRow.textContent).toContain("100% · 2 / 2 leaves");
    expect(parentRow.textContent).toContain("50% · 1 / 2 terminal");
    expect(within(parentRow as HTMLElement).queryByText("Verification", { exact: true })).toBeNull();
    expect(within(parentRow as HTMLElement).queryByText("Implementation", { exact: true })).toBeNull();
    expect(within(parentRow as HTMLElement).getByRole("group", { name: /^Verification: Verified;/ })).toBeTruthy();
    expect(within(parentRow as HTMLElement).getByRole("group", { name: /^Implementation: Blocked · 1 requirement;/ })).toBeTruthy();
    expect([...parentRow.querySelectorAll("[data-color-token]")].map(bar => [
      bar.getAttribute("data-color-token"), bar.querySelector("rect")?.getAttribute("width"),
    ])).toEqual([["--verification", "100"], ["--resource", "50"]]);
    expect(within(branch).getAllByRole("link", { name: "requirement Alpha Gap" })).toHaveLength(1);
    expect(within(branch).getAllByRole("link", { name: "requirement Alpha Middle" })).toHaveLength(1);
    expect(within(branch).getByRole("article", { name: "Alpha Middle" })).toBeTruthy();
    expect(within(branch).queryByRole("region")).toBeNull();
    const gap = screen.getByRole("article", { name: "Alpha Gap" });
    expect(within(gap).queryByRole("button")).toBeNull();
    expect(within(gap).queryByRole("region")).toBeNull();
    expect(within(gap).queryByText("Verified", { exact: true })).toBeNull();
    expect(within(gap).queryByText("Uncovered", { exact: true })).toBeNull();
    expect(within(gap).getByText("100% · 1 / 1 leaves")).toBeTruthy();
    expect(within(gap).getByText("0% · 0 / 1 terminal")).toBeTruthy();
    expect([...gap.querySelectorAll("[data-color-token]")].map(bar => [
      bar.getAttribute("data-color-token"), bar.querySelector("rect")?.getAttribute("width"),
    ])).toEqual([["--verification", "100"], ["--resource", "0"]]);
    fireEvent.click(within(gap).getByRole("link", { name: "requirement Alpha Gap" }));
    expect(open).toHaveBeenCalledWith("specifications/Alpha.md#alpha-gap");
  });

  it.each(["specify", "specifiedBy"])("opens nested capability attachments through %s even when their requirement parent is elsewhere", relationType => {
    const loaded = loadStoreCandidate(structuredClone(showcase));
    if (!loaded.ok) throw new Error(loaded.reason);
    const store = loaded.store;
    const middleId = "specifications/Alpha.md#alpha-middle";
    const capabilityId = cap("Alpha Right");
    store.relations.push({ ...store.relations[0], id: "nested-attachment", relation_type: relationType,
      canonical_relation_type: "specify", source_id: relationType === "specify" ? middleId : capabilityId,
      target_id: relationType === "specify" ? capabilityId : middleId });
    if (relationType === "specifiedBy") store.relations.reverse();
    const snapshot = structuredClone(store.coverage);
    const open = vi.fn();
    render(shell(store, open));
    selectScope("Alpha Root");
    const summary = screen.getByText(/terminal requirements covered/).textContent;
    expand("Alpha Right");
    const nested = screen.getByRole("article", { name: "Alpha Right" });
    const middle = within(nested).getByRole("article", { name: "Alpha Middle" });
    fireEvent.click(within(middle).getByRole("button", { name: "Expand Alpha Middle" }));
    const terminal = within(middle).getByRole("article", { name: "Alpha Implemented" });
    expect(within(terminal).queryByText("Verified", { exact: true })).toBeNull();
    expect(within(terminal).queryByRole("button")).toBeNull();
    expect(within(terminal).getByRole("link", { name: "requirement Alpha Implemented" }).getAttribute("href"))
      .toBe("#/elements/specifications/Alpha.md#alpha-implemented");
    fireEvent.click(within(middle).getByRole("link", { name: "requirement Alpha Middle" }));
    expect(open).toHaveBeenCalledWith(middleId);
    openParent();
    const shared = screen.getByRole("article", { name: "Shared Branch" });
    fireEvent.click(within(shared).getByRole("button", { name: "Expand Alpha Middle" }));
    expect(within(shared).getAllByRole("article", { name: "Alpha Implemented" })).toHaveLength(1);
    expect(within(nested).getAllByRole("article", { name: "Alpha Implemented" })).toHaveLength(1);
    fireEvent.click(within(shared).getByRole("button", { name: "Collapse Alpha Middle" }));
    expect(within(shared).queryByRole("article", { name: "Alpha Implemented" })).toBeNull();
    expect(within(nested).getByRole("article", { name: "Alpha Implemented" })).toBeTruthy();
    expect(screen.getByText(/terminal requirements covered/).textContent).toBe(summary);
    expect(store.coverage).toEqual(snapshot);
    fireEvent.click(within(nested).getByRole("button", { name: "Collapse Alpha Right" }));
    expect(within(nested).queryByRole("article", { name: "Alpha Middle" })).toBeNull();
    expect(within(shared).getByRole("article", { name: "Alpha Middle" })).toBeTruthy();
  });

  it("keeps a shared child reachable from its other parent without duplicating its tree row", () => {
    const store = fixture();
    const parentId = "specifications/Alpha.md#alpha-parent";
    const sharedId = "specifications/Alpha.md#alpha-implemented";
    store.relations.push({ ...store.relations[0], id: "shared-child", source_id: parentId, target_id: sharedId,
      relation_type: "derive", canonical_relation_type: "derive" });
    const records = store.coverage.uncovered_requirements as { files: Record<string, { identifier: string; contributing_requirements: string[] }[]> };
    Object.values(records.files).flat().find(record => record.identifier === parentId)!.contributing_requirements.push(sharedId);
    render(shell(store));
    openParent();
    expand("Alpha Middle");
    expect(screen.getAllByRole("article", { name: "Alpha Implemented" })).toHaveLength(1);
    const details = screen.getByRole("region", { name: "Coverage details for Alpha Parent" });
    const related = within(details).getByRole("list", { name: "Additional child requirements" });
    expect(within(related).getByRole("link", { name: "requirement Alpha Implemented" })).toBeTruthy();
    expect(related.textContent).toContain("Covered");
    expect(within(details).queryByRole("list", { name: "Binding consumers" })).toBeNull();
  });

  it("displays capability completeness separately when terminal percentage units are external", () => {
    const store = fixture();
    const owner = store.coverage.capability_coverage!.capabilities!.find(row => row.name === "Alpha Left")!;
    Object.assign(owner, { aggregate_terminal_requirements: 0, aggregate_covered_terminal_requirements: 0,
      aggregate_requirements: 1, aggregate_covered_requirements: 1, implementation_covered: true,
      implementation_coverage_percentage: 0 });
    const { container } = render(shell(store));
    selectScope("Alpha Root");
    const row = [...container.querySelectorAll<HTMLElement>('[data-kind="capability"][data-coverage-depth]')]
      .find(row => row.querySelector("a")?.textContent?.includes("Alpha Left"))!;
    expect(within(row).getByRole("group", { name: /^Implementation: Covered;/ })).toBeTruthy();
    expect(within(row).getByRole("group", { name: /^Implementation: Covered;/ }).querySelector("strong")).toBeNull();
    expect(row.textContent).toContain("0 / 0 terminal");
  });

  it("handles empty scopes and resets a removed scope with an explanation", () => {
    const store = fixture();
    const view = render(shell(store));
    selectScope("Empty Branch");
    expect(screen.getByText(/No requirements in this capability scope/)).toBeTruthy();
    expect(screen.getByText("0 / 0 terminal requirements covered")).toBeTruthy();
    const next = structuredClone(store);
    next.elements = next.elements.filter(row => row.id !== cap("Empty Branch"));
    const index = next.coverage.scope_index as Record<string, unknown>;
    delete index[cap("Empty Branch")];
    view.rerender(shell(next));
    expect(selectedScope()).toBe("");
    expect(screen.getByText(/selected capability is no longer available/i)).toBeTruthy();
  });

});
