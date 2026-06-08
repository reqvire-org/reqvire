import type { HTMLAttributes, ReactNode } from "react";

export interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  children?: ReactNode;
  variant?: "default" | "accent" | "solid" | "dot";
}

export function Badge({ children, variant = "default", className = "", ...props }: BadgeProps) {
  const cls = [
    "rq-badge",
    variant !== "default" ? `rq-badge--${variant}` : "",
    className,
  ].filter(Boolean).join(" ");

  return (
    <span className={cls} {...props}>
      {variant === "dot" ? null : children}
    </span>
  );
}
