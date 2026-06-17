import { cx } from "@linaria/atomic";
import type { AnchorHTMLAttributes, HTMLAttributes } from "react";
import { paneActionRowClass, paneGhostLinkClass } from "./classes";

export type PaneActionRowProps = Omit<HTMLAttributes<HTMLDivElement>, "style">;

export function PaneActionRow({ className = "", ...props }: PaneActionRowProps) {
  return <div className={cx("ux-pane-action-row", paneActionRowClass, className)} {...props} />;
}

export type PaneGhostLinkProps = Omit<AnchorHTMLAttributes<HTMLAnchorElement>, "style">;

export function PaneGhostLink({ className = "", ...props }: PaneGhostLinkProps) {
  return <a className={cx("ux-pane-ghost-link", paneGhostLinkClass, className)} {...props} />;
}
