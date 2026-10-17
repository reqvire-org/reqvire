import { ContractBindingList } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./ContractBindingList.fixture";
export const scenario = {"title": "ContractBindingList", covers: ["ContractBindingList"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ContractBindingList contract_bindings={fixture.bindings} onOpenElement={() => {}} />
</ShowcaseRegion>;
}
