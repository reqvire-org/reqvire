import { useCallback, useEffect, useState } from "react";
import { buildTraceFlowGraph, type TraceFlowGraph, type TraceFlowTopology, type TraceFlowDirection } from "./traceFlowLayout";

/** Keep the map available during layout and apply only the latest request. */
export function useTraceFlowLayout(topology: TraceFlowTopology, direction: TraceFlowDirection = "RIGHT") {
  const [attempt, setAttempt] = useState(0);
  const [state, setState] = useState<{ graph: TraceFlowGraph | null; pending: boolean; failed: boolean }>({
    graph: null, pending: true, failed: false,
  });
  useEffect(() => {
    let cancelled = false;
    setState(previous => ({ ...previous, pending: true, failed: false }));
    void buildTraceFlowGraph(topology, direction).then(graph => {
      if (!cancelled) setState({ graph, pending: false, failed: false });
    }).catch(() => {
      if (!cancelled) setState(previous => ({ ...previous, pending: false, failed: true }));
    });
    return () => { cancelled = true; };
  }, [topology, direction, attempt]);
  const retry = useCallback(() => setAttempt(previous => previous + 1), []);
  return { ...state, retry };
}
