#!/bin/bash
# Semantic assertions shared across the CLI and typed HTTP MCP interfaces.
set -uo pipefail
TEST_SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
source "$TEST_SCRIPT_DIR/helpers.sh"
trap stop_mcp EXIT
BACKEND="$1"
CASE_NAME="$2"

case_status() {
    fixture status yes flat
    prepare_backend
    local before
    before=$(head_of "$OWNER")
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture source'
    status_projection "$CASE_DIR/status.json"
    assert_project "$CASE_DIR/status.json" vendor-product/website "$PIN_A" "$PIN_A"
    assert_json "$CASE_DIR/status.json.projection" '.repositories_dir == "vendor-product" and (.projects|length) == 1'
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$before" "$(head_of "$OWNER")" 'status changed root HEAD'
}

case_add() {
    fixture add yes flat
    prepare_backend
    local before args
    before=$(head_of "$OWNER")
    args=$(jq -n --arg url "file://$CASE_DIR/remotes/docs.git" \
        '{url:$url,path:"nested/docs",ref:"refs/heads/main"}')
    ok_action add "$args" "$CASE_DIR/add.json"
    status_projection "$CASE_DIR/status.json"
    assert_project "$CASE_DIR/status.json" vendor-product/nested/docs "$DOCS_PIN" "$DOCS_PIN"
    assert_equal "$DOCS_PIN" "$(head_of "$ROOT/vendor-product/nested/docs")" 'wrong added revision'
    assert_equal "$before" "$(head_of "$OWNER")" 'add created a root commit'
    printf 'ordinary application content\n' > "$CASE_DIR/keep.expected"
    assert_file "$CASE_DIR/keep.expected" "$ROOT/repositories/keep.txt"
}

case_remove() {
    fixture remove yes flat
    prepare_backend
    local before
    before=$(head_of "$OWNER")
    cp "$COMPONENT/code.txt" "$CASE_DIR/code.before" || die 'Cannot capture component'
    ok_action remove '{"path":"website"}' "$CASE_DIR/remove.json"
    status_projection "$CASE_DIR/status.json"
    assert_json "$CASE_DIR/status.json.projection" '.repositories_dir == "vendor-product" and .projects == []'
    assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'remove changed/deleted checkout history'
    assert_file "$CASE_DIR/code.before" "$COMPONENT/code.txt"
    assert_equal "$before" "$(head_of "$OWNER")" 'remove created a commit'
}

case_sync() {
    local prepared=no
    [[ "$BACKEND" != mcp ]] || prepared=yes
    fixture sync "$prepared" flat
    prepare_backend
    local before
    before=$(head_of "$OWNER")
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture source'
    ok_action sync '{}' "$CASE_DIR/sync.json"
    assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'sync followed ref instead of pin'
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$before" "$(head_of "$OWNER")" 'sync created a commit'
    status_projection "$CASE_DIR/status.json"
    assert_project "$CASE_DIR/status.json" vendor-product/website "$PIN_A" "$PIN_A"
}

case_latest() {
    fixture latest yes flat
    prepare_backend
    local before
    before=$(head_of "$OWNER")
    sed "s/$PIN_A/$PIN_B/g" "$ROOT/reqvire-workspace.toml" > "$CASE_DIR/after.expected.toml"
    ok_action sync '{"latest":true}' "$CASE_DIR/latest.json"
    assert_equal "$PIN_B" "$(head_of "$COMPONENT")" 'latest did not resolve configured ref'
    assert_file "$CASE_DIR/after.expected.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$before" "$(head_of "$OWNER")" 'latest created a commit'
    status_projection "$CASE_DIR/status.json"
    assert_project "$CASE_DIR/status.json" vendor-product/website "$PIN_B" "$PIN_B"
}

case_initial_ref() {
    fixture initial-ref no flat
    sed '/^commit = /d' "$ROOT/reqvire-workspace.toml" > "$CASE_DIR/unresolved.toml"
    cp "$CASE_DIR/unresolved.toml" "$ROOT/reqvire-workspace.toml" || die 'Cannot write initial declaration'
    ok_action sync '{}' "$CASE_DIR/sync.json"
    assert_equal "$PIN_B" "$(head_of "$COMPONENT")" 'initial sync did not resolve ref'
    status_projection "$CASE_DIR/status.json"
    assert_project "$CASE_DIR/status.json" vendor-product/website "$PIN_B" "$PIN_B"
}

case_freeze() {
    fixture freeze yes flat
    prepare_backend
    local before
    before=$(head_of "$OWNER")
    commit_component_change
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture source'
    sed "s/$PIN_A/$PIN_C/g" "$CASE_DIR/before.toml" > "$CASE_DIR/after.expected.toml"
    ok_action freeze '{"dry_run":true}' "$CASE_DIR/preview.json"
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    assert_json "$CASE_DIR/preview.json" '[..|strings] | any(contains($pin)) and any(contains($head))' \
        --arg pin "$PIN_A" --arg head "$PIN_C"
    ok_action freeze '{}' "$CASE_DIR/freeze.json"
    assert_file "$CASE_DIR/after.expected.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$before" "$(head_of "$OWNER")" 'freeze created a root commit'
    assert_equal "$PIN_C" "$(head_of "$COMPONENT")" 'freeze changed component HEAD'
    ok_action freeze '{}' "$CASE_DIR/repeat.json"
    assert_file "$CASE_DIR/after.expected.toml" "$ROOT/reqvire-workspace.toml"
    status_projection "$CASE_DIR/status.json"
    assert_project "$CASE_DIR/status.json" vendor-product/website "$PIN_C" "$PIN_C"
}

case_dirty_freeze() {
    fixture dirty-freeze yes flat
    prepare_backend
    require_operation freeze
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture source'
    printf 'uncommitted change\n' > "$COMPONENT/code.txt"
    if workspace_call freeze '{}' "$CASE_DIR/rejected.json"; then
        die 'freeze accepted uncommitted component changes'
    fi
    if ! rg -qi 'dirty|uncommitted' "$CASE_DIR/rejected.json" "$CASE_DIR/rejected.json.stderr" \
        "$CASE_DIR/rejected.json.raw" 2>/dev/null; then
        die 'freeze failed without a dirty-component diagnostic'
    fi
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'rejected freeze changed HEAD'
}

case_sync_rejection() {
    fixture "$CASE_NAME" yes flat
    prepare_backend
    require_operation sync
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture manifest'
    local expected="$PIN_A" pattern='dirty|uncommitted'
    if [[ "$CASE_NAME" == diverged-sync ]]; then
        commit_component_change
        expected="$PIN_C"
        pattern='diverg|unrecorded|conflict'
    else
        printf 'preserve uncommitted content\n' > "$COMPONENT/code.txt"
    fi
    if workspace_call sync '{"latest":true}' "$CASE_DIR/rejected.json"; then
        die 'sync overwrote conflicting component work'
    fi
    rg -qi "$pattern" "$CASE_DIR/rejected.json" "$CASE_DIR/rejected.json.stderr" \
        "$CASE_DIR/rejected.json.raw" 2>/dev/null || die 'Missing sync rejection diagnostic'
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$expected" "$(head_of "$COMPONENT")" 'Rejected sync changed component HEAD'
    if [[ "$CASE_NAME" == dirty-sync ]]; then
        printf 'preserve uncommitted content\n' > "$CASE_DIR/code.expected"
        assert_file "$CASE_DIR/code.expected" "$COMPONENT/code.txt"
    fi
}

case_latest_without_ref() {
    fixture latest-without-ref yes flat
    sed '/^ref = /d' "$ROOT/reqvire-workspace.toml" > "$CASE_DIR/without-ref.toml"
    cp "$CASE_DIR/without-ref.toml" "$ROOT/reqvire-workspace.toml" || die 'Cannot prepare pin-only declaration'
    commit_all "$OWNER" pin-only
    prepare_backend
    require_operation sync
    if workspace_call sync '{"latest":true}' "$CASE_DIR/rejected.json"; then
        die 'Latest sync invented a tracking ref'
    fi
    rg -qi 'tracking ref|configured.*ref' "$CASE_DIR/rejected.json" "$CASE_DIR/rejected.json.stderr" \
        "$CASE_DIR/rejected.json.raw" 2>/dev/null || die 'Missing tracking-ref diagnostic'
    assert_file "$CASE_DIR/without-ref.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'Missing-ref rejection changed HEAD'
}

case_shared_ownership() {
    fixture shared-ownership yes flat
    prepare_backend
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture manifest'
    if "$REQVIRE_BIN" --workspace "$ROOT" workspace freeze --json \
        > "$CASE_DIR/rejected.json" 2> "$CASE_DIR/rejected.stderr"; then
        die 'Standalone freeze bypassed the active MCP owner'
    fi
    rg -qi 'owned|busy|lock' "$CASE_DIR/rejected.json" "$CASE_DIR/rejected.stderr" || die 'Missing ownership diagnostic'
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    ok_action freeze '{"dry_run":true}' "$CASE_DIR/owner-preview.json"
}

case_rejected_composition() {
    fixture rejected-composition yes flat
    cat >> "$COMPONENT/Model.md" <<'EOF'

### Website Contract

The website contract remains required by the root consumer.

#### Metadata
  * type: specification

#### Relations
  * define: [Website Requirement](#website-requirement)
---
EOF
    commit_all "$COMPONENT" contract
    PIN_C=$(head_of "$COMPONENT")
    sed "s/$PIN_A/$PIN_C/g" "$ROOT/reqvire-workspace.toml" > "$CASE_DIR/pinned.toml"
    cp "$CASE_DIR/pinned.toml" "$ROOT/reqvire-workspace.toml" || die 'Cannot pin contract'
    cat >> "$ROOT/Model.md" <<'EOF'

### Root Consumer

The system SHALL consume the website contract.

#### Metadata
  * type: requirement

#### Contract References
  * [Website Contract](vendor-product/website/Model.md#website-contract)

#### Relations
  * specify: [Root Capability](#root-capability)
---
EOF
    commit_all "$OWNER" consumer
    prepare_backend
    status_projection "$CASE_DIR/before.json"
    require_operation remove
    if workspace_call remove '{"path":"website"}' "$CASE_DIR/rejected.json"; then
        die 'MCP admitted a composition with a missing contract'
    fi
    assert_file "$CASE_DIR/pinned.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$PIN_C" "$(head_of "$COMPONENT")" 'Rejected removal changed component history'
    status_projection "$CASE_DIR/after.json"
    assert_equal "$(jq -r '.context.model_revision' "$CASE_DIR/before.json")" \
        "$(jq -r '.context.model_revision' "$CASE_DIR/after.json")" 'Rejected removal replaced accepted model'
    assert_json "$CASE_DIR/after.json" '.context.writes_available == true and .context.pending_changes == []'
    mcp_rpc tools/call '{"name":"reqvire.read_element","arguments":{"branch":"main","name":"Root Consumer"}}' \
        "$CASE_DIR/accepted-read.json" || die 'Accepted read failed'
    assert_json "$CASE_DIR/accepted-read.json" \
        '.error == null and .result.isError != true and ([.result.structuredContent|..|strings] | index("Root Consumer") != null)'
}

case_partial_sync() {
    fixture partial-sync yes flat
    cat >> "$ROOT/reqvire-workspace.toml" <<EOF

[[repositories]]
path = "zz-docs"
url = "file://$CASE_DIR/remotes/docs.git"
ref = "refs/heads/missing-ref"
commit = "$DOCS_PIN"
EOF
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture source'
    require_operation sync
    if workspace_call sync '{"latest":true}' "$CASE_DIR/partial.json"; then
        die 'Partial sync failure was reported as success'
    fi
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$PIN_B" "$(head_of "$COMPONENT")" 'Confirmed first preparation was lost'
    assert_json "$CASE_DIR/partial.json" \
        'any(.outcomes[]; .path == "vendor-product/website" and .outcome == "prepared" and .commit == $pin) and any(.outcomes[]; .path == "vendor-product/zz-docs" and .outcome == "failed")' --arg pin "$PIN_B"
    sed 's|refs/heads/missing-ref|refs/heads/main|' "$ROOT/reqvire-workspace.toml" > "$CASE_DIR/retry.toml"
    cp "$CASE_DIR/retry.toml" "$ROOT/reqvire-workspace.toml" || die 'Cannot repair requested ref'
    ok_action sync '{"latest":true}' "$CASE_DIR/retry.json"
    assert_equal "$DOCS_PIN" "$(head_of "$ROOT/vendor-product/zz-docs")" 'Retry did not reuse prepared checkout'
}

case_selection() {
    fixture selection yes nested
    require_operation status
    "$REQVIRE_BIN" --workspace "$ROOT" workspace status --json > "$CASE_DIR/explicit.json" \
        2> "$CASE_DIR/explicit.stderr" || die 'Explicit relative workspace selection failed'
    (cd "$ROOT/client" && "$REQVIRE_BIN" workspace status --json) > "$CASE_DIR/ancestor.json" \
        2> "$CASE_DIR/ancestor.stderr" || die 'Ancestor native discovery failed'
    for file in "$CASE_DIR/explicit.json" "$CASE_DIR/ancestor.json"; do
        assert_json "$file" '.workspace_root == $root and .repositories_dir == "vendor-product"' --arg root "$ROOT"
        assert_project "$file" vendor-product/website "$PIN_A" "$PIN_A"
    done
    (cd "$COMPONENT" && "$REQVIRE_BIN" workspace status --json) > "$CASE_DIR/component.json" \
        2> "$CASE_DIR/component.stderr" || die 'Component single-repository status failed'
    assert_json "$CASE_DIR/component.json" '.workspace_root == $root and .projects == []' --arg root "$COMPONENT"
}

case_single() {
    fixture single no flat
    git_do -C "$OWNER" rm -q reqvire-workspace.toml
    commit_all "$OWNER" single
    status_projection "$CASE_DIR/status.json"
    assert_json "$CASE_DIR/status.json.projection" '.repositories_dir == null and .projects == []'
    [[ ! -e "$ROOT/reqvire-workspace.toml" ]] || die 'Single repository synthesized a manifest'
}

case_invalid_manifest() {
    fixture invalid-manifest yes flat
    require_operation status
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/valid.toml"
    sed 's/version = 1/version = 999/' "$ROOT/reqvire-workspace.toml" > "$CASE_DIR/invalid.toml"
    cp "$CASE_DIR/invalid.toml" "$ROOT/reqvire-workspace.toml" || die 'Cannot write invalid source'
    if workspace_call status '{}' "$CASE_DIR/rejected.json"; then
        die 'Invalid native manifest silently fell back to single-repository mode'
    fi
    if ! rg -qi 'version|manifest|toml' "$CASE_DIR/rejected.json" "$CASE_DIR/rejected.json.stderr"; then
        die 'Missing invalid-manifest diagnostic'
    fi
    assert_file "$CASE_DIR/invalid.toml" "$ROOT/reqvire-workspace.toml"
    local key
    for key in include fragments overlays; do
        { printf '%s = ["missing.toml"]\n' "$key"; cat "$CASE_DIR/valid.toml"; } > "$ROOT/reqvire-workspace.toml"
        cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/unsupported.toml"
        if workspace_call status '{}' "$CASE_DIR/splitting.json"; then
            die 'Unsupported native splitting was silently accepted'
        fi
        rg -q 'Unsupported workspace key' "$CASE_DIR/splitting.json" "$CASE_DIR/splitting.json.stderr" || die 'Missing unsupported-configuration diagnostic'
        assert_file "$CASE_DIR/unsupported.toml" "$ROOT/reqvire-workspace.toml"
        assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'Rejected configuration changed component HEAD'
    done
}

case_migrate() {
    legacy_fixture migrate
    prepare_backend
    local args before
    before=$(head_of "$OWNER")
    cp "$ROOT/manifests/default.xml" "$CASE_DIR/source.before.xml" || die 'Cannot capture XML'
    args='{"from":"repo"}'
    if [[ "$BACKEND" == mcp ]]; then
        args='{"from":"repo","source":"manifests/default.xml"}'
    fi
    ok_action migrate "$(jq '. + {dry_run:true}' <<< "$args")" "$CASE_DIR/preview.json"
    [[ ! -e "$ROOT/reqvire-workspace.toml" ]] || die 'Migration preview wrote TOML'
    assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'Migration preview changed checkout'
    ok_action migrate "$args" "$CASE_DIR/migrate.json"
    [[ -f "$ROOT/reqvire-workspace.toml" ]] || die 'Migration did not write native root manifest'
    assert_file "$CASE_DIR/source.before.xml" "$ROOT/manifests/default.xml"
    assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'Migration did not synchronize XML pin'
    assert_equal "$before" "$(head_of "$OWNER")" 'Migration created a commit'
    status_projection "$CASE_DIR/status.json"
    assert_project "$CASE_DIR/status.json" vendor-product/website "$PIN_A" "$PIN_A"
    ok_action sync '{}' "$CASE_DIR/native-sync.json"
    assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'Post-migration native sync changed pin'
}

case_migrate_conflict() {
    legacy_fixture migrate-conflict
    require_operation migrate
    cat > "$ROOT/reqvire-workspace.toml" <<EOF
version = 1
repositories_dir = "another-parent"
repositories = []
EOF
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture existing native source'
    if workspace_call migrate '{"from":"repo","source":"manifests/default.xml"}' "$CASE_DIR/rejected.json"; then
        die 'Migration overwrote a conflicting native manifest'
    fi
    if ! rg -qi 'exist|conflict|overwrite' "$CASE_DIR/rejected.json" "$CASE_DIR/rejected.json.stderr"; then
        die 'Migration failed without a target-conflict diagnostic'
    fi
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$PIN_A" "$(head_of "$COMPONENT")" 'Rejected migration changed checkout'
}

case_migration_impact() {
    legacy_fixture migration-impact
    local base current
    base=$(head_of "$OWNER")
    ok_action migrate '{"from":"repo"}' "$CASE_DIR/migrate.json"
    sed '/The system SHALL/ s/website information/reviewed website information/' "$COMPONENT/Model.md" > "$CASE_DIR/reviewed.md"
    cp "$CASE_DIR/reviewed.md" "$COMPONENT/Model.md" || die 'Cannot prepare changed component model'
    commit_all "$COMPONENT" reviewed-model
    PIN_C=$(head_of "$COMPONENT")
    ok_action freeze '{}' "$CASE_DIR/freeze.json"
    commit_all "$OWNER" native-composition
    current=$(head_of "$OWNER")
    cp "$ROOT/reqvire-workspace.toml" "$CASE_DIR/before.toml" || die 'Cannot capture native source'
    "$REQVIRE_BIN" --workspace "$ROOT" change-impact --git-commit "$base" --json \
        > "$CASE_DIR/impact.json" 2> "$CASE_DIR/impact.stderr" || die 'XML-to-TOML migration comparison failed'
    assert_json "$CASE_DIR/impact.json" \
        '.comparison.base.central_commit == $base and .comparison.current.central_commit == $current and .comparison.base.projects[0].commit == $old and .comparison.current.projects[0].commit == $new and any(.comparison.base.manifest_inputs[]; .path == "manifests/default.xml") and any(.comparison.current.manifest_inputs[]; .path == "reqvire-workspace.toml") and ([.changed[] | .. | strings] | index("Website Requirement") != null)' \
        --arg base "$base" --arg current "$current" --arg old "$PIN_A" --arg new "$PIN_C"
    assert_file "$CASE_DIR/before.toml" "$ROOT/reqvire-workspace.toml"
    assert_equal "$current" "$(head_of "$OWNER")" 'Analysis changed root HEAD'
    assert_equal "$PIN_C" "$(head_of "$COMPONENT")" 'Analysis changed component HEAD'
}

case_subdirectory() {
    fixture subdirectory yes nested
    prepare_backend
    ok_action sync '{"latest":true}' "$CASE_DIR/latest.json"
    assert_equal "$PIN_B" "$(head_of "$COMPONENT")" 'subdirectory sync selected wrong commit'
    mcp_rpc tools/call '{"name":"reqvire.git.commit","arguments":{"branch":"main","message":"Record nested workspace pins"}}' "$CASE_DIR/commit.json" || die 'Checkpoint request failed'
    assert_json "$CASE_DIR/commit.json" '.result.isError != true'
    git -C "$OWNER" show HEAD:engineering/reqvire-workspace.toml > "$CASE_DIR/saved.toml" || die 'Checkpoint lost workspace prefix'
    assert_file "$CASE_DIR/saved.toml" "$ROOT/reqvire-workspace.toml"
    [[ ! -e "$OWNER/reqvire-workspace.toml" ]] || die 'Checkpoint created a manifest at the Git root'
    mcp_rpc tools/call '{"name":"reqvire.branch.create","arguments":{"branch":"feature/nested","from_branch":"main","base_ref":"HEAD"}}' "$CASE_DIR/create.json" || die 'Branch request failed'
    assert_json "$CASE_DIR/create.json" '.result.isError != true and (.result.structuredContent.workspace_root | endswith("/engineering"))'
    local child
    child=$(jq -r '.result.structuredContent.workspace_root' "$CASE_DIR/create.json")
    assert_equal "$PIN_B" "$(head_of "$child/vendor-product/website")" 'Child lost nested composition'
    assert_equal feature/nested "$(git -C "$child/vendor-product/website" branch --show-current)" 'Child lost component authoring branch'
    mcp_rpc tools/call '{"name":"reqvire.branch.list","arguments":{}}' "$CASE_DIR/list.json" || die 'Branch inventory request failed'
    assert_json "$CASE_DIR/list.json" '.result.isError != true and .result.structuredContent.original_branch == "main" and ([.result.structuredContent.worktrees[] | select(.owned == true)] | length == 2) and all(.result.structuredContent.worktrees[]; .workspace_root | endswith("/engineering"))'
    mcp_rpc tools/call '{"name":"reqvire.branch.close","arguments":{"branch":"feature/nested"}}' "$CASE_DIR/close.json" || die 'Close request failed'
    assert_json "$CASE_DIR/close.json" '.result.isError != true'
    mcp_rpc tools/call '{"name":"reqvire.branch.open","arguments":{"branch":"feature/nested"}}' "$CASE_DIR/open.json" || die 'Open request failed'
    assert_json "$CASE_DIR/open.json" '.result.isError != true and (.result.structuredContent.workspace_root | endswith("/engineering"))'
    assert_equal "$PIN_B" "$(head_of "$ROOT/vendor-product/website")" 'Branch lifecycle changed original checkout'
}

case_discovery() {
    fixture discovery yes flat
    prepare_backend
    for operation in status add remove sync freeze migrate; do
        require_operation "$operation"
    done
    assert_json "$CASE_DIR/tools.json" 'all(.result.tools[]; .name != "reqvire.workspace.update")'
}

case_readonly() {
    fixture readonly yes flat
    start_mcp readonly
    require_operation status
    for name in add remove sync freeze migrate; do
        assert_json "$CASE_DIR/tools.json" 'all(.result.tools[]; .name != $name)' \
            --arg name "reqvire.workspace.$name"
    done
    status_projection "$CASE_DIR/status.json"
    assert_project "$CASE_DIR/status.json" vendor-product/website "$PIN_A" "$PIN_A"
}

case_boundaries() {
    CASE_DIR="$TEST_DIR/output/model-boundaries"
    mkdir -p "$CASE_DIR" || die 'Cannot prepare model check output'
    "$REQVIRE_BIN" --workspace "$REQVIRE_SOURCE_ROOT" submodels --json > "$CASE_DIR/submodels.json" ||
        die 'System model submodel analysis failed'
    assert_json "$CASE_DIR/submodels.json" \
        '.summary.total_cross_submodel_couplings == 0 and any(.submodels[]; .root_name == "Multi-repository Workspace Management and Orchestration")'
    "$REQVIRE_BIN" --workspace "$REQVIRE_SOURCE_ROOT" collect \
        'Multi-repository Workspace Management and Orchestration' --direction DOWNSTREAM --json \
        > "$CASE_DIR/collect.json" || die 'Workspace capability collection failed'
    assert_json "$CASE_DIR/collect.json" \
        'all(.items[]; (.file_path | startswith("system-model/Workspaces/")) or (.file_path | startswith("system-model/Thesaurus/")))'
    local consumer index=0
    for consumer in \
        'Website Command and Workflow Documentation' \
        'Website Assistant Integration Documentation' \
        'AI Skills Instruction Contracts' \
        'AI Skills Markdown Implementation Artifacts'; do
        index=$((index + 1))
        "$REQVIRE_BIN" --workspace "$REQVIRE_SOURCE_ROOT" collect "$consumer" \
            --direction DOWNSTREAM --json > "$CASE_DIR/consumer-$index.json" ||
            die "Cannot collect workspace guidance: $consumer"
        jq -r '[.items[] | select(.file_path == "system-model/Workspaces/Specifications.md") | .name] | unique | .[]' \
            "$CASE_DIR/consumer-$index.json" > "$CASE_DIR/consumer-$index-contracts.txt" ||
            die "Cannot project workspace guidance contracts: $consumer"
        assert_file "$TEST_SCRIPT_DIR/expected/documentation-workspace-contracts.txt" \
            "$CASE_DIR/consumer-$index-contracts.txt"
    done
}

case "$CASE_NAME" in
    status) case_status ;;
    add) case_add ;;
    remove) case_remove ;;
    sync) case_sync ;;
    latest) case_latest ;;
    initial-ref) case_initial_ref ;;
    freeze) case_freeze ;;
    dirty-freeze) case_dirty_freeze ;;
    dirty-sync|diverged-sync) case_sync_rejection ;;
    latest-without-ref) case_latest_without_ref ;;
    partial-sync) case_partial_sync ;;
    shared-ownership) case_shared_ownership ;;
    rejected-composition) case_rejected_composition ;;
    selection) case_selection ;;
    single) case_single ;;
    invalid-manifest) case_invalid_manifest ;;
    migrate) case_migrate ;;
    migrate-conflict) case_migrate_conflict ;;
    migration-impact) case_migration_impact ;;
    discovery) case_discovery ;;
    subdirectory) case_subdirectory ;;
    readonly) case_readonly ;;
    workspace-boundaries) case_boundaries ;;
    *) die "Unknown case: $CASE_NAME" ;;
esac
