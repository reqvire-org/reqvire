import { testFlowLayoutEngine } from "../test/flowLayoutEngine";
import { act, cleanup, fireEvent, render, screen, within, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { ElementFlow, type ElementFlowData } from "@ds";
import { MocksPage } from "./pages/MocksPage";

vi.mock("../../src/workers/flowLayoutEngine", () => ({ flowLayoutEngine: testFlowLayoutEngine }));

vi.mock("../../src/views/GraphLibraryViews", () => ({ KnowledgeGraphView: () => null }));
vi.mock("../../src/lib/ontologyGraphRenderer", () => ({ mountOntologyGraph: vi.fn() }));

const originalSeed = window.reqvireProjectStore;
const originalBBox = Object.getOwnPropertyDescriptor(SVGElement.prototype, "getBBox");
let canvasWidth = 1000;
let canvasHeight = 700;
const resizeNotifications = new Set<() => void>();

function viewport(region: HTMLElement) {
  const transform = getComputedStyle(region.querySelector<HTMLElement>(".react-flow__viewport")!).transform;
  const numbers = transform.match(/-?\d*\.?\d+(?:e[+-]?\d+)?/gi)!.map(Number);
  return { x: numbers[0], y: numbers[1], zoom: numbers[2] };
}

function expectCardsFit(region: HTMLElement) {
  const { x, y, zoom } = viewport(region);
  const cards = region.querySelectorAll<HTMLElement>(".react-flow__node");
  expect(cards.length).toBeGreaterThan(0);
  for (const card of cards) {
    const [left, top] = getComputedStyle(card).transform.match(/-?\d*\.?\d+(?:e[+-]?\d+)?/gi)!.map(Number);
    expect(left * zoom + x).toBeGreaterThanOrEqual(0);
    expect(top * zoom + y).toBeGreaterThanOrEqual(0);
    expect((left + card.offsetWidth) * zoom + x).toBeLessThanOrEqual(canvasWidth);
    expect((top + card.offsetHeight) * zoom + y).toBeLessThanOrEqual(canvasHeight);
  }
}

// Directly dispatched test events bypass browser hit testing. Check the wrapper too.
function expectPointerTarget(card: HTMLElement) {
  const wrapper = card.closest<HTMLElement>(".react-flow__node");
  expect(wrapper).not.toBeNull();
  expect(getComputedStyle(wrapper!).pointerEvents).not.toBe("none");
  expect(getComputedStyle(card).pointerEvents).not.toBe("none");
}
beforeEach(() => {
  canvasWidth = 1000;
  canvasHeight = 700;
  resizeNotifications.clear();
  window.history.replaceState(null, "", "/?tab=mocks#/traces");
  localStorage.clear();
  localStorage.setItem("reqvire-explorer-theme", "light");
  Element.prototype.scrollIntoView = vi.fn();
  vi.stubGlobal("Worker", class { postMessage() {} terminate() {} });
  // jsdom has no layout. Supply browser measurements so real React Flow edges mount.
  vi.stubGlobal("DOMMatrixReadOnly", class { m22 = 1; });
  vi.spyOn(HTMLElement.prototype, "offsetWidth", "get").mockImplementation(function (this: HTMLElement) {
    const width = getComputedStyle(this).width;
    if (width.endsWith("%")) return (this.parentElement?.offsetWidth ?? canvasWidth) * Number.parseFloat(width) / 100;
    return Number.parseFloat(width) || (this.classList.contains("react-flow__handle") ? 6 : canvasWidth);
  });
  vi.spyOn(HTMLElement.prototype, "offsetHeight", "get").mockImplementation(function (this: HTMLElement) {
    const height = getComputedStyle(this).height;
    if (height.endsWith("%")) return (this.parentElement?.offsetHeight ?? canvasHeight) * Number.parseFloat(height) / 100;
    return Number.parseFloat(height) || (this.classList.contains("react-flow__handle") ? 6 : canvasHeight);
  });
  Object.defineProperty(SVGElement.prototype, "getBBox", { configurable: true, value: () => ({ x: 0, y: 0, width: 100, height: 28 }) });
  vi.stubGlobal("ResizeObserver", class {
    targets = new Set<Element>();
    notify = () => this.callback([...this.targets].map(target => ({ target })));
    constructor(private callback: (entries: { target: Element }[]) => void) {}
    observe(target: Element) { this.targets.add(target); resizeNotifications.add(this.notify); queueMicrotask(() => { if (this.targets.has(target)) this.callback([{ target }]); }); }
    unobserve(target: Element) { this.targets.delete(target); }
    disconnect() { this.targets.clear(); resizeNotifications.delete(this.notify); }
  });
});
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
  if (originalBBox) Object.defineProperty(SVGElement.prototype, "getBBox", originalBBox);
  else Reflect.deleteProperty(SVGElement.prototype, "getBBox");
  window.reqvireProjectStore = originalSeed;
  window.history.replaceState(null, "", "/");
});

describe("trace flow showcase", () => {
  it.each([
    { width: 800, height: 160 },
    { width: 0, height: 0 },
  ])("keeps relation labels visible and compact with SVG text bounds $width × $height", async (bounds) => {
    Object.defineProperty(SVGElement.prototype, "getBBox", {
      configurable: true, value: () => ({ x: 0, y: 0, ...bounds }),
    });
    const data: ElementFlowData = {
      id: "label-bounds", title: "Label bounds",
      nodes: ["root", "leaf"].map((id, index) => ({
        element: { id, name: id, file: "Labels.md" },
        type: index === 0 ? "capability" : "requirement", context: "Model", root: index === 0,
      })),
      edges: [{ id: "root-leaf", source: "root", target: "leaf", label: "specifiedBy" }],
    };
    render(<ElementFlow layoutEngine={testFlowLayoutEngine} data={data} onOpenElement={vi.fn()} onOpenSource={vi.fn()} />);
    const region = screen.getByRole("region", { name: "Model flow" });
    const assertLabel = () => {
      const edge = region.querySelector('.react-flow__edge[data-id="root-leaf"]')!;
      expect(edge).not.toBeNull();
      const label = edge.querySelector("text")!;
      expect(label.textContent).toBe("specifiedBy");
      // Oversized cached rectangles hide adjacent routes; the label uses a glyph outline.
      expect([...edge.querySelectorAll("rect")].map(rect => ({
        width: Number(rect.getAttribute("width")), height: Number(rect.getAttribute("height")),
      }))).toEqual([]);
      expect(label.closest('[visibility="hidden"]')).toBeNull();
      expect(label.getAttribute("text-anchor")).toBe("middle");
      expect(label.getAttribute("dominant-baseline")).toBe("central");
      expect(Number(label.getAttribute("x"))).toBeGreaterThan(0);
      expect(Number(label.getAttribute("y"))).toBeGreaterThan(0);
      expect(edge.querySelector(".react-flow__edge-path")?.getAttribute("marker-end")).toBeTruthy();
      return label;
    };
    for (const direction of ["Top to bottom", "Left to right"]) {
      fireEvent.click(within(region).getByRole("button", { name: direction }));
      await waitFor(() => expect(region.getAttribute("aria-busy")).toBe("false"));
      await waitFor(() => expectCardsFit(region));
      const label = assertLabel();
      const placement = label.outerHTML;
      fireEvent.click(within(region).getByRole("button", { name: "Zoom out" }));
      await waitFor(() => expect(viewport(region).zoom).toBeLessThan(1));
      expect(assertLabel().outerHTML).toBe(placement);
      fireEvent.click(within(region).getByRole("button", { name: "100%" }));
      await waitFor(() => expect(viewport(region).zoom).toBe(1));
      expect(assertLabel().outerHTML).toBe(placement);
    }
  });

  it("preserves connections, centered labels, cards, and focus during viewport navigation", async () => {
    render(<MocksPage />);
    fireEvent.click(screen.getByRole("treeitem", { name: /Ontology Projection Verification/ }));
    const region = screen.getByRole("region", { name: "Verification trace flow" });
    await within(region).findByRole("article", { name: "Property Projection" });
    fireEvent.click(within(region).getByRole("button", { name: "Focus path through Property Projection" }));

    for (const direction of ["Left to right", "Top to bottom"]) {
      fireEvent.click(within(region).getByRole("button", { name: direction }));
      await waitFor(() => expect(region.getAttribute("aria-busy")).toBe("false"));
      await waitFor(() => expectCardsFit(region));
      // Allow React Flow's asynchronous initialization and initial fit to settle.
      await act(async () => { await new Promise(resolve => setTimeout(resolve, 30)); });
      const renderer = region.querySelector(".react-flow");
      const cards = [...region.querySelectorAll(".react-flow__node")];
      const positions = cards.map(card => getComputedStyle(card).transform);
      const wrappers = [...region.querySelectorAll<SVGElement>(".react-flow__edge")];
      expect(wrappers.length).toBeGreaterThan(0);
      const labels = [...region.querySelectorAll(".ux-trace-edge-label")];
      expect(labels).toHaveLength(wrappers.length);
      const descriptions = labels.map(label => label.outerHTML);
      const paths = () => [...region.querySelectorAll<SVGPathElement>(".react-flow__edge-path")];
      const geometry = paths().map(path => [path.getAttribute("d"), path.getAttribute("marker-end")]);
      const pane = region.querySelector(".react-flow__pane")!;
      const view = pane.ownerDocument.defaultView!;
      const panEvent = (target: Element | Window, type: string, init: MouseEventInit) => {
        const event = new MouseEvent(type, { bubbles: true, cancelable: true, ...init });
        // jsdom rejects Vitest's Window proxy in MouseEventInit; D3 needs event.view.
        Object.defineProperty(event, "view", { value: view });
        fireEvent(target, event);
      };
      const navigate = async (action: () => void) => {
        const previousPaths = paths();
        const previousViewport = viewport(region);
        // Keep focus on the edge wrapper while navigating the canvas.
        act(() => wrappers[0].focus());
        action();
        await waitFor(() => {
          expect(viewport(region)).not.toEqual(previousViewport);
          expect(previousPaths.every(path => path.isConnected)).toBe(true);
        });
        expect(paths().map(path => [path.getAttribute("d"), path.getAttribute("marker-end")])).toEqual(geometry);
        const currentLabels = [...region.querySelectorAll(".ux-trace-edge-label")];
        expect(currentLabels.length).toBe(labels.length);
        labels.forEach((label, index) => expect(currentLabels[index] === label, `label ${index} stays mounted`).toBe(true));
        expect(labels.map(label => label.outerHTML)).toEqual(descriptions);
        const currentCards = [...region.querySelectorAll(".react-flow__node")];
        expect(currentCards.length).toBe(cards.length);
        cards.forEach((card, index) => expect(currentCards[index] === card, `card ${index} stays mounted`).toBe(true));
        expect(cards.map(card => getComputedStyle(card).transform)).toEqual(positions);
        expect(region.querySelector(".react-flow")).toBe(renderer);
        expect(document.activeElement).toBe(wrappers[0]);
        expect(within(region).getByRole("article", { name: "Property Projection" }).getAttribute("data-focused")).toBe("true");
      };
      await navigate(() => fireEvent.click(within(region).getByRole("button", { name: "100%" })));
      await navigate(() => fireEvent.click(within(region).getByRole("button", { name: "Fit trace" })));
      await navigate(() => fireEvent.click(within(region).getByRole("button", { name: "Zoom out" })));
      await navigate(() => fireEvent.click(within(region).getByRole("button", { name: "Zoom in" })));
      await navigate(() => fireEvent.wheel(pane, { deltaY: 100, clientX: 150, clientY: 100 }));
      await navigate(() => {
        panEvent(pane, "mousedown", { button: 0, buttons: 1, clientX: 100, clientY: 100 });
        panEvent(view, "mousemove", { buttons: 1, clientX: 150, clientY: 120 });
        panEvent(view, "mouseup", { button: 0, clientX: 150, clientY: 120 });
      });
    }
  });

  it("expands Model Flow without replacing its canvas and keeps element details above it", async () => {
    window.history.replaceState(null, "", "/?tab=mocks#/model");
    render(<MocksPage />);
    fireEvent.click(screen.getByRole("button", { name: "Flow" }));
    const card = await screen.findByRole("article", { name: "Ontology Construct Classification" });
    const renderer = card.closest(".react-flow");
    fireEvent.click(screen.getByRole("button", { name: "Focus path through Ontology Construct Classification" }));
    const expand = screen.getByRole("button", { name: "Expand flow" });
    act(() => expand.focus());
    fireEvent.click(expand);
    const overlay = screen.getByRole("dialog", { name: "Expanded model flow" });
    expect(overlay.getAttribute("aria-modal")).toBe("true");
    expect(overlay.querySelector(".react-flow")).toBe(renderer);
    expect(within(overlay).getByRole("article", { name: "Ontology Construct Classification" })).toBe(card);
    expect(card.getAttribute("data-focused")).toBe("true");
    expect(within(overlay).getByRole("button", { name: "Top to bottom" }).getAttribute("aria-pressed")).toBe("true");
    expect(document.activeElement).toBe(within(overlay).getByRole("button", { name: "Close expanded flow" }));
    const first = within(overlay).getByRole("button", { name: "Open details" });
    const last = within(overlay).getByRole("link", { name: "React Flow attribution" });
    act(() => first.focus());
    fireEvent.keyDown(first, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(last);
    fireEvent.keyDown(last, { key: "Tab" });
    expect(document.activeElement).toBe(first);
    fireEvent.click(within(overlay).getByRole("link", { name: "Ontology Construct Classification" }));
    await waitFor(() => expect(screen.getAllByRole("dialog")).toHaveLength(2));
    const detail = screen.getAllByRole("dialog").find(dialog => dialog !== overlay)!;
    expect(within(detail).getByRole("heading", { name: "Ontology Construct Classification" })).toBeTruthy();
    fireEvent.keyDown(within(detail).getAllByRole("button", { name: "Close" })[0], { key: "Escape" });
    await waitFor(() => expect(screen.getAllByRole("dialog")).toEqual([overlay]));
    expect(window.location.hash).toContain("#/model?selected=");
    expect(card.closest(".react-flow")).toBe(renderer);
    fireEvent.click(within(overlay).getByRole("button", { name: "100%" }));
    const region = within(overlay).getByRole("region", { name: "Model flow" });
    await waitFor(() => expect(viewport(region).zoom).toBe(1));
    fireEvent.click(within(overlay).getByRole("button", { name: "Fit flow" }));
    await waitFor(() => expectCardsFit(region));
    fireEvent.click(within(overlay).getByRole("button", { name: "Close expanded flow" }));
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(card.closest(".react-flow")).toBe(renderer);
    expect(card.getAttribute("data-focused")).toBe("true");
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Expand flow" }));
  });

  it("retains trace disclosure and path focus across expansion, Escape, and source navigation", async () => {
    render(<MocksPage />);
    fireEvent.click(screen.getByRole("treeitem", { name: /Ontology Projection Verification/ }));
    const card = await screen.findByRole("article", { name: "Property Projection" });
    fireEvent.click(screen.getByRole("button", { name: "Collapse ancestors of Property Projection" }));
    await waitFor(() => expect(screen.queryByRole("article", { name: "Property Resolution" })).toBeNull());
    fireEvent.click(screen.getByRole("button", { name: "Focus path through Property Projection" }));
    fireEvent.click(screen.getByRole("button", { name: "Expand flow" }));
    const overlay = screen.getByRole("dialog", { name: "Expanded verification trace flow" });
    expect(within(overlay).queryByRole("article", { name: "Property Resolution" })).toBeNull();
    expect(card.getAttribute("data-focused")).toBe("true");
    fireEvent.click(within(overlay).getByRole("button", { name: "Top to bottom" }));
    await waitFor(() => expect(within(overlay).getByRole("region", { name: "Verification trace flow" }).getAttribute("aria-busy")).toBe("false"));
    const renderer = overlay.querySelector(".react-flow");
    fireEvent.keyDown(within(overlay).getByRole("button", { name: "Close expanded flow" }), { key: "Escape" });
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(screen.queryByRole("article", { name: "Property Resolution" })).toBeNull();
    expect(screen.getByRole("article", { name: "Property Projection" }).getAttribute("data-focused")).toBe("true");
    expect(screen.getByRole("button", { name: "Top to bottom" }).getAttribute("aria-pressed")).toBe("true");
    expect(screen.getByRole("region", { name: "Verification trace flow" }).querySelector(".react-flow")).toBe(renderer);
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Expand flow" }));
    fireEvent.click(screen.getByRole("button", { name: "Expand flow" }));
    fireEvent.click(within(screen.getByRole("dialog", { name: "Expanded verification trace flow" })).getByRole("link", { name: "Open source for Property Projection" }));
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
    expect(window.location.hash).toContain("#/content/");
  });

  it("fits a large layout below 20% in both directions and after canvas or scope changes", async () => {
    const data: ElementFlowData = {
      id: "large-flow", title: "Large model",
      nodes: Array.from({ length: 121 }, (_, index) => ({
        element: { id: `large-${index}`, name: `Element ${index}`, file: "Large.md" },
        type: index === 0 ? "capability" : "requirement", context: "Model", root: index === 0,
      })),
      edges: [...Array.from({ length: 120 }, (_, index) => ({
        id: `edge-${index}`, source: `large-${index}`, target: `large-${index + 1}`, label: "derive",
      })), { id: "long-route", source: "large-0", target: "large-120", label: "specifiedBy" }],
    };
    const navigation = { onOpenElement: vi.fn(), onOpenSource: vi.fn() };
    const { rerender } = render(<ElementFlow layoutEngine={testFlowLayoutEngine} data={data} {...navigation} />);
    const region = screen.getByRole("region", { name: "Model flow" });
    await waitFor(() => expect(region.getAttribute("aria-busy")).toBe("false"), { timeout: 10000 });
    await waitFor(() => expectCardsFit(region));
    expect(viewport(region).zoom).toBeLessThan(0.2);
    const resizeCanvas = async (width: number, height: number) => {
      const before = viewport(region);
      act(() => {
        canvasWidth = width;
        canvasHeight = height;
        for (const notify of resizeNotifications) notify();
      });
      await waitFor(() => {
        expect(viewport(region)).not.toEqual(before);
        expectCardsFit(region);
      });
    };
    for (const direction of ["Top to bottom", "Left to right"]) {
      if (canvasWidth !== 1000) await resizeCanvas(1000, 700);
      fireEvent.click(screen.getByRole("button", { name: direction }));
      await waitFor(() => expect(region.getAttribute("aria-busy")).toBe("false"), { timeout: 10000 });
      await waitFor(() => expectCardsFit(region));
      const beforeActualSize = [...region.querySelectorAll(".react-flow__edge-path")];
      expect(beforeActualSize).toHaveLength(data.edges.length);
      fireEvent.click(screen.getByRole("button", { name: "100%" }));
      await waitFor(() => expect(viewport(region).zoom).toBe(1));
      expect(beforeActualSize.every(path => path.isConnected)).toBe(true);
      const beforeFit = [...region.querySelectorAll(".react-flow__edge-path")];
      fireEvent.click(screen.getByRole("button", { name: "Fit flow" }));
      await waitFor(() => expectCardsFit(region));
      expect(beforeFit.every(path => path.isConnected)).toBe(true);
      const fittedZoom = viewport(region).zoom;
      fireEvent.click(screen.getByRole("button", { name: "Zoom out" }));
      await waitFor(() => expect(viewport(region).zoom).toBeLessThan(fittedZoom));
      await resizeCanvas(360, 520);
    }
    rerender(<ElementFlow layoutEngine={testFlowLayoutEngine} data={{ ...data, nodes: data.nodes.slice(0, 1), edges: [] }} {...navigation} />);
    await waitFor(() => expect(region.querySelectorAll(".react-flow__node")).toHaveLength(1));
    await waitFor(() => {
      expectCardsFit(region);
      expect(viewport(region).zoom).toBeGreaterThan(0.2);
    });
    expect(viewport(region).zoom).toBeLessThanOrEqual(1);
    await resizeCanvas(1000, 700);
    expect(viewport(region).zoom).toBe(1);
  }, 30000);

  it("uses the left tree to scope Model Flow and reserves element details for canvas cards", async () => {
    window.history.replaceState(null, "", "/?tab=mocks#/model");
    render(<MocksPage />);
    fireEvent.click(screen.getByRole("button", { name: "Flow" }));
    await screen.findByRole("article", { name: "Ontology Construct Classification" });
    const filter = screen.getByRole("searchbox", { name: "Filter project tree" });
    const tree = screen.getByRole("tree", { name: "Project tree" });
    fireEvent.change(filter, { target: { value: "Ontology Construct Classification" } });
    fireEvent.click(within(tree).getByText("Ontology Construct Classification"));
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(window.location.hash).toContain("#/model?selected=");
    const flow = screen.getByRole("region", { name: "Model flow" });
    expect(within(flow).getByRole("heading", { name: "Ontology Construct Classification" })).toBeTruthy();
    await waitFor(() => expect(within(flow).queryByRole("article", { name: "Property Projection" })).toBeNull());
    expect(within(flow).getByRole("article", { name: "Ontology Exploration" })).toBeTruthy();
    fireEvent.change(filter, { target: { value: "API Exploration Capability" } });
    fireEvent.click(within(tree).getByText("API Exploration Capability"));
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(await within(flow).findByRole("article", { name: "API Surface Exploration" })).toBeTruthy();
    expect(within(flow).queryByRole("article", { name: "Ontology Construct Classification" })).toBeNull();
    fireEvent.click(within(flow).getByRole("link", { name: "API Exploration Capability" }));
    expect(within(await screen.findByRole("dialog")).getByRole("heading", { name: "API Exploration Capability" })).toBeTruthy();
  });
  it("opens Model Flow from the existing selector with the same element modal and path interactions", async () => {
    window.history.replaceState(null, "", "/?tab=mocks#/model");
    render(<MocksPage />);
    fireEvent.click(screen.getByRole("button", { name: "Flow" }));
    const flow = await screen.findByRole("region", { name: "Model flow" });
    const node = await within(flow).findByRole("article", { name: "Ontology Construct Classification" });
    expectPointerTarget(node);
    expect(screen.getByRole("button", { name: "Top to bottom" }).getAttribute("aria-pressed")).toBe("true");
    const capabilityPosition = within(flow).getByRole("article", { name: "Ontology Exploration" }).parentElement!.getAttribute("style")!;
    const requirementPosition = node.parentElement!.getAttribute("style")!;
    const y = (transform: string) => Number(/translate\([^,]+,\s*([\d.]+)px/.exec(transform)?.[1]);
    expect(y(capabilityPosition)).toBeLessThan(y(requirementPosition));
    fireEvent.mouseEnter(node);
    expect(within(flow).getByRole("article", { name: "SHACL and Ontology Algorithm Services" }).getAttribute("data-dimmed")).toBe("false");
    expect(within(flow).getByRole("article", { name: "Property Resolution" }).getAttribute("data-dimmed")).toBe("true");
    fireEvent.click(node);
    expect(within(await screen.findByRole("dialog")).getByText("verifiedBy", { exact: true })).toBeTruthy();
    fireEvent.click(within(screen.getByRole("dialog")).getAllByRole("button", { name: "Close" })[0]);
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
    expect(window.location.hash).toContain("#/model?selected=");
    expect(screen.getByRole("button", { name: "Flow" }).getAttribute("aria-pressed")).toBe("true");
    fireEvent.click(within(flow).getByRole("button", { name: "Left to right" }));
    await waitFor(() => expect(flow.getAttribute("aria-busy")).toBe("false"));
    fireEvent.click(screen.getByRole("button", { name: "Grid" }));
    expect(screen.queryByRole("region", { name: "Model flow" })).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Flow" }));
    expect(await screen.findByRole("region", { name: "Model flow" })).toBeTruthy();
    fireEvent.change(screen.getByRole("searchbox", { name: "Filter project tree" }), { target: { value: "OntologyKernelRequirements" } });
    fireEvent.click(within(screen.getByRole("tree", { name: "Project tree" })).getByText("OntologyKernelRequirements.md"));
    await waitFor(() => expect(screen.queryByRole("article", { name: "API Exploration Capability" })).toBeNull());
    expect(await screen.findByRole("article", { name: "Ontology Construct Classification" })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "List" }));
    expect(screen.getByRole("button", { name: /Ontology Construct Classification$/ })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Flow" }));
    await screen.findByRole("article", { name: "Ontology Construct Classification" });
    expect(screen.queryByRole("article", { name: "API Exploration Capability" })).toBeNull();
  });
  it("opens the native trace from the existing Explorer menu and supports returning to it", async () => {
    window.history.replaceState(null, "", "/?tab=mocks#/model");
    render(<MocksPage />);
    expect(screen.queryByRole("group", { name: "Example model" })).toBeNull();
    fireEvent.click(screen.getByRole("tab", { name: "Traces" }));
    expect(await screen.findByRole("article", { name: "Ontology Construct Classification" })).toBeTruthy();
    expect(window.location.hash).toBe("#/traces?selected=classification-test");
    expect(screen.getAllByRole("navigation", { name: "Explorer views" })).toHaveLength(1);
    expect(screen.getByRole("tab", { name: "Traces" }).getAttribute("aria-selected")).toBe("true");
    fireEvent.click(screen.getByRole("tab", { name: "Model" }));
    expect(screen.queryByRole("region", { name: "Verification trace flow" })).toBeNull();
    fireEvent.click(screen.getByRole("tab", { name: "Traces" }));
    expect(await screen.findByRole("article", { name: "Ontology Construct Classification" })).toBeTruthy();
  });

  it("switches flow direction while retaining collapsed branches and focused paths", async () => {
    render(<MocksPage />);
    fireEvent.click(screen.getByRole("treeitem", { name: /Ontology Projection Verification/ }));
    await screen.findByRole("article", { name: "Property Projection" });
    fireEvent.click(screen.getByRole("button", { name: "Collapse ancestors of Property Projection" }));
    await waitFor(() => expect(screen.queryByRole("article", { name: "Property Resolution" })).toBeNull());
    fireEvent.click(screen.getByRole("button", { name: "Focus path through Property Projection" }));
    const horizontal = screen.getByRole("article", { name: "Property Projection" }).parentElement?.getAttribute("style");
    fireEvent.click(screen.getByRole("button", { name: "Top to bottom" }));
    await waitFor(() => expect(screen.getByRole("region", { name: "Verification trace flow" }).getAttribute("aria-busy")).toBe("false"));
    expect(screen.getByRole("button", { name: "Top to bottom" }).getAttribute("aria-pressed")).toBe("true");
    expect(screen.getByRole("article", { name: "Property Projection" }).parentElement?.getAttribute("style")).not.toBe(horizontal);
    expect(screen.getByRole("article", { name: "Property Projection" }).getAttribute("data-focused")).toBe("true");
    expect(screen.queryByRole("article", { name: "Property Resolution" })).toBeNull();
    expect(screen.getByText("Test · 2 directly verified · 5 requirements in trace")).toBeTruthy();
    expect(screen.queryByRole("dialog")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Left to right" }));
    await waitFor(() => expect(screen.getByRole("article", { name: "Property Projection" }).parentElement?.getAttribute("style")).toBe(horizontal));
    fireEvent.click(screen.getByRole("button", { name: "Expand all" }));
    expect(await screen.findByRole("article", { name: "Property Resolution" })).toBeTruthy();
  });

  it("highlights and pins paths through repeated splits and merges without moving cards", async () => {
    render(<MocksPage />);
    fireEvent.click(screen.getByRole("treeitem", { name: /Split and Merge Trace Verification/ }));
    const source = await screen.findByRole("article", { name: "Source Parsing" });
    expectPointerTarget(source);
    const sibling = screen.getByRole("article", { name: "Reference Resolution" });
    const merge = screen.getByRole("article", { name: "Normalized Model" });
    const position = getComputedStyle(source.parentElement!).transform;
    fireEvent.mouseEnter(source);
    await waitFor(() => expect(document.querySelectorAll(".react-flow__edge.ux-trace-edge-highlighted")).toHaveLength(8));
    const highlighted = document.querySelectorAll(".react-flow__edge.ux-trace-edge-highlighted");
    expect(highlighted).toHaveLength(8);
    for (const edge of highlighted) expect(edge.querySelector(".react-flow__edge-path")?.getAttribute("marker-end")).toContain("--accent");
    expect(sibling.getAttribute("data-dimmed")).toBe("true");
    expect(merge.getAttribute("data-dimmed")).toBe("false");
    fireEvent.click(within(source).getByRole("button", { name: "Focus path through Source Parsing" }));
    fireEvent.mouseLeave(source);
    expect(source.getAttribute("data-focused")).toBe("true");
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(sibling.getAttribute("data-dimmed")).toBe("true");
    expect(getComputedStyle(source.parentElement!).transform).toBe(position);
    fireEvent.click(screen.getByRole("button", { name: "Clear path focus" }));
    expect(document.querySelectorAll(".ux-trace-edge-highlighted")).toHaveLength(0);
    expect(sibling.getAttribute("data-dimmed")).toBe("false");
    fireEvent.focus(within(sibling).getByRole("link", { name: "Reference Resolution" }));
    expect(source.getAttribute("data-dimmed")).toBe("true");
    fireEvent.keyDown(sibling, { key: "Escape" });
    expect(source.getAttribute("data-dimmed")).toBe("false");
  });

  it("opens the application's element modal from card bodies and names with complete relations", async () => {
    render(<MocksPage />);
    const node = await screen.findByRole("article", { name: "Ontology Construct Classification" });
    expectPointerTarget(node);
    const position = getComputedStyle(node.parentElement!).transform;
    fireEvent.click(node);
    const detail = await screen.findByRole("dialog");
    expect(window.location.hash).toBe("#/elements/classification");
    expect(within(detail).getByText(/The system shall classify ontology constructs/)).toBeTruthy();
    expect(within(detail).getByText("requirement", { exact: true })).toBeTruthy();
    expect(within(detail).getByText("approved", { exact: true })).toBeTruthy();
    expect(within(detail).getByText("derivedFrom", { exact: true })).toBeTruthy();
    expect(within(detail).getByText("verifiedBy", { exact: true })).toBeTruthy();
    expect(within(detail).getByRole("link", { name: /O-Kernel Ontology Classification Unit Test Verification/ })).toBeTruthy();
    fireEvent.click(within(detail).getByRole("link", { name: /SHACL and Ontology Algorithm Services/ }));
    await waitFor(() => expect(window.location.hash).toBe("#/elements/algorithms"));
    expect(within(screen.getByRole("dialog")).getByText(/provide shared SHACL/)).toBeTruthy();
    fireEvent.click(within(screen.getByRole("dialog")).getByRole("button", { name: "Back to Ontology Construct Classification" }));
    await waitFor(() => expect(window.location.hash).toBe("#/elements/classification"));
    fireEvent.click(within(screen.getByRole("dialog")).getAllByRole("button", { name: "Close" })[0]);
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
    expect(window.location.hash).toBe("#/traces?selected=classification-test");
    expect(getComputedStyle(screen.getByRole("article", { name: "Ontology Construct Classification" }).parentElement!).transform).toBe(position);
    fireEvent.click(within(node).getByRole("link", { name: "Ontology Construct Classification" }));
    fireEvent.click(within(await screen.findByRole("dialog")).getByRole("link", { name: "Open source page" }));
    await waitFor(() => expect(window.location.hash).toContain("#/content/system-model/Architecture/OntologyKernelRequirements.md"));
    expect(screen.queryByRole("dialog")).toBeNull();
    expect(await screen.findByText(/The system shall classify ontology constructs/)).toBeTruthy();
  });

  it("selects a file overview and opens a branching trace with one shared ancestor", async () => {
    render(<MocksPage />);
    fireEvent.click(screen.getByRole("treeitem", { name: /OntologyKernelVerifications.md/ }));
    expect(screen.queryByRole("region", { name: "Verification trace flow" })).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Ontology Projection Verification" }));
    expect(screen.getByText("Test · 2 directly verified · 5 requirements in trace")).toBeTruthy();
    await screen.findByRole("article", { name: "Ontology Projection" });
    expect(screen.getAllByRole("article", { name: "Ontology Projection" })).toHaveLength(1);
    fireEvent.click(screen.getByRole("button", { name: "Collapse ancestors of Property Projection" }));
    await waitFor(() => expect(screen.queryByRole("article", { name: "Property Resolution" })).toBeNull());
    expect(screen.getByRole("article", { name: "Ontology Projection" })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Expand all" }));
    expect(await screen.findByRole("article", { name: "Property Resolution" })).toBeTruthy();
  });

  it("provides long-name, unlinked, filtering, and theme examples", async () => {
    render(<MocksPage />);
    fireEvent.change(screen.getByRole("searchbox", { name: "Filter trace tree" }), { target: { value: "Built-In External Ontology Vocabulary" } });
    fireEvent.click(screen.getByRole("treeitem", { name: /Built-In External Ontology Vocabulary/ }));
    expect(await screen.findByRole("article", { name: "Built-In External Ontology Source Resolution for Referenced Vocabulary Terms" })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Switch to dark mode" }));
    expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
    fireEvent.change(screen.getByRole("searchbox", { name: "Filter trace tree" }), { target: { value: "Pending Vocabulary" } });
    expect(screen.queryByRole("treeitem", { name: /Ontology Projection Verification/ })).toBeNull();
    fireEvent.keyDown(screen.getByRole("treeitem", { name: /Pending Vocabulary Verification/ }), { key: "Enter" });
    expect(screen.getByText("Test · 0 directly verified · 0 requirements in trace")).toBeTruthy();
    await screen.findByRole("article", { name: "Pending Vocabulary Verification" });
    expect(screen.getAllByRole("article")).toHaveLength(1);
    expect(screen.getByRole("button", { name: "Zoom in" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "Fit trace" })).toBeTruthy();
  });
});
