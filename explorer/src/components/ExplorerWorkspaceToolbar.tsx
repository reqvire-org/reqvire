import { type ComponentChildren } from "preact";
import { WorkspaceToolbar } from "@ds";

export interface ExplorerWorkspaceToolbarProps {
  ariaLabel: string;
  className?: string;
  children: ComponentChildren;
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
