import type { HTMLAttributes } from "react";
import { css, cx } from "@linaria/atomic";
import { ELEMENT_TYPES, elementRole, type ElementIconShape, type ElementType } from "../../palette";

export type { ElementIconShape, ElementRole, ElementType } from "../../palette";
export type ElementIconSize = "sm" | "md" | "lg";

const baseUX = css`
  --ds-elemicon-size: var(--type-icon-md);
  --ds-elemicon-glyph-fs: calc(var(--ds-elemicon-size) * 0.46);
  --ds-elemicon-wide-glyph-fs: calc(var(--ds-elemicon-size) * 0.36);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex: none;
  width: var(--ds-elemicon-size);
  height: var(--ds-elemicon-size);
  border-radius: var(--radius-xs);

  svg {
    width: 60%;
    height: 60%;
  }

  .ds-elemicon__glyph {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    font-family: var(--font-mono);
    font-size: var(--ds-elemicon-glyph-fs);
    font-weight: var(--weight-bold);
    letter-spacing: 0;
    line-height: 1;
  }

  &.ds-elemicon--sm {
    --ds-elemicon-size: var(--type-icon-sm);
    border-radius: var(--ds-elemicon-sm-radius, var(--radius-xs));
  }

  &.ds-elemicon--lg {
    --ds-elemicon-size: var(--type-icon-lg);
    border-radius: var(--radius-sm);
  }

  &.ds-elemicon--diamond {
    --ds-elemicon-glyph-fs: calc(var(--ds-elemicon-size) * 0.72);
    --ds-elemicon-wide-glyph-fs: calc(var(--ds-elemicon-size) * 0.54);
    border-radius: var(--ds-elemicon-diamond-radius, var(--radius-xs));
    transform: rotate(45deg) scale(0.74);
  }

  &.ds-elemicon--diamond svg,
  &.ds-elemicon--diamond .ds-elemicon__glyph {
    transform: rotate(-45deg);
  }

  &.ds-elemicon--wide-glyph .ds-elemicon__glyph {
    font-size: var(--ds-elemicon-wide-glyph-fs);
  }

  &.ds-elemicon--hub .ds-elemicon__pip {
    width: 36%;
    height: 36%;
    border-radius: 50%;
  }
`;

const skinX = css`
  color: var(--slate-0);
  background: var(--ds-elemicon-color, var(--other));
  box-shadow: inset 0 0 0 var(--border-w) var(--element-icon-ring);

  .ds-elemicon__glyph {
    color: var(--text-strong);
  }

  &.ds-elemicon--hub {
    background: var(--slate-900);
    box-shadow: inset 0 0 0 var(--border-w) var(--element-icon-ring-inverse);
  }

  &.ds-elemicon--hub .ds-elemicon__pip {
    background: var(--ds-elemicon-color, var(--capability));
  }
`;

const roleSkinX = css`
  &[data-element-role="capability"] { --ds-elemicon-color: var(--capability); }
  &[data-element-role="requirement"] { --ds-elemicon-color: var(--requirement); }
  &[data-element-role="refinement"],
  &[data-element-role="source"],
  &[data-element-role="constraint"],
  &[data-element-role="behavior"],
  &[data-element-role="state"],
  &[data-element-role="input-output"],
  &[data-element-role="specification"] { --ds-elemicon-color: var(--refinement); }
  &[data-element-role="semantic-contract"] { --ds-elemicon-color: var(--semantic-contract); }
  &[data-element-role="verification-objective"] { --ds-elemicon-color: var(--verification-objective); }
  &[data-element-role="verification"] { --ds-elemicon-color: var(--verification); }
  &[data-element-role="ontology"] { --ds-elemicon-color: var(--ontology); }
  &[data-element-role="resource"] { --ds-elemicon-color: var(--resource); }
  &[data-element-role="other"] { --ds-elemicon-color: var(--other); }
`;

const DIAMOND_TYPES = new Set([
  "source",
  "specification",
  "constraint",
  "behavior",
  "state",
  "input-output",
]);

export interface ElementIconProps extends Omit<HTMLAttributes<HTMLSpanElement>, "style"> {
  type?: string | null;
  family?: string | null;
  size?: ElementIconSize;
  shape?: ElementIconShape;
  glyph?: string | null;
}

export function ElementIcon({
  type,
  family,
  size = "md",
  className = "",
  title,
  shape,
  glyph,
  ...props
}: ElementIconProps) {
  const role = elementRole(type, family);
  const normalizedType = (type ?? "").toLowerCase();
  const explicitType = normalizedType in ELEMENT_TYPES ? ELEMENT_TYPES[normalizedType as ElementType] : null;
  const resolvedShape = shape ?? explicitType?.shape ?? (DIAMOND_TYPES.has(normalizedType) ? "diamond" : "square");
  const resolvedGlyph = glyph ?? explicitType?.glyph ?? null;
  const isDiamond = resolvedShape === "diamond";
  const isCapability = resolvedShape === "hub";
  const isWideGlyph = Boolean(resolvedGlyph && [...resolvedGlyph].length > 1);
  const classes = cx(
    "ds-elemicon",
    baseUX,
    skinX,
    roleSkinX,
    size !== "md" ? `ds-elemicon--${size}` : undefined,
    isDiamond ? "ds-elemicon--diamond" : undefined,
    isCapability ? "ds-elemicon--hub" : undefined,
    isWideGlyph ? "ds-elemicon--wide-glyph" : undefined,
    className,
  );

  return (
    <span className={classes} data-element-role={role} title={title ?? type ?? role} aria-label={type ?? role} {...props}>
      {isCapability ? <span className="ds-elemicon__pip" /> : null}
      {!isCapability && resolvedGlyph ? <span className="ds-elemicon__glyph">{resolvedGlyph}</span> : null}
    </span>
  );
}
