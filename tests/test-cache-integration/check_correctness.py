#!/usr/bin/env python3
"""Issue #73 regressions through real standalone and embedded HTTP MCP servers.

Every expected check is PASS. Failures accumulate, so one stale input does not
hide the remaining regressions. Logs and responses live outside each worktree.
"""

import argparse
import difflib
import importlib.util
import json
import os
from pathlib import Path
import socket
import subprocess
import traceback
import urllib.error
import urllib.request

# Reuse the existing protocol/session/process cleanup helper, not a mock server.
spec = importlib.util.spec_from_file_location(
    "revision_e2e", Path(__file__).resolve().parents[1]
    / "test-model-revision-hashing/check_revision.py")
revision_e2e = importlib.util.module_from_spec(spec)
spec.loader.exec_module(revision_e2e)


class Server(revision_e2e.McpServer):
    def __init__(self, binary, workspace, output, mode, name):
        self.output = output / name
        self.output.mkdir()
        self.log = (self.output / "server.log").open("w")
        self.session = None
        self.request_id = 0
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            port = listener.getsockname()[1]
        self.base = f"http://127.0.0.1:{port}"
        self.url = self.base + "/mcp"
        options = ["--enable-mcp"] if mode == "serve" else []
        self.process = subprocess.Popen(
            [binary, mode, "--host", "127.0.0.1", "--port", str(port),
             "--enable-mutations", *options], cwd=workspace,
            stdout=self.log, stderr=subprocess.STDOUT, start_new_session=True,
            env={**os.environ, "TOKIO_WORKER_THREADS": "4",
                 "RUST_LOG": "reqvire::model=debug,reqvire::model_cache=debug,reqvire::utils=debug"})

    def counts(self):
        log = (self.output / "server.log").read_text()
        return {"builds": log.count("Starting two-pass validation architecture"),
                "scans": log.count("Scanning for markdown files in:"),
                "loads": log.count("model cache hit (") + log.count("model cache miss (")}

    def raw_tool(self, name, **arguments):
        return self.rpc("tools/call", {"name": name, "arguments": arguments})

    def http(self, path, data=None):
        request = urllib.request.Request(self.base + path,
            data=None if data is None else json.dumps(data).encode(),
            headers={"Content-Type": "application/json"})
        try:
            response = urllib.request.urlopen(request, timeout=10)
        except urllib.error.HTTPError as error:
            response = error
        with response:
            return response.status, response.read().decode()


QUERY = ('SELECT ?label WHERE { <https://example.test/external#ExternalResource> '
         '<http://www.w3.org/2000/01/rdf-schema#label> ?label }')


class Checks:
    def __init__(self, args):
        self.binary = args.binary
        self.root = Path(args.workspace) / "correctness"
        self.root.mkdir()
        self.output = Path(args.workspace) / "output/correctness"
        self.output.mkdir()
        self.fixtures = Path(__file__).with_name("fixtures")
        self.results = []
        self.mode = "mcp"
        self.serial = 0

    def check(self, name, passed, actual=None, expected=None):
        name = f"{self.mode}/{name}"
        if actual is None and expected is None:
            actual, expected = bool(passed), True
        self.results.append({"name": name, "passed": bool(passed),
                             "actual": actual, "expected": expected})
        if not passed:
            print(f"FAIL {name}: expected {str(expected)[:240]!r}, received {str(actual)[:240]!r}", flush=True)

    def equal(self, name, actual, expected):
        self.check(name, actual == expected, actual, expected)

    def fixture(self, name):
        return (self.fixtures / name).read_text()

    def golden_file(self, name, actual, fixture):
        expected_path = Path(__file__).with_name("expected") / fixture
        expected = expected_path.read_text()
        received = actual.read_text()
        self.equal(name, received, expected)
        if received != expected:
            print("".join(difflib.unified_diff(expected.splitlines(True), received.splitlines(True),
                fromfile=str(expected_path), tofile=str(actual))), end="")

    def workspace(self, name, files):
        path = self.root / f"{self.mode}-{name}"
        path.mkdir()
        subprocess.run(["git", "init", "-q", str(path)], check=True)
        # Unborn HEAD and this untracked file keep HEAD/dirty metadata constant.
        (path / "dirty.txt").write_text("Always dirty, independent of tested inputs.\n")
        for target, fixture in files.items():
            dest = path / target
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_text(self.fixture(fixture))
        return path

    def server(self, workspace, mode=None):
        self.serial += 1
        return Server(self.binary, workspace, self.output, mode or self.mode,
                      f"{workspace.name}-{self.serial}")

    def names(self, server):
        result = server.tool("reqvire.search")
        return sorted(element["name"] for file in result["files"].values()
                      for element in file["elements"])

    def unchanged(self):
        path = self.workspace("regex", {"Model.md": "model.md.txt", "Ignored9.md": "other.md.txt"})
        (path / ".reqvireignore").write_text("**/*[0-9]*.md\n")
        with self.server(path) as server:
            self.equal("regex/inventory", self.names(server), ["Cache Subject"])
            before = server.counts()["builds"]
            for _ in range(12):
                server.tool("reqvire.workspace_status")
            self.equal("regex/unchanged-reads-do-not-build", server.counts()["builds"] - before, 0)

    def ignores(self):
        for filename in (".reqvireignore", ".gitignore"):
            prefix = filename[1:]
            path = self.workspace(prefix, {"Model.md": "model.md.txt", "Other.md": "other.md.txt",
                                           "Third.md": "third.md.txt"})
            with self.server(path) as server:
                self.equal(prefix + "/initial", self.names(server),
                           ["Cache Subject", "Other Subject", "Third Subject"])
                for action, rules, wanted in (
                    ("create", "Other.md\n", ["Cache Subject", "Third Subject"]),
                    ("edit", "Third.md\n", ["Cache Subject", "Other Subject"]),
                    ("remove", None, ["Cache Subject", "Other Subject", "Third Subject"]),
                ):
                    if rules is None:
                        (path / filename).unlink()
                    else:
                        (path / filename).write_text(rules)
                    actual = self.names(server)
                    with self.server(path) as fresh:
                        oracle = self.names(fresh)
                    self.equal(prefix + "/" + action + "-oracle", oracle, wanted)
                    self.equal(prefix + "/" + action, actual, oracle)
                (path / "nested").mkdir()
                (path / "nested" / filename).write_text("../Model.md\n")
                before = server.counts()["builds"]
                self.equal(prefix + "/nested-policy-unchanged", self.names(server), wanted)
                self.equal(prefix + "/nested-policy-no-build", server.counts()["builds"] - before, 0)
            (path / filename).write_text("Other.md\n")
            with self.server(path) as server:
                self.equal(prefix + "/initial-exclusion", self.names(server), ["Cache Subject", "Third Subject"])
                (path / filename).unlink()
                self.equal(prefix + "/remove-active-rule", self.names(server), ["Cache Subject", "Other Subject", "Third Subject"])

    def dependencies(self):
        for extension, format_name in (("ttl", "turtle"), ("rdf", "rdfxml"), ("jsonld", "jsonld")):
            prefix = "external-" + extension
            file = "external." + extension
            path = self.workspace(prefix, {"Model.md": "ontology.md.txt", file: file})
            model = path / "Model.md"
            model.write_text(model.read_text().replace("external.ttl", file).replace("format: turtle", "format: " + format_name))
            dependency = path / file
            original = dependency.read_text()
            with self.server(path) as server:
                revision = server.revision()
                self.check(prefix + "/initial", "Label alpha" in json.dumps(server.tool(
                    "reqvire.semantic.sparql", query=QUERY, include_external=True)))
                stat = dependency.stat()
                dependency.write_text(original.replace("Label alpha", "Label omega"))
                os.utime(dependency, ns=(stat.st_atime_ns, stat.st_mtime_ns))
                self.equal(prefix + "/same-size-mtime", (dependency.stat().st_size, dependency.stat().st_mtime_ns),
                           (stat.st_size, stat.st_mtime_ns))
                self.equal(prefix + "/revision-unchanged", server.revision(), revision)
                for tool, args in (("reqvire.semantic.sparql", {"query": QUERY, "include_external": True}),
                                   ("reqvire.semantic.vocabulary", {"include_external": True, "section": "classes"})):
                    actual = server.tool(tool, **args)
                    with self.server(path) as fresh:
                        oracle = fresh.tool(tool, **args)
                    self.check(prefix + "/" + tool.rsplit(".", 1)[1] + "-oracle", "Label omega" in json.dumps(oracle))
                    self.equal(prefix + "/" + tool.rsplit(".", 1)[1] + "-fresh", actual, oracle)
                # A fresh CLI builder supplies the authoritative validation result;
                # strict server startup cannot run over these invalid fixtures.
                for action in ("remove", "invalid", "unreadable", "restore"):
                    if action == "remove":
                        dependency.unlink()
                    elif action == "invalid":
                        dependency.write_text("This is not RDF.\n")
                    elif action == "unreadable":
                        dependency.unlink()
                        dependency.mkdir()  # deterministic EISDIR, also when run as root
                    else:
                        dependency.rmdir()
                        dependency.write_text(original.replace("Label alpha", "Label omega"))
                    actual = server.raw_tool("reqvire.semantic.vocabulary", include_external=True)
                    if action == "restore":
                        with self.server(path) as fresh:
                            oracle = fresh.raw_tool("reqvire.semantic.vocabulary", include_external=True)
                        self.equal(prefix + "/" + action, actual, oracle)
                        result = server.tool("reqvire.semantic.sparql", query=QUERY, include_external=True)
                        self.check(prefix + "/restore-query", "Label omega" in json.dumps(result))
                    else:
                        oracle = subprocess.run([self.binary, "validate", "--json"], cwd=path, capture_output=True, text=True)
                        (self.output / f"{self.mode}-{prefix}-{action}.json").write_text(oracle.stdout + oracle.stderr)
                        self.check(prefix + "/" + action + "-oracle", oracle.returncode != 0, oracle.returncode, "nonzero")
                        self.equal(prefix + "/" + action, actual.get("isError", False), True)

    def resolution(self):
        path = self.workspace("resolution", {"sub/Model.md": "ontology.md.txt", "sub/external.ttl": "external.ttl"})
        with self.server(path) as server:
            self.check("resolution/fallback", "Label alpha" in json.dumps(server.tool(
                "reqvire.semantic.sparql", query=QUERY, include_external=True)))
            (path / "external.ttl").write_text(self.fixture("external.ttl").replace("Label alpha", "Label omega"))
            for action in ("higher-priority-created", "higher-priority-removed"):
                if action.endswith("removed"):
                    (path / "external.ttl").unlink()
                actual = server.tool("reqvire.semantic.sparql", query=QUERY, include_external=True)
                with self.server(path) as fresh:
                    oracle = fresh.tool("reqvire.semantic.sparql", query=QUERY, include_external=True)
                self.equal("resolution/" + action, actual, oracle)
        (path / "external.ttl").write_text(self.fixture("external.ttl").replace("Label alpha", "Label omega"))
        with self.server(path) as server:
            server.tool("reqvire.semantic.sparql", query=QUERY, include_external=True)
            (path / "external.ttl").unlink()
            actual = server.tool("reqvire.semantic.sparql", query=QUERY, include_external=True)
            with self.server(path) as fresh:
                oracle = fresh.tool("reqvire.semantic.sparql", query=QUERY, include_external=True)
            self.equal("resolution/remove-active-root-source", actual, oracle)

    def evidence(self):
        path = self.workspace("evidence", {"Model.md": "evidence.md.txt", "evidence.txt": "evidence.txt"})
        with self.server(path) as server:
            server.tool("reqvire.search")
            (path / "evidence.txt").unlink()
            oracle = subprocess.run([self.binary, "validate", "--json"], cwd=path, capture_output=True, text=True)
            (self.output / f"{self.mode}-missing-evidence.json").write_text(oracle.stdout + oracle.stderr)
            self.check("evidence/fresh-builder-rejects", oracle.returncode != 0, oracle.returncode, "nonzero")
            actual = server.raw_tool("reqvire.search")
            self.equal("evidence/removal-rejects-stale-success", actual.get("isError", False), True)
            (path / "evidence.txt").write_text(self.fixture("evidence.txt"))
            self.equal("evidence/recovery", self.names(server), ["Evidence Capability", "Evidence Requirement"])
            before = server.counts()["builds"]
            (path / "evidence.txt").write_text("Changed evidence contents, same existence.\n")
            server.tool("reqvire.search")
            self.equal("evidence/content-only-does-not-build", server.counts()["builds"] - before, 0)

    def source_changes(self):
        path = self.workspace("sources", {"Model.md": "model-with-page.md.txt"})
        with self.server(path) as server:
            before = server.revision()
            page = path / "Model.md"
            stat = page.stat()
            page.write_text(page.read_text().replace("Page marker alpha", "Page marker omega"))
            os.utime(page, ns=(stat.st_atime_ns, stat.st_mtime_ns))
            builds = server.counts()["builds"]
            self.equal("sources/page-revision-unchanged", server.revision(), before)
            self.equal("sources/page-change-builds", server.counts()["builds"] - builds, 1)
            page_content = server.tool("reqvire.search")["files"]["Model.md"]["page_content"]
            self.check("sources/page-change-visible", "omega" in page_content, page_content, "Page marker omega")
            model = path / "Model.md"
            stat = model.stat()
            model.write_text(model.read_text().replace("alpha", "omega"))
            os.utime(model, ns=(stat.st_atime_ns, stat.st_mtime_ns))
            self.check("sources/same-stat-edit-visible", "omega" in json.dumps(server.tool("reqvire.read_element", name="Cache Subject")))
            (path / "Other.md").write_text(self.fixture("other.md.txt"))
            self.equal("sources/add-visible", self.names(server), ["Cache Subject", "Other Subject"])
            (path / "Other.md").rename(path / "Moved.md")
            self.check("sources/move-visible", "Moved.md" in server.tool("reqvire.search")["files"])
            (path / "Moved.md").unlink()
            self.equal("sources/remove-visible", self.names(server), ["Cache Subject"])
            (path / "Broken.md").write_text(self.fixture("broken.md.txt"))
            self.equal("sources/invalid-rejects-stale-success", server.raw_tool("reqvire.search").get("isError", False), True)
            (path / "Broken.md").unlink()
            self.equal("sources/repair-recovers", self.names(server), ["Cache Subject"])

    def runtime(self):
        path = self.workspace("runtime", {"Model.md": "model.md.txt"})
        with self.server(path) as server:
            server.tool("reqvire.search")
            initial = server.http("/api/project-store/manifest")
            model = path / "Model.md"
            original = model.read_text()
            model.write_text(original.replace("alpha", "omega"))
            self.check("runtime/external-edit-visible-to-mcp", "omega" in json.dumps(server.tool("reqvire.read_element", name="Cache Subject")))
            counts = server.counts()
            self.equal("runtime/no-external-polling", server.http("/api/project-store/manifest"), initial)
            self.equal("runtime/manifest-does-not-load-model", server.counts(), counts)
            add = {"file": "Model.md", "content": self.fixture("other.md.txt").split("# Elements\n\n", 1)[1], "dry_run": True}
            counts = server.counts()["loads"]
            preview = server.raw_tool("reqvire.add_element", **add)
            self.equal("runtime/preview-success", preview.get("isError", False), False)
            self.equal("runtime/preview-no-refresh-load", server.counts()["loads"] - counts, 1)
            self.equal("runtime/preview-keeps-assets", server.http("/api/project-store/manifest"), initial)
            self.golden_file("runtime/preview-keeps-file", model, "runtime-before-write.md.txt")
            # Duplicate names pass argument validation but fail core mutation validation:
            # JSON-RPC result exists, with isError:true. This is the broken hook gate.
            counts = server.counts()["loads"]
            rejected = server.raw_tool("reqvire.add_element", file="Model.md",
                content=original.split("# Elements\n\n", 1)[1], dry_run=False)
            self.equal("runtime/rejected-is-tool-error", rejected.get("isError", False), True)
            self.equal("runtime/rejected-no-refresh-load", server.counts()["loads"] - counts, 1)
            self.equal("runtime/rejected-keeps-assets", server.http("/api/project-store/manifest"), initial)
            self.golden_file("runtime/rejected-keeps-file", model, "runtime-before-write.md.txt")
            add["dry_run"] = False
            server.tool("reqvire.add_element", **add)
            self.golden_file("runtime/success-persists-file", model, "runtime-after-write.md.txt")
            status, store = server.http("/api/project-store")
            self.equal("runtime/success-status", status, 200)
            self.check("runtime/success-publishes-new-graph", "Other Subject" in store and "omega" in store)
            self.equal("runtime/post-write-search", self.names(server), ["Cache Subject", "Other Subject"])
            query = 'ASK { ?s <https://www.reqvire.org/ontology#elementName> "Other Subject" }'
            self.equal("runtime/post-write-sparql", server.tool("reqvire.semantic.sparql", query=query).get("boolean"), True)
            counts = server.counts()
            manifest = json.loads(server.http("/api/project-store/manifest")[1])
            hashes = []
            for section in manifest["sections"].values():
                hashes.extend(section["hashes"] if section["kind"] == "array" else [section["hash"]])
            revision = json.loads(store)["revision"]
            self.equal("runtime/chunks-status", server.http("/api/project-store/chunks", {"revision": revision, "hashes": hashes})[0], 200)
            self.equal("runtime/chunks-do-not-load-model", server.counts(), counts)

    def run(self):
        for self.mode in ("mcp", "serve"):
            cases = [self.unchanged, self.ignores, self.dependencies, self.resolution, self.evidence, self.source_changes]
            if self.mode == "serve":
                cases.append(self.runtime)
            for case in cases:
                try:
                    case()
                except Exception:
                    details = traceback.format_exc()
                    self.check(case.__name__ + "/harness-error", False, details, "scenario completes")
        (self.output / "results.json").write_text(json.dumps(self.results, indent=2) + "\n")
        (self.output / "checks.txt").write_text("".join(
            f"{'PASS' if result['passed'] else 'FAIL'} {result['name']}\n" for result in self.results))
        failures = sum(not result["passed"] for result in self.results)
        print(f"Cache correctness: {len(self.results) - failures} passed, {failures} failed; artifacts: {self.output}")
        return bool(failures)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", required=True)
    parser.add_argument("--workspace", required=True)
    raise SystemExit(Checks(parser.parse_args()).run())
