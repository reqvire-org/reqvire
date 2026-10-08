#!/usr/bin/env node

const childProcess = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");

const packageJson = require("../package.json");

function target() {
  if (process.platform === "linux" && process.arch === "x64") {
    return {
      archive: "reqvire-linux-x86_64.tar.gz",
      extracted: "reqvire-linux-x86_64"
    };
  }

  if (process.platform === "darwin" && process.arch === "arm64") {
    return {
      archive: "reqvire-darwin-arm64.tar.gz",
      extracted: "reqvire-darwin-arm64"
    };
  }

  if (process.platform === "darwin" && process.arch === "x64") {
    return {
      archive: "reqvire-darwin-x86_64.tar.gz",
      extracted: "reqvire-darwin-x86_64"
    };
  }

  console.error(
    `Unsupported platform for @reqvire-org/reqvire: ${process.platform}/${process.arch}`
  );
  process.exit(1);
}

const selected = target();
const packageRoot = path.resolve(__dirname, "..");
const archivePath = path.join(packageRoot, "artifacts", selected.archive);
const cacheDir = path.join(
  os.tmpdir(),
  "reqvire-npm",
  `${packageJson.version}-${process.platform}-${process.arch}`
);
const binaryPath = path.join(cacheDir, "reqvire");

if (!fs.existsSync(archivePath)) {
  console.error(`Missing Reqvire archive: ${archivePath}`);
  process.exit(1);
}

if (!fs.existsSync(binaryPath)) {
  // Every invocation owns its extraction. Publish the complete executable with
  // an atomic no-overwrite link; a competing publisher keeps its winning inode.
  let staging;
  try {
    fs.mkdirSync(cacheDir, { recursive: true });
    staging = fs.mkdtempSync(path.join(path.dirname(cacheDir), ".extract-"));
    const extract = childProcess.spawnSync("tar", ["-xzf", archivePath, "-C", staging], {
      stdio: "inherit"
    });
    if (extract.error) throw extract.error;
    if (extract.status !== 0) {
      throw new Error(`Reqvire archive extraction failed (${extract.signal || extract.status})`);
    }

    const extractedPath = path.join(staging, selected.extracted);
    if (!fs.lstatSync(extractedPath).isFile()) {
      throw new Error("Reqvire archive does not contain a regular native executable");
    }
    fs.chmodSync(extractedPath, 0o755);
    try {
      fs.linkSync(extractedPath, binaryPath);
    } catch (error) {
      if (error.code !== "EEXIST") throw error;
      if (!fs.lstatSync(binaryPath).isFile()) {
        throw new Error(`Invalid Reqvire executable cache: ${binaryPath}`);
      }
      fs.accessSync(binaryPath, fs.constants.X_OK);
    }
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  } finally {
    if (staging) fs.rmSync(staging, { recursive: true, force: true });
  }
  if (process.exitCode) process.exit(process.exitCode);
}

const result = childProcess.spawnSync(binaryPath, process.argv.slice(2), {
  stdio: "inherit"
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

if (result.signal) {
  process.kill(process.pid, result.signal);
}

process.exit(result.status === null ? 1 : result.status);
