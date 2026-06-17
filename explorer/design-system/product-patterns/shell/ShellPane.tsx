import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export type ShellPanePlacement = "start" | "main" | "end";

export interface ShellPaneProps extends Omit<HTMLAttributes<HTMLElement>, "style"> {
  placement?: ShellPanePlacement;
  collapsed?: boolean;
  children?: ReactNode;
}

const baseUX = css`
  display: flex;
  min-width: 0;
  min-height: 0;
`;

const startUX = css`
  flex: 0 0 var(--ux-current-left-width);
  width: var(--ux-current-left-width);
  height: 100%;
`;

const mainUX = css`
  position: relative;
  flex: 1 1 auto;
  height: 100%;
`;

const endUX = css`
  flex: 0 0 var(--ux-current-right-width);
  width: var(--ux-current-right-width);
  height: 100%;
`;

const collapsedUX = css`
  display: none;
`;

export function ShellPane({
  placement = "main",
  collapsed = false,
  children,
  className = "",
  ...props
}: ShellPaneProps) {
  return (
    <section
      data-product-pattern="shell-pane"
      data-product-pattern-slot={`${placement}-pane`}
      data-placement={placement}
      data-collapsed={collapsed || undefined}
      className={cx(
        "ux-shell-pane",
        baseUX,
        placement === "start" && startUX,
        placement === "main" && mainUX,
        placement === "end" && endUX,
        collapsed && collapsedUX,
        className,
      )}
      {...props}
    >
      {children}
    </section>
  );
}
