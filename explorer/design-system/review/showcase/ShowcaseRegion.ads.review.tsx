import { Children } from "preact/compat";
import { h, cloneElement, type HTMLAttributes, type VNode } from "preact";
import { cx } from "@linaria/atomic";
import { previewFrameClasses } from "@reqvire-org/ads/preview";
import "./frame.ads.review.css";

// Generic arrangements use ADS's preview framing; the project classes below
// are Reqvire-specific demonstration layouts used by real scenarios.
const regionClasses = {
  "page": cx(previewFrameClasses("page"), "ux-showcase-page"),
  "section": previewFrameClasses("section"),
  "section__heading": previewFrameClasses("heading"),
  "section__title": previewFrameClasses("title"),
  "section__desc": previewFrameClasses("description"),
  "label": previewFrameClasses("label"),
  "row": previewFrameClasses("row"),
  "row--center": previewFrameClasses("row-center"),
  "col": previewFrameClasses("column"),
  "canvas": "ux-showcase-canvas",
  "card-demo": "ux-showcase-card-demo",
  "card-demo__desc": "ux-showcase-card-demo__desc",
  "card-demo__title": "ux-showcase-card-demo__title",
  "control-list": "ux-showcase-control-list",
  "icon-grid": "ux-showcase-icon-grid",
  "icon-item": "ux-showcase-icon-item",
  "input-demo": "ux-showcase-input-demo",
  "input-demo--lg": "ux-showcase-input-demo--lg",
  "input-demo--md": "ux-showcase-input-demo--md",
  "modal-copy": "ux-showcase-modal-copy",
  "modal-footer-spacer": "ux-showcase-modal-footer-spacer",
  "muted-note": "ux-showcase-muted-note",
  "pattern-card": "ux-showcase-pattern-card",
  "pattern-card--notice": "ux-showcase-pattern-card--notice",
  "pattern-card--tall": "ux-showcase-pattern-card--tall",
  "pattern-card__body": "ux-showcase-pattern-card__body",
  "pattern-grid": "ux-showcase-pattern-grid",
  "pattern-grid--two": "ux-showcase-pattern-grid--two",
  "pattern-muted": "ux-showcase-pattern-muted",
  "pattern-toolbar": "ux-showcase-pattern-toolbar",
  "shell-badge-placeholder": "ux-showcase-shell-badge-placeholder",
  "shell-board": "ux-showcase-shell-board",
  "shell-demo": "ux-showcase-shell-demo",
  "shell-demo__app": "ux-showcase-shell-demo__app",
  "shell-detail-rail": "ux-showcase-shell-detail-rail",
  "shell-main-mock": "ux-showcase-shell-main-mock",
  "shell-nav-placeholder": "ux-showcase-shell-nav-placeholder",
  "shell-nav-placeholder--medium": "ux-showcase-shell-nav-placeholder--medium",
  "shell-nav-placeholder--wide": "ux-showcase-shell-nav-placeholder--wide",
  "shell-pane-mock": "ux-showcase-shell-pane-mock",
  "shell-placeholder": "ux-showcase-shell-placeholder",
  "shell-placeholder--action": "ux-showcase-shell-placeholder--action",
  "shell-placeholder--heading": "ux-showcase-shell-placeholder--heading",
  "shell-placeholder--icon": "ux-showcase-shell-placeholder--icon",
  "shell-placeholder--panel": "ux-showcase-shell-placeholder--panel",
  "shell-placeholder--panel-accent": "ux-showcase-shell-placeholder--panel-accent",
  "shell-placeholder--row": "ux-showcase-shell-placeholder--row",
  "shell-placeholder--search": "ux-showcase-shell-placeholder--search",
  "shell-placeholder--short": "ux-showcase-shell-placeholder--short",
  "shell-placeholder--stat": "ux-showcase-shell-placeholder--stat",
  "shell-placeholder--subtitle": "ux-showcase-shell-placeholder--subtitle",
  "shell-placeholder--tile": "ux-showcase-shell-placeholder--tile",
  "shell-placeholder--title": "ux-showcase-shell-placeholder--title",
  "shell-placeholder-group": "ux-showcase-shell-placeholder-group",
  "shell-placeholder-stack": "ux-showcase-shell-placeholder-stack",
  "shell-stat-grid": "ux-showcase-shell-stat-grid",
  "shell-visually-hidden": "ux-showcase-shell-visually-hidden",
  "shell-workspace-header": "ux-showcase-shell-workspace-header",
  "sidebar-demo": "ux-showcase-sidebar-demo",
  "sidebar-stats": "ux-showcase-sidebar-stats",
  "stat-line": "ux-showcase-stat-line",
  "status": "ux-showcase-status",
  "status--ready": "ux-showcase-status--ready",
  "table-strong": "ux-showcase-table-strong",
  "tree-demo": "ux-showcase-tree-demo",
} as const;

export type ShowcaseRegionName = keyof typeof regionClasses;
export interface ShowcaseRegionProps extends Omit<(HTMLAttributes<HTMLElement> & { className?: string }), "style" | "className"> {
  regions: readonly ShowcaseRegionName[];
  as?: "div" | "span" | "section" | "main" | "p" | "aside";
  asChild?: boolean;
}

/** A finite set of demonstration arrangements shared by the project's scenarios. */
export function ShowcaseRegion({ regions, as: Tag = "div", asChild = false, children, ...props }: ShowcaseRegionProps) {
  const classes = cx(...regions.map(region => regionClasses[region]));
  if (asChild) {
    const child = Children.only(children) as VNode<{ className?: string }>;
    return cloneElement(child, { ...props, className: cx(child.props.className, classes) });
  }
  return h(Tag, { className: classes, ...props }, children);
}
