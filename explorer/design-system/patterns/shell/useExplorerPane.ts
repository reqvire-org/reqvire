import { useEffect, useRef, useState } from "preact/hooks";
import { type TargetedKeyboardEvent, type TargetedPointerEvent } from "preact";

const LEFT_PANE_WIDTH_DEFAULT = 380;
const LEFT_PANE_WIDTH_MIN = 300;
const LEFT_PANE_WIDTH_MAX = 720;
const LEFT_PANE_WIDTH_STORAGE_KEY = "reqvire:explorer:left-pane-width";

export function useExplorerPane() {
  const [leftPaneOpen, setLeftPaneOpen] = useState(true);
  const [leftPaneResizing, setLeftPaneResizing] = useState(false);
  const [leftPaneWidth, setLeftPaneWidth] = useState(readStoredLeftPaneWidth);
  const shellRef = useRef<HTMLDivElement | null>(null);
  const cancelDrag = useRef<(() => void) | null>(null);
  const leftPaneWidthRef = useRef(leftPaneWidth);
  useEffect(() => {
    leftPaneWidthRef.current = leftPaneWidth;
    shellRef.current?.style.setProperty("--ux-left-pane-width", `${leftPaneWidth}px`);
    try { window.localStorage.setItem(LEFT_PANE_WIDTH_STORAGE_KEY, String(leftPaneWidth)); }
    catch { /* Sandboxed Apps and restricted browsers retain session-local geometry. */ }
  }, [leftPaneWidth]);
  useEffect(() => () => cancelDrag.current?.(), []);

  useEffect(() => {
    function handleResize() {
      setLeftPaneWidth((width) => clampLeftPaneWidth(width));
    }

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  function toggleLeftPane() {
    setLeftPaneOpen((open) => !open);
  }

  function handleLeftPaneResizePointerDown(event: TargetedPointerEvent<HTMLDivElement>) {
    if (!leftPaneOpen || event.button !== 0) return;
    cancelDrag.current?.();

    const startX = event.clientX;
    const startWidth = leftPaneWidthRef.current;
    let nextWidth = startWidth;
    setLeftPaneResizing(true);
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";

    function handlePointerMove(moveEvent: PointerEvent) {
      const delta = moveEvent.clientX - startX;
      nextWidth = clampLeftPaneWidth(startWidth + delta);
      shellRef.current?.style.setProperty("--ux-left-pane-width", `${nextWidth}px`);
    }

    function clearPointerDrag() {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", finishPointerDrag);
      window.removeEventListener("pointercancel", finishPointerDrag);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
      cancelDrag.current = null;
    }

    function finishPointerDrag() {
      clearPointerDrag();
      setLeftPaneResizing(false);
      leftPaneWidthRef.current = nextWidth;
      setLeftPaneWidth(nextWidth);
    }

    window.addEventListener("pointermove", handlePointerMove);
    window.addEventListener("pointerup", finishPointerDrag);
    window.addEventListener("pointercancel", finishPointerDrag);
    cancelDrag.current = clearPointerDrag;
  }

  function handleLeftPaneResizeKeyDown(event: TargetedKeyboardEvent<HTMLDivElement>) {
    if (!leftPaneOpen) return;

    if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
      event.preventDefault();
      const direction = event.key === "ArrowLeft" ? -1 : 1;
      const step = event.shiftKey ? 40 : 16;
      setLeftPaneWidth((width) => clampLeftPaneWidth(width + direction * step));
    }
  }

  return { leftPaneOpen, leftPaneResizing, leftPaneWidth, shellRef, toggleLeftPane,
    handleLeftPaneResizePointerDown, handleLeftPaneResizeKeyDown,
    minWidth: LEFT_PANE_WIDTH_MIN, maxWidth: LEFT_PANE_WIDTH_MAX };
}

function readStoredLeftPaneWidth() {
  if (typeof window === "undefined") return LEFT_PANE_WIDTH_DEFAULT;

  try {
    const value = window.localStorage.getItem(LEFT_PANE_WIDTH_STORAGE_KEY);
    const stored = value === null ? LEFT_PANE_WIDTH_DEFAULT : Number(value);
    return clampLeftPaneWidth(Number.isFinite(stored) ? stored : LEFT_PANE_WIDTH_DEFAULT);
  } catch {
    return clampLeftPaneWidth(LEFT_PANE_WIDTH_DEFAULT);
  }
}

function clampLeftPaneWidth(width: number) {
  const viewportMax =
    typeof window === "undefined"
      ? LEFT_PANE_WIDTH_MAX
      : Math.max(
          LEFT_PANE_WIDTH_MIN,
          Math.min(LEFT_PANE_WIDTH_MAX, window.innerWidth - 420),
        );

  return Math.round(
    Math.min(Math.max(width, LEFT_PANE_WIDTH_MIN), viewportMax),
  );
}
