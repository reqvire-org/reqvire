import { useCallback, useEffect, useRef, useState } from "react";
import {
  DEFAULT_VIEW,
  parseHash,
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
  const lastViewRef = useRef<ViewId>(DEFAULT_VIEW);

  const read = useCallback((): ParsedRoute => {
    const parsed = parseHash(
      typeof window !== "undefined" ? window.location.hash : "",
      lastViewRef.current,
    );
    lastViewRef.current = parsed.view;
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
    applyHash(routeForView(lastViewRef.current));
  }, [applyHash]);

  return { route, navigateView, openElement, closeElement };
}
