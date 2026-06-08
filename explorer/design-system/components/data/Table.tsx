import type {
  ButtonHTMLAttributes,
  HTMLAttributes,
  ReactNode,
  TableHTMLAttributes,
  TdHTMLAttributes,
  ThHTMLAttributes,
} from "react";

export type TableViewportProps = HTMLAttributes<HTMLDivElement> & { children: ReactNode };
export type TableProps = TableHTMLAttributes<HTMLTableElement> & { children: ReactNode };
export type TableHeaderProps = HTMLAttributes<HTMLTableSectionElement> & { children: ReactNode };
export type TableBodyProps = HTMLAttributes<HTMLTableSectionElement> & { children: ReactNode };
export type TableRowProps = HTMLAttributes<HTMLTableRowElement> & { children: ReactNode; selected?: boolean };
export type TableHeadProps = ThHTMLAttributes<HTMLTableCellElement> & { children: ReactNode };
export type TableCellProps = TdHTMLAttributes<HTMLTableCellElement> & { children: ReactNode };
export type TableSortButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  children: ReactNode;
  direction?: "asc" | "desc";
};

export function TableViewport({
  children,
  className = "",
  ...props
}: TableViewportProps) {
  return (
    <div className={["rq-tablewrap", className].filter(Boolean).join(" ")} {...props}>
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
    <table className={["rq-table", className].filter(Boolean).join(" ")} {...props}>
      {children}
    </table>
  );
}

export function TableHeader({
  children,
  className = "",
  ...props
}: TableHeaderProps) {
  return (
    <thead className={className} {...props}>
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
    <tbody className={className} {...props}>
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
    <tr className={[selected ? "is-selected" : "", className].filter(Boolean).join(" ")} {...props}>
      {children}
    </tr>
  );
}

export function TableHead({
  children,
  className = "",
  ...props
}: TableHeadProps) {
  return (
    <th className={className} {...props}>
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
    <td className={className} {...props}>
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
    <button type="button" className={["rq-table__sort", className].filter(Boolean).join(" ")} {...props}>
      <span>{children}</span>
      {direction ? <span className="rq-table__sortdir">{direction}</span> : null}
    </button>
  );
}
