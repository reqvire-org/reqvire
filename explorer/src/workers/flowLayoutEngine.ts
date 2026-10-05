import type { FlowLayoutEngine } from "@ds";
import type { ElkNode } from "elkjs/lib/elk-api";
import ELK from "elkjs/lib/elk-api";

type LayoutOutcome = { ok: true; graph: ElkNode } | { ok: false; error: string };

/** One task per mounted flow, cancelled by its owner before any replacement. */
export const flowLayoutEngine: FlowLayoutEngine = input => {
  const worker = new Worker(new URL("./flowLayout.worker.ts", import.meta.url), { type: "module" });
  let settled = false;
  let resolve!: (graph: ElkNode) => void;
  let reject!: (error: Error) => void;
  const result = new Promise<ElkNode>((yes, no) => { resolve = yes; reject = no; });
  const finish = (response: LayoutOutcome) => {
    if (settled) return;
    settled = true;
    worker.onmessage = null;
    worker.onerror = null;
    worker.onmessageerror = null;
    worker.terminate();
    if (response.ok) resolve(response.graph);
    else reject(new Error(response.error));
  };
  worker.onerror = event => {
    event.preventDefault();
    finish({ ok: false, error: event.message || "Flow layout worker failed to load or execute" });
  };
  worker.onmessageerror = () => finish({ ok: false, error: "Unable to read flow layout response" });
  try {
    const engine = new ELK({ algorithms: ["layered"], workerFactory: () => worker });
    void engine.layout(input).then(graph => {
      finish(graph?.id === input.id ? { ok: true, graph } : { ok: false, error: "Invalid flow layout response" });
    }, error => finish({ ok: false, error: error instanceof Error ? error.message : String(error) }));
  } catch (error) {
    finish({ ok: false, error: error instanceof Error ? error.message : String(error) });
  }
  return { result, cancel: () => finish({ ok: false, error: "Flow layout cancelled" }) };
};
