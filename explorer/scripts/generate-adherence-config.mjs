import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const barrelPath = resolve(root, "design-system/index.ts");
const configPath = resolve(root, ".vite/_adherence.oxlintrc.json");

const barrel = readFileSync(barrelPath, "utf8");

function exportedComponentNames(source) {
  const names = [];
  for (const match of source.matchAll(/export\s+\{([\s\S]*?)\}\s+from\s+["'][^"']+["'];/g)) {
    for (const rawPart of match[1].split(",")) {
      const name = rawPart.trim().replace(/\s+as\s+\w+$/, "");
      if (/^[A-Z][A-Za-z0-9]*$/.test(name) && !/^[A-Z0-9_]+$/.test(name)) {
        names.push(name);
      }
    }
  }
  return [...new Set(names)].sort();
}

const components = Object.fromEntries(exportedComponentNames(barrel).map((name) => [name, { replaces: [] }]));

const config = {
  $schema: "https://raw.githubusercontent.com/oxc-project/oxc/main/npm/oxlint/configuration_schema.json",
  rules: {
    "no-restricted-imports": [
      "error",
      {
        patterns: [
          {
            group: [
              "@ds/*",
              "design-system/components/**",
              "../design-system/components/**",
              "../../design-system/components/**",
              "components/controls/**",
              "components/core/**",
              "components/data/**",
              "components/navigation/**",
            ],
            message: "Import design-system components from the @ds public barrel, not @ds/* or component internals.",
          },
        ],
      },
    ],
  },
  ignorePatterns: ["design-system/_ds_bundle.js"],
  overrides: [
    {
      files: ["design-system/index.ts"],
      rules: {
        "no-restricted-imports": "off",
      },
    },
  ],
  settings: {
    reqvireDesignSystem: {
      generatedFrom: "design-system/index.ts",
      components,
    },
  },
};

mkdirSync(dirname(configPath), { recursive: true });
writeFileSync(configPath, `${JSON.stringify(config, null, 2)}\n`);
console.log(`Generated ${configPath} from ${barrelPath} (${Object.keys(components).length} components).`);
