import { ShowcaseRegion, GraphRoute, GraphCanvasFrame, GraphCanvasSurface, GraphCanvasNotice } from "@ds";

import { fixture } from "./GraphCanvasFrame.fixture";
export const scenario = {"title": "GraphCanvasFrame", "covers": ["GraphRoute", "GraphCanvasFrame", "GraphCanvasSurface", "GraphCanvasNotice"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<GraphRoute><GraphCanvasFrame><GraphCanvasSurface role="img" aria-label="Graph canvas slot" /><GraphCanvasNotice>Waiting for graph data.</GraphCanvasNotice></GraphCanvasFrame></GraphRoute>
</ShowcaseRegion>;
}
