import type { CSSProperties, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import {
  Button,
  ElementIcon,
  Icon,
  IconButton,
  Modal,
  ModalBody,
  ModalClose,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalTitle,
  TypeBadge,
} from "@ds";

const helpModalBaseUX = css`
  --ex-help-dialog-w: 1120px;
  width: min(var(--ex-help-dialog-w), calc(100vw - var(--space-24)));
  max-width: min(var(--ex-help-dialog-w), calc(100vw - var(--space-24)));
`;

const helpHeaderUX = css`
  align-items: center;

  h2 {
    flex: 1 1 auto;
  }
`;

const helpBodyUX = css`
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
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(18rem, 1fr));
  gap: var(--space-14);
`;

const helpLegendListUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  margin-top: var(--space-5);
`;

const helpLegendRowUX = css`
  display: flex;
  min-height: 2rem;
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

const helpColorSwatchUX = css`
  width: var(--space-6);
  height: var(--space-6);
  border-radius: var(--radius-xs);
  background: var(--help-color);
  box-shadow: inset 0 0 0 var(--border-w) color-mix(in srgb, var(--help-color) 70%, var(--text-strong));
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

const footerSpacerUX = css`
  flex: 1 1 auto;
`;

const ELEMENT_LEGEND = [
  ["capability", "Capability"],
  ["requirement", "Requirement"],
  ["behavior", "Behavior"],
  ["constraint", "Constraint"],
  ["test-verification", "Test Verification"],
  ["analysis-verification", "Analysis Verification"],
  ["specification", "Specification"],
  ["semantic-contract", "Semantic Contract"],
  ["semantic-query-contract", "Semantic Query Contract"],
  ["ontology", "Ontology"],
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

export function HelpModal({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  return (
    <Modal open={open} onOpenChange={onOpenChange}>
      <ModalContent className={cx(helpModalBaseUX)} aria-label="Help">
        <ModalHeader className={cx(helpHeaderUX)}>
          <ModalTitle>Help</ModalTitle>
          <ModalClose asChild>
            <IconButton tone="ghost" aria-label="Close help">
              <Icon name="x" />
            </IconButton>
          </ModalClose>
        </ModalHeader>
        <ModalBody className={cx(helpBodyUX)}>
          <section className={cx(helpSectionUX)}>
            <h3>Model Legend</h3>
            <div className={cx(helpLegendGridUX)}>
              <div>
                <h4>Result kinds</h4>
                <div className={cx(helpLegendListUX)}>
                  {RESULT_LEGEND.map(([kind, label, token]) => (
                    <LegendRow
                      key={kind}
                      marker={<span className={cx(helpColorSwatchUX)} style={legendColor(token)} />}
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
                      label={<TypeBadge type={badgeTypeForLegend(type)} tinted>{label}</TypeBadge>}
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
                      marker={<span className={cx(helpColorSwatchUX)} style={legendColor(token)} />}
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
        </ModalBody>
        <ModalFooter>
          <span className={cx(footerSpacerUX)} />
          <ModalClose asChild>
            <Button tone="primary">Close</Button>
          </ModalClose>
        </ModalFooter>
      </ModalContent>
    </Modal>
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
  if (type === "concept-reference" || type === "evidence-file") return "resource";
  return type;
}

function badgeTypeForLegend(type: (typeof ELEMENT_LEGEND)[number][0]) {
  if (type === "concept-reference" || type === "evidence-file") return "resource";
  return type;
}

function legendColor(token: string): CSSProperties {
  return { "--help-color": `var(${token})` } as CSSProperties;
}
