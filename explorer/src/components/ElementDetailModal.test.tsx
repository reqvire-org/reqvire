import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { ElementDetailModal } from "./ElementDetailModal";

describe("ElementDetailModal", () => {
  it("uses the back button title for previous element context without rendering a duplicate From line", () => {
    render(
      <StoreProvider store={devFixture} schemaMismatch={null}>
        <ElementDetailModal
          identifier="system-model/Specifications.md#example-capability"
          previousElementLabel="system-model/Specifications.md#example-requirement"
          onClose={vi.fn()}
          onOpenElement={vi.fn()}
          onOpenOntologyNode={vi.fn()}
          onNavigateBack={vi.fn()}
        />
      </StoreProvider>,
    );

    expect(screen.getByRole("button", { name: "Back to Example Requirement" })).toBeTruthy();
    expect(screen.queryByText(/From:/)).toBeNull();
  });

  it("renders resolved concept references as ontology term links instead of raw IRI text", () => {
    const onOpenOntologyNode = vi.fn();
    render(
      <StoreProvider
        store={{
          ...devFixture,
          concept_refs: [
            {
              id: "concept:service-endpoint",
              source_id: "system-model/Specifications.md#example-requirement",
              label: "API endpoint",
              iri: "urn:reqvire:test:api:ServiceEndpoint",
              line_number: 9,
            },
          ],
        }}
        schemaMismatch={null}
      >
        <ElementDetailModal
          identifier="system-model/Specifications.md#example-requirement"
          onClose={vi.fn()}
          onOpenElement={vi.fn()}
          onOpenOntologyNode={onOpenOntologyNode}
        />
      </StoreProvider>,
    );

    const conceptLink = screen.getByRole("button", { name: /ServiceEndpoint/ });
    expect(conceptLink.getAttribute("title")).toBe("urn:reqvire:test:api:ServiceEndpoint");
    expect(screen.getByText("(API endpoint)")).toBeTruthy();
    expect(screen.queryByText("urn:reqvire:test:api:ServiceEndpoint")).toBeNull();

    fireEvent.click(conceptLink);
    expect(onOpenOntologyNode).toHaveBeenCalledWith("urn:reqvire:test:api:ServiceEndpoint");
  });
});
