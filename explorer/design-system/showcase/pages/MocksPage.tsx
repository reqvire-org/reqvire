import { useState } from "react";
import { Icon } from "@ds";
import { FULL_APP_MOCKS, type ShowcaseMockViewId } from "../fixtures/productPatterns";
import { MockShell } from "../MockShell";

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

export function MocksPage() {
  const [selected, setSelected] = useState<ShowcaseMockViewId | null>(null);

  if (selected) return <MockShell />;

  return (
    <div className="showcase-page">
      <Section
        title="Explorer Mocks"
        desc="Each card launches the real App with devFixture data, pre-navigated to that view. Use the Mocks tab above to return."
      >
        <div className="showcase-mocks-grid">
          {FULL_APP_MOCKS.map((m) => (
            <button
              key={m.id}
              className="showcase-mock-card"
              onClick={() => {
                window.location.hash = m.hash;
                setSelected(m.id);
              }}
            >
              <Icon name={m.icon} size={20} className="showcase-mock-card__icon" />
              <div className="showcase-mock-card__label">{m.label}</div>
              <div className="showcase-mock-card__desc">{m.desc}</div>
            </button>
          ))}
        </div>
      </Section>
    </div>
  );
}
