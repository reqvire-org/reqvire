import { type HTMLAttributes } from "preact";
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
  color: var(--color-paper);
  background: var(--ds-elemicon-color, var(--color-role-other));
  box-shadow: inset 0 0 0 var(--border-w) var(--color-role-element-icon-ring);

  /* Role fills are the same in every theme, so glyphs use the fixed ink for light fills. */
  .ds-elemicon__glyph {
    color: var(--color-ink);
  }

  &.ds-elemicon--hub {
    background: var(--color-ink);
    box-shadow: inset 0 0 0 var(--border-w) var(--color-role-element-icon-ring-inverse);
  }

  &.ds-elemicon--hub .ds-elemicon__pip {
    background: var(--ds-elemicon-color, var(--color-role-capability));
  }
`;

const roleSkinX = css`
  &[data-element-role="capability"] { --ds-elemicon-color: var(--color-role-capability); }
  &[data-element-role="requirement"] { --ds-elemicon-color: var(--color-role-requirement); }
  &[data-element-role="contract"],
  &[data-element-role="source"],
  &[data-element-role="constraint"],
  &[data-element-role="behavior"],
  &[data-element-role="state"],
  &[data-element-role="input-output"],
  &[data-element-role="specification"] { --ds-elemicon-color: var(--color-role-contract); }
  &[data-element-role="semantic-contract"] { --ds-elemicon-color: var(--color-role-semantic-contract); }
  &[data-element-role="verification-objective"] { --ds-elemicon-color: var(--color-role-verification-objective); }
  &[data-element-role="verification"] { --ds-elemicon-color: var(--color-role-verification); }
  &[data-element-role="semantic-query"],
  &[data-element-role="ontology"] { --ds-elemicon-color: var(--color-role-ontology); }
  &[data-element-role="concept"] { --ds-elemicon-color: var(--color-role-concept); }
  &[data-element-role="concept-scheme"] { --ds-elemicon-color: var(--color-role-concept-scheme); }
  &[data-element-role="concept-reference"] { --ds-elemicon-color: var(--color-role-concept-reference); }
  &[data-element-role="resource"] { --ds-elemicon-color: var(--color-role-resource); }
  &[data-element-role="other"] { --ds-elemicon-color: var(--color-role-other); }
`;

const DIAMOND_TYPES = new Set([
  "source",
  "specification",
  "constraint",
  "behavior",
  "state",
  "input-output",
]);

export interface ElementIconProps extends Omit<(HTMLAttributes<HTMLSpanElement> & { className?: string }), "style"> {
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
    <span className={classes} data-element-role={role} title={title ?? type ?? role} role="img" aria-label={type ?? role} {...props}>
      {isCapability ? <span className="ds-elemicon__pip" /> : null}
      {!isCapability && resolvedGlyph ? <span className="ds-elemicon__glyph">{resolvedGlyph}</span> : null}
    </span>
  );
}
