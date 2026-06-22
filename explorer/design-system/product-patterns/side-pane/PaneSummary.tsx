import { cx } from "@linaria/atomic";
import type { ReactNode } from "react";
import { SidebarSection } from "../../components/navigation/SidebarSection";
import { paneSummaryClass, paneSummaryFooterClass, summaryClass } from "./classes";

export interface PaneSummaryItem {
  label: string;
  value: ReactNode;
  title?: string;
}

export interface PaneSummaryProps {
  title?: string;
  ariaLabel?: string;
  items: readonly PaneSummaryItem[];
  placement?: "default" | "footer";
}

export function PaneSummary({
  title = "Summary",
  ariaLabel = title,
  items,
  placement = "default",
}: PaneSummaryProps) {
  return (
    <SidebarSection
      title={title}
      className={cx(
        "ux-pane-summary",
        paneSummaryClass,
        placement === "footer" && "ux-pane-summary--footer",
        placement === "footer" && paneSummaryFooterClass,
      )}
      aria-label={ariaLabel}
    >
      <div className={cx("ux-summary", summaryClass)}>
        {items.map((item) => (
          <span key={item.label} className="ux-summary__item" title={item.title}>
            <span className="ux-summary__label">{item.label}</span>
            <span className="ux-summary__value">{item.value}</span>
          </span>
        ))}
      </div>
    </SidebarSection>
  );
}
