import { fireEvent, render, screen, waitFor } from "@/test/render";
import { describe, expect, it, vi } from "vitest";
import {
  MarkdownContent,
  sourceUrlTransform,
} from "./MarkdownContent";

function mockMermaidRender() {
  const renderMermaid = vi.fn().mockResolvedValue({
    svg: '<svg data-testid="mermaid-svg"></svg>',
    bindFunctions: vi.fn(),
  });
  window.mermaid = {
    render: renderMermaid,
  } as unknown as typeof window.mermaid;
  return renderMermaid;
}

describe("MarkdownContent", () => {
  it("hides the structural Reqvire document heading", () => {
    render(
      <MarkdownContent
        markdown={["# Elements", "", "### Example Requirement", "", "Body."].join("\n")}
        sourceFilePath="system-model/Capabilities.md"
      />,
    );

    expect(screen.queryByRole("heading", { name: "Elements" })).toBeNull();
    expect(screen.getByRole("heading", { name: "Example Requirement" })).toBeTruthy();
  });

  it("replaces CSS token references before handing code to Mermaid", async () => {
    const renderMermaid = mockMermaidRender();

    render(
      <MarkdownContent
        markdown={[
          "```mermaid",
          "graph TD",
          "  classDef capability fill:var(--color-role-capability),stroke:var(--color-role-capability-ink),stroke-width:2.5px;",
          "  A[Capability]:::capability",
          "```",
        ].join("\n")}
        sourceFilePath="system-model/Capabilities.md"
      />,
    );

    await waitFor(() => expect(renderMermaid).toHaveBeenCalled());
    const renderedCode = renderMermaid.mock.calls[0][1] as string;
    expect(renderedCode).not.toContain("var(--");
    expect(renderedCode).toContain("fill:#bbdefb");
  });

  it("mounts Mermaid SVG output that contains HTML label markup", async () => {
    window.mermaid = {
      render: vi.fn().mockResolvedValue({
        svg: '<svg data-testid="mermaid-svg"><foreignObject><div><br></div></foreignObject></svg>',
        bindFunctions: vi.fn(),
      }),
    } as unknown as typeof window.mermaid;

    render(
      <MarkdownContent
        markdown={["```mermaid", "graph TD", "  A[Line<br/>Break]", "```"].join("\n")}
        sourceFilePath="system-model/Capabilities.md"
      />,
    );

    expect(await screen.findByTestId("mermaid-svg")).toBeTruthy();
    expect(screen.queryByText("Unable to render Mermaid diagram.")).toBeNull();
  });

  it("ends Mermaid panning when pointer capture is lost and detaches listeners on unmount", async () => {
    let captured = false;
    const releasePointerCapture = vi.fn(() => { captured = false; });
    window.mermaid = {
      render: vi.fn().mockResolvedValue({
        svg: '<svg data-testid="mermaid-svg"></svg>',
        bindFunctions: (container: HTMLElement) => Object.assign(container.querySelector("svg")!, {
          getBBox: () => ({ x: 0, y: 0, width: 100, height: 100 }),
          setPointerCapture: () => { captured = true; },
          hasPointerCapture: () => captured,
          releasePointerCapture,
        }),
      }),
    } as unknown as typeof window.mermaid;
    const view = render(<MarkdownContent markdown={["```mermaid", "graph TD", "  A-->B", "```"].join("\n")} sourceFilePath="model.md" />);
    const svg = await screen.findByTestId("mermaid-svg");
    const container = svg.parentElement!;
    const pointer = (type: string) => Object.assign(new Event(type, { bubbles: true }), { button: 0, pointerId: 1, clientX: 10, clientY: 20 });

    fireEvent(svg, pointer("pointerdown"));
    expect(container.classList.contains("is-panning")).toBe(true);
    captured = false;
    fireEvent(svg, pointer("lostpointercapture"));
    expect(container.classList.contains("is-panning")).toBe(false);
    expect(releasePointerCapture).not.toHaveBeenCalled();

    fireEvent(svg, pointer("pointerdown"));
    expect(container.classList.contains("is-panning")).toBe(true);
    fireEvent(svg, pointer("pointercancel"));
    expect(container.classList.contains("is-panning")).toBe(false);
    expect(releasePointerCapture).toHaveBeenCalledWith(1);
    view.unmount();
    fireEvent(svg, pointer("pointerdown"));
    expect(container.classList.contains("is-panning")).toBe(false);
  });

  it("renders element markdown as safe React output", () => {
    render(
      <MarkdownContent
        markdown={[
          "## Rendered heading",
          "",
          "The **system** shall preserve _formatting_.",
          "",
          "| Name | Result |",
          "| --- | --- |",
          "| Markdown | HTML |",
          "",
          "![Diagram](images/diagram.png)",
        ].join("\n")}
        sourceFilePath="specifications/Subfolder/MixedLinkTypes.md"
        sourceAnchor="#/content/specifications/Subfolder/MixedLinkTypes.md#mixed-link-types"
      />,
    );

    expect(
      screen.getByRole("heading", { name: "Rendered heading" }),
    ).toBeTruthy();
    expect(screen.getByText("system").tagName.toLowerCase()).toBe("strong");
    expect(screen.getByText("formatting").tagName.toLowerCase()).toBe("em");
    expect(screen.getByRole("table")).toBeTruthy();
    expect(screen.getByRole("img", { name: "Diagram" }).getAttribute("src")).toBe(
      "specifications/Subfolder/images/diagram.png",
    );
  });

  it("rewrites static-relative markdown links from the source file location", () => {
    render(
      <MarkdownContent
        markdown={[
          "[Sibling](Sibling.md#target)",
          "[Parent](../Parent.md)",
          "[Hash](#local-section)",
        ].join(" ")}
        sourceFilePath="specifications/Subfolder/MixedLinkTypes.md"
        sourceAnchor="#/content/specifications/Subfolder/MixedLinkTypes.md#mixed-link-types"
      />,
    );

    expect(screen.getByRole("link", { name: "Sibling" }).getAttribute("href")).toBe(
      "#/content/specifications/Subfolder/Sibling.md#target",
    );
    expect(screen.getByRole("link", { name: "Parent" }).getAttribute("href")).toBe(
      "#/content/specifications/Parent.md",
    );
    expect(screen.getByRole("link", { name: "Hash" }).getAttribute("href")).toBe(
      "#/content/specifications/Subfolder/MixedLinkTypes.md#local-section",
    );
  });

  it("does not emit raw HTML or active javascript URLs", () => {
    const { container } = render(
      <MarkdownContent
        markdown={[
          "<script>window.__reqvireUnsafe = true</script>",
          "",
          "[Unsafe](javascript:alert(1))",
          "![Unsafe image](javascript:alert(1))",
        ].join("\n")}
        sourceFilePath="specifications/TestRequirements.md"
      />,
    );

    expect(container.querySelector("script")).toBeNull();
    const unsafeLink = container.querySelector("a");
    expect(unsafeLink?.textContent).toBe("Unsafe");
    expect(
      unsafeLink?.getAttribute("href"),
    ).not.toContain("javascript:");
    expect(screen.queryByRole("img", { name: "Unsafe image" })).toBeNull();
  });
});

describe("sourceUrlTransform", () => {
  it("keeps SPA routes and safe external URLs intact", () => {
    const context = {
      sourceFilePath: "system-model/Specifications.md",
      sourceHtmlPath: "#/content/system-model/Specifications.md",
    };

    expect(sourceUrlTransform("#/search/requirement", context)).toBe(
      "#/search/requirement",
    );
    expect(sourceUrlTransform("https://example.com/path.md", context)).toBe(
      "https://example.com/path.md",
    );
  });

  it("does not compose hash fallbacks onto unsafe source page schemes", () => {
    expect(
      sourceUrlTransform("#local", {
        sourceFilePath: "javascript:alert(1).md",
        sourceHtmlPath: "javascript:alert(1).html",
      }),
    ).toBe("#local");
  });
});
