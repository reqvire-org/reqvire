import { Icon, ICON_NAMES } from "@ds";

type SwatchProps = { token: string; label?: string };

function Swatch({ token, label }: SwatchProps) {
  return (
    <div className="sc-swatch">
      <div className="sc-swatch__color" style={{ background: `var(${token})` }} />
      <div className="sc-swatch__name">{label ?? token}</div>
    </div>
  );
}

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

const ELEMENT_TYPES = [
  { name: "capability", fill: "--capability", tint: "--capability-tint", ink: "--capability-ink" },
  { name: "requirement", fill: "--requirement", tint: "--requirement-tint", ink: "--requirement-ink" },
  { name: "refinement", fill: "--refinement", tint: "--refinement-tint", ink: "--refinement-ink" },
  { name: "verification", fill: "--verification", tint: "--verification-tint", ink: "--verification-ink" },
  { name: "ontology", fill: "--ontology", tint: "--ontology-tint", ink: "--ontology-ink" },
  { name: "resource", fill: "--resource", tint: "--resource-tint", ink: "--resource-ink" },
  { name: "other", fill: "--other", tint: "--other-tint", ink: "--other-ink" },
] as const;

const SURFACE_TOKENS = [
  "--bg-canvas", "--bg-surface", "--bg-raised", "--bg-overlay",
  "--bg-sunken", "--bg-hover", "--bg-active", "--bg-selected",
];

const TEXT_TOKENS = [
  "--text-strong", "--text-body", "--text-secondary", "--text-muted",
  "--text-faint", "--text-inverse", "--text-link", "--text-code",
];

const BORDER_TOKENS = ["--border-subtle", "--border-default", "--border-strong", "--border-focus", "--border-selected"];

const ACCENT_TOKENS = ["--accent", "--accent-hover", "--accent-active", "--accent-subtle"];

const STATUS_TOKENS = [
  { token: "--success", label: "success" }, { token: "--success-tint", label: "success-tint" },
  { token: "--warning", label: "warning" }, { token: "--warning-tint", label: "warning-tint" },
  { token: "--danger", label: "danger" }, { token: "--danger-tint", label: "danger-tint" },
  { token: "--info", label: "info" }, { token: "--info-tint", label: "info-tint" },
];

const EDGE_TOKENS = [
  "--edge-default", "--edge-derive", "--edge-satisfy", "--edge-trace", "--edge-attach",
];

const RDF_TOKENS = [
  "--rdf-class", "--rdf-objprop", "--rdf-dtprop", "--rdf-rdfprop",
  "--rdf-individual", "--rdf-datatype", "--rdf-restriction", "--rdf-classexpr",
  "--rdf-nodeshape", "--rdf-propshape", "--rdf-resource",
];

const TYPE_SCALE = [
  { token: "--text-micro", label: "micro — 11px", note: "eyebrow labels" },
  { token: "--text-caption", label: "caption — 12px", note: "meta, counts, tags" },
  { token: "--text-sm", label: "sm — 13px", note: "default UI text" },
  { token: "--text-base", label: "base — 14px", note: "body / reading copy" },
  { token: "--text-md", label: "md — 15px", note: "" },
  { token: "--text-lg", label: "lg — 17px", note: "card titles, headings" },
  { token: "--text-xl", label: "xl — 21px", note: "dialog titles" },
  { token: "--text-2xl", label: "2xl — 26px", note: "hero counts" },
  { token: "--text-3xl", label: "3xl — 33px", note: "display" },
];

const SPACE_SCALE = [
  { token: "--space-1", px: 2 }, { token: "--space-2", px: 4 },
  { token: "--space-3", px: 6 }, { token: "--space-4", px: 8 },
  { token: "--space-5", px: 10 }, { token: "--space-6", px: 12 },
  { token: "--space-7", px: 14 }, { token: "--space-8", px: 16 },
  { token: "--space-10", px: 20 }, { token: "--space-12", px: 24 },
  { token: "--space-14", px: 28 }, { token: "--space-16", px: 32 },
  { token: "--space-20", px: 40 }, { token: "--space-24", px: 48 },
  { token: "--space-32", px: 64 },
];

const SHADOWS = [
  { token: "--shadow-xs", label: "xs" }, { token: "--shadow-sm", label: "sm" },
  { token: "--shadow-md", label: "md" }, { token: "--shadow-lg", label: "lg" },
  { token: "--shadow-xl", label: "xl" },
];

export function TokensPage() {
  return (
    <div className="sc-page">
      <Section title="Element-type colors" desc="Six semantic hues encode the model vocabulary. Never repurpose for decoration.">
        {ELEMENT_TYPES.map((t) => (
          <div key={t.name}>
            <div className="sc-label" style={{ marginBottom: "var(--space-5)" }}>{t.name}</div>
            <div className="sc-swatch-grid" style={{ gridTemplateColumns: "repeat(3, 130px)" }}>
              <Swatch token={t.fill} label="fill" />
              <Swatch token={t.tint} label="tint" />
              <Swatch token={t.ink} label="ink" />
            </div>
          </div>
        ))}
      </Section>

      <Section title="Surfaces" desc="Semantic surface aliases — always use these, not raw ramp values.">
        <div className="sc-swatch-grid">
          {SURFACE_TOKENS.map((t) => <Swatch key={t} token={t} />)}
        </div>
      </Section>

      <Section title="Text">
        <div className="sc-swatch-grid">
          {TEXT_TOKENS.map((t) => (
            <div key={t} className="sc-swatch">
              <div className="sc-swatch__color" style={{ background: `var(${t})`, border: "var(--border-w) solid var(--border-subtle)" }} />
              <div className="sc-swatch__name">{t}</div>
            </div>
          ))}
        </div>
      </Section>

      <Section title="Borders &amp; accent">
        <div className="sc-row">
          <div className="sc-col">
            <div className="sc-label">Borders</div>
            <div className="sc-swatch-grid" style={{ gridTemplateColumns: "repeat(5, 130px)" }}>
              {BORDER_TOKENS.map((t) => <Swatch key={t} token={t} />)}
            </div>
          </div>
          <div className="sc-col">
            <div className="sc-label">Accent (rose)</div>
            <div className="sc-swatch-grid" style={{ gridTemplateColumns: "repeat(4, 130px)" }}>
              {ACCENT_TOKENS.map((t) => <Swatch key={t} token={t} />)}
            </div>
          </div>
        </div>
      </Section>

      <Section title="Status">
        <div className="sc-swatch-grid">
          {STATUS_TOKENS.map((t) => <Swatch key={t.token} token={t.token} label={t.label} />)}
        </div>
      </Section>

      <Section title="Graph edges &amp; RDF palette">
        <div className="sc-col">
          <div className="sc-label">Graph edges</div>
          <div className="sc-swatch-grid" style={{ gridTemplateColumns: "repeat(5, 130px)" }}>
            {EDGE_TOKENS.map((t) => <Swatch key={t} token={t} />)}
          </div>
        </div>
        <div className="sc-col">
          <div className="sc-label">RDF / ontology</div>
          <div className="sc-swatch-grid">
            {RDF_TOKENS.map((t) => <Swatch key={t} token={t} />)}
          </div>
        </div>
      </Section>

      <Section title="Typography" desc="Geist (UI) + Geist Mono (code/IDs). Dense product: default UI text is 13px.">
        <div style={{ display: "flex", flexDirection: "column", gap: 0 }}>
          {TYPE_SCALE.map(({ token, label, note }) => (
            <div key={token} className="sc-type-row">
              <span className="sc-type-row__label">{token}</span>
              <span style={{ fontSize: `var(${token})`, color: "var(--text-strong)", lineHeight: 1.3 }}>
                {label}
              </span>
              {note && <span style={{ fontSize: "var(--text-micro)", color: "var(--text-faint)" }}>{note}</span>}
            </div>
          ))}
        </div>
        <div className="sc-row" style={{ marginTop: "var(--space-8)" }}>
          <div className="sc-col">
            <div className="sc-label">Geist (sans)</div>
            <div style={{ fontFamily: "var(--font-sans)", fontSize: "var(--text-base)", color: "var(--text-strong)" }}>
              The quick brown fox jumps over the lazy dog
            </div>
          </div>
          <div className="sc-col">
            <div className="sc-label">Geist Mono</div>
            <div style={{ fontFamily: "var(--font-mono)", fontSize: "var(--text-base)", color: "var(--text-code)" }}>
              requirements/Capabilities.md:3
            </div>
          </div>
        </div>
      </Section>

      <Section title="Spacing" desc="4px base grid.">
        <div style={{ display: "flex", flexDirection: "column", gap: "var(--space-3)" }}>
          {SPACE_SCALE.map(({ token, px }) => (
            <div key={token} className="sc-space-row">
              <span className="sc-space-row__label">{token} — {px}px</span>
              <div className="sc-space-row__bar" style={{ width: `var(${token})` }} />
            </div>
          ))}
        </div>
      </Section>

      <Section title="Elevation" desc="Shadows are soft and low. Dark mode deepens all shadows.">
        <div className="sc-shadow-grid">
          {SHADOWS.map(({ token, label }) => (
            <div key={token} className="sc-shadow-card" style={{ boxShadow: `var(${token})` }}>
              <span className="sc-label">{label}</span>
              <span className="sc-swatch__name">{token}</span>
            </div>
          ))}
        </div>
      </Section>

      <Section title="Iconography" desc={`${ICON_NAMES.length} Lucide-geometry icons. Use <Icon name="…" />.`}>
        <div className="sc-icon-grid">
          {ICON_NAMES.map((name) => (
            <div key={name} className="sc-icon-item">
              <Icon name={name} size={20} />
              <span className="sc-icon-item__name">{name}</span>
            </div>
          ))}
        </div>
      </Section>
    </div>
  );
}
