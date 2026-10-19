import difflib
import json
import importlib.util
from pathlib import Path
import subprocess
import sys

binary, workspace, source = sys.argv[1], Path(sys.argv[2]), Path(sys.argv[3])
checks = []


def check(name, condition, detail=""):
    if not condition:
        raise AssertionError(f"{name}: {detail}")
    checks.append(f"{name}: PASS")


def golden(name, actual):
    expected = (source / "expected" / name).read_text()
    if expected != actual:
        raise AssertionError("".join(difflib.unified_diff(
            expected.splitlines(True), actual.splitlines(True),
            fromfile=f"expected/{name}", tofile="actual")))


spec = importlib.util.spec_from_file_location("revision_e2e", source.parent / "test-model-revision-hashing/check_revision.py")
helpers = importlib.util.module_from_spec(spec)
spec.loader.exec_module(helpers)


class Server:
    def __init__(self, root, mutations=False):
        self.inner = helpers.McpServer(binary, root, workspace / "output",
                                      "enabled" if mutations else "read-only",
                                      *(["--enable-mutations"] if mutations else []))
        self.inner.__enter__()

    def rpc(self, method, params):
        try:
            return {"result": self.inner.rpc(method, params)}
        except RuntimeError as error:
            return {"error": str(error)}

    def raw(self, name, **args):
        return self.rpc("tools/call", {"name": name, "arguments": args})

    def tool(self, name, **args):
        return self.inner.tool(name, **args)

    def close(self):
        self.inner.__exit__(None, None, None)


removed = ["add", "rm", "mv", "rename", "merge", "mv-file", "mv-folder", "link",
           "unlink", "relink", "mv-asset", "rm-asset", "format"]
for args in [[name, "--dry-run"] for name in removed] + [["lint", "--fix"]]:
    result = subprocess.run([binary, "--workspace", str(workspace / "missing"), *args],
                            text=True, capture_output=True)
    assert result.returncode == 2 and "MCP" in result.stderr, result.stderr
    replacement = {"add": "add_element", "rm": "remove_element", "mv": "move_element", "rename": "rename_element",
                   "merge": "merge_elements", "mv-file": "move_file", "mv-folder": "move_folder", "mv-asset": "move_asset", "rm-asset": "remove_asset"}.get(args[0], args[0])
    assert "reqvire." + replacement in result.stderr, result.stderr
    assert "Failed to resolve workspace" not in result.stderr
check("cli/removed-before-model-load", True)
help_text = subprocess.check_output([binary, "--help"], text=True)
commands = help_text.split("Commands:", 1)[1].split("Options:", 1)[0]
assert not any(line.split()[0] in removed for line in commands.splitlines() if line.strip())
for retained in ["validate", "lint", "coverage", "search", "collect", "submodels", "model",
                 "traces", "containment", "resources", "change-impact", "export", "semantic",
                 "concepts", "ontologies", "migrate", "mcp", "serve"]:
    assert any(line.split()[0] == retained for line in commands.splitlines() if line.strip()), retained
assert "--fix" in subprocess.check_output([binary, "migrate", "--help"], text=True)
check("cli/retained-catalog-and-migration", True)

root = workspace / "boundary"
root.mkdir()
before = (source / "fixtures/Repair.md.fixture").read_bytes()
(root / "Repair.md").write_bytes(before)
untouched = b"# Elements\n\n### Untouched Capability\n\nAn untouched document.\n\n#### Metadata\n  * type: capability\n---\n"
(root / "Untouched.md").write_bytes(untouched)
subprocess.run(["git", "init", "-q", str(root)], check=True)
subprocess.run(["git", "-C", str(root), "config", "user.name", "Test"], check=True)
subprocess.run(["git", "-C", str(root), "config", "user.email", "test@example.org"], check=True)
subprocess.run(["git", "-C", str(root), "add", "Repair.md", "Untouched.md"], check=True)
subprocess.run(["git", "-C", str(root), "-c", "commit.gpgSign=false", "commit", "-qm", "fixture"], check=True)
server = Server(root)
try:
    tools = {t["name"]: t for t in server.rpc("tools/list", {})["result"]["tools"]}
    revision = server.tool("reqvire.model_revision")
    for name in ["reqvire.format", "reqvire.lint"]:
        assert tools[name]["inputSchema"]["properties"]["fix"]["enum"] == [False]
        assert tools[name]["annotations"]["readOnlyHint"] is True
        assert "error" in server.raw(name, fix=True)
    assert not any("migrate" in name or name == "reqvire.validate" for name in tools)
    assert (root / "Repair.md").read_bytes() == before
    assert server.tool("reqvire.model_revision") == revision
    check("mcp/read-only-discovery-and-rejection", True)
finally:
    server.close()

server = Server(root, True)
try:
    tools = {t["name"]: t for t in server.rpc("tools/list", {})["result"]["tools"]}
    assert tools["reqvire.lint"]["annotations"]["readOnlyHint"] is False
    inspected = server.tool("reqvire.lint")
    assert len(inspected["auto_fixable"]) == 1 and inspected["needs_manual_review"]
    result = server.tool("reqvire.lint", fix=True)
    (workspace / "repair-result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    assert result["applied_fixes"]["relations_removed"] == 1
    assert result["applied_fixes"]["changed_files"] == ["Repair.md"]
    assert not result["auto_fixable"] and result["needs_manual_review"] == inspected["needs_manual_review"]
    golden("after-repair.md.txt", (root / "Repair.md").read_text())
    golden("repair-diffs.json", json.dumps(result["applied_fixes"]["diffs"], indent=2, sort_keys=True) + "\n")
    assert (root / "Untouched.md").read_bytes() == untouched
    check("mcp/safe-repair-and-preserved-review", True)
    revision = server.tool("reqvire.model_revision")
    result = server.tool("reqvire.lint", fix=True)
    assert result["applied_fixes"]["relations_removed"] == 0
    assert result["applied_fixes"]["changed_files"] == []
    assert server.tool("reqvire.model_revision") == revision
    assert not any(d["file_path"] == "Repair.md" for d in server.tool("reqvire.format")["diffs"])
    check("mcp/repair-no-op-and-format-preview", True)
    added = "### Added Requirement\n\n#### Metadata\n  * type: requirement\n\n#### Relations\n  * derivedFrom: [Repair Root](Repair.md#repair-root)\n"
    server.tool("reqvire.add_element", file="Added.md", content=added)
    assert server.tool("reqvire.format")["diffs"] == []
    previous = (root / "Added.md").read_bytes()
    revision = server.tool("reqvire.model_revision")
    rejected = server.raw("reqvire.add_element", file="Added.md", override_existing=True,
                          content=added.replace("#### Metadata", "See [Root](Repair.md#repair-root).\n\n#### Metadata"))
    assert rejected.get("result", {}).get("isError") is True, rejected
    assert (root / "Added.md").read_bytes() == previous and server.tool("reqvire.model_revision") == revision
    assert (root / "Untouched.md").read_bytes() == untouched
    check("mcp/canonical-authoring-and-atomic-rejection", True)
    # Explicit inverse materialization must survive the session's canonical write.
    before_format = (root / "Repair.md").read_bytes()
    preview = server.tool("reqvire.format", with_full_relations=True)
    assert preview["diffs"] and (root / "Repair.md").read_bytes() == before_format
    applied = server.tool("reqvire.format", fix=True, with_full_relations=True)
    assert applied["diffs"] == preview["diffs"]
    golden("full-relations.md.txt", (root / "Repair.md").read_text())
    assert "* derive: [Added Requirement](Added.md#added-requirement)" in (root / "Repair.md").read_text()
    assert not server.tool("reqvire.format", fix=True, with_full_relations=True)["diffs"]
    assert (root / "Untouched.md").read_bytes() == untouched
finally:
    server.close()
(workspace / "checks.txt").write_text("\n".join(checks) + "\n")
