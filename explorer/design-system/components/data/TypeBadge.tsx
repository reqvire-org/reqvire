import type { CSSProperties, HTMLAttributes, ReactNode } from "react";
import { elementRole, roleColorToken } from "../../palette";

export interface TypeBadgeProps extends HTMLAttributes<HTMLSpanElement> {
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
  style,
  ...props
}: TypeBadgeProps) {
  const role = elementRole(type, family);
  const color = `var(${roleColorToken(role)})`;
  const badgeStyle = tinted
    ? ({
        "--_tint": `color-mix(in srgb, ${color} 16%, transparent)`,
        "--_ink": `color-mix(in srgb, ${color} 78%, var(--text-strong))`,
        ...style,
      } as CSSProperties)
    : style;

  return (
    <span
      className={["rq-typebadge", tinted ? "rq-typebadge--tinted" : "", className].filter(Boolean).join(" ")}
      style={badgeStyle}
      {...props}
    >
      {dot ? <span className="rq-typebadge__dot" style={{ background: color }} /> : null}
      {children ?? type}
    </span>
  );
}
