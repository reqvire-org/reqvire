import { cx } from "@linaria/atomic";
import { type AnchorHTMLAttributes, type HTMLAttributes } from "preact";
import { paneActionRowClass, paneGhostLinkClass } from "./classes";

export type PaneActionRowProps = Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style">;

export function PaneActionRow({ className = "", ...props }: PaneActionRowProps) {
  return <div className={cx("ux-pane-action-row", paneActionRowClass, className)} {...props} />;
}

export type PaneGhostLinkProps = Omit<(AnchorHTMLAttributes<HTMLAnchorElement> & { className?: string }), "style" | "role" | "href"> & { href: string };

export function PaneGhostLink({ className = "", ...props }: PaneGhostLinkProps) {
  return <a className={cx("ux-pane-ghost-link", paneGhostLinkClass, className)} {...props} />;
}
