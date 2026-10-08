import { runForceAtlasLayout, type ForceAtlasInput } from "../lib/forceAtlasLayout";

self.onmessage = (event: MessageEvent<ForceAtlasInput>) => {
  try { self.postMessage({ ok: true, positions: runForceAtlasLayout(event.data) }); }
  catch (error) { self.postMessage({ ok: false, error: error instanceof Error ? error.message : String(error) }); }
};
