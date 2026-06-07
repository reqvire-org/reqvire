import type { ReactNode } from "react";

export function ExplorerWorkspaceToolbar({
  ariaLabel,
  children,
}: {
  ariaLabel: string;
  children: ReactNode;
}) {
  return (
    <div className="explorer-workspace-toolbar graph-control-panel" aria-label={ariaLabel}>
      {children}
    </div>
  );
}
