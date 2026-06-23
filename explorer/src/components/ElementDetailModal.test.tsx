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

  it("filters the selected concept out of its own broader and mapped concept lists", () => {
    const apiSurfaceNode = devFixture.ontology.graph_data?.nodes?.find(
      (node) => node.label === "API Surface" && node.semantic_type === "skos-concept",
    );
    if (!apiSurfaceNode) {
      throw new Error("API Surface concept node is missing from the test fixture");
    }

    const selfConceptIri = apiSurfaceNode.full_uri || apiSurfaceNode.id;
    const baseGraphData = devFixture.ontology.graph_data ?? { nodes: [], edges: [] };

    render(
      <StoreProvider
        store={{
          ...devFixture,
          ontology: {
            ...devFixture.ontology,
            graph_data: {
              ...baseGraphData,
              edges: [
                ...(baseGraphData.edges ?? []),
                {
                  source: apiSurfaceNode.id,
                  target: apiSurfaceNode.id,
                  label: "broader",
                  layer: "concepts",
                  source_kind: "concepts",
                },
                {
                  source: apiSurfaceNode.id,
                  target: apiSurfaceNode.id,
                  label: "mapsToConcept",
                  layer: "concepts",
                  source_kind: "concepts",
                },
              ],
            },
          },
        }}
        schemaMismatch={null}
      >
        <ElementDetailModal
          identifier="system-model/Thesaurus/Thesaurus.md#api-surface"
          onClose={vi.fn()}
          onOpenElement={vi.fn()}
          onOpenOntologyNode={vi.fn()}
        />
      </StoreProvider>,
    );

    expect(screen.getByText("Traceability")).toBeTruthy();
    expect(screen.queryByTitle(selfConceptIri)).toBeNull();
  });

  it("does not render concept references as a separate subsection", () => {
    const onOpenOntologyNode = vi.fn();
    render(
      <StoreProvider
        store={{
          ...devFixture,
          concept_refs: [
            {
              id: "concept:service-endpoint",
              source_id: "system-model/Specifications.md#example-requirement",
              target_element_id: "system-model/Thesaurus/Thesaurus.md#service-endpoint",
              label: "API endpoint",
              iri: "urn:reqvire:test:concepts#ServiceEndpoint",
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

    expect(screen.queryByRole("button", { name: /ServiceEndpoint/ })).toBeNull();
    expect(screen.queryByRole("heading", { name: "Concept References" })).toBeNull();
    expect(screen.queryByText("urn:reqvire:test:api:ServiceEndpoint")).toBeNull();
    expect(onOpenOntologyNode).not.toHaveBeenCalled();
  });

  it("renders native concept references inline and opens the concept element", () => {
    const onOpenElement = vi.fn();
    render(
      <StoreProvider
        store={{
          ...devFixture,
          concept_refs: [
            {
              id: "concept:service-endpoint",
              source_id: "system-model/Specifications.md#example-requirement",
              target_element_id: "system-model/Thesaurus/Thesaurus.md#service-endpoint",
              label: "system",
              iri: "urn:reqvire:test:concepts#ServiceEndpoint",
              line_number: 9,
            },
          ],
        }}
        schemaMismatch={null}
      >
        <ElementDetailModal
          identifier="system-model/Specifications.md#example-requirement"
          onClose={vi.fn()}
          onOpenElement={onOpenElement}
          onOpenOntologyNode={vi.fn()}
        />
      </StoreProvider>,
    );

    const conceptLink = screen.getByRole("button", { name: /system/ });
    expect(conceptLink.getAttribute("title")).toBe("urn:reqvire:test:concepts#ServiceEndpoint");

    fireEvent.click(conceptLink);
    expect(onOpenElement).toHaveBeenCalledWith("system-model/Thesaurus/Thesaurus.md#service-endpoint");
    expect(screen.queryByRole("heading", { name: "Concept references" })).toBeNull();
  });

  it("renders native concept reference alt labels inline", () => {
    const onOpenElement = vi.fn();
    render(
      <StoreProvider
        store={{
          ...devFixture,
          elements: devFixture.elements.map((element) =>
            element.id === "system-model/Specifications.md#example-requirement"
              ? {
                  ...element,
                  content: "The system shall expose an Endpoint for fixture data.",
                }
              : element,
          ),
          concept_refs: [
            {
              id: "concept:service-endpoint",
              source_id: "system-model/Specifications.md#example-requirement",
              target_element_id: "system-model/Thesaurus/Thesaurus.md#service-endpoint",
              label: "Service Endpoint",
              iri: "urn:reqvire:test:concepts#ServiceEndpoint",
              line_number: 9,
            },
          ],
        }}
        schemaMismatch={null}
      >
        <ElementDetailModal
          identifier="system-model/Specifications.md#example-requirement"
          onClose={vi.fn()}
          onOpenElement={onOpenElement}
          onOpenOntologyNode={vi.fn()}
        />
      </StoreProvider>,
    );

    const conceptLink = screen.getByRole("button", { name: /Endpoint/ });
    fireEvent.click(conceptLink);
    expect(onOpenElement).toHaveBeenCalledWith("system-model/Thesaurus/Thesaurus.md#service-endpoint");
    expect(screen.queryByRole("heading", { name: "Concept references" })).toBeNull();
  });
});
