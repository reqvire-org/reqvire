import { Button } from "../../components/core/Button";
import { ElementIcon } from "../../components/data/ElementIcon";
import { ToggleRow } from "../../components/controls/ToggleRow";
import { PaneControlSection, PaneFilterGroup } from "../side-pane/PaneControls";
import type { DesignSystemColorToken } from "../../palette";

export type SearchResultKind = "file" | "element" | "resource" | "ontology";
const KINDS = ["file", "element", "resource", "ontology"] as const;
const LABELS: Record<SearchResultKind, string> = { file: "Files", element: "Elements", resource: "Resources", ontology: "Ontology terms" };
const COLORS: Record<SearchResultKind, DesignSystemColorToken> = {
  file: "--color-role-resource", element: "--color-role-requirement",
  resource: "--color-role-ontology", ontology: "--color-role-rdf-resource",
};
export interface SearchControlsPaneProps {
  kinds: ReadonlySet<SearchResultKind>;
  counts: Record<SearchResultKind, number>;
  elementTypes: ReadonlySet<string>;
  options: readonly { type: string; family: string; count: number }[];
  onReset: () => void;
  onToggleKind: (kind: SearchResultKind) => void;
  onToggleElementType: (type: string) => void;
}
export function SearchControlsPane({ kinds, counts, elementTypes, options, onReset,
  onToggleKind, onToggleElementType }: SearchControlsPaneProps) {
  return <PaneControlSection aria-label="Search controls" title="Filter by">
    <Button size="sm" onClick={onReset}>Reset filters</Button>
    <PaneFilterGroup label="Result types">{KINDS.map(kind => <ToggleRow key={kind}
      label={LABELS[kind]} on={kinds.has(kind)} variant="filter" colorToken={COLORS[kind]}
      meta={formatCompactCount(counts[kind] ?? 0)} onToggle={() => onToggleKind(kind)} />)}</PaneFilterGroup>
    {options.length > 0 && <PaneFilterGroup label="Element types">{options.map(option => <ToggleRow key={option.type}
      label={option.type.replace(/-/g, " ").replace(/\b\w/g, match => match.toUpperCase())}
      on={elementTypes.has(option.type)} variant="filter"
      icon={<ElementIcon type={option.type} family={option.family} size="sm" />}
      meta={formatCompactCount(option.count)} onToggle={() => onToggleElementType(option.type)} />)}</PaneFilterGroup>}
  </PaneControlSection>;
}
function formatCompactCount(value: number): string {
  const trim = (number: number) => number >= 10 ? Math.round(number).toString() : number.toFixed(1).replace(/\.0$/, "");
  return value >= 1_000_000 ? `${trim(value / 1_000_000)}M` : value >= 1_000 ? `${trim(value / 1_000)}K` : value.toLocaleString();
}
