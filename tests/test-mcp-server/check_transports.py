"""Black-box transport and process-lifecycle checks; no transport implementation.

Invoked by the existing MCP suite. --only allows the transport baseline to be
rerun independently. Each case retains protocol transcripts and a JSON report.
"""
import argparse
import difflib
import json
import os
import pathlib
import queue
import shutil
import signal
import socket
import subprocess
import sys
import tarfile
import threading
import time
import urllib.error
import urllib.parse
import urllib.request
from contextlib import ExitStack

PROTOCOL = "2025-11-25"
SUITE = pathlib.Path(__file__).resolve().parent
REPO = SUITE.parent.parent
INIT = {"protocolVersion": PROTOCOL, "capabilities": {},
        "clientInfo": {"name": "reqvire-transport-e2e", "version": "1"}}
ADD_CONTENT = """### Stdio Added Requirement

The system SHALL preserve accepted local model work.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Stdio Project A](Model.md#stdio-project-a)
"""


def require(ok, message):
    if not ok:
        raise AssertionError(message)


def git(root, *args):
    return subprocess.check_output(["git", *args], cwd=root, text=True).strip()


def project(folder, label="A", invalid=False):
    folder.mkdir(parents=True)
    (folder / "Model.md").write_text(f"""# Elements

### Stdio Project {label}

As a developer, I want isolated engineering context for this project.

#### Metadata
  * type: capability
---

### Stdio Requirement {label}

The system SHALL expose only this project's model context.

#### Metadata
  * type: {'invalid-type' if invalid else 'requirement'}

#### Relations
  * specify: [Stdio Project {label}](#stdio-project-{label.lower()})
---
""")
    git(folder, "init", "-qb", "main")
    git(folder, "config", "user.name", "Transport Test")
    git(folder, "config", "user.email", "transport@example.invalid")
    git(folder, "add", ".")
    git(folder, "-c", "commit.gpgSign=false", "commit", "-qm", "Transport fixture")
    return folder


def stop(process):
    # Kill only this test's owned group, including npm/native/worker descendants.
    try:
        os.killpg(process.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass
    try:
        process.wait(timeout=3)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait(timeout=3)


class Stdio:
    def __init__(self, binary, root, logs, *options, launcher=None):
        self.command = [*(launcher or [binary]), "--workspace", str(root), "mcp",
                        "--transport", "stdio", *options]
        self.logs = logs
        logs.mkdir(parents=True)
        self.error_file = (logs / "stderr.txt").open("w")
        self.wire_file = (logs / "stdout.jsonl").open("w")
        self.responses = queue.Queue()
        self.invalid_output = []
        self.next_id = 0
        self.closed = False
        self.process = subprocess.Popen(self.command, cwd=logs, stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=self.error_file,
                                        text=True, bufsize=1, start_new_session=True)
        self.reader = threading.Thread(target=self.read, daemon=True)
        self.reader.start()

    def read(self):
        for line in self.process.stdout:
            self.wire_file.write(line)
            self.wire_file.flush()
            try:
                message = json.loads(line)
                require(isinstance(message, dict) and message.get("jsonrpc") == "2.0",
                        "stdout contains a non-MCP JSON object")
                require("method" in message or ("id" in message and
                        ("result" in message or "error" in message)), "invalid JSON-RPC stdout message")
                self.responses.put(message)
            except (ValueError, AssertionError) as error:
                self.invalid_output.append(str(error) + ": " + line.rstrip())
                self.responses.put(error)
        self.responses.put(None)

    def send(self, message):
        try:
            self.process.stdin.write(json.dumps(message) + "\n")
            self.process.stdin.flush()
        except BrokenPipeError:
            raise AssertionError("stdio process exited before request: " + self.diagnostics()) from None

    def diagnostics(self):
        return (self.logs / "stderr.txt").read_text().strip()

    def raw(self, method, params):
        self.next_id += 1
        ident = self.next_id
        self.send({"jsonrpc": "2.0", "id": ident, "method": method, "params": params})
        deadline = time.monotonic() + 10
        while True:
            try:
                value = self.responses.get(timeout=max(0.01, deadline - time.monotonic()))
            except queue.Empty:
                raise AssertionError(f"no stdio response for {method}: {self.diagnostics()}") from None
            if value is None:
                raise AssertionError(f"stdio exited before {method}: {self.diagnostics()}")
            if isinstance(value, Exception):
                raise AssertionError(f"invalid stdout: {self.invalid_output}")
            if value.get("id") == ident:
                return value
            require("id" not in value, f"unexpected response ID or notification response: {value}")

    def rpc(self, method, params):
        value = self.raw(method, params)
        require("error" not in value, f"{method} protocol error: {value}")
        return value["result"]

    def __enter__(self):
        try:
            result = self.rpc("initialize", INIT)
            require(result["protocolVersion"] == PROTOCOL, "unexpected protocol version")
            require(all(name in result["capabilities"] for name in ("tools", "resources", "prompts")),
                    "initialization omits capabilities")
            self.send({"jsonrpc": "2.0", "method": "notifications/initialized"})
            return self
        except BaseException:
            self.__exit__(*sys.exc_info())
            raise

    def finish(self, sig=None):
        if sig:
            self.process.send_signal(sig)
        else:
            self.process.stdin.close()
        status = self.process.wait(timeout=10)
        self.reader.join(timeout=2)
        require(not self.reader.is_alive(), "stdout remains open after server exit; worker may have survived")
        require(not self.invalid_output, str(self.invalid_output))
        if not sig:
            require(status == 0, f"EOF exit status {status}: {self.diagnostics()}")
        self.closed = True
        return status

    def __exit__(self, kind=None, *_):
        try:
            if not self.closed and kind is None:
                self.finish()
        finally:
            stop(self.process)
            self.reader.join(timeout=2)
            self.error_file.close()
            self.wire_file.close()


class Http:
    def __init__(self, binary, root, logs, *options, explicit=False):
        logs.mkdir(parents=True)
        self.logs = logs
        with socket.socket() as sock:
            sock.bind(("127.0.0.1", 0))
            self.port = sock.getsockname()[1]
        self.output = (logs / "server.txt").open("w")
        args = [binary, "--workspace", str(root), "mcp", "--port", str(self.port), *options]
        if explicit:
            args += ["--transport", "http"]
        self.process = subprocess.Popen(args, cwd=logs, stdout=self.output, stderr=self.output,
                                        start_new_session=True)
        self.next_id = 0

    def raw(self, method, params):
        self.next_id += 1
        data = json.dumps({"jsonrpc": "2.0", "id": self.next_id,
                           "method": method, "params": params}).encode()
        request = urllib.request.Request(f"http://127.0.0.1:{self.port}/mcp", data,
            {"Content-Type": "application/json", "Accept": "application/json, text/event-stream",
             "Mcp-Protocol-Version": PROTOCOL})
        with urllib.request.urlopen(request, timeout=10) as response:
            value = json.load(response)
        (self.logs / f"response-{self.next_id:03}.json").write_text(json.dumps(value, indent=2))
        return value

    def rpc(self, method, params):
        value = self.raw(method, params)
        require("error" not in value, f"HTTP protocol error: {value}")
        return value["result"]

    def __enter__(self):
        try:
            for _ in range(200):
                require(self.process.poll() is None,
                        "HTTP startup failed: " + (self.logs / "server.txt").read_text())
                try:
                    result = self.rpc("initialize", INIT)
                    require(result["protocolVersion"] == PROTOCOL, "HTTP protocol version mismatch")
                    return self
                except (urllib.error.URLError, ConnectionError, TimeoutError):
                    time.sleep(0.05)
            raise AssertionError("HTTP initialization timed out")
        except BaseException:
            self.__exit__()
            raise

    def __exit__(self, *_):
        stop(self.process)
        self.output.close()


def call(server, tool_name, **arguments):
    result = server.rpc("tools/call", {"name": tool_name, "arguments": arguments})
    require(not result.get("isError"), f"{tool_name} tool failed: {result}")
    return result["structuredContent"]


def catalogs(server):
    return {method: server.rpc(method, {}) for method in
            ("tools/list", "resources/list", "resources/templates/list", "prompts/list")}


def mutation_catalogs(server):
    discovery = catalogs(server)
    original = call(server, "reqvire.worktree.list")["original_worktree_id"]
    scoped = 0
    for resource in discovery["resources/list"]["resources"]:
        uri = urllib.parse.urlsplit(resource["uri"])
        if uri.scheme == "reqvire" and uri.netloc == "workspace" and uri.path in (
                "/status", "/model-revision"):
            require(urllib.parse.parse_qs(uri.query) == {"worktree_id": [original]},
                    "workspace resource does not select the current process's original context")
            # Context IDs belong to a server process, not to its transport.
            resource["uri"] = uri._replace(query="worktree_id=SESSION").geturl()
            scoped += 1
    require(scoped == 2, "missing context-scoped workspace resources")
    return discovery


def compare(expected, actual, label):
    if expected != actual:
        delta = "\n".join(difflib.unified_diff(
            json.dumps(expected, indent=2, sort_keys=True).splitlines(),
            json.dumps(actual, indent=2, sort_keys=True).splitlines(),
            fromfile="HTTP/expected", tofile="stdio/actual"))
        raise AssertionError(label + "\n" + delta)


def expected_document(root):
    actual = (root / "Added.md").read_text()
    expected = (SUITE / "expected/stdio-added-document.md.txt").read_text()
    if actual != expected:
        raise AssertionError("persisted document differs:\n" + "\n".join(difflib.unified_diff(
            expected.splitlines(), actual.splitlines(), fromfile="expected", tofile="Added.md")))


def rejected(binary, root, logs, *args, diagnostic, launcher=None):
    command = [*(launcher or [binary]), "--workspace", str(root), "mcp", "--transport", "stdio", *args]
    result = subprocess.run(command, cwd=logs, input="", capture_output=True, text=True, timeout=10)
    (logs / "rejected.stdout").write_text(result.stdout)
    (logs / "rejected.stderr").write_text(result.stderr)
    require(result.returncode != 0 and diagnostic.lower() in result.stderr.lower(), result.stderr)
    require("unexpected argument '--transport'" not in result.stderr, "transport unsupported: " + result.stderr)
    require(not result.stdout, "startup failure contaminated protocol stdout")
    return result.returncode


def default_http(binary, folder):
    root = project(folder / "project")
    with Http(binary, root, folder / "http") as server:
        discovery = catalogs(server)
        names = [row["name"] for row in discovery["tools/list"]["tools"]]
        require("reqvire.search" in names and "reqvire.add_element" not in names, "default permissions changed")
        require("Stdio Requirement A" in json.dumps(call(server, "reqvire.search")), "wrong workspace")


def explicit_http(binary, folder):
    root = project(folder / "project")
    with Http(binary, root, folder / "default") as server:
        expected = catalogs(server)
    with Http(binary, root, folder / "explicit", explicit=True) as server:
        compare(expected, catalogs(server), "explicit HTTP discovery differs from default")


def default_http_mutation_fixture(binary, folder):
    root = project(folder / "project")
    with Http(binary, root, folder / "http", "--enable-mutations") as server:
        call(server, "reqvire.add_element", file="Added.md", content=ADD_CONTENT)
        expected_document(root)


def stdio_parity(binary, folder):
    root = project(folder / "project")
    with Http(binary, root, folder / "http") as server:
        expected = catalogs(server)
        search = call(server, "reqvire.search", filter_name="^Stdio Requirement A$")
        resource = server.rpc("resources/read", {"uri": "reqvire://workspace/status"})
        prompt = server.rpc("prompts/get", {"name": "reqvire.workflow.explore_model",
                                           "arguments": {"question": "Show relevant requirements"}})
    with Stdio(binary, root, folder / "stdio") as server:
        compare(expected, catalogs(server), "default discovery parity")
        compare(search, call(server, "reqvire.search", filter_name="^Stdio Requirement A$"), "search parity")
        compare(resource, server.rpc("resources/read", {"uri": "reqvire://workspace/status"}), "resource metadata parity")
        compare(prompt, server.rpc("prompts/get", {"name": "reqvire.workflow.explore_model",
                "arguments": {"question": "Show relevant requirements"}}), "prompt parity")


def permissions_errors(binary, folder):
    root = project(folder / "project")
    before = (root / "Model.md").read_bytes()
    with Stdio(binary, root, folder / "stdio") as server:
        names = [t["name"] for t in server.rpc("tools/list", {})["tools"]]
        require("reqvire.add_element" not in names, "stdio implicitly enabled mutations")
        denied = server.raw("tools/call", {"name": "reqvire.add_element",
            "arguments": {"file": "Added.md", "content": ADD_CONTENT}})
        require("error" in denied or denied.get("result", {}).get("isError"), "direct mutation accepted")
        require(server.raw("unknown/method", {})["error"]["code"] == -32601, "unknown method not rejected")
        invalid = server.raw("tools/call", {"name": "reqvire.search", "arguments": {"unknown": True}})
        require(invalid.get("error", {}).get("code") == -32602, "invalid arguments not rejected")
    require(before == (root / "Model.md").read_bytes() and not (root / "Added.md").exists(), "denied mutation wrote files")


def project_isolation(binary, folder):
    a, b = project(folder / "a"), project(folder / "b", "B")
    with ExitStack() as stack:
        port = stack.enter_context(socket.socket())
        # An existing listener is also sufficient to establish this precondition.
        try:
            port.bind(("127.0.0.1", 8081))
            port.listen()
        except OSError:
            with socket.create_connection(("127.0.0.1", 8081), timeout=2):
                pass
        first = stack.enter_context(Stdio(binary, a, folder / "client-a"))
        second = stack.enter_context(Stdio(binary, b, folder / "client-b"))
        left, right = json.dumps(call(first, "reqvire.search")), json.dumps(call(second, "reqvire.search"))
        require("Stdio Requirement A" in left and "Stdio Requirement B" not in left, "project A leaks project B")
        require("Stdio Requirement B" in right and "Stdio Requirement A" not in right, "project B leaks project A")


def mutation(binary, folder):
    root = project(folder / "project")
    head, index = git(root, "rev-parse", "HEAD"), git(root, "write-tree")
    with Http(binary, root, folder / "http", "--enable-mutations") as server:
        expected = mutation_catalogs(server)
    with Stdio(binary, root, folder / "stdio", "--enable-mutations") as server:
        compare(expected, mutation_catalogs(server), "mutation catalog parity")
        before = (root / "Model.md").read_bytes()
        preview = call(server, "reqvire.add_element", file="Added.md", content=ADD_CONTENT, dry_run=True)
        require(preview["dry_run"] and preview["diffs"], "preview lacks diff")
        require(not (root / "Added.md").exists() and before == (root / "Model.md").read_bytes(), "preview persisted")
        accepted = call(server, "reqvire.add_element", file="Added.md", content=ADD_CONTENT)
        require(accepted["dry_run"] is False, "mutation was not applied")
        expected_document(root)
        require(call(server, "reqvire.read_element", name="Stdio Added Requirement")["name"] ==
                "Stdio Added Requirement", "post-write read is stale")
    require(git(root, "rev-parse", "HEAD") == head and git(root, "write-tree") == index, "mutation committed or staged implicitly")
    expected_document(root)


def estimates(binary, folder):
    root = project(folder / "project")
    for enabled in (False, True):
        options = ["--with-size-estimates"] if enabled else []
        with Http(binary, root, folder / f"http-{enabled}", *options) as http:
            expected = call(http, "reqvire.read_element", name="Stdio Requirement A")
        with Stdio(binary, root, folder / f"stdio-{enabled}", *options) as server:
            actual = call(server, "reqvire.read_element", name="Stdio Requirement A")
            compare(expected, actual, "size estimates parity")
            status = call(server, "reqvire.workspace_status")
            require(status["size_estimates_enabled"] == enabled, "estimate flag not preserved")
            require(("size_estimate" in actual) == enabled, "estimates are not opt-in")


def ownership(binary, folder):
    root = project(folder / "project")
    with Stdio(binary, root, folder / "owner", "--enable-mutations"):
        rejected(binary, root, folder, "--enable-mutations", diagnostic="owned")
    with Stdio(binary, root, folder / "reacquired", "--enable-mutations"):
        pass


def lifecycle(binary, folder, sig=None):
    root = project(folder / "project")
    with Stdio(binary, root, folder / "owner", "--enable-mutations") as server:
        server.finish(sig)
        # Reacquire BEFORE forced test cleanup, so surviving worker locks cannot be hidden.
        with Stdio(binary, root, folder / "reacquired", "--enable-mutations"):
            pass


def preserved_worktree(binary, folder):
    root = project(folder / "project")
    with Stdio(binary, root, folder / "stdio", "--enable-mutations") as server:
        created = call(server, "reqvire.worktree.create", branch="stdio-pending", base_ref="main")
        child = pathlib.Path(created["workspace_root"])
        head, index = git(child, "rev-parse", "HEAD"), git(child, "write-tree")
        call(server, "reqvire.add_element", worktree_id=created["worktree_id"], file="Added.md", content=ADD_CONTENT)
        expected_document(child)
        server.finish()
        require(child.is_dir() and git(child, "branch", "--show-current") == "stdio-pending", "shutdown removed managed worktree")
        require(git(child, "rev-parse", "HEAD") == head and git(child, "write-tree") == index, "shutdown committed or changed index")
        expected_document(child)
        # Original context is still clean; ensure ownership was actually released.
        with Stdio(binary, root, folder / "reacquired", "--enable-mutations"):
            pass


def help_case(binary, folder):
    for args in ([], ["mcp", "--help"]):
        result = subprocess.run([binary, *args], capture_output=True, text=True, timeout=10)
        require(result.returncode == 0 and all(s in result.stdout for s in
                ("--transport", "http", "stdio", "--host", "--port", "--allow-origin", "--allow-host")),
                "help omits transport selection or HTTP options")
        transport = next(line for line in result.stdout.splitlines() if "--transport" in line)
        require("http" in transport and "stdio" in transport and "default" in transport.lower(),
                "help omits transport choices or HTTP default: " + transport)
        for flag in ("--host", "--port", "--allow-origin", "--allow-host"):
            # Root help also includes SERVE options; inspect the MCP section.
            scope = result.stdout.split("MCP OPTIONS:", 1)[-1].split("MIGRATE OPTIONS:", 1)[0]
            line = next(line for line in scope.splitlines() if flag in line)
            require("HTTP-only" in line, "help does not identify HTTP-only option: " + line)


def argv_case(binary, folder, flag, value):
    root = project(folder / "invalid", invalid=True)
    rejected(binary, root, folder, flag, value, diagnostic=flag)


def unknown_transport(binary, folder):
    root = project(folder / "invalid", invalid=True)
    result = subprocess.run([binary, "--workspace", str(root), "mcp", "--transport", "unknown"],
                            capture_output=True, text=True, timeout=10)
    require(result.returncode != 0 and "unknown" in result.stderr and "stdio" in result.stderr
            and "http" in result.stderr and not result.stdout, "transport enum not validated: " + result.stderr)


def npm_case(binary, folder):
    node = shutil.which("node")
    require(node is not None, "Node is required for the distributed npm launcher check")
    package = folder / "npm"
    (package / "bin").mkdir(parents=True)
    (package / "artifacts").mkdir()
    shutil.copy2(REPO / "npm/reqvire/bin/reqvire.js", package / "bin/reqvire.js")
    metadata = json.loads((REPO / "npm/reqvire/package.json").read_text())
    # A private cache identity prevents reuse of an unrelated installed binary.
    metadata["version"] += "-stdio-e2e-" + folder.name + "-" + str(os.getpid())
    (package / "package.json").write_text(json.dumps(metadata))
    target = {("linux", "x86_64"): "linux-x86_64", ("darwin", "arm64"): "darwin-arm64",
              ("darwin", "x86_64"): "darwin-x86_64"}.get((sys.platform, os.uname().machine))
    require(target is not None, "npm launcher does not support this platform")
    with tarfile.open(package / f"artifacts/reqvire-{target}.tar.gz", "w:gz", compresslevel=1) as archive:
        archive.add(binary, arcname=f"reqvire-{target}")
    launcher = [node, str(package / "bin/reqvire.js")]
    root = project(folder / "project")
    with Stdio(binary, root, folder / "client", launcher=launcher) as server:
        require("Stdio Requirement A" in json.dumps(call(server, "reqvire.search")), "npm selected wrong project")
    invalid = project(folder / "invalid", invalid=True)
    native = rejected(binary, invalid, folder, diagnostic="validation")
    wrapped = rejected(binary, invalid, folder, diagnostic="validation", launcher=launcher)
    require(native == wrapped, "npm launcher changed startup failure exit status")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("binary")
    parser.add_argument("destination", type=pathlib.Path)
    parser.add_argument("--only", action="append", default=[])
    args = parser.parse_args()
    binary = str(pathlib.Path(args.binary).resolve())
    output = args.destination / "output/transports"
    output.mkdir(parents=True, exist_ok=True)
    cases = [("default-http", default_http), ("default-http-mutation-fixture", default_http_mutation_fixture),
             ("explicit-http", explicit_http),
             ("stdio-parity", stdio_parity), ("stdio-permissions-errors", permissions_errors),
             ("stdio-project-isolation", project_isolation), ("stdio-mutation", mutation),
             ("stdio-size-estimates", estimates), ("stdio-owned-context", ownership),
             ("stdio-eof-reacquisition", lifecycle),
             ("stdio-sigint-reacquisition", lambda b, f: lifecycle(b, f, signal.SIGINT)),
             ("stdio-sigterm-reacquisition", lambda b, f: lifecycle(b, f, signal.SIGTERM)),
             ("stdio-pending-worktree", preserved_worktree), ("transport-help", help_case),
             ("unknown-transport", unknown_transport),
             ("stdio-invalid-model", lambda b, f: rejected(b, project(f / "invalid", invalid=True), f, diagnostic="validation")),
             ("stdio-missing-workspace", lambda b, f: rejected(b, f / "missing", f, diagnostic="workspace")),
             ("stdio-dirty-context", lambda b, f: dirty_case(b, f)),
             ("npm-stdio", npm_case)]
    for flag, value in (("--host", "127.0.0.1"), ("--port", "8081"),
                        ("--allow-origin", "https://client.example"), ("--allow-host", "mcp.example")):
        cases.append(("stdio-reject-" + flag[2:], lambda b, f, flag=flag, value=value: argv_case(b, f, flag, value)))
    require(not set(args.only) - {name for name, _ in cases}, "unknown --only case")
    results = []
    for name, run in cases:
        if args.only and name not in args.only:
            continue
        folder = output / name
        folder.mkdir()
        started = time.monotonic()
        try:
            run(binary, folder)
            result = {"case": name, "status": "pass"}
        except Exception as error:
            result = {"case": name, "status": "fail", "detail": str(error)}
            print(f"FAIL {name}: {error}", file=sys.stderr, flush=True)
        result["duration_seconds"] = round(time.monotonic() - started, 3)
        results.append(result)
    report = {"binary": binary, "protocol_version": PROTOCOL,
              "passed": sum(r["status"] == "pass" for r in results),
              "failed": sum(r["status"] == "fail" for r in results), "cases": results}
    (output / "results.json").write_text(json.dumps(report, indent=2) + "\n")
    if report["failed"]:
        print(f"Transport cases: {report['passed']} passed, {report['failed']} failed; {output / 'results.json'}", file=sys.stderr)
    return int(report["failed"] != 0)


def dirty_case(binary, folder):
    root = project(folder / "project")
    (root / "Model.md").write_text((root / "Model.md").read_text() + "\nUncommitted local edit.\n")
    rejected(binary, root, folder, "--enable-mutations", diagnostic="clean")


if __name__ == "__main__":
    raise SystemExit(main())
