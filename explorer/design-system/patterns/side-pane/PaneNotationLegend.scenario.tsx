import { ShowcaseRegion, PaneNotationLegend } from "@ds";

import { fixture } from "./PaneNotationLegend.fixture";
export const scenario = {"title": "PaneNotationLegend", "covers": ["PaneNotationLegend"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneNotationLegend rows={fixture.rows} />
</ShowcaseRegion>;
}
