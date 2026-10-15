import type { OntologyGraphData } from "./types";
import type { mountOntologyGraph as Implementation } from "./ontologyGraphRendererImpl";
export type { OntologyGraphRendererHandle } from "./ontologyGraphRendererImpl";
export type OntologyGraphMountOptions = Parameters<typeof Implementation>[2] & { signal?: AbortSignal };

/** GPU dependencies load only when the consumer mounts an ontology graph. */
export async function mountOntologyGraph(container: HTMLElement, data: OntologyGraphData,
  options: OntologyGraphMountOptions = {}) {
  const implementation = await import("./ontologyGraphRendererImpl");
  if (options.signal?.aborted) throw new DOMException("Graph mounting cancelled", "AbortError");
  return implementation.mountOntologyGraph(container, data, options);
}
