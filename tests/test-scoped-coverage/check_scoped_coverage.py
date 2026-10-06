#!/usr/bin/env python3
"""Black-box CLI/MCP checks for the shared coverage scope contract."""

import argparse
from collections import Counter
from copy import deepcopy
import difflib
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import re
import shutil
import signal
import socket
import subprocess
import sys
import time
from threading import Thread
import urllib.error
import urllib.request

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from stop_test_processes import stop as stop_test_processes


VERIFICATION_KINDS = {
    "shared-check": "test",
    "gap-check": "test",
    "parent-review": "inspection",
    "shared-proof": "formal_proof",
    "middle-demonstration": "demonstration",
    "beta-analysis": "analysis",
    "orphan-check": "test",
}
REQUIREMENT_SECTIONS = (
    "verified_leaf_requirements", "unverified_leaf_requirements",
    "covered_requirements", "uncovered_requirements",
)
VERIFICATION_SECTIONS = (
    "satisfied_test_verifications", "unsatisfied_test_verifications",
    "orphaned_verifications",
)
REPORT_FIELDS = (*REQUIREMENT_SECTIONS, *VERIFICATION_SECTIONS,
                 "summary", "capability_coverage")
INVALID_SCOPES = (
    "Missing Capability", "Alpha Local", "Exchange Format", "Shared Check", "alpha root",
)


def normalized(value):
    if isinstance(value, dict):
        return {key: normalized(item) for key, item in sorted(value.items())}
    if isinstance(value, list):
        return sorted((normalized(item) for item in value), key=lambda item: json.dumps(item, sort_keys=True))
    return value


def equal(actual, expected, label):
    if normalized(actual) != normalized(expected):
        diff = difflib.unified_diff(
            json.dumps(normalized(expected), indent=2).splitlines(),
            json.dumps(normalized(actual), indent=2).splitlines(),
            fromfile="expected", tofile="actual", lineterm="",
        )
        raise AssertionError(label + "\n" + "\n".join(list(diff)[:45]))


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def run_browser(command, output, timeout=90):
    stdout_path, stderr_path = output / "browser.stdout", output / "browser.stderr"
    timed_out = False
    # Persist while running: a timeout must not discard progress or diagnostics.
    with stdout_path.open("w") as stdout, stderr_path.open("w") as stderr:
        process = subprocess.Popen(command, stdout=stdout, stderr=stderr, start_new_session=True)
        try:
            process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            timed_out = True
        finally:
            # Also catch browser children left by an unexpectedly exited driver.
            stop_test_processes(process.pid)
            process.wait(timeout=5)
    stdout, stderr = stdout_path.read_text(), stderr_path.read_text()
    diagnostics = f"{stdout[-4000:]}\n{stderr[-6000:]}"
    require(not timed_out, f"Browser timed out after {timeout}s: {diagnostics}")
    require(process.returncode == 0, f"Browser failed: {diagnostics}")
    return stdout


def records(report, section):
    return [entry for entries in report[section]["files"].values() for entry in entries]


def selected_section(report, section, identifiers):
    return {"files": {
        path: chosen for path, entries in report[section]["files"].items()
        if (chosen := [entry for entry in entries if entry["identifier"] in identifiers])
    }}


def percentage(numerator, denominator):
    return round(100 * numerator / denominator, 2) if denominator else 0.0


def expected_projection(whole, scope):
    """Select whole-model evidence using authored memberships, never graph traversal."""
    projected = {"scope": scope}
    for section in REQUIREMENT_SECTIONS:
        projected[section] = selected_section(whole, section, scope["requirement_ids"])
    for section in VERIFICATION_SECTIONS:
        projected[section] = selected_section(whole, section, scope["verification_ids"])
    projected["capability_coverage"] = {"capabilities": [
        entry for entry in whole["capability_coverage"]["capabilities"]
        if entry["identifier"] in scope["capability_ids"]
    ]}
    verified = len(records(projected, "verified_leaf_requirements"))
    unverified = len(records(projected, "unverified_leaf_requirements"))
    satisfied = len(records(projected, "satisfied_test_verifications"))
    unsatisfied = len(records(projected, "unsatisfied_test_verifications"))
    covered = records(projected, "covered_requirements")
    uncovered = records(projected, "uncovered_requirements")
    covered_terminal = sum(entry["is_terminal"] for entry in covered)
    uncovered_terminal = sum(entry["is_terminal"] for entry in uncovered)
    terminal = covered_terminal + uncovered_terminal
    kinds = Counter(VERIFICATION_KINDS[identifier.split("#")[1]] for identifier in scope["verification_ids"])
    sources = Counter(entry["coverage_source"] for entry in covered)
    total = len(scope["requirement_ids"])
    projected["summary"] = {
        "total_leaf_requirements": verified + unverified,
        "verified_leaf_requirements": verified,
        "unverified_leaf_requirements": unverified,
        "leaf_requirements_coverage_percentage": percentage(verified, verified + unverified),
        "total_test_verifications": satisfied + unsatisfied,
        "satisfied_test_verifications": satisfied,
        "unsatisfied_test_verifications": unsatisfied,
        "test_verifications_satisfaction_percentage": percentage(satisfied, satisfied + unsatisfied),
        "total_verifications": len(scope["verification_ids"]),
        "orphaned_verifications": 0,
        "orphaned_verifications_percentage": 0.0,
        "verification_types": {kind: kinds[kind] for kind in ("test", "formal_proof", "analysis", "inspection", "demonstration")},
        "total_requirements_in_scope": total,
        "covered_requirements": len(covered),
        "uncovered_requirements": total - len(covered),
        "total_terminal_requirements": terminal,
        "covered_terminal_requirements": covered_terminal,
        "uncovered_terminal_requirements": uncovered_terminal,
        "implementation_coverage_percentage": percentage(covered_terminal, terminal),
        "coverage_sources": {source: sources[source] for source in (
            "direct_satisfied", "requirement_rollup", "contract_consumer_rollup", "combined_rollup",
        )},
    }
    return projected


def assert_projection(actual, expected):
    # Check original arrays before canonicalizing unordered report records.
    require(actual.get("scope") == expected["scope"], "Missing or incorrect scope metadata / sorted distinct memberships")
    for field in REPORT_FIELDS:
        equal(actual[field], expected[field], field)
    for section in (*REQUIREMENT_SECTIONS, *VERIFICATION_SECTIONS):
        identifiers = [entry["identifier"] for entry in records(actual, section)]
        require(len(identifiers) == len(set(identifiers)), f"Duplicate records in {section}")


class McpServer:
    """Use the repository's HTTP MCP request/session pattern with bounded waits."""

    def __init__(self, binary, workspace, output, command="mcp", mutations=False):
        self.output = output
        self.log = (output / f"{command}-server.log").open("w")
        self.command = command
        self.session = None
        self.request_id = 0
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            port = listener.getsockname()[1]
        self.base_url = f"http://127.0.0.1:{port}"
        self.url = self.base_url + "/mcp"
        self.process = subprocess.Popen(
            [binary, command, "--host", "127.0.0.1", "--port", str(port), *(["--enable-mcp"] if command == "serve" else []), *(["--enable-mutations"] if mutations else [])],
            cwd=workspace, stdout=self.log, stderr=subprocess.STDOUT, start_new_session=True,
        )

    def __enter__(self):
        try:
            deadline = time.monotonic() + 15
            while time.monotonic() < deadline:
                require(self.process.poll() is None, "MCP exited during startup; see mcp-server.log")
                try:
                    response = self.rpc("initialize", {
                        "protocolVersion": "2025-11-25", "capabilities": {},
                        "clientInfo": {"name": "scoped-coverage-e2e", "version": "1"},
                    })
                    require(response["result"]["protocolVersion"] == "2025-11-25", "MCP initialization failed")
                    return self
                except (urllib.error.URLError, TimeoutError, ConnectionError):
                    time.sleep(0.1)
            raise RuntimeError("MCP startup timed out")
        except BaseException:
            self.__exit__(None, None, None)
            raise

    def __exit__(self, *_):
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
            "Content-Type": "application/json", "Accept": "application/json, text/event-stream",
            "Mcp-Protocol-Version": "2025-11-25",
        }
        if self.session:
            headers["Mcp-Session-Id"] = self.session
        with urllib.request.urlopen(urllib.request.Request(self.url, json.dumps(request).encode(), headers), timeout=5) as response:
            self.session = response.headers.get("Mcp-Session-Id", self.session)
            payload = json.loads(response.read())
        (self.output / f"{self.command}-{self.request_id:03}.json").write_text(json.dumps({"request": request, "response": payload}, indent=2) + "\n")
        require(payload.get("id") == self.request_id, "MCP response ID mismatch")
        return payload

    def tool(self, name, **arguments):
        response = self.rpc("tools/call", {"name": name, "arguments": arguments})
        result = response.get("result", {})
        require(not response.get("error") and not result.get("isError") and "structuredContent" in result,
                f"{name} failed: {json.dumps(response)}")
        return result["structuredContent"]


def main():
    parser = argparse.ArgumentParser()
    for name in ("binary", "workspace", "expected", "fixtures"):
        parser.add_argument("--" + name, required=True)
    args = parser.parse_args()
    browser_checks_mode = os.environ.get("REQVIRE_COVERAGE_BROWSER_CHECKS", "navigation")
    require(browser_checks_mode in ("all", "navigation"), "REQVIRE_COVERAGE_BROWSER_CHECKS must be all or navigation")
    root, expected, fixtures = map(Path, (args.workspace, args.expected, args.fixtures))
    output = root / "output"
    scopes = json.loads((expected / "scopes.json").read_text())
    failures = 0
    outcomes = []

    def check(name, action):
        nonlocal failures
        try:
            action()
            outcomes.append("PASS " + name)
        except Exception as error:
            failures += 1
            outcomes.append("FAIL " + name)
            print(f"FAILED {name}: {error}")

    def cli(label, *arguments):
        result = subprocess.run([args.binary, *arguments], cwd=root, text=True, capture_output=True, timeout=20)
        (output / (label + ".stdout")).write_text(result.stdout)
        (output / (label + ".stderr")).write_text(result.stderr)
        return result

    def cli_report(label, scope=None):
        result = cli(label, "coverage", *(["--from", scope] if scope is not None else []), "--json")
        require(result.returncode == 0, f"coverage exited {result.returncode}: {result.stderr.strip()}")
        return json.loads(result.stdout)

    def mcp_report(server, scope=None):
        result = server.tool("reqvire.coverage", **({"from": scope} if scope is not None else {}))
        return {key: result[key] for key in (*REPORT_FIELDS, "scope") if key in result}

    def text_scope(name, wanted):
        result = cli("cli-text-" + name.replace(" ", "-"), "coverage", "--from", name)
        require(result.returncode == 0, f"scoped text exited {result.returncode}: {result.stderr.strip()}")
        text = result.stdout
        require(re.search(r"scope[^\n]*" + re.escape(name), text, re.IGNORECASE), "Text does not identify selected scope")
        require(re.search(r"orphan[^\n]*whole[ -]model|whole[ -]model[^\n]*orphan", text, re.IGNORECASE), "Text lacks whole-model-only orphan explanation")
        labels = {
            "Total Leaf Requirements": "total_leaf_requirements", "Verified Leaf Requirements": "verified_leaf_requirements",
            "Unverified Leaf Requirements": "unverified_leaf_requirements", "Total Test Verifications": "total_test_verifications",
            "Satisfied Test Verifications": "satisfied_test_verifications", "Unsatisfied Test Verifications": "unsatisfied_test_verifications",
            "Total Verifications": "total_verifications", "Total Requirements in Scope": "total_requirements_in_scope",
            "Covered Requirements": "covered_requirements", "Uncovered Requirements": "uncovered_requirements",
            "Total Terminal Requirements": "total_terminal_requirements", "Covered Terminal Requirements": "covered_terminal_requirements",
            "Uncovered Terminal Requirements": "uncovered_terminal_requirements",
        }
        for label, key in labels.items():
            match = re.search(r"\*\*" + re.escape(label) + r":\*\*\s*(\d+)", text)
            require(match and int(match[1]) == wanted["summary"][key], f"Text/JSON mismatch: {label}")
        for key, value in wanted["summary"]["coverage_sources"].items():
            require(re.search(r"(?m)^- " + key + rf": {value}$", text), f"Text source count differs: {key}")
        for key, value in wanted["summary"]["verification_types"].items():
            label = key.replace("_", " ").title()
            require(re.search(r"(?m)^- " + label + rf": {value}$", text), f"Text verification count differs: {key}")
        for label, key in (
            ("Verified Leaf Requirements", "leaf_requirements_coverage_percentage"),
            ("Satisfied Test Verifications", "test_verifications_satisfaction_percentage"),
            ("Covered Terminal Requirements", "implementation_coverage_percentage"),
        ):
            match = re.search(r"\*\*" + label + r":\*\* \d+ \(([\d.]+)%\)", text)
            require(match and match[1] == f"{wanted['summary'][key]:.1f}", f"Text percentage differs: {label}")
        for section in (*REQUIREMENT_SECTIONS, *VERIFICATION_SECTIONS):
            heading = section.replace("_", " ").title()
            match = re.search(r"(?m)^## " + heading + r"\n([\s\S]*?)(?=^## |\Z)", text)
            body = match[1] if match else ""
            blocks = list(re.finditer(r"(?m)^- [^\n]*?\*\*\[([^\]]+)\]\(([^)]+)\)\*\*([^\n]*)\n([\s\S]*?)(?=^- |^## |\Z)", body))
            expected_records = records(wanted, section)
            equal([block[2] for block in blocks], [entry["identifier"] for entry in expected_records], f"Text subjects differ in {section}")
            by_id = {entry["identifier"]: entry for entry in expected_records}
            for block in blocks:
                entry = by_id[block[2]]
                for field in ("coverage_source", "verification_type"):
                    if field in entry:
                        require(f"({entry[field]})" in block[3], f"Text {field} differs for {entry['name']}")
                evidence = re.findall(r"(?m)^\s+- \[[^\]]+\]\(([^)]+)\)$", block[4])
                equal(evidence, entry.get("direct_evidence", []) + entry.get("evidence", [])
                      + entry.get("contributing_requirements", []) + entry.get("blocking_requirements", [])
                      + entry.get("verified_by", []) + entry.get("satisfied_by", []), f"Text evidence differs for {entry['name']}")

    def cli_invalid(name):
        result = cli("cli-invalid-" + name.replace(" ", "-"), "coverage", "--from", name, "--json")
        message = result.stdout + result.stderr
        require(result.returncode != 0, "Invalid selection silently succeeded")
        require(name in message and "unexpected argument" not in message.lower(), "Expected a selector diagnostic, not an unsupported-option error")
        require("capabilit" in message.lower() or "not found" in message.lower(), "Diagnostic does not explain invalid selection")
        require('"summary"' not in result.stdout, "Invalid selection returned a report")

    def mcp_invalid(server, name):
        response = server.rpc("tools/call", {"name": "reqvire.coverage", "arguments": {"from": name}})
        result = response.get("result", {})
        message = json.dumps(result)
        require(result.get("isError") is True, f"Expected semantic tool error, received {json.dumps(response)}")
        require(name in message and ("capabilit" in message.lower() or "not found" in message.lower()), "Missing selector-specific error")
        require("summary" not in result.get("structuredContent", {}), "Invalid selection returned coverage")

    def help_check():
        result = cli("help", "coverage", "--help")
        require(result.returncode == 0 and "--from" in result.stdout and "capabilit" in result.stdout.lower(), "Help does not advertise capability scope")

    validation = cli("validate", "validate", "--json")
    check("fixture-valid", lambda: require(validation.returncode == 0 and json.loads(validation.stdout)["errors"] == [], "Invalid test fixture"))
    try:
        whole = cli_report("whole-model")
        check("cli-whole-model-json", lambda: equal(whole, json.loads((expected / "whole-model.json").read_text()), "Whole-model output changed"))
        check("cli-whole-model-text", lambda: equal(cli("whole-text", "coverage").stdout, (expected / "whole-model.txt").read_text(), "Whole-model text changed"))
        check("cli-help", help_check)
        for name, scope in scopes.items():
            wanted = expected_projection(whole, scope)
            check("cli-json-" + name, lambda name=name, wanted=wanted: assert_projection(cli_report("cli-" + name.replace(" ", "-"), name), wanted))
            check("cli-text-" + name, lambda name=name, wanted=wanted: text_scope(name, wanted))
        for name in INVALID_SCOPES:
            check("cli-invalid-" + name, lambda name=name: cli_invalid(name))

        def inspect_store(javascript):
            store = json.JSONDecoder().raw_decode(javascript.split("=", 1)[1].lstrip())[0]
            index = store["coverage"].get("scope_index", {})
            equal(sorted(index), sorted(scope["capability_identifier"] for scope in scopes.values()), "Store scope index omits or adds subjects")
            for name, scope in scopes.items():
                report = cli_report("store-parity-" + name.replace(" ", "-"), name)
                equal(index[scope["capability_identifier"]], {"scope": report["scope"], "summary": report["summary"]}, "Store/CLI scope mismatch")
            return store

        def exported_store():
            result = cli("export", "export", "--output", str(output / "site"))
            require(result.returncode == 0, result.stderr)
            inspect_store((output / "site/assets/project-store.js").read_text())

        check("exported-store-scope-index", exported_store)
        with McpServer(args.binary, root, output, "serve", mutations=True) as served:
            def served_store():
                with urllib.request.urlopen(served.base_url + "/assets/project-store.js", timeout=5) as response:
                    inspect_store(response.read().decode())
            check("served-store-scope-index", served_store)

            def browser_checks():
                reports = {"whole": whole, **{name: expected_projection(whole, scope) for name, scope in scopes.items()}}
                (output / "browser-reports.json").write_text(json.dumps(reports))
                class QuietHandler(SimpleHTTPRequestHandler):
                    def log_message(self, *_):
                        pass
                http = ThreadingHTTPServer(("127.0.0.1", 0), partial(QuietHandler, directory=str(output / "site")))
                thread = Thread(target=http.serve_forever, daemon=True)
                thread.start()
                originals = {path: path.read_bytes() for path in (root / "specifications").glob("*.md")}
                other = None
                try:
                    drivers = [("browser-navigation.mjs", "browser-navigation.txt", str(output / "browser-contexts.json"))]
                    if browser_checks_mode == "all":
                        drivers.insert(0, ("browser-check.mjs", "browser.txt", str(fixtures / "Added.md")))
                    for driver, expected_file, extra in drivers:
                        if driver == "browser-navigation.mjs":
                            inventory = served.tool("reqvire.worktree.list")
                            original = next(row for row in inventory["worktrees"] if row.get("original"))
                            other = served.tool("reqvire.worktree.create", branch="coverage-links", base_ref="HEAD")
                            (output / "browser-contexts.json").write_text(json.dumps({
                                "original": original["worktree_id"], "other": other["worktree_id"],
                                "roots": {original["worktree_id"]: original["workspace_root"], other["worktree_id"]: other["workspace_root"]},
                            }))
                        driver_output = output / Path(driver).stem
                        driver_output.mkdir(exist_ok=True)
                        try:
                            browser_stdout = run_browser(["node", str(Path(__file__).with_name(driver)),
                                served.base_url, f"http://127.0.0.1:{http.server_port}", str(driver_output / "profile"),
                                str(output / "browser-reports.json"), extra], driver_output)
                            equal(browser_stdout, (expected / expected_file).read_text(), "Browser check outcomes")
                        finally:
                            for path in (root / "specifications").glob("*.md"):
                                if path not in originals:
                                    path.unlink()
                            for path, content in originals.items():
                                path.write_bytes(content)
                finally:
                    http.shutdown()
                    http.server_close()
                    thread.join(timeout=5)
                    for path in (root / "specifications").glob("*.md"):
                        if path not in originals:
                            path.unlink()
                    for path, content in originals.items():
                        path.write_bytes(content)
                    if other is not None:
                        served.tool("reqvire.worktree.remove", worktree_id=other["worktree_id"])
            check("browser-served-exported-and-live-coverage", browser_checks)

        with McpServer(args.binary, root, output) as server:
            def schema_check():
                tools = server.rpc("tools/list", {})["result"]["tools"]
                schema = next(tool["inputSchema"] for tool in tools if tool["name"] == "reqvire.coverage")
                require(schema.get("properties", {}).get("from", {}).get("type") == "string", "Missing string from argument")
                require("from" not in schema.get("required", []), "Scope must remain optional")

            check("mcp-discovery", schema_check)
            check("mcp-whole-model", lambda: equal(mcp_report(server), whole, "MCP/CLI whole-model mismatch"))
            for name, scope in scopes.items():
                wanted = expected_projection(whole, scope)
                check("mcp-scope-" + name, lambda name=name, wanted=wanted: assert_projection(mcp_report(server, name), wanted))
            for name in INVALID_SCOPES:
                check("mcp-invalid-" + name, lambda name=name: mcp_invalid(server, name))

            def repeat_check():
                first = mcp_report(server, "Alpha Root")
                assert_projection(first, expected_projection(whole, scopes["Alpha Root"]))
                equal(mcp_report(server, "Alpha Root"), first, "Repeated scoped report changed")
                equal(cli_report("repeat-cli", "Alpha Root"), first, "MCP/CLI scoped report mismatch")

            check("repeated-scope-and-cli-mcp-parity", repeat_check)
            before_revision = server.tool("reqvire.model_revision")["model_fingerprint"]
            beta_file = root / "specifications/Beta.md"
            original_beta = beta_file.read_bytes()
            added_file = root / "specifications/Added.md"
            try:
                shutil.copyfile(fixtures / "BetaWithoutEvidence.md", beta_file)
                shutil.copyfile(fixtures / "Added.md", added_file)
                changed_whole = cli_report("changed-whole")
                check("mcp-revision-after-external-edit", lambda: require(server.tool("reqvire.model_revision")["model_fingerprint"] != before_revision, "Revision did not change"))
                check("mcp-whole-model-after-external-edit", lambda: equal(mcp_report(server), changed_whole, "Persistent MCP retained stale whole-model data"))
                owner = "specifications/Alpha.md#alpha-contract-owner"
                check("external-evidence-removed", lambda: require(owner in {entry["identifier"] for entry in records(changed_whole, "uncovered_requirements")}, "Owner retained removed consumer evidence"))
                for name in ("Alpha Root", "Shared Branch"):
                    scope = deepcopy(scopes[name])
                    scope["requirement_ids"] = sorted(scope["requirement_ids"] + ["specifications/Added.md#added-alpha-gap"])
                    wanted = expected_projection(changed_whole, scope)
                    check("cli-after-edit-" + name, lambda name=name, wanted=wanted: assert_projection(cli_report("changed-cli-" + name.replace(" ", "-"), name), wanted))
                    check("mcp-after-edit-" + name, lambda name=name, wanted=wanted: assert_projection(mcp_report(server, name), wanted))
            finally:
                beta_file.write_bytes(original_beta)
                added_file.unlink(missing_ok=True)
    except Exception as error:
        failures += 1
        outcomes.append("FAIL infrastructure")
        print(f"FAILED infrastructure: {error}")
    finally:
        (output / "checks.txt").write_text("\n".join(outcomes) + "\n")
    return bool(failures)


if __name__ == "__main__":
    raise SystemExit(main())
