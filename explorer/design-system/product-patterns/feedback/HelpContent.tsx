import type { ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { ElementIcon } from "../../components/data/ElementIcon";
import { TokenSwatch } from "../../components/data/TokenVisual";
import { TypeBadge } from "../../components/data/TypeBadge";

const helpContentUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-16);
`;

const helpSectionUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-8);

  h3 {
    margin: 0;
    color: var(--text-strong);
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
  }

  h4 {
    margin: 0;
    color: var(--text-muted);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    text-transform: uppercase;
  }
`;

const helpLegendGridUX = css`
  --ux-help-legend-column-min: 18rem;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(var(--ux-help-legend-column-min), 1fr));
  gap: var(--space-14);
`;

const helpLegendListUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--stack-gap-compact);
  margin-top: var(--space-5);
`;

const helpLegendRowUX = css`
  --ux-help-legend-row-min-h: 2rem;
  display: flex;
  min-height: var(--ux-help-legend-row-min-h);
  align-items: center;
  gap: var(--space-6);
  border-radius: var(--radius-sm);
  padding: var(--space-2) var(--space-4);
`;

const helpLegendMarkerUX = css`
  display: inline-flex;
  width: var(--space-10);
  align-items: center;
  justify-content: center;
  flex: none;
`;

const helpLegendLabelSkinX = css`
  min-width: 0;
  color: var(--text-body);
  font-size: var(--text-sm);
`;

const helpNotationGlyphUX = css`
  display: inline-flex;
  min-width: var(--space-10);
  align-items: center;
  justify-content: center;
  color: var(--accent);
  font-family: var(--font-mono);
  font-size: var(--text-caption);
  font-weight: var(--weight-semibold);
`;

const ELEMENT_LEGEND = [
  ["capability", "Capability"],
  ["requirement", "Requirement"],
  ["behavior", "Behavior"],
  ["constraint", "Constraint"],
  ["verification-objective", "Verification Objective"],
  ["test-verification", "Test Verification"],
  ["formal-proof-verification", "Formal Proof Verification"],
  ["analysis-verification", "Analysis Verification"],
  ["inspection-verification", "Inspection Verification"],
  ["demonstration-verification", "Demonstration Verification"],
  ["specification", "Specification"],
  ["semantic-contract", "Semantic Contract"],
  ["ontology", "Ontology"],
  ["concept-scheme", "Concept Scheme"],
  ["concept", "Concept"],
  ["concept-reference", "Concept Reference"],
  ["evidence-file", "Evidence File"],
] as const;

const RESULT_LEGEND = [
  ["file", "Files", "--resource"],
  ["element", "Elements", "--requirement"],
  ["resource", "Resources", "--ontology"],
  ["ontology", "Ontology terms", "--rdf-resource"],
] as const;

const ONTOLOGY_TYPE_LEGEND = [
  ["--rdf-class", "Class"],
  ["--rdf-concept", "SKOS concept"],
  ["--rdf-concept-scheme", "SKOS concept scheme"],
  ["--rdf-objprop", "Object property"],
  ["--rdf-dtprop", "Datatype property"],
  ["--rdf-individual", "Individual"],
  ["--rdf-datatype", "Datatype"],
  ["--rdf-restriction", "Restriction"],
  ["--rdf-classexpr", "Class expression"],
  ["--rdf-nodeshape", "Node shape"],
  ["--rdf-propshape", "Property shape"],
  ["--rdf-resource", "Resource"],
] as const;

const ONTOLOGY_NOTATION_LEGEND = [
  ["D/R", "Domain/range"],
  ["⊆", "Subclass"],
  ["∈", "Membership"],
  ["⟂", "Disjoint"],
  ["⇔", "Equivalence"],
  ["⟲", "Inverse"],
  ["∘", "Property chain"],
  ["∩", "Class expression"],
  ["SH", "SHACL overlay"],
] as const;

export function HelpContent() {
  return (
    <div className={cx(helpContentUX)} data-product-pattern="help-content">
      <section className={cx(helpSectionUX)}>
        <h3>Model Legend</h3>
        <div className={cx(helpLegendGridUX)}>
          <div>
            <h4>Result kinds</h4>
            <div className={cx(helpLegendListUX)}>
              {RESULT_LEGEND.map(([kind, label, token]) => (
                <LegendRow
                  key={kind}
                  marker={<TokenSwatch colorToken={token} />}
                  label={label}
                />
              ))}
            </div>
          </div>

          <div>
            <h4>Element types</h4>
            <div className={cx(helpLegendListUX)}>
              {ELEMENT_LEGEND.map(([type, label]) => (
                <LegendRow
                  key={type}
                  marker={<ElementIcon type={iconTypeForLegend(type)} size="sm" />}
                  label={
                    <TypeBadge type={badgeTypeForLegend(type)} tinted>
                      {label}
                    </TypeBadge>
                  }
                />
              ))}
            </div>
          </div>
        </div>
      </section>

      <section className={cx(helpSectionUX)}>
        <h3>Ontology Legend</h3>
        <div className={cx(helpLegendGridUX)}>
          <div>
            <h4>Node types</h4>
            <div className={cx(helpLegendListUX)}>
              {ONTOLOGY_TYPE_LEGEND.map(([token, label]) => (
                <LegendRow
                  key={label}
                  marker={<TokenSwatch colorToken={token} />}
                  label={label}
                />
              ))}
            </div>
          </div>
          <div>
            <h4>Notation</h4>
            <div className={cx(helpLegendListUX)}>
              {ONTOLOGY_NOTATION_LEGEND.map(([glyph, label]) => (
                <LegendRow
                  key={label}
                  marker={<span className={cx(helpNotationGlyphUX)}>{glyph}</span>}
                  label={label}
                />
              ))}
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}

function LegendRow({
  marker,
  label,
}: {
  marker: ReactNode;
  label: ReactNode;
}) {
  return (
    <div className={cx(helpLegendRowUX)}>
      <span className={cx(helpLegendMarkerUX)}>{marker}</span>
      <span className={cx(helpLegendLabelSkinX)}>{label}</span>
    </div>
  );
}

function iconTypeForLegend(type: (typeof ELEMENT_LEGEND)[number][0]) {
  if (type === "evidence-file") return "other";
  return type;
}

function badgeTypeForLegend(type: (typeof ELEMENT_LEGEND)[number][0]) {
  if (type === "evidence-file") return "other";
  return type;
}
