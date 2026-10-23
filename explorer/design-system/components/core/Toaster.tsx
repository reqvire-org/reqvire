import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "preact/hooks";
import { useSyncExternalStore } from "preact/compat";
import { type TargetedPointerEvent } from "preact";
// Sonner-derived stacking, motion, and input behavior: see toast/README.md and toast/LICENSE.sonner.md.
import { createPortal } from "preact";
import { css, cx } from "@linaria/atomic";
import { Toast } from "./Toast";
import { toastStore } from "./toast/store";
import { setToastExit, setToastLayout, setToasterLayout, setToastSwipe, transitionTime, type ExitDirection } from "./toast/geometry";
import type { ToastId, ToastPosition, ToastRecord } from "./toast/types";

export interface ToasterProps {
  /** Mount one toaster per id. Pass toasterId in toast options to target it. */
  id?: string;
  position?: ToastPosition;
  /** Maximum simultaneous visible cards; queued cards pause their remaining lifetime. */
  visibleToasts?: number;
  duration?: number;
  expand?: boolean;
  closeButton?: boolean;
  label?: string;
  className?: string;
}

const listUX = css`
  --ds-toast-width: var(--toast-width);
  --ds-toast-gap: var(--space-7);
  --ds-toast-peek: var(--space-7);
  --ds-toast-inset: var(--space-12);
  --ds-toast-lift: -1;
  --ds-toast-zero: var(--space-zero-length);
  position: fixed;
  z-index: var(--z-toast);
  width: min(var(--ds-toast-width), calc(100vw - var(--ds-toast-inset) * 2));
  height: calc(var(--ds-toast-front-height, var(--ds-toast-zero)) + var(--ds-toast-count, 0) * var(--ds-toast-peek));
  max-height: calc(100dvh - var(--ds-toast-inset) * 2);
  padding: 0;
  margin: 0;
  list-style: none;
  outline: none;
  isolation: isolate;

  &[data-expanded="true"] { height: var(--ds-toast-total-height, auto); }
  &[data-empty="true"] { pointer-events: none; }
  &[data-y="bottom"] { bottom: var(--ds-toast-inset); }
  &[data-y="top"] { top: var(--ds-toast-inset); --ds-toast-lift: 1; }
  &[data-x="right"] { right: var(--ds-toast-inset); }
  &[data-x="left"] { left: var(--ds-toast-inset); }
  &[data-x="center"] { left: 50%; transform: translateX(-50%); }

  @media (max-width: 600px) {
    --ds-toast-inset: var(--space-8);
    --ds-toast-width: calc(100vw - var(--ds-toast-inset) * 2);
  }
`;

const itemUX = css`
  --ds-toast-y: calc(var(--ds-toast-lift) * -100%);
  --ds-toast-scale: 1;
  --ds-toast-card-height: var(--ds-toast-height);
  position: absolute;
  inset-inline: 0;
  z-index: var(--ds-toast-layer);
  height: var(--ds-toast-card-height);
  opacity: 0;
  outline: none;
  transform: translate3d(var(--ds-toast-swipe-x, var(--ds-toast-zero)), calc(var(--ds-toast-y) + var(--ds-toast-swipe-y, var(--ds-toast-zero))), 0) scale(var(--ds-toast-scale));
  transition:
    transform var(--dur-toast-stack) var(--ease-toast-stack),
    height var(--dur-toast-stack) var(--ease-toast-stack),
    opacity var(--dur-toast-stack) var(--ease-toast-stack);
  touch-action: none;

  &[data-y="bottom"] { bottom: 0; }
  &[data-y="top"] { top: 0; }

  &[data-mounted="true"] { --ds-toast-y: var(--space-zero-length); opacity: 1; }

  &[data-mounted="true"][data-expanded="false"][data-front="false"] {
    --ds-toast-card-height: var(--ds-toast-front-height);
    --ds-toast-y: calc(var(--ds-toast-lift) * var(--ds-toast-index) * var(--ds-toast-peek));
    --ds-toast-scale: max(0.6, calc(1 - var(--ds-toast-index) * var(--toast-stack-scale-step)));
  }

  &[data-mounted="true"][data-expanded="true"] {
    --ds-toast-y: calc(var(--ds-toast-lift) * var(--ds-toast-offset));
  }

  .ds-toast__body { transition: opacity var(--dur-toast-stack) var(--ease-toast-stack); }

  &[data-expanded="false"][data-front="false"] .ds-toast__body { opacity: 0; }

  &[data-visible="false"] { opacity: 0; pointer-events: none; }

  &[data-swiping="true"] { transition: none; user-select: none; }

  &[data-exiting="true"] {
    opacity: 0;
    pointer-events: none;
    transform: translate3d(var(--ds-toast-exit-x), var(--ds-toast-exit-y), 0) scale(var(--ds-toast-exit-scale));
    transition: transform var(--dur-toast-stack) var(--ease-toast-stack), opacity var(--dur-toast-stack) var(--ease-toast-stack);
  }

  &[data-exiting="true"][data-swipe-out="true"] {
    transition: transform var(--dur-toast-exit) var(--ease-toast-exit), opacity var(--dur-toast-exit) var(--ease-toast-exit);
  }

  &[data-exiting="true"][data-swipe-out="false"][data-front="false"][data-expanded="false"] {
    transition: transform var(--dur-toast-rear-exit) var(--ease-toast-stack), opacity var(--dur-toast-exit) var(--ease-toast-stack);
  }

  &:focus-visible .ds-toast { box-shadow: var(--shadow-md), var(--ring-focus); }

  @media (prefers-reduced-motion: reduce) {
    transition: none;
    .ds-toast__body { transition: none; }
    &[data-exiting="true"],
    &[data-exiting="true"][data-swipe-out="true"],
    &[data-exiting="true"][data-swipe-out="false"][data-front="false"][data-expanded="false"] { transition: none; }
  }
`;

const announcementUX = css`
  position: absolute;
  width: var(--border-w);
  height: var(--border-w);
  padding: 0;
  margin: 0;
  overflow: hidden;
  clip-path: inset(50%);
  white-space: nowrap;
`;

interface ToastItemProps {
  record: ToastRecord;
  index: number;
  offset: number;
  height: number;
  visible: boolean;
  expanded: boolean;
  paused: boolean;
  duration: number;
  closeButton: boolean;
  position: ToastPosition;
  onMeasure: (id: ToastId, height: number) => void;
  onDrag: (active: boolean) => void;
}

interface Gesture {
  x: number;
  y: number;
  start: number;
  axis: "x" | "y" | null;
  delta: number;
  allowed: boolean;
}

function ToastItem({ record, index, offset, height, visible, expanded, paused, duration, closeButton, position, onMeasure, onDrag }: ToastItemProps) {
  const nodeRef = useRef<HTMLLIElement>(null);
  const cardRef = useRef<HTMLDivElement>(null);
  const [mounted, setMounted] = useState(false);
  const [exiting, setExiting] = useState(false);
  const [swiping, setSwiping] = useState(false);
  const gesture = useRef<Gesture | null>(null);
  const swipeExit = useRef<ExitDirection | null>(null);
  const remaining = useRef(duration);
  const lastIndex = useRef(index);
  const lastVisible = useRef(visible);
  if (!record.dismissed) lastVisible.current = visible;
  const isVisible = record.dismissed ? lastVisible.current : visible;
  const [y, x] = position.split("-");

  useEffect(() => {
    let second = 0;
    const first = requestAnimationFrame(() => { second = requestAnimationFrame(() => setMounted(true)); });
    return () => { cancelAnimationFrame(first); cancelAnimationFrame(second); };
  }, []);

  useLayoutEffect(() => {
    const card = cardRef.current;
    const body = card?.querySelector<HTMLElement>(".ds-toast__body");
    if (!card || !body) return;
    const measure = () => {
      const skin = getComputedStyle(card);
      onMeasure(record.id, body.offsetHeight + parseFloat(skin.borderTopWidth) + parseFloat(skin.borderBottomWidth));
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(body);
    return () => observer.disconnect();
  }, [record.id, onMeasure]);

  useLayoutEffect(() => {
    if (!record.dismissed && nodeRef.current) {
      lastIndex.current = index;
      setToastLayout(nodeRef.current, index, offset, height);
    }
  }, [index, offset, height, record.dismissed]);

  useEffect(() => { remaining.current = duration; }, [duration, record.revision]);

  useEffect(() => {
    if (paused || !visible || record.dismissed || record.variant === "loading" || !Number.isFinite(duration)) return;
    const start = performance.now();
    const timer = window.setTimeout(() => toastStore.dismiss(record.id, true), Math.max(0, remaining.current));
    return () => {
      window.clearTimeout(timer);
      remaining.current = Math.max(0, remaining.current - (performance.now() - start));
    };
  }, [paused, visible, record.dismissed, record.variant, record.id, record.revision, duration]);

  useLayoutEffect(() => {
    const node = nodeRef.current;
    if (!record.dismissed || !node) {
      setExiting(false);
      return;
    }
    setToastExit(node, swipeExit.current ?? (y as ExitDirection), !swipeExit.current && lastIndex.current > 0 && !expanded);
    setExiting(true);
  }, [record.dismissed, record.revision, y]);

  useEffect(() => {
    const node = nodeRef.current;
    if (!exiting || !node) return;
    // transitionend handles the normal path; the measured fallback covers reduced motion and interrupted entry.
    const timeout = window.setTimeout(() => toastStore.remove(record.id, record.revision), transitionTime(node) + 50);
    return () => window.clearTimeout(timeout);
  }, [exiting, record.id, record.revision]);

  const resetGesture = () => {
    gesture.current = null;
    if (nodeRef.current) setToastSwipe(nodeRef.current, 0, 0);
    setSwiping(false);
    onDrag(false);
  };

  useEffect(() => () => onDrag(false), [onDrag]);

  const move = (event: TargetedPointerEvent<HTMLLIElement>) => {
    const current = gesture.current;
    if (!current || !nodeRef.current) return;
    const dx = event.clientX - current.x;
    const dy = event.clientY - current.y;
    if (!current.axis && Math.max(Math.abs(dx), Math.abs(dy)) > 1) current.axis = Math.abs(dx) > Math.abs(dy) ? "x" : "y";
    if (!current.axis) return;
    const delta = current.axis === "x" ? dx : dy;
    const allowed = current.axis === "x"
      ? x === "center" || (x === "right" ? delta > 0 : delta < 0)
      : y === "bottom" ? delta > 0 : delta < 0;
    // A saturating resistance gives a little movement toward the interior without permitting dismissal.
    const travel = allowed ? delta : (delta * 20) / (30 + Math.abs(delta));
    current.delta = delta;
    current.allowed = allowed;
    setToastSwipe(nodeRef.current, current.axis === "x" ? travel : 0, current.axis === "y" ? travel : 0);
  };

  const accessible = isVisible && !record.dismissed && (expanded || index === 0);
  return (
    <li
      ref={nodeRef}
      className={cx("ds-toaster__item", itemUX)}
      data-toast-id={String(record.id)}
      data-toast-key={`${typeof record.id}:${record.id}`}
      data-y={y}
      data-mounted={mounted}
      data-front={(record.dismissed ? lastIndex.current : index) === 0}
      data-expanded={expanded}
      data-visible={isVisible}
      data-exiting={exiting}
      data-swipe-out={swipeExit.current !== null}
      data-swiping={swiping}
      tabIndex={accessible ? 0 : -1}
      aria-hidden={!accessible || undefined}
      inert={!accessible}
      onTransitionEnd={(event) => {
        if (exiting && event.target === event.currentTarget && event.propertyName === "opacity") toastStore.remove(record.id, record.revision);
      }}
      onPointerDown={(event) => {
        if (event.button !== 0 || !event.isPrimary || record.dismissed || record.dismissible === false || record.variant === "loading") return;
        if ((event.target as HTMLElement).closest("button, a, input, textarea, select, [contenteditable=true]")) return;
        gesture.current = { x: event.clientX, y: event.clientY, start: performance.now(), axis: null, delta: 0, allowed: false };
        swipeExit.current = null;
        event.currentTarget.setPointerCapture(event.pointerId);
        setSwiping(true);
        onDrag(true);
      }}
      onPointerMove={move}
      onPointerUp={(event) => {
        const current = gesture.current;
        if (!current) return;
        const threshold = parseFloat(getComputedStyle(event.currentTarget).getPropertyValue("--toast-swipe-threshold")) || 45;
        const distance = Math.abs(current.delta);
        const velocity = distance / Math.max(1, performance.now() - current.start);
        if (current.allowed && (distance >= threshold || velocity > 0.11)) {
          swipeExit.current = current.axis === "x" ? (current.delta > 0 ? "right" : "left") : (current.delta > 0 ? "bottom" : "top");
          gesture.current = null;
          setSwiping(false);
          onDrag(false);
          toastStore.dismiss(record.id);
        } else resetGesture();
        if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
      }}
      onPointerCancel={resetGesture}
      onLostPointerCapture={() => { if (gesture.current) resetGesture(); }}
    >
      <Toast
        ref={cardRef}
        title={record.message}
        description={record.description}
        variant={record.variant}
        action={record.action}
        cancel={record.cancel}
        closeButton={record.closeButton ?? closeButton}
        role="group"
        onDismiss={record.dismissible === false ? undefined : () => toastStore.dismiss(record.id)}
      />
    </li>
  );
}

export function Toaster({ id = "default", position = "bottom-right", visibleToasts = 3, duration = 4000, expand = false, closeButton = false, label = "Notifications", className = "" }: ToasterProps) {
  const snapshot = useSyncExternalStore(toastStore.subscribe, toastStore.getSnapshot, toastStore.getServerSnapshot);
  const records = snapshot.filter((record) => (record.toasterId ?? "default") === id);
  const active = records.filter((record) => !record.dismissed);
  const limit = Number.isFinite(visibleToasts) ? Math.max(1, Math.floor(visibleToasts)) : 3;
  const listRef = useRef<HTMLOListElement>(null);
  const lastFocus = useRef<HTMLElement | null>(null);
  const [ready, setReady] = useState(false);
  const [hovered, setHovered] = useState(false);
  const [focused, setFocused] = useState(false);
  const [dragging, setDragging] = useState(false);
  const [hidden, setHidden] = useState(false);
  const [heights, setHeights] = useState<Map<ToastId, number>>(() => new Map());
  const expanded = expand || hovered || focused;
  const [y, x] = position.split("-");
  const onMeasure = useCallback((toastId: ToastId, height: number) => {
    setHeights((current) => {
      if (Math.abs((current.get(toastId) ?? 0) - height) < 0.5) return current;
      return new Map(current).set(toastId, height);
    });
  }, []);
  const onDrag = useCallback((value: boolean) => setDragging(value), []);

  useEffect(() => {
    setReady(true);
    const visibility = () => setHidden(document.hidden);
    visibility();
    document.addEventListener("visibilitychange", visibility);
    return () => document.removeEventListener("visibilitychange", visibility);
  }, []);

  useEffect(() => {
    if (!ready) return;
    const list = listRef.current;
    return () => {
      if (list?.contains(document.activeElement)) lastFocus.current?.focus({ preventScroll: true });
      toastStore.clearScope(id);
    };
  }, [id, ready]);

  useLayoutEffect(() => {
    const list = listRef.current;
    if (!list) return;
    const gap = parseFloat(getComputedStyle(list).getPropertyValue("--ds-toast-gap")) || 14;
    const frontId = active[0]?.id ?? records[0]?.id;
    const front = frontId === undefined ? 0 : heights.get(frontId) ?? 0;
    const shown = active.slice(0, limit);
    const total = shown.reduce((sum, record) => sum + (heights.get(record.id) ?? front), 0) + Math.max(0, shown.length - 1) * gap;
    setToasterLayout(list, front, total, shown.length);
  });

  useEffect(() => {
    setHeights((current) => {
      const ids = new Set(snapshot.map((record) => record.id));
      if ([...current.keys()].every((toastId) => ids.has(toastId))) return current;
      return new Map([...current].filter(([toastId]) => ids.has(toastId)));
    });
    if (!active.length) setHovered(false);
    const list = listRef.current;
    const focusedItem = [...(list?.querySelectorAll<HTMLElement>("[data-toast-id]") ?? [])]
      .find((item) => item.contains(document.activeElement) && records.some((record) => record.dismissed && `${typeof record.id}:${record.id}` === item.dataset.toastKey));
    if (focusedItem?.contains(document.activeElement)) {
      const next = list?.querySelector<HTMLElement>('[data-exiting="false"][data-visible="true"]');
      (next ?? lastFocus.current)?.focus({ preventScroll: true });
    }
  }, [snapshot, active.length]);

  useEffect(() => {
    const hotkey = (event: KeyboardEvent) => {
      if (event.altKey && event.code === "KeyT" && listRef.current?.querySelector('[data-exiting="false"][data-visible="true"]')) {
        event.preventDefault();
        lastFocus.current = document.activeElement instanceof HTMLElement ? document.activeElement : null;
        listRef.current.focus({ preventScroll: true });
      }
    };
    document.addEventListener("keydown", hotkey);
    return () => document.removeEventListener("keydown", hotkey);
  }, []);

  if (!ready) return null;
  let offset = 0;
  const gap = listRef.current ? parseFloat(getComputedStyle(listRef.current).getPropertyValue("--ds-toast-gap")) || 14 : 14;
  return createPortal(
    <section aria-label={label}>
      <div className={cx("ds-toaster__announcements", announcementUX)} role="status" aria-live="polite" aria-atomic="false" aria-relevant="additions text">
        {active.map((record) => <div key={`${typeof record.id}:${record.id}:${record.revision}`}>{record.message} {record.description}</div>)}
      </div>
      <ol
        ref={listRef}
        className={cx("ds-toaster", listUX, className)}
        aria-label={`${label} (Alt+T)`}
        tabIndex={-1}
        data-x={x}
        data-y={y}
        data-expanded={expanded}
        data-empty={!records.length}
        onPointerEnter={(event) => { if (event.pointerType === "mouse") setHovered(true); }}
        onPointerLeave={() => { if (!dragging) setHovered(false); }}
        onFocus={(event) => {
          if (!(event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget))) lastFocus.current = event.relatedTarget instanceof HTMLElement ? event.relatedTarget : lastFocus.current;
          setFocused(true);
        }}
        onBlur={(event) => { if (!(event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget))) setFocused(false); }}
        onKeyDown={(event) => {
          if (event.key === "Escape") {
            event.preventDefault();
            lastFocus.current?.focus({ preventScroll: true });
            setFocused(false);
            setHovered(false);
          }
        }}
      >
        {records.map((record) => {
          const index = active.findIndex((item) => item.id === record.id);
          const itemOffset = offset;
          const height = heights.get(record.id) ?? 0;
          if (!record.dismissed) offset += height + gap;
          return <ToastItem
            key={`${typeof record.id}:${record.id}`}
            record={record}
            index={Math.max(0, index)}
            offset={itemOffset}
            height={height}
            visible={record.dismissed || index < limit}
            expanded={expanded}
            paused={hovered || focused || dragging || hidden}
            duration={record.duration ?? duration}
            closeButton={closeButton}
            position={position}
            onMeasure={onMeasure}
            onDrag={onDrag}
          />;
        })}
      </ol>
    </section>,
    document.body,
  );
}
