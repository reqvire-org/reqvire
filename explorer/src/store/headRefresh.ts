/** Small action client; all diff/classification policy stays in the worker. */
const PROTOCOL = "2025-11-25";
let session: string | null = null;
let initialized: Promise<void> | undefined;
let sequence = 0;

async function rpc(method: string, params: unknown, signal: AbortSignal) {
  const response = await fetch("/mcp", {
    method: "POST", cache: "no-store", signal,
    headers: { "Content-Type": "application/json", Accept: "application/json, text/event-stream",
      "Mcp-Protocol-Version": PROTOCOL, ...(session ? { "Mcp-Session-Id": session } : {}) },
    body: JSON.stringify({ jsonrpc: "2.0", id: ++sequence, method, params }),
  });
  if (!response.ok) throw new Error(`HEAD refresh connection failed (${response.status}).`);
  session = response.headers.get("Mcp-Session-Id") ?? session;
  const value = await response.json();
  if (value.error) throw new Error(value.error.message ?? "HEAD refresh failed.");
  return value.result;
}

export async function refreshWorktreeHead(branch: string, signal: AbortSignal) {
  initialized ??= rpc("initialize", { protocolVersion: PROTOCOL, capabilities: {},
    clientInfo: { name: "reqvire-explorer", version: "1" } }, signal).then(() => undefined)
    .catch(error => { initialized = undefined; session = null; throw error; });
  await initialized;
  signal.throwIfAborted();
  const result = await rpc("tools/call", { name: "reqvire.worktree.refresh_head",
    arguments: { branch, dry_run: false } }, signal);
  const data = result?.structuredContent;
  if (data?.outcome === "blocked") {
    const paths = data.blockers?.map((b: { path?: string; reason: string; next_action?: string }) =>
      [b.path, b.reason, b.next_action].filter(Boolean).join(": ")).join("\n");
    throw new Error(`HEAD refresh blocked. Showing accepted commit ${data.accepted_head}; observed ${data.observed_head}. Ordinary writes are disabled.\n${paths ?? ""}`);
  }
  if (result?.isError || !["refreshed", "unchanged"].includes(data?.outcome)) {
    throw new Error(data?.error?.message ?? "HEAD refresh did not complete.");
  }
}
