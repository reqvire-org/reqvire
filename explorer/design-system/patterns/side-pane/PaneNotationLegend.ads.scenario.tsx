import { PaneNotationLegend } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./PaneNotationLegend.ads.fixture";
export const scenario = {"title": "PaneNotationLegend", covers: ["PaneNotationLegend"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneNotationLegend rows={fixture.rows} />
</ShowcaseRegion>;
}
