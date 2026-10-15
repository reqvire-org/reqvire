import { fireEvent, render, screen, waitFor } from "@/test/render";
import { afterEach, expect, it, vi } from "vitest";
import { ShowcaseApp } from "./ShowcaseApp";
import { TRACE_EXAMPLES } from "./fixtures/traces";

vi.mock("./pages/TokensPage", () => ({ TokensPage: () => <p>Token catalogue</p> }));
vi.mock("./pages/ComponentsPage", () => ({ ComponentsPage: () => <p>Component catalogue</p> }));
vi.mock("./pages/PatternsPage", () => ({ PatternsPage: () => <p>Pattern catalogue</p> }));

afterEach(() => { history.replaceState(null, "", "/"); });

it("uses the neutral navigation, switches theme, and hides the header without replacing the mock", async () => {
  history.replaceState(null, "", "/?tab=mocks");
  const view = render(<ShowcaseApp />);
  try {
    const navigation = screen.getByRole("navigation", { name: "Showcase sections" });
    expect(navigation.querySelectorAll("button")).toHaveLength(4);
    expect(screen.getByRole("button", { name: "Mocks" }).getAttribute("aria-current")).toBe("page");
    const shell = view.container.querySelector("[data-product-pattern=app-shell]");
    expect(shell).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Use dark theme" }));
    expect(document.documentElement.dataset.theme).toBe("dark");
    fireEvent.click(screen.getByRole("button", { name: "Use light theme" }));
    expect(document.documentElement.dataset.theme).toBe("light");
    fireEvent.click(screen.getByRole("button", { name: "Minimize showcase menu" }));
    expect(screen.queryByRole("navigation", { name: "Showcase sections" })).toBeNull();
    expect(view.container.querySelector("[data-product-pattern=app-shell]")).toBe(shell);
    expect(screen.queryByRole("button", { name: /Maximize showcase/ })).toBeNull();
  } finally { view.unmount(); }
});

it("composes public mock patterns with fixture navigation and local detail callbacks", async () => {
  history.replaceState(null, "", "/?tab=mocks");
  const view = render(<ShowcaseApp />);
  try {
    fireEvent.click(screen.getByRole("tab", { name: "Traces" }));
    await waitFor(() => expect(view.container.querySelector("[data-product-pattern=trace-flow]")).toBeTruthy());
    fireEvent.click(screen.getByRole("treeitem", { name: /^OntologyKernelVerifications\.md / }));
    expect(screen.getByTestId("trace-rows")).toBeTruthy();
    const item = TRACE_EXAMPLES[1];
    fireEvent.input(screen.getByRole("searchbox", { name: "Filter trace tree" }), { target: { value: item.trace.verification.name } });
    fireEvent.click(screen.getByRole("treeitem", { name: `test-verification ${item.trace.verification.name}` }));
    await waitFor(() => expect(view.container.querySelector("[data-product-pattern=trace-flow]")).toBeTruthy());
    await waitFor(() => expect(view.container.textContent).toContain(item.trace.verification.name));
    fireEvent.click(screen.getByRole("tab", { name: "Coverage" }));
    expect(view.container.querySelector("[data-product-pattern=coverage-drilldown]")).toBeTruthy();
  } finally { view.unmount(); }
});
