"""Check served/exported seed data independently of JSON whitespace."""
import argparse
import json
from pathlib import Path
import sys


parser = argparse.ArgumentParser()
parser.add_argument("seed")
parser.add_argument("--requirement-text", default="Test Requirement One")
parser.add_argument("--evidence-text", default="serve command evidence")
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
except (AssertionError, KeyError, TypeError, ValueError) as error:
    sys.exit(f"FAILED: {error}")
