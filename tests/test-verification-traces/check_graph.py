"""Independent trace topology oracle, exercised through the CLI and static export."""
import json
import subprocess
import sys
from pathlib import Path

binary, destination = sys.argv[1:3]
# A safe pre-fix reproduction can request 3; the suite exercises 20 and 40 after the fix.
depths = [int(value) for value in sys.argv[3:]] or [20, 40]
output = Path(destination)
output.mkdir(parents=True, exist_ok=True)

for depth in depths:
    workspace = output / f"dag-{depth}"
    workspace.mkdir(exist_ok=True)
    subprocess.run(["git", "init", "-q", str(workspace)], check=True)
    (workspace / ".reqvireignore").write_text("export/\n")
    records = []
    expected_nodes = {}
    expected_edges = set()
    direct = {f"layer-{depth-1}-a", "layer-0-a"}

    def identifier(name):
        return f"Model.md#{name}"

    def add(name, kind, relations=()):
        records.append(f"### {name}\n\nFixture {name}.\n\n#### Metadata\n  * type: {kind}\n")
        if relations:
            records.append("\n#### Relations\n" + "".join(
                f"  * {relation}: [{target}](#{target})\n" for relation, target in relations))
        records.append("\n---\n\n")
        if kind in {"requirement", "capability"}:
            expected_nodes[identifier(name)] = {
                "id": identifier(name), "name": name, "type": kind,
                "is_directly_verified": name in direct,
            }
            expected_edges.update((identifier(name), relation, identifier(target)) for relation, target in relations)

    add("cap-root", "capability")
    add("cap-child", "capability", [("derivedFrom", "cap-root")])
    add("req-root", "requirement", [("specify", "cap-child")])
    for level in range(depth):
        parents = ["req-root"] if level == 0 else [f"layer-{level-1}-{side}" for side in "ab"]
        for side in "ab":
            add(f"layer-{level}-{side}", "requirement", [("derivedFrom", parent) for parent in parents])
    # Both top nodes must be reachable; this final join also provides a direct ancestor case.
    add("leaf", "requirement", [("derivedFrom", f"layer-{depth-1}-{side}") for side in "ab"])
    direct.add("leaf")
    expected_nodes[identifier("leaf")]["is_directly_verified"] = True
    add("objective", "verification-objective")
    add("check", "test-verification", [("derivedFrom", "objective")] + [("verify", name) for name in sorted(direct)])
    add("orphan", "test-verification", [("derivedFrom", "objective")])
    (workspace / "Model.md").write_text("# Elements\n\n" + "".join(records))

    def run(*arguments):
        result = subprocess.run([binary, "--workspace", str(workspace), *arguments],
                                check=True, text=True, capture_output=True, timeout=60)
        return result.stdout

    report = json.loads(run("traces"))
    (workspace / "traces.json").write_text(json.dumps(report, indent=2) + "\n")
    traces = [row for file in report["files"].values() for row in file["verifications"]]
    assert len(traces) == 1, "orphan/objective should not produce traces"
    trace = traces[0]
    assert trace["directly_verified_requirements"] == sorted(map(identifier, direct))
    assert trace["directly_verified_count"] == len(direct)
    assert trace["total_requirements_in_tree"] == 2 * depth + 2
    graph = trace["trace_graph"]
    assert graph["nodes"] == [expected_nodes[key] for key in sorted(expected_nodes)]
    assert graph["edges"] == [dict(source=source, relation_type=relation, target=target)
                              for source, relation, target in sorted(expected_edges)]
    assert "trace_tree" not in trace
    assert all("children" not in node for node in graph["nodes"])
    # Data size bound, independent of execution speed: no expanded path encoding elsewhere.
    assert len(json.dumps(trace)) < 400 * (len(expected_nodes) + len(expected_edges))
    assert report == json.loads(run("traces")), "unchanged traces must be deterministic"
    print(f"PASS {depth}-layers/cli-topology-counts-and-determinism")

    run("export", "--output", str(workspace / "export"))
    seed = (workspace / "export/assets/project-store.js").read_text()
    store = json.JSONDecoder().raw_decode(seed.removeprefix("window.reqvireProjectStore = ").lstrip())[0]
    assert store["schema_version"] == "2026-10-06.project-store.v5"
    assert store["traces"] == report, "export and CLI must share exactly the same trace projection"
    print(f"PASS {depth}-layers/export-cli-parity")
