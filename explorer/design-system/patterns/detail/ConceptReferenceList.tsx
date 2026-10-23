import { cx } from "@linaria/atomic";
import { ElementIcon } from "../../components/data/ElementIcon";
import {
  conceptReferenceQualifierSkinX,
  conceptReferenceRowBaseUX,
  conceptReferenceRowSkinX,
  relationEndpointBaseUX,
  relationEndpointLabelUX,
  relationEndpointSkinX,
  relationStackUX,
} from "./detailStyles";
import { DetailSection } from "./DetailSection";
import type { DetailConceptReferenceItem, OpenConceptReferenceHandler } from "./types";

export interface ConceptReferenceListProps {
  title?: string;
  references: readonly DetailConceptReferenceItem[];
  onOpenConceptReference?: OpenConceptReferenceHandler;
}

export function ConceptReferenceList({
  title = "Concept references",
  references,
  onOpenConceptReference,
}: ConceptReferenceListProps) {
  if (references.length === 0) return null;

  return (
    <DetailSection title={title}>
      <div className={cx(relationStackUX)}>
        {references.map((reference) => {
          const displayLabel = reference.ontologyLabel ?? reference.label;
          const showQualifier = normalizedConceptLabel(displayLabel) !== normalizedConceptLabel(reference.label);
          return (
            <div key={reference.id} className={cx(conceptReferenceRowBaseUX, conceptReferenceRowSkinX)}>
              {reference.ontologyNodeId && onOpenConceptReference ? (
                <button
                  type="button"
                  className={cx(relationEndpointBaseUX, relationEndpointSkinX)}
                  title={reference.iri}
                  onClick={() => onOpenConceptReference(reference)}
                >
                  <ElementIcon type="concept" family="ontology" title="concept" size="sm" />
                  <span className={cx(relationEndpointLabelUX)}>{displayLabel}</span>
                </button>
              ) : (
                <span className={cx(relationEndpointBaseUX, relationEndpointSkinX)} title={reference.iri}>
                  <ElementIcon type="concept-reference" family="resource" title="concept reference" size="sm" />
                  <span className={cx(relationEndpointLabelUX)}>{displayLabel}</span>
                </span>
              )}
              {showQualifier ? <span className={cx(conceptReferenceQualifierSkinX)}>({reference.label})</span> : null}
            </div>
          );
        })}
      </div>
    </DetailSection>
  );
}

function normalizedConceptLabel(value: string | null | undefined) {
  return String(value ?? "").toLowerCase().replace(/[^a-z0-9]/g, "");
}
