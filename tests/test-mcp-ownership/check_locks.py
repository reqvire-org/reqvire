"""Ownership and snapshot regressions using the production worker's private pipes.

These checks need no listening sockets and do not exercise the HTTP transport.
"""
import contextlib
import json
import os
import pathlib
import selectors
import signal
import subprocess
import sys
import tempfile
import traceback


# A disposable supervisor forwards the test-owned pipes to a real worker. The
# parent can die while the worker still has valid input: its locks must survive.
if len(sys.argv) > 1 and sys.argv[1] == "--supervise":
    child = subprocess.Popen([sys.argv[2], "__mcp-worktree-worker"])
    print(json.dumps({"worker_pid": child.pid}), flush=True)
    raise SystemExit(child.wait())

binary = str(pathlib.Path(sys.argv[1]).resolve())
fixture = pathlib.Path(__file__).parent.parent / "test-cache-integration/fixtures/model.md.txt"
failures = []


def git(root, *args):
    return subprocess.check_output(["git", *args], cwd=root, text=True).strip()


def line(process):
    with selectors.DefaultSelector() as selector:
        selector.register(process.stdout, selectors.EVENT_READ)
        assert selector.select(10), "Worker pipe timed out"
    data = process.stdout.readline()
    assert data, f"Worker exited unexpectedly: {process.poll()}"
    return json.loads(data)


@contextlib.contextmanager
def worker(root, commits, supervised=False):
    command = ([sys.executable, __file__, "--supervise", binary] if supervised
               else [binary, "__mcp-worktree-worker"])
    process = subprocess.Popen(command, cwd=root, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                               stderr=subprocess.DEVNULL, start_new_session=True)
    try:
        pid = line(process)["worker_pid"] if supervised else process.pid
        options = dict(worktree_id="lock-test", enable_commits=commits,
                       with_size_estimates=False, explorer=False)
        process.stdin.write((json.dumps(options) + "\n").encode())
        process.stdin.flush()
        yield process, pid, line(process)
    finally:
        # Includes the deliberately surviving worker when the supervisor died.
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.wait(timeout=10)
        process.stdin.close()
        process.stdout.close()


def run(label, check):
    try:
        check()
        print(f"PASS {label}", flush=True)
    except Exception:
        failures.append(label)
        print(f"FAIL {label}", flush=True)
        traceback.print_exc(file=sys.stderr)


def assert_ready(ready):
    assert ready.get("status", {}).get("available") is True, ready


def assert_error(ready, path, operation, contention):
    error = ready.get("error", "")
    assert str(path) in error and operation in error and "os error" in error, error
    assert ("already owned" in error) == contention, error


def cases(commits):
    prefix = f"locks/commits-{'enabled' if commits else 'disabled'}"
    with tempfile.TemporaryDirectory(prefix="reqvire-locks-") as directory:
        root = pathlib.Path(directory)
        git(root, "init", "-qb", "main")
        git(root, "config", "user.email", "locks@example.invalid")
        git(root, "config", "user.name", "Lock Test")
        model = root / "Model.md"
        model.write_bytes(fixture.read_bytes())
        git(root, "add", ".")
        git(root, "commit", "-qm", "baseline")
        before = (model.read_bytes(), git(root, "rev-parse", "HEAD"), (root / ".git/index").read_bytes())
        with worker(root, commits) as (process, _, ready):
            assert_ready(ready)
            process.stdin.close()
            assert process.wait(timeout=10) == 0
        worktree = root / ".git/reqvire-mcp-worktree.lock"
        branch, = [p for p in (root / ".git").glob("reqvire-mcp-*.lock") if p != worktree]
        for path in (branch, worktree):
            path.write_text("retained lock file\n")
        identities = {p: (p.stat().st_ino, p.read_bytes(), p.stat().st_mode) for p in (branch, worktree)}

        def unchanged():
            assert before == (model.read_bytes(), git(root, "rev-parse", "HEAD"), (root / ".git/index").read_bytes())
            assert identities == {p: (p.stat().st_ino, p.read_bytes(), p.stat().st_mode) for p in identities}

        def restart():
            with worker(root, commits) as (_, _, ready):
                assert_ready(ready)
            unchanged()
        run(f"{prefix}/orderly-restart", restart)

        def contention(switch):
            with worker(root, commits) as (_, _, ready):
                assert_ready(ready)
                if switch:
                    git(root, "switch", "-qc", "alternate")
                try:
                    with worker(root, commits) as (_, _, rejected):
                        assert_error(rejected, worktree if switch else branch, "acquire", True)
                finally:
                    if switch:
                        git(root, "switch", "-q", "main")
            restart()
        run(f"{prefix}/branch-contention", lambda: contention(False))
        run(f"{prefix}/worktree-contention", lambda: contention(True))

        def open_error(path):
            # A directory is unopenable as a lock even when tests run as root.
            saved = path.with_suffix(".saved")
            path.rename(saved)
            path.mkdir()
            try:
                with worker(root, commits) as (_, _, rejected):
                    assert_error(rejected, path, "open", False)
                for mode in (["mcp", "--enable-mutations"], ["serve", "--enable-mcp", "--enable-mutations"]):
                    result = subprocess.run([binary, *mode, *(["--enable-commits"] if commits else [])],
                                            cwd=root, capture_output=True, text=True, timeout=10)
                    assert result.returncode != 0, result.stdout
                    assert_error({"error": result.stderr}, path, "open", False)
            finally:
                path.rmdir()
                saved.rename(path)
            # Also reproduce genuine EACCES where the test identity is subject
            # to Unix permissions. Injected Rust cases cover privileged runs.
            if os.geteuid() != 0:
                permissions = path.stat().st_mode
                path.chmod(0)
                try:
                    with worker(root, commits) as (_, _, rejected):
                        assert_error(rejected, path, "open", False)
                        assert "permission denied" in rejected["error"], rejected
                        assert "server/container user" in rejected["error"], rejected
                finally:
                    path.chmod(permissions)
            restart()  # A failed second claim must release its first claim.
        run(f"{prefix}/branch-open-error", lambda: open_error(branch))
        run(f"{prefix}/worktree-open-error", lambda: open_error(worktree))

        def forced_restart():
            with worker(root, commits) as (process, _, ready):
                assert_ready(ready)
                process.kill()
                process.wait(timeout=10)
            restart()
        run(f"{prefix}/forced-restart", forced_restart)

        def surviving_worker():
            with worker(root, commits, supervised=True) as (parent, pid, ready):
                assert_ready(ready)
                parent.kill()
                parent.wait(timeout=10)
                os.kill(pid, 0)
                with worker(root, commits) as (_, _, rejected):
                    assert_error(rejected, branch, "acquire", True)
                os.kill(pid, signal.SIGKILL)
                with selectors.DefaultSelector() as selector:
                    selector.register(parent.stdout, selectors.EVENT_READ)
                    assert selector.select(10), "Worker did not exit"
                assert parent.stdout.read() == b""
            restart()
        run(f"{prefix}/surviving-worker", surviving_worker)


def snapshot_case(commits):
    with tempfile.TemporaryDirectory(prefix="reqvire-snapshot-") as directory:
        root = pathlib.Path(directory)
        git(root, "init", "-qb", "main")
        git(root, "config", "user.email", "snapshot@example.invalid")
        git(root, "config", "user.name", "Snapshot Test")
        (root / "Model.md").write_bytes(fixture.read_bytes())
        git(root, "add", ".")
        git(root, "commit", "-qm", "baseline")
        head = git(root, "rev-parse", "HEAD")
        with worker(root, commits) as (process, _, ready):
            assert_ready(ready)

            def request(operation, **values):
                process.stdin.write((json.dumps(dict(operation=operation, **values)) + "\n").encode())
                process.stdin.flush()
                return line(process)

            def tool(tool_name, **arguments):
                result = request("rpc", method="tools/call", params=dict(name=tool_name, arguments=arguments))
                assert not result.get("error") and not result.get("rpc_error"), result
                return result

            content = """### Snapshot Child

Independent capability for snapshot publication.

#### Metadata
  * type: capability
"""
            for _ in range(3):
                read = tool("reqvire.read_element", name="Cache Subject")
                assert "Source marker alpha" in json.dumps(read["result"]), read
                assert read["status"]["pending_changes"] == []
            preview = tool("reqvire.add_element", file="Added.md", content=content, dry_run=True)
            assert not preview["result"].get("isError"), preview
            assert preview["status"]["pending_changes"] == []
            assert not (root / "Added.md").exists()
            added = tool("reqvire.add_element", file="Added.md", content=content)
            assert not added["result"].get("isError"), added
            expected = [] if commits else ["Added.md"]
            assert added["status"]["pending_changes"] == expected, added
            rejected = tool("reqvire.add_element", file="Added.md", content=content)
            assert rejected["result"]["isError"], rejected
            assert rejected["status"]["pending_changes"] == expected
            (root / "Model.md").write_text("external edit must stay outside the snapshot")
            read = tool("reqvire.read_element", name="Cache Subject")
            assert "Source marker alpha" in json.dumps(read["result"]), read
            removed = tool("reqvire.remove_element", element_name="Snapshot Child")
            assert not removed["result"].get("isError"), removed
            assert removed["status"]["pending_changes"] == [], removed
            assert not (root / "Added.md").exists()
            assert (root / "Model.md").read_text() == "external edit must stay outside the snapshot"
            if not commits:
                assert git(root, "rev-parse", "HEAD") == head
            assert request("commit", message="no pending changes")["result"]["outcome"] == "no_op"


for commits in (False, True):
    cases(commits)
    run(f"snapshots/commits-{'enabled' if commits else 'disabled'}/accepted-state-and-pending-reversion",
        lambda: snapshot_case(commits))
raise SystemExit(bool(failures))
