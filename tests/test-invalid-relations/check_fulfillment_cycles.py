"""Black-box validation checks for requirement fulfillment dependency cycles."""

import difflib
import json
from pathlib import Path
import re
import subprocess
import sys


binary, directory = sys.argv[1:]
root = Path(directory)
fixtures = root / "cycle-fixtures"
output = root / "output/fulfillment-cycles"
output.mkdir(parents=True, exist_ok=True)
expected = json.loads((root / "expected/fulfillment-cycles.json").read_text())
actual = {}
failures = []


def snapshot(workspace):
    return {str(path.relative_to(workspace)): path.read_bytes()
            for path in workspace.rglob("*")
            if path.is_file() and ".git" not in path.relative_to(workspace).parts}


def run(workspace, label, *args):
    result = subprocess.run([binary, *args], cwd=workspace, text=True,
                            capture_output=True, timeout=30)
    (output / f"{label}.stdout").write_text(result.stdout)
    (output / f"{label}.stderr").write_text(result.stderr)
    return result


def prepare(name, fixture, transform=None):
    workspace = root / "output/fulfillment-cycle-workspaces" / name
    workspace.mkdir(parents=True, exist_ok=True)
    model = (fixtures / fixture).read_text()
    (workspace / "Model.md").write_text(transform(model) if transform else model)
    (workspace / "implementation.txt").write_text("Implementation evidence for cycle fixtures.\n")
    subprocess.run(["git", "init", "-q"], cwd=workspace, check=True,
                   capture_output=True, text=True)
    return workspace


def rejected_case(name, fixture, command, transform=None):
    workspace = prepare(name, fixture, transform)
    before = snapshot(workspace)
    if command[0] in ("link", "merge"):
        initial = run(workspace, name + "-before", "validate", "--json")
        if initial.returncode != 0:
            raise AssertionError("Mutation must start from a valid acyclic model: " + initial.stdout + initial.stderr)
    result = run(workspace, name, *command)
    is_json = "--json" in command
    payload = json.loads(result.stdout) if is_json else None
    errors = payload.get("errors", []) if is_json else [result.stdout + result.stderr]
    cycle_errors = [error for error in errors if "Circular dependency error:" in error]
    for error in cycle_errors:
        path = re.findall(r"Model\.md#[a-z0-9-]+", error)
        if len(path) < 2 or path[0] != path[-1] or "-[contract consumer]->" not in error:
            raise AssertionError("Expected a closed fulfillment path with labeled edges: " + error)
        if fixture == "mixed.md" and "-[child requirement]->" not in error:
            raise AssertionError("Mixed cycle must identify its hierarchy contribution: " + error)
    return {
        "rejected": result.returncode > 0,
        "cycle_reported": bool(cycle_errors),
        "participants": sorted(set(re.findall(r"Model\.md#[a-z0-9-]+", "\n".join(cycle_errors)))),
        "coverage_report_emitted": isinstance(payload, dict) and "summary" in payload,
        "sources_unchanged": snapshot(workspace) == before,
    }


def valid_case(name, fixture, transform=None):
    workspace = prepare(name, fixture, transform)
    before = snapshot(workspace)
    validation = run(workspace, name + "-validate", "validate", "--json")
    coverage = run(workspace, name + "-coverage", "coverage", "--json")
    report = json.loads(coverage.stdout)
    summary = report.get("summary", {})
    return {
        "validation_succeeded": validation.returncode == 0,
        "validation_errors": json.loads(validation.stdout).get("errors"),
        "coverage_succeeded": coverage.returncode == 0,
        "all_requirements_covered": summary.get("uncovered_requirements") == 0
            and summary.get("covered_requirements", 0) > 0,
        "sources_unchanged": snapshot(workspace) == before,
    }


def reverse_declarations(model):
    blocks = [block.strip() for block in model.removeprefix("# Elements\n").split("\n---") if block.strip()]
    reordered = []
    for block in reversed(blocks):
        if "#### Relations\n" in block:
            body, relations = block.split("#### Relations\n", 1)
            block = body + "#### Relations\n" + "\n".join(reversed(relations.splitlines()))
        reordered.append(block)
    return "# Elements\n\n" + "\n---\n\n".join(reordered) + "\n---\n"


def remove_reverse_binding(model):
    binding = "#### Contract Bindings\n  * [Serve Contract](#serve-contract)\n\n"
    if model.count(binding) != 1:
        raise AssertionError("Reciprocal fixture must contain exactly one reverse binding")
    return model.replace(binding, "")


def remove_child_edge(model):
    return model.replace("  * derive: [Child Requirement](#child-requirement)\n", "").replace(
        "  * definedBy: [Child Contract](#child-contract)\n",
        "  * specify: [Mixed Root](#mixed-root)\n  * definedBy: [Child Contract](#child-contract)\n")


def merge_candidate(model):
    source = (fixtures / "merge-source.md").read_text().removeprefix("# Elements\n")
    return remove_reverse_binding(model) + source


def check(name, action):
    try:
        result = action()
        actual[name] = result
        if result != expected[name]:
            before = json.dumps(expected[name], indent=2, sort_keys=True).splitlines()
            after = json.dumps(result, indent=2, sort_keys=True).splitlines()
            raise AssertionError("\n".join(difflib.unified_diff(before, after,
                                 fromfile="expected", tofile="actual", lineterm="")))
    except Exception as error:
        failures.append(name)
        print(f"FAIL {name}:\n{error}")


for name, fixture, command in [
    ("same-root-text", "same-root.md", ("validate",)),
    ("same-root-json", "same-root.md", ("validate", "--json")),
    ("same-root-coverage", "same-root.md", ("coverage", "--json")),
    ("three-roots-json", "three-roots.md", ("validate", "--json")),
    ("mixed-json", "mixed.md", ("validate", "--json")),
    ("mixed-coverage", "mixed.md", ("coverage", "--json")),
]:
    check(name, lambda name=name, fixture=fixture, command=command:
          rejected_case(name, fixture, command))
check("same-root-reordered", lambda: rejected_case("same-root-reordered", "same-root.md",
      ("validate", "--json"), reverse_declarations))
check("repaired-binding", lambda: valid_case("repaired-binding", "same-root.md", remove_reverse_binding))
check("shared-dag", lambda: valid_case("shared-dag", "shared-dag.md"))
for dry_run in (False, True):
    suffix = "-dry-run" if dry_run else ""
    extra = ("--dry-run",) if dry_run else ()
    name = "binding-mutation" + suffix
    check(name, lambda name=name, extra=extra: rejected_case(name, "same-root.md",
          ("link", "Embedded Endpoint", "bindContract", "Model.md#serve-contract", *extra), remove_reverse_binding))
    name = "child-mutation" + suffix
    check(name, lambda name=name, extra=extra: rejected_case(name, "mixed.md",
          ("link", "Parent Requirement", "derive", "Child Requirement", *extra), remove_child_edge))
    name = "merge-mutation" + suffix
    check(name, lambda name=name, extra=extra: rejected_case(name, "same-root.md",
          ("merge", "Embedded Endpoint", "Merge Source", *extra), merge_candidate))

(output / "actual.json").write_text(json.dumps(actual, indent=2, sort_keys=True) + "\n")
summary = {"checks": len(expected), "passed": len(expected) - len(failures), "failed": failures}
(output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
if failures:
    print(f"Fulfillment cycles: {summary['passed']} passed, {len(failures)} failed; artifacts: {output}")
    raise SystemExit(1)
