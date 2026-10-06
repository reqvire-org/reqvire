import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { projectCoverage } from "../lib/coverage";
import { DEFAULT_VIEW, parseHash, routeForCoverage } from "../router/routes";
import { useExplorerLocation, writeExplorerUrl } from "../router/location";
import type { ExplorerProjectStore } from "../store/types";

export const COVERAGE_ISSUES = [
  { id: "unverified-requirements", label: "Unverified requirements", field: "unverified_leaf_requirements", emptyLabel: "All leaf requirements have verification." },
  { id: "unimplemented-requirements", label: "Unimplemented requirements", field: "uncovered_requirements", emptyLabel: "All requirements in scope are implementation-covered." },
  { id: "unsatisfied-verifications", label: "Unsatisfied verifications", field: "unsatisfied_test_verifications", emptyLabel: "All test verifications have evidence." },
  { id: "orphaned-verifications", label: "Orphaned verifications", field: "orphaned_verifications", emptyLabel: "Every verification links to a requirement." },
] as const;
interface Selection { scopeId: string | null; notice: string | null; }
function readPreferences(key: string): Selection {
  let stored;
  try { stored = JSON.parse(localStorage.getItem(key) ?? "null"); } catch { /* Optional browser storage. */ }
  return { scopeId: typeof stored?.scopeId === "string" ? stored.scopeId : null, notice: null };
}

/** One context-owned selection drives the URL, report projection and capability navigator. */
export function useCoverageNavigation(store: ExplorerProjectStore, worktreeRouting: boolean) {
  const location = useExplorerLocation();
  const url = new URL(location);
  const route = parseHash(url.hash, DEFAULT_VIEW);
  const active = route.view === "coverage" && !route.elementId;
  const requested = url.searchParams.get("worktree_id");
  const displayed = !worktreeRouting || !requested || requested === store.project.worktree_id;
  const identity = [store.project.workspace_root, store.project.repository, store.project.name];
  if (store.project.worktree_id) identity.push(store.project.worktree_id);
  const key = `reqvire:coverage:${JSON.stringify(identity)}`;
  const urlContext = useRef({ key, location });
  const sameContext = urlContext.current.key === key || urlContext.current.location !== location;
  if (displayed && urlContext.current.location !== location) urlContext.current = { key, location };
  const initial = useMemo(() => readPreferences(key), [key]);
  const [contexts, setContexts] = useState<Record<string, Selection>>({});
  const preferences = contexts[key] ?? initial;
  const query = displayed && sameContext && active && route.param !== null ? new URLSearchParams(route.param) : null;
  // Retired mode links still resolve to their originally visible scope, then canonicalize.
  const explicit = Boolean(query && ["scope", "mode", "issue", "section"].some(name => query.has(name)));
  const inputScope = explicit ? query!.get("mode") === "summary" || query!.get("issue") === "orphaned-verifications"
    ? null : query!.get("scope") || null : preferences.scopeId;
  const missing = inputScope !== null && (!store.coverage.scope_index[inputScope]
    || !store.elements.some(element => element.id === inputScope && element.element_type === "capability"));
  const scopeId = missing ? null : inputScope;
  const selection: Selection = { scopeId, notice: missing
    ? "The selected capability is no longer available. Showing the whole-model summary." : preferences.notice };
  const selectionRef = useRef(selection);
  selectionRef.current = selection;
  const navigationRef = useRef({ displayed, active });
  navigationRef.current = { displayed, active };
  const update = useCallback((next: Selection) => {
    setContexts(current => {
      if (JSON.stringify(current[key] ?? initial) === JSON.stringify(next)) return current;
      try { localStorage.setItem(key, JSON.stringify({ ...next, notice: undefined })); } catch { /* Session state remains usable. */ }
      return { ...current, [key]: next };
    });
  }, [key, initial]);
  const serialized = JSON.stringify(selection);
  useEffect(() => {
    if (!displayed || JSON.stringify(selectionRef.current) !== serialized) return;
    update(selection);
    if (active) {
      const nextUrl = new URL(location);
      nextUrl.hash = routeForCoverage(scopeId);
      if (worktreeRouting && store.project.worktree_id) nextUrl.searchParams.set("worktree_id", store.project.worktree_id);
      writeExplorerUrl(nextUrl, true);
    }
  }, [active, displayed, serialized, update, location, worktreeRouting, store.project.worktree_id]);

  const setCoverageScopeId = useCallback((nextScope: string | null) => {
    if (!navigationRef.current.displayed) return;
    const next: Selection = { scopeId: nextScope, notice: null };
    selectionRef.current = next;
    update(next);
    if (navigationRef.current.active) {
      const nextUrl = new URL(window.location.href);
      nextUrl.hash = routeForCoverage(nextScope);
      writeExplorerUrl(nextUrl);
    }
  }, [update]);
  const coverageProjection = useMemo(() => projectCoverage(store.coverage, scopeId), [store.coverage, scopeId]);
  return { coverageScopeId: scopeId, coverageNotice: selection.notice, coverageProjection, setCoverageScopeId };
}
