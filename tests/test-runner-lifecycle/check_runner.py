"""Exercise the real shell runner in disposable repositories without sockets."""
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time


RUNNER_DIR = Path(__file__).resolve().parents[1]
CHECKS = []


class Harness:
    def __init__(self, root):
        self.root = root
        self.suites = root / "suites"
        self.suites.mkdir()
        self.temp = root / "fixtures"
        self.temp.mkdir()
        self.logs = root / "retained logs"
        self.observations = root / "observations"
        self.observations.mkdir()
        for name in ("run_tests.sh", "reqvire_timing_wrapper.sh", "stop_test_processes.py"):
            shutil.copy2(RUNNER_DIR / name, self.suites / name)
        self.binary = root / "reqvire stub"
        self.binary.write_text('#!/bin/bash\nprintf "stub invocation\\n"\n')
        self.binary.chmod(0o755)
        self.env = {
            **os.environ,
            "REQVIRE_BIN": str(self.binary),
            "TMPDIR": str(self.temp),
            "REQVIRE_TEST_LOG_DIR": str(self.logs),
            "RUNNER_OBSERVATIONS": str(self.observations),
            "GIT_CONFIG_GLOBAL": os.devnull,
            "GIT_CONFIG_NOSYSTEM": "1",
        }

    def suite(self, name, body):
        folder = self.suites / name
        folder.mkdir()
        (folder / "test.sh").write_text('''#!/bin/bash
set -euo pipefail
printf '%s\\n' "$TEST_DIR" > "$RUNNER_OBSERVATIONS/$RUNNER_CASE.fixture"
git -C "$TEST_DIR" rev-parse --verify HEAD >/dev/null
''' + body)

    def start(self, case, suite=None, **extra):
        return subprocess.Popen(
            ["bash", str(self.suites / "run_tests.sh"), *([suite] if suite else [])],
            env={**self.env, "RUNNER_CASE": case, **extra},
            cwd=self.root,
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
            start_new_session=True,
        )

    def finish(self, process, expected=0):
        output, _ = process.communicate(timeout=12)
        assert process.returncode == expected, output
        return output

    def fixture(self, case):
        return Path((self.observations / f"{case}.fixture").read_text().strip())


def reported_path(output, label):
    match = re.search(re.escape(label) + r": (.+)", output)
    assert match, output
    return Path(match[1])


def stopped(pid):
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return True
    # Linux may briefly retain an orphan as a zombie; it is no longer executing.
    stat = Path(f"/proc/{pid}/stat")
    return stat.exists() and stat.read_text().split(") ", 1)[1].startswith("Z")


def wait_until(predicate, seconds=5):
    deadline = time.monotonic() + seconds
    while not predicate():
        assert time.monotonic() < deadline, "condition timed out"
        time.sleep(0.02)


def check(name, function):
    try:
        with tempfile.TemporaryDirectory(prefix="reqvire-runner-check-") as folder:
            function(Harness(Path(folder)))
        CHECKS.append(f"PASS {name}")
    except Exception as error:
        CHECKS.append(f"FAIL {name}")
        print(f"{name}: {error}", file=sys.stderr)


def success(h):
    h.suite("test-pass", '"$REQVIRE_BIN" --version\n')
    output = h.finish(h.start("pass", "test-pass"))
    run_root = reported_path(output, "Temporary directory")
    fixture = h.fixture("pass")
    assert fixture.parent == run_root, f"fixture escaped run root: {fixture} != {run_root}"
    assert not fixture.exists() and not run_root.exists(), "successful fixture leaked"
    logs = reported_path(output, "Test logs directory")
    assert logs.parent == h.logs and (logs / "test-pass.log").exists(), output
    assert "stub invocation" in (logs / "test-pass.log").read_text()


def failure(h):
    h.suite("test-fail", 'printf "failure evidence\\n" > "$TEST_DIR/output/evidence.txt"\nexit 7\n')
    output = h.finish(h.start("fail", "test-fail"), expected=7)
    fixture = h.fixture("fail")
    assert str(fixture) in output, "failed fixture path not reported"
    assert fixture.parent == reported_path(output, "Temporary directory")
    assert (fixture / "output/evidence.txt").read_text() == "failure evidence\n"
    assert "Tests: 0 passed, 1 failed, 1 total" in output, output


def concurrency(h):
    h.suite("test-same", 'printf "%s\\n" "$RUNNER_CASE"\nsleep 0.2\n')
    a = h.start("first", "test-same")
    b = h.start("second", "test-same")
    out_a, out_b = h.finish(a), h.finish(b)
    logs_a = reported_path(out_a, "Test logs directory")
    logs_b = reported_path(out_b, "Test logs directory")
    assert logs_a != logs_b, "concurrent runs share log directory"
    assert (logs_a / "test-same.log").read_text() == "first\n"
    assert (logs_b / "test-same.log").read_text() == "second\n"
    assert h.fixture("first") != h.fixture("second")
    assert not h.fixture("first").exists() and not h.fixture("second").exists()


def executable_snapshot(h):
    replacement = h.observations / "replacement build"
    replacement.write_text('#!/bin/bash\nprintf "replacement invocation\\n"\n')
    replacement.chmod(0o755)
    h.suite("test-a-snapshot", '''
printf '%s\\n' "$REAL_REQVIRE_BIN" > "$RUNNER_OBSERVATIONS/executable"
[[ "$("$REQVIRE_BIN" --version)" == "stub invocation" ]]
rm -- "$RUNNER_CONFIGURED_BINARY"
[[ "$("$REQVIRE_BIN" --version)" == "stub invocation" ]]
cp -- "$RUNNER_OBSERVATIONS/replacement build" "$RUNNER_CONFIGURED_BINARY"
[[ "$("$REQVIRE_BIN" --version)" == "stub invocation" ]]
''')
    h.suite("test-b-snapshot", '[[ "$("$REQVIRE_BIN" --version)" == "stub invocation" ]]\n')
    output = h.finish(h.start("snapshot", RUNNER_CONFIGURED_BINARY=str(h.binary)))
    executable = Path((h.observations / "executable").read_text().strip())
    assert executable != h.binary and not executable.exists(), "private executable not cleaned up"
    assert "Tests: 2 passed, 0 failed, 2 total" in output, output
    h.suite("test-c-next-build", '[[ "$("$REQVIRE_BIN" --version)" == "replacement invocation" ]]\n')
    h.finish(h.start("next-build", "test-c-next-build"))


def invalid_executable(h, missing):
    h.suite("test-startup", 'touch "$RUNNER_OBSERVATIONS/body-ran"\n')
    if missing:
        h.binary.unlink()
    else:
        h.binary.chmod(0o644)
    output = h.finish(h.start("startup", "test-startup"), expected=127)
    assert "Reqvire executable" in output, output
    assert not (h.observations / "body-ran").exists(), "test ran without a captured executable"
    assert not list(h.temp.iterdir()), "failed startup leaked run scratch files"


def interruption(h, detached=False, stubborn=False):
    body = '''sleep 60 &
descendant=$!
printf '%s %s\\n' "$BASHPID" "$descendant" > "$RUNNER_OBSERVATIONS/ready"
wait "$descendant"
'''
    if detached:
        body = '''python3 - <<'CHILD'
import os
from pathlib import Path
import subprocess
import sys
import time
child = subprocess.Popen([sys.executable, '-c',
    "import os, signal, time; from pathlib import Path; "
    "signal.signal(signal.SIGTERM, signal.SIG_IGN if os.environ.get('STUBBORN_CHILD') else signal.SIG_DFL); "
    "Path(os.environ['RUNNER_OBSERVATIONS'], 'armed').touch(); time.sleep(60)"],
    start_new_session=True)
while not Path(os.environ['RUNNER_OBSERVATIONS'], 'armed').exists():
    assert child.poll() is None
    time.sleep(0.01)
Path(os.environ['RUNNER_OBSERVATIONS'], 'ready').write_text(f'{os.getpid()} {child.pid}')
child.wait()
CHILD
'''
    h.suite("test-wait", body)
    unrelated = subprocess.Popen(["sleep", "60"], start_new_session=True)
    process = h.start("interrupted", "test-wait", STUBBORN_CHILD="1" if stubborn else "")
    pids = []
    try:
        ready = h.observations / "ready"
        wait_until(lambda: ready.exists() and len(ready.read_text().split()) == 2)
        pids = [int(pid) for pid in ready.read_text().split()]
        process.send_signal(signal.SIGTERM)
        output = h.finish(process, expected=143)
        wait_until(lambda: all(stopped(pid) for pid in pids))
        assert not h.fixture("interrupted").exists(), "interrupted fixture leaked"
        assert not reported_path(output, "Temporary directory").exists()
        assert reported_path(output, "Test logs directory").is_dir()
        assert unrelated.poll() is None, "unrelated process was stopped"
    finally:
        unrelated.terminate()
        unrelated.wait(timeout=3)
        # Also clean up the unfixed runner when this regression intentionally fails.
        for pid in [*pids, process.pid]:
            try:
                os.killpg(os.getpgid(pid), signal.SIGKILL)
            except ProcessLookupError:
                pass
        process.communicate(timeout=3)


def setup_failure(h):
    h.suite("test-setup", 'touch "$RUNNER_OBSERVATIONS/body-ran"\n')
    commands = h.root / "commands"
    commands.mkdir()
    git = commands / "git"
    git.write_text("#!/bin/bash\nexit 19\n")
    git.chmod(0o755)
    output = h.finish(h.start("setup", "test-setup", PATH=f"{commands}:{os.environ['PATH']}"), expected=19)
    assert not (h.observations / "body-ran").exists(), "test body ran after failed setup"
    assert not (h.observations / "setup.fixture").exists(), "test entered after failed setup"
    assert "fixture setup" in output.lower(), output
    assert "Tests: 0 passed, 1 failed, 1 total" in output


def cleanup_failure(h):
    h.suite("test-a-cleanup", '''sleep 60 </dev/null >/dev/null 2>&1 &
printf '%s\\n' "$!" > "$RUNNER_OBSERVATIONS/child"
''')
    h.suite("test-b-next", 'touch "$RUNNER_OBSERVATIONS/next-ran"\n')
    commands = h.root / "commands"
    commands.mkdir()
    ps = commands / "ps"
    ps.write_text('#!/bin/bash\nprintf "attempt\\n" >> "$RUNNER_OBSERVATIONS/cleanup-attempts"\nexit 23\n')
    ps.chmod(0o755)
    process = h.start("cleanup", PATH=f"{commands}:{os.environ['PATH']}")
    try:
        output = h.finish(process, expected=1)
        assert not (h.observations / "next-ran").exists(), "continued after cleanup failure"
        assert h.fixture("cleanup").is_dir(), "cleanup failure fixture was removed"
        assert reported_path(output, "Test executable").is_file(), "active executable was removed before cleanup completed"
        assert str(h.fixture("cleanup")) in output, output
        assert "Failed to stop test processes" in output, output
        assert (h.observations / "cleanup-attempts").read_text().splitlines() == ["attempt", "attempt"], "exit did not retry the active group"
        assert "Tests: 0 passed, 1 failed, 1 total" in output, output
    finally:
        child = h.observations / "child"
        if child.exists():
            try:
                os.killpg(os.getpgid(int(child.read_text())), signal.SIGKILL)
            except ProcessLookupError:
                pass
        if process.poll() is None:
            os.killpg(process.pid, signal.SIGKILL)
        process.communicate(timeout=3)


def timing(h):
    h.suite("test-timing", 'sleep 1.1\n"$REQVIRE_BIN" --version\n')
    output = h.finish(h.start("timing", "test-timing"))
    wall = re.search(r"Suite wall time: ([0-9.]+)s", output)
    process_time = re.search(r"Accumulated Reqvire time: ([0-9.]+)s", output)
    assert wall and process_time, f"wall and process timing not distinguished:\n{output}"
    assert float(wall[1]) >= 1 and float(wall[1]) > float(process_time[1]), output
    logs = reported_path(output, "Test logs directory")
    calls = [line.split("\t") for line in (logs / "invocations.tsv").read_text().splitlines()]
    rows = [line.split("\t") for line in (logs / "tests.tsv").read_text().splitlines()]
    assert len(calls) == len(rows) == 1
    assert int(rows[0][0]) >= 1000, rows
    assert rows[0][1:4] == [calls[0][0], "1", "0"], (calls, rows)
    assert rows[0][4] == "test-timing"
    assert round(float(process_time[1]) * 1000) == int(calls[0][0])
    assert re.search(r"wall [0-9.]+s, reqvire [0-9.]+s, 1 calls", output), output


def mixed(h):
    h.suite("test-a-pass", 'printf "%s\\n" "$TEST_DIR" > "$RUNNER_OBSERVATIONS/passed"\n')
    h.suite("test-b-fail", 'printf "%s\\n" "$TEST_DIR" > "$RUNNER_OBSERVATIONS/failed"\nexit 9\n')
    h.suite("test-c-pass", 'printf "%s\\n" "$TEST_DIR" > "$RUNNER_OBSERVATIONS/last"\n')
    output = h.finish(h.start("all"), expected=1)
    assert "Tests: 2 passed, 1 failed, 3 total" in output, output
    for name in ("passed", "last"):
        assert not Path((h.observations / name).read_text().strip()).exists()
    failed = Path((h.observations / "failed").read_text().strip())
    assert failed.is_dir() and str(failed) in output
    run_root = reported_path(output, "Temporary directory")
    assert list(run_root.iterdir()) == [failed], "completed fixtures or timing scratch leaked"


for name, function in (
    ("successful-fixture-cleanup", success),
    ("failed-fixture-retention", failure),
    ("concurrent-run-isolation", concurrency),
    ("executable-snapshot-survives-source-replacement", executable_snapshot),
    ("missing-executable-fails-startup", lambda h: invalid_executable(h, True)),
    ("non-executable-fails-startup", lambda h: invalid_executable(h, False)),
    ("interrupted-run-cleanup", interruption),
    ("detached-child-interruption", lambda h: interruption(h, detached=True)),
    ("unresponsive-child-escalation", lambda h: interruption(h, detached=True, stubborn=True)),
    ("fixture-setup-failure", setup_failure),
    ("cleanup-failure-stops-run", cleanup_failure),
    ("distinct-wall-and-process-timing", timing),
    ("mixed-suite-results", mixed),
):
    check(name, function)

Path(sys.argv[1]).write_text("\n".join(CHECKS) + "\n")
