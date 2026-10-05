import { fireEvent, render, screen, within } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
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

function shell(store: ExplorerProjectStore, onOpenElement = vi.fn()) {
  return <StoreProvider store={store} schemaMismatch={null}><ExplorerUiStateProvider>
    <ExplorerSidePane activeView="coverage" open onToggle={vi.fn()} onNavigate={vi.fn()}
      onOpenElement={onOpenElement} onOpenOntologyNode={vi.fn()} />
    <CoverageView onOpenElement={onOpenElement} />
  </ExplorerUiStateProvider></StoreProvider>;
}
function selectScope(name: string) {
  fireEvent.change(screen.getByRole("combobox", { name: "Scope" }), { target: { value: name ? cap(name) : "" } });
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
    Element.prototype.scrollIntoView = vi.fn();
  });

  it("nests requirements while terminal names open element details", () => {
    const loaded = loadStoreCandidate(showcase);
    if (!loaded.ok) throw new Error(loaded.reason);
    render(shell(loaded.store));
    const header = screen.getByRole("heading", { name: "Coverage" }).closest(".coverage-header")!;
    expect(within(header as HTMLElement).getByRole("combobox", { name: "Scope" })).toBeTruthy();
    expect(screen.queryByRole("heading", { name: "Implementation evidence" })).toBeNull();
    expect(screen.queryByRole("article", { name: "Alpha Parent" })).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Expand Shared Branch" }));
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
    expect(within(terminal).getByText("Verified", { exact: true })).toBeTruthy();
    expect(within(terminal).getByText("Covered", { exact: true })).toBeTruthy();
    expect(within(terminal).getByText("100% · 1 / 1 leaves")).toBeTruthy();
    expect(within(terminal).getByText("100% · 1 / 1 terminal")).toBeTruthy();
    expect([...terminal.querySelectorAll("[data-color-token]")].map(bar => [
      bar.getAttribute("data-color-token"), bar.querySelector("rect")?.getAttribute("width"),
    ])).toEqual([["--requirement", "100"], ["--resource", "100"]]);
    expect(within(terminal).queryByRole("button", { name: /Evidence for/ })).toBeNull();
    expect(within(terminal).queryByRole("list")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Collapse Shared Branch" }));
    expect(screen.queryByRole("article", { name: "Alpha Parent" })).toBeNull();
  });

  it("ranks roots and siblings within one hierarchy and preserves both parent paths", () => {
    const store = fixture();
    const { container } = render(shell(store));
    expect(capabilityNames(container)).toEqual(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right", "Beta Root"]);
    expect(screen.getAllByText("Partially verified", { exact: true }).length).toBeGreaterThan(0);
    expect(screen.queryByRole("button", { name: "Root summaries" })).toBeNull();
    expect(screen.queryByRole("button", { name: "Hierarchy" })).toBeNull();
    expect(screen.queryByRole("button", { name: "Ranked capabilities" })).toBeNull();
    selectScope("Alpha Root");
    expect(screen.getByText("1 / 3 terminal requirements covered")).toBeTruthy();
    expect(capabilityNames(container)).toEqual(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right"]);
    expect([...container.querySelectorAll('[data-kind="capability"][data-coverage-depth]')].map(row => row.getAttribute("data-coverage-depth")))
      .toEqual(["0", "1", "1", "2", "1"]);
    const pane = screen.getByRole("region", { name: "Coverage explorer" });
    expect(within(pane).getByRole("button", { name: /Unimplemented requirements.*3/ })).toBeTruthy();
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
    expect((screen.getByRole("combobox", { name: "Scope" }) as HTMLSelectElement).value).toBe(cap("Alpha Root"));
    expect(screen.queryByText("0 / 5 orphaned")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "View whole-model orphan diagnostics" }));
    expect((screen.getByRole("combobox", { name: "Scope" }) as HTMLSelectElement).value).toBe("");
    expect(screen.getByText("1 / 7 orphaned")).toBeTruthy();
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
    expect((screen.getByRole("combobox", { name: "Scope" }) as HTMLSelectElement).value).toBe(cap("Alpha Root"));
    expect(capabilityNames(second.container)).toEqual(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right"]);
    second.rerender(shell({ ...store, project: { ...store.project, workspace_root: "/different-project" } }));
    expect((screen.getByRole("combobox", { name: "Scope" }) as HTMLSelectElement).value).toBe("");
    expect(capabilityNames(second.container)).toHaveLength(6);
  });

  it("shows parent blockers and aggregate metrics alongside child requirements", () => {
    const loaded = loadStoreCandidate(showcase);
    if (!loaded.ok) throw new Error(loaded.reason);
    const open = vi.fn();
    render(shell(loaded.store, open));
    expect(screen.getByRole("heading", { name: "Coverage" })).toBeTruthy();
    selectScope("Alpha Root");
    expect(screen.getByText("1 / 3 terminal requirements covered")).toBeTruthy();
    openParent();
    expect(screen.queryByText(/Implementation depends on/)).toBeNull();
    const branch = screen.getByRole("article", { name: "Alpha Parent" });
    expect(within(branch).getByText("Blocked · 1 requirement", { exact: true })).toBeTruthy();
    const parentRow = branch.querySelector("[data-coverage-depth]")!;
    expect(parentRow.textContent).toContain("100% · 2 / 2 leaves");
    expect(parentRow.textContent).toContain("50% · 1 / 2 terminal");
    expect([...parentRow.querySelectorAll("[data-color-token]")].map(bar => [
      bar.getAttribute("data-color-token"), bar.querySelector("rect")?.getAttribute("width"),
    ])).toEqual([["--requirement", "100"], ["--resource", "50"]]);
    expect(within(branch).getAllByRole("link", { name: "requirement Alpha Gap" })).toHaveLength(1);
    expect(within(branch).getAllByRole("link", { name: "requirement Alpha Middle" })).toHaveLength(1);
    expect(within(branch).getByRole("article", { name: "Alpha Middle" })).toBeTruthy();
    expect(within(branch).queryByRole("region")).toBeNull();
    const gap = screen.getByRole("article", { name: "Alpha Gap" });
    expect(within(gap).queryByRole("button")).toBeNull();
    expect(within(gap).queryByRole("region")).toBeNull();
    expect(within(gap).getByText("Verified", { exact: true })).toBeTruthy();
    expect(within(gap).getByText("Uncovered", { exact: true })).toBeTruthy();
    expect(within(gap).getByText("100% · 1 / 1 leaves")).toBeTruthy();
    expect(within(gap).getByText("0% · 0 / 1 terminal")).toBeTruthy();
    expect([...gap.querySelectorAll("[data-color-token]")].map(bar => [
      bar.getAttribute("data-color-token"), bar.querySelector("rect")?.getAttribute("width"),
    ])).toEqual([["--requirement", "100"], ["--resource", "0"]]);
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
    const summary = screen.getByText(/terminal requirements covered/).textContent;
    expand("Alpha Right");
    const nested = screen.getByRole("article", { name: "Alpha Right" });
    const middle = within(nested).getByRole("article", { name: "Alpha Middle" });
    fireEvent.click(within(middle).getByRole("button", { name: "Expand Alpha Middle" }));
    const terminal = within(middle).getByRole("article", { name: "Alpha Implemented" });
    expect(within(terminal).getByText("Verified", { exact: true })).toBeTruthy();
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
    const row = [...container.querySelectorAll('[data-kind="capability"][data-coverage-depth]')]
      .find(row => row.querySelector("a")?.textContent?.includes("Alpha Left"))!;
    expect(row.textContent).toContain("ImplementationCovered");
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
    expect((screen.getByRole("combobox", { name: "Scope" }) as HTMLSelectElement).value).toBe("");
    expect(screen.getByText(/selected capability is no longer available/i)).toBeTruthy();
  });

});
