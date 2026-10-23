import { spawn, spawnSync } from "node:child_process";
import { existsSync, readdirSync } from "node:fs";
import os from "node:os";
import path from "node:path";

// Bun supplies a node shim when Node is absent. Verify the actual runtime before
// launching Vite/Vitest: their heap flags must reach V8, not that shim.
const executable = process.platform === "win32" ? "node.exe" : "node";
const candidates = [];
if (process.env.REQVIRE_NODE) candidates.push(process.env.REQVIRE_NODE);
else {
  const hermitNode = path.resolve(import.meta.dirname, "../../bin", executable);
  if (existsSync(hermitNode)) candidates.push(hermitNode);
  if (!process.versions.bun) candidates.push(process.execPath);
  for (const folder of (process.env.PATH ?? "").split(path.delimiter)) if (folder) candidates.push(path.join(folder, executable));
  if (process.env.NVM_BIN) candidates.push(path.join(process.env.NVM_BIN, executable));
  const versions = path.join(process.env.NVM_DIR ?? path.join(os.homedir(), ".nvm"), "versions", "node");
  if (existsSync(versions)) {
    const installed = readdirSync(versions).filter(name => /^v\d+\.\d+\.\d+$/.test(name));
    installed.sort((left, right) => {
      const a = left.slice(1).split(".").map(Number), b = right.slice(1).split(".").map(Number);
      if ((a[0] === 24) !== (b[0] === 24)) return a[0] === 24 ? -1 : 1;
      return b[0] - a[0] || b[1] - a[1] || b[2] - a[2];
    });
    candidates.push(...installed.map(version => path.join(versions, version, "bin", executable)));
  }
}

let node;
const seen = new Set();
for (const candidate of candidates) {
  if (!existsSync(candidate) || seen.has(candidate)) continue;
  seen.add(candidate);
  // Preserve argv[0] for package-manager proxies such as Hermit's bin/node.
  const probe = spawnSync(candidate, ["-p", "JSON.stringify({node:process.versions.node,bun:!!process.versions.bun,execPath:process.execPath})"], { encoding: "utf8", timeout: 15000, maxBuffer: 8192, windowsHide: true });
  try {
    const runtime = JSON.parse(probe.stdout);
    if (probe.status === 0 && !runtime.bun && Number(runtime.node.split(".")[0]) >= 24) { node = runtime.execPath; break; }
  } catch { /* An executable/shim that is not real Node cannot run these tools. */ }
}
if (!node) throw new Error("Reqvire requires real Node 24+ for Vite/Vitest. Put Node on PATH, activate nvm, or set REQVIRE_NODE to its executable. Bun's node shim is not supported.");
if (!process.argv[2]) throw new Error("Specify the Node tool and its arguments.");

const child = spawn(node, process.argv.slice(2), {
  stdio: "inherit", windowsHide: true,
  env: { ...process.env, PATH: [path.dirname(node), process.env.PATH ?? ""].join(path.delimiter) },
});
for (const signal of ["SIGINT", "SIGTERM"]) process.once(signal, () => child.kill(signal));
child.once("error", error => { console.error(error.message); process.exitCode = 1; });
child.once("close", (code, signal) => { process.exitCode = code ?? (signal === "SIGINT" ? 130 : signal === "SIGTERM" ? 143 : 1); });
