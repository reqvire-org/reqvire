import type { ReactNode } from "react";
import { WorkspaceToolbar } from "@ds";

export interface ExplorerWorkspaceToolbarProps {
  ariaLabel: string;
  className?: string;
  children: ReactNode;
}

export function ExplorerWorkspaceToolbar({
  ariaLabel,
  className = "",
  children,
}: ExplorerWorkspaceToolbarProps) {
  return (
    <WorkspaceToolbar className={className} aria-label={ariaLabel}>
      {children}
    </WorkspaceToolbar>
  );
}
