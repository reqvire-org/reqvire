import { useCallback, useEffect, useRef } from "react";
import { useExplorerLocation, writeExplorerHash } from "./location";
import {
  DEFAULT_VIEW,
  parseHash,
  routeForContent,
  routeForElement,
  routeForFile,
  routeForResource,
  routeForView,
  type ParsedRoute,
  type ViewId,
} from "./routes";

/**
 * Subscribe to `location.hash` and expose the parsed Explorer route plus
 * navigation helpers. Tracks the last non-element base view so element-detail
 * overlays render over the correct underlying view.
 */
export function useHashRoute() {
  const lastBaseRouteRef = useRef<Pick<ParsedRoute, "view" | "param">>({
    view: DEFAULT_VIEW,
    param: null,
  });

  const location = useExplorerLocation();
  const route = parseHash(new URL(location).hash, lastBaseRouteRef.current);
  if (!route.elementId) lastBaseRouteRef.current = { view: route.view, param: route.param };
  const applyHash = useCallback((hash: string) => writeExplorerHash(hash), []);

  useEffect(() => {
    // Normalize an empty initial hash to the default route.
    if (!window.location.hash) {
      writeExplorerHash(routeForView(DEFAULT_VIEW), true);
    }
  }, []);

  const navigateView = useCallback(
    (view: ViewId) => {
      applyHash(routeForView(view));
    },
    [applyHash],
  );

  const openElement = useCallback(
    (identifier: string) => {
      applyHash(routeForElement(identifier));
    },
    [applyHash],
  );

  const closeElement = useCallback((replace = false, resetSelection = false) => {
    const base = lastBaseRouteRef.current;
    const hash = resetSelection && ["coverage", "model", "traces", "thesaurus", "ontologies"].includes(base.view)
      ? routeForView(base.view) : routeForBase(base);
    writeExplorerHash(hash, replace);
  }, []);

  return { route, navigateView, openElement, closeElement };
}

function routeForBase(route: Pick<ParsedRoute, "view" | "param">) {
  if (["coverage", "model", "traces", "thesaurus", "ontologies"].includes(route.view) && route.param !== null) return `#/${route.view}?${route.param}`;
  if (route.view === "content" && route.param) return routeForContent(route.param);
  if (route.view === "files" && route.param) return routeForFile(route.param);
  if (route.view === "resources" && route.param) return routeForResource(route.param);
  if (route.view === "search" && route.param) return `#/search/${encodeURIComponent(route.param)}`;
  return routeForView(route.view);
}
