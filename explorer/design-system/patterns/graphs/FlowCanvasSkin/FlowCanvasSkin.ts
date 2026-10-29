import { css, cx } from "@linaria/atomic";

// Adapted from @xyflow/react 12.12.0 (MIT), copyright 2019–2025 webkid GmbH.
// See LICENSE.xyflow.md. JavaScript remains an installed dependency. Only the
// shared structure used by Reqvire's custom-node canvases is owned here.
// The base layer lets each graph's existing skin retain cascade precedence.
const canvasUX = css`
  @layer base {
    direction: ltr;
    .react-flow__container { position: absolute; width: 100%; height: 100%; top: 0; left: 0; }
    .react-flow__pane { z-index: var(--z-local-base); touch-action: none; }
    .react-flow__pane.draggable { cursor: grab; }
    .react-flow__pane.dragging { cursor: grabbing; }
    .react-flow__pane.selection { cursor: pointer; }
    .react-flow__viewport { transform-origin: 0 0; z-index: var(--z-local-raised); pointer-events: none; }
    .react-flow__renderer { z-index: var(--z-local-raised); }
    .react-flow__selection { z-index: var(--z-local-overlay); }
    .react-flow__edges { position: absolute; }
    .react-flow__edges svg { overflow: visible; position: absolute; pointer-events: none; }
    .react-flow__edge { pointer-events: visibleStroke; }
    .react-flow__edge.selectable { cursor: pointer; }
    .react-flow__edge.inactive { pointer-events: none; }
    .react-flow__edge-textwrapper { pointer-events: all; }
    .react-flow__edge-text { pointer-events: none; user-select: none; }
    .react-flow__connection { pointer-events: none; }
    svg.react-flow__connectionline { position: absolute; overflow: visible; z-index: var(--z-local-overlay); }
    .react-flow__nodes { pointer-events: none; transform-origin: 0 0; }
    .react-flow__node { position: absolute; user-select: none; pointer-events: all; transform-origin: 0 0; box-sizing: border-box; cursor: default; }
    .react-flow__node.selectable { cursor: pointer; }
    .react-flow__node.draggable { cursor: grab; pointer-events: all; }
    .react-flow__node.draggable.dragging { cursor: grabbing; }
    .react-flow__nodesselection { z-index: var(--z-local-overlay); transform-origin: left top; pointer-events: none; }
    .react-flow__nodesselection-rect { position: absolute; pointer-events: all; cursor: grab; }
    .react-flow__handle { position: absolute; pointer-events: none; min-width: var(--space-3); min-height: var(--space-3); width: var(--space-3); height: var(--space-3); }
    .react-flow__handle.connectingfrom { pointer-events: all; }
    .react-flow__handle.connectionindicator { pointer-events: all; cursor: crosshair; }
    .react-flow__handle-bottom { top: auto; left: 50%; bottom: 0; transform: translate(-50%, 50%); }
    .react-flow__handle-top { top: 0; left: 50%; transform: translate(-50%, -50%); }
    .react-flow__handle-left { top: 50%; left: 0; transform: translate(-50%, -50%); }
    .react-flow__handle-right { top: 50%; right: 0; transform: translate(50%, -50%); }
    .react-flow__edgeupdater { cursor: move; pointer-events: all; }
    .react-flow__pane.selection .react-flow__panel { pointer-events: none; }
    .react-flow__panel { position: absolute; z-index: var(--z-local-overlay); margin: var(--space-8); }
    .react-flow__panel.top { top: 0; }
    .react-flow__panel.bottom { bottom: 0; }
    .react-flow__panel.left { left: 0; }
    .react-flow__panel.right { right: 0; }
    .react-flow__panel.top.center, .react-flow__panel.bottom.center { left: 50%; transform: translateX(calc(-1 * var(--space-8))) translateX(-50%); }
    .react-flow__panel.left.center, .react-flow__panel.right.center { top: 50%; transform: translateY(calc(-1 * var(--space-8))) translateY(-50%); }
    .react-flow__edgelabel-renderer, .react-flow__viewport-portal { position: absolute; width: 100%; height: 100%; left: 0; top: 0; user-select: none; }
    .react-flow__edgelabel-renderer { pointer-events: none; }
  }
`;

const canvasSkin = css`
  @layer base {
    background-color: transparent;
    .react-flow__edge-path, .react-flow__connection-path { stroke: var(--text-muted); stroke-width: var(--border-w); fill: none; }
    .react-flow__edge.selected .react-flow__edge-path, .react-flow__edge:focus-visible .react-flow__edge-path { stroke: var(--accent); }
    .react-flow__arrowhead polyline { stroke: var(--text-muted); }
    .react-flow__arrowhead polyline.arrowclosed { fill: var(--text-muted); }
    .react-flow__edge-text { font-size: var(--text-caption); fill: var(--text-body); }
    .react-flow__edge-textbg { fill: var(--bg-surface); }
    .react-flow__handle { background-color: var(--text-strong); border: var(--border-w) solid var(--bg-surface); border-radius: var(--radius-pill); }
    .react-flow__nodesselection-rect, .react-flow__selection { background: var(--bg-selected); border: var(--border-w) dotted var(--accent); }
    .react-flow__node:focus-visible { outline: var(--border-w-2) solid var(--accent); outline-offset: var(--space-1); }
    .react-flow__attribution { font-size: var(--text-micro); background: var(--bg-canvas); padding: var(--space-1) var(--space-2); margin: 0; }
    .react-flow__attribution a { text-decoration: none; color: var(--text-muted); }
  }
`;

/** Public styling contract for the installed renderer's root className. No wrapper DOM. */
export function flowCanvasClasses() {
  return cx("ux-flow-canvas", canvasUX, canvasSkin);
}
