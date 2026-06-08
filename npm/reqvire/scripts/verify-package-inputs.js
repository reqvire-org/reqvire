#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "..");
const requiredFiles = [
  "artifacts/reqvire-linux-x86_64.tar.gz",
  "artifacts/reqvire-darwin-arm64.tar.gz",
  "artifacts/reqvire-darwin-x86_64.tar.gz",
  "explorer-dist/index.html",
  "explorer-dist/assets/explorer.js",
  "explorer-dist/assets/explorer.css",
];

const missing = requiredFiles.filter((file) => !fs.existsSync(path.join(root, file)));

if (missing.length > 0) {
  console.error("Cannot package @reqvire-org/reqvire; missing required files:");
  for (const file of missing) {
    console.error(`  - ${file}`);
  }
  console.error("");
  console.error("Build Explorer first and stage release archives before npm pack/publish.");
  process.exit(1);
}
