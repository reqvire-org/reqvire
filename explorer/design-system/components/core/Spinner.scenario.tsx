import { ShowcaseRegion, Spinner } from "@ds";

import { fixture } from "./Spinner.fixture";
export const scenario = {"title": "Spinner", "covers": ["Spinner"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<Spinner label="Loading project" /><Spinner size="sm" label="Loading compact preview" />
</ShowcaseRegion>;
}
