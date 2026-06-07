import { Theme } from "@radix-ui/themes";
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { buildOntologyExplorerModel, OntologiesView } from "./OntologiesView";

vi.mock("graphology-layout-forceatlas2", () => ({
  default: {
    inferSettings: () => ({}),
    assign: vi.fn(),
  },
}));

vi.mock("sigma", () => ({
  default: class MockSigma {
    refresh = vi.fn();
    kill = vi.fn();
    on = vi.fn();
    getNodeDisplayData = vi.fn();
    getCamera = () => ({
      animate: vi.fn(),
      animatedReset: vi.fn(),
    });
  },
}));

function renderWithStore() {
  return render(
    <Theme>
      <StoreProvider store={devFixture} schemaMismatch={null}>
        <OntologiesView />
      </StoreProvider>
    </Theme>,
  );
}

describe("OntologiesView", () => {
  it("builds an OWL-aware term and construct model from the Project Store projection", () => {
    const model = buildOntologyExplorerModel(
      devFixture.ontology.declarations,
      devFixture.ontology.projection?.constructs,
    );

    expect(model.terms.some((term) => term.role === "class")).toBe(true);
    expect(model.terms.some((term) => term.role === "datatype-property")).toBe(true);
    expect(model.terms.some((term) => term.role === "node-shape")).toBe(true);
    expect(model.edges.some((edge) => edge.label === "domain")).toBe(true);
    expect(model.edges.some((edge) => edge.label === "range")).toBe(true);
  });

  it("renders the canonical committed ontology renderer with inspector search and Turtle download", async () => {
    const { container } = renderWithStore();

    const graph = screen.getByRole("img", { name: "Ontology and SHACL relationship graph" });
    expect(graph).toBeTruthy();
    expect(container.querySelector('[data-view="ontologies"]')).toBeTruthy();
    expect(container.querySelector("iframe")).toBeNull();
    expect(screen.getByRole("link", { name: /Download \.ttl/i }).getAttribute("href")).toBe(
      "ontologies.ttl",
    );

    expect(container.querySelector('script[type="module"]')?.textContent).toContain(
      "ontologyGraphData",
    );
    expect(screen.getByRole("searchbox")).toBeTruthy();
    expect(screen.getByText("Node Inspector")).toBeTruthy();
  });
});
