import { cx } from "@linaria/atomic";
import { type AsideHTMLAttributes, type ComponentChildren } from "preact";
import { Icon } from "../../components/core/Icon";
import {
  appRootClass,
  baseUX,
  sideContentAppClass,
  sideContentClass,
  skinX,
  treeTabClass,
  treeTabLabelClass,
  treeTabMarkClass,
  treeTabToggleClass,
} from "./classes";

export interface SidePaneFrameProps extends Omit<(AsideHTMLAttributes<HTMLElement> & { className?: string }), "style"> {
  open: boolean;
  chrome?: "standalone" | "app";
  header?: ComponentChildren;
  railMark?: ComponentChildren;
  tabLabel?: ComponentChildren;
  onToggle: () => void;
}

export function SidePaneFrame({
  open,
  chrome = "standalone",
  header,
  railMark,
  tabLabel = "Explorer",
  onToggle,
  className = "",
  children,
  ...props
}: SidePaneFrameProps) {
  const appChrome = chrome === "app";
  const showStandaloneChrome = chrome === "standalone";

  return (
    <aside
      className={cx(
        "ux-side-pane",
        baseUX,
        skinX,
        appChrome ? appRootClass : "is-standalone",
        !open && "is-collapsed",
        className,
      )}
      {...props}
    >
      <div className={cx("ux-side-content", sideContentClass, appChrome && sideContentAppClass)}>
        {showStandaloneChrome ? header : null}
        {children}
      </div>
      {showStandaloneChrome ? (
        <button
          type="button"
          className={cx("ux-tree-tab", treeTabClass)}
          aria-label={open ? "Collapse explorer pane" : "Expand explorer pane"}
          aria-expanded={open}
          onClick={onToggle}
        >
          {railMark != null ? <span className={cx("ux-tree-tab-mark", treeTabMarkClass)}>{railMark}</span> : null}
          <span className={cx("ux-tree-tab-label", treeTabLabelClass)}>{tabLabel}</span>
          <span className={cx("ux-tree-toggle", treeTabToggleClass)} aria-hidden="true">
            {open ? <Icon name="chevron-left" /> : <Icon name="chevron-right" />}
          </span>
        </button>
      ) : null}
    </aside>
  );
}
