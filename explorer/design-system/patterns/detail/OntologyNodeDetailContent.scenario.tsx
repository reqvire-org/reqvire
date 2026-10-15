import { ShowcaseRegion, OntologyConstructRows, OntologyPropertyUsages, OntologySourceList, OntologyTermList } from "@ds";

import { fixture } from "./OntologyNodeDetailContent.fixture";
export const scenario = {"title": "OntologyNodeDetailContent", "covers": ["OntologyConstructRows", "OntologyPropertyUsages", "OntologySourceList", "OntologyTermList"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<OntologyConstructRows constructs={fixture.constructs} /><OntologyPropertyUsages node={fixture.node} nodes={fixture.nodes} /><OntologySourceList sources={fixture.sources} onOpenSource={() => {}} /><OntologyTermList terms={fixture.terms} emptyLabel="No related terms" />
</ShowcaseRegion>;
}
