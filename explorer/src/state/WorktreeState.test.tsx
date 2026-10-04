import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { type ReactNode } from "react";
import { ExplorerUiStateProvider, useExplorerUiState } from "./ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { worktreeUrl } from "../store/worktreeUrls";

afterEach(cleanup);

describe("worktree navigation and assets", () => {
  it("keeps selections per context and clears unavailable identities on return", () => {
    let store = structuredClone(devFixture);
    store.project.worktree_id = "a";
    const identifier = store.elements[0].id;
    const wrapper = ({ children }: { children: ReactNode }) => <StoreProvider store={store} schemaMismatch={null}>
      <ExplorerUiStateProvider>{children}</ExplorerUiStateProvider>
    </StoreProvider>;
    const hook = renderHook(() => useExplorerUiState(), { wrapper });
    act(() => { hook.result.current.setModelMode("flow"); hook.result.current.setModelSelectionId(identifier); });
    store = { ...store, project: { ...store.project, worktree_id: "b" } };
    hook.rerender();
    expect(hook.result.current.modelSelectionId).toBe("__root__");
    expect(hook.result.current.modelMode).toBe("flow");
    store = { ...store, project: { ...store.project, worktree_id: "a" } };
    hook.rerender();
    expect(hook.result.current.modelSelectionId).toBe(identifier);
    store = { ...store, elements: store.elements.filter(element => element.id !== identifier) };
    hook.rerender();
    expect(hook.result.current.modelSelectionId).toBe("__root__");
    expect(hook.result.current.navigationNotice).toContain("unavailable");
  });

  it("preserves valid thesaurus IRIs rather than treating them as element identifiers", () => {
    const store = structuredClone(devFixture);
    store.project.worktree_id = "a";
    const wrapper = ({ children }: { children: ReactNode }) => <StoreProvider store={store} schemaMismatch={null}>
      <ExplorerUiStateProvider>{children}</ExplorerUiStateProvider>
    </StoreProvider>;
    const hook = renderHook(() => useExplorerUiState(), { wrapper });
    const concept = store.thesaurus.concepts[0];
    act(() => hook.result.current.setThesaurusSelectionId(concept.id));
    expect(hook.result.current.thesaurusSelectionId).toBe(concept.id);
    expect(hook.result.current.navigationNotice).toBeNull();
  });

  it("qualifies local assets only and preserves queries, fragments, and static export links", () => {
    expect(worktreeUrl("assets/a.svg?v=2#shape", "b")).toBe("assets/a.svg?v=2&worktree_id=b#shape");
    expect(worktreeUrl("/ontologies.ttl?worktree_id=a", "b")).toBe("/ontologies.ttl?worktree_id=b");
    for (const url of ["#/model", "https://example.test/a", "//example.test/a", "mailto:a@example.test", "data:image/svg+xml,x"])
      expect(worktreeUrl(url, "b")).toBe(url);
    expect(worktreeUrl("assets/a.svg")).toBe("assets/a.svg");
  });
});
