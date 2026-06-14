import type { ButtonHTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

const baseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-4);
  height: var(--control-sm);
  padding: 0 var(--space-7);
  border-radius: var(--radius-pill);
  cursor: var(--rq-chip-cursor, pointer);
  white-space: nowrap;
  transition:
    background var(--dur-fast),
    border-color var(--dur-fast),
    color var(--dur-fast);

  svg {
    display: block;
    flex: 0 0 auto;
    width: var(--icon-xs);
    height: var(--icon-xs);
    opacity: 0.8;
  }

  .rq-chip__count {
    font-variant-numeric: tabular-nums;
    opacity: 0.7;
  }
`;

const skinX = css`
  color: var(--text-secondary);
  background: var(--rq-chip-bg, var(--bg-surface));
  border: var(--border-w) solid var(--rq-chip-border-color, var(--border-default));
  box-shadow: var(--rq-chip-shadow);
  font-size: var(--text-caption);
  font-weight: var(--weight-medium);

  &:hover {
    color: var(--rq-chip-hover-color);
    background: var(--rq-chip-hover-bg, var(--bg-hover));
    border-color: var(--rq-chip-hover-border, var(--border-strong));
  }

  &[aria-pressed="true"],
  &.is-active {
    color: var(--slate-0);
    background: var(--slate-900);
    border-color: var(--slate-900);
  }

  [data-theme="dark"] &[aria-pressed="true"],
  [data-theme="dark"] &.is-active {
    color: var(--slate-900);
    background: var(--slate-0);
    border-color: var(--slate-0);
  }
`;

export interface ChipProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  active?: boolean;
  icon?: ReactNode;
  count?: ReactNode;
  children: ReactNode;
}

export function Chip({
  active = false,
  icon,
  count,
  children,
  className = "",
  ...props
}: ChipProps) {
  return (
    <button
      type="button"
      aria-pressed={active}
      className={cx("rq-chip", baseUX, skinX, active ? "is-active" : undefined, className)}
      {...props}
    >
      {icon}
      <span>{children}</span>
      {count != null ? <span className="rq-chip__count">{count}</span> : null}
    </button>
  );
}
