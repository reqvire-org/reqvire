import type { ButtonHTMLAttributes, ReactNode } from "react";
import type { ButtonSize, ButtonTone } from "./button_contract";

export interface IconButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
  size?: Extract<ButtonSize, "sm" | "md">;
  tone?: Extract<ButtonTone, "secondary" | "ghost">;
  active?: boolean;
}

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
      className={[
        "rq-iconbtn",
        size === "sm" ? "rq-iconbtn--sm" : "",
        tone === "ghost" ? "rq-iconbtn--ghost" : "",
        active ? "is-active" : "",
        className,
      ].filter(Boolean).join(" ")}
      aria-pressed={active || undefined}
      {...props}
    >
      {children}
    </button>
  );
}
