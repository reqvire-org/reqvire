import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface AlertProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
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
  background: var(--danger-tint);
  border-color: var(--danger-border);
  color: var(--danger);
`;

const warningSkinX = css`
  background: var(--warning-tint);
  border-color: var(--warning-border);
  color: var(--warning);
`;

const successSkinX = css`
  background: var(--success-tint);
  border-color: var(--success-border);
  color: var(--success);
`;

const infoSkinX = css`
  background: var(--info-tint);
  border-color: var(--info-border);
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
        "ds-alert",
        baseUX,
        variantSkinX[variant],
        variant !== "default" ? `ds-alert--${variant}` : "",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}
