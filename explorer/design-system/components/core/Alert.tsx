import { type HTMLAttributes, type ComponentChildren } from "preact";
import { css, cx } from "@linaria/atomic";

export interface AlertProps extends Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style"> {
  children?: ComponentChildren;
  variant?: "default" | "danger" | "warning" | "success" | "info";
}

const baseUX = css`
  display: flex;
  align-items: flex-start;
  gap: var(--space-5);
  padding: var(--space-7) var(--space-8);
  border-width: var(--border-w);
  border-style: solid;
  border-color: var(--border-subtle);
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
  background: var(--danger-bg);
  border-color: var(--danger-border);
  color: var(--danger-text);
`;

const warningSkinX = css`
  background: var(--warning-bg);
  border-color: var(--warning-border);
  color: var(--warning-text);
`;

const successSkinX = css`
  background: var(--success-bg);
  border-color: var(--success-border);
  color: var(--success-text);
`;

const infoSkinX = css`
  background: var(--info-bg);
  border-color: var(--info-border);
  color: var(--info-text);
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
