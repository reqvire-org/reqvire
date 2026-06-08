import type { HTMLAttributes, ReactNode } from "react";

export interface AlertProps extends HTMLAttributes<HTMLDivElement> {
  children?: ReactNode;
  variant?: "default" | "danger" | "warning" | "success";
}

export function Alert({ children, variant = "default", className = "", role = "alert", ...props }: AlertProps) {
  return (
    <div
      role={role}
      className={["rq-alert", variant !== "default" ? `rq-alert--${variant}` : "", className].filter(Boolean).join(" ")}
      {...props}
    >
      {children}
    </div>
  );
}
