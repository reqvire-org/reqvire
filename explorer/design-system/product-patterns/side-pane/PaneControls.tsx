import { cx } from "@linaria/atomic";
import type {
  ButtonHTMLAttributes,
  FormEventHandler,
  HTMLAttributes,
  ReactNode,
} from "react";
import { Badge } from "../../components/core/Badge";
import { Icon, type IconName } from "../../components/core/Icon";
import { SearchInput } from "../../components/controls/SearchInput";
import {
  globalSearchClass,
  globalSearchControlClass,
  globalSearchResultsClass,
  paneControlsClass,
  paneControlsTitleClass,
  paneFilterGridClass,
  paneFilterGridTwoClass,
  paneNavListClass,
  paneNavRowClass,
  paneNavRowCountClass,
  paneNavRowIconClass,
  paneNavRowLabelClass,
  paneSectionLabelClass,
} from "./classes";

export interface PaneControlSectionProps extends Omit<HTMLAttributes<HTMLElement>, "title" | "style"> {
  title?: ReactNode;
}

export function PaneControlSection({
  title,
  className = "",
  children,
  ...props
}: PaneControlSectionProps) {
  return (
    <section className={cx("ux-pane-controls", paneControlsClass, className)} {...props}>
      {title != null ? <h2 className={cx("ux-pane-controls-title", paneControlsTitleClass)}>{title}</h2> : null}
      {children}
    </section>
  );
}

export interface PaneFilterGroupProps {
  label: ReactNode;
  children?: ReactNode;
}

export function PaneFilterGroup({ label, children }: PaneFilterGroupProps) {
  return (
    <>
      <span className={cx("ux-pane-section-label", paneSectionLabelClass)}>{label}</span>
      {children}
    </>
  );
}

export type PaneFilterGridProps = Omit<HTMLAttributes<HTMLDivElement>, "style"> & {
  columns?: "auto" | "two";
};

export function PaneFilterGrid({ className = "", columns = "auto", ...props }: PaneFilterGridProps) {
  return (
    <div
      className={cx(
        "ux-pane-filter-grid",
        paneFilterGridClass,
        columns === "two" && paneFilterGridTwoClass,
        className,
      )}
      {...props}
    />
  );
}

export interface PaneSearchFormProps {
  searchInputId: string;
  inputLabel: string;
  placeholder?: string;
  value: string;
  resultsId?: string;
  onQueryChange: (query: string) => void;
  onSubmit?: FormEventHandler<HTMLFormElement>;
  children?: ReactNode;
}

export function PaneSearchForm({
  searchInputId,
  inputLabel,
  placeholder,
  value,
  resultsId,
  onQueryChange,
  onSubmit,
  children,
}: PaneSearchFormProps) {
  return (
    <form className={cx("ux-global-search", globalSearchClass)} role="search" onSubmit={onSubmit}>
      <SearchInput
        id={searchInputId}
        className={cx("ux-global-search-control", globalSearchControlClass)}
        size="lg"
        aria-label={inputLabel}
        type="search"
        autoComplete="off"
        spellCheck={false}
        placeholder={placeholder}
        value={value}
        onChange={(event) => onQueryChange(event.target.value)}
      />
      <ul id={resultsId} className={cx("ontology-graph-results", globalSearchResultsClass)}>
        {children}
      </ul>
    </form>
  );
}

export type PaneFilterNavListProps = Omit<HTMLAttributes<HTMLDivElement>, "style">;

export function PaneFilterNavList({ className = "", ...props }: PaneFilterNavListProps) {
  return <div className={cx("ux-pane-nav-list", paneNavListClass, className)} {...props} />;
}

export interface PaneFilterNavRowProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "style"> {
  icon: IconName;
  label: ReactNode;
  count: ReactNode;
  selected?: boolean;
}

export function PaneFilterNavRow({
  icon,
  label,
  count,
  selected = false,
  className = "",
  ...props
}: PaneFilterNavRowProps) {
  return (
    <button
      type="button"
      className={cx("ux-pane-nav-row", paneNavRowClass, selected && "is-selected", className)}
      aria-current={selected ? "page" : undefined}
      {...props}
    >
      <span className={cx("ux-pane-nav-row__icon", paneNavRowIconClass)} aria-hidden="true">
        <Icon name={icon} />
      </span>
      <span className={cx("ux-pane-nav-row__label", paneNavRowLabelClass)}>{label}</span>
      <Badge className={cx("ux-pane-nav-row__count", paneNavRowCountClass)}>{count}</Badge>
    </button>
  );
}
