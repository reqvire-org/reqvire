import { ShowcaseRegion, ConceptElementDetailContent } from "@ds";

import { fixture } from "./ConceptElementDetailContent.fixture";
export const scenario = {"title": "ConceptElementDetailContent", "covers": ["ConceptElementDetailContent"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ConceptElementDetailContent definition={<p>{fixture.definition}</p>} onOpenElement={() => {}} />
</ShowcaseRegion>;
}
