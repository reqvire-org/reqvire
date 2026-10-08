import { act, cleanup, renderHook, waitFor } from "@testing-library/react";
import type { ReactNode } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { loadStoreCandidate } from "../store/loadStore";
import fixture from "../store/fixtures/scopedCoverage.json";
import { ExplorerUiStateProvider, useExplorerUiState } from "./ExplorerUiState";
import { routeForCoverage, worktreeUrl } from "../router/routes";
import { writeExplorerUrl } from "../router/location";
import { useHashRoute } from "../router/useHashRoute";

const root = "specifications/Capabilities.md#alpha-root";
const shared = "specifications/Capabilities.md#shared-branch";
afterEach(() => { cleanup(); vi.restoreAllMocks(); });
beforeEach(() => { localStorage.clear(); history.replaceState(null, "", "/"); });
function setup(hash: string, routing = false) {
  const loaded = loadStoreCandidate(structuredClone(fixture));
  if (!loaded.ok) throw new Error(loaded.reason);
  let store = { ...loaded.store, project: { ...loaded.store.project, worktree_id: "a" } };
  history.replaceState(null, "", `/${routing ? "?worktree_id=a" : ""}${hash}`);
  const wrapper = ({ children }: { children: ReactNode }) => <StoreProvider store={store} schemaMismatch={null}>
    <ExplorerUiStateProvider worktreeRouting={routing}>{children}</ExplorerUiStateProvider>
  </StoreProvider>;
  const hook = renderHook(() => ({ ui: useExplorerUiState(), router: useHashRoute() }), { wrapper });
  return { ...hook, store, change: (next: typeof store) => { store = next; hook.rerender(); } };
}

describe("Coverage scope navigation", () => {
  it("uses scope-only links and explicitly resets the whole-model dashboard", () => {
    const hook = setup(routeForCoverage(root));
    expect(hook.result.current.ui.coverageScopeId).toBe(root);
    const push = vi.spyOn(history, "pushState");
    act(() => hook.result.current.ui.setCoverageScopeId(root));
    expect(push).not.toHaveBeenCalled();
    act(() => hook.result.current.ui.setCoverageScopeId(null));
    expect(hook.result.current.ui.coverageProjection).toBe(hook.store.coverage);
    expect(location.hash).toBe(routeForCoverage(null));
    expect(push).toHaveBeenCalledTimes(1);
  });

  it("preserves scopes through details, copied reload and Back/Forward", async () => {
    const hook = setup(routeForCoverage(root));
    act(() => hook.result.current.ui.setCoverageScopeId(shared));
    const hash = location.hash;
    act(() => hook.result.current.router.openElement("specifications/Alpha.md#alpha-gap"));
    act(() => hook.result.current.router.closeElement());
    expect(location.hash).toBe(hash);
    act(() => hook.result.current.ui.setCoverageScopeId(null));
    act(() => history.back());
    await waitFor(() => expect(hook.result.current.ui.coverageScopeId).toBe(shared));
    act(() => history.forward());
    await waitFor(() => expect(hook.result.current.ui.coverageScopeId).toBeNull());
    hook.unmount(); localStorage.clear();
    const copied = setup(hash);
    expect(copied.result.current.ui.coverageScopeId).toBe(shared);
  });

  it.each([
    [`#/coverage?scope=${encodeURIComponent(root)}&mode=capabilities`, root],
    [`#/coverage?scope=${encodeURIComponent(root)}&mode=issues&issue=unimplemented-requirements`, root],
    [`#/coverage?scope=${encodeURIComponent(root)}&mode=summary&section=implementation`, null],
    [`#/coverage?scope=${encodeURIComponent(root)}&mode=issues&issue=orphaned-verifications`, null],
    ["#/coverage?mode=capabilities", null],
  ])("migrates retired view links to the complete dashboard for their visible scope: %s", (hash, scope) => {
    const hook = setup(hash);
    expect(hook.result.current.ui.coverageScopeId).toBe(scope);
    expect(location.hash).toBe(routeForCoverage(scope));
  });

  it.each(["missing", "specifications/Alpha.md#alpha-gap"])("explains unavailable scope %s", id => {
    const hook = setup(routeForCoverage(id));
    expect(hook.result.current.ui.coverageScopeId).toBeNull();
    expect(hook.result.current.ui.coverageNotice).toContain("capability");
    expect(location.hash).toBe(routeForCoverage(null));
  });

  it("keeps accepted scope while loading and restores each worktree independently", () => {
    const hook = setup(routeForCoverage(root), true);
    const target = worktreeUrl(location.href, "b");
    act(() => writeExplorerUrl(target));
    expect(hook.result.current.ui.coverageScopeId).toBe(root);
    expect(location.href).toBe(target.href);
    const b = { ...hook.store, project: { ...hook.store.project, worktree_id: "b" } };
    hook.change(b);
    expect(hook.result.current.ui.coverageScopeId).toBeNull();
    expect(location.hash).toBe(routeForCoverage(null));
    act(() => hook.result.current.ui.setCoverageScopeId(shared));
    act(() => writeExplorerUrl(worktreeUrl(location.href, "a")));
    hook.change(hook.store);
    expect(hook.result.current.ui.coverageScopeId).toBe(root);
    expect(location.hash).toBe(routeForCoverage(root));
  });
});
