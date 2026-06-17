import type {
  ButtonHTMLAttributes,
  HTMLAttributes,
  ReactNode,
  TableHTMLAttributes,
  TdHTMLAttributes,
  ThHTMLAttributes,
} from "react";
import { css, cx } from "@linaria/atomic";

const baseUXViewport = css`
  min-height: 0;
  overflow: auto;
`;

const skinXViewport = css`
  background: var(--ds-tablewrap-bg, var(--bg-surface));
  border: var(--ds-tablewrap-border, var(--border-w) solid var(--border-subtle));
  border-radius: var(--ds-tablewrap-radius, var(--radius-lg));
`;

const baseUXTable = css`
  display: table;
  width: 100%;
  min-width: var(--ds-table-min-w, var(--content-max));
  border-collapse: collapse;
  font-size: var(--text-sm);
`;

const skinXTable = css`
  color: var(--text-body);
`;

const baseUXHeader = css`
  display: table-header-group;
`;

const baseUXBody = css`
  display: table-row-group;
`;

const baseUXRow = css`
  display: table-row;
`;

const skinXRow = css`
  &:hover td {
    background: var(--ds-table-row-hover-bg, var(--bg-hover));
  }

  &.is-selected td {
    color: var(--text-strong);
    background: var(--ds-table-sel-bg, var(--bg-selected));
  }
`;

const baseUXHead = css`
  position: sticky;
  top: 0;
  z-index: var(--z-local-base);
  display: table-cell;
  padding: var(--space-5) var(--space-7);
  text-align: left;

  &:has(.ds-table__sort) {
    padding: 0;
  }
`;

const skinXHead = css`
  color: var(--text-secondary);
  background: var(--ds-table-th-bg, var(--bg-sunken));
  border-bottom: var(--border-w) solid var(--ds-table-th-border, var(--border-subtle));
  font-weight: var(--ds-table-th-fw, var(--weight-semibold));
`;

const baseUXCell = css`
  display: table-cell;
  padding: var(--ds-table-td-p, var(--space-5) var(--space-7));
  vertical-align: middle;
`;

const skinXCell = css`
  border-bottom: var(--border-w) solid var(--ds-table-td-border, var(--border-subtle));

  tr:last-child & {
    border-bottom: 0;
  }
`;

const baseUXSort = css`
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-5);
  padding: var(--space-5) var(--space-7);
  border: 0;
  text-align: left;
  font: inherit;
  cursor: pointer;

  .ds-table__sortdir {
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
  }
`;

const skinXSort = css`
  color: inherit;
  background: transparent;

  &:hover {
    color: var(--text-strong);
  }

  .ds-table__sortdir {
    color: var(--text-strong);
  }
`;

export type TableViewportProps = Omit<HTMLAttributes<HTMLDivElement>, "style"> & { children: ReactNode };
export type TableProps = Omit<TableHTMLAttributes<HTMLTableElement>, "style"> & { children: ReactNode };
export type TableHeaderGroupProps = Omit<HTMLAttributes<HTMLTableSectionElement>, "style"> & { children: ReactNode };
export type TableBodyProps = Omit<HTMLAttributes<HTMLTableSectionElement>, "style"> & { children: ReactNode };
export type TableRowProps = Omit<HTMLAttributes<HTMLTableRowElement>, "style"> & { children: ReactNode; selected?: boolean };
export type TableHeaderCellProps = Omit<ThHTMLAttributes<HTMLTableCellElement>, "style"> & { children: ReactNode };
export type TableCellProps = Omit<TdHTMLAttributes<HTMLTableCellElement>, "style"> & { children: ReactNode };
export type TableSortButtonProps = Omit<ButtonHTMLAttributes<HTMLButtonElement>, "style"> & {
  children: ReactNode;
  direction?: "asc" | "desc";
};

export function TableViewport({
  children,
  className = "",
  ...props
}: TableViewportProps) {
  return (
    <div className={cx("ds-tablewrap", baseUXViewport, skinXViewport, className)} {...props}>
      {children}
    </div>
  );
}

export function Table({
  children,
  className = "",
  ...props
}: TableProps) {
  return (
    <table className={cx("ds-table", baseUXTable, skinXTable, className)} {...props}>
      {children}
    </table>
  );
}

export function TableHeaderGroup({
  children,
  className = "",
  ...props
}: TableHeaderGroupProps) {
  return (
    <thead className={cx(baseUXHeader, className)} {...props}>
      {children}
    </thead>
  );
}

export function TableBody({
  children,
  className = "",
  ...props
}: TableBodyProps) {
  return (
    <tbody className={cx(baseUXBody, className)} {...props}>
      {children}
    </tbody>
  );
}

export function TableRow({
  children,
  selected = false,
  className = "",
  ...props
}: TableRowProps) {
  return (
    <tr className={cx(baseUXRow, skinXRow, selected ? "is-selected" : undefined, className)} {...props}>
      {children}
    </tr>
  );
}

export function TableHeaderCell({
  children,
  className = "",
  ...props
}: TableHeaderCellProps) {
  return (
    <th className={cx(baseUXHead, skinXHead, className)} {...props}>
      {children}
    </th>
  );
}

export function TableCell({
  children,
  className = "",
  ...props
}: TableCellProps) {
  return (
    <td className={cx(baseUXCell, skinXCell, className)} {...props}>
      {children}
    </td>
  );
}

export function TableSortButton({
  children,
  direction,
  className = "",
  ...props
}: TableSortButtonProps) {
  return (
    <button type="button" className={cx("ds-table__sort", baseUXSort, skinXSort, className)} {...props}>
      <span>{children}</span>
      {direction ? <span className="ds-table__sortdir">{direction}</span> : null}
    </button>
  );
}
