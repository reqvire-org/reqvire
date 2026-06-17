import { cx } from "@linaria/atomic";
import type { ReactNode } from "react";
import { Stat, StatRow } from "../../components/data/Stat";
import { SidebarSection } from "../../components/navigation/SidebarSection";
import { paneSummaryClass, summaryClass } from "./classes";

export interface PaneSummaryItem {
  label: string;
  value: ReactNode;
  title?: string;
}

export interface PaneSummaryProps {
  title?: string;
  ariaLabel?: string;
  items: readonly PaneSummaryItem[];
}

export function PaneSummary({ title = "Summary", ariaLabel = title, items }: PaneSummaryProps) {
  return (
    <SidebarSection title={title} className={cx("ux-pane-summary", paneSummaryClass)} aria-label={ariaLabel}>
      <StatRow className={cx("ux-summary", summaryClass)}>
        {items.map((item) => (
          <Stat key={item.label} label={item.label} value={item.value} title={item.title} />
        ))}
      </StatRow>
    </SidebarSection>
  );
}
