#!/usr/bin/env python3
"""Black-box revision checks; expected canonical bytes never come from Reqvire."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import socket
import subprocess
import time
import urllib.error
import urllib.request


PROTOCOL = "2025-11-25"


class McpServer:
    def __init__(self, binary, workspace, output, name, *options):
        self.output = output / name
        self.output.mkdir()
        self.log = (self.output / "server.log").open("w")
        self.session = None
        self.request_id = 0
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            port = listener.getsockname()[1]
        self.url = f"http://127.0.0.1:{port}/mcp"
        self.process = subprocess.Popen(
            [binary, "mcp", "--host", "127.0.0.1", "--port", str(port), *options],
            cwd=workspace,
            stdout=self.log,
            stderr=subprocess.STDOUT,
            start_new_session=True,
        )

    def __enter__(self):
        try:
            deadline = time.monotonic() + 15
            while time.monotonic() < deadline:
                if self.process.poll() is not None:
                    raise RuntimeError(f"MCP server exited; see {self.output / 'server.log'}")
                try:
                    result = self.rpc("initialize", {
                        "protocolVersion": PROTOCOL,
                        "capabilities": {},
                        "clientInfo": {"name": "revision-e2e", "version": "1"},
                    })
                    if result["protocolVersion"] != PROTOCOL:
                        raise RuntimeError(f"Unexpected initialization result: {result}")
                    return self
                except (urllib.error.URLError, TimeoutError, ConnectionError):
                    time.sleep(0.1)
            raise RuntimeError(f"MCP startup timed out; see {self.output / 'server.log'}")
        except BaseException:
            self.__exit__(None, None, None)
            raise

    def __exit__(self, *_):
        # The timing wrapper and CLI share this new process group. Stop both.
        try:
            os.killpg(self.process.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
        try:
            self.process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(self.process.pid, signal.SIGKILL)
            self.process.wait(timeout=5)
        self.log.close()

    def rpc(self, method, params):
        self.request_id += 1
        request = {"jsonrpc": "2.0", "id": self.request_id, "method": method, "params": params}
        headers = {
            "Content-Type": "application/json",
            "Accept": "application/json, text/event-stream",
            "Mcp-Protocol-Version": PROTOCOL,
        }
        if self.session:
            headers["Mcp-Session-Id"] = self.session
        http_request = urllib.request.Request(self.url, json.dumps(request).encode(), headers)
        with urllib.request.urlopen(http_request, timeout=5) as response:
            self.session = response.headers.get("Mcp-Session-Id", self.session)
            payload = json.loads(response.read())
        (self.output / f"response-{self.request_id:03}.json").write_text(
            json.dumps({"request": request, "response": payload}, indent=2) + "\n"
        )
        if payload.get("id") != self.request_id or "error" in payload:
            raise RuntimeError(f"MCP {method} failed: {payload}")
        return payload["result"]

    def tool(self, tool_name, **arguments):
        result = self.rpc("tools/call", {"name": tool_name, "arguments": arguments})
        if result.get("isError") or "structuredContent" not in result:
            raise RuntimeError(f"Tool {tool_name} failed: {result}")
        return result["structuredContent"]

    def revision(self):
        value = self.tool("reqvire.model_revision")["model_fingerprint"]
        if not isinstance(value, str):
            raise RuntimeError(f"Non-string model fingerprint: {value!r}")
        return value


def main():
    parser = argparse.ArgumentParser()
    for argument in ("binary", "workspace", "fixtures", "expected"):
        parser.add_argument("--" + argument, required=True)
    args = parser.parse_args()
    root, fixtures, expected = map(Path, (args.workspace, args.fixtures, args.expected))
    output = root / "output"
    workspace = root / "model"
    workspace.mkdir()
    model_file = workspace / "Model.md"
    failures = 0

    with (output / "checks.txt").open("w") as checks:
        def check(name, passed, actual=None, wanted=None):
            nonlocal failures
            checks.write(f"{'PASS' if passed else 'FAIL'} {name}\n")
            checks.flush()
            if not passed:
                failures += 1
                print(f"FAIL {name}: expected {wanted!r}, received {actual!r}", flush=True)

        def golden(name, actual):
            # These are checked-in byte fixtures, not serialization of tool output.
            data = bytes.fromhex((expected / f"{name}.hex").read_text())
            digest = (expected / f"{name}.sha256").read_text().strip()
            if hashlib.sha256(data).hexdigest() != digest:
                raise RuntimeError(f"Corrupt canonical fixture: {name}")
            check(f"canonical-v1-{name}", actual == digest, actual, digest)

        def write(text):
            model_file.write_text(text, encoding="utf-8")

        def changed(server, name, original, modified, baseline, equal=False):
            if modified == original:
                raise RuntimeError(f"Case {name} did not modify its fixture")
            write(modified)
            actual = server.revision()
            check(name, (actual == baseline) == equal, actual,
                  baseline if equal else f"different from {baseline}")
            write(original)

        minimal = (fixtures / "minimal.md.txt").read_text()
        multiple = (fixtures / "multiple.md.txt").read_text()
        graph = (fixtures / "graph.md.txt").read_text()
        namespaces = (fixtures / "namespaces.md.txt").read_text()

        # An empty effective workspace is still inside the runner's Git worktree.
        with McpServer(args.binary, workspace, output, "primary") as server:
            golden("empty", server.revision())
            write(minimal)
            baseline = server.revision()
            parsed = server.tool("reqvire.read_element", name="Hash Subject")
            if parsed["content"] != "Café 測定." or parsed["identifier"] != "Model.md#hash-subject":
                raise RuntimeError(f"Minimal fixture parsed unexpectedly: {parsed}")
            check("digest-format", re.fullmatch(r"[0-9a-f]{64}", baseline) is not None,
                  baseline, "64 lowercase hexadecimal characters")
            golden("minimal", baseline)
            check("unchanged-repeat", server.revision() == baseline)

            definitions = {tool["name"]: tool for tool in server.rpc("tools/list", {})["tools"]}
            for name in ("workspace_status", "model_revision", "semantic.prefixes", "semantic.vocabulary", "semantic.sparql"):
                properties = definitions["reqvire." + name]["outputSchema"].get("properties", {})
                schema = (properties.get("model", {}).get("properties", {}).get("fingerprint", {})
                          if name == "workspace_status" else properties.get("model_fingerprint", {}))
                check(f"schema-{name}", schema.get("type") == "string" and schema.get("pattern") == "^[0-9a-f]{64}$",
                      schema, "string with ^[0-9a-f]{64}$ pattern")

            tools = [
                ("reqvire.workspace_status", {}, lambda r: r["model"]["fingerprint"]),
                ("reqvire.semantic.prefixes", {}, lambda r: r["model_fingerprint"]),
                ("reqvire.semantic.vocabulary", {"section": "all"}, lambda r: r["model_fingerprint"]),
                ("reqvire.semantic.sparql", {"query": "ASK {}"}, lambda r: r["model_fingerprint"]),
            ]
            for tool, arguments, fingerprint in tools:
                actual = fingerprint(server.tool(tool, **arguments))
                check(f"same-snapshot-{tool.removeprefix('reqvire.')}", actual == baseline, actual, baseline)

            for key, before, after in [
                ("owner", "team-a", "team-b"), ("status", "draft", "approved"),
                ("priority", "medium", "high"), ("risk", "low", "critical"),
            ]:
                changed(server, f"metadata-{key}", minimal,
                        minimal.replace(f"{key}: {before}", f"{key}: {after}"), baseline)
            metadata = "".join(line + "\n" for line in minimal.splitlines() if line.startswith("  * "))
            changed(server, "metadata-order", minimal,
                    minimal.replace(metadata, "".join(reversed(metadata.splitlines(keepends=True)))),
                    baseline, equal=True)
            changed(server, "content-whitespace", minimal, minimal.replace("Café 測定.", "Café  測定."), baseline)
            changed(server, "page-frontmatter", minimal,
                    minimal.replace("# Elements\n", "# Elements\n\nPage annotation.\n"), baseline, equal=True)
            changed(server, "element-rename", minimal, minimal.replace("Hash Subject", "Renamed Subject"), baseline)
            model_file.rename(workspace / "Moved.md")
            moved = server.revision()
            check("element-move", moved != baseline, moved, f"different from {baseline}")
            (workspace / "Moved.md").rename(model_file)

            relocated = root / "relocated-model"
            relocated.mkdir()
            (relocated / "Model.md").write_text(minimal)
            with McpServer(args.binary, relocated, output, "relocated") as copy:
                actual = copy.revision()
                check("absolute-workspace-location", actual == baseline, actual, baseline)
            with McpServer(args.binary, workspace, output, "restarted") as restarted:
                actual = restarted.revision()
                check("process-restart", actual == baseline, actual, baseline)
            with McpServer(args.binary, workspace, output, "size-estimates", "--with-size-estimates") as sized:
                if "size_estimate" not in sized.tool("reqvire.read_element", name="Hash Subject"):
                    raise RuntimeError("Size-estimate fixture did not enable size estimates")
                actual = sized.revision()
                check("excluded-size-estimates", actual == baseline, actual, baseline)

            write(multiple)
            multi_revision = server.revision()
            golden("multiple", multi_revision)
            heading, body = multiple.split("### Alpha", 1)
            alpha, omega = ("### Alpha" + body).split("### Omega", 1)
            changed(server, "element-order", multiple, heading + "### Omega" + omega + "\n" + alpha,
                    multi_revision, equal=True)
            write(heading + alpha)
            single_revision = server.revision()
            check("element-removal", single_revision != multi_revision, single_revision,
                  f"different from {multi_revision}")
            write(multiple)
            check("element-addition-restores-revision", server.revision() == multi_revision)

            (workspace / "artifact-a.txt").write_text("Evidence A\n")
            (workspace / "artifact-b.txt").write_text("Evidence B\n")
            write(graph)
            graph_revision = server.revision()
            relation_a = "  * satisfiedBy: [First evidence](artifact-a.txt)\n"
            relation_b = "  * satisfiedBy: [Second evidence](artifact-b.txt)\n"
            changed(server, "relation-order", graph, graph.replace(relation_a + relation_b, relation_b + relation_a),
                    graph_revision, equal=True)
            # Duplicate authored relations are rejected before hashing. Encoder
            # deduplication needs a Rust fixture for the resolved graph instead.
            inverse_graph = graph.replace("  * specify: [Hash Feature](#hash-feature)\n", "")
            inverse_graph = inverse_graph.replace("Consumer capability.\n\n#### Metadata\n  * type: capability\n",
                "Consumer capability.\n\n#### Metadata\n  * type: capability\n\n#### Relations\n"
                "  * specifiedBy: [Hash Requirement](#hash-requirement)\n")
            changed(server, "authored-inverse-equivalence", graph, inverse_graph, graph_revision, equal=True)
            changed(server, "relation-label", graph, graph.replace("[First evidence]", "[New display label]"),
                    graph_revision, equal=True)
            changed(server, "relation-removal", graph, graph.replace(relation_b, ""), graph_revision)
            binding_a, binding_b = "  * [Spec A](#spec-a)\n", "  * [Spec B](#spec-b)\n"
            changed(server, "binding-order", graph, graph.replace(binding_a + binding_b, binding_b + binding_a),
                    graph_revision, equal=True)
            changed(server, "binding-target", graph, graph.replace(binding_b, "  * [Spec C](#spec-c)\n"), graph_revision)
            changed(server, "literal-whitespace", graph, graph.replace("`a b`", "`ab`"), graph_revision)
            (workspace / "artifact-a.txt").write_text("Changed evidence bytes\n")
            check("excluded-artifact-bytes", server.revision() == graph_revision)

            write(namespaces)
            namespace_revision = server.revision()
            for key, before, after in [
                ("ontology_base", "https://example.test/hashing", "https://example.test/hashing-v2"),
                ("ontology_prefix", "hash", "hashv2"),
                ("concept_base", "https://example.test/concepts", "https://example.test/concepts-v2"),
                ("concept_prefix", "concepts", "conceptsv2"),
            ]:
                modified = namespaces.replace(f"{key}: {before}", f"{key}: {after}")
                if key == "ontology_base":
                    # The new base requires a matching prefix. Both prefixes and
                    # ontology declarations already exist in unchanged content.
                    modified = modified.replace("ontology_prefix: hash\n", "ontology_prefix: relocated\n")
                changed(server, f"metadata-{key}", namespaces,
                        modified, namespace_revision)

            # Supported Unix platforms permit filenames that cannot be UTF-8.
            # The source reader must reject them before lossy parsed identifiers
            # can be passed into the canonical encoder.
            invalid_path = os.fsencode(workspace) + b"/invalid-\xff.md"
            try:
                with open(invalid_path, "wb") as invalid:
                    invalid.write(minimal.replace("Hash Subject", "Invalid Path Subject").encode())
                result = server.rpc("tools/call", {"name": "reqvire.model_revision", "arguments": {}})
                check("non-utf8-source-path", result.get("isError") is True and "not UTF-8" in json.dumps(result),
                      result, "explicit UTF-8 path error")
            finally:
                os.unlink(invalid_path)

    print(f"Model revision checks: {failures} failure(s); responses and server logs: {output}")
    return int(failures != 0)


if __name__ == "__main__":
    raise SystemExit(main())
