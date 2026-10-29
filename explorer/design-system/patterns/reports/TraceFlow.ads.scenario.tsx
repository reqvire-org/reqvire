import { TraceFlow } from "@ds";
import { ShowcaseRegion } from "@ds/review";
import { flowLayoutEngine } from "@ds";
import { fixture } from "./TraceFlow.ads.fixture";
export const scenario = {"title": "TraceFlow", covers: ["TraceFlow"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ShowcaseRegion regions={["shell-demo"]}><TraceFlow trace={fixture.trace} layoutEngine={flowLayoutEngine} onOpenElement={() => {}} onOpenSource={() => {}} /></ShowcaseRegion>
</ShowcaseRegion>;
}
