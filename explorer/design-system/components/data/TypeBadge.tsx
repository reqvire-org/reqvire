import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { ELEMENT_TYPES, elementRole, type ElementType } from "../../palette";

const baseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  height: var(--ds-typebadge-h, calc(var(--space-8) + var(--space-2)));
  padding: 0 var(--space-5);
  border-radius: var(--radius-sm);
  font-size: var(--text-micro);
  font-weight: var(--weight-medium);
  line-height: 1;
  white-space: nowrap;

  .ds-typebadge__dot {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex: none;
    width: var(--ds-typebadge-dot-size, calc(var(--space-3) + var(--space-1) / 2));
    height: var(--ds-typebadge-dot-size, calc(var(--space-3) + var(--space-1) / 2));
    border-radius: var(--ds-typebadge-dot-radius, calc(var(--radius-xs) / 2));
    font-family: var(--font-mono);
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    letter-spacing: 0;
    line-height: 1;
  }

  .ds-typebadge__dot--diamond {
    transform: rotate(45deg) scale(0.8);
  }

  .ds-typebadge__dot--glyph {
    --ds-typebadge-dot-size: var(--type-icon-sm);
    color: var(--text-strong);
  }

  .ds-typebadge__dot--wide-glyph {
    font-size: calc(var(--text-micro) - var(--space-1));
  }

  .ds-typebadge__glyph {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .ds-typebadge__dot--diamond .ds-typebadge__glyph {
    transform: rotate(-45deg);
  }
`;

const skinX = css`
  color: var(--text-secondary);
  background: var(--bg-sunken);

  .ds-typebadge__dot {
    background: var(--ds-typebadge-color);
  }

  &.ds-typebadge--tinted {
    color: var(--ds-typebadge-ink);
    background: var(--ds-typebadge-tint);
  }
`;

const roleSkinX = css`
  &[data-element-role="capability"] {
    --ds-typebadge-color: var(--capability);
    --ds-typebadge-tint: var(--capability-tint);
    --ds-typebadge-ink: var(--capability-ink);
  }

  &[data-element-role="requirement"] {
    --ds-typebadge-color: var(--requirement);
    --ds-typebadge-tint: var(--requirement-tint);
    --ds-typebadge-ink: var(--requirement-ink);
  }

  &[data-element-role="contract"],
  &[data-element-role="source"],
  &[data-element-role="constraint"],
  &[data-element-role="behavior"],
  &[data-element-role="state"],
  &[data-element-role="input-output"],
  &[data-element-role="specification"] {
    --ds-typebadge-color: var(--contract);
    --ds-typebadge-tint: var(--contract-tint);
    --ds-typebadge-ink: var(--contract-ink);
  }

  &[data-element-role="semantic-contract"] {
    --ds-typebadge-color: var(--semantic-contract);
    --ds-typebadge-tint: var(--semantic-contract-tint);
    --ds-typebadge-ink: var(--semantic-contract-ink);
  }

  &[data-element-role="verification-objective"] {
    --ds-typebadge-color: var(--verification-objective);
    --ds-typebadge-tint: var(--verification-objective-tint);
    --ds-typebadge-ink: var(--verification-objective-ink);
  }

  &[data-element-role="verification"] {
    --ds-typebadge-color: var(--verification);
    --ds-typebadge-tint: var(--verification-tint);
    --ds-typebadge-ink: var(--verification-ink);
  }

  &[data-element-role="ontology"] {
    --ds-typebadge-color: var(--ontology);
    --ds-typebadge-tint: var(--ontology-tint);
    --ds-typebadge-ink: var(--ontology-ink);
  }

  &[data-element-role="concept"] {
    --ds-typebadge-color: var(--concept);
    --ds-typebadge-tint: var(--concept-tint);
    --ds-typebadge-ink: var(--concept-ink);
  }

  &[data-element-role="concept-scheme"] {
    --ds-typebadge-color: var(--concept-scheme);
    --ds-typebadge-tint: var(--concept-scheme-tint);
    --ds-typebadge-ink: var(--concept-scheme-ink);
  }

  &[data-element-role="concept-reference"] {
    --ds-typebadge-color: var(--concept-reference);
    --ds-typebadge-tint: var(--concept-reference-tint);
    --ds-typebadge-ink: var(--concept-reference-ink);
  }

  &[data-element-role="resource"] {
    --ds-typebadge-color: var(--resource);
    --ds-typebadge-tint: var(--resource-tint);
    --ds-typebadge-ink: var(--resource-ink);
  }

  &[data-element-role="other"] {
    --ds-typebadge-color: var(--other);
    --ds-typebadge-tint: var(--other-tint);
    --ds-typebadge-ink: var(--other-ink);
  }
`;

export interface TypeBadgeProps extends Omit<HTMLAttributes<HTMLSpanElement>, "style"> {
  type?: string | null;
  family?: string | null;
  children?: ReactNode;
  dot?: boolean;
  tinted?: boolean;
}

export function TypeBadge({
  type,
  family,
  children,
  dot = true,
  tinted = false,
  className = "",
  ...props
}: TypeBadgeProps) {
  const role = elementRole(type, family);
  const normalizedType = (type ?? "").toLowerCase();
  const explicitType = normalizedType in ELEMENT_TYPES ? ELEMENT_TYPES[normalizedType as ElementType] : null;
  const markerShape = explicitType?.shape ?? "square";
  const markerGlyph = explicitType?.glyph ?? null;
  const isWideGlyph = Boolean(markerGlyph && [...markerGlyph].length > 1);

  return (
    <span
      className={cx("ds-typebadge", baseUX, skinX, roleSkinX, tinted ? "ds-typebadge--tinted" : undefined, className)}
      data-element-role={role}
      {...props}
    >
      {dot ? (
        <span
          className={cx(
            "ds-typebadge__dot",
            markerShape === "diamond" ? "ds-typebadge__dot--diamond" : undefined,
            markerGlyph ? "ds-typebadge__dot--glyph" : undefined,
            isWideGlyph ? "ds-typebadge__dot--wide-glyph" : undefined,
          )}
        >
          {markerGlyph ? <span className="ds-typebadge__glyph">{markerGlyph}</span> : null}
        </span>
      ) : null}
      {children ?? type}
    </span>
  );
}
