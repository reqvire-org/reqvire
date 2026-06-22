import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

const baseUX = css`
  display: var(--ds-stat-display, inline-flex);
  min-width: var(--ds-stat-min-w);
  align-items: baseline;
  justify-content: var(--ds-stat-jc);
  gap: var(--space-3);

  &.ds-stat--stacked {
    flex-direction: column;
    gap: var(--stack-gap-compact);
  }
`;

const skinX = css`
  .ds-stat__label {
    color: var(--text-muted);
    font-size: var(--text-caption);
  }

  .ds-stat__value {
    color: var(--text-strong);
    font-size: var(--text-caption);
    font-variant-numeric: tabular-nums;
    font-weight: var(--weight-semibold);
  }

  &.ds-stat--stacked .ds-stat__value {
    font-size: var(--text-2xl);
    font-weight: var(--weight-semibold);
    line-height: 1.1;
  }
`;

const baseUXRow = css`
  display: var(--ds-statrow-display, flex);
  width: var(--ds-statrow-width, auto);
  grid-template-columns: var(--ds-statrow-grid-template-columns, none);
  flex-wrap: var(--ds-statrow-flex-wrap, wrap);
  align-items: var(--ds-statrow-align-items, baseline);
  gap: var(--ds-statrow-gap, var(--space-3) var(--space-8));
`;

export type StatProps = Omit<HTMLAttributes<HTMLSpanElement>, "style"> & {
  label: ReactNode;
  value: ReactNode;
  stacked?: boolean;
};

export function Stat({
  label,
  value,
  stacked = false,
  className = "",
  ...props
}: StatProps) {
  return (
    <span className={cx("ds-stat", baseUX, skinX, stacked ? "ds-stat--stacked" : undefined, className)} {...props}>
      {stacked ? (
        <>
          <span className="ds-stat__value">{value}</span>
          <span className="ds-stat__label">{label}</span>
        </>
      ) : (
        <>
          <span className="ds-stat__label">{label}</span>
          <span className="ds-stat__value">{value}</span>
        </>
      )}
    </span>
  );
}

export type StatRowProps = Omit<HTMLAttributes<HTMLDivElement>, "style"> & { children: ReactNode };

export function StatRow({
  children,
  className = "",
  ...props
}: StatRowProps) {
  return (
    <div className={cx("ds-statrow", baseUXRow, className)} {...props}>
      {children}
    </div>
  );
}
