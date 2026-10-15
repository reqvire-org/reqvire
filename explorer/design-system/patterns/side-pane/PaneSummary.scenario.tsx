import { ShowcaseRegion, PaneSummary } from "@ds";

import { fixture } from "./PaneSummary.fixture";
export const scenario = {"title": "PaneSummary", "covers": ["PaneSummary"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneSummary items={fixture.items} />
</ShowcaseRegion>;
}
