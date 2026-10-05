"""Execute the repository's literal workflow shell steps in isolated fixtures."""
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

root = Path(__file__).resolve().parents[2]
binary = Path(os.environ.get("REAL_REQVIRE_BIN", sys.argv[1])).resolve()
checks = []


def steps(text):
    # These owned workflows use named steps and literal run blocks. Reject
    # unsupported shell styles rather than silently ignoring their commands.
    matches = list(re.finditer(r"^      - name: (.+)$", text, re.M))
    result = []
    for index, match in enumerate(matches):
        block = text[match.end():matches[index + 1].start() if index + 1 < len(matches) else len(text)]
        run = re.search(r"^        run: (.*)$", block, re.M)
        if not run:
            continue
        command = run[1]
        if command == "|":
            lines = block[run.end():].splitlines()[1:]
            command = "\n".join(line[10:] for line in lines if line.startswith("          "))
        elif command.startswith((">", "|")):
            raise AssertionError(f"Unsupported run block for {match[1]}")
        result.append((match[1], command, block))
    return result


def execute(command, folder, env):
    return subprocess.run(["bash", "--noprofile", "--norc", "-e", "-o", "pipefail", "-c", command],
                          cwd=folder, env={**os.environ, **env}, text=True, capture_output=True, timeout=60)


def record(name, check):
    try:
        check()
        checks.append(f"PASS {name}")
    except Exception as error:
        checks.append(f"FAIL {name}")
        print(f"{name}: {error}", file=sys.stderr)


pr = (root / ".github/workflows/pr.yml").read_text()
release = (root / ".github/workflows/release.yml").read_text()


def workers(workflow, stale=False, fail=None):
    selected = [(name, cmd, block) for name, cmd, block in steps(workflow)
                if "cargo test " in cmd or ("cargo build " in cmd and "--release" not in cmd)]
    with tempfile.TemporaryDirectory(prefix="reqvire-workflow-check-") as temp:
        folder = Path(temp)
        commands = folder / "commands"
        commands.mkdir()
        target = folder / "target/debug"
        target.mkdir(parents=True)
        executable = target / "reqvire"
        if stale:
            executable.write_text("#!/bin/sh\necho stale-worker\n")
            executable.chmod(0o755)
        cargo = commands / "cargo"
        cargo.write_text('''#!/bin/bash
set -eu
echo "$1" >> "$TRACE"
case "$1" in
  build)
    [[ "${FAIL_STAGE:-}" != build ]] || exit 7
    printf '#!/bin/sh\\necho current-worker\\n' > target/debug/reqvire
    chmod +x target/debug/reqvire
    ;;
  test)
    [[ "$REQVIRE_TEST_BIN" == "$PWD/target/debug/reqvire" ]]
    [[ "$("$REQVIRE_TEST_BIN")" == current-worker ]]
    [[ "${FAIL_STAGE:-}" != test ]] || exit 9
    ;;
esac
''')
        cargo.chmod(0o755)
        trace = folder / "commands.txt"
        statuses = []
        for _, command, block in selected:
            env = {"PATH": f"{commands}:{os.environ['PATH']}", "TRACE": str(trace), "FAIL_STAGE": fail or "", "RUNNER_OS": "Linux"}
            worker = re.search(r"^          REQVIRE_TEST_BIN: (.+)$", block, re.M)
            if worker:
                env["REQVIRE_TEST_BIN"] = worker[1].strip('"').replace("${{ github.workspace }}", temp)
            result = execute(command, folder, env)
            statuses.append(result.returncode)
            if result.returncode:
                break
        calls = trace.read_text().splitlines() if trace.exists() else []
        assert calls == (["build"] if fail == "build" else ["build", "test"]), (calls, result.stderr)
        assert statuses[-1] == ({"build": 7, "test": 9}.get(fail, 0)), (statuses, result.stderr)


def policy():
    trigger = pr.split("jobs:", 1)[0]
    assert "pull_request:" in trigger and "branches:" not in trigger, "stacked PR targets are excluded"
    assert "name: Run Rust Tests" in pr, "required check renamed"
    validation = pr.split("  validate:", 1)[1]
    assert "continue-on-error:" not in validation, "validation errors are tolerated"
    assert "scripts/install.sh" not in validation, "released validator used"
    assert "needs: test" in validation, "validator does not depend on this run's build"
    assert "uses: actions/download-artifact@" in validation, "current compiled CLI is not consumed"
    assert pr.count("name: reqvire-pr-cli") == 2, "built and downloaded artifact names differ"


def model_gate(invalid):
    command = next(cmd for name, cmd, _ in steps(pr) if name == "Validate Requirements via validate command")
    assert "./target/debug/reqvire validate" in command, "validation must use current compiled binary"
    with tempfile.TemporaryDirectory(prefix="reqvire-validation-gate-") as temp:
        folder = Path(temp)
        target = folder / "target/debug"
        target.mkdir(parents=True)
        (target / "reqvire").symlink_to(binary)
        content = (root / "tests/test-cache-integration/fixtures/model.md.txt").read_text()
        (folder / "Model.md").write_text(content.replace("type: capability", "type: invalid-type") if invalid else content)
        (folder / ".reqvireignore").write_text("reports/**\ntarget/**\n")
        for args in (["init", "-q"], ["config", "user.name", "Workflow Test"], ["config", "user.email", "test@example.invalid"], ["add", "."], ["-c", "commit.gpgSign=false", "commit", "-qm", "fixture"]):
            subprocess.run(["git", *args], cwd=folder, check=True, capture_output=True)
        result = execute(command, folder, {})
        assert (result.returncode != 0) == invalid, result.stdout + result.stderr
        report = folder / "reports/validation_report.txt"
        assert report.is_file() and report.stat().st_size > 0, "validation diagnostics not retained"


for label, workflow in (("pr", pr), ("release", release)):
    for stale in (False, True):
        record(f"{label}-{'stale' if stale else 'clean'}-worker", lambda w=workflow, s=stale: workers(w, s))
    for fail in ("build", "test"):
        record(f"{label}-{fail}-failure", lambda w=workflow, f=fail: workers(w, fail=f))
record("pr-current-revision-policy", policy)
record("valid-model-gate", lambda: model_gate(False))
record("invalid-model-fails-gate", lambda: model_gate(True))
Path(sys.argv[2]).write_text("\n".join(checks) + "\n")
