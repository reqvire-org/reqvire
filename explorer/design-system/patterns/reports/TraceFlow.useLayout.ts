import { useCallback, useEffect, useState } from "preact/hooks";
import { buildTraceFlowGraph, type TraceFlowGraph, type TraceFlowTopology, type TraceFlowDirection, type FlowLayoutEngine, type FlowLayoutTask, type TraceFlowNodeSize } from "./TraceFlow.layout";

/** Keep the map available during layout and apply only the latest request. */
export function useTraceFlowLayout(topology: TraceFlowTopology, direction: TraceFlowDirection, layoutEngine: FlowLayoutEngine, nodeSize: TraceFlowNodeSize | null) {
  const [attempt, setAttempt] = useState(0);
  const [state, setState] = useState<{ graph: TraceFlowGraph | null; pending: boolean; failed: boolean }>({
    graph: null, pending: true, failed: false,
  });
  const width = nodeSize?.width;
  const height = nodeSize?.height;
  useEffect(() => {
    if (width === undefined || height === undefined) return;
    let cancelled = false;
    let task: FlowLayoutTask | undefined;
    setState(previous => ({ ...previous, pending: true, failed: false }));
    void buildTraceFlowGraph(topology, direction, input => {
      task = layoutEngine(input);
      return task.result;
    }, { width, height }).then(graph => {
      if (!cancelled) setState({ graph, pending: false, failed: false });
    }).catch(() => {
      if (!cancelled) setState(previous => ({ ...previous, pending: false, failed: true }));
    });
    return () => { cancelled = true; task?.cancel(); };
  }, [topology, direction, layoutEngine, attempt, width, height]);
  const retry = useCallback(() => setAttempt(previous => previous + 1), []);
  return { ...state, retry };
}
