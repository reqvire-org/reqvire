"""Black-box regressions using explicitly authored requirement assessments."""
import difflib
import json
from pathlib import Path
import subprocess
import sys


binary, directory, expected_directory = sys.argv[1:]
workspace, expected_path = Path(directory), Path(expected_directory)
expected = json.loads((expected_path / "requirements.json").read_text())
scopes = json.loads((expected_path / "scopes.json").read_text())
failures = []


def run(*args):
    result = subprocess.run([binary, *args], cwd=workspace, text=True, capture_output=True, timeout=30)
    if result.returncode:
        raise AssertionError(result.stderr + result.stdout)
    return result.stdout


def equal(actual, wanted, label):
    a, b = (json.dumps(value, indent=2, sort_keys=True).splitlines() for value in (actual, wanted))
    if a != b:
        raise AssertionError(label + "\n" + "\n".join(difflib.unified_diff(b, a, fromfile="expected", tofile="actual")))


def check(label, action):
    try:
        action()
        print("PASS " + label)
    except Exception as error:
        failures.append(label)
        print("FAIL " + label + ": " + str(error))


def entries(report, section):
    return [entry for values in report[section]["files"].values() for entry in values]


def assessments(report, wanted):
    actual = {}
    for section, covered in (("covered_requirements", True), ("uncovered_requirements", False)):
        for record in entries(report, section):
            assert record["name"] not in actual, "Duplicate requirement record"
            assert (wanted[record["name"]]["coverage_source"] != "uncovered") == covered, record["name"]
            actual[record["name"]] = record
    equal(actual, wanted, "Requirement assessments")


def summary(report, wanted):
    records = list(wanted.values())
    terminal = [record for record in records if record["is_terminal"]]
    covered = [record for record in records if record["coverage_source"] != "uncovered"]
    implemented = [record for record in terminal if record["coverage_source"] != "uncovered"]
    fields = {
        "total_requirements_in_scope": len(records), "covered_requirements": len(covered),
        "uncovered_requirements": len(records) - len(covered),
        "total_terminal_requirements": len(terminal), "covered_terminal_requirements": len(implemented),
        "uncovered_terminal_requirements": len(terminal) - len(implemented),
        "implementation_coverage_percentage": round(len(implemented) * 100 / len(terminal), 2) if terminal else 0.0,
        "coverage_sources": {name: sum(record["coverage_source"] == name for record in covered)
                             for name in ("direct_satisfied", "requirement_rollup", "contract_consumer_rollup", "combined_rollup")},
    }
    equal({key: report["summary"].get(key) for key in fields}, fields, "Implementation summary")


def scoped(name):
    report = json.loads(run("coverage", "--from", name, "--json"))
    ids = scopes[name]
    wanted = {key: value for key, value in expected.items() if value["identifier"] in ids}
    equal(report["scope"]["requirement_ids"], ids, "Scope membership")
    assessments(report, wanted)
    summary(report, wanted)
    capability = next(row for row in report["capability_coverage"]["capabilities"] if row["name"] == name)
    equal(capability["aggregate_terminal_requirements"], report["summary"]["total_terminal_requirements"], "Capability terminal denominator")
    equal(capability["aggregate_covered_terminal_requirements"], report["summary"]["covered_terminal_requirements"], "Capability terminal numerator")
    equal(capability["implementation_coverage_percentage"], report["summary"]["implementation_coverage_percentage"], "Capability percentage")
    equal(capability["implementation_covered"], bool(wanted) and all(row["coverage_source"] != "uncovered" for row in wanted.values()), "Capability completeness")


whole = json.loads(run("coverage", "--json"))
(workspace / "output/whole.json").write_text(json.dumps(whole, indent=2) + "\n")
check("whole model assessments", lambda: assessments(whole, expected))
check("terminal implementation percentage", lambda: summary(whole, expected))


def verification_leaf_names():
    equal(sorted(row["name"] for row in entries(whole, "verified_leaf_requirements")),
          ["Complete Owner", "Unused Owner"], "Contract-owning verification leaves")


check("verification leaves differ from implementation terminals", verification_leaf_names)
for scope in scopes:
    check("scope " + scope, lambda scope=scope: scoped(scope))


def text_evidence():
    text = run("coverage")
    (workspace / "output/whole.txt").write_text(text)
    assert "Terminal Requirements" in text and "Covered Terminal Requirements" in text
    gap_section = text.split("## Uncovered Requirements", 1)[1].split("## Capability Coverage", 1)[0]
    assert "evidence/parent.txt" in gap_section, "Uncovered parent lost its direct evidence"
    assert "Blocking requirements" in gap_section and "Contributing requirements" in gap_section


check("text retains partial evidence and blockers", text_evidence)
model = workspace / "specifications/Model.md"
original = model.read_text()


def order_independence():
    header, body = original.split("\n", 1)
    blocks = [block.strip() for block in body.split("\n---") if block.strip()]
    reordered = []
    for block in reversed(blocks):
        if "#### Relations\n" in block:
            before, relations = block.split("#### Relations\n", 1)
            block = before + "#### Relations\n" + "\n".join(reversed(relations.splitlines()))
        reordered.append(block)
    try:
        model.write_text(header + "\n\n" + "\n---\n\n".join(reordered) + "\n---\n")
        assessments(json.loads(run("coverage", "--json")), expected)
    finally:
        model.write_text(original)


check("declaration and relation order independence", order_independence)


def mutation():
    before, target = original.split("### Complete Consumer B\n", 1)
    target = target.replace("  * satisfiedBy: [b](../evidence/b.txt)\n", "", 1)
    try:
        model.write_text(before + "### Complete Consumer B\n" + target)
        report = json.loads(run("coverage", "--from", "Delegated Service", "--json"))
        owner = entries(report, "uncovered_requirements")[0]
        equal(owner["name"], "Complete Owner", "External evidence loss invalidates owner")
        equal(owner["evidence"], ["evidence/a.txt"], "Remaining external evidence")
        equal(owner["blocking_requirements"], ["specifications/Model.md#complete-consumer", "specifications/Model.md#complete-consumer-b"], "Recursive external blockers")
        equal(owner["aggregate_terminal_requirements"], 2, "Owner terminal denominator retains external contributors")
        equal(owner["aggregate_covered_terminal_requirements"], 1, "Owner partial implementation progress")
        equal(report["summary"]["total_terminal_requirements"], 0, "External terminals remain outside scope")
    finally:
        model.write_text(original)


check("external evidence removal invalidates recursive owner", mutation)


def exported_store():
    run("export", "--output", "output/site")
    javascript = (workspace / "output/site/assets/project-store.js").read_text()
    store = json.JSONDecoder().raw_decode(javascript.split("=", 1)[1].lstrip())[0]
    assessments(store["coverage"], expected)
    summary(store["coverage"], expected)
    for name, ids in scopes.items():
        scope = store["coverage"]["scope_index"]["specifications/Model.md#" + name.lower().replace(" ", "-")]
        equal(scope["scope"]["requirement_ids"], ids, "Exported scope membership")
        summary(scope, {key: value for key, value in expected.items() if value["identifier"] in ids})


check("Explorer export reuses shared assessments and scope summaries", exported_store)
if failures:
    raise SystemExit(f"{len(failures)} checks failed: {', '.join(failures)}")
