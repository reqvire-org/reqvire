import { expect, it } from "vitest";
it("loads real foundation tokens and applies dark overrides without inline token writes", () => {
 const root = document.documentElement;
 const read = (name: string) => getComputedStyle(root).getPropertyValue(name).trim();
 expect(read("--bg-canvas")).not.toBe("");
 // Roles alias ramp steps; a theme redefines the ramps under the same names.
 const light = read("--warm-2");
 expect(light).not.toBe("");
 expect(read("--space-8")).not.toBe("");
 expect(read("--font-sans")).not.toBe("");
 expect(read("--dur-normal")).not.toBe("");
 root.setAttribute("data-theme", "dark");
 expect(read("--warm-2")).not.toBe(light);
 expect(root.getAttribute("style")).toBeNull();
});
