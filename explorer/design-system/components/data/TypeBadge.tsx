import type { CSSProperties, HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { elementRole, roleColorToken } from "../../palette";

const baseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  height: var(--rq-typebadge-h, calc(var(--space-8) + var(--space-2)));
  padding: 0 var(--space-5);
  border-radius: var(--radius-sm);
  font-size: var(--text-micro);
  font-weight: var(--weight-medium);
  line-height: 1;
  white-space: nowrap;

  .rq-typebadge__dot {
    flex: none;
    width: var(--rq-typebadge-dot-size, calc(var(--space-3) + var(--space-1) / 2));
    height: var(--rq-typebadge-dot-size, calc(var(--space-3) + var(--space-1) / 2));
    border-radius: var(--rq-typebadge-dot-radius, calc(var(--radius-xs) / 2));
  }
`;

const skinX = css`
  color: var(--text-secondary);
  background: var(--bg-sunken);

  .rq-typebadge__dot {
    background: var(--rq-typebadge-color);
  }

  &.rq-typebadge--tinted {
    color: var(--rq-typebadge-ink);
    background: var(--rq-typebadge-tint);
  }
`;

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
        "--rq-typebadge-color": color,
        "--rq-typebadge-tint": `color-mix(in srgb, ${color} 16%, transparent)`,
        "--rq-typebadge-ink": `color-mix(in srgb, ${color} 78%, var(--text-strong))`,
        ...style,
      } as CSSProperties)
    : ({ "--rq-typebadge-color": color, ...style } as CSSProperties);

  return (
    <span
      className={cx("rq-typebadge", baseUX, skinX, tinted ? "rq-typebadge--tinted" : undefined, className)}
      style={badgeStyle}
      {...props}
    >
      {dot ? <span className="rq-typebadge__dot" /> : null}
      {children ?? type}
    </span>
  );
}
