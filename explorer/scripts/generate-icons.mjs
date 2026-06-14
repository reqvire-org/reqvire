import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import sharp from "sharp";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const source = resolve(root, "design-system/assets/logo-mark.svg");
const outputDir = resolve(root, ".vite/generated-assets");

const pngOutputs = [
  ["logo.png", 200],
  ["apple-touch-icon.png", 180],
  ["android-chrome-192x192.png", 192],
  ["icon-192.png", 192],
  ["icon-512.png", 512],
  ["favicon-32x32.png", 32],
  ["favicon-16x16.png", 16],
  ["mstile-150x150.png", 150],
];

function icoBuffer(entries) {
  const headerSize = 6;
  const entrySize = 16;
  const directorySize = headerSize + entries.length * entrySize;
  const totalSize = directorySize + entries.reduce((sum, entry) => sum + entry.buffer.length, 0);
  const buffer = Buffer.alloc(totalSize);

  buffer.writeUInt16LE(0, 0);
  buffer.writeUInt16LE(1, 2);
  buffer.writeUInt16LE(entries.length, 4);

  let imageOffset = directorySize;
  entries.forEach((entry, index) => {
    const directoryOffset = headerSize + index * entrySize;
    buffer.writeUInt8(entry.size >= 256 ? 0 : entry.size, directoryOffset);
    buffer.writeUInt8(entry.size >= 256 ? 0 : entry.size, directoryOffset + 1);
    buffer.writeUInt8(0, directoryOffset + 2);
    buffer.writeUInt8(0, directoryOffset + 3);
    buffer.writeUInt16LE(1, directoryOffset + 4);
    buffer.writeUInt16LE(32, directoryOffset + 6);
    buffer.writeUInt32LE(entry.buffer.length, directoryOffset + 8);
    buffer.writeUInt32LE(imageOffset, directoryOffset + 12);
    entry.buffer.copy(buffer, imageOffset);
    imageOffset += entry.buffer.length;
  });

  return buffer;
}

async function renderPng(size) {
  return sharp(source, { density: 512 })
    .resize(size, size, { fit: "contain" })
    .png({ compressionLevel: 9, adaptiveFiltering: true })
    .toBuffer();
}

mkdirSync(outputDir, { recursive: true });

const buffers = new Map();
for (const [name, size] of pngOutputs) {
  const buffer = await renderPng(size);
  buffers.set(name, buffer);
  writeFileSync(resolve(outputDir, name), buffer);
}

writeFileSync(
  resolve(outputDir, "favicon.ico"),
  icoBuffer([
    { size: 16, buffer: buffers.get("favicon-16x16.png") },
    { size: 32, buffer: buffers.get("favicon-32x32.png") },
  ]),
);

const manifest = {
  name: "Reqvire Explorer",
  short_name: "Reqvire",
  description: "Browser Explorer for Reqvire system models.",
  icons: [
    { src: "icon-192.png", sizes: "192x192", type: "image/png" },
    { src: "icon-512.png", sizes: "512x512", type: "image/png" },
  ],
  theme_color: "#0f172a",
  background_color: "#f8f7f3",
  display: "standalone",
  start_url: "../",
  scope: "../",
};

writeFileSync(resolve(outputDir, "site.webmanifest"), `${JSON.stringify(manifest, null, 2)}\n`);
writeFileSync(
  resolve(outputDir, "browserconfig.xml"),
  `<?xml version="1.0" encoding="utf-8"?>\n<browserconfig>\n  <msapplication>\n    <tile>\n      <square150x150logo src="mstile-150x150.png"/>\n      <TileColor>#0f172a</TileColor>\n    </tile>\n  </msapplication>\n</browserconfig>\n`,
);

console.log(`Generated ${pngOutputs.length + 3} browser icon assets from ${source} into ${outputDir}.`);
