import type Graph from "graphology";
import type { ForceAtlasInput, ForceAtlasPositions } from "../lib/forceAtlasLayout";

export interface ForceAtlasTask { result: Promise<ForceAtlasPositions>; cancel: () => void }
export type ForceAtlasEngine = (input: ForceAtlasInput) => ForceAtlasTask;

export function forceAtlasInput(graph: Graph): ForceAtlasInput {
  return {
    nodes: graph.mapNodes((id, { x, y, size }) => ({ id, x, y, size })),
    edges: graph.mapEdges((id, _attrs, source, target) => ({ id, source, target })),
  };
}

/** Each finite layout owns a worker; cancellation cannot affect another graph. */
export const forceAtlasLayoutEngine: ForceAtlasEngine = input => {
  let worker: Worker | undefined;
  let settled = false;
  let resolve!: (positions: ForceAtlasPositions) => void;
  let reject!: (error: Error) => void;
  const result = new Promise<ForceAtlasPositions>((yes, no) => { resolve = yes; reject = no; });
  const finish = (positions?: ForceAtlasPositions, error?: Error) => {
    if (settled) return;
    settled = true;
    if (worker) {
      worker.onmessage = null; worker.onerror = null; worker.onmessageerror = null;
      worker.terminate();
    }
    if (error) reject(error); else resolve(positions!);
  };
  try {
    worker = new Worker(new URL("./forceAtlasLayout.worker.ts", import.meta.url), { type: "module" });
    worker.onerror = event => { event.preventDefault(); finish(undefined, new Error(event.message || "ForceAtlas worker failed")); };
    worker.onmessageerror = () => finish(undefined, new Error("Unable to read ForceAtlas response"));
    worker.onmessage = ({ data }: MessageEvent) => {
      if (!data?.ok) { finish(undefined, new Error(data?.error || "ForceAtlas layout failed")); return; }
      const positions: ForceAtlasPositions = data.positions;
      const expected = new Set(input.nodes.map(node => node.id));
      if (!Array.isArray(positions) || positions.length !== expected.size || !positions.every(position =>
        position && expected.delete(position.id) && Number.isFinite(position.x) && Number.isFinite(position.y))) {
        finish(undefined, new Error("Invalid ForceAtlas response")); return;
      }
      finish(positions);
    };
    worker.postMessage(input);
  } catch (error) { finish(undefined, error instanceof Error ? error : new Error(String(error))); }
  return { result, cancel: () => finish(undefined, new Error("ForceAtlas layout cancelled")) };
};
