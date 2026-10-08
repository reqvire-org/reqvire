import contextlib
import http.client
import json
import pathlib
import socket
import subprocess
import sys
import time

binary, workspace = sys.argv[1:]
root = pathlib.Path(workspace)
origins = ["https://app.example", "http://192.0.2.10:3000"]
initialize = {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
    "protocolVersion": "2025-11-25", "capabilities": {},
    "clientInfo": {"name": "cors-test", "version": "1"}}}


def request(port, method="POST", origin=None, payload=initialize, path="/mcp", extra=None, connect_host="127.0.0.1"):
    headers = {"Content-Type": "application/json", "Accept": "application/json, text/event-stream",
               "Mcp-Protocol-Version": "2025-11-25"}
    if origin is not None:
        headers["Origin"] = origin
    headers.update(extra or {})
    conn = http.client.HTTPConnection(connect_host, port, timeout=3)
    try:
        conn.request(method, path, json.dumps(payload) if method == "POST" else None, headers)
        response = conn.getresponse()
        return response.status, dict((k.lower(), v) for k, v in response.getheaders()), response.read()
    finally:
        conn.close()


@contextlib.contextmanager
def server(command, configured, bind="127.0.0.1", hosts=()):
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        port = sock.getsockname()[1]
    args = [binary, command, "--host", bind, "--port", str(port), "--enable-mutations"]
    if command == "serve":
        args += ["--enable-mcp"]
    if configured:
        for origin in origins:
            args += ["--allow-origin", origin]
    for host in hosts:
        args += ["--allow-host", host]
    connect_host = "127.0.0.1" if bind == "0.0.0.0" else bind
    with (root / "output" / f"{command}-{configured}-{bind}.log").open("w") as log:
        process = subprocess.Popen(args, cwd=root, stdout=log, stderr=log)
        try:
            for _ in range(100):
                assert process.poll() is None, f"{command} exited; see {log.name}"
                try:
                    if request(port, connect_host=connect_host)[0] == 200:
                        break
                except OSError:
                    pass
                time.sleep(0.05)
            else:
                raise AssertionError(f"{command} did not start; see {log.name}")
            yield port
        finally:
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()


def check_allowed(port, origin):
    status, headers, body = request(port, origin=origin)
    assert status == 200, (origin, status, body)
    assert json.loads(body)["result"]["protocolVersion"] == "2025-11-25"
    if origin is not None:
        assert headers["access-control-allow-origin"] == origin
        assert "origin" in headers["vary"].lower()
        assert "mcp-session-id" in headers["access-control-expose-headers"].lower()
    return headers


def main():
    for command in ("mcp", "serve"):
        help_result = subprocess.run([binary, command, "--help"], capture_output=True, text=True)
        assert "--allow-origin" in help_result.stdout and "repeatable" in help_result.stdout.lower()
        for invalid in ("*", "null", "file:///tmp/client", "ftp://app.example", "https://user@app.example",
                        "https://app.example/path", "https://app.example/", "https://app.example?q=1",
                        "https://app.example#x", "app.example", "https://app.example:bad", "https://app.example:65536"):
            args = [binary, command, "--allow-origin", invalid]
            if command == "serve":
                args += ["--enable-mcp"]
            result = subprocess.run(args, cwd=root, capture_output=True, text=True, timeout=5)
            assert result.returncode == 2 and "invalid value" in result.stderr, (args, result.stderr)
    result = subprocess.run([binary, "serve", "--allow-origin", origins[0]], capture_output=True, text=True)
    assert result.returncode == 2 and "--enable-mcp" in result.stderr
    print("PASS cli-help-and-invalid-origins", flush=True)

    for command in ("mcp", "serve"):
        help_result = subprocess.run([binary, command, "--help"], capture_output=True, text=True)
        assert "--allow-host" in help_result.stdout, f"missing --allow-host in {command} help"
        for invalid in ("*", "*.example", "0.0.0.0", "[::]", "https://mcp.example", "user@mcp.example",
                        "mcp.example/path", "mcp.example?x=1", "mcp.example#x", "mcp.example:bad", "mcp.example:65536", "mcp..example", "-mcp.example", "mcp_.example"):
            args = [binary, command, f"--allow-host={invalid}"]
            if command == "serve":
                args += ["--enable-mcp"]
            result = subprocess.run(args, cwd=root, capture_output=True, text=True, timeout=5)
            assert result.returncode == 2 and "invalid value" in result.stderr, (args, result.stderr)
    result = subprocess.run([binary, "serve", "--allow-host", "mcp.example"], capture_output=True, text=True)
    assert result.returncode == 2 and "--enable-mcp" in result.stderr
    print("PASS cli-help-and-invalid-hosts", flush=True)

    for command in ("mcp", "serve"):
        with server(command, False) as port:
            for origin in (None, "http://localhost:5173", "https://127.0.0.1:8443", "http://[::1]:9000"):
                check_allowed(port, origin)
            assert request(port, origin=origins[0])[0] == 403
            print(f"PASS {command}-default-origins", flush=True)
        with server(command, True) as port:
            for origin in (None, "http://localhost:5173", *origins, "https://app.example:443"):
                check_allowed(port, origin)
            for origin in ("https://other.example", "http://app.example", "https://app.example:444",
                           "https://sub.app.example", "http://192.0.2.10:3001", "null", "file:///tmp/client",
                           "https://app.example/path", "https://app.example https://other.example"):
                for method in ("POST", "OPTIONS", "GET", "DELETE"):
                    status, headers, _ = request(port, method, origin)
                    assert status == 403 and "access-control-allow-origin" not in headers, (method, origin, status)
            for origin in origins:
                status, headers, _ = request(port, "OPTIONS", origin, extra={
                    "Access-Control-Request-Method": "POST",
                    "Access-Control-Request-Headers": "content-type,authorization,mcp-protocol-version,mcp-session-id,last-event-id"})
                assert status == 200 and headers["access-control-allow-origin"] == origin
                assert "POST" in headers["access-control-allow-methods"]
                for name in ("content-type", "authorization", "mcp-protocol-version", "mcp-session-id", "last-event-id"):
                    assert name in headers["access-control-allow-headers"].lower()
                status, headers, _ = request(port, origin=origin, extra={"Mcp-Protocol-Version": "unsupported"})
                assert status == 400 and headers["access-control-allow-origin"] == origin
                assert request(port, "GET", origin)[0] == 405
                assert request(port, "DELETE", origin)[0] == 405
                tools_call = {"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
                status, _, body = request(port, origin=origin, payload=tools_call)
                assert status == 200 and any(t["name"] == "reqvire.search" for t in json.loads(body)["result"]["tools"])
            print(f"PASS {command}-configured-origins-and-preflight", flush=True)
            source = root / "specifications" / "Model.md"
            before = source.read_bytes()
            mutation = {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {
                "name": "reqvire.add_element", "arguments": {"file": "specifications/Model.md",
                "content": "### Unexpected Requirement\n\nThe system SHALL reject this write.\n\n#### Metadata\n  * type: requirement\n\n#### Relations\n  * derivedFrom: [CORS Requirement](#cors-requirement)\n"}}}
            assert request(port, origin="https://other.example", payload=mutation)[0] == 403
            assert source.read_bytes() == before
            print(f"PASS {command}-rejected-mutation", flush=True)
            if command == "serve":
                status, headers, body = request(port, "GET", origins[0], path="/")
                assert status == 200 and b"<html" in body.lower()
                assert "access-control-allow-origin" not in headers
                print("PASS explorer-route-isolation", flush=True)

    for command in ("mcp", "serve"):
        with server(command, False, bind="127.0.0.2") as port:
            status, _, body = request(port, connect_host="127.0.0.2")
            assert status == 200 and "result" in json.loads(body)
            status, _, _ = request(port, connect_host="127.0.0.2", extra={"Host": f"127.0.0.2:{port + 1}"})
            assert status == 403
            print(f"PASS {command}-explicit-bind-host", flush=True)
        with server(command, False, bind="0.0.0.0", hosts=("mcp.example", "restricted.example:8443", "192.0.2.51:8081")) as port:
            for host in ("mcp.example", "MCP.EXAMPLE:443", "restricted.example:8443", "192.0.2.51:8081"):
                status, _, body = request(port, extra={"Host": host})
                assert status == 200 and "result" in json.loads(body), (host, status, body)
                status, _, body = request(port, extra={"Host": host}, payload={"jsonrpc": "2.0", "id": 4, "method": "tools/list", "params": {}})
                assert status == 200 and "tools" in json.loads(body)["result"]
            for host in ("other.example", "sub.mcp.example", "restricted.example:8444", "192.0.2.51:8082"):
                status, _, _ = request(port, extra={"Host": host, "X-Forwarded-Host": "mcp.example", "Forwarded": "host=mcp.example"})
                assert status == 403, (host, status)
            assert request(port, origin="https://mcp.example", extra={"Host": "mcp.example"})[0] == 403
            source = root / "specifications" / "Model.md"
            before = source.read_bytes()
            assert request(port, payload=mutation, extra={"Host": "unlisted.example"})[0] == 403
            assert source.read_bytes() == before
            print(f"PASS {command}-public-hosts-and-rejected-mutation", flush=True)


try:
    main()
except Exception as error:
    print(f"FAIL {type(error).__name__}: {error}", flush=True)
    sys.exit(1)
