import { ShowcaseRegion, Icon } from "@ds";

import { fixture } from "./Icon.fixture";
export const scenario = {"title": "Icon", "covers": ["Icon"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ShowcaseRegion regions={["icon-grid"]}>{fixture.names.map(name => <ShowcaseRegion key={name} regions={["icon-item"]} as="span" title={name}><Icon name={name} /></ShowcaseRegion>)}</ShowcaseRegion>
</ShowcaseRegion>;
}
