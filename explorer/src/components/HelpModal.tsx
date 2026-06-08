import type { CSSProperties, ReactNode } from "react";
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
      <ModalContent className="help-modal" aria-label="Help">
        <ModalHeader className="help-modal-header">
          <ModalTitle>Help</ModalTitle>
          <ModalClose asChild>
            <IconButton tone="ghost" aria-label="Close help">
              <Icon name="x" />
            </IconButton>
          </ModalClose>
        </ModalHeader>
        <ModalBody className="help-modal-body">
          <section className="help-modal-section">
            <h3>Model Legend</h3>
            <div className="help-legend-grid">
              <div>
                <h4>Result kinds</h4>
                <div className="help-legend-list">
                  {RESULT_LEGEND.map(([kind, label, token]) => (
                    <LegendRow
                      key={kind}
                      marker={<span className="help-color-swatch" style={legendColor(token)} />}
                      label={label}
                    />
                  ))}
                </div>
              </div>

              <div>
                <h4>Element types</h4>
                <div className="help-legend-list">
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

          <section className="help-modal-section">
            <h3>Ontology Legend</h3>
            <div className="help-legend-grid">
              <div>
                <h4>Node types</h4>
                <div className="help-legend-list">
                  {ONTOLOGY_TYPE_LEGEND.map(([token, label]) => (
                    <LegendRow
                      key={label}
                      marker={<span className="help-color-swatch" style={legendColor(token)} />}
                      label={label}
                    />
                  ))}
                </div>
              </div>
              <div>
                <h4>Notation</h4>
                <div className="help-legend-list">
                  {ONTOLOGY_NOTATION_LEGEND.map(([glyph, label]) => (
                    <LegendRow
                      key={label}
                      marker={<span className="help-notation-glyph">{glyph}</span>}
                      label={label}
                    />
                  ))}
                </div>
              </div>
            </div>
          </section>
        </ModalBody>
        <ModalFooter>
          <span className="ex-spacer" />
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
    <div className="help-legend-row">
      <span className="help-legend-marker">{marker}</span>
      <span className="help-legend-label">{label}</span>
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
