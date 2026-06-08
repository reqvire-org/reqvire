import { useState } from "react";
import { Icon } from "@ds";
import { routeForView, type ViewId } from "../../../src/router/routes";
import { MockShell } from "../mocks/MockShell";

const MOCKS: { id: ViewId; label: string; desc: string; icon: string }[] = [
  { id: "model",      label: "Model View",      desc: "File tree + element grid. The primary workspace.",         icon: "folder"   },
  { id: "traces",     label: "Traces View",     desc: "Coverage Sankey — requirements → verifications.",         icon: "activity" },
  { id: "ontologies", label: "Ontologies View", desc: "RDF/SHACL class hierarchy and node inspector.",            icon: "globe"    },
  { id: "search",     label: "Search View",     desc: "Full-text element search with live filtering.",            icon: "search"   },
  { id: "coverage",   label: "Coverage View",   desc: "Coverage KPIs, evidence bars, capability rollups, and gaps.", icon: "pie-chart" },
];

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

export function MocksPage() {
  const [selected, setSelected] = useState<ViewId | null>(null);

  if (selected) return <MockShell />;

  return (
    <div className="sc-page">
      <Section
        title="Explorer Mocks"
        desc="Each card launches the real App with devFixture data, pre-navigated to that view. Use the Mocks tab above to return."
      >
        <div className="sc-mocks-grid">
          {MOCKS.map((m) => (
            <button
              key={m.id}
              className="sc-mock-card"
              onClick={() => {
                window.location.hash = routeForView(m.id);
                setSelected(m.id);
              }}
            >
              <Icon name={m.icon as Parameters<typeof Icon>[0]["name"]} size={20} style={{ color: "var(--text-secondary)" }} />
              <div className="sc-mock-card__label">{m.label}</div>
              <div className="sc-mock-card__desc">{m.desc}</div>
            </button>
          ))}
        </div>
      </Section>
    </div>
  );
}
