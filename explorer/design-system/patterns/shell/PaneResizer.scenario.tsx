import { ShowcaseRegion, PaneResizer } from "@ds";
import { useState } from "preact/hooks";

import { fixture } from "./PaneResizer.fixture";
export const scenario = {"title": "PaneResizer", "covers": ["PaneResizer"]};
export default function Scenario() {
const [width,setWidth] = useState(320);
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneResizer aria-label="Resize preview pane" onKeyDown={event => { if(event.key === "ArrowRight") setWidth(width+10); if(event.key === "ArrowLeft") setWidth(width-10); }} /><p>Preview pane width: {width}</p>
</ShowcaseRegion>;
}
