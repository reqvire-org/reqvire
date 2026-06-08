import type { CSSProperties, HTMLAttributes } from "react";
import { ELEMENT_TYPES, elementRole, roleColorToken, type ElementIconShape, type ElementType } from "../../palette";

export type { ElementIconShape, ElementRole, ElementType } from "../../palette";
export type ElementIconSize = "sm" | "md" | "lg";

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
  const classes = [
    "rq-elemicon",
    size !== "md" ? `rq-elemicon--${size}` : "",
    isDiamond ? "rq-elemicon--diamond" : "",
    isCapability ? "rq-elemicon--hub" : "",
    className,
  ]
    .filter(Boolean)
    .join(" ");
  const iconStyle = { "--_c": `var(${roleColorToken(role)})`, ...style } as CSSProperties;

  return (
    <span className={classes} style={iconStyle} title={title ?? type ?? role} aria-label={type ?? role} {...props}>
      {isCapability ? <span className="rq-elemicon__pip" /> : null}
      {!isCapability && resolvedGlyph ? <span className="rq-elemicon__glyph">{resolvedGlyph}</span> : null}
    </span>
  );
}
