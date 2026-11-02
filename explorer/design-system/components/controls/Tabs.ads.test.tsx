import { render } from "preact";
import { describe, expect, it, vi } from "vitest";
import { Tabs } from "./Tabs";

const items = ["a", "b", "c"].map((value) => ({ value }));

function mount(value: string, host: HTMLElement) {
  render(<Tabs items={items} value={value} />, host);
  return host.querySelector<HTMLElement>('[role="tablist"]')!;
}

describe("Tabs in a narrow container", () => {
  it("scrolls only itself to bring the selected tab into view", async () => {
    const host = document.createElement("div");
    document.body.append(host);
    const list = mount("a", host);
    vi.spyOn(list, "scrollWidth", "get").mockReturnValue(600);
    vi.spyOn(list, "clientWidth", "get").mockReturnValue(200);
    vi.spyOn(list, "getBoundingClientRect").mockReturnValue({ left: 0, right: 200 } as DOMRect);
    const tab = list.querySelectorAll<HTMLElement>('[role="tab"]')[2];
    vi.spyOn(tab, "getBoundingClientRect").mockReturnValue({ left: 400, right: 500 } as DOMRect);
    const page = vi.fn();
    tab.scrollIntoView = page;
    mount("c", host);
    await vi.waitFor(() => expect(list.scrollLeft).toBe(300));
    expect(page).not.toHaveBeenCalled();
    render(null, host);
    host.remove();
  });
});
