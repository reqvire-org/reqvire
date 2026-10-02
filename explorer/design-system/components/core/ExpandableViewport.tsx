import { useEffect, useLayoutEffect, useRef, useState, type ReactNode } from "react";
import { createPortal } from "react-dom";
import { css, cx } from "@linaria/atomic";
import { useLatestRef } from "../../hooks/useLatestRef";

export interface ExpandableViewportProps {
  expanded: boolean;
  onExpandedChange: (expanded: boolean) => void;
  label: string;
  children: ReactNode;
}

const slotUX = css`
  display: flex;
  flex-direction: column;
  height: 100%;
  min-width: 0;
  min-height: 0;
`;
const hostUX = css`display: contents;`;
const frameUX = css`
  display: flex;
  flex-direction: column;
  height: 100%;
  min-width: 0;
  min-height: 0;

  &[data-expanded="true"] {
    position: fixed;
    inset: 0;
    z-index: var(--z-overlay);
    isolation: isolate;
    overflow: hidden;
    overscroll-behavior: contain;
    background: var(--bg-canvas);
    color: var(--text-body);
  }
`;

/** Relocate one portal host so expanding a canvas preserves its mounted renderer. */
export function ExpandableViewport({ expanded, onExpandedChange, label, children }: ExpandableViewportProps) {
  const slot = useRef<HTMLDivElement>(null);
  const frame = useRef<HTMLDivElement>(null);
  const returnFocus = useRef<HTMLElement | null>(null);
  const changeExpanded = useLatestRef(onExpandedChange);
  const [host] = useState(() => {
    const element = document.createElement("div");
    element.className = hostUX;
    return element;
  });

  useLayoutEffect(() => {
    if (expanded) {
      returnFocus.current = frame.current?.querySelector<HTMLElement>("[data-expanded-close]")
        ?? (document.activeElement instanceof HTMLElement ? document.activeElement : null);
      document.body.appendChild(host);
      (frame.current?.querySelector<HTMLElement>("[data-expanded-close]") ?? frame.current)?.focus();
    } else {
      slot.current?.appendChild(host);
      if (returnFocus.current?.isConnected) returnFocus.current.focus();
      returnFocus.current = null;
    }
  }, [expanded, host]);
  useLayoutEffect(() => () => host.remove(), [host]);

  useEffect(() => {
    if (!expanded) return;
    const onKeyDown = (event: KeyboardEvent) => {
      const element = frame.current;
      if (!element || event.defaultPrevented) return;
      // Shared element details use the modal layer above this full-page viewport.
      if ([...document.querySelectorAll('[role="dialog"][aria-modal="true"]')].some(dialog => dialog !== element)) return;
      if (event.key === "Escape") {
        event.preventDefault();
        changeExpanded.current(false);
      } else if (event.key === "Tab") {
        const targets = [...element.querySelectorAll<HTMLElement>("a[href], button, input, select, textarea, [tabindex]")]
          .filter(target => target.tabIndex >= 0 && !target.matches(":disabled") && !target.closest("[hidden], [inert]"));
        const first = targets[0] ?? element;
        const last = targets.at(-1) ?? element;
        if (!element.contains(document.activeElement) || document.activeElement === element) {
          event.preventDefault();
          (event.shiftKey ? last : first).focus();
        } else if (event.shiftKey && document.activeElement === first) {
          event.preventDefault();
          last.focus();
        } else if (!event.shiftKey && document.activeElement === last) {
          event.preventDefault();
          first.focus();
        }
      }
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [expanded, changeExpanded]);

  return <div className={cx("ds-expandable-viewport", slotUX)} ref={slot}>
    {createPortal(<div ref={frame} className={frameUX} data-expanded={expanded}
      role={expanded ? "dialog" : undefined} aria-modal={expanded || undefined}
      aria-label={expanded ? label : undefined} tabIndex={expanded ? -1 : undefined}>
      {children}
    </div>, host)}
  </div>;
}
