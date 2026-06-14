import type { ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

const baseUX = css`
  box-sizing: border-box;
`;

const skinX = css`
  color: inherit;
`;

export function ExplorerWorkspaceToolbar({
  ariaLabel,
  className = "",
  children,
}: {
  ariaLabel: string;
  className?: string;
  children: ReactNode;
}) {
  return (
    <div className={cx("ex-workspace-toolbar", "ex-graph-control-panel", baseUX, skinX, className)} aria-label={ariaLabel}>
      {children}
    </div>
  );
}
