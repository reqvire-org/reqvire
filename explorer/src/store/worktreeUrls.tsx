import { createContext } from "preact";
import { useContext } from "preact/hooks";

/** The adopted store owns asset routing, even while a different URL selection is loading. */
export const WorktreeUrlContext = createContext<string | undefined>(undefined);
export function worktreeUrl(url: string, branch?: string): string {
  if (!branch || !url || url.startsWith("#") || url.startsWith("//") || /^[a-z][a-z0-9+.-]*:/i.test(url)) return url;
  const [pathAndQuery, ...fragment] = url.split("#");
  const [path, query = ""] = pathAndQuery.split("?");
  const params = new URLSearchParams(query);
  params.delete("worktree_id");
  params.set("branch", branch);
  return `${path}?${params}${fragment.length ? `#${fragment.join("#")}` : ""}`;
}
export function useWorktreeUrl() {
  const id = useContext(WorktreeUrlContext);
  return (url: string) => worktreeUrl(url, id);
}
