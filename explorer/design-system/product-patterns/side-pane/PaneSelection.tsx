import { cx } from "@linaria/atomic";
import type { ReactNode } from "react";
import { Icon } from "../../components/core/Icon";
import { IconButton } from "../../components/core/IconButton";
import {
  emptyClass,
  graphControlSwatchClass,
  paneSelectedElementClass,
  paneSelectedElementLinkClass,
  paneSelectionHintClass,
  paneSelectionKindClass,
  paneSelectionNameClass,
  paneSelectionOpenClass,
  paneSelectionRowClass,
} from "./classes";
import { PaneFilterGroup } from "./PaneFilterSection";

export interface PaneSelectionValue {
  icon?: ReactNode;
  name: ReactNode;
  kind?: ReactNode;
}

export interface PaneSelectionProps {
  ariaLabel: string;
  emptyMessage: ReactNode;
  selection?: PaneSelectionValue;
  openTitle?: string;
  clearLabel?: string;
  onOpen?: () => void;
  onClear?: () => void;
}

export function PaneSelection({
  ariaLabel,
  emptyMessage,
  selection,
  openTitle = "Open details",
  clearLabel = "Clear selection",
  onOpen,
  onClear,
}: PaneSelectionProps) {
  return (
    <section className={cx("ux-pane-selected-element", paneSelectedElementClass)} aria-label={ariaLabel}>
      <PaneFilterGroup label="Selection">
        {selection == null ? (
          <p className={cx(emptyClass, "ux-pane-selection-hint", paneSelectionHintClass)}>{emptyMessage}</p>
        ) : (
          <div className={cx("ux-pane-selection-row", paneSelectionRowClass)}>
            <button
              type="button"
              className={cx("ux-pane-selected-element-link", paneSelectedElementLinkClass)}
              onClick={onOpen}
              title={openTitle}
            >
              {selection.icon ?? <span className={cx("ux-graph-control-swatch", graphControlSwatchClass)} />}
              <span className={cx("ux-pane-selection-name", paneSelectionNameClass)}>{selection.name}</span>
              {selection.kind != null ? (
                <span className={cx("ux-pane-selection-kind", paneSelectionKindClass)}>{selection.kind}</span>
              ) : null}
              <Icon name="arrow-up-right" size={13} className={cx("ux-pane-selection-open", paneSelectionOpenClass)} />
            </button>
            {onClear ? (
              <IconButton size="sm" tone="ghost" aria-label={clearLabel} title={clearLabel} onClick={onClear}>
                <Icon name="x" />
              </IconButton>
            ) : null}
          </div>
        )}
      </PaneFilterGroup>
    </section>
  );
}
