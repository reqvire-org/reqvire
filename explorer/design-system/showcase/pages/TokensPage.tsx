import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { Icon, ICON_NAMES, TokenSwatch, type DesignSystemColorToken } from "@ds";

type SwatchProps = { token: DesignSystemColorToken; label?: string };

function Swatch({ token, label }: SwatchProps) {
  return (
    <ShowcaseRegion regions={["swatch"]}>
      <ShowcaseRegion regions={["swatch__color"]} asChild><TokenSwatch variant="sample" colorToken={token}  /></ShowcaseRegion>
      <ShowcaseRegion regions={["swatch__name"]}>{label ?? token}</ShowcaseRegion>
    </ShowcaseRegion>
  );
}

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

const ELEMENT_TYPES = [
  { name: "capability", fill: "--color-role-capability", tint: "--color-role-capability-tint", ink: "--color-role-capability-ink" },
  { name: "requirement", fill: "--color-role-requirement", tint: "--color-role-requirement-tint", ink: "--color-role-requirement-ink" },
  { name: "contract", fill: "--color-role-contract", tint: "--color-role-contract-tint", ink: "--color-role-contract-ink" },
  { name: "verification", fill: "--color-role-verification", tint: "--color-role-verification-tint", ink: "--color-role-verification-ink" },
  { name: "ontology", fill: "--color-role-ontology", tint: "--color-role-ontology-tint", ink: "--color-role-ontology-ink" },
  { name: "concept", fill: "--color-role-concept", tint: "--color-role-concept-tint", ink: "--color-role-concept-ink" },
  { name: "concept-scheme", fill: "--color-role-concept-scheme", tint: "--color-role-concept-scheme-tint", ink: "--color-role-concept-scheme-ink" },
  { name: "concept-reference", fill: "--color-role-concept-reference", tint: "--color-role-concept-reference-tint", ink: "--color-role-concept-reference-ink" },
  { name: "resource", fill: "--color-role-resource", tint: "--color-role-resource-tint", ink: "--color-role-resource-ink" },
  { name: "other", fill: "--color-role-other", tint: "--color-role-other-tint", ink: "--color-role-other-ink" },
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
  "--color-role-edge-default", "--color-role-edge-derive", "--color-role-edge-satisfy", "--color-role-edge-trace", "--color-role-edge-bind", "--color-role-concept-reference",
] as const satisfies readonly DesignSystemColorToken[];

const RDF_TOKENS = [
  "--color-role-rdf-class", "--color-role-rdf-concept", "--color-role-rdf-concept-scheme", "--color-role-rdf-objprop", "--color-role-rdf-dtprop", "--color-role-rdf-rdfprop",
  "--color-role-rdf-individual", "--color-role-rdf-datatype", "--color-role-rdf-restriction", "--color-role-rdf-classexpr",
  "--color-role-rdf-nodeshape", "--color-role-rdf-propshape", "--color-role-rdf-resource",
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
    <ShowcaseRegion regions={["page"]}>
      <Section title="Element-type colors" desc="Semantic hues encode the model vocabulary. Never repurpose for decoration.">
        {ELEMENT_TYPES.map((t) => (
          <div key={t.name}>
            <ShowcaseRegion regions={["label", "label--spaced"]}>{t.name}</ShowcaseRegion>
            <ShowcaseRegion regions={["swatch-grid", "swatch-grid--fixed-3"]}>
              <Swatch token={t.fill} label="fill" />
              <Swatch token={t.tint} label="tint" />
              <Swatch token={t.ink} label="ink" />
            </ShowcaseRegion>
          </div>
        ))}
      </Section>

      <Section title="Surfaces" desc="Semantic surface aliases — always use these, not raw ramp values.">
        <ShowcaseRegion regions={["swatch-grid"]}>
          {SURFACE_TOKENS.map((t) => <Swatch key={t} token={t} />)}
        </ShowcaseRegion>
      </Section>

      <Section title="Text">
        <ShowcaseRegion regions={["swatch-grid"]}>
          {TEXT_TOKENS.map((t) => (
            <ShowcaseRegion key={t} regions={["swatch"]}>
              <ShowcaseRegion regions={["swatch__color"]} asChild><TokenSwatch variant="sample" colorToken={t}  /></ShowcaseRegion>
              <ShowcaseRegion regions={["swatch__name"]}>{t}</ShowcaseRegion>
            </ShowcaseRegion>
          ))}
        </ShowcaseRegion>
      </Section>

      <Section title="Borders &amp; accent">
        <ShowcaseRegion regions={["row"]}>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>Borders</ShowcaseRegion>
            <ShowcaseRegion regions={["swatch-grid", "swatch-grid--fixed-5"]}>
              {BORDER_TOKENS.map((t) => <Swatch key={t} token={t} />)}
            </ShowcaseRegion>
          </ShowcaseRegion>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>Accent (rose)</ShowcaseRegion>
            <ShowcaseRegion regions={["swatch-grid", "swatch-grid--fixed-4"]}>
              {ACCENT_TOKENS.map((t) => <Swatch key={t} token={t} />)}
            </ShowcaseRegion>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="Status">
        <ShowcaseRegion regions={["swatch-grid"]}>
          {STATUS_TOKENS.map((t) => <Swatch key={t.token} token={t.token} label={t.label} />)}
        </ShowcaseRegion>
      </Section>

      <Section title="Graph edges &amp; RDF palette">
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Graph edges</ShowcaseRegion>
          <ShowcaseRegion regions={["swatch-grid", "swatch-grid--fixed-5"]}>
            {EDGE_TOKENS.map((t) => <Swatch key={t} token={t} />)}
          </ShowcaseRegion>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>RDF / ontology</ShowcaseRegion>
          <ShowcaseRegion regions={["swatch-grid"]}>
            {RDF_TOKENS.map((t) => <Swatch key={t} token={t} />)}
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="Typography" desc="Geist (UI) + Geist Mono (code/IDs). Dense product: default UI text is 13px.">
        <ShowcaseRegion regions={["type-stack"]}>
          {TYPE_SCALE.map(({ token, label, note }) => (
            <ShowcaseRegion key={token} regions={["type-row"]}>
              <ShowcaseRegion regions={["type-row__label"]} as="span">{token}</ShowcaseRegion>
              <ShowcaseRegion regions={["type-sample"]} as="span" data-type-token={token}>
                {label}
              </ShowcaseRegion>
              {note && <ShowcaseRegion regions={["type-note"]} as="span">{note}</ShowcaseRegion>}
            </ShowcaseRegion>
          ))}
        </ShowcaseRegion>
        <ShowcaseRegion regions={["row", "row--spaced"]}>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>Geist (sans)</ShowcaseRegion>
            <ShowcaseRegion regions={["font-sample", "font-sample--sans"]}>
              The quick brown fox jumps over the lazy dog
            </ShowcaseRegion>
          </ShowcaseRegion>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>Geist Mono</ShowcaseRegion>
            <ShowcaseRegion regions={["font-sample", "font-sample--mono"]}>
              system-model/Capabilities.md:3
            </ShowcaseRegion>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="Spacing" desc="4px base grid.">
        <ShowcaseRegion regions={["space-stack"]}>
          {SPACE_SCALE.map(({ token, px }) => (
            <ShowcaseRegion key={token} regions={["space-row"]}>
              <ShowcaseRegion regions={["space-row__label"]} as="span">{token} — {px}px</ShowcaseRegion>
              <ShowcaseRegion regions={["space-row__bar"]} data-space-token={token} />
            </ShowcaseRegion>
          ))}
        </ShowcaseRegion>
      </Section>

      <Section title="Elevation" desc="Shadows are soft and low. Dark mode deepens all shadows.">
        <ShowcaseRegion regions={["shadow-grid"]}>
          {SHADOWS.map(({ token, label }) => (
            <ShowcaseRegion key={token} regions={["shadow-card"]} data-shadow-token={token}>
              <ShowcaseRegion regions={["label"]} as="span">{label}</ShowcaseRegion>
              <ShowcaseRegion regions={["swatch__name"]} as="span">{token}</ShowcaseRegion>
            </ShowcaseRegion>
          ))}
        </ShowcaseRegion>
      </Section>

      <Section title="Iconography" desc={`${ICON_NAMES.length} Lucide-geometry icons. Use <Icon name="…" />.`}>
        <ShowcaseRegion regions={["icon-grid"]}>
          {ICON_NAMES.map((name) => (
            <ShowcaseRegion key={name} regions={["icon-item"]}>
              <Icon name={name} size={20} />
              <ShowcaseRegion regions={["icon-item__name"]} as="span">{name}</ShowcaseRegion>
            </ShowcaseRegion>
          ))}
        </ShowcaseRegion>
      </Section>
    </ShowcaseRegion>
  );
}
