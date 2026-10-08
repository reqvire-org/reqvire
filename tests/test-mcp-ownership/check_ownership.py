"""Opt-in commit policy through the real CLI and standalone/embedded HTTP MCP."""
import difflib
import importlib.util
import pathlib
import os
import signal
import subprocess
import sys
import tempfile
import traceback

binary, destination = sys.argv[1:]
suite = pathlib.Path(__file__).resolve().parent
source = suite.parent / "test-cache-integration"
spec = importlib.util.spec_from_file_location("cache_checks", source / "check_correctness.py")
helpers = importlib.util.module_from_spec(spec)
spec.loader.exec_module(helpers)
output = pathlib.Path(destination) / "output"
failures = []


def check(name, passed, detail=""):
    print(f"{'PASS' if passed else 'FAIL'} {name}", flush=True)
    if not passed:
        failures.append(name)
        print(f"{name}: {detail}", file=sys.stderr)


def git(root, *args):
    return subprocess.check_output(["git", *args], cwd=root, text=True).strip()


def command(mode, commits):
    return [binary, mode, "--enable-mutations", "--port", "0",
            *(["--enable-mcp"] if mode == "serve" else []),
            *(["--enable-commits"] if commits else [])]


def reject(root, mode, commits, reason):
    process = subprocess.Popen(command(mode, commits), cwd=root, text=True,
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    try:
        _, stderr = process.communicate(timeout=10)
        assert process.returncode != 0 and reason in stderr, stderr
    finally:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.communicate(timeout=5)


def cli_options():
    # Argument checks do not need sockets or a valid model.
    with tempfile.TemporaryDirectory(prefix="reqvire-commit-options-") as directory:
        for mode in ("mcp", "serve"):
            result = subprocess.run([binary, mode, "--help"], cwd=directory,
                                    capture_output=True, text=True, timeout=10)
            check(f"{mode}/commit-option-help", result.returncode == 0
                  and "--enable-commits" in result.stdout and "default: false" in result.stdout
                  and "requires --enable-mutations" in result.stdout, result.stdout + result.stderr)
            result = subprocess.run([binary, mode, "--enable-commits",
                                     *(["--enable-mcp"] if mode == "serve" else [])],
                                    cwd=directory, capture_output=True, text=True, timeout=10)
            check(f"{mode}/commits-require-mutations", result.returncode != 0
                  and "required arguments" in result.stderr and "--enable-mutations" in result.stderr
                  and "unexpected argument" not in result.stderr, result.stderr)
            result = subprocess.run([*command(mode, True), "--help"], cwd=directory,
                                    capture_output=True, text=True, timeout=10)
            check(f"{mode}/commit-option-accepted", result.returncode == 0
                  and "--enable-commits" in result.stdout, result.stderr)
        result = subprocess.run([binary, "serve", "--enable-mutations", "--enable-commits"],
                                cwd=directory, capture_output=True, text=True, timeout=10)
        check("serve/commits-require-embedded-mcp", result.returncode != 0
              and "required arguments" in result.stderr and "--enable-mcp" in result.stderr
              and "unexpected argument" not in result.stderr, result.stderr)


def lifecycle(mode, commits):
    prefix = f"{mode}/commits-{'enabled' if commits else 'disabled'}"
    label = prefix.replace("/", "-")
    with tempfile.TemporaryDirectory(prefix="reqvire-ownership-") as temporary:
        root = pathlib.Path(temporary)
        git(root, "init", "-q")
        git(root, "config", "user.email", "test@example.invalid")
        git(root, "config", "user.name", "MCP E2E")
        model = root / "Model.md"
        model.write_text((source / "fixtures/model.md.txt").read_text())
        git(root, "add", ".")
        git(root, "commit", "-qm", "baseline")
        original = model.read_text()
        initial = git(root, "rev-parse", "HEAD")
        branch = git(root, "symbolic-ref", "HEAD")
        (root / "extra.txt").write_text("untracked")
        reject(root, mode, commits, "clean")
        git(root, "add", "extra.txt")
        reject(root, mode, commits, "clean")
        git(root, "reset", "-q", "HEAD", "--", "extra.txt")
        (root / "extra.txt").unlink()
        model.write_text(original + "\n")
        reject(root, mode, commits, "clean")
        model.write_text(original)
        check(f"{prefix}/dirty-startup", True)
        with helpers.Server(binary, root, output, mode, f"{label}-owned",
                            mutations=True, commits=commits) as server:
            reject(root, mode, commits, "owned")
            check(f"{prefix}/exclusive-owner", True)
            definitions = server.rpc("tools/list", {})["tools"]
            reconcile = next(tool for tool in definitions if tool["name"] == "reqvire.git.reconcile")
            denied = server.raw_tool("reqvire.git.reconcile", attempted_commit=initial, dry_run=False)
            check(f"{prefix}/reconcile-contract", reconcile["inputSchema"]["properties"]["dry_run"]["default"] is True
                  and "attempted_commit" in reconcile["inputSchema"]["required"]
                  and reconcile["annotations"]["readOnlyHint"] is False
                  and denied.get("isError") is True and "No recorded" in str(denied)
                  and git(root, "rev-parse", "HEAD") == initial and model.read_text() == original)
            content = (source / "fixtures/other.md.txt").read_text().split("# Elements\n\n", 1)[1]
            before_index = git(root, "ls-files", "--stage")
            runtime_before = server.http("/api/project-store/manifest") if mode == "serve" else None
            preview = server.tool("reqvire.add_element", file="Model.md", content=content, dry_run=True)
            rejected = server.raw_tool("reqvire.add_element", file="Model.md",
                                       content=original.split("# Elements\n\n", 1)[1])
            check(f"{prefix}/preview-and-rejection", rejected.get("isError") is True
                  and "commit" not in preview and "commit" not in rejected.get("structuredContent", {})
                  and model.read_text() == original and git(root, "rev-parse", "HEAD") == initial
                  and git(root, "ls-files", "--stage") == before_index
                  and (mode != "serve" or server.http("/api/project-store/manifest") == runtime_before))
            model.write_text("external invalid edit")
            check(f"{prefix}/external-edit-not-imported", "Cache Subject" in str(server.tool("reqvire.search")))
            (root / "unrelated.txt").write_text("human work")
            git(root, "add", "unrelated.txt")
            before_index = git(root, "ls-files", "--stage")
            result = server.tool("reqvire.add_element", file="Model.md", content=content)
            head = git(root, "rev-parse", "HEAD")
            if commits:
                publication = (result.get("commit") == head and git(root, "rev-parse", "HEAD^") == initial
                               and git(root, "diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD") == "Model.md"
                               and git(root, "show", "HEAD:Model.md") == model.read_text().strip())
            else:
                publication = ("commit" not in result and head == initial and git(root, "ls-files", "--stage") == before_index
                               and git(root, "diff", "--name-only") == "Model.md")
            expected = (source / "expected/runtime-after-write.md.txt").read_text()
            check(f"{prefix}/persist-and-commit-policy", publication and model.read_text() == expected,
                  str(result) + "\n" + "".join(difflib.unified_diff(expected.splitlines(True), model.read_text().splitlines(True))))
            check(f"{prefix}/unrelated-staging-preserved", git(root, "diff", "--cached", "--name-only") == "unrelated.txt")
            check(f"{prefix}/semantic-snapshot", server.tool("reqvire.semantic.sparql",
                  query='ASK { ?s <https://www.reqvire.org/ontology#elementName> "Other Subject" }')["boolean"] is True)
            if mode == "serve":
                status, store = server.http("/api/project-store")
                refreshed = server.http("/api/project-store/manifest")
                check(f"{prefix}/runtime-refresh", status == 200 and "Other Subject" in store
                      and refreshed[0] == 200 and refreshed != runtime_before)
            counts = server.counts()
            server.tool("reqvire.search")
            server.tool("reqvire.search")
            check(f"{prefix}/snapshot-reuse", server.counts() == counts)
            third = (source / "fixtures/third.md.txt").read_text().split("# Elements\n\n", 1)[1]
            result = server.tool("reqvire.add_element", file="Model.md", content=third)
            check(f"{prefix}/successive-mutation", "Third Subject" in str(server.tool("reqvire.search"))
                  and ((result.get("commit") == git(root, "rev-parse", "HEAD")
                        and git(root, "rev-parse", "HEAD^") == head) if commits
                       else ("commit" not in result and git(root, "rev-parse", "HEAD") == initial
                             and git(root, "ls-files", "--stage") == before_index)))
            # Normalize once, then exercise a genuinely empty execution.
            server.tool("reqvire.format", fix=True)
            before_file, before_index = model.read_bytes(), git(root, "ls-files", "--stage")
            before_head = git(root, "rev-parse", "HEAD")
            before_runtime = server.http("/api/project-store/manifest") if mode == "serve" else None
            noop = server.tool("reqvire.format", fix=True)
            check(f"{prefix}/noop", "commit" not in noop and model.read_bytes() == before_file
                  and git(root, "ls-files", "--stage") == before_index and git(root, "rev-parse", "HEAD") == before_head
                  and (mode != "serve" or server.http("/api/project-store/manifest") == before_runtime))
            git(root, "reset", "-q", "HEAD", "--", "unrelated.txt")
            (root / "unrelated.txt").unlink()
            head = git(root, "rev-parse", "HEAD")
        if not commits:
            reject(root, mode, commits, "clean")
            check(f"{prefix}/dirty-restart-rejected", git(root, "rev-parse", "HEAD") == initial
                  and "Third Subject" in model.read_text())
            git(root, "add", "Model.md")
            git(root, "commit", "-qm", "user accepts MCP changes")
            head = git(root, "rev-parse", "HEAD")
        with helpers.Server(binary, root, output, mode, f"{label}-resumed",
                            mutations=True, commits=commits) as server:
            check(f"{prefix}/restart-same-branch", "Third Subject" in str(server.tool("reqvire.search"))
                  and git(root, "rev-parse", "HEAD") == head and git(root, "symbolic-ref", "HEAD") == branch)
            git(root, "commit", "--allow-empty", "-qm", "external change")
            result = server.raw_tool("reqvire.add_element", file="Model.md", content=content)
            check(f"{prefix}/external-head-rejected", result.get("isError") is True and "HEAD changed" in str(result))


def recovery_reads(mode):
    prefix = f"{mode}/recovery"
    with tempfile.TemporaryDirectory(prefix="reqvire-recovery-") as directory:
        root = pathlib.Path(directory)
        git(root, "init", "-q")
        git(root, "config", "user.email", "test@example.invalid")
        git(root, "config", "user.name", "MCP Recovery E2E")
        (root / "Model.md").write_text((source / "fixtures/model.md.txt").read_text())
        (root / "evidence.txt").write_text("accepted evidence")
        git(root, "add", ".")
        git(root, "commit", "-qm", "baseline")
        head = git(root, "rev-parse", "HEAD")
        with helpers.Server(binary, root, output, mode, f"{mode}-recovery", mutations=True, commits=True) as server:
            before = server.tool("reqvire.search")
            before_store = server.http("/api/project-store") if mode == "serve" else None
            hook = root / ".git/hooks/reference-transaction"
            hook.write_text('#!/bin/sh\nif [ "$1" = prepared ]; then rm -f Model.md; mkdir Model.md; exit 1; fi\n')
            hook.chmod(0o755)
            content = (source / "fixtures/other.md.txt").read_text().split("# Elements\n\n", 1)[1]
            failed = server.raw_tool("reqvire.add_element", file="Model.md", content=content)
            hook.unlink()
            check(f"{prefix}/failure-injected", failed.get("isError") is True and "recovery failed" in str(failed)
                  and (root / "Model.md").is_dir())
            after = server.tool("reqvire.search")
            context = after["context"]
            check(f"{prefix}/accepted-reads", after["files"] == before["files"]
                  and context["available"] and not context["writes_available"]
                  and context["state"] == "recovery_required" and context["diagnostic"]
                  and context["model_revision"] == before["context"]["model_revision"])
            status = server.tool("reqvire.workspace_status")
            resource = server.rpc("resources/read", {"uri":"reqvire://workspace/status"})
            semantic = server.tool("reqvire.semantic.sparql", query='ASK { ?s <https://www.reqvire.org/ontology#elementName> "Cache Subject" }')
            check(f"{prefix}/status-resources-semantics", status["context"]["recovery_required"]
                  and resource["_meta"]["reqvire/context"]["recovery_required"] and semantic["boolean"])
            blocked = [server.raw_tool("reqvire.add_element", file="Model.md", content=content, dry_run=True),
                       server.raw_tool("reqvire.format", fix=False), server.raw_tool("reqvire.change_impact"),
                       server.raw_tool("reqvire.git.commit", message="forbidden"),
                       server.raw_tool("reqvire.git.reconcile", attempted_commit=head, dry_run=False)]
            check(f"{prefix}/unsafe-operations-blocked", all(r.get("isError") is True and "recovery" in str(r) for r in blocked)
                  and git(root, "rev-parse", "HEAD") == head and (root / "Model.md").is_dir())
            if mode == "serve":
                check(f"{prefix}/explorer-snapshot", server.http("/api/project-store") == before_store
                      and server.http("/evidence.txt")[0] == 503)


cli_options()
for mode in ("mcp", "serve"):
    for commits in (False, True):
        try:
            lifecycle(mode, commits)
        except Exception:
            check(f"{mode}/commits-{'enabled' if commits else 'disabled'}/infrastructure-or-startup", False,
                  traceback.format_exc())
for mode in ("mcp", "serve"):
    try:
        recovery_reads(mode)
    except Exception:
        check(f"{mode}/recovery/infrastructure-or-startup", False, traceback.format_exc())
raise SystemExit(bool(failures))
