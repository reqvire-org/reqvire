import type { ButtonHTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import type { ButtonSize, ButtonTone } from "./button_contract";

export interface IconButtonProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "style"> {
  children: ReactNode;
  size?: Extract<ButtonSize, "sm" | "md">;
  tone?: Extract<ButtonTone, "secondary" | "ghost">;
  active?: boolean;
}

const baseUX = css`
  display: inline-flex;
  width: var(--control-md);
  height: var(--control-md);
  align-items: center;
  justify-content: center;
  border: var(--border-w) solid transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition:
    background var(--dur-fast) var(--ease-standard),
    border-color var(--dur-fast) var(--ease-standard),
    color var(--dur-fast) var(--ease-standard);

  svg {
    display: block;
    width: var(--icon-md);
    height: var(--icon-md);
    flex: 0 0 auto;
  }

  &:focus-visible {
    outline: none;
    box-shadow: var(--ring-focus);
  }

  &:disabled {
    opacity: 0.45;
    pointer-events: none;
  }
`;

const skinX = css`
  background: transparent;
  color: var(--text-muted);

  &:hover {
    background: var(--bg-hover);
    color: var(--text-strong);
  }

  &:active {
    background: var(--bg-active);
  }

  &[aria-pressed="true"],
  &.is-active {
    background: var(--accent-subtle);
    color: var(--accent);
  }
`;

const ghostSkinX = css`
  &:hover {
    background: transparent;
    color: var(--text-strong);
  }
`;

const smUX = css`
  width: var(--control-sm);
  height: var(--control-sm);

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
  }
`;

export function IconButton({
  children,
  size = "md",
  tone = "secondary",
  active = false,
  className = "",
  type = "button",
  ...props
}: IconButtonProps) {
  return (
    <button
      type={type}
      className={cx(
        "ds-iconbtn",
        baseUX,
        skinX,
        size === "sm" ? smUX : "",
        size === "sm" ? "ds-iconbtn--sm" : "",
        tone === "ghost" ? ghostSkinX : "",
        tone === "ghost" ? "ds-iconbtn--ghost" : "",
        active ? "is-active" : "",
        className,
      )}
      aria-pressed={active || undefined}
      {...props}
    >
      {children}
    </button>
  );
}
