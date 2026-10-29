const fs = require('node:fs');
const path = require('node:path');
const tool = path.basename(process.argv[1]);
const args = process.argv.slice(2);
const name = tool === 'ads' ? 'ads' : ['vitest','vitest.mjs'].includes(tool) ? 'unit' : tool === 'tsc' ? 'typecheck' : ['vite','vite.js'].includes(tool)
  ? (args.includes('--config') ? 'kit' : 'app') : args[0];
fs.appendFileSync(process.env.TRACE, JSON.stringify(name) + '\n');
if (name === 'ci') {
  fs.mkdirSync('node_modules/.bin', {recursive: true});
  // Failed installs can leave partial output; this is not a success receipt.
  fs.writeFileSync('node_modules/.bun.lock', '{}');
  for (const binary of ['typescript/bin/tsc', 'vite/bin/vite.js', 'vitest/vitest.mjs']) {
    const target='node_modules/'+binary;fs.mkdirSync(path.dirname(target),{recursive:true});fs.writeFileSync(target, binary.endsWith('.mjs') ? 'import { createRequire } from "node:module";createRequire(import.meta.url)(process.cwd()+"/stage.cjs");' : 'require(process.cwd() + "/stage.cjs");');
  }
  for (const binary of ['tsc', 'vite', 'vitest', 'ads']) {
    fs.writeFileSync('node_modules/.bin/' + binary,
      '#!/usr/bin/env bun\nrequire(process.cwd() + "/stage.cjs");\n', {mode: 0o755});
  }
}
if (process.env.FAIL_STAGE === name || Number(process.env.FAIL_AT) ===
    fs.readFileSync(process.env.TRACE, 'utf8').trim().split('\n').length) process.exit(23);
if (name === 'app' || name === 'kit') {
  const dir = name === 'app' ? 'dist' : 'design-system/dist-kit';
  fs.mkdirSync(dir, {recursive: true});
  fs.writeFileSync(dir + '/built', 'current artifact');
}
