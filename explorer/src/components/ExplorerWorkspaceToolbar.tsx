import type { ReactNode } from "react";

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
    <div className={["ex-workspace-toolbar graph-control-panel", className].filter(Boolean).join(" ")} aria-label={ariaLabel}>
      {children}
    </div>
  );
}
