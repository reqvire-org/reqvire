import { MetadataStrip } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./MetadataStrip.ads.fixture";
export const scenario = {"title": "MetadataStrip", covers: ["MetadataStrip"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<MetadataStrip badges={fixture.badges} />
</ShowcaseRegion>;
}
