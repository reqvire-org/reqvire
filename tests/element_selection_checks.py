#!/usr/bin/env python3
"""Shared black-box assertions, invoked by the existing owning E2E suites.

All expected checks are PASS. Identifier/ambiguity failures remain real failures;
there is no expected-failure mode. Only temporary fixture repositories mutate.
"""

import argparse
from dataclasses import dataclass
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import traceback


TESTS = Path(__file__).resolve().parent


def require(condition, detail):
    if not condition:
        raise AssertionError(detail)


def identifier(name):
    # These fixture identifiers are authored expectations, not resolver output.
    if name in ("Item Lookup", "Item Ontology"):
        return "Queries.md#" + name.lower().replace(" ", "-")
    if name in ("Sparse Concept Scheme", "Detailed Concept"):
        return "Concepts.md#" + name.lower().replace(" ", "-")
    if name == "Cross-file Subject":
        return "nested/Endpoint.md#cross-file-subject"
    return "Model.md#" + name.lower().replace(" ", "-")


def git(root, *args):
    return subprocess.check_output(["git", *args], cwd=root)


def fixture(root, binary):
    shutil.copytree(TESTS / "fixtures/element-selection", root)
    shutil.copy2(TESTS / "test-semantic-queries/specifications/Queries.md", root / "Queries.md")
    shutil.copy2(TESTS / "test-concept-elements/specifications/Concepts.md", root / "Concepts.md")
    git(root, "init", "-q")
    git(root, "config", "user.name", "Selection E2E")
    git(root, "config", "user.email", "selection@example.invalid")
    git(root, "add", ".")
    git(root, "-c", "commit.gpgSign=false", "commit", "-qm", "Selection baseline")
    result = subprocess.run([binary, "validate", "--json"], cwd=root,
                            text=True, capture_output=True, timeout=20)
    require(result.returncode == 0, "Invalid selection fixture: " + result.stdout + result.stderr)


def physical_state(root):
    return {str(path.relative_to(root)): (path.read_bytes(), path.stat().st_mode & 0o777)
            for path in root.rglob("*") if path.is_file() and ".git" not in path.relative_to(root).parts}


def state(root, server=None):
    # Perform metadata readers first: Git may refresh index stat information.
    accepted = None if server is None else (
        server.revision(), server.tool("reqvire.search"), server.tool("reqvire.workspace_status"))
    runtime = server.http("/api/project-store/manifest") if server and server.mode == "serve" else None
    return (physical_state(root), git(root, "rev-parse", "HEAD"),
            git(root, "ls-files", "--stage"), (root / ".git/index").read_bytes(), accepted, runtime)


class Checks:
    def __init__(self, output):
        self.output = output
        self.results = []
        self.artifacts = []

    def check(self, label, action):
        try:
            action()
            passed = True
        except Exception:
            passed = False
            detail = traceback.format_exc()
            (self.output / f"failure-{len(self.results):03}.txt").write_text(detail)
            message = next(line for line in reversed(detail.splitlines()) if line.strip())
            print(f"FAIL {label}: {message[:350]}", file=sys.stderr, flush=True)
        self.results.append(f"{'PASS' if passed else 'FAIL'} {label}")

    def cli(self, root, binary, args, ok=True):
        result = subprocess.run([binary, *args], cwd=root, text=True, capture_output=True, timeout=20)
        self.artifacts.append({"args": args, "exit": result.returncode,
                               "stdout": result.stdout, "stderr": result.stderr})
        require((result.returncode == 0) == ok,
                f"{args}: exit={result.returncode}, {result.stderr or result.stdout}")
        return json.loads(result.stdout) if ok else result.stdout + result.stderr


@dataclass(frozen=True)
class Read:
    label: str
    tool: str
    args: dict
    cli: tuple
    selector: str


def readers(profile):
    cases = []
    if profile in ("model", "submodels", "coverage"):
        roots = ["Root Capability", "Nested Capability"]
        if profile != "coverage":
            roots += ["Parent Requirement", "Child Requirement"]
        for name in roots:
            cases.append(Read(profile + "/" + name, "reqvire." + profile,
                              {"from": name}, (profile, "--from", name, "--json")
                              if profile != "model" else (profile, "--from", name), "from"))
    if profile == "collect":
        for name in ("Root Capability", "Child Requirement", "Item Ontology", "Item Lookup", "Sparse Concept Scheme", "Detailed Concept"):
            for direction in ("UPSTREAM", "DOWNSTREAM"):
                cases.append(Read(f"collect/{name}/{direction}", "reqvire.collect",
                                  {"element_name": name, "direction": direction},
                                  ("collect", name, "--direction", direction, "--json"), "element_name"))
    if profile == "queries":
        for action in ("list", "validate", "export", "check"):
            cli = ("semantic", "query", action, "--name", "Item Lookup", "--json")
            if action == "check":
                cli += ("--artifact", "query.sparql")
            tool = "reqvire.semantic.queries.validate" if action == "validate" else "reqvire.semantic.queries"
            cases.append(Read("queries/" + action, tool,
                              {"name": "Item Lookup", **({"include_content": True} if action == "export" else {})},
                              cli, "name"))
    if profile == "native":
        cases += [Read("read-element", "reqvire.read_element", {"name": "Child Requirement"}, (), "name")]
        for name in ("Sparse Concept Scheme", "Detailed Concept"):
            cases.append(Read("native/" + name, "reqvire.concepts.get", {"name": name}, (), "name"))
    return cases


def cli_reads(checks, root, binary, profile):
    if profile == "queries":
        exported = subprocess.run([binary, "semantic", "query", "export", "--name", "Item Lookup"],
                                  cwd=root, capture_output=True, check=True, timeout=20).stdout
        (root / "query.sparql").write_bytes(exported)
    for case in readers(profile):
        baseline = {}
        def control(case=case):
            baseline["result"] = checks.cli(root, binary, case.cli)
            if profile == "submodels":
                require("submodels" in baseline["result"], "Invalid submodel control")
            elif case.label == "queries/check":
                require(baseline["result"].get("status") == "matching", "Artifact control did not match")
            else:
                require(identifier(case.args[case.selector]) in json.dumps(baseline["result"]),
                        "Name control omitted the intended canonical subject")
        checks.check("cli/" + case.label + "/name-control", control)
        def parity(case=case):
            name = case.args[case.selector]
            arguments = tuple(identifier(name) if value == name else value for value in case.cli)
            before = state(root)
            result = checks.cli(root, binary, arguments)
            require(result == baseline["result"], "Identifier and exact-name evidence differ")
            require(state(root) == before, "Read changed fixture or Git state")
        checks.check("cli/" + case.label + "/identifier-parity", parity)
    if profile in ("model", "submodels", "coverage", "collect"):
        collision = "Ambiguous.md#collision-capability"
        args = (profile, "--from", collision) if profile != "collect" else ("collect", collision)
        if profile != "model":
            args += ("--json",)
        def ambiguity():
            before = state(root)
            error = checks.cli(root, binary, args, ok=False)
            require("ambig" in error.lower() and collision in error and
                    "Ambiguous.md#ambiguousmdcollision-capability" in error,
                    "Ambiguity diagnostic must identify both canonical candidates")
            require(state(root) == before, "Ambiguous read changed files or Git")
        checks.check("cli/" + profile + "/ambiguous-name-identifier", ambiguity)
    if profile == "coverage":
        def type_check():
            error = checks.cli(root, binary, ("coverage", "--from", identifier("Child Requirement"), "--json"), ok=False)
            require("capability" in error.lower() and "not found" not in error.lower(),
                    "Known wrong-type identifier was reported as missing")
        checks.check("cli/coverage/identifier-type-check", type_check)
    def help_check():
        commands = [("semantic", "query", action, "--help") for action in ("list", "validate", "export", "check")] if profile == "queries" else [(profile, "--help")]
        for args in commands:
            result = subprocess.run([binary, *args], cwd=root, text=True, capture_output=True, timeout=20)
            require(result.returncode == 0 and "name" in result.stdout.lower() and "identifier" in result.stdout.lower(),
                    "Selector help must advertise exact names and canonical identifiers: " + " ".join(args))
    checks.check("cli/" + profile + "/selector-help", help_check)
    if profile == "queries":
        def iri_control():
            result = checks.cli(root, binary, ("semantic", "query", "export", "--iri", "urn:reqvire:semantic-query:item-lookup", "--json"))
            require(result == checks.cli(root, binary, readers(profile)[2].cli), "Explicit IRI semantics changed")
        checks.check("cli/queries/explicit-iri-control", iri_control)


@dataclass(frozen=True)
class Mutation:
    label: str
    tool: str
    args: dict
    selectors: tuple
    cli: tuple


def mutations(profile):
    all_cases = [
        Mutation("remove", "reqvire.remove_element", {"element_name": "Child Requirement"}, ("element_name",), ("rm", "Child Requirement")),
        Mutation("move", "reqvire.move_element", {"element_name": "Child Requirement", "file": "Moved.md"}, ("element_name",), ("mv", "Child Requirement", "Moved.md")),
        Mutation("rename", "reqvire.rename_element", {"element_name": "Child Requirement", "new_name": "Renamed Requirement"}, ("element_name",), ("rename", "Child Requirement", "Renamed Requirement")),
        Mutation("merge", "reqvire.merge_elements", {"target": "Child Requirement", "sources": ["Sibling Requirement", "Merge Requirement"]}, ("target", "sources"), ("merge", "Child Requirement", "Sibling Requirement", "Merge Requirement")),
        Mutation("link", "reqvire.link", {"source": "Child Requirement", "relation_type": "verifiedBy", "target": "Other Check"}, ("source", "target"), ("link", "Child Requirement", "verifiedBy", "Other Check")),
        Mutation("cross-file-link", "reqvire.link", {"source": "Cross-file Subject", "relation_type": "verifiedBy", "target": "Other Check"}, ("source", "target"), ("link", "Cross-file Subject", "verifiedBy", "Other Check")),
        Mutation("unlink", "reqvire.unlink", {"source": "Child Requirement", "target": "Selection Check"}, ("source", "target"), ("unlink", "Child Requirement", "Selection Check")),
        Mutation("relink", "reqvire.relink", {"source": "Child Requirement", "relation_type": "verifiedBy", "from_target": "Selection Check", "to_target": "Other Check"}, ("source", "from_target", "to_target"), ("relink", "Child Requirement", "verifiedBy", "Selection Check", "Other Check")),
        Mutation("binding", "reqvire.link", {"source": "Child Requirement", "relation_type": "bindContract", "target": identifier("External Specification")}, ("source",), ("link", "Child Requirement", "bindContract", identifier("External Specification"))),
        Mutation("reference", "reqvire.link", {"source": "Child Requirement", "relation_type": "referenceContract", "target": "Owned Specification"}, ("source", "target"), ("link", "Child Requirement", "referenceContract", "Owned Specification")),
        Mutation("binding-unlink", "reqvire.unlink", {"source": "Binding Consumer", "target": "Owned Specification"}, ("source", "target"), ("unlink", "Binding Consumer", "Owned Specification")),
        Mutation("reference-unlink", "reqvire.unlink", {"source": "Reference Consumer", "target": "Owned Specification"}, ("source", "target"), ("unlink", "Reference Consumer", "Owned Specification")),
        Mutation("reference-relink", "reqvire.relink", {"source": "Reference Consumer", "relation_type": "referenceContract", "from_target": "Owned Specification", "to_target": "External Specification"}, ("source", "from_target", "to_target"), ("relink", "Reference Consumer", "referenceContract", "Owned Specification", "External Specification")),
    ]
    groups = {"crud": ("remove", "move", "rename"), "merge": ("merge",),
              "links": ("link", "cross-file-link", "unlink", "binding", "reference", "binding-unlink", "reference-unlink"),
              "relink": ("relink", "reference-relink")}
    return [case for case in all_cases if profile == "all" or case.label in groups[profile]]


def variants(case):
    for key in case.selectors:
        value = case.args[key]
        # Every merge list member is asserted independently as well as together.
        if isinstance(value, list):
            for index, name in enumerate(value):
                yield f"{key}-{index}", {**case.args, key: [identifier(v) if i == index else v for i, v in enumerate(value)]}
        else:
            yield key, {**case.args, key: identifier(value)}
    yield "all-identifiers", {key: ([identifier(v) for v in value] if isinstance(value, list) else identifier(value))
                              if key in case.selectors else value for key, value in case.args.items()}
    if case.label == "binding":
        yield "target-name", {**case.args, "target": "External Specification"}


def mutation_cli(case, arguments):
    if case.label == "merge":
        return ("merge", arguments["target"], *arguments["sources"])
    keys = {"remove": ("element_name",), "move": ("element_name", "file"),
            "rename": ("element_name", "new_name"), "link": ("source", "relation_type", "target"),
            "binding": ("source", "relation_type", "target"), "reference": ("source", "relation_type", "target"),
            "unlink": ("source", "target"), "relink": ("source", "relation_type", "from_target", "to_target")}
    kind = {"reqvire.remove_element": "remove", "reqvire.move_element": "move", "reqvire.rename_element": "rename"}.get(case.tool, case.tool.rsplit(".", 1)[-1])
    return (case.cli[0], *(arguments[key] for key in keys[kind]))


def cli_mutations(checks, root, binary, profile):
    for case in mutations(profile):
        baseline = {}
        def control(case=case):
            before = state(root)
            baseline["result"] = checks.cli(root, binary, (*case.cli, "--dry-run", "--json"))
            require(baseline["result"].get("diffs"), "Control did not exercise a mutation")
            require(state(root) == before, "Preview changed files or Git")
        checks.check("cli/" + case.label + "/name-preview-control", control)
        for label, args in variants(case):
            def parity(args=args, case=case):
                before = state(root)
                result = checks.cli(root, binary, (*mutation_cli(case, args), "--dry-run", "--json"))
                require(result == baseline["result"], "Identifier preview differs from name preview")
                require(state(root) == before, "Identifier preview changed files or Git")
            checks.check("cli/" + case.label + "/" + label, parity)
    if profile == "merge":
        for label, sources in (("mixed-alias-self", [identifier("Child Requirement")]),
                               ("mixed-alias-duplicate", ["Sibling Requirement", identifier("Sibling Requirement")]),
                               ("late-unknown", ["Sibling Requirement", "Model.md#absent"])):
            def rejection(sources=sources, label=label):
                before = state(root)
                error = checks.cli(root, binary, ("merge", "Child Requirement", *sources, "--json"), ok=False)
                require(state(root) == before, "Rejected merge changed physical or Git state")
                if label != "late-unknown":
                    names = ["Child Requirement"] if label == "mixed-alias-self" else ["Sibling Requirement", "Sibling Requirement"]
                    expected = checks.cli(root, binary, ("merge", "Child Requirement", *names, "--json"), ok=False)
                    normalize = lambda text: re.sub(r"(?m)^\[[^\]]+\]\s*", "", text)
                    require(normalize(error) == normalize(expected), "Mixed aliases did not retain the owning merge duplicate/self behavior")
            checks.check("cli/merge/" + label, rejection)
        def merge_type_check():
            before = state(root)
            error = checks.cli(root, binary, ("merge", "Child Requirement", identifier("Owned Specification"), "--json"), ok=False)
            require("not found" not in error.lower() and ("type" in error.lower() or "merge" in error.lower()), "Known incompatible identifier was reported as missing")
            require(state(root) == before, "Type rejection persisted a merge candidate")
        checks.check("cli/merge/identifier-type-check", merge_type_check)
    if profile == "crud":
        def ambiguity():
            before = state(root)
            error = checks.cli(root, binary, ("rename", "Ambiguous.md#collision-requirement", "Unexpected Rename", "--json"), ok=False)
            require("ambig" in error.lower() and "Ambiguous.md#ambiguousmdcollision-requirement" in error,
                    "Ambiguous subject was not rejected with both candidates")
            require(state(root) == before, "Ambiguous mutation persisted a candidate")
        checks.check("cli/crud/ambiguous-subject-atomic", ambiguity)
        # Isolate accepted-write cases from the preview fixture.
        accepted = root.parent / "accepted"
        fixture(accepted, binary)
        def accepted_identifier():
            checks.cli(accepted, binary, ("rename", identifier("Child Requirement"), "Renamed Requirement", "--json"))
            require(re.search(r"(?m)^#{3,6} Renamed Requirement$", (accepted / "Model.md").read_text()), "Identifier mutation did not persist")
            checks.cli(accepted, binary, ("rm", identifier("Child Requirement"), "--json"), ok=False)
        checks.check("cli/crud/accepted-identifier-and-stale-rejection", accepted_identifier)
        def literal_name():
            literal = identifier("Parent Requirement")
            checks.cli(accepted, binary, ("rename", "Sibling Requirement", literal, "--json"))
            require(re.search(r"(?m)^#{3,6} " + re.escape(literal) + r"$", (accepted / "Model.md").read_text()), "new_name was resolved instead of authored literally")
        checks.check("cli/crud/literal-new-name", literal_name)


def error_result(result, tool, value=None, ambiguity=False):
    require(result.get("isError") is True, "Expected a structured tool error, got " + str(result)[:600])
    error = result.get("structuredContent", {}).get("error")
    require(isinstance(error, dict) and error.get("message") and error.get("tool") == tool,
            "Missing structured tool identity/cause: " + str(result)[:600])
    if value:
        require(value in json.dumps(error), "Diagnostic omitted the rejected selector value")
    if ambiguity:
        require("ambig" in json.dumps(error).lower() and "Ambiguous.md#ambiguousmdcollision-" in json.dumps(error),
                "Diagnostic omitted ambiguity or the second canonical candidate")


def mcp(checks, base, binary):
    spec = importlib.util.spec_from_file_location("selection_mcp_helpers", TESTS / "test-cache-integration/check_correctness.py")
    helpers = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helpers)
    for mode in ("mcp", "serve"):
        for commits in (False, True):
            prefix = f"{mode}/commits-{'enabled' if commits else 'disabled'}"
            root = base / (prefix.replace("/", "-") + "-fixture")
            fixture(root, binary)
            with helpers.Server(binary, root, checks.output, mode, prefix.replace("/", "-"),
                                mutations=True, commits=commits) as server:
                server.mode = mode
                for profile in ("model", "submodels", "coverage", "collect", "queries", "native"):
                    for case in readers(profile):
                        # MCP exposes discovery/validation/content export, not an artifact-check tool.
                        if case.label == "queries/check":
                            continue
                        baseline = {}
                        def control(case=case):
                            baseline["result"] = server.tool(case.tool, **case.args)
                            if profile == "submodels":
                                require("submodels" in baseline["result"], "Invalid submodel control")
                            else:
                                require(identifier(case.args[case.selector]) in json.dumps(baseline["result"]),
                                        "Name control selected an unexpected subject")
                        checks.check(prefix + "/" + case.label + "/name-control", control)
                        def parity(case=case):
                            result = server.tool(case.tool, **{**case.args, case.selector: identifier(case.args[case.selector])})
                            require(result == baseline["result"], "Identifier and name selected different evidence")
                        checks.check(prefix + "/" + case.label + "/identifier-parity", parity)
                definitions = {tool["name"]: tool for tool in server.rpc("tools/list", {})["tools"]}
                for tool, key in (("reqvire.coverage", "from"), ("reqvire.collect", "element_name"),
                                  ("reqvire.rename_element", "element_name"), ("reqvire.semantic.queries", "name")):
                    def discovery(tool=tool, key=key):
                        definition = definitions[tool]
                        text = definition["description"] + " " + definition["inputSchema"]["properties"][key].get("description", "")
                        require("name" in text.lower() and "identifier" in text.lower(), "Discovery omitted selector domain")
                    checks.check(prefix + "/discovery/" + tool, discovery)
                for case in mutations("all"):
                    baseline = {}
                    def control(case=case):
                        before = state(root, server)
                        baseline["result"] = server.tool(case.tool, **case.args, dry_run=True)
                        require(baseline["result"].get("diffs"), "Name control did not preview a candidate")
                        require(state(root, server) == before, "Preview changed accepted/physical/Git/runtime state")
                    checks.check(prefix + "/" + case.label + "/name-preview-control", control)
                    for label, arguments in variants(case):
                        def parity(arguments=arguments, case=case):
                            before = state(root, server)
                            result = server.tool(case.tool, **arguments, dry_run=True)
                            require(result == baseline["result"], "Identifier preview differs from name preview")
                            require(state(root, server) == before, "Identifier preview published state")
                        checks.check(prefix + "/" + case.label + "/" + label, parity)
                for label, tool, arguments, value, ambiguous in (
                    ("ambiguous-read", "reqvire.read_element", {"name": "Ambiguous.md#collision-requirement"}, "Ambiguous.md#collision-requirement", True),
                    ("ambiguous-write", "reqvire.rename_element", {"element_name": "Ambiguous.md#collision-requirement", "new_name": "Unexpected Rename", "dry_run": True}, "Ambiguous.md#collision-requirement", True),
                    ("ambiguous-apply", "reqvire.rename_element", {"element_name": "Ambiguous.md#collision-requirement", "new_name": "Unexpected Rename"}, "Ambiguous.md#collision-requirement", True),
                    ("unknown-write", "reqvire.rename_element", {"element_name": "Model.md#absent", "new_name": "Unexpected Rename"}, "Model.md#absent", False),
                    ("contradictory-read", "reqvire.read_element", {"name": "Child Requirement", "identifier": identifier("Sibling Requirement")}, None, False),
                    ("late-unknown-merge", "reqvire.merge_elements", {"target": "Child Requirement", "sources": ["Sibling Requirement", "Model.md#absent"]}, "Model.md#absent", False),
                ):
                    def rejection(tool=tool, arguments=arguments, value=value, ambiguous=ambiguous):
                        before = state(root, server)
                        result = server.raw_tool(tool, **arguments)
                        error_result(result, tool, value, ambiguous)
                        require(state(root, server) == before, "Selector rejection changed accepted/physical/Git/runtime state")
                        require(server.tool("reqvire.read_element", identifier=identifier("Child Requirement"))["name"] == "Child Requirement",
                                "Reads unusable after rejection")
                    checks.check(prefix + "/" + label + "/unchanged-state", rejection)
                def matching_selectors():
                    result = server.tool("reqvire.read_element", name="Child Requirement", identifier=identifier("Child Requirement"))
                    require(result["identifier"] == identifier("Child Requirement"), "Consistent selectors did not resolve once")
                checks.check(prefix + "/consistent-explicit-selectors-control", matching_selectors)
                def native_iri_control():
                    name = server.tool("reqvire.concepts.get", name="Detailed Concept")
                    explicit = server.tool("reqvire.concepts.get", iri="https://example.test/native-concepts#DetailedConcept")
                    require(name == explicit, "Native concept explicit IRI semantics changed")
                    error_result(server.raw_tool("reqvire.concepts.get", name="Sparse Concept Scheme", identifier=identifier("Detailed Concept")), "reqvire.concepts.get")
                checks.check(prefix + "/native-contradictory-selectors", native_iri_control)
                def merge_type_check():
                    before = state(root, server)
                    result = server.raw_tool("reqvire.merge_elements", target="Child Requirement", sources=[identifier("Owned Specification")])
                    error_result(result, "reqvire.merge_elements", identifier("Owned Specification"))
                    require("not found" not in json.dumps(result["structuredContent"]["error"]).lower(), "Known incompatible identifier was reported as missing")
                    require(state(root, server) == before, "Type rejection persisted a candidate")
                checks.check(prefix + "/merge/identifier-type-check", merge_type_check)
                for label, name_sources, alias_sources in (
                    ("self", ["Child Requirement"], [identifier("Child Requirement")]),
                    ("duplicate", ["Sibling Requirement", "Sibling Requirement"], ["Sibling Requirement", identifier("Sibling Requirement")]),
                ):
                    def alias_rejection(name_sources=name_sources, alias_sources=alias_sources):
                        before = state(root, server)
                        expected = server.raw_tool("reqvire.merge_elements", target="Child Requirement", sources=name_sources)
                        error_result(expected, "reqvire.merge_elements")
                        result = server.raw_tool("reqvire.merge_elements", target="Child Requirement", sources=alias_sources)
                        error_result(result, "reqvire.merge_elements")
                        require(result["structuredContent"]["error"] == expected["structuredContent"]["error"], "Mixed aliases changed the operation's duplicate/self rejection")
                        require(state(root, server) == before, "Mixed-alias rejection persisted a candidate")
                    checks.check(prefix + "/merge/mixed-alias-" + label, alias_rejection)
                def type_check():
                    result = server.raw_tool("reqvire.coverage", **{"from": identifier("Child Requirement")})
                    error_result(result, "reqvire.coverage", identifier("Child Requirement"))
                    message = json.dumps(result["structuredContent"]["error"]).lower()
                    require("capability" in message and "not found" not in message, "Known wrong-type identifier was reported as missing")
                checks.check(prefix + "/identifier-type-check", type_check)
                def explicit_iri():
                    expected = server.tool("reqvire.semantic.queries", name="Item Lookup", include_content=True)
                    actual = server.tool("reqvire.semantic.queries", iri="urn:reqvire:semantic-query:item-lookup", include_content=True)
                    require(actual == expected, "Explicit semantic IRI domain changed")
                checks.check(prefix + "/explicit-query-iri-control", explicit_iri)
                def filter_control():
                    selected = server.tool("reqvire.search", filter_name="^Child Requirement$")
                    require(identifier("Child Requirement") in json.dumps(selected) and identifier("Sibling Requirement") not in json.dumps(selected), "filter_name lost regex semantics")
                checks.check(prefix + "/pattern-filter-control", filter_control)
                def accepted_snapshot():
                    path = root / "Model.md"
                    original = path.read_bytes()
                    expected = server.tool("reqvire.read_element", identifier=identifier("Child Requirement"))
                    try:
                        path.write_bytes(original.replace(b"preserve the selected subject", b"import unaccepted physical content"))
                        actual = server.tool("reqvire.read_element", name=identifier("Child Requirement"))
                        expected_context = expected.pop("context")
                        actual_context = actual.pop("context")
                        require(actual == expected, "Identifier selected unaccepted model content or evidence")
                        require(actual_context.pop("dirty") is True, "Physical edit missing from live Git status")
                        expected_context.pop("dirty")
                        require(actual_context == expected_context, "Physical edit changed accepted revision or context identity")
                    finally:
                        path.write_bytes(original)
                checks.check(prefix + "/identifier-uses-accepted-snapshot", accepted_snapshot)
                def later_name_write():
                    server.tool("reqvire.rename_element", element_name="Sibling Requirement", new_name="Usable Sibling")
                    require(server.tool("reqvire.read_element", name="Usable Sibling")["identifier"] == identifier("Usable Sibling"), "Writes unusable after selector rejection")
                    before = state(root, server)
                    error_result(server.raw_tool("reqvire.remove_element", element_name="Model.md#absent"), "reqvire.remove_element", "Model.md#absent")
                    require(state(root, server) == before, "Rejected selector discarded earlier accepted changes")
                checks.check(prefix + "/later-write-and-pending-state-control", later_name_write)
                def accepted_write():
                    initial = git(root, "rev-parse", "HEAD").decode().strip()
                    result = server.tool("reqvire.rename_element", element_name=identifier("Child Requirement"), new_name="Renamed Requirement")
                    accepted_head = git(root, "rev-parse", "HEAD").decode().strip()
                    require((accepted_head != initial) == commits and
                            (result.get("commit") == accepted_head if commits else "commit" not in result),
                            "Identifier mutation violated the commit policy")
                    require(server.tool("reqvire.read_element", name="Renamed Requirement")["identifier"] == identifier("Renamed Requirement"),
                            "Accepted identifier mutation did not publish its subject")
                    before = state(root, server)
                    error_result(server.raw_tool("reqvire.remove_element", element_name=identifier("Child Requirement")),
                                 "reqvire.remove_element", identifier("Child Requirement"))
                    require(state(root, server) == before, "Stale identifier changed accepted pending changes")
                checks.check(prefix + "/accepted-identifier-and-stale-rejection", accepted_write)
    # Context setup owns a fresh, clean repository under each commit policy.
    for mode in ("mcp", "serve"):
        for commits in (False, True):
            prefix = f"{mode}/commits-{'enabled' if commits else 'disabled'}"
            root = base / (prefix.replace("/", "-") + "-contexts-fixture")
            fixture(root, binary)
            with helpers.Server(binary, root, checks.output, mode, prefix.replace("/", "-") + "-contexts",
                                mutations=True, commits=commits) as server:
                contexts = {}
                def context_control():
                    inventory = server.tool("reqvire.worktree.list")["worktrees"]
                    contexts["origin"] = next(row["worktree_id"] for row in inventory if row["original"])
                    contexts["child"] = server.tool("reqvire.worktree.create", branch="selection-child", base_ref="HEAD")["worktree_id"]
                    parent = (TESTS / "fixtures/element-selection/Model.md").read_text().split("### Child Requirement\n", 1)[1].split("---", 1)[0]
                    content = "### Child Requirement\n" + parent.replace("preserve the selected subject", "provide distinct child context evidence")
                    server.tool("reqvire.add_element", worktree_id=contexts["child"], file="Model.md", content=content, override_existing=True)
                    a = server.tool("reqvire.read_element", worktree_id=contexts["origin"], identifier=identifier("Child Requirement"))
                    b = server.tool("reqvire.read_element", worktree_id=contexts["child"], identifier=identifier("Child Requirement"))
                    require(a["identifier"] == b["identifier"] and a["name"] == b["name"] and a["content"] != b["content"],
                            "Context control did not establish different accepted content at the same relative identifier")
                    contexts["expected"] = {"origin": a, "child": b}
                checks.check(prefix + "/two-context-setup-control", context_control)
                for context in ("origin", "child"):
                    def context_selection(context=context):
                        expected = contexts["expected"][context]
                        selected = server.tool("reqvire.read_element", worktree_id=contexts[context], name=identifier("Child Requirement"))
                        require(selected == expected, "Selection escaped its requested context")
                    checks.check(prefix + "/two-context/" + context + "/identifier-parity", context_selection)
                def foreign_context():
                    content = "### Context Only\n\nA child-context capability.\n\n#### Metadata\n  * type: capability\n"
                    server.tool("reqvire.add_element", worktree_id=contexts["child"], file="Model.md", content=content)
                    before = server.tool("reqvire.model_revision", worktree_id=contexts["origin"])
                    result = server.raw_tool("reqvire.read_element", worktree_id=contexts["origin"], name=identifier("Context Only"))
                    error_result(result, "reqvire.read_element", identifier("Context Only"))
                    require(server.tool("reqvire.model_revision", worktree_id=contexts["origin"]) == before, "Foreign selector changed the origin snapshot")
                checks.check(prefix + "/foreign-context-identifier-rejected", foreign_context)
    for mode in ("mcp", "serve"):
        root = base / (mode + "-readonly-fixture")
        fixture(root, binary)
        with helpers.Server(binary, root, checks.output, mode, mode + "-readonly") as server:
            def fresh_name_control():
                path = root / "Model.md"
                path.write_bytes(path.read_bytes().replace(b"preserve the selected subject", b"expose fresh physical content"))
                require("expose fresh physical content" in server.tool("reqvire.read_element", name="Child Requirement")["content"],
                        "Read-only exact-name control did not refresh")
            checks.check(mode + "/readonly/fresh-name-control", fresh_name_control)
            def fresh_identifier():
                expected = server.tool("reqvire.read_element", identifier=identifier("Child Requirement"))
                actual = server.tool("reqvire.read_element", name=identifier("Child Requirement"))
                require(actual == expected and "expose fresh physical content" in actual["content"], "Read-only identifier selected stale content")
            checks.check(mode + "/readonly/fresh-identifier-parity", fresh_identifier)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", required=True)
    parser.add_argument("--workspace", required=True)
    parser.add_argument("--profile", required=True, choices=("model", "submodels", "coverage", "collect", "queries", "crud", "merge", "links", "relink", "mcp"))
    args = parser.parse_args()
    base = Path(args.workspace) / "output/element-selection"
    base.mkdir()
    checks = Checks(base)
    try:
        if args.profile == "mcp":
            mcp(checks, base, args.binary)
        else:
            root = base / "fixture"
            fixture(root, args.binary)
            if args.profile in ("crud", "merge", "links", "relink"):
                cli_mutations(checks, root, args.binary, args.profile)
            else:
                cli_reads(checks, root, args.binary, args.profile)
    except Exception:
        checks.check("setup-or-harness-error", lambda: (_ for _ in ()).throw(RuntimeError(traceback.format_exc())))
    finally:
        (base / "checks.txt").write_text("\n".join(checks.results) + "\n")
        (base / "cli-responses.json").write_text(json.dumps(checks.artifacts, indent=2) + "\n")
    print("\n".join(checks.results))
    return int(any(line.startswith("FAIL ") for line in checks.results))


if __name__ == "__main__":
    sys.exit(main())
