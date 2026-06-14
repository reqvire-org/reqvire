import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

const baseUX = css`
  display: var(--rq-stat-display, inline-flex);
  min-width: var(--rq-stat-min-w);
  align-items: baseline;
  justify-content: var(--rq-stat-jc);
  gap: var(--space-3);

  &.rq-stat--stacked {
    flex-direction: column;
    gap: 0;
  }
`;

const skinX = css`
  .rq-stat__label {
    color: var(--text-muted);
    font-size: var(--text-caption);
  }

  .rq-stat__value {
    color: var(--text-strong);
    font-size: var(--text-caption);
    font-variant-numeric: tabular-nums;
    font-weight: var(--weight-semibold);
  }

  &.rq-stat--stacked .rq-stat__value {
    font-size: var(--text-2xl);
    font-weight: var(--weight-semibold);
    line-height: 1.1;
  }
`;

const baseUXRow = css`
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: var(--space-3) var(--space-8);
`;

export type StatProps = HTMLAttributes<HTMLSpanElement> & {
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
    <span className={cx("rq-stat", baseUX, skinX, stacked ? "rq-stat--stacked" : undefined, className)} {...props}>
      {stacked ? (
        <>
          <span className="rq-stat__value">{value}</span>
          <span className="rq-stat__label">{label}</span>
        </>
      ) : (
        <>
          <span className="rq-stat__label">{label}</span>
          <span className="rq-stat__value">{value}</span>
        </>
      )}
    </span>
  );
}

export type StatRowProps = HTMLAttributes<HTMLDivElement> & { children: ReactNode };

export function StatRow({
  children,
  className = "",
  ...props
}: StatRowProps) {
  return (
    <div className={cx("rq-statrow", baseUXRow, className)} {...props}>
      {children}
    </div>
  );
}
