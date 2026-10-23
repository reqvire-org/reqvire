"""Exercise real bun/Make orchestration; replace costly leaf tools with recording tools."""
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

root = Path(__file__).resolve().parents[2]
output = Path(sys.argv[1]).resolve()
output.mkdir(parents=True, exist_ok=True)
bun = shutil.which("bun")
assert bun, "Bun must be on PATH"
guards = ["lint:ds", "lint:style"]
checked = [*guards, "typecheck"]
built = ["lint:ds", "assets:ds", "lint:style", "typecheck", "unit", "icons", "app", "icons", "previews", "ads"]
checks = []


def record(name, run):
    try:
        run()
        checks.append(f"PASS {name}")
    except Exception as error:
        checks.append(f"FAIL {name}")
        print(f"{name}: {error}", file=sys.stderr)


def fixture(name):
    repo = output / name
    app = repo / "explorer"
    app.mkdir(parents=True)
    package = json.loads((root / "explorer/package.json").read_text())
    # Keep the production composition and real bun dispatcher, replacing only leaves.
    package.pop("dependencies", None)
    package.pop("devDependencies", None)
    for guard in [*guards, "assets:ds"]:
        package["scripts"][guard] = f"bun stage.cjs {guard}"
    package["scripts"]["generate:icons"] = "bun stage.cjs icons"
    package["scripts"]["build:mcp-previews"] = "bun stage.cjs previews"
    (app / "package.json").write_text(json.dumps(package))
    (app / "bun.lock").write_text('{}')
    shutil.copy(root / "Makefile", repo / "Makefile")
    (app / "scripts").mkdir()
    shutil.copy(root / "explorer/scripts/run-node.mjs", app / "scripts/run-node.mjs")
    trace = repo / "trace.jsonl"
    stage = app / "stage.cjs"
    stage.write_text('''const fs = require('node:fs');
const path = require('node:path');
const tool = path.basename(process.argv[1]);
const args = process.argv.slice(2);
const name = tool === 'ads' ? 'ads' : ['vitest','vitest.mjs'].includes(tool) ? 'unit' : tool === 'tsc' ? 'typecheck' : ['vite','vite.js'].includes(tool)
  ? (args.includes('--config') ? 'kit' : 'app') : args[0];
fs.appendFileSync(process.env.TRACE, JSON.stringify(name) + '\\n');
if (name === 'ci') {
  fs.mkdirSync('node_modules/.bin', {recursive: true});
  // bun can leave partial output on failure; it must not be a success receipt.
  fs.writeFileSync('node_modules/.bun.lock', '{}');
  for (const binary of ['typescript/bin/tsc', 'vite/bin/vite.js', 'vitest/vitest.mjs']) {
    const target='node_modules/'+binary;fs.mkdirSync(path.dirname(target),{recursive:true});fs.writeFileSync(target, binary.endsWith('.mjs') ? 'import { createRequire } from \"node:module\";createRequire(import.meta.url)(process.cwd()+\"/stage.cjs\");' : 'require(process.cwd() + \"/stage.cjs\");');
  }
  for (const binary of ['tsc', 'vite', 'vitest', 'ads']) {
    fs.writeFileSync('node_modules/.bin/' + binary,
      '#!/usr/bin/env bun\\nrequire(process.cwd() + "/stage.cjs");\\n', {mode: 0o755});
  }
}
if (process.env.FAIL_STAGE === name || Number(process.env.FAIL_AT) ===
    fs.readFileSync(process.env.TRACE, 'utf8').trim().split('\\n').length) process.exit(23);
if (name === 'app' || name === 'kit') {
  const dir = name === 'app' ? 'dist' : 'design-system/dist-kit';
  fs.mkdirSync(dir, {recursive: true});
  fs.writeFileSync(dir + '/built', 'current artifact');
}
''')
    env = {**os.environ, "TRACE": str(trace), "BUN_INSTALL_CACHE_DIR": str(repo / "bun-cache")}
    subprocess.run([bun, str(stage), "ci"], cwd=app, env=env, check=True)
    trace.unlink()
    return repo, app, trace, env


def calls(trace):
    return [json.loads(line) for line in trace.read_text().splitlines()] if trace.exists() else []


def invoke(app, args, env):
    result = subprocess.run(args, cwd=app, env=env, text=True, capture_output=True)
    (app.parent / "last-command.log").write_text(result.stdout + result.stderr)
    return result


def composition(command, expected):
    _, app, trace, env = fixture(command.replace(':', '-'))
    result = invoke(app, [bun, "run", command], env)
    assert result.returncode == 0, result.stdout + result.stderr
    assert calls(trace) == expected, calls(trace)
    assert (app / "dist/built").exists() == ("app" in expected)
    assert (app / "design-system/dist-kit/built").exists() == ("kit" in expected)


def failures():
    expected = [*built, "kit"]
    for index, stage in enumerate(expected):
        _, app, trace, env = fixture(f"failure-{index}-" + stage.replace(':', '-'))
        result = invoke(app, [bun, "run", "build:all"], {**env, "FAIL_AT": str(index + 1)})
        assert result.returncode != 0, f"accepted {stage} failure"
        assert calls(trace) == expected[:index + 1], (index, stage, calls(trace))


def make_dependencies():
    repo, app, trace, env = fixture("make")
    commands = repo / "commands"
    commands.mkdir()
    # Exercise the real Makefile; bun install --frozen-lockfile records/fails without registry access.
    stub = commands / "bun"
    stub.write_text('#!/bin/sh\nif [ "$1" = install ]; then exec "$REAL_BUN" stage.cjs ci; fi\nexec "$REAL_BUN" "$@"\n')
    stub.chmod(0o755)
    env.update(PATH=str(commands) + os.pathsep + env["PATH"], REAL_BUN=bun)

    def make(fail=False, default=False):
        if trace.exists():
            trace.unlink()
        return invoke(repo, ["make"] if default else ["make", "explorer"], {**env, "FAIL_STAGE": "ci" if fail else ""})

    assert make(default=True).returncode == 0
    assert calls(trace) == ["ci", *built], calls(trace)
    assert make().returncode == 0
    assert calls(trace) == built, calls(trace)
    for filename in ["package.json", "bun.lock"]:
        path = app / filename
        path.write_text(path.read_text() + "\n")
        assert make().returncode == 0
        assert calls(trace) == ["ci", *built], (filename, calls(trace))
        assert make().returncode == 0
        assert "ci" not in calls(trace), calls(trace)
    # A failed reinstall must also retire an older success receipt, including
    # failures that occur before bun has removed the previous node_modules.
    lock = app / "bun.lock"
    lock.write_text(lock.read_text() + "\n")
    assert make(True).returncode != 0
    assert calls(trace) == ["ci"], calls(trace)
    assert make().returncode == 0
    assert calls(trace) == ["ci", *built], calls(trace)
    shutil.rmtree(app / "node_modules")
    assert make().returncode == 0
    assert calls(trace) == ["ci", *built], calls(trace)


def workflow_stages():
    # Count the stages invoked by the owned literal bun compositions, and require
    # prior bundle/check stages in each Cargo-consuming CI job. No YAML dependency.
    scripts = json.loads((root / "explorer/package.json").read_text())["scripts"]

    def expand(command):
        stages = []
        for part in command.split("&&"):
            part = part.strip()
            if part.startswith("bun run "):
                name = part.split()[2]
                if "pre" + name in scripts:
                    stages.extend(expand(scripts["pre" + name]))
                if name in [*guards, "assets:ds", "typecheck", "generate:icons", "build:ds-bundle", "test:ads"]:
                    stages.append(name)
                else:
                    stages.extend(expand(scripts[name]))
            elif part == "bun node_modules/typescript/bin/tsc --noEmit":
                stages.append("typecheck")
            elif part == "bun run test":
                stages.extend(expand(scripts["test"]))
            elif part == "bun scripts/run-node.mjs --max-old-space-size=1200 node_modules/vitest/vitest.mjs run":
                stages.append("unit")
            elif part == "bun scripts/run-node.mjs --max-old-space-size=1600 node_modules/vite/bin/vite.js build":
                stages.append("app")
        return stages

    for filename, jobs in [("pr.yml", ["test"]), ("release.yml", ["build", "release"]), ("pages.yml", ["deploy"])]:
        text = (root / ".github/workflows" / filename).read_text()
        for job in jobs:
            block = re.split(r"^  " + job + r":\s*$", text, flags=re.M)[1]
            block = re.split(r"^  [a-zA-Z_][\w-]*:\s*$", block, maxsplit=1, flags=re.M)[0]
            assert "REQVIRE_BUILD_EXPLORER" not in block, f"{filename}/{job} rebuilds inside Cargo"
            job_env = re.search(r"^    env:\n((?:^      .*\n)+)", block, re.M)
            assert not job_env or not re.search(r"\$\{\{\s*runner\.", job_env[1]), f"{filename}/{job} uses runner context before a runner is available"
            stages = []
            for match in re.finditer(r"^        run: (cd explorer && .+)$", block, re.M):
                stages.extend(expand(match[1]))
            assert all(stages.count(stage) == 1 for stage in [*guards, "typecheck", "unit", "app", "test:ads"]), (filename, job, stages)
            assert stages.index("app") > max(stages.index(stage) for stage in [*guards, "typecheck", "unit"])
            assert stages.index("test:ads") > stages.index("app")
            assert stages.count("build:ds-bundle") == (1 if filename == "pr.yml" else 0), (filename, job, stages)
            bundle = block.find("run: cd explorer && bun run build")
            browser = re.search(r"^        run: cd explorer && .*playwright install[^\n]*chromium", block, re.M)
            assert browser and browser.start() < bundle, f"{filename}/{job} must provision Chromium before the checked browser build"
            cargo = block.find("run: cargo build")
            if cargo >= 0:
                assert 0 <= bundle < cargo


for command, expected in [
    ("build", built), ("check", checked),
    ("build:app", ["icons", "app"]), ("build:ds-bundle", ["kit"]),
    ("build:all", [*built, "kit"]),
]:
    record("pipeline-" + command, lambda c=command, e=expected: composition(c, e))
record("pipeline-stops-on-failure", failures)
record("make-dependency-reuse-and-retry", make_dependencies)
record("ci-builds-and-checks-once", workflow_stages)
(output / "checks.txt").write_text("\n".join(checks) + "\n")
sys.exit(any(check.startswith("FAIL") for check in checks))
