import { expect, it } from "vitest";
it("loads real foundation tokens and switches theme halves without inline token writes", () => {
 const root = document.documentElement;
 const read = (name: string, element: Element = root) => getComputedStyle(element).getPropertyValue(name).trim();
 expect(read("--bg-canvas")).not.toBe("");
 expect(read("--space-8")).not.toBe("");
 expect(read("--font-sans")).not.toBe("");
 expect(read("--dur-normal")).not.toBe("");
 // A themed ramp step has one half per theme; the generated sheet resolves the step from them.
 expect(read("--warm-2-light")).not.toBe("");
 expect(read("--warm-2-dark")).not.toBe(read("--warm-2-light"));
 expect(read("--warm-2")).toContain("var(--warm-2-light)");
 expect(read("--warm-2")).toContain("var(--warm-2-dark)");
 // The generated theme switch: --display-dark is none outside the dark theme.
 const lightSwitch = read("--display-dark");
 expect(lightSwitch).toBe("none");
 root.setAttribute("data-theme", "dark");
 expect(read("--display-dark")).not.toBe(lightSwitch);
 // A section follows its own data-theme inside another theme.
 const section = document.createElement("section");
 section.setAttribute("data-theme", "light");
 document.body.append(section);
 try {
  expect(read("--display-dark", section)).toBe(lightSwitch);
 } finally { section.remove(); }
 expect(root.getAttribute("style")).toBeNull();
});
