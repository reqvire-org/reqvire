import { useState } from "react";
import { Icon, SearchInput, SegmentedControl, ToggleRow } from "@ds";

function Section({ title, desc, children }: { title: string; desc?: string; children: React.ReactNode }) {
  return (
    <section className="showcase-section">
      <div className="showcase-section__heading">
        <div className="showcase-section__title">{title}</div>
        {desc && <div className="showcase-section__desc">{desc}</div>}
      </div>
      {children}
    </section>
  );
}

export function ControlsPage() {
  const [query, setQuery] = useState("");
  const [queryLg, setQueryLg] = useState("");
  const [view, setView] = useState<"list" | "grid">("list");
  const [mode, setMode] = useState<"model" | "graph" | "traces">("model");

  return (
    <div className="showcase-page">

      <Section title="SearchInput" desc="Controlled text input with leading icon and optional clear.">
        <div className="showcase-col">
          <div className="showcase-label">Default (md)</div>
          <div className="showcase-input-demo showcase-input-demo--md">
            <SearchInput
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search elements, files, relations…"
            />
          </div>
        </div>
        <div className="showcase-col">
          <div className="showcase-label">Large (lg)</div>
          <div className="showcase-input-demo showcase-input-demo--lg">
            <SearchInput
              size="lg"
              value={queryLg}
              onChange={(e) => setQueryLg(e.target.value)}
              placeholder="Search the project store…"
            />
          </div>
        </div>
      </Section>

      <Section title="SegmentedControl" desc="Mutually exclusive button group. Active segment inverts to dark ink fill.">
        <div className="showcase-col">
          <div className="showcase-label">Text only</div>
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
        </div>
        <div className="showcase-col">
          <div className="showcase-label">With icons</div>
          <SegmentedControl
            items={[
              { value: "list" as const, label: "List", icon: <Icon name="list" size={14} /> },
              { value: "grid" as const, label: "Grid", icon: <Icon name="grid" size={14} /> },
            ]}
            value={view}
            onChange={setView}
            ariaLabel="Layout mode"
          />
        </div>
      </Section>

      <Section title="ToggleRow" desc="Primitive row building block. Product filter panels and graph legends are showcased under Product Patterns.">
        <div className="showcase-row">
          <div className="showcase-col">
            <div className="showcase-label">Default primitive</div>
            <div className="showcase-control-list">
              <ToggleRow label="Primary option" colorToken="--accent" meta={4} />
              <ToggleRow label="Successful option" colorToken="--success" meta={12} />
              <ToggleRow label="Muted option" colorToken="--text-muted" on={false} meta={24} />
            </div>
          </div>
          <div className="showcase-col">
            <div className="showcase-label">Line primitive</div>
            <div className="showcase-control-list">
              <ToggleRow line label="Primary line" colorToken="--accent" on />
              <ToggleRow line label="Success line" colorToken="--success" on />
              <ToggleRow line label="Muted line" colorToken="--text-muted" on={false} />
              <ToggleRow line label="Neutral line" colorToken="--border-default" on />
            </div>
          </div>
        </div>
      </Section>

    </div>
  );
}
