import { RouteFrame, RouteLayout, RoutePanel } from "../shell/RouteFrame";
import { SearchPageToolbar, SearchResults, type SearchPageToolbarProps, type SearchResultsProps } from "./SearchPatterns";

export interface SearchScreenProps extends SearchPageToolbarProps, SearchResultsProps {}
export function SearchScreen({ results, emptyMessage, ...toolbar }: SearchScreenProps) {
  return <RouteFrame viewId="search"><RouteLayout><RoutePanel>
    <SearchPageToolbar {...toolbar} />
    <SearchResults results={results} emptyMessage={emptyMessage} />
  </RoutePanel></RouteLayout></RouteFrame>;
}
