"""Regenerate the showcase store with the real Reqvire exporter and E2E model."""
import json
from pathlib import Path
import shutil
import subprocess
import tempfile


root = Path(__file__).resolve().parents[2]
fixture = root / "tests/test-scoped-coverage"
destination = root / "explorer/src/store/fixtures/scopedCoverage.json"

with tempfile.TemporaryDirectory(prefix="reqvire-coverage-showcase-") as directory:
    workspace = Path(directory)
    for name in ("specifications", "evidence"):
        shutil.copytree(fixture / name, workspace / name)
    shutil.copy(fixture / ".reqvireignore", workspace)
    # Several artifacts exercise wrapping and repeated basenames in the evidence panel.
    artifacts = ["README.md", "skills/SKILL.md", "skills/AnalyzeCoverage.md",
                 "skills/AnalyzeModel.md", "skills/ChangeImpact.md", "skills/Lint.md",
                 "implementation/SKILL.md"]
    requirement = workspace / "specifications/Alpha.md"
    content = requirement.read_text()
    links = []
    for artifact in artifacts:
        path = workspace / "evidence" / artifact
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(f"Synthetic showcase evidence: {artifact}\n")
        links.append(f"  * satisfiedBy: [{path.name}](../evidence/{artifact})")
    content = content.replace("  * satisfiedBy: [Alpha implementation](../evidence/alpha.txt)",
                              "  * satisfiedBy: [Alpha implementation](../evidence/alpha.txt)\n" + "\n".join(links))
    (workspace / "evidence/parent.txt").write_text("Direct parent evidence does not cover Alpha Gap.\n")
    content = content.replace("  * definedBy: [Shared Service Format](#shared-service-format)",
                              "  * satisfiedBy: [Parent implementation](../evidence/parent.txt)\n"
                              "  * definedBy: [Shared Service Format](#shared-service-format)")
    requirement.write_text(content)
    with (workspace / ".reqvireignore").open("a") as ignore:
        ignore.write("evidence/\n")
    subprocess.run(["git", "init", "-q", "--initial-branch=showcase", directory], check=True)
    subprocess.run([str(root / "target/debug/reqvire"), "--workspace", directory,
                    "export", "--output", str(workspace / "output/site")], check=True,
                   stdout=subprocess.DEVNULL)
    javascript = (workspace / "output/site/assets/project-store.js").read_text()
    store = json.JSONDecoder().raw_decode(javascript.split("=", 1)[1].lstrip())[0]
    store["project"] = {
        "name": "Scoped coverage example", "root_label": "coverage-example",
        "workspace_root": "/examples/coverage", "repository": None, "branch": "showcase",
        "eligible_git_worktrees": [{"root": "/examples/coverage", "workspace_relative_root": "",
                                    "head": None, "dirty": True}],
    }
    destination.write_text(json.dumps(store, indent=2, ensure_ascii=False) + "\n")
print(destination.relative_to(root))
