import type { HTMLAttributes, ReactNode } from "react";

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
    <span className={["rq-stat", stacked ? "rq-stat--stacked" : "", className].filter(Boolean).join(" ")} {...props}>
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
    <div className={["rq-statrow", className].filter(Boolean).join(" ")} {...props}>
      {children}
    </div>
  );
}
