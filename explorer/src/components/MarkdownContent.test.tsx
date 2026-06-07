import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import {
  MarkdownContent,
  staticExportUrlTransform,
} from "./MarkdownContent";

describe("MarkdownContent", () => {
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
        sourceAnchor="specifications/Subfolder/MixedLinkTypes.html#mixed-link-types"
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
        sourceAnchor="specifications/Subfolder/MixedLinkTypes.html#mixed-link-types"
      />,
    );

    expect(screen.getByRole("link", { name: "Sibling" }).getAttribute("href")).toBe(
      "specifications/Subfolder/Sibling.html#target",
    );
    expect(screen.getByRole("link", { name: "Parent" }).getAttribute("href")).toBe(
      "specifications/Parent.html",
    );
    expect(screen.getByRole("link", { name: "Hash" }).getAttribute("href")).toBe(
      "specifications/Subfolder/MixedLinkTypes.html#local-section",
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
    expect(
      screen.getByRole("img", { name: "Unsafe image" }).getAttribute("src"),
    ).not.toContain("javascript:");
  });
});

describe("staticExportUrlTransform", () => {
  it("keeps SPA routes and safe external URLs intact", () => {
    const context = {
      sourceFilePath: "requirements/Specifications.md",
      sourceHtmlPath: "requirements/Specifications.html",
    };

    expect(staticExportUrlTransform("#/search/requirement", context)).toBe(
      "#/search/requirement",
    );
    expect(staticExportUrlTransform("https://example.com/path.md", context)).toBe(
      "https://example.com/path.md",
    );
  });

  it("does not compose hash fallbacks onto unsafe source page schemes", () => {
    expect(
      staticExportUrlTransform("#local", {
        sourceFilePath: "javascript:alert(1).md",
        sourceHtmlPath: "javascript:alert(1).html",
      }),
    ).toBe("#local");
  });
});
