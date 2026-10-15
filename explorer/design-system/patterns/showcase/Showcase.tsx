import { Children } from "preact/compat";
import { h, cloneElement, type HTMLAttributes, type VNode, type ComponentChildren } from "preact";
import { useId, useState } from "preact/hooks";
import { cx } from "@linaria/atomic";
import "./frame.css";

const regionClasses = {
  "components-page": "ux-showcase-components-page",
  "input-demo": "ux-showcase-input-demo",
  "app": "ux-showcase-app",
  "body": "ux-showcase-body",
  "body--full": "ux-showcase-body--full",
  "canvas": "ux-showcase-canvas",
  "canvas--sunken": "ux-showcase-canvas--sunken",
  "card-demo": "ux-showcase-card-demo",
  "card-demo__desc": "ux-showcase-card-demo__desc",
  "card-demo__title": "ux-showcase-card-demo__title",
  "col": "ux-showcase-col",
  "control-list": "ux-showcase-control-list",
  "divider": "ux-showcase-divider",
  "font-sample": "ux-showcase-font-sample",
  "font-sample--mono": "ux-showcase-font-sample--mono",
  "font-sample--sans": "ux-showcase-font-sample--sans",
  "icon-grid": "ux-showcase-icon-grid",
  "icon-item": "ux-showcase-icon-item",
  "icon-item__name": "ux-showcase-icon-item__name",
  "input-demo--lg": "ux-showcase-input-demo--lg",
  "input-demo--md": "ux-showcase-input-demo--md",
  "label": "ux-showcase-label",
  "label--spaced": "ux-showcase-label--spaced",
  "mock-card": "ux-showcase-mock-card",
  "mock-card__desc": "ux-showcase-mock-card__desc",
  "mock-card__icon": "ux-showcase-mock-card__icon",
  "mock-card__label": "ux-showcase-mock-card__label",
  "mocks": "ux-showcase-mocks",
  "mocks-grid": "ux-showcase-mocks-grid",
  "mocks__workspace": "ux-showcase-mocks__workspace",
  "modal-copy": "ux-showcase-modal-copy",
  "modal-footer-spacer": "ux-showcase-modal-footer-spacer",
  "modal-link": "ux-showcase-modal-link",
  "muted-note": "ux-showcase-muted-note",
  "page": "ux-showcase-page",
  "page--wide": "ux-showcase-page--wide",
  "pattern-card": "ux-showcase-pattern-card",
  "pattern-card--notice": "ux-showcase-pattern-card--notice",
  "pattern-card--tall": "ux-showcase-pattern-card--tall",
  "pattern-card__body": "ux-showcase-pattern-card__body",
  "pattern-grid": "ux-showcase-pattern-grid",
  "pattern-grid--two": "ux-showcase-pattern-grid--two",
  "pattern-muted": "ux-showcase-pattern-muted",
  "pattern-toolbar": "ux-showcase-pattern-toolbar",
  "row": "ux-showcase-row",
  "row--baseline": "ux-showcase-row--baseline",
  "row--center": "ux-showcase-row--center",
  "row--spaced": "ux-showcase-row--spaced",
  "section": "ux-showcase-section",
  "section__desc": "ux-showcase-section__desc",
  "section__heading": "ux-showcase-section__heading",
  "section__title": "ux-showcase-section__title",
  "shadow-card": "ux-showcase-shadow-card",
  "shadow-grid": "ux-showcase-shadow-grid",
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
  "space-row": "ux-showcase-space-row",
  "space-row__bar": "ux-showcase-space-row__bar",
  "space-row__label": "ux-showcase-space-row__label",
  "space-stack": "ux-showcase-space-stack",
  "stat-line": "ux-showcase-stat-line",
  "status": "ux-showcase-status",
  "status--ready": "ux-showcase-status--ready",
  "swatch": "ux-showcase-swatch",
  "swatch-grid": "ux-showcase-swatch-grid",
  "swatch-grid--fixed-3": "ux-showcase-swatch-grid--fixed-3",
  "swatch-grid--fixed-4": "ux-showcase-swatch-grid--fixed-4",
  "swatch-grid--fixed-5": "ux-showcase-swatch-grid--fixed-5",
  "swatch__color": "ux-showcase-swatch__color",
  "swatch__name": "ux-showcase-swatch__name",
  "table-strong": "ux-showcase-table-strong",
  "topbar": "ux-showcase-topbar",
  "topbar__brand": "ux-showcase-topbar__brand",
  "topbar__brand-mark": "ux-showcase-topbar__brand-mark",
  "tree-demo": "ux-showcase-tree-demo",
  "type-note": "ux-showcase-type-note",
  "type-row": "ux-showcase-type-row",
  "type-row__label": "ux-showcase-type-row__label",
  "type-sample": "ux-showcase-type-sample",
  "type-stack": "ux-showcase-type-stack",
} as const;

export type ShowcaseRegionName = keyof typeof regionClasses;
export interface ShowcaseRegionProps extends Omit<(HTMLAttributes<HTMLElement> & { className?: string }), "style" | "className"> {
  regions: readonly ShowcaseRegionName[];
  as?: "div" | "span" | "section" | "main" | "p" | "aside";
  asChild?: boolean;
}

/** A finite set of frame layouts, shared by examples and review screens. */
export function ShowcaseRegion({ regions, as: Tag = "div", asChild = false, children, ...props }: ShowcaseRegionProps) {
  const classes = cx(...regions.map(region => regionClasses[region]));
  if (asChild) {
    const child = Children.only(children) as VNode<{ className?: string }>;
    return cloneElement(child, { ...props, className: cx(child.props.className, classes) });
  }
  return h(Tag, { className: classes, ...props }, children);
}

export interface ShowcaseNavigationItem<Value extends string> {
  value: Value;
  label: ComponentChildren;
  icon?: ComponentChildren;
}
export interface ShowcaseNavigationProps<Value extends string> {
  items: readonly ShowcaseNavigationItem<Value>[];
  value: Value;
  onChange: (value: Value) => void;
}

/** Neutral demo navigation; product navigation is shown inside examples. */
export function ShowcaseNavigation<Value extends string>({ items, value, onChange }: ShowcaseNavigationProps<Value>) {
  return <nav className="ux-showcase-navigation" aria-label="Showcase sections">
    {items.map((item) => <button key={item.value} type="button" className="ux-showcase-navigation__item" aria-current={value === item.value ? "page" : undefined} onClick={() => onChange(item.value)}>{item.icon}{item.label}</button>)}
  </nav>;
}

export interface ShowcaseHeaderProps {
  title: ComponentChildren;
  navigation: ComponentChildren;
  theme: "light" | "dark";
  onThemeChange: (theme: "light" | "dark") => void;
}

/** Hide the frame until reload without changing the examples or their state. */
export function ShowcaseHeader({ title, navigation, theme, onThemeChange }: ShowcaseHeaderProps) {
  const [minimized, setMinimized] = useState(false);
  const menuId = useId();
  if (minimized) return null;
  return <header id={menuId} className="ux-showcase-topbar">
      <div className="ux-showcase-topbar__brand">
        <span className="ux-showcase-topbar__brand-mark">DS</span>{title}
      </div>
      {navigation}
      <div className="ux-showcase-header-actions">
      <button type="button" className="ux-showcase-theme-toggle" aria-label={theme === "dark" ? "Use light theme" : "Use dark theme"} title={theme === "dark" ? "Use light theme" : "Use dark theme"} aria-pressed={theme === "dark"} onClick={() => onThemeChange(theme === "dark" ? "light" : "dark")}>
        <svg viewBox="0 0 16 16" aria-hidden="true" focusable="false">
          {theme === "dark" ? <><circle cx="8" cy="8" r="3"/><path d="M8 1v1M8 14v1M1 8h1M14 8h1M3 3l1 1M12 12l1 1M3 13l1-1M12 4l1-1"/></> : <path d="M13.5 9a5.5 5.5 0 0 1-6.5-6.5A5.5 5.5 0 1 0 13.5 9Z"/>}
        </svg>
      </button>
    <button type="button" className="ux-showcase-menu-toggle" aria-label="Minimize showcase menu" title="Minimize showcase menu (refresh to restore)" aria-controls={menuId} aria-expanded="true" onClick={() => setMinimized(true)}>
      <svg viewBox="0 0 16 16" aria-hidden="true" focusable="false">
        <path d="M4 10l4-4 4 4" />
      </svg>
    </button>
    </div>
  </header>;
}
