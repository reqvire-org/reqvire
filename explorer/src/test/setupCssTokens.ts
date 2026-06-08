import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { beforeEach } from "vitest";

const colorsCss = readFileSync(
  resolve(process.cwd(), "design-system/tokens/colors.css"),
  "utf8",
);

const rootBlock = colorsCss.match(/:root\s*\{([\s\S]*?)\n\}/)?.[1] ?? "";
const rootTokens = Array.from(rootBlock.matchAll(/(--[a-z0-9-]+)\s*:\s*([^;]+);/gi)).map(
  ([, token, value]) => [token, value.trim()] as const,
);

beforeEach(() => {
  document.documentElement.classList.remove("dark");
  document.documentElement.removeAttribute("data-theme");
  for (const [token, value] of rootTokens) {
    document.documentElement.style.setProperty(token, value);
  }
});
