"""HEAD adoption through real MCP workers, reusing loaded contexts.

Six workflows use four servers: ordinary HTTP, stdio, automatic-commit HTTP,
and embedded Explorer. Rejection variations restore only their disposable Git
and file state; they never restart a worker or reopen/rebuild its model.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time
import traceback

SUITE = Path(__file__).resolve().parent
BINARY, DESTINATION = sys.argv[1:]
DESTINATION = Path(DESTINATION)
OUTPUT = DESTINATION / "output"
OUTPUT.mkdir(exist_ok=True)
TOOL = "reqvire.branch.refresh"
report = []
metrics = {"server_starts": 0}
started = time.monotonic()


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


helpers = module("head_cache_helpers", SUITE.parent / "test-cache-integration/check_correctness.py")
transports = module("head_transport_helpers", SUITE.parent / "test-mcp-server/check_transports.py")
MODEL = """# Elements

### Refresh Capability

As an engineer, I want a coherent accepted model.

#### Metadata
  * type: capability

---

### Refresh Requirement

The system SHALL preserve explicit traceability.

#### Metadata
  * type: requirement

#### Relations
  * specify: [Refresh Capability](#refresh-capability)
  * satisfiedBy: [Implementation](impl.rs)
"""
ONTOLOGY = (SUITE.parent / "test-cache-integration/fixtures/ontology.md.txt").read_text()
EXTERNAL = (SUITE.parent / "test-cache-integration/fixtures/external.ttl").read_text()


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def git(root, *arguments):
    return subprocess.check_output(["git", *arguments], cwd=root, text=True,
                                   env={**os.environ, "GIT_OPTIONAL_LOCKS": "0"}).strip()


def commit(root, amend=False):
    git(root, "add", ".")
    git(root, "-c", "commit.gpgSign=false", "commit", "-qm", "External artifact update",
        *(["--amend"] if amend else []))
    return git(root, "rev-parse", "HEAD")


def project(name, semantic=False):
    root = DESTINATION / name / "repo"
    root.mkdir(parents=True)
    git(root, "init", "-qb", "main")
    git(root, "config", "user.email", "head-refresh@example.invalid")
    git(root, "config", "user.name", "HEAD Refresh Test")
    (root / "Model.md").write_text(MODEL)
    (root / "impl.rs").write_text("// accepted implementation\n")
    (root / "README.md").write_text("# Project Notes\n\nOrdinary documentation.\n")
    (root / ".reqvireignore").write_text("excluded/**\nignored/**\n")
    # Leave root .gitignore absent so its addition can be tested in this session.
    # Git excludes the captured ignored dependency; Reqvire uses .reqvireignore.
    (root / ".git/info/exclude").write_text("ignored/**\n")
    external_paths = []
    if semantic:
        (root / "Scope.md").write_text("# Elements\n\n### Scope Policy Capability\n\nPolicy scoped behavior.\n\n#### Metadata\n  * type: capability\n")
        (root / "excluded").mkdir()
        (root / "excluded/Hidden.md").write_text("# Elements\n\n### Hidden Policy Capability\n\nHidden by current policy.\n\n#### Metadata\n  * type: capability\n")
        folder = root / "ontologies"
        folder.mkdir()
        for kind, source, dependency in [
            ("loaded", "external.ttl", folder / "external.ttl"),
            ("ignored", "../ignored/ignored.ttl", root / "ignored/ignored.ttl"),
            ("outside", str(root.parent / "outside.ttl"), root.parent / "outside.ttl"),
        ]:
            ontology = (ONTOLOGY.replace("Cache Ontology", "Refresh " + kind.title() + " Ontology")
                        .replace("https://example.test/external", "https://example.test/external/" + kind)
                        .replace("local:Resource", "local:Resource" + kind)
                        .replace("prefix: ext", "prefix: ext" + kind)
                        .replace("ext:", "ext" + kind + ":")
                        .replace("source: external.ttl", "source: " + source))
            if kind != "loaded":
                ontology = ontology.replace("  * ontology_base: https://example.test/cache\n", "")
                ontology = ontology.replace("  * ontology_prefix: local\n", "")
                ontology += ("\n#### Relations\n"
                             "  * derivedFrom: [Refresh Loaded Ontology](loaded.md#refresh-loaded-ontology)\n")
            (folder / (kind + ".md")).write_text(ontology)
            dependency.parent.mkdir(exist_ok=True)
            dependency.write_text(EXTERNAL.replace("https://example.test/external",
                                                   "https://example.test/external/" + kind)
                                  .replace("ext:", "ext" + kind + ":"))
            if kind != "loaded":
                external_paths.append(dependency)
        # One consumed source serves both roles; a second model/process would
        # duplicate the same semantic-source protection assertion.
        (root / "Model.md").write_text(MODEL + "  * satisfiedBy: [Vocabulary Source](ontologies/external.ttl)\n")
    commit(root)
    return root, external_paths


def physical(root):
    index = Path(git(root, "rev-parse", "--path-format=absolute", "--git-path", "index"))
    return {
        "head": git(root, "rev-parse", "HEAD"),
        "refs": git(root, "for-each-ref", "--format=%(refname) %(objectname)"),
        "index": hashlib.sha256(index.read_bytes()).hexdigest(),
        "files": {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest()
                  for p in root.rglob("*") if p.is_file() and ".git" not in p.relative_to(root).parts},
    }


def server(root, name, mode="mcp", commits=False):
    metrics["server_starts"] += 1
    return helpers.Server(BINARY, root, OUTPUT, mode, name, mutations=True, commits=commits)


def original(client):
    return next(row["branch"] for row in client.tool("reqvire.branch.list")["worktrees"]
                if row.get("original"))


def refresh(client, ident, branch, **arguments):
    result = client.raw_tool(TOOL, branch=branch, **arguments)
    data = result.get("structuredContent", {})
    require(data.get("outcome") in {"refreshed", "unchanged", "checked", "blocked"},
            "Missing refresh contract, not an eligible refresh/rejection: " + json.dumps(result))
    require(data.get("branch") == branch and "worktree_id" not in data, "Refresh lost selected context")
    return data


class LoadedContext:
    def __init__(self, client, root, ident, external_paths=()):
        self.client, self.root, self.ident = client, root, ident
        self.branch = git(root, "branch", "--show-current")
        self.head = git(root, "rev-parse", "HEAD")
        self.revision = client.tool("reqvire.model_revision", branch=self.branch)["model_fingerprint"]
        self.external = {path: path.read_bytes() for path in external_paths}

    def refresh(self, **arguments):
        data = refresh(self.client, self.ident, self.branch, **arguments)
        if data["outcome"] == "refreshed":
            # Subsequent variations restore to the most recently accepted tip,
            # including when an assertion after successful adoption fails.
            self.head = data["accepted_head"]
        return data

    def blocked(self, reason, path=None):
        before = physical(self.root)
        data = refresh(self.client, self.ident, self.branch)
        require(data["outcome"] == "blocked" and data.get("can_refresh") is False, str(data))
        require(data.get("accepted_head") == self.head, "Rejected refresh adopted HEAD")
        require(any(b.get("reason") == reason and (path is None or b.get("path") == path)
                    for b in data.get("blockers", [])), "Wrong/missing blocker: " + str(data))
        require(physical(self.root) == before, "Rejected refresh changed files, index or refs")
        revision = self.client.tool("reqvire.model_revision", branch=self.branch)["model_fingerprint"]
        require(revision == self.revision, "Rejected refresh replaced the model or blocked accepted reads")

    def restore(self, branch_changed=False):
        # These commands target only a repo created by project() in this suite.
        # Restore physical inputs to what the still-running worker has accepted.
        # Never reopen the context or use model loading as test cleanup.
        if branch_changed:
            git(self.root, "checkout", "-qf", self.branch)
            git(self.root, "branch", "-D", "external-test")
        git(self.root, "reset", "--hard", self.head)
        git(self.root, "clean", "-fd")
        for path, content in self.external.items():
            path.write_bytes(content)
        revision = self.client.tool("reqvire.model_revision", branch=self.branch)["model_fingerprint"]
        require(revision == self.revision, "Fixture restoration lost the accepted model")
        require(git(self.root, "rev-parse", "HEAD") == self.head, "Fixture restoration lost accepted HEAD")

    def variation(self, callback, branch_changed=False):
        try:
            callback()
        finally:
            self.restore(branch_changed=branch_changed)


def workflow(name, callback):
    steps = []

    def check(label, action):
        try:
            action()
            steps.append({"case": label, "passed": True})
        except Exception as error:
            steps.append({"case": label, "passed": False, "error": str(error)})
            print(name + "/" + label + ": " + traceback.format_exc(), file=sys.stderr, flush=True)

    began = time.monotonic()
    try:
        callback(check)
    except Exception as error:
        steps.append({"case": "workflow-setup", "passed": False, "error": str(error)})
        print(name + ": " + traceback.format_exc(), file=sys.stderr, flush=True)
    passed = bool(steps) and all(step["passed"] for step in steps)
    report.append({"workflow": name, "passed": passed, "seconds": time.monotonic() - began, "checks": steps})
    print(("PASS " if passed else "FAIL ") + name, flush=True)


def catalog(client):
    definitions = {d["name"]: d for d in client.rpc("tools/list", {})["tools"]}
    require(TOOL in definitions, "Real HTTP MCP discovery is missing " + TOOL)
    definition = definitions[TOOL]
    fields = definition["inputSchema"]["properties"]
    require(fields["dry_run"].get("default") is False and "branch" in fields and "worktree_id" not in fields,
            "Incorrect refresh argument contract")
    require(definition["annotations"].get("readOnlyHint") is False, "Refresh is classified as a read")


def input_protection(check, context):
    root = context.root
    check("http-contract", lambda: catalog(context.client))
    source = root / "ontologies/external.ttl"
    ignored = root / "ignored/ignored.ttl"
    outside = root.parent / "outside.ttl"
    cases = [
        ("model-content", lambda: (root / "Model.md").write_text((root / "Model.md").read_text() + "\n"),
         "protected_model_source", "Model.md"),
        ("model-removal", lambda: (root / "Model.md").unlink(), "protected_model_source", "Model.md"),
        ("model-rename", lambda: (root / "Model.md").rename(root / "Renamed.md"), "protected_model_source", "Model.md"),
        ("model-mode", lambda: (root / "Model.md").chmod((root / "Model.md").stat().st_mode ^ 0o111),
         "protected_model_source", "Model.md"),
        ("new-model", lambda: (root / "New.md").write_text("# Elements\n\n### New Capability\n\nNew.\n\n#### Metadata\n  * type: capability\n"),
         "protected_model_source", "New.md"),
        ("new-empty-model", lambda: (root / "Empty.md").write_text("# Elements\n"), "protected_model_source", "Empty.md"),
        ("new-single-element", lambda: (root / "Single.md").write_text("# Element\n"), "protected_model_source", "Single.md"),
        ("documentation-conversion", lambda: (root / "README.md").write_text("# Elements\n"), "protected_model_source", "README.md"),
        ("consumed-semantic-source-and-dual-role", lambda: source.write_text(source.read_text() + "\n"),
         "semantic_source_changed", "ontologies/external.ttl"),
        ("semantic-source-removal", lambda: source.unlink(), "semantic_source_changed", "ontologies/external.ttl"),
        ("semantic-resolution-shadow", lambda: (root / "external.ttl").write_bytes(source.read_bytes()),
         "semantic_resolution_changed", "external.ttl"),
        ("newly-included-model-change", lambda: (
            (root / ".reqvireignore").write_text("ignored/**\n"),
            (root / "excluded/Hidden.md").write_text((root / "excluded/Hidden.md").read_text() + "\n")),
         "protected_model_source", "excluded/Hidden.md"),
        ("missing-evidence", lambda: (root / "impl.rs").rename(root / "moved.rs"), "missing_relation_target", "impl.rs"),
    ]
    for name, edit, reason, path in cases:
        def attempt(edit=edit, reason=reason, path=path):
            edit()
            commit(root)
            context.blocked(reason, path)
        check(name, lambda attempt=attempt: context.variation(attempt))
    for label, path, expected in [("ignored-semantic-drift", ignored, "ignored/ignored.ttl"),
                                   ("outside-semantic-drift", outside, None)]:
        def attempt(path=path, expected=expected):
            path.write_bytes(path.read_bytes() + b"\n")
            context.blocked("semantic_source_changed", expected)
        check(label, lambda attempt=attempt: context.variation(attempt))


def git_guards(check, context):
    for state in ("staged", "unstaged", "untracked"):
        def attempt(state=state):
            path = context.root / ("extra.txt" if state == "untracked" else "impl.rs")
            path.write_text("dirty\n")
            if state == "staged":
                git(context.root, "add", "impl.rs")
            context.blocked("dirty_worktree")
        check("dirty-" + state, lambda attempt=attempt: context.variation(attempt))

    def branch():
        git(context.root, "checkout", "-qb", "external-test")
        context.blocked("branch_changed")
    check("external-branch-switch", lambda: context.variation(branch, branch_changed=True))


def lifecycle(check, context):
    root = context.root

    def preview():
        old = context.head
        (root / "impl.rs").write_text("// previewed\n")
        head = commit(root)
        before = physical(root)
        data = context.refresh(dry_run=True)
        require(data["outcome"] == "checked" and data.get("can_refresh") is True
                and data.get("accepted_head") == old and data.get("observed_head") == head, str(data))
        require(physical(root) == before, "Preview changed physical state")
        model = root / "Model.md"
        model.write_text(model.read_text().replace("explicit traceability", "changed traceability"))
        commit(root)
        context.blocked("protected_model_source", "Model.md")
    check("preview-then-trigger-guard", lambda: context.variation(preview))

    def allowed():
        old, builds = context.head, context.client.counts()["builds"]
        (root / "impl.rs").write_text("// changed implementation\n")
        (root / "README.md").write_text("# Project Notes\n\n" + chr(96) * 3 + "markdown\n# Elements\n" + chr(96) * 3 + "\n")
        (root / "unreferenced.ttl").write_text(EXTERNAL)
        (root / "excluded").mkdir(exist_ok=True)
        (root / "excluded/Example.md").write_text("# Elements\n")
        head = commit(root)
        before = physical(root)
        data = context.refresh()
        require(data["outcome"] == "refreshed" and data.get("can_refresh") is True
                and data.get("previous_head") == old and data.get("accepted_head") == head, str(data))
        require(physical(root) == before, "Refresh changed Git/files")
        require(original(context.client) == context.ident, "Refresh replaced the context")
        require(context.client.counts()["builds"] == builds + 1, "Eligible refresh did not use normal validated loading")
        require(context.refresh()["outcome"] == "unchanged", "Repeated refresh is not idempotent")
        require(context.client.counts()["builds"] == builds + 1, "No-op refresh rebuilt the model")
        (root / "impl.rs").write_text("// amended implementation\n")
        amended = commit(root, amend=True)
        data = context.refresh()
        require(data["outcome"] == "refreshed" and data["accepted_head"] == amended, "Same-branch amendment rejected")
        require(context.client.revision() == context.revision, "Refresh changed semantic model revision")
    check("ordinary-edits-amend-and-noop", lambda: context.variation(allowed))

    def current_policy():
        policy = root / ".reqvireignore"
        scope = root / "Scope.md"
        ignored = root / "ignored/ignored.ttl"
        (root / ".gitignore").write_text("ordinary/**\n")
        policy.write_text("excluded/**\nignored/**\nScope.md\nontologies/ignored.md\n")
        scope.write_text(scope.read_text().replace("Policy scoped", "Policy changed"))
        ignored.write_bytes(ignored.read_bytes() + b"\n")
        head = commit(root)
        before = physical(root)
        preview = context.refresh(dry_run=True)
        require(preview["outcome"] == "checked" and preview["accepted_head"] == context.head, str(preview))
        data = context.refresh()
        require(data["outcome"] == "refreshed" and data["accepted_head"] == head, str(data))
        require(physical(root) == before, "Policy refresh changed Git/files")
        context.revision = context.client.revision()
        context.external[ignored] = ignored.read_bytes()
        model = str(context.client.tool("reqvire.search", branch=context.branch))
        require("Scope Policy Capability" not in model and "Refresh Ignored Ontology" not in model,
                "Reload used stale ignore rules/owners")
        excluded = context.client.raw_tool("reqvire.add_element", branch=context.branch,
            file="ordinary/New.md", dry_run=True,
            content="### Excluded Capability\n\nExcluded.\n\n#### Metadata\n  * type: capability\n")
        require(excluded.get("isError") is True, "Subsequent MCP operation used stale ignore rules")

        # Policy-only re-inclusion uses normal loading; no model file changed.
        policy.write_text("excluded/**\nignored/**\n")
        (root / ".gitignore").unlink()
        head = commit(root)
        data = context.refresh()
        require(data["outcome"] == "refreshed" and data["accepted_head"] == head, str(data))
        context.revision = context.client.revision()
        model = str(context.client.tool("reqvire.search", branch=context.branch))
        require("Scope Policy Capability" in model and "Refresh Ignored Ontology" in model,
                "Policy-only re-inclusion did not reload current membership")
        scope.write_text(scope.read_text() + "\n")
        commit(root)
        context.blocked("protected_model_source", "Scope.md")
    check("current-exclusions-and-owner-eligibility", lambda: context.variation(current_policy))

    def mutation_and_pending():
        # Last main-session check: this is a real accepted model mutation and
        # needs candidate validation. Its pending state is intentionally retained.
        head = context.head
        context.client.tool("reqvire.add_element", branch=context.branch, file="Model.md", content="""### Post Refresh Capability

The refreshed context remains writable.

#### Metadata
  * type: capability
""")
        revision = context.client.revision()
        require(revision != context.revision and git(root, "rev-parse", "HEAD") == head,
                "Commit-disabled mutation lost accepted model/HEAD")
        context.revision = revision
        require("Post Refresh Capability" in str(context.client.tool("reqvire.search", branch=context.branch)),
                "Mutation did not publish accepted content")
        context.blocked("pending_changes")
    check("mutation-resumes-and-pending-blocks", mutation_and_pending)


def main_http():
    names = ["http-input-protection", "http-git-state-guards", "http-refresh-lifecycle"]
    try:
        root, paths = project("http-shared", semantic=True)
        with server(root, "http-shared") as client:
            context = LoadedContext(client, root, original(client), paths)
            pid = client.process.pid
            builds = client.counts()["builds"]
            metrics["http_initial_model_builds"] = builds
            workflow(names[0], lambda check: input_protection(check, context))
            workflow(names[1], lambda check: git_guards(check, context))
            metrics["http_model_builds_after_rejection_checks"] = client.counts()["builds"]
            workflow(names[2], lambda check: (
                check("rejection-checks-reuse-loaded-model",
                      lambda: require(client.counts()["builds"] == builds,
                                      "Read/rejection variations rebuilt the accepted model")),
                lifecycle(check, context),
                check("same-http-process",
                      lambda: require(client.process.pid == pid and client.process.poll() is None,
                                      "HTTP workflows did not reuse their server")),
            ))
            metrics["http_model_builds_after_accepted_mutation"] = client.counts()["builds"]
    except Exception:
        error = traceback.format_exc()
        for name in names[len([r for r in report if r["workflow"] in names]):]:
            workflow(name, lambda check: check("session-setup", lambda: require(False, error)))


def stdio(check):
    root, _ = project("stdio")
    metrics["server_starts"] += 1
    with transports.Stdio(BINARY, root, OUTPUT / "stdio", "--enable-mutations") as client:
        initial = client.rpc("tools/call", {"name": "reqvire.branch.list", "arguments": {}})
        ident = next(r["branch"] for r in initial["structuredContent"]["worktrees"] if r.get("original"))

        def adopt():
            (root / "impl.rs").write_text("// updated over stdio\n")
            head = commit(root)
            before = physical(root)
            result = client.rpc("tools/call", {"name": TOOL, "arguments": {"branch": "main"}})
            data = result.get("structuredContent", {})
            require(data.get("outcome") == "refreshed" and data.get("accepted_head") == head, str(result))
            require(physical(root) == before, "stdio refresh changed Git/files")
        check("stdio-refresh", adopt)


def automatic(check):
    root, _ = project("automatic")
    with server(root, "automatic", commits=True) as client:
        context = LoadedContext(client, root, original(client))

        def adopt_and_write():
            (root / "impl.rs").write_text("// external commit before automatic model commit\n")
            head = commit(root)
            before = physical(root)
            data = context.refresh()
            require(data["outcome"] == "refreshed" and data["accepted_head"] == head, str(data))
            require(physical(root) == before and client.revision() == context.revision,
                    "Refresh created a commit or changed the model")
            data = client.tool("reqvire.add_element", branch=context.branch, file="Model.md", content="""### Automatic Refresh Capability

Automatic model commits follow the refreshed branch tip.

#### Metadata
  * type: capability
""")
            require(data.get("commit") == git(root, "rev-parse", "HEAD") and git(root, "rev-parse", "HEAD^") == head,
                    "Later automatic commit was not parented by refreshed HEAD")
        check("automatic-commit-refresh", adopt_and_write)


def explorer(check):
    root, _ = project("explorer")
    with server(root, "explorer", mode="serve") as client:
        ident = original(client)
        child = client.tool("reqvire.branch.create", branch="secondary", base_ref="main")
        child_root = Path(child["workspace_root"])
        # Reuse the two contexts required by browser selection for backend
        # isolation as well; do not start a separate isolation server/worktree.
        context = LoadedContext(client, child_root, child["branch"])

        def isolation():
            (child_root / "impl.rs").write_text("// secondary only\n")
            head = commit(child_root)
            # Branch refs are shared by Git worktrees. Observe them after the
            # external commit so this checks the refresh's own effects.
            before = physical(root)
            data = context.refresh()
            require(data["accepted_head"] == head and physical(root) == before, "Refresh crossed contexts")
            require(client.tool("reqvire.workspace_status", branch="main")["context"]["writes_available"] is True,
                    "Another context became unavailable")
        check("selected-context-isolation", lambda: context.variation(isolation))

        def browser():
            result = subprocess.run(["node", str(SUITE / "browser-refresh.mjs"), client.base, ident,
                                     child["branch"], str(root), str(child_root),
                                     str(OUTPUT / "explorer/browser-profile")],
                                    capture_output=True, text=True, timeout=90)
            (OUTPUT / "explorer/browser.txt").write_text(result.stdout + result.stderr)
            require(result.returncode == 0, result.stdout + result.stderr)
        check("head-selection-and-refresh-control", browser)


main_http()
workflow("stdio-refresh", stdio)
workflow("automatic-commit-refresh", automatic)
workflow("explorer-head-selection-and-refresh", explorer)
metrics["seconds"] = time.monotonic() - started
metrics["workflows"] = len(report)
metrics["checks"] = sum(len(r["checks"]) for r in report)
(OUTPUT / "report.json").write_text(json.dumps({"metrics": metrics, "workflows": report}, indent=2) + "\n")
raise SystemExit(any(not r["passed"] for r in report))
