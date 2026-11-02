# Workspace preparation and coordinated development

Use the same branch-selected MCP workflow for one repository or a declared
composition. Git worktrees are private implementation details.

## Native setup

The owning Git repository may contain server/client code and a shared model,
or only configuration while pinned repositories supply all model content.
Create reqvire-workspace.toml at the effective root, including a subdirectory
root when appropriate:

```toml
version = 1
repositories_dir = "external"
```

Register dependencies and record exact commits:

```bash
reqvire workspace add https://github.com/YOUR_ORG/website.git --path website --ref refs/heads/main
reqvire workspace add https://github.com/YOUR_ORG/user-docs.git --path user-docs --ref refs/heads/main
reqvire workspace status
```

Add specific root Git ignore entries for /external/website/ and
/external/user-docs/, preserving unrelated content in the chosen parent.
When the manifest is below the owning Git root, prefix those root ignore entries
with the effective workspace directory's relative path.
Commit the manifest and ignore configuration through the normal project review.
Cloning that root and running reqvire workspace sync reproduces the selected
composition before validation or MCP startup. Preparation needs no model.

Each declaration records path, URL, tracking ref and exact commit. The ref is
the configured update source; commit selects reproducible content. Never
replace the tracking ref with a feature branch merely because work starts there.
Manual declarations may omit ref for pinned sync; latest sync requires it.
A ref-only initial declaration needs explicit sync to record its exact pin
before model admission.

All native declarations belong in one reqvire-workspace.toml. Native includes,
fragments and overlays are unsupported. XML migration resolves supported includes
and defaults into that one file.

| Operation | CLI | MCP |
| --- | --- | --- |
| Inspect | workspace status | reqvire.workspace_status |
| Register | workspace add URL --path PATH --ref REF | reqvire.workspace.add |
| Unregister, keep checkout | workspace remove PATH | reqvire.workspace.remove |
| Reproduce pins | workspace sync | reqvire.workspace.sync |
| Follow refs and advance pins | workspace sync --latest | same tool, latest: true |
| Record committed HEADs | workspace freeze [--dry-run] | reqvire.workspace.freeze |
| Convert Repo XML and sync | workspace migrate --from repo [--source FILE] [--dry-run] | reqvire.workspace.migrate |

MCP workspace mutations require mutation enablement and a valid admitted
context. Inspect discovery/schema first. Configuration edits remain pending
unless automatic commits were enabled. Status and freeze stay local; explicit
registration, synchronization and migration may fetch. Preparation does not
implicitly publish branches or create PRs.

## Branches and external work

Create coordinated model branches with reqvire.branch.create, explicitly
selecting branch and base_ref; use from_branch for an admitted source when the
base is relative. The selected root base supplies its own manifest and pins.
Components get the requested named branch at their own pin. Creation alone
does not change pins or existing user checkouts.

For ordinary work outside MCP, prepare the root feature branch and synchronize
its pins. Named owners use matching component branches; detached CLI/CI owners
use detached component checkouts. Conflicting, dirty or diverged branches are
reported for normal Git resolution, without automatic reset/merge/rebase.

Commit component work first, then record and review its pins:

```bash
reqvire workspace freeze --dry-run
reqvire workspace freeze
git add reqvire-workspace.toml
git commit -m "Update product composition"
```

Standalone CLI operations cannot modify a composition owned by a live MCP
session. The owning MCP context can explicitly freeze committed component work
through full candidate validation. Resolve pending model edits and recovery
first; rejected candidates retain accepted reads. Clean externally prepared
compositions use full reqvire.branch.open admission, without a reload/adopt mode.
Artifact-only reqvire.branch.refresh cannot admit changed component selections.

MCP model checkpoints commit changed components first, update exact pins while
preserving configured refs, and commit those pins with accepted root changes
in one root commit. Publish components before root only when requested. One
explicit PR-create call creates/reuses linked changed-component PRs and the root
PR, using explicit review targets. Refresh pins after squash/rebase changes the
final component commit.

## Repo migration and impact analysis

```bash
reqvire workspace migrate --from repo --dry-run
reqvire workspace migrate --from repo
```

Migration converts selected XML to native TOML at the effective root and invokes
normal sync, reusing checkouts and their Git metadata. Preview writes nothing.
Keep existing administration in place; conversion creates no separate manifest
authoring branch or PR. Model-syntax upgrades remain the separate CLI migrate.
Unsupported XML elements and Repo local overlays fail explicitly.

Change impact analysis uses the actual PR baseline when available and one
explicit --git-commit override. Explicit HEAD reviews pending work. Ask for the
baseline when review context is unavailable, rather than assuming origin/main.
The chosen root tree supplies the manifest and component commits; a pre-conversion
tree can supply versioned XML. Missing local objects need explicit preparation.
Analysis never fetches, synchronizes or advances pins.
