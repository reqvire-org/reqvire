import { cx } from "@linaria/atomic";
import type { ReactNode } from "react";
import {
  paneLegendClass,
  paneLegendRowClass,
  paneLegendTextClass,
  paneSymbolClass,
} from "./classes";

export interface PaneNotationLegendRow {
  symbol: ReactNode;
  label: ReactNode;
}

export interface PaneNotationLegendProps {
  rows: readonly PaneNotationLegendRow[];
}

export function PaneNotationLegend({ rows }: PaneNotationLegendProps) {
  return (
    <div className={cx("ux-pane-legend", paneLegendClass)}>
      {rows.map((row, index) => (
        <div key={index} className={cx("ux-pane-legend-row", paneLegendRowClass)}>
          <span className={cx("ux-pane-symbol", paneSymbolClass)}>{row.symbol}</span>
          <span className={cx("ux-pane-legend-text", paneLegendTextClass)}>{row.label}</span>
        </div>
      ))}
    </div>
  );
}
