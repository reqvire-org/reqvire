import type { CSSProperties, HTMLAttributes } from "react";
import { css, cx } from "@linaria/atomic";
import { ELEMENT_TYPES, elementRole, roleColorToken, type ElementIconShape, type ElementType } from "../../palette";

export type { ElementIconShape, ElementRole, ElementType } from "../../palette";
export type ElementIconSize = "sm" | "md" | "lg";

const baseUX = css`
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex: none;
  width: var(--type-icon-md);
  height: var(--type-icon-md);
  border-radius: var(--radius-xs);

  svg {
    width: 60%;
    height: 60%;
  }

  .rq-elemicon__glyph {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    font-family: var(--font-mono);
    font-size: var(--rq-elemicon-glyph-fs, calc(var(--text-micro) - var(--space-1)));
    font-weight: var(--weight-bold);
    line-height: 1;
  }

  &.rq-elemicon--sm {
    width: var(--type-icon-sm);
    height: var(--type-icon-sm);
    border-radius: var(--rq-elemicon-sm-radius, var(--radius-xs));
  }

  &.rq-elemicon--lg {
    width: var(--type-icon-lg);
    height: var(--type-icon-lg);
    border-radius: var(--radius-sm);
  }

  &.rq-elemicon--sm .rq-elemicon__glyph {
    font-size: var(--rq-elemicon-sm-glyph-fs, calc(var(--text-micro) - var(--space-2)));
  }

  &.rq-elemicon--lg .rq-elemicon__glyph {
    font-size: var(--rq-elemicon-lg-glyph-fs, calc(var(--text-micro) - var(--space-1) / 2));
  }

  &.rq-elemicon--diamond {
    border-radius: var(--rq-elemicon-diamond-radius, var(--radius-xs));
    transform: rotate(45deg) scale(0.74);
  }

  &.rq-elemicon--diamond svg,
  &.rq-elemicon--diamond .rq-elemicon__glyph {
    transform: rotate(-45deg);
  }

  &.rq-elemicon--hub .rq-elemicon__pip {
    width: 36%;
    height: 36%;
    border-radius: 50%;
  }
`;

const skinX = css`
  color: var(--slate-0);
  background: var(--rq-elemicon-color, var(--other));
  box-shadow: inset 0 0 0 var(--border-w) color-mix(in srgb, var(--slate-950) 18%, transparent);

  .rq-elemicon__glyph {
    color: var(--text-strong);
  }

  &.rq-elemicon--hub {
    background: var(--slate-900);
    box-shadow: inset 0 0 0 var(--border-w) color-mix(in srgb, var(--slate-0) 10%, transparent);
  }

  &.rq-elemicon--hub .rq-elemicon__pip {
    background: var(--rq-elemicon-color, var(--capability));
  }
`;

const DIAMOND_TYPES = new Set([
  "source",
  "specification",
  "constraint",
  "behavior",
  "state",
  "input-output",
  "semantic-contract",
  "semantic-query-contract",
]);

export interface ElementIconProps extends HTMLAttributes<HTMLSpanElement> {
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
  style,
  shape,
  glyph,
  ...props
}: ElementIconProps) {
  const role = elementRole(type, family);
  const normalizedType = (type ?? "").toLowerCase();
  const explicitType = normalizedType in ELEMENT_TYPES ? ELEMENT_TYPES[normalizedType as ElementType] : null;
  const resolvedShape = shape ?? explicitType?.shape ?? (DIAMOND_TYPES.has(normalizedType) ? "diamond" : role === "capability" ? "hub" : "square");
  const resolvedGlyph = glyph ?? explicitType?.glyph ?? null;
  const isDiamond = resolvedShape === "diamond";
  const isCapability = resolvedShape === "hub";
  const classes = cx(
    "rq-elemicon",
    baseUX,
    skinX,
    size !== "md" ? `rq-elemicon--${size}` : undefined,
    isDiamond ? "rq-elemicon--diamond" : undefined,
    isCapability ? "rq-elemicon--hub" : undefined,
    className,
  );
  const iconStyle = { "--rq-elemicon-color": `var(${roleColorToken(role)})`, ...style } as CSSProperties;

  return (
    <span className={classes} style={iconStyle} title={title ?? type ?? role} aria-label={type ?? role} {...props}>
      {isCapability ? <span className="rq-elemicon__pip" /> : null}
      {!isCapability && resolvedGlyph ? <span className="rq-elemicon__glyph">{resolvedGlyph}</span> : null}
    </span>
  );
}
