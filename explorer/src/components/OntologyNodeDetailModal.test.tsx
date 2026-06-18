import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { OntologyNodeDetailModal } from "./OntologyNodeDetailModal";

describe("OntologyNodeDetailModal", () => {
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
