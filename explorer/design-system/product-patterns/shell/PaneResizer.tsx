import type { HTMLAttributes } from "react";
import { css, cx } from "@linaria/atomic";

export type PaneResizerOrientation = "vertical" | "horizontal";

export interface PaneResizerProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  orientation?: PaneResizerOrientation;
  active?: boolean;
}

const baseUX = css`
  position: absolute;
  z-index: var(--z-sticky);
  cursor: ew-resize;
  touch-action: none;

  &::before {
    position: absolute;
    background: transparent;
    content: "";
  }

  &:hover::before,
  &:focus-visible::before,
  &[data-active="true"]::before {
    background: var(--border-strong);
  }

  &:focus-visible {
    outline: none;
  }
`;

const verticalUX = css`
  top: 0;
  bottom: 0;
  left: calc(var(--ux-current-left-width) - var(--space-1));
  width: var(--space-3);
  transform: translateX(-50%);

  &::before {
    top: 0;
    bottom: 0;
    left: 50%;
    width: var(--border-w);
    transform: translateX(-50%);
  }
`;

const horizontalUX = css`
  right: 0;
  bottom: calc(var(--ux-current-bottom-height, 0) - var(--space-1));
  left: 0;
  height: var(--space-3);
  cursor: ns-resize;
  transform: translateY(50%);

  &::before {
    top: 50%;
    right: 0;
    left: 0;
    height: var(--border-w);
    transform: translateY(-50%);
  }
`;

export function PaneResizer({
  orientation = "vertical",
  active = false,
  role = "separator",
  className = "",
  ...props
}: PaneResizerProps) {
  return (
    <div
      data-product-pattern="pane-resizer"
      data-orientation={orientation}
      data-active={active || undefined}
      role={role}
      className={cx(
        "ux-pane-resizer",
        baseUX,
        orientation === "vertical" ? verticalUX : horizontalUX,
        className,
      )}
      {...props}
    />
  );
}
