import { lazy, Suspense } from "preact/compat";
import { GraphCanvasFrame, GraphCanvasNotice, GraphCanvasSurface } from "../content/GraphCanvasFrame";
import { Spinner } from "../../components/core/Spinner";
import type { KnowledgeGraphCanvasProps } from "./KnowledgeGraphCanvasImpl";
export type { KnowledgeGraphCanvasProps } from "./KnowledgeGraphCanvasImpl";

const Graph = lazy(() => import("./KnowledgeGraphCanvasImpl"));
export function KnowledgeGraphCanvas(props: KnowledgeGraphCanvasProps) {
  return <Suspense fallback={<GraphCanvasFrame aria-busy="true">
    <GraphCanvasSurface data-testid="kg-sigma-canvas" role="img" aria-label="Actual project elements and facts graph" />
    <GraphCanvasNotice><Spinner label="Loading graph..." /><span>Loading graph...</span></GraphCanvasNotice>
  </GraphCanvasFrame>}><Graph {...props} /></Suspense>;
}
