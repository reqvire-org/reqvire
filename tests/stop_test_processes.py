"""Stop a runner-owned process group and its still-attached descendant sessions."""
import os
import signal
import subprocess
import sys
import time


def process_table():
    result = subprocess.run(
        ["ps", "-axo", "pid=,ppid=,pgid=,stat=,lstart="],
        capture_output=True, text=True, check=True, timeout=2,
        env={**os.environ, "LC_ALL": "C"},
    )
    rows = {}
    for line in result.stdout.splitlines():
        pid, parent, group, state, started = line.split(maxsplit=4)
        rows[int(pid)] = (int(parent), int(group), state, started)
    return rows


def identity(row):
    # Parent IDs can change during shutdown. Group and start time let us avoid
    # signalling an unrelated process if a captured PID has already been reused.
    return row[1], row[3]


def send(pid, sig):
    try:
        os.kill(pid, sig)
    except ProcessLookupError:
        pass


def stop(group):
    if group <= 1 or group == os.getpgrp():
        raise ValueError("Refusing to stop a group not isolated from the runner")
    try:
        # Freeze ordinary owners before discovering their detached servers. If
        # owners die first, those servers reparent and their ownership is lost.
        os.killpg(group, signal.SIGSTOP)
    except ProcessLookupError:
        return
    owned = {}
    try:
        deadline = time.monotonic() + 3
        while True:
            rows = process_table()
            descendants = {pid for pid, row in rows.items() if row[1] == group}
            descendants.update(pid for pid, old in owned.items()
                               if pid in rows and identity(rows[pid]) == old)
            while True:
                children = {pid for pid, row in rows.items() if row[0] in descendants}
                if children <= descendants:
                    break
                descendants.update(children)
            added = descendants - owned.keys()
            for pid in added:
                owned[pid] = identity(rows[pid])
                send(pid, signal.SIGSTOP)
            if not added:
                break
            if time.monotonic() >= deadline:
                raise RuntimeError("Could not stabilize the test descendant tree")

        def surviving():
            return {pid for pid, row in process_table().items()
                    if owned.get(pid) == identity(row) and not row[2].startswith("Z")}

        remaining = surviving()
        for pid in remaining:
            send(pid, signal.SIGTERM)
            send(pid, signal.SIGCONT)
        deadline = time.monotonic() + 5
        while remaining and time.monotonic() < deadline:
            time.sleep(0.05)
            remaining = surviving()
        for pid in remaining:
            send(pid, signal.SIGKILL)
        deadline = time.monotonic() + 1
        while remaining and time.monotonic() < deadline:
            time.sleep(0.05)
            remaining = surviving()
        if remaining:
            raise RuntimeError(f"Test processes survived cleanup: {sorted(remaining)}")
    finally:
        # Never leave a test frozen if discovery itself fails. Only this known
        # private group is group-signalled; detached descendants use exact PIDs.
        try:
            os.killpg(group, signal.SIGCONT)
        except ProcessLookupError:
            pass
        if owned:
            for pid, row in process_table().items():
                if owned.get(pid) == identity(row):
                    send(pid, signal.SIGCONT)


if __name__ == "__main__":
    try:
        stop(int(sys.argv[1]))
    except Exception as error:
        print(f"Failed to stop test processes: {error}", file=sys.stderr)
        sys.exit(1)
