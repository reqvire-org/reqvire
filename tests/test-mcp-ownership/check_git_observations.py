"""Count real worker Git calls and inject checkout changes at operation boundaries.

Private pipes avoid a listening socket; these checks do not replace HTTP E2Es.
"""
import contextlib
import json
import os
from pathlib import Path
import selectors
import shutil
import subprocess
import sys
import tempfile
import traceback

binary = str(Path(sys.argv[1]).resolve())
real_git = shutil.which("git")
fixture = Path(__file__).parent.parent / "test-cache-integration/fixtures/model.md.txt"
failures = []


def git(root, *args):
    return subprocess.check_output([real_git, *args], cwd=root, text=True).strip()


@contextlib.contextmanager
def worker(read_only=False, commits=False):
    with tempfile.TemporaryDirectory(prefix="reqvire-git-observation-") as directory:
        base = Path(directory)
        root = base / "repo"
        root.mkdir()
        git(root, "init", "-qb", "main")
        git(root, "config", "user.email", "observations@example.invalid")
        git(root, "config", "user.name", "Observation Test")
        (root / "Model.md").write_bytes(fixture.read_bytes())
        git(root, "add", ".")
        git(root, "commit", "-qm", "baseline")
        git(root, "branch", "alternate")
        log = base / "git.jsonl"
        hook = base / "hook.json"
        wrapper = base / "git"
        wrapper.write_text(f"#!{sys.executable}\n" + '''import json, os, pathlib, subprocess, sys
args = sys.argv[1:]
with open(os.environ['GIT_OBSERVATION_LOG'], 'a') as log:
    log.write(json.dumps(args) + '\\n')
hook = pathlib.Path(os.environ['GIT_OBSERVATION_HOOK'])
config = json.loads(hook.read_text()) if hook.exists() else {}
match = all(token in args for token in config.get('match', [])) and bool(config)
if match and config.get('fail'):
    sys.exit(1)
result = subprocess.run([os.environ['REAL_TEST_GIT'], *args], capture_output=True)
if match:
    hook.unlink()
    subprocess.run([os.environ['REAL_TEST_GIT'], *config['run']], check=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
sys.stdout.buffer.write(result.stdout)
sys.stderr.buffer.write(result.stderr)
sys.exit(result.returncode)
''')
        wrapper.chmod(0o755)
        env = dict(os.environ, PATH=str(base) + os.pathsep + os.environ["PATH"],
                   REAL_TEST_GIT=real_git, GIT_OBSERVATION_LOG=str(log), GIT_OBSERVATION_HOOK=str(hook))
        process = subprocess.Popen([binary, "__mcp-worktree-worker"], cwd=root, env=env,
                                   stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.DEVNULL)

        def request(**value):
            process.stdin.write((json.dumps(value) + "\n").encode())
            process.stdin.flush()
            with selectors.DefaultSelector() as selector:
                selector.register(process.stdout, selectors.EVENT_READ)
                assert selector.select(20), "Worker pipe timed out"
            line = process.stdout.readline()
            assert line, f"Worker exited: {process.poll()}"
            return json.loads(line)

        try:
            ready = request(worktree_id="observations", read_only=read_only, enable_commits=commits,
                            with_size_estimates=False, explorer=True)
            assert ready.get("status", {}).get("available"), ready
            yield root, request, log, hook, ready
        finally:
            process.stdin.close()
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=10)
            process.stdout.close()


def tool(request, tool_name, **arguments):
    return request(operation="rpc", method="tools/call", params=dict(name=tool_name, arguments=arguments))


def counted(log, budget, call):
    log.write_text("")
    response = call()
    calls = [json.loads(line) for line in log.read_text().splitlines()]
    print(f"Git call budget {budget}: observed {len(calls)} subprocesses", file=sys.stderr)
    assert len(calls) <= budget, f"Expected <= {budget} Git calls, observed {len(calls)}: {calls}"
    return response


def accepted_reads():
    with worker() as (_, request, log, _, ready):
        for _ in range(2):
            result = counted(log, 2, lambda: tool(request, "reqvire.search", short=True))
            assert not result["result"].get("isError"), result
            assert result["status"]["model_revision"] == ready["status"]["model_revision"]


def runtime_metadata():
    with worker() as (_, request, log, _, _):
        # Runtime content itself observes HEAD/status and the source remote.
        result = counted(log, 4, lambda: request(operation="runtime"))
        store = result["runtime"]["project_store"]
        assert store["project"]["branch"] == "main", store["project"]


def read_only_loads():
    with worker(read_only=True) as (_, request, log, _, _):
        for _ in range(2):
            result = counted(log, 4, lambda: request(operation="load"))
            assert result.get("runtime_unchanged"), result
            assert result["status"]["branch"] == "main"


def live_dirty_state():
    with worker() as (root, request, _, hook, ready):
        for action in ("unstaged", "staged", "untracked", "clean", "status-failure", "recovered"):
            if action == "unstaged":
                (root / "Model.md").write_text("external content stays outside the accepted snapshot")
            elif action == "staged":
                git(root, "add", "Model.md")
            elif action == "untracked":
                git(root, "reset", "--hard", "HEAD")
                (root / "untracked\n# branch.head confusing.txt").write_text("untracked")
            elif action == "clean":
                (root / "untracked\n# branch.head confusing.txt").unlink()
            elif action == "status-failure":
                hook.write_text(json.dumps(dict(match=["status"], fail=True)))
            else:
                hook.unlink()
            response = tool(request, "reqvire.read_element", name="Cache Subject")
            status = response["status"]
            assert "Source marker alpha" in json.dumps(response["result"]), response
            assert status["head"] == ready["status"]["head"] and status["writes_available"], status
            assert status["dirty"] == (None if action == "status-failure" else action in
                                       ("unstaged", "staged", "untracked")), (action, status)


def checkout_changes(commits):
    with worker(commits=commits) as (root, request, _, _, ready):
        head = ready["status"]["head"]
        for command in (("switch", "-q", "alternate"), ("commit", "--allow-empty", "-qm", "external"),
                        ("checkout", "--detach", "-q", head)):
            git(root, *command)
            response = tool(request, "reqvire.search", short=True)
            assert response["result"].get("isError"), response
            assert response["status"]["head"] == head, response
            assert not response["status"]["writes_available"], response
            git(root, "switch", "-q", "main")
            git(root, "reset", "--hard", head)
            assert request(operation="status")["status"]["writes_available"]


def read_only_failures():
    with worker(read_only=True) as (root, request, _, hook, _):
        model = root / "Model.md"
        model.write_bytes(fixture.read_bytes().replace(b"alpha", b"bravo"))
        changed = request(operation="load")
        assert "bravo" in json.dumps(changed["runtime"]["project_store"]), changed
        model.write_bytes(fixture.read_bytes() + fixture.read_bytes())
        invalid = request(operation="load")
        assert invalid.get("runtime_error") and "runtime" not in invalid, invalid
        model.write_bytes(fixture.read_bytes())
        assert "runtime" in request(operation="load")
        # The core cache observes Git after the worker's entry branch check.
        hook.write_text(json.dumps(dict(match=["status", "--porcelain"], run=["switch", "-q", "alternate"])))
        changed = request(operation="load")
        assert "changed during loading" in changed.get("error", ""), changed
        assert changed["status"]["branch"] == "alternate", changed
        assert "runtime" not in changed and "runtime_unchanged" not in changed
        assert "changed outside" in request(operation="load").get("error", "")
        git(root, "switch", "-q", "main")
        assert "runtime" in request(operation="load")


def commit_preparation_race():
    with worker(commits=True) as (root, request, _, hook, ready):
        hook.write_text(json.dumps(dict(match=["commit-tree"], run=["commit", "--allow-empty", "-qm", "external"])))
        response = tool(request, "reqvire.add_element", file="Added.md",
                        content="### Added\n\n#### Metadata\n  * type: capability\n")
        assert response["result"].get("isError"), response
        assert "changed externally" in json.dumps(response), response
        assert not (root / "Added.md").exists()
        assert response["status"]["model_revision"] == ready["status"]["model_revision"]
        assert not response["status"]["writes_available"]


def accepted_read_final_status():
    with worker() as (_, request, _, hook, ready):
        # Return the entry observation, then switch branches before dispatch.
        # The accepted read can finish; its final physical status must be fresh.
        hook.write_text(json.dumps(dict(match=["rev-parse", "HEAD"], run=["switch", "-q", "alternate"])))
        response = tool(request, "reqvire.search", short=True)
        assert not response["result"].get("isError"), response
        assert response["status"]["branch"] == "main", response
        assert response["status"]["head"] == ready["status"]["head"]
        assert not response["status"]["writes_available"], response
        assert "changed externally" in response["status"]["diagnostic"], response


for label, check in (
    ("accepted-read-call-budget", accepted_reads),
    ("runtime-metadata-call-budget", runtime_metadata),
    ("read-only-load-call-budget", read_only_loads),
    ("live-dirty-and-unavailable-metadata", live_dirty_state),
    ("checkout-changes-without-commits", lambda: checkout_changes(False)),
    ("checkout-changes-with-commits", lambda: checkout_changes(True)),
    ("read-only-invalid-reload-and-branch-race", read_only_failures),
    ("commit-preparation-head-race", commit_preparation_race),
    ("accepted-read-final-status-is-fresh", accepted_read_final_status),
):
    try:
        check()
        print(f"PASS git-observations/{label}", flush=True)
    except Exception:
        failures.append(label)
        print(f"FAIL git-observations/{label}", flush=True)
        traceback.print_exc(file=sys.stderr)
raise SystemExit(bool(failures))
