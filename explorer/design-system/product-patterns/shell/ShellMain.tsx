import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { Icon } from "../../components/core/Icon";

export interface ShellMainProps extends Omit<HTMLAttributes<HTMLElement>, "style"> {
  warning?: ReactNode;
  children?: ReactNode;
}

const contentClass = css`
  position: relative;
  flex: 1 1 auto;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
  background: var(--bg-canvas);
`;

const schemaWarningClass = css`
  position: absolute;
  top: var(--space-8);
  right: var(--space-8);
  left: var(--space-8);
  z-index: var(--z-popover);
`;

const schemaAlertBaseUX = css`
  display: flex;
  align-items: center;
  gap: var(--space-5);
  margin: var(--space-4);
  padding: var(--space-6) var(--space-8);

  svg {
    flex: none;
  }
`;

const schemaAlertSkinX = css`
  border: var(--border-w) solid var(--danger-border);
  border-radius: var(--radius-md);
  background: var(--danger-tint);
  color: var(--text-strong);
  box-shadow: var(--shadow-xs);

  svg {
    color: var(--danger);
  }
`;

const iconSmClass = css`
  width: var(--space-8);
  height: var(--space-8);
  flex: none;
`;

export function ShellMain({ warning, children, className = "", ...props }: ShellMainProps) {
  return (
    <main
      data-product-pattern="shell-main"
      data-product-pattern-slot="main"
      className={cx("ux-shell-main", contentClass, className)}
      {...props}
    >
      {warning != null ? (
        <div className={cx(schemaWarningClass)}>
          <div role="alert" className={cx(schemaAlertBaseUX, schemaAlertSkinX)}>
            <Icon name="alert-triangle" className={cx(iconSmClass)} />
            <span>{warning}</span>
          </div>
        </div>
      ) : null}
      {children}
    </main>
  );
}
