"""Run the production npm launcher against executable fixture archives."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import time

root = Path(__file__).resolve().parents[2]
output = Path(sys.argv[1]).resolve()
output.mkdir(parents=True, exist_ok=True)
node = shutil.which("node")
assert node, "node is required"
tar = shutil.which("tar")
assert tar, "tar is required"
checks = []

def fixture(name, version="1.2.3"):
    base = output / name
    (base / "package/bin").mkdir(parents=True)
    (base / "package/artifacts").mkdir()
    (base / "tmp").mkdir()
    (base / "bin").mkdir()
    (base / "barrier").mkdir()
    shutil.copy(root / "npm/reqvire/bin/reqvire.js", base / "package/bin/reqvire.js")
    (base / "package/package.json").write_text(json.dumps({"version": version}))
    executable = base / "native"
    executable.write_text('#!/bin/sh\ncase "$1" in\nfail) exit 23;;\nsignal) kill -TERM $$;;\n*) printf "%s\\n" "$@";;\nesac\n')
    executable.chmod(0o755)
    platform = subprocess.check_output([node, "-p", 'process.platform + "/" + process.arch'], text=True).strip()
    targets = {"linux/x64": "linux-x86_64", "darwin/arm64": "darwin-arm64", "darwin/x64": "darwin-x86_64"}
    target = targets[platform]
    with tarfile.open(base / f"package/artifacts/reqvire-{target}.tar.gz", "w:gz") as archive:
        archive.add(executable, arcname=f"reqvire-{target}")
    wrapper = base / "bin/tar"
    shutil.copy2(Path(__file__).parent / 'fixtures/tar-double.sh', wrapper)
    wrapper.chmod(0o755)
    env = {**os.environ, "TMPDIR": str(base / "tmp"), "PATH": str(base / "bin") + os.pathsep + os.environ["PATH"], "BARRIER": str(base / "barrier"),
           "REQVIRE_TEST_REAL_TAR": tar, "REQVIRE_TEST_TAR_TARGET": target}
    cache = base / "tmp/reqvire-npm" / (version + "-" + platform.replace("/", "-"))
    return base, env, cache

def launch(base, env, *args):
    return subprocess.run([node, str(base / "package/bin/reqvire.js"), *args], env=env, text=True, capture_output=True, timeout=20)

def require(condition, message):
    assert condition, message

def no_staging(base):
    require(not list((base / "tmp/reqvire-npm").glob(".extract-*")), "private extraction directory leaked")

def concurrent(failing=False):
    base, env, cache = fixture("concurrent-failure" if failing else "concurrent")
    env["BARRIER_COUNT"] = "16"
    if failing:
        env["FAIL_ONE"] = "1"
    children = [subprocess.Popen([node, str(base / "package/bin/reqvire.js"), f"argument {i}"], env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE) for i in range(16)]
    results = []
    try:
        for i, child in enumerate(children):
            stdout, stderr = child.communicate(timeout=20)
            results.append(child.returncode)
            if child.returncode == 0:
                require(stdout == f"argument {i}\n", f"wrong arguments: {stdout}")
            elif failing:
                require("archive extraction failed (17)" in stderr, f"unexpected launcher failure: {stderr}")
    finally:
        for child in children:
            if child.poll() is None: child.kill()
            child.wait()
    require(sum(code == 0 for code in results) == (15 if failing else 16), f"launcher exits: {results}")
    require((cache / "reqvire").is_file(), "winner missing")
    no_staging(base)
    # An existing cached executable must work even when tar would fail.
    env.pop("BARRIER_COUNT")
    env["FAIL_EXTRACT"] = "1"
    require(launch(base, env, "warm").stdout == "warm\n", "warm cache extracted again")

def failure_retry():
    base, env, cache = fixture("retry")
    result = launch(base, {**env, "FAIL_EXTRACT": "1"}, "first")
    require(result.returncode != 0, "failed extraction succeeded")
    require(not (cache / "reqvire").exists(), "partial executable published")
    no_staging(base)
    require(not list((base / "tmp").rglob("reqvire-*64")), "partial extraction remains")
    require(launch(base, env, "retry").stdout == "retry\n", "retry failed")

def process_contract():
    base, env, _ = fixture("process")
    require(launch(base, env, "two words", "--flag").stdout == "two words\n--flag\n", "arguments changed")
    require(launch(base, env, "fail").returncode == 23, "native status lost")
    require(launch(base, env, "signal").returncode == -15, "native signal lost")

def errors():
    base, env, _ = fixture("errors")
    for archive in (base / "package/artifacts").iterdir(): archive.unlink()
    result = launch(base, env)
    require(result.returncode != 0 and "Missing Reqvire archive" in result.stderr, "missing archive diagnostic absent")
    script = 'Object.defineProperty(process, "platform", {value: "unsupported"}); require(process.argv[1]);'
    result = subprocess.run([node, "-e", script, str(base / "package/bin/reqvire.js")], env=env, text=True, capture_output=True)
    require(result.returncode != 0 and "Unsupported platform" in result.stderr, "unsupported platform diagnostic absent")

def versions():
    base, env, cache = fixture("versions")
    require(launch(base, env, "old").returncode == 0, "first version failed")
    (base / "package/package.json").write_text('{"version":"2.0.0"}')
    require(launch(base, env, "new").returncode == 0, "second version failed")
    require((cache / "reqvire").exists(), "old version removed")
    require(len(list((base / "tmp/reqvire-npm").glob("*/reqvire"))) == 2, "versions not isolated")

for name, check in [("concurrent-first-use-and-warm-cache", concurrent),
                    ("concurrent-failed-extractor", lambda: concurrent(True)),
                    ("extraction-failure-and-retry", failure_retry),
                    ("native-arguments-status-signal", process_contract),
                    ("invalid-platform-and-archive", errors), ("version-isolation", versions)]:
    try:
        check()
        checks.append(f"PASS {name}")
    except Exception as error:
        checks.append(f"FAIL {name}")
        print(f"{name}: {error}", file=sys.stderr)
(output / "checks.txt").write_text("\n".join(checks) + "\n")
sys.exit(any(check.startswith("FAIL") for check in checks))
