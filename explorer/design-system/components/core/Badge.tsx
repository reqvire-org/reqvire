import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface BadgeProps extends Omit<HTMLAttributes<HTMLSpanElement>, "style"> {
  children?: ReactNode;
  variant?: "default" | "accent" | "solid" | "dot";
}

const baseUX = css`
  display: inline-flex;
  min-width: var(--ds-badge-min-w, calc(var(--space-8) + var(--space-1)));
  height: var(--ds-badge-h, calc(var(--space-8) + var(--space-1)));
  align-items: center;
  justify-content: center;
  padding: 0 var(--space-3);
  border-radius: var(--radius-pill);
  font-size: var(--text-micro);
  font-variant-numeric: tabular-nums;
  font-weight: var(--weight-semibold);
  line-height: 1;
`;

const skinX = css`
  background: var(--bg-sunken);
  color: var(--text-secondary);
`;

const accentSkinX = css`
  background: var(--accent-subtle);
  color: var(--accent);
`;

const solidSkinX = css`
  background: var(--slate-700);
  color: var(--slate-0);

  [data-theme="dark"] & {
    background: var(--slate-300);
    color: var(--slate-900);
  }
`;

const dotSkinX = css`
  min-width: 0;
  width: var(--ds-badge-dot-size, calc(var(--space-3) + var(--space-1) / 2));
  height: var(--ds-badge-dot-size, calc(var(--space-3) + var(--space-1) / 2));
  padding: 0;
`;

const variantSkinX: Record<NonNullable<BadgeProps["variant"]>, string> = {
  default: skinX,
  accent: accentSkinX,
  solid: solidSkinX,
  dot: cx(skinX, dotSkinX),
};

export function Badge({ children, variant = "default", className = "", ...props }: BadgeProps) {
  const cls = cx(
    "ds-badge",
    baseUX,
    variantSkinX[variant],
    variant !== "default" ? `ds-badge--${variant}` : "",
    className,
  );

  return (
    <span className={cls} {...props}>
      {variant === "dot" ? null : children}
    </span>
  );
}
