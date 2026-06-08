import type { HTMLAttributes, ReactNode } from "react";

export interface CardProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  interactive?: boolean;
  selected?: boolean;
  padded?: boolean;
  accentColor?: string;
}

export function Card({
  children,
  interactive = false,
  selected = false,
  padded = true,
  accentColor,
  className = "",
  ...props
}: CardProps) {
  return (
    <div
      className={[
        "rq-card",
        padded ? "rq-card--pad" : "",
        interactive ? "rq-card--interactive" : "",
        selected ? "rq-card--selected" : "",
        className,
      ].filter(Boolean).join(" ")}
      {...props}
    >
      {accentColor ? <span className="rq-card__accent" style={{ background: accentColor }} /> : null}
      {children}
    </div>
  );
}
