import { cx } from "@linaria/atomic";
import type { ReactNode } from "react";
import { ToggleRow } from "../../components/controls/ToggleRow";
import type { DesignSystemColorToken } from "../../palette";
import { paneLegendClass } from "./classes";

export interface PaneLegendRow {
  id: string;
  label: ReactNode;
  colorToken: DesignSystemColorToken;
  line?: boolean;
}

export interface PaneLegendProps {
  rows: readonly PaneLegendRow[];
}

export function PaneLegend({ rows }: PaneLegendProps) {
  return (
    <div className={cx("ux-pane-legend", paneLegendClass)}>
      {rows.map((row) => (
        <ToggleRow
          key={row.id}
          label={row.label}
          colorToken={row.colorToken}
          line={row.line}
          static
        />
      ))}
    </div>
  );
}
