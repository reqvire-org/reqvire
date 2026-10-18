import { RouteFrame, RouteLayout, RoutePanel } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./RouteFrame.ads.fixture";
export const scenario = {"title": "RouteFrame", covers: ["RouteFrame", "RouteLayout", "RoutePanel"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<RouteFrame viewId="model"><RouteLayout><RoutePanel><p>Route content area.</p></RoutePanel></RouteLayout></RouteFrame>
</ShowcaseRegion>;
}
