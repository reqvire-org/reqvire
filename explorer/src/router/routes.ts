/*
 * Canonical Explorer hash routes.
 *
 * Routes are `index.html#/<view>` so the served Explorer works from local
 * files and simple static servers. Element identifiers themselves contain `#`
 * (e.g. `path/File.md#fragment`); since `location.hash` captures everything
 * after the first `#`, the inner `#` is preserved as part of the route string.
 */

export type ViewId =
  | "model"
  | "thesaurus"
  | "traces"
  | "ontologies"
  | "coverage"
  | "resources"
  | "files"
  | "search"
  | "content";

export const DEFAULT_VIEW: ViewId = "model";

/** Primary Explorer workspace view. */
export const PRIMARY_VIEWS: { id: ViewId; label: string }[] = [
  { id: "thesaurus", label: "Thesaurus" },
  { id: "model", label: "Model" },
];

/** Specialist views reached from the right vertical tool rail. */
export const TOOL_RAIL_VIEWS: { id: ViewId; label: string }[] = [
  { id: "ontologies", label: "Ontologies" },
  { id: "traces", label: "Traces" },
  { id: "coverage", label: "Coverage" },
];

/** Secondary/report views reachable by route but not in primary navigation. */
const SECONDARY_VIEWS: ViewId[] = ["resources", "files", "search", "content"];

const ALL_VIEW_IDS = new Set<ViewId>([
  ...PRIMARY_VIEWS.map((v) => v.id),
  ...TOOL_RAIL_VIEWS.map((v) => v.id),
  ...SECONDARY_VIEWS,
]);

export const VIEW_TITLES: Record<ViewId, string> = {
  model: "Model",
  thesaurus: "Thesaurus",
  traces: "Traces",
  ontologies: "Ontologies",
  coverage: "Coverage",
  resources: "Resources",
  files: "Model",
  search: "Search",
  content: "Content",
};

export interface ParsedRoute {
  /** Active base view rendered under any element-detail modal. */
  view: ViewId;
  /** Path/id param for `files`, `content`, and `resources` routes, or query for `search`. */
  param: string | null;
  /** Set when the route is an element-detail overlay (`#/elements/<id>`). */
  elementId: string | null;
}

type PreviousRoute = ViewId | Pick<ParsedRoute, "view" | "param">;

export function isViewId(value: string): value is ViewId {
  return ALL_VIEW_IDS.has(value as ViewId);
}

export function routeForView(view: ViewId): string {
  return `#/${view}`;
}

export function routeForElement(identifier: string): string {
  return `#/elements/${identifier}`;
}

export function routeForContent(path: string): string {
  return `#/content/${path}`;
}

export function routeForSearch(query: string): string {
  const trimmed = query.trim();
  return trimmed ? `#/search/${encodeURIComponent(trimmed)}` : "#/search";
}

/**
 * Parse a raw `location.hash` into a route. `previousView` is used as the base
 * view when the hash is an element-detail overlay so closing the modal returns
 * to the underlying Explorer route.
 */
export function parseHash(rawHash: string, previousRoute: PreviousRoute): ParsedRoute {
  const previous =
    typeof previousRoute === "string"
      ? { view: previousRoute, param: null }
      : previousRoute;
  let hash = rawHash.startsWith("#") ? rawHash.slice(1) : rawHash;
  if (hash.startsWith("/")) hash = hash.slice(1);

  if (hash === "") {
    return { view: DEFAULT_VIEW, param: null, elementId: null };
  }

  if (hash.startsWith("elements/")) {
    const identifier = hash.slice("elements/".length);
    return {
      view: previous.view,
      param: previous.param,
      elementId: identifier.length > 0 ? identifier : null,
    };
  }

  if (hash.startsWith("files/")) {
    return { view: "files", param: hash.slice("files/".length), elementId: null };
  }

  if (hash.startsWith("content/")) {
    return { view: "content", param: hash.slice("content/".length), elementId: null };
  }

  if (hash.startsWith("resources/")) {
    return { view: "resources", param: hash.slice("resources/".length), elementId: null };
  }

  if (hash === "search" || hash.startsWith("search/") || hash.startsWith("search?")) {
    const rawQuery = hash.startsWith("search/")
      ? hash.slice("search/".length)
      : hash.startsWith("search?")
        ? hash.slice("search?".length)
        : "";
    let q = rawQuery;
    try {
      q = decodeURIComponent(rawQuery);
    } catch {
      q = rawQuery;
    }
    return { view: "search", param: q || null, elementId: null };
  }

  const segment = hash.split("/")[0];
  if (segment === "knowledge-graph") {
    return { view: DEFAULT_VIEW, param: null, elementId: null };
  }
  if (isViewId(segment)) {
    return { view: segment, param: null, elementId: null };
  }

  return { view: DEFAULT_VIEW, param: null, elementId: null };
}
