import { ElementFlow } from "@ds";
import { ShowcaseRegion } from "@ds/review";
import { flowLayoutEngine } from "@ds";
import { fixture } from "./ElementFlow.ads.fixture";
export const scenario = {"title": "ElementFlow", covers: ["ElementFlow"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ElementFlow data={fixture.data} layoutEngine={flowLayoutEngine} onOpenElement={() => {}} onOpenSource={() => {}} />
</ShowcaseRegion>;
}
