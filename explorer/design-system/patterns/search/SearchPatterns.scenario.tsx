import { ShowcaseRegion, SearchPageToolbar, SearchResults, SearchEmptyState } from "@ds";
import { useState } from "preact/hooks";

import { fixture } from "./SearchPatterns.fixture";
export const scenario = {"title": "SearchPatterns", "covers": ["SearchPageToolbar", "SearchResults", "SearchEmptyState"]};
export default function Scenario() {
const [query,setQuery] = useState("");
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<SearchPageToolbar query={query} onQueryChange={setQuery} onSubmit={event => event.preventDefault()} resultSummary={`${fixture.results.length} fixture results`} statusText="Local fixture index" /><SearchResults results={fixture.results.filter(item => item.title.toLowerCase().includes(query.toLowerCase()))} /><SearchEmptyState />
</ShowcaseRegion>;
}
