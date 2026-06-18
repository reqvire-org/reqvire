import { Icon, ICON_NAMES, TokenSwatch, type DesignSystemColorToken } from "@ds";

type SwatchProps = { token: DesignSystemColorToken; label?: string };

function Swatch({ token, label }: SwatchProps) {
  return (
    <div className="showcase-swatch">
      <TokenSwatch colorToken={token} className="showcase-swatch__color" />
      <div className="showcase-swatch__name">{label ?? token}</div>
    </div>
  );
}

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

const ELEMENT_TYPES = [
  { name: "capability", fill: "--capability", tint: "--capability-tint", ink: "--capability-ink" },
  { name: "requirement", fill: "--requirement", tint: "--requirement-tint", ink: "--requirement-ink" },
  { name: "contract", fill: "--contract", tint: "--contract-tint", ink: "--contract-ink" },
  { name: "verification", fill: "--verification", tint: "--verification-tint", ink: "--verification-ink" },
  { name: "ontology", fill: "--ontology", tint: "--ontology-tint", ink: "--ontology-ink" },
  { name: "resource", fill: "--resource", tint: "--resource-tint", ink: "--resource-ink" },
  { name: "other", fill: "--other", tint: "--other-tint", ink: "--other-ink" },
] as const;

const SURFACE_TOKENS = [
  "--bg-canvas", "--bg-surface", "--bg-raised", "--bg-overlay",
  "--bg-sunken", "--bg-hover", "--bg-active", "--bg-selected",
] as const satisfies readonly DesignSystemColorToken[];

const TEXT_TOKENS = [
  "--text-strong", "--text-body", "--text-secondary", "--text-muted",
  "--text-faint", "--text-inverse", "--text-link", "--text-code",
] as const satisfies readonly DesignSystemColorToken[];

const BORDER_TOKENS = ["--border-subtle", "--border-default", "--border-strong", "--border-focus", "--border-selected"] as const satisfies readonly DesignSystemColorToken[];

const ACCENT_TOKENS = ["--accent", "--accent-hover", "--accent-active", "--accent-subtle"] as const satisfies readonly DesignSystemColorToken[];

const STATUS_TOKENS = [
  { token: "--success", label: "success" }, { token: "--success-tint", label: "success-tint" },
  { token: "--warning", label: "warning" }, { token: "--warning-tint", label: "warning-tint" },
  { token: "--danger", label: "danger" }, { token: "--danger-tint", label: "danger-tint" },
  { token: "--info", label: "info" }, { token: "--info-tint", label: "info-tint" },
] as const satisfies readonly { token: DesignSystemColorToken; label: string }[];

const EDGE_TOKENS = [
  "--edge-default", "--edge-derive", "--edge-satisfy", "--edge-trace", "--edge-attach",
] as const satisfies readonly DesignSystemColorToken[];

const RDF_TOKENS = [
  "--rdf-class", "--rdf-objprop", "--rdf-dtprop", "--rdf-rdfprop",
  "--rdf-individual", "--rdf-datatype", "--rdf-restriction", "--rdf-classexpr",
  "--rdf-nodeshape", "--rdf-propshape", "--rdf-resource",
] as const satisfies readonly DesignSystemColorToken[];

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
    <div className="showcase-page">
      <Section title="Element-type colors" desc="Six semantic hues encode the model vocabulary. Never repurpose for decoration.">
        {ELEMENT_TYPES.map((t) => (
          <div key={t.name}>
            <div className="showcase-label showcase-label--spaced">{t.name}</div>
            <div className="showcase-swatch-grid showcase-swatch-grid--fixed-3">
              <Swatch token={t.fill} label="fill" />
              <Swatch token={t.tint} label="tint" />
              <Swatch token={t.ink} label="ink" />
            </div>
          </div>
        ))}
      </Section>

      <Section title="Surfaces" desc="Semantic surface aliases — always use these, not raw ramp values.">
        <div className="showcase-swatch-grid">
          {SURFACE_TOKENS.map((t) => <Swatch key={t} token={t} />)}
        </div>
      </Section>

      <Section title="Text">
        <div className="showcase-swatch-grid">
          {TEXT_TOKENS.map((t) => (
            <div key={t} className="showcase-swatch">
              <TokenSwatch colorToken={t} className="showcase-swatch__color" />
              <div className="showcase-swatch__name">{t}</div>
            </div>
          ))}
        </div>
      </Section>

      <Section title="Borders &amp; accent">
        <div className="showcase-row">
          <div className="showcase-col">
            <div className="showcase-label">Borders</div>
            <div className="showcase-swatch-grid showcase-swatch-grid--fixed-5">
              {BORDER_TOKENS.map((t) => <Swatch key={t} token={t} />)}
            </div>
          </div>
          <div className="showcase-col">
            <div className="showcase-label">Accent (rose)</div>
            <div className="showcase-swatch-grid showcase-swatch-grid--fixed-4">
              {ACCENT_TOKENS.map((t) => <Swatch key={t} token={t} />)}
            </div>
          </div>
        </div>
      </Section>

      <Section title="Status">
        <div className="showcase-swatch-grid">
          {STATUS_TOKENS.map((t) => <Swatch key={t.token} token={t.token} label={t.label} />)}
        </div>
      </Section>

      <Section title="Graph edges &amp; RDF palette">
        <div className="showcase-col">
          <div className="showcase-label">Graph edges</div>
          <div className="showcase-swatch-grid showcase-swatch-grid--fixed-5">
            {EDGE_TOKENS.map((t) => <Swatch key={t} token={t} />)}
          </div>
        </div>
        <div className="showcase-col">
          <div className="showcase-label">RDF / ontology</div>
          <div className="showcase-swatch-grid">
            {RDF_TOKENS.map((t) => <Swatch key={t} token={t} />)}
          </div>
        </div>
      </Section>

      <Section title="Typography" desc="Geist (UI) + Geist Mono (code/IDs). Dense product: default UI text is 13px.">
        <div className="showcase-type-stack">
          {TYPE_SCALE.map(({ token, label, note }) => (
            <div key={token} className="showcase-type-row">
              <span className="showcase-type-row__label">{token}</span>
              <span className="showcase-type-sample" data-type-token={token}>
                {label}
              </span>
              {note && <span className="showcase-type-note">{note}</span>}
            </div>
          ))}
        </div>
        <div className="showcase-row showcase-row--spaced">
          <div className="showcase-col">
            <div className="showcase-label">Geist (sans)</div>
            <div className="showcase-font-sample showcase-font-sample--sans">
              The quick brown fox jumps over the lazy dog
            </div>
          </div>
          <div className="showcase-col">
            <div className="showcase-label">Geist Mono</div>
            <div className="showcase-font-sample showcase-font-sample--mono">
              requirements/Capabilities.md:3
            </div>
          </div>
        </div>
      </Section>

      <Section title="Spacing" desc="4px base grid.">
        <div className="showcase-space-stack">
          {SPACE_SCALE.map(({ token, px }) => (
            <div key={token} className="showcase-space-row">
              <span className="showcase-space-row__label">{token} — {px}px</span>
              <div className="showcase-space-row__bar" data-space-token={token} />
            </div>
          ))}
        </div>
      </Section>

      <Section title="Elevation" desc="Shadows are soft and low. Dark mode deepens all shadows.">
        <div className="showcase-shadow-grid">
          {SHADOWS.map(({ token, label }) => (
            <div key={token} className="showcase-shadow-card" data-shadow-token={token}>
              <span className="showcase-label">{label}</span>
              <span className="showcase-swatch__name">{token}</span>
            </div>
          ))}
        </div>
      </Section>

      <Section title="Iconography" desc={`${ICON_NAMES.length} Lucide-geometry icons. Use <Icon name="…" />.`}>
        <div className="showcase-icon-grid">
          {ICON_NAMES.map((name) => (
            <div key={name} className="showcase-icon-item">
              <Icon name={name} size={20} />
              <span className="showcase-icon-item__name">{name}</span>
            </div>
          ))}
        </div>
      </Section>
    </div>
  );
}
