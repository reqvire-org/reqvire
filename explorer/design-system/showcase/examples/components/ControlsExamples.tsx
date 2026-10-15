import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { useState } from "preact/hooks";
import { Icon, SearchInput, SegmentedControl, ToggleRow } from "@ds";

function Section({ title, desc, children }: { title: string; desc?: string; children: ComponentChildren }) {
  return (
    <ShowcaseRegion regions={["section"]} as="section">
      <ShowcaseRegion regions={["section__heading"]}>
        <ShowcaseRegion regions={["section__title"]}>{title}</ShowcaseRegion>
        {desc && <ShowcaseRegion regions={["section__desc"]}>{desc}</ShowcaseRegion>}
      </ShowcaseRegion>
      {children}
    </ShowcaseRegion>
  );
}

export function ControlsExamples() {
  const [query, setQuery] = useState("");
  const [queryLg, setQueryLg] = useState("");
  const [view, setView] = useState<"list" | "grid">("list");
  const [mode, setMode] = useState<"model" | "graph" | "traces">("model");

  return (
    <ShowcaseRegion regions={["page"]}>

      <Section title="SearchInput" desc="Controlled text input with leading icon and optional clear.">
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Default (md)</ShowcaseRegion>
          <ShowcaseRegion regions={["input-demo", "input-demo--md"]}>
            <SearchInput
              value={query}
              onInput={(e) => setQuery(e.currentTarget.value)}
              placeholder="Search elements, files, relations…"
            />
          </ShowcaseRegion>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Large (lg)</ShowcaseRegion>
          <ShowcaseRegion regions={["input-demo", "input-demo--lg"]}>
            <SearchInput
              size="lg"
              value={queryLg}
              onInput={(e) => setQueryLg(e.currentTarget.value)}
              placeholder="Search the project store…"
            />
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="SegmentedControl" desc="Mutually exclusive button group. Active segment inverts to dark ink fill.">
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Text only</ShowcaseRegion>
          <SegmentedControl
            items={[
              { value: "model" as const, label: "Model" },
              { value: "graph" as const, label: "Graph" },
              { value: "traces" as const, label: "Traces" },
            ]}
            value={mode}
            onChange={setMode}
            ariaLabel="View mode"
          />
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>With icons</ShowcaseRegion>
          <SegmentedControl
            items={[
              { value: "list" as const, label: "List", icon: <Icon name="list" size={14} /> },
              { value: "grid" as const, label: "Grid", icon: <Icon name="grid" size={14} /> },
            ]}
            value={view}
            onChange={setView}
            ariaLabel="Layout mode"
          />
        </ShowcaseRegion>
      </Section>

      <Section title="ToggleRow" desc="Primitive row building block. Product filter panels and graph legends are showcased under Product Patterns.">
        <ShowcaseRegion regions={["row"]}>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>Default primitive</ShowcaseRegion>
            <ShowcaseRegion regions={["control-list"]}>
              <ToggleRow label="Primary option" colorToken="--accent" meta={4} />
              <ToggleRow label="Successful option" colorToken="--success" meta={12} />
              <ToggleRow label="Muted option" colorToken="--text-muted" on={false} meta={24} />
            </ShowcaseRegion>
          </ShowcaseRegion>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>Line primitive</ShowcaseRegion>
            <ShowcaseRegion regions={["control-list"]}>
              <ToggleRow line label="Primary line" colorToken="--accent" on />
              <ToggleRow line label="Success line" colorToken="--success" on />
              <ToggleRow line label="Muted line" colorToken="--text-muted" on={false} />
              <ToggleRow line label="Neutral line" colorToken="--border-default" on />
            </ShowcaseRegion>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

    </ShowcaseRegion>
  );
}
