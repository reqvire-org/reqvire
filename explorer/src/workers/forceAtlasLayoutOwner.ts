import type Graph from "graphology";
import { forceAtlasInput, forceAtlasLayoutEngine, type ForceAtlasEngine, type ForceAtlasTask } from "./forceAtlasLayoutEngine";
import type { ForceAtlasPositions } from "../lib/forceAtlasLayout";

/** One active job per mounted renderer, with generation guards for late replies. */
export class ForceAtlasLayoutOwner {
  private task?: ForceAtlasTask;
  private generation = 0;
  constructor(private readonly engine: ForceAtlasEngine = forceAtlasLayoutEngine) {}
  cancel() { this.generation++; this.task?.cancel(); this.task = undefined; }
  run(graph: Graph, apply: (positions: ForceAtlasPositions) => void, fail: (error: Error) => void) {
    this.cancel();
    const generation = this.generation;
    try {
      const task = this.engine(forceAtlasInput(graph));
      this.task = task;
      void task.result.then(positions => {
        if (generation !== this.generation) return;
        this.task = undefined;
        apply(positions);
      }).catch(error => {
        if (generation !== this.generation) return;
        this.task = undefined;
        fail(error instanceof Error ? error : new Error(String(error)));
      });
    } catch (error) { fail(error instanceof Error ? error : new Error(String(error))); }
  }
}
