"""Deletion must preserve unrelated evidence file targets in rewritten source."""
import json
import os
from pathlib import Path
import re
import subprocess

SUITE = Path(__file__).parent
OUTPUT = Path(os.environ["TEST_DIR"]) / "output"
ROOT = OUTPUT / "rm-artifact-links"
MODEL = ROOT / "system-model/Interfaces/WebExplorer/Capabilities.md"
CHECKS = ROOT / "system-model/Verifications/Checks.md"
ARTIFACTS = {"Makefile": b"all:\n\t@echo retained artifact\n",
             "LICENSE": b"Retained fixture license.\n",
             "src/evidence.rs": b"// Retained implementation evidence.\n"}
TARGETS = {"../../../" + name for name in ARTIFACTS}
FILES = ARTIFACTS | {"system-model/Interfaces/WebExplorer/src/evidence.rs": b"// Local decoy, not the evidence target.\n"}
results = []


def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, check=True, capture_output=True).stdout


def run(label, *args):
    result = subprocess.run([os.environ["REQVIRE_BIN"], *args, "--json"], cwd=ROOT,
                            text=True, capture_output=True, timeout=40)
    (OUTPUT / f"rm-artifact-links-{label}.json").write_text(result.stdout)
    (OUTPUT / f"rm-artifact-links-{label}.stderr").write_text(result.stderr)
    return result


def require(value, message):
    if not value:
        raise AssertionError(message)


def check(name, callback):
    try:
        callback()
        results.append("PASS " + name)
    except Exception as error:
        results.append("FAIL " + name)
        print(f"{results[-1]}: {error}", flush=True)


def artifact_targets(text):
    return set(re.findall(r"satisfiedBy: \[[^\]]+\]\(([^)]+)\)", text))


MODEL.parent.mkdir(parents=True)
CHECKS.parent.mkdir(parents=True)
fixture = SUITE / "fixtures/rm-artifact-links"
MODEL.write_bytes((fixture / "Capabilities.md.txt").read_bytes())
CHECKS.write_bytes((fixture / "Checks.md.txt").read_bytes())
for name, content in FILES.items():
    path = ROOT / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(content)
git("init", "-q")
git("config", "user.name", "Artifact link regression")
git("config", "user.email", "tests@example.com")
git("remote", "add", "origin", "https://example.com/model.git")
git("add", ".")
git("-c", "commit.gpgSign=false", "commit", "-qm", "Fixture")
head = git("rev-parse", "HEAD")
index = (ROOT / ".git/index").read_bytes()
before = {path: path.read_bytes() for path in [MODEL, CHECKS, *(ROOT / name for name in FILES)]}
initial = run("initial-validation", "validate")
check("rm-artifact-links-initial-valid", lambda: require(initial.returncode == 0, initial.stdout + initial.stderr))

preview = run("preview", "rm", "Disposable Verification", "--dry-run")
check("rm-artifact-links-preview-keeps-files", lambda: require(
    preview.returncode == 0 and all(path.read_bytes() == content for path, content in before.items()),
    "dry-run failed or modified source files: " + preview.stdout + preview.stderr))


def preview_targets():
    require(preview.returncode == 0, preview.stdout + preview.stderr)
    report = json.loads(preview.stdout)
    relevant = next(item for item in report["diffs"] if item["file_path"] == str(MODEL.relative_to(ROOT)))
    lines = relevant["lines"]
    removed = artifact_targets("\n".join(line["content"] for line in lines if line["color"] == "red"))
    added = artifact_targets("\n".join(line["content"] for line in lines if line["color"] == "green"))
    actual = artifact_targets("\n".join(line["content"] for line in lines if line["color"] != "red"))
    require(removed == added and actual.issubset(TARGETS),
            f"preview rewrites artifact targets: removed {sorted(removed)}, added {sorted(added)}")


check("rm-artifact-links-preview-keeps-targets", preview_targets)
applied = run("apply", "rm", "Disposable Verification")
check("rm-artifact-links-removes-verification", lambda: require(
    applied.returncode == 0 and "Disposable Verification" not in MODEL.read_text()
    and "Disposable Verification" not in CHECKS.read_text(), applied.stdout + applied.stderr))
check("rm-artifact-links-apply-keeps-targets", lambda: require(
    artifact_targets(MODEL.read_text()) == TARGETS,
    f"persisted artifact targets changed: {sorted(artifact_targets(MODEL.read_text()))}"))
check("rm-artifact-links-keeps-artifacts-head-index", lambda: require(
    all((ROOT / name).read_bytes() == content for name, content in FILES.items())
    and git("rev-parse", "HEAD") == head and (ROOT / ".git/index").read_bytes() == index,
    "deletion changed evidence bytes, HEAD, or index"))
validation = run("persisted-validation", "validate")
check("rm-artifact-links-persisted-model-valid", lambda: require(
    validation.returncode == 0, validation.stdout + validation.stderr))
(OUTPUT / "rm-artifact-links-checks.txt").write_text("\n".join(results) + "\n")
print("\n".join(results), flush=True)
raise SystemExit(0 if all(line.startswith("PASS ") for line in results) else 1)
