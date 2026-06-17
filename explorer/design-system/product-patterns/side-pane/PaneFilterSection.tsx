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
  paneNavListClass,
  paneNavRowClass,
  paneNavRowCountClass,
  paneNavRowIconClass,
  paneNavRowLabelClass,
  paneSectionLabelClass,
} from "./classes";

export interface PaneFilterSectionProps extends Omit<HTMLAttributes<HTMLElement>, "title" | "style"> {
  title?: ReactNode;
}

export function PaneFilterSection({
  title,
  className = "",
  children,
  ...props
}: PaneFilterSectionProps) {
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
}

export function PaneFilterNavRow({
  icon,
  label,
  count,
  className = "",
  ...props
}: PaneFilterNavRowProps) {
  return (
    <button type="button" className={cx("ux-pane-nav-row", paneNavRowClass, className)} {...props}>
      <span className={cx("ux-pane-nav-row__icon", paneNavRowIconClass)} aria-hidden="true">
        <Icon name={icon} />
      </span>
      <span className={cx("ux-pane-nav-row__label", paneNavRowLabelClass)}>{label}</span>
      <Badge className={cx("ux-pane-nav-row__count", paneNavRowCountClass)}>{count}</Badge>
    </button>
  );
}
