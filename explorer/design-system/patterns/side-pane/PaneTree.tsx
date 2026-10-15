import { cx } from "@linaria/atomic";
import { type HTMLAttributes } from "preact";
import { treeClass, treeNodeClass } from "./classes";

export type PaneTreeProps = Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style">;

export function PaneTree({ className = "", ...props }: PaneTreeProps) {
  return <div role="tree" className={cx("ux-tree", treeClass, className)} {...props} />;
}

export type PaneTreeNodeProps = Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style">;

export function PaneTreeNode({ className = "", ...props }: PaneTreeNodeProps) {
  return <div className={cx(treeNodeClass, className)} {...props} />;
}
