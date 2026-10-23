import { cx } from "@linaria/atomic";
import {
  inlineConceptReferenceBaseUX,
  inlineConceptReferenceSkinX,
} from "./detailStyles";
import type { DetailConceptReferenceItem, OpenConceptReferenceHandler } from "./types";

export interface InlineConceptReferenceProps {
  reference: DetailConceptReferenceItem;
  label?: string;
  onOpenConceptReference?: OpenConceptReferenceHandler;
}

export function InlineConceptReference({
  reference,
  label,
  onOpenConceptReference,
}: InlineConceptReferenceProps) {
  const displayLabel = label ?? reference.ontologyLabel ?? reference.label;

  return (
    <button
      type="button"
      className={cx(inlineConceptReferenceBaseUX, inlineConceptReferenceSkinX)}
      title={reference.iri}
      onClick={() => onOpenConceptReference?.(reference)}
    >
      <span>{displayLabel}</span>
    </button>
  );
}
