import { ShowcaseRegion, TraceFlow } from "@ds";
import { flowLayoutEngine } from "@ds";
import { fixture } from "./TraceFlow.fixture";
export const scenario = {"title": "TraceFlow", "covers": ["TraceFlow"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<TraceFlow trace={fixture.trace} layoutEngine={flowLayoutEngine} onOpenElement={() => {}} onOpenSource={() => {}} />
</ShowcaseRegion>;
}
