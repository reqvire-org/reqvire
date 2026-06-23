import { useCallback, useEffect, useRef, useState } from "react";
import {
  DEFAULT_VIEW,
  parseHash,
  routeForContent,
  routeForElement,
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

  const read = useCallback((): ParsedRoute => {
    const parsed = parseHash(
      typeof window !== "undefined" ? window.location.hash : "",
      lastBaseRouteRef.current,
    );
    if (!parsed.elementId) {
      lastBaseRouteRef.current = { view: parsed.view, param: parsed.param };
    }
    return parsed;
  }, []);

  const [route, setRoute] = useState<ParsedRoute>(read);

  const applyHash = useCallback(
    (hash: string) => {
      window.location.hash = hash;
      setRoute(read());
    },
    [read],
  );

  useEffect(() => {
    const onHashChange = () => setRoute(read());
    window.addEventListener("hashchange", onHashChange);
    // Normalize an empty initial hash to the default route.
    if (!window.location.hash) {
      window.location.replace(routeForView(DEFAULT_VIEW));
    }
    return () => window.removeEventListener("hashchange", onHashChange);
  }, [read]);

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

  const closeElement = useCallback(() => {
    applyHash(routeForBase(lastBaseRouteRef.current));
  }, [applyHash]);

  return { route, navigateView, openElement, closeElement };
}

function routeForBase(route: Pick<ParsedRoute, "view" | "param">) {
  if (route.view === "content" && route.param) return routeForContent(route.param);
  if (route.view === "files" && route.param) return `#/files/${route.param}`;
  if (route.view === "resources" && route.param) return `#/resources/${route.param}`;
  if (route.view === "search" && route.param) return `#/search/${encodeURIComponent(route.param)}`;
  return routeForView(route.view);
}
