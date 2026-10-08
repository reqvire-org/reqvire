import { useSyncExternalStore } from "react";

const LOCATION_CHANGE = "reqvire:location-change";
const readLocation = () => window.location.href;
function subscribe(listener: () => void) {
  window.addEventListener("hashchange", listener);
  window.addEventListener("popstate", listener);
  window.addEventListener(LOCATION_CHANGE, listener);
  return () => {
    window.removeEventListener("hashchange", listener);
    window.removeEventListener("popstate", listener);
    window.removeEventListener(LOCATION_CHANGE, listener);
  };
}

/** History API writes and browser traversal share one location snapshot. */
export function useExplorerLocation(): string {
  return useSyncExternalStore(subscribe, readLocation);
}

export function writeExplorerUrl(url: URL, replace = false) {
  if (url.href === window.location.href) return;
  if (replace) window.history.replaceState(window.history.state, "", url);
  else window.history.pushState(null, "", url);
  window.dispatchEvent(new Event(LOCATION_CHANGE));
}

export function writeExplorerHash(hash: string, replace = false) {
  const url = new URL(window.location.href);
  url.hash = hash;
  writeExplorerUrl(url, replace);
}
