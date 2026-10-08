"""Check served/exported seed data independently of JSON whitespace."""
import argparse
import json
from pathlib import Path
import sys


parser = argparse.ArgumentParser()
parser.add_argument("seed")
parser.add_argument("--requirement-text", default="Test Requirement One")
parser.add_argument("--evidence-text", default="serve command evidence")
parser.add_argument("--traces-json", help="CLI trace projection for exact served/exported parity")
args = parser.parse_args()

try:
    seed = Path(args.seed).read_text()
    prefix = "window.reqvireProjectStore = "
    assert seed.startswith(prefix), "Project Store seed assignment is missing"
    store, end = json.JSONDecoder().raw_decode(seed[len(prefix):].lstrip())
    assert seed[len(prefix):].lstrip()[end:].startswith(";"), "Project Store seed terminator is missing"
    files = {row["path"]: row for row in store["files"]}
    assert "specifications/Requirements.md" in files, "Project Store is missing modeled source file records"
    assert args.requirement_text in files["specifications/Requirements.md"]["markdown_content"], "Modeled source content is missing"
    assert "scripts/evidence.sh" not in files, "Resource-only evidence appeared as a modeled file"
    assert "notes/unrelated.md" not in files, "Unrelated repository file appeared as a modeled file"
    evidence = next((row for row in store["resources"] if row["id"] == "resource:scripts/evidence.sh"), None)
    assert evidence is not None, "Graph-referenced evidence resource is missing"
    assert evidence["file_path"] == "scripts/evidence.sh", "Evidence resource path is incorrect"
    assert args.evidence_text in evidence["source_text"], "Evidence source preview is missing"
    assert store["schema_version"] == "2026-10-06.project-store.v5", "Unexpected Project Store schema"
    for file in store["traces"]["files"].values():
        for trace in file["verifications"]:
            graph = trace["trace_graph"]
            nodes = {node["id"]: node for node in graph["nodes"]}
            assert len(nodes) == len(graph["nodes"]), "Repeated trace node"
            assert all("children" not in node for node in nodes.values()), "Trace contains nested subtrees"
            assert all(edge["source"] in nodes and edge["target"] in nodes for edge in graph["edges"]), "Dangling trace edge"
            assert trace["total_requirements_in_tree"] == sum(node["type"] == "requirement" for node in nodes.values())
            assert sorted(node["id"] for node in nodes.values() if node["is_directly_verified"]) == trace["directly_verified_requirements"]
    if args.traces_json:
        assert store["traces"] == json.loads(Path(args.traces_json).read_text()), "Served/exported traces differ from CLI"
except (AssertionError, KeyError, TypeError, ValueError) as error:
    sys.exit(f"FAILED: {error}")
