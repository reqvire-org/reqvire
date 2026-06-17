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
  references: DetailConceptReferenceItem[];
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
        {references.map((reference) => (
          <div key={reference.id} className={cx(conceptReferenceRowBaseUX, conceptReferenceRowSkinX)}>
            {reference.ontologyNodeId && onOpenConceptReference ? (
              <button
                type="button"
                className={cx(relationEndpointBaseUX, relationEndpointSkinX)}
                title={reference.iri}
                onClick={() => onOpenConceptReference(reference)}
              >
                <ElementIcon type="ontology" family="ontology" title="ontology" size="sm" />
                <span className={cx(relationEndpointLabelUX)}>{reference.ontologyLabel ?? reference.label}</span>
              </button>
            ) : (
              <span className={cx(relationEndpointBaseUX, relationEndpointSkinX)} title={reference.iri}>
                <ElementIcon type="resource" family="resource" title="concept reference" size="sm" />
                <span className={cx(relationEndpointLabelUX)}>{reference.ontologyLabel ?? reference.label}</span>
              </span>
            )}
            <span className={cx(conceptReferenceQualifierSkinX)}>({reference.label})</span>
          </div>
        ))}
      </div>
    </DetailSection>
  );
}
