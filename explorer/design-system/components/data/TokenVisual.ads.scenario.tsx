import { TokenSwatch, DonutMeter, ConicSwatch, SegmentedMeter, BarMeterFill } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./TokenVisual.ads.fixture";
export const scenario = {"title": "TokenVisual", covers: ["TokenSwatch", "DonutMeter", "ConicSwatch", "SegmentedMeter", "BarMeterFill"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<TokenSwatch colorToken="--accent" /><DonutMeter percent={fixture.percent} colorToken="--success">75%</DonutMeter>
<ConicSwatch segments={fixture.segments} /><SegmentedMeter segments={fixture.segments} /><BarMeterFill value={fixture.percent} colorToken="--accent" />
</ShowcaseRegion>;
}
