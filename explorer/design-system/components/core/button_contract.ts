export type ButtonTone = "primary" | "accent" | "secondary" | "ghost" | "danger" | "link";
export type ButtonSize = "sm" | "md" | "lg";

export const BUTTON_TONE_CLASSES: Record<ButtonTone, string> = {
  primary: "rq-btn--primary",
  accent: "rq-btn--accent",
  secondary: "rq-btn--secondary",
  ghost: "rq-btn--ghost",
  danger: "rq-btn--danger",
  link: "rq-btn--link",
};

export const BUTTON_SIZE_CLASSES: Record<ButtonSize, string> = {
  sm: "rq-btn--sm",
  md: "",
  lg: "rq-btn--lg",
};
