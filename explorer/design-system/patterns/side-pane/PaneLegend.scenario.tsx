import { PaneLegend } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./PaneLegend.fixture";
export const scenario = {"title": "PaneLegend", covers: ["PaneLegend"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneLegend rows={fixture.rows} />
</ShowcaseRegion>;
}
