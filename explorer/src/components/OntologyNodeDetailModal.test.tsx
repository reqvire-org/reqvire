import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { semanticQueryStore, queryId, queryText } from "../store/fixtures/semanticQueryGraph";
import { OntologyNodeDetailModal } from "./OntologyNodeDetailModal";

describe("OntologyNodeDetailModal", () => {
  it("shows query form, source text, ontology dependencies and declared outputs", () => {
    const onClose = vi.fn();
    render(<StoreProvider store={semanticQueryStore} schemaMismatch={null}>
      <OntologyNodeDetailModal nodeId={queryId} onClose={onClose} />
    </StoreProvider>);
    expect(screen.getByRole("heading", { name: "Query · CONSTRUCT" })).toBeTruthy();
    expect(document.querySelector("pre code")?.textContent).toBe(queryText);
    expect(screen.getByRole("heading", { name: "Used ontologies" })).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Produces properties" })).toBeTruthy();
    expect(screen.getByRole("heading", { name: "Referenced vocabulary" })).toBeTruthy();
    fireEvent.click(screen.getByRole("link", { name: /Open query source/ }));
    expect(window.location.hash).toBe("#/content/system-model/Queries.md#item-labels");
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("opens ontology source links as Explorer content routes and closes the modal", () => {
    const onClose = vi.fn();
    window.location.hash = "#/ontologies";

    render(
      <StoreProvider store={devFixture} schemaMismatch={null}>
        <OntologyNodeDetailModal
          nodeId="urn:reqvire:test:api:ServiceEndpoint"
          onClose={onClose}
        />
      </StoreProvider>,
    );

    fireEvent.click(screen.getByRole("link", { name: /Open ontology source/ }));

    expect(window.location.hash).toBe("#/content/system-model/Specifications.md#example-requirement");
    expect(onClose).toHaveBeenCalledTimes(1);
  });
});
