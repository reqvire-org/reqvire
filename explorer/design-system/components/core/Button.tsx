import type { ButtonHTMLAttributes, ReactNode } from "react";
import type { ButtonSize, ButtonTone } from "./button_contract";
import { BUTTON_SIZE_CLASSES, BUTTON_TONE_CLASSES } from "./button_contract";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children?: ReactNode;
  tone?: ButtonTone;
  size?: ButtonSize;
  iconLeft?: ReactNode;
  iconRight?: ReactNode;
  block?: boolean;
}

export function Button({
  children,
  tone = "secondary",
  size = "md",
  iconLeft,
  iconRight,
  block = false,
  className = "",
  type = "button",
  ...props
}: ButtonProps) {
  return (
    <button
      type={type}
      className={[
        "rq-btn",
        BUTTON_TONE_CLASSES[tone],
        BUTTON_SIZE_CLASSES[size],
        block ? "rq-btn--block" : "",
        className,
      ].filter(Boolean).join(" ")}
      {...props}
    >
      {iconLeft ? <span className="rq-btn__icon">{iconLeft}</span> : null}
      {children ? <span className="rq-btn__label">{children}</span> : null}
      {iconRight ? <span className="rq-btn__icon">{iconRight}</span> : null}
    </button>
  );
}
