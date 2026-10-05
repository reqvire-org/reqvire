"""Exercise real CLI/worker startup up to an intentionally invalid listener.

Counts come from the model/runtime entry points in both parent and worker stderr.
No listening socket, browser, or skipped infrastructure failure is needed here.
"""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import shutil
import signal

binary = str(Path(sys.argv[1]).resolve())
output = Path(sys.argv[2]).resolve()
output.mkdir(parents=True, exist_ok=True)
# Keep aggregate/empty workspaces outside the runner's enclosing Git fixture.
fixtures = Path(tempfile.mkdtemp(prefix="reqvire-serve-startup-"))
model = "# Elements\n\n### {name}\n\nStartup fixture.\n\n#### Metadata\n  * type: capability\n---\n"


def git(root, *args):
    return subprocess.run(
        ["git", "-C", str(root), *args], check=True, capture_output=True, text=True,
        timeout=10,
    ).stdout


def repo(root, name="Startup Root"):
    root.mkdir(parents=True)
    git(root, "init", "-qb", "main")
    git(root, "config", "user.name", "Startup Test")
    git(root, "config", "user.email", "startup@example.invalid")
    (root / "Model.md").write_text(model.format(name=name))
    git(root, "add", ".")
    git(root, "commit", "-qm", "fixture")
    return root


results = []


def check(name, root, flags, builds, runtimes, error):
    env = {**os.environ, "RUST_LOG": "error,reqvire::model=debug,reqvire::explorer_runtime=debug"}
    process = subprocess.Popen(
        [binary, "--workspace", str(root), "serve", "--host", "::invalid", *flags],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env,
        start_new_session=True,
    )
    try:
        stdout, stderr = process.communicate(timeout=30)
    except subprocess.TimeoutExpired:
        # Kill only this invocation's isolated group, including its pipe worker.
        os.killpg(process.pid, signal.SIGKILL)
        stdout, stderr = process.communicate(timeout=5)
        (output / f"{name}.log").write_text(stdout + stderr)
        raise
    log = stdout + stderr
    (output / f"{name}.log").write_text(log)
    actual = (
        log.count("Starting two-pass validation architecture"),
        log.count("Building Explorer runtime assets from validated model"),
    )
    # A rejected model/admission must not get as far as listener construction.
    diagnostic = error in log and (error == "Failed to start server" or "Failed to start server" not in log)
    passed = process.returncode != 0 and actual == (builds, runtimes) and diagnostic
    results.append(f"{'PASS' if passed else 'FAIL'} {name}")
    if not passed:
        print(f"{name}: expected builds/runtime {(builds, runtimes)} and {error!r}; got {actual}, exit {process.returncode}\n{log}", file=sys.stderr)


for name, flags in [
    ("plain", []),
    ("read-only-mcp", ["--enable-mcp"]),
    ("writable", ["--enable-mcp", "--enable-mutations"]),
    ("auto-commits", ["--enable-mcp", "--enable-mutations", "--enable-commits"]),
]:
    root = repo(fixtures / name)
    check(name, root, flags, 1, 1, "Failed to start server")
    (root / "Model.md").write_text(model.format(name="Broken").replace("capability", "invalid-type"))
    git(root, "add", ".")
    git(root, "commit", "-qm", "invalid fixture")
    check(f"{name}-invalid", root, flags, 1, 0, "invalid-type")
    if "--enable-mutations" in flags:
        (root / "Model.md").write_text(model.format(name="Dirty"))
        check(f"{name}-dirty", root, flags, 0, 0, "clean")

aggregate = fixtures / "aggregate"
repo(aggregate / "first", "First Root")
repo(aggregate / "second", "Second Root")
check("aggregate", aggregate, [], 1, 1, "Failed to start server")
(aggregate / "second" / "Model.md").write_text("# Elements\n\n### Broken\n\n#### Metadata\n  * type: invalid-type\n")
check("aggregate-invalid", aggregate, [], 1, 0, "invalid-type")
empty = fixtures / "empty"
empty.mkdir()
check("no-eligible-worktree", empty, [], 1, 0, "Git")
(output / "checks.txt").write_text("\n".join(results) + "\n")
failed = any(result.startswith("FAIL") for result in results)
if failed:
    print(f"Retained startup fixtures: {fixtures}", file=sys.stderr)
else:
    shutil.rmtree(fixtures)
sys.exit(failed)
