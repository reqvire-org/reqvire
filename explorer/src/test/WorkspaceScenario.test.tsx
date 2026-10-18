import { fireEvent, render, screen, waitFor } from "@/test/render";
import { expect, it } from "vitest";
import WorkspaceScenario from "../../design-system/composition/Workspace/Workspace.ads.scenario";
import { TRACE_EXAMPLES } from "../../design-system/patterns/reports/fixtures/traces.ads.fixture";
it("composes public mock patterns with fixture navigation and local detail callbacks", async () => {
  history.replaceState(null, "", "/?tab=mocks");
  const view = render(<WorkspaceScenario />);
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
