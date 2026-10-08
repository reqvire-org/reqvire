"""Independent trace topology oracle, exercised through the CLI and static export."""
import json
import shutil
import socket
import threading
import time
import urllib.request
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
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
    # Reuse owning semantic fixtures for shared-selection browser checks.
    for fixture in ("Thesaurus.md", "Ontology.md"):
        shutil.copyfile(Path(__file__).parents[1] / "test-thesaurus-project-store/specifications" / fixture, workspace / fixture)

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

# Exercise the compiled production view against the same independent topology oracle.
# Use the smaller graph; the 40-layer case above remains a data-size/parity gate.
workspace = output / f"dag-{depths[0]}"
with socket.socket() as listener:
    listener.bind(("127.0.0.1", 0))
    port = listener.getsockname()[1]

class QuietHandler(SimpleHTTPRequestHandler):
    def log_message(self, *_args):
        pass

static = ThreadingHTTPServer(("127.0.0.1", 0), partial(QuietHandler, directory=str(workspace / "export")))
thread = threading.Thread(target=static.serve_forever, daemon=True)
thread.start()
with (output / "serve.log").open("w") as log:
    server = subprocess.Popen([binary, "--workspace", str(workspace), "serve", "--host", "127.0.0.1", "--port", str(port)], stdout=log, stderr=log)
    try:
        url = f"http://127.0.0.1:{port}"
        deadline = time.monotonic() + 30
        while True:
            if server.poll() is not None:
                raise RuntimeError((output / "serve.log").read_text())
            try:
                with urllib.request.urlopen(url, timeout=1) as response:
                    assert response.status == 200
                break
            except OSError:
                if time.monotonic() > deadline:
                    raise RuntimeError("Trace browser server did not start")
                time.sleep(0.05)
        result = subprocess.run(["node", str(Path(__file__).with_name("browser-flow.mjs")), url,
                                 f"http://127.0.0.1:{static.server_port}", str(output / "browser-profile"),
                                 str(workspace / "traces.json")], text=True, capture_output=True, timeout=90)
        (output / "browser.log").write_text(result.stderr)
        print(result.stdout, end="")
        if result.returncode:
            raise RuntimeError(result.stderr)
    finally:
        server.terminate()
        try:
            server.wait(timeout=5)
        except subprocess.TimeoutExpired:
            server.kill()
            server.wait(timeout=5)
        static.shutdown()
        static.server_close()
        thread.join(timeout=5)
