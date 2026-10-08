import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { ElementDetailModal } from "./ElementDetailModal";
import { ContentView } from "./ContentView";

const source = "system-model/Specifications.md#example-requirement";
const target = "system-model/Contracts/API.md#api-response-contract";
const store = { ...devFixture, contract_references: [{ id: "reference:example", source_id: source,
  target, target_kind: "element", resource_id: null, content_hash: null }] };

describe("Contract References", () => {
  it.each(["detail", "source"])("opens reference targets in the real %s view", (view) => {
    const open = vi.fn();
    render(<StoreProvider store={store} schemaMismatch={null}>
      {view === "detail" ? <ElementDetailModal identifier={source} onClose={vi.fn()}
        onOpenElement={open} onOpenOntologyNode={vi.fn()} />
        : <ContentView path="system-model/Specifications.md" onOpenElement={open} />}
    </StoreProvider>);
    const disclosure = screen.getByRole("button", { name: "Contract References 1" });
    expect(screen.queryByRole("button", { name: /Contract Bindings/ })).toBeNull();
    if (disclosure.getAttribute("aria-expanded") === "false") fireEvent.click(disclosure);
    const section = disclosure.closest("section")!;
    fireEvent.click(within(section).getByRole("link", { name: /API Response Contract/ }));
    if (view === "detail") expect(open).toHaveBeenCalledWith(target);
    else expect(window.location.hash).toContain("api-response-contract");
  });
});
