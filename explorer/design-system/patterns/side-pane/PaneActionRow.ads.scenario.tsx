import { PaneActionRow, Button, PaneGhostLink } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./PaneActionRow.ads.fixture";
export const scenario = {"title": "PaneActionRow", covers: ["PaneActionRow", "PaneGhostLink"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<PaneActionRow><Button>Inspect selection</Button><PaneGhostLink href="#source">Open source</PaneGhostLink></PaneActionRow>
</ShowcaseRegion>;
}
