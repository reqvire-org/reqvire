import { useState } from "react";
import { Icon, SearchInput, SegmentedControl, Tabs, ToggleRow } from "@ds";

function Section({ title, desc, children }: { title: string; desc?: string; children: React.ReactNode }) {
  return (
    <section className="sc-section">
      <div className="sc-section__heading">
        <div className="sc-section__title">{title}</div>
        {desc && <div className="sc-section__desc">{desc}</div>}
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
  const [tabUnderline, setTabUnderline] = useState("model");
  const [tabPill, setTabPill] = useState("grid");

  const [toggles, setToggles] = useState({
    capability: true, requirement: true, verification: true,
    refinement: false, ontology: true,
  });

  const toggle = (key: keyof typeof toggles) =>
    setToggles((s) => ({ ...s, [key]: !s[key] }));

  return (
    <div className="sc-page">

      <Section title="SearchInput" desc="Controlled text input with leading icon and optional clear.">
        <div className="sc-col">
          <div className="sc-label">Default (md)</div>
          <div style={{ width: 360 }}>
            <SearchInput
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search elements, files, relations…"
            />
          </div>
        </div>
        <div className="sc-col">
          <div className="sc-label">Large (lg)</div>
          <div style={{ width: 480 }}>
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
        <div className="sc-col">
          <div className="sc-label">Text only</div>
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
        <div className="sc-col">
          <div className="sc-label">With icons</div>
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

      <Section title="Tabs" desc="underline variant for top nav, pill variant for in-panel switching.">
        <div className="sc-col">
          <div className="sc-label">underline (top nav)</div>
          <Tabs
            items={[
              { value: "model", label: "Model", icon: <Icon name="folder" size={14} /> },
              { value: "graph", label: "Knowledge Graph", icon: <Icon name="network" size={14} /> },
              { value: "ontologies", label: "Ontologies", icon: <Icon name="globe" size={14} /> },
              { value: "search", label: "Search", icon: <Icon name="search" size={14} /> },
              { value: "traces", label: "Traces", icon: <Icon name="activity" size={14} />, badge: 145 },
            ]}
            value={tabUnderline}
            onChange={setTabUnderline}
          />
        </div>
        <div className="sc-col">
          <div className="sc-label">pill (in-panel)</div>
          <Tabs
            variant="pill"
            items={[
              { value: "grid", label: "Grid" },
              { value: "list", label: "List" },
              { value: "sunburst", label: "Sunburst" },
            ]}
            value={tabPill}
            onChange={setTabPill}
          />
        </div>
      </Section>

      <Section title="ToggleRow" desc="Block variant for filter panels. Line variant for graph overlay legends.">
        <div className="sc-row">
          <div className="sc-col">
            <div className="sc-label">Block (filter panel)</div>
            <div style={{ width: 240, display: "flex", flexDirection: "column", gap: "var(--space-3)" }}>
              <ToggleRow label="Capability" colorToken="--capability" on={toggles.capability} onToggle={() => toggle("capability")} meta={4} />
              <ToggleRow label="Requirement" colorToken="--requirement" on={toggles.requirement} onToggle={() => toggle("requirement")} meta={12} />
              <ToggleRow label="Verification" colorToken="--verification" on={toggles.verification} onToggle={() => toggle("verification")} meta={8} />
              <ToggleRow label="Refinement" colorToken="--refinement" on={toggles.refinement} onToggle={() => toggle("refinement")} meta={24} />
              <ToggleRow label="Ontology" colorToken="--ontology" on={toggles.ontology} onToggle={() => toggle("ontology")} meta={1} />
            </div>
          </div>
          <div className="sc-col">
            <div className="sc-label">Line (graph overlay legend)</div>
            <div style={{ display: "flex", flexDirection: "column", gap: "var(--space-2)" }}>
              <ToggleRow line label="derivedFrom" colorToken="--edge-derive" on />
              <ToggleRow line label="verifiedBy" colorToken="--edge-satisfy" on />
              <ToggleRow line label="attachedTo" colorToken="--edge-attach" on={false} />
              <ToggleRow line label="trace" colorToken="--edge-trace" on />
            </div>
          </div>
        </div>
      </Section>

    </div>
  );
}
