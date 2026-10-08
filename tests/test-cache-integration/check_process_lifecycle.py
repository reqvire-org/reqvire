"""Check test-server shutdown without needing a network listener."""
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time

wrapper = Path(__file__).resolve().parents[1] / "reqvire_timing_wrapper.sh"

with tempfile.TemporaryDirectory(prefix="reqvire-timing-lifecycle-") as directory:
    root = Path(directory)
    child = root / "child"
    child.write_text(f"#!{sys.executable}\n" + '''
import os
from pathlib import Path
import signal
import sys
import time
if sys.argv[1] == "normal":
    print(sys.stdin.read(), end="")
    sys.exit(7)
root = Path(os.environ["LIFECYCLE_ROOT"])
def stop(*_):
    (root / "stopped").write_text("stopped")
    sys.exit(0)
signal.signal(signal.SIGTERM, stop)
signal.signal(signal.SIGINT, stop)
(root / "ready").write_text(str(os.getpid()))
while True:
    time.sleep(0.01)
''')
    child.chmod(0o755)
    timings = root / "timings.tsv"
    environment = {
        **os.environ,
        "REAL_REQVIRE_BIN": str(child),
        "REQVIRE_BENCHMARK_INVOCATIONS": str(timings),
        "REQVIRE_BENCHMARK_TEST": "process-lifecycle",
        "LIFECYCLE_ROOT": str(root),
    }
    normal = subprocess.run([str(wrapper), "normal"], env=environment, input="stdin preserved\n",
                            capture_output=True, text=True, timeout=5)
    assert normal.returncode == 7 and normal.stdout == "stdin preserved\n", normal
    print("PASS exit-and-stdin")

    for name, signum in (("terminate", signal.SIGTERM), ("interrupt", signal.SIGINT)):
        (root / "ready").unlink(missing_ok=True)
        (root / "stopped").unlink(missing_ok=True)
        process = subprocess.Popen([str(wrapper), "server"], env=environment,
                                   stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, start_new_session=True)
        try:
            deadline = time.monotonic() + 5
            while not (root / "ready").exists():
                assert process.poll() is None, "wrapper exited before child startup"
                assert time.monotonic() < deadline, "child startup timed out"
                time.sleep(0.01)
            pid = int((root / "ready").read_text())
            process.send_signal(signum)
            process.wait(timeout=5)
            try:
                os.kill(pid, 0)
            except ProcessLookupError:
                pass
            else:
                raise AssertionError(f"{name}: wrapper exited while child is still running")
            assert (root / "stopped").read_text() == "stopped"
            assert process.returncode == 0, "wrapper did not preserve child exit status"
            print(f"PASS {name}-stops-child")
        finally:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.communicate(timeout=5)

    records = [line.split("\t") for line in timings.read_text().splitlines()]
    assert [record[2] for record in records] == ["7", "0", "0"], records
    assert all(int(record[0]) >= 0 and record[1] == "process-lifecycle" for record in records)
