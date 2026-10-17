import { ConceptReferenceList } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./ConceptReferenceList.fixture";
export const scenario = {"title": "ConceptReferenceList", covers: ["ConceptReferenceList"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ConceptReferenceList references={fixture.references} />
</ShowcaseRegion>;
}
