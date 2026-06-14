import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface AlertProps extends HTMLAttributes<HTMLDivElement> {
  children?: ReactNode;
  variant?: "default" | "danger" | "warning" | "success" | "info";
}

const baseUX = css`
  display: flex;
  align-items: flex-start;
  gap: var(--space-5);
  padding: var(--space-7) var(--space-8);
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);

  svg {
    display: block;
    flex: 0 0 auto;
  }
`;

const skinX = css`
  background: var(--bg-sunken);
  color: var(--text-body);
`;

const dangerSkinX = css`
  background: color-mix(in srgb, var(--danger) 8%, var(--bg-surface));
  border-color: color-mix(in srgb, var(--danger) 32%, var(--border-subtle));
  color: var(--danger);
`;

const warningSkinX = css`
  background: color-mix(in srgb, var(--warning) 9%, var(--bg-surface));
  border-color: color-mix(in srgb, var(--warning) 34%, var(--border-subtle));
  color: var(--warning);
`;

const successSkinX = css`
  background: color-mix(in srgb, var(--success) 9%, var(--bg-surface));
  border-color: color-mix(in srgb, var(--success) 34%, var(--border-subtle));
  color: var(--success);
`;

const infoSkinX = css`
  background: color-mix(in srgb, var(--info) 9%, var(--bg-surface));
  border-color: color-mix(in srgb, var(--info) 34%, var(--border-subtle));
  color: var(--info);
`;

const variantSkinX: Record<NonNullable<AlertProps["variant"]>, string> = {
  default: skinX,
  danger: dangerSkinX,
  warning: warningSkinX,
  success: successSkinX,
  info: infoSkinX,
};

export function Alert({ children, variant = "default", className = "", role = "alert", ...props }: AlertProps) {
  return (
    <div
      role={role}
      className={cx(
        "rq-alert",
        baseUX,
        variantSkinX[variant],
        variant !== "default" ? `rq-alert--${variant}` : "",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}
