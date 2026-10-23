import { ConceptElementDetailContent } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./ConceptElementDetailContent.ads.fixture";
export const scenario = {"title": "ConceptElementDetailContent", covers: ["ConceptElementDetailContent"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ConceptElementDetailContent definition={<p>{fixture.definition}</p>} onOpenElement={() => {}} />
</ShowcaseRegion>;
}
