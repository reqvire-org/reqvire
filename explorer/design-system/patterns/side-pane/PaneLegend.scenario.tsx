import { ShowcaseRegion, PaneLegend } from "@ds";

import { fixture } from "./PaneLegend.fixture";
export const scenario = {"title": "PaneLegend", "covers": ["PaneLegend"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneLegend rows={fixture.rows} />
</ShowcaseRegion>;
}
