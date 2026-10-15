/**
 * Private measured-geometry boundary. Only computed positions, dimensions and
 * stacking order cross into CSS; appearance and motion remain token-owned.
 * The architecture guard permits these exact custom-property writes only.
 */
function pixels(value: number) {
  return `${Number.isFinite(value) ? value : 0}px`;
}

export function setToastLayout(node: HTMLElement, index: number, offset: number, height: number) {
  node.style.setProperty("--ds-toast-index", String(index));
  node.style.setProperty("--ds-toast-layer", String(1000 - index));
  node.style.setProperty("--ds-toast-offset", pixels(offset));
  node.style.setProperty("--ds-toast-height", pixels(height));
}

export function setToasterLayout(node: HTMLElement, frontHeight: number, totalHeight: number, count: number) {
  node.style.setProperty("--ds-toast-front-height", pixels(frontHeight));
  node.style.setProperty("--ds-toast-total-height", pixels(totalHeight));
  node.style.setProperty("--ds-toast-count", String(Math.max(0, count - 1)));
}

export function setToastSwipe(node: HTMLElement, x: number, y: number) {
  node.style.setProperty("--ds-toast-swipe-x", pixels(x));
  node.style.setProperty("--ds-toast-swipe-y", pixels(y));
}

export type ExitDirection = "left" | "right" | "top" | "bottom";

export function setToastExit(node: HTMLElement, direction: ExitDirection, collapsedRear = false) {
  const matrix = new DOMMatrixReadOnly(getComputedStyle(node).transform);
  const rect = node.getBoundingClientRect();
  const x = matrix.m41 + (direction === "left" ? -rect.width : direction === "right" ? rect.width : 0);
  const y = collapsedRear ? node.offsetHeight * 0.4 : matrix.m42 + (direction === "top" ? -rect.height : direction === "bottom" ? rect.height : 0);
  node.style.setProperty("--ds-toast-exit-x", pixels(x));
  node.style.setProperty("--ds-toast-exit-y", pixels(y));
  node.style.setProperty("--ds-toast-exit-scale", String(collapsedRear ? 1 : matrix.m11));
}

export function transitionTime(node: HTMLElement) {
  const computed = getComputedStyle(node);
  const milliseconds = (value: string) => parseFloat(value) * (value.endsWith("ms") ? 1 : 1000);
  const durations = computed.transitionDuration.split(",").map(milliseconds);
  const delays = computed.transitionDelay.split(",").map(milliseconds);
  return Math.max(0, ...durations.map((duration, index) => duration + delays[index % delays.length]));
}
