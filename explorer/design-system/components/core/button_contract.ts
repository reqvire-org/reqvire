export type ButtonTone = "primary" | "accent" | "secondary" | "ghost" | "danger" | "link";
export type ButtonSize = "sm" | "md" | "lg";

export const BUTTON_TONE_CLASSES: Record<ButtonTone, string> = {
  primary: "ds-btn--primary",
  accent: "ds-btn--accent",
  secondary: "ds-btn--secondary",
  ghost: "ds-btn--ghost",
  danger: "ds-btn--danger",
  link: "ds-btn--link",
};

export const BUTTON_SIZE_CLASSES: Record<ButtonSize, string> = {
  sm: "ds-btn--sm",
  md: "",
  lg: "ds-btn--lg",
};
