import { act, cleanup, renderHook } from "@testing-library/react";
import type { ReactNode } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { ExplorerUiStateProvider, useExplorerUiState } from "./ExplorerUiState";
import { routeForSelection } from "../router/routes";
import { writeExplorerHash } from "../router/location";
import { useHashRoute } from "../router/useHashRoute";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });
beforeEach(() => { localStorage.clear(); window.history.replaceState(null, "", "/#/model"); });
function setup(hash: string, routing = false) {
  window.history.replaceState(null, "", `/${routing ? '?worktree_id=a' : ''}${hash}`);
  const verification = devFixture.elements.find(node => node.element_type === "test-verification")!;
  let store = { ...devFixture, traces: { files: { [verification.file_path]: { verifications: [{ identifier: verification.id, name: verification.name, file: verification.file_path }] } } }, project: { ...devFixture.project, worktree_id: "a" } };
  const wrapper = ({ children }: { children: ReactNode }) => <StoreProvider store={store} schemaMismatch={null}>
    <ExplorerUiStateProvider worktreeRouting={routing}>{children}</ExplorerUiStateProvider>
  </StoreProvider>;
  const hook = renderHook(() => ({ ui: useExplorerUiState(), router: useHashRoute() }), { wrapper });
  return { ...hook, change: (next: typeof store) => { store = next; hook.rerender(); }, store };
}
function query() { return new URLSearchParams(window.location.hash.split('?')[1]); }
describe("shareable view selections", () => {
  it("restores explicit Model selection and mode, pushes once, and preserves selection under details", () => {
    const element = devFixture.elements[0];
    const hook = setup(routeForSelection("model", element.id, { mode: "flow" }));
    expect(hook.result.current.ui.modelSelectionId).toBe(element.id);
    expect(hook.result.current.ui.modelMode).toBe("flow");
    const push = vi.spyOn(history, "pushState");
    act(() => hook.result.current.ui.setModelSelectionId(`file:${element.file_path}`));
    act(() => hook.result.current.ui.setModelSelectionId(`file:${element.file_path}`));
    expect(push).toHaveBeenCalledTimes(1);
    const selectionHash = window.location.hash;
    act(() => hook.result.current.router.openElement(element.id));
    act(() => hook.result.current.router.closeElement());
    expect(window.location.hash).toBe(selectionHash);
    act(() => writeExplorerHash(routeForSelection("model", element.id, { mode: "grid" }), true));
    expect(hook.result.current.ui.modelMode).toBe("grid");
    expect(hook.result.current.ui.modelSelectionId).toBe(element.id);
  });
  it("resumes preferences on bare routes and lets copied links override them after remount", () => {
    let hook = setup("#/model");
    const element = devFixture.elements[0];
    act(() => { hook.result.current.ui.setModelMode("list"); hook.result.current.ui.setModelSelectionId(element.id); });
    hook.unmount(); hook = setup("#/model");
    expect(hook.result.current.ui.modelSelectionId).toBe(element.id);
    expect(hook.result.current.ui.modelMode).toBe("list");
    hook.unmount(); hook = setup(routeForSelection("model", null, { mode: "flow" }));
    expect(hook.result.current.ui.modelSelectionId).toBe("__root__");
    expect(hook.result.current.ui.modelMode).toBe("flow");
  });
  it("updates Traces verification and file together and restores file overviews", () => {
    const file = devFixture.elements.find(node => node.element_type === "test-verification")!.file_path;
    const verification = { identifier: devFixture.elements.find(node => node.element_type === "test-verification")!.id };
    const hook = setup(routeForSelection("traces", verification.identifier));
    expect(hook.result.current.ui.traceSelectionId).toBe(verification.identifier);
    expect(hook.result.current.ui.traceFilePath).toBe(file);
    const push = vi.spyOn(history, "pushState");
    act(() => hook.result.current.ui.setTraceFilePath(file));
    expect(push).toHaveBeenCalledTimes(1);
    expect(query().get("file")).toBe(file);
    expect(hook.result.current.ui.traceSelectionId).toBeNull();
    act(() => hook.result.current.ui.setTraceSelectionId(verification.identifier));
    expect(query().get("selected")).toBe(verification.identifier);
    expect(query().has("file")).toBe(false);
    act(() => writeExplorerHash(routeForSelection("traces", null, { file }), true));
    expect(hook.result.current.ui.traceSelectionId).toBeNull();
  });
  it("uses native concept identifiers in links and resolves their semantic IRI", () => {
    const concept = devFixture.thesaurus.concepts[0];
    const hook = setup(routeForSelection("thesaurus", concept.element_id!));
    expect(hook.result.current.ui.thesaurusSelectionId).toBe(concept.id);
    act(() => hook.result.current.ui.setThesaurusSelectionId(null));
    act(() => hook.result.current.ui.setThesaurusSelectionId(concept.id));
    expect(query().get("selected")).toBe(concept.element_id);
  });
  it("restores ontology IRIs and explains missing/wrong-kind selections with replacement history", () => {
    const id = devFixture.ontology.graph_data!.nodes![0].id;
    const hook = setup(routeForSelection("ontologies", id));
    expect(hook.result.current.ui.ontologySelectionId).toBe(id);
    const push = vi.spyOn(history, "pushState");
    act(() => writeExplorerHash(routeForSelection("ontologies", devFixture.elements[0].id), true));
    expect(hook.result.current.ui.ontologySelectionId).toBeNull();
    expect(query().get("selected")).toBe("");
    expect(hook.result.current.ui.navigationNotice).toContain("unavailable");
    expect(push).not.toHaveBeenCalled();
  });
  it("does not reinterpret a target URL against the retained worktree and resumes each context", () => {
    const id = devFixture.elements[0].id;
    const hook = setup(routeForSelection("model", id, { mode: "flow" }), true);
    act(() => writeExplorerHash(routeForSelection("model", null, { mode: "list" }), true));
    act(() => { history.replaceState(null, "", `/?worktree_id=b${routeForSelection("model", id, { mode: "grid" })}`); window.dispatchEvent(new PopStateEvent("popstate")); });
    expect(window.location.hash).toBe(routeForSelection("model", id, { mode: "grid" }));
    hook.change({ ...hook.store, project: { ...hook.store.project, worktree_id: "b" } });
    expect(hook.result.current.ui.modelSelectionId).toBe(id);
    expect(hook.result.current.ui.modelMode).toBe("grid");
    act(() => { history.replaceState(null, "", "/?worktree_id=a#/model"); window.dispatchEvent(new PopStateEvent("popstate")); });
    hook.change(hook.store);
    expect(hook.result.current.ui.modelSelectionId).toBe("__root__");
    expect(hook.result.current.ui.modelMode).toBe("list");
  });
  it.each(["traces", "thesaurus", "ontologies"] as const)("clears a removed %s selection and explains its fallback", view => {
    const id = view === "traces" ? devFixture.elements.find(node => node.element_type === "test-verification")!.id
      : view === "thesaurus" ? devFixture.thesaurus.concepts[0].element_id : devFixture.ontology.graph_data!.nodes![0].id;
    const hook = setup(routeForSelection(view, id));
    const next = view === "traces" ? { ...hook.store, traces: { files: {} }, elements: hook.store.elements.filter(node => node.type_family !== "verification") }
      : view === "thesaurus" ? { ...hook.store, thesaurus: { ...hook.store.thesaurus, concepts: [] } }
        : { ...hook.store, ontology: { ...hook.store.ontology, graph_data: { ...hook.store.ontology.graph_data!, nodes: [] } } };
    hook.change(next);
    const ui = hook.result.current.ui;
    expect(view === "traces" ? ui.traceSelectionId : view === "thesaurus" ? ui.thesaurusSelectionId : ui.ontologySelectionId).toBeNull();
    expect(ui.navigationNotice).toContain("unavailable");
    expect(query().get("selected")).toBe("");
  });
  it("rejects a requirement identifier used as a Traces verification", () => {
    const hook = setup(routeForSelection("traces", devFixture.elements.find(node => node.type_family === "requirement")!.id));
    expect(hook.result.current.ui.traceSelectionId).toBeNull();
    expect(hook.result.current.ui.navigationNotice).toContain("unavailable");
    expect(query().has("file")).toBe(true);
  });

  it("clears removed Model identities on refresh and keeps navigation usable without storage", () => {
    const id = devFixture.elements[0].id;
    const hook = setup(routeForSelection("model", id, { mode: "flow" }));
    hook.change({ ...hook.store, elements: hook.store.elements.filter(node => node.id !== id) });
    expect(hook.result.current.ui.modelSelectionId).toBe("__root__");
    expect(hook.result.current.ui.navigationNotice).toContain("unavailable");
    vi.spyOn(Storage.prototype, "setItem").mockImplementation(() => { throw new Error("Storage disabled"); });
    act(() => hook.result.current.ui.setModelMode("list"));
    expect(query().get("mode")).toBe("list");
  });
});
