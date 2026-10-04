import { createContext, useContext } from "react";

/** The adopted store owns asset routing, even while a different URL selection is loading. */
export const WorktreeUrlContext = createContext<string | undefined>(undefined);
export function worktreeUrl(url: string, worktreeId?: string): string {
  if (!worktreeId || !url || url.startsWith("#") || url.startsWith("//") || /^[a-z][a-z0-9+.-]*:/i.test(url)) return url;
  const [pathAndQuery, ...fragment] = url.split("#");
  const [path, query = ""] = pathAndQuery.split("?");
  const params = new URLSearchParams(query);
  params.set("worktree_id", worktreeId);
  return `${path}?${params}${fragment.length ? `#${fragment.join("#")}` : ""}`;
}
export function useWorktreeUrl() {
  const id = useContext(WorktreeUrlContext);
  return (url: string) => worktreeUrl(url, id);
}
