#!/bin/bash
# Verifies Manifest-Based Change Impact Baselines and Cross-Repository Change
# Impact Reporting. Require real reports; missing support is never an expected
# failure. Reuse the suite's actual Repo clients, local remotes and fixture setup.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$TEST_SCRIPT_DIR/check_native_repo.sh"
composition_server_options=()

impact_commit() {
    git -C "$1" add .
    git -C "$1" -c commit.gpgSign=false commit -qm "$2"
    git -C "$1" rev-parse HEAD
}

impact_prepare() {
    native_fixture "impact-$1"
    mkdir -p "$native_case/docs"
    cp "$TEST_SCRIPT_DIR/fixtures/impact-docs.md.txt" "$native_case/docs/Model.md"
    printf 'Guide for the selected public contract.\n' > "$native_case/docs/Guide.txt"
    git -C "$native_case/docs" init -qb main
    docs_commit=$(impact_commit "$native_case/docs" 'Documentation model and page')
    git clone -q --bare "$native_case/docs" "$native_case/remotes/docs"
    { fixture_manifest; fixture_project component "$alpha"; fixture_project docs "$docs_commit"; } > "$central/reqvire-workspace.toml"
    impact_base=$(impact_commit "$central" 'Historical combined model')
    sed -i "s/$alpha/$omega/" "$central/reqvire-workspace.toml"
    impact_current=$(impact_commit "$central" 'Current combined model')
    git -C "$central" push -q "file://$native_case/remotes/central" HEAD:refs/heads/main
    (cd "$central" && "$REQVIRE_BIN" workspace sync)
    git -C "$central/repositories/component" checkout -q -B main "$omega"
    git -C "$central/repositories/docs" checkout -q -B main "$docs_commit"
    # Keep Central's web identity distinct from component file remotes. Reports
    # must not invent Central blob URLs for component-owned paths.
    git -C "$central" remote set-url origin https://github.com/reqvire-test/central.git
    (cd "$central" && "$REQVIRE_BIN" --manifest reqvire-workspace.toml validate)
    (cd "$central" && "$REQVIRE_BIN" validate)
}

impact_state() {
    local owner path
    for owner in "$central" "$central"/repositories/*; do
        [[ -d "$owner" ]] || continue
        printf 'OWNER %s\n' "$owner"
        git -C "$owner" rev-parse HEAD
        git -C "$owner" ls-files --stage
        git -C "$owner" for-each-ref --format='%(refname) %(objectname)'
        git -C "$owner" status --porcelain
        while IFS= read -r -d '' path; do
            printf 'FILE %s\n' "$path"
            if [[ -f "$owner/$path" ]]; then sha256sum "$owner/$path"; else printf 'MISSING\n'; fi
        done < <(git -C "$owner" ls-files -z --cached --others --exclude-standard)
    done

}

impact_cli() {
    local selection=$1 result
    shift
    local options=()
    [[ "$selection" == native ]] || options+=(--manifest reqvire-workspace.toml)
    impact_state > "$native_case/before-state.txt"
    if (cd "$central" && "$REQVIRE_BIN" "${options[@]}" change-impact \
            --git-commit "$impact_base" "$@") > "$native_case/impact.out" 2> "$native_case/impact.err"; then
        result=0
    else
        result=$?
    fi
    impact_state > "$native_case/after-state.txt"
    diff -u "$native_case/before-state.txt" "$native_case/after-state.txt"
    cat "$native_case/impact.out" "$native_case/impact.err" > "$native_case/impact.diagnostic"
    if [[ "$result" != 0 ]]; then cat "$native_case/impact.diagnostic" >&2; fi
    return "$result"
}

impact_graph() {
    jq -e '
      (.added == [] and .removed == [] and .relocated == []) and
      ([.changed[] | .. | objects | .name? // .target_text? // empty] |
        index("Component Specification") != null and index("API Guide Requirement") != null
        and index("Documentation Inspection") != null) and
      ([.changed[] | .. | objects | keys[]] | index("contract_references") != null) and
      ([.changed[] | .. | strings] | any(contains("repositories/docs/Guide.txt"))) and
      ([.invalidated_verifications[].target_text] == ["Documentation Inspection"])
    ' "$1"
}

impact_provenance() {
    jq -S --arg base "$impact_base" --arg current "$impact_current" \
        --arg alpha "$alpha" --arg omega "$omega" --arg docs "$docs_commit" \
        '
      .comparison | select(.workspace_mode == "repo") |
      [.base, .current] | to_entries | map(
        .key as $side | .value |
        {central: (.central_commit == (if $side == 0 then $base else $current end)),
         inputs: (.manifest_inputs == [
           {path: "reqvire-workspace.toml", commit: (if $side == 0 then $base else $current end)}]),
         projects: ([.projects[] | {name,path,commit}] == [
           {name:"component",path:"repositories/component",commit:(if $side == 0 then $alpha else $omega end)},
           {name:"docs",path:"repositories/docs",commit:$docs}]),
         owners: (all(.projects[]; .repository | type == "string" and length > 0))})
    ' "$native_case/impact.out" > "$native_case/provenance.json"
    diff -u "$TEST_SCRIPT_DIR/expected/impact-provenance.json" "$native_case/provenance.json"
    jq -e '[.. | strings | select(contains("repositories/"))] |
      all(contains("https://github.com/reqvire-test/central/blob/") | not)' "$native_case/impact.out"
}

impact_start_server() {
    local mode=$1 selection=$2 attempt options=() server_options=()
    [[ "$selection" == native ]] || options+=(--manifest reqvire-workspace.toml)
    [[ "$mode" != serve ]] || server_options+=(--enable-mcp)
    server_options+=("${composition_server_options[@]}")
    (cd "$central" && exec "$REQVIRE_BIN" "${options[@]}" "$mode" \
        --host 127.0.0.1 --port 0 --enable-mutations "${server_options[@]}") > "$native_case/server.log" 2>&1 &
    impact_pid=$!
    trap 'kill "$impact_pid" 2>/dev/null || true; wait "$impact_pid" 2>/dev/null || true' EXIT
    for ((attempt=0; attempt<200; attempt++)); do
        impact_url=$(sed -n 's@.*\(http://127\.0\.0\.1:[0-9][0-9]*/mcp\).*@\1@p' "$native_case/server.log" | head -n 1)
        if [[ -n "$impact_url" ]] && curl -fsS --max-time 1 -D "$native_case/headers" \
            -H 'Content-Type: application/json' -H 'Accept: application/json, text/event-stream' \
            -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"repo-impact-e2e","version":"1"}}}' \
            "$impact_url" > "$native_case/initialize.json" 2>/dev/null; then
            impact_session=$(awk 'tolower($1)=="mcp-session-id:" {gsub("\r", "", $2); print $2}' "$native_case/headers")
            return 0
        fi
        kill -0 "$impact_pid" 2>/dev/null || { cat "$native_case/server.log"; return 1; }
        sleep 0.1
    done
    cat "$native_case/server.log"
    return 1
}

impact_rpc() {
    local tool=$1 arguments=$2 target=$3
    jq -n --arg tool "$tool" --argjson arguments "$arguments" \
        '{jsonrpc:"2.0",id:2,method:"tools/call",params:{name:$tool,arguments:$arguments}}' > "$native_case/request.json"
    curl -fsS --max-time 30 -H 'Content-Type: application/json' \
        -H 'Accept: application/json, text/event-stream' -H "Mcp-Session-Id: $impact_session" \
        --data-binary "@$native_case/request.json" "$impact_url" > "$target"
    jq -e '.error == null and .result.isError != true and .result.structuredContent != null' "$target"
}

impact_complete() {
    impact_cli central --json
    impact_graph "$native_case/impact.out"
    cp "$native_case/impact.out" "$native_case/first-impact.json"
    impact_cli central --json
    diff -u "$native_case/first-impact.json" "$native_case/impact.out"
}
impact_provenance_case() { impact_cli central --json; impact_provenance; }
impact_text() {
    impact_cli central
    local token
    for token in 'API Guide Requirement' 'Documentation Inspection' "$alpha" "$omega" "$impact_base" "$impact_current"; do
        grep -Fq "$token" "$native_case/impact.out"
    done
}

impact_mcp() {
    local mode=${1:-mcp} selection=${2:-central} args
    impact_start_server "$mode" "$selection"
    impact_rpc reqvire.tool_contract '{}' "$native_case/tool-contract.json"
    jq -e '.result.structuredContent.tools[] | select(.name == "reqvire.change_impact") |
      .inputSchema.properties | has("git_commit") and (has("base_manifest_commit") | not)' "$native_case/tool-contract.json"
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-before.json"
    args=$(jq -n --arg commit "$impact_base" '{branch:"main",git_commit:$commit}')
    impact_state > "$native_case/before-state.txt"
    local result=0
    impact_rpc reqvire.change_impact "$args" "$native_case/impact-result.json" || result=1
    impact_state > "$native_case/after-state.txt"
    diff -u "$native_case/before-state.txt" "$native_case/after-state.txt"
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-after.json"
    jq -S '.result.structuredContent.context | {model_revision,accepted_head,pending_changes,writes_available}' \
        "$native_case/status-before.json" > "$native_case/accepted-before.json"
    jq -S '.result.structuredContent.context | {model_revision,accepted_head,pending_changes,writes_available}' \
        "$native_case/status-after.json" > "$native_case/accepted-after.json"
    diff -u "$native_case/accepted-before.json" "$native_case/accepted-after.json"
    [[ "$result" == 0 ]] || { cat "$native_case/impact-result.json"; return 1; }
    jq '.result.structuredContent | del(.context)' "$native_case/impact-result.json" > "$native_case/mcp-impact.json"
    impact_graph "$native_case/mcp-impact.json"
    impact_cli "$selection" --json
    jq -S . "$native_case/impact.out" > "$native_case/cli-normalized.json"
    jq -S . "$native_case/mcp-impact.json" > "$native_case/mcp-normalized.json"
    diff -u "$native_case/cli-normalized.json" "$native_case/mcp-normalized.json"
}

impact_native() {
    impact_cli native --json
    impact_graph "$native_case/impact.out"
    impact_provenance
    test ! -e "$central/.reqvire-composition.json"
}

impact_single_selector() {
    impact_cli central --json
    impact_graph "$native_case/impact.out"
    if impact_cli central --base-manifest-commit HEAD --json; then
        echo 'Removed second baseline selector was accepted'; return 1
    fi
    grep -Fq "unexpected argument '--base-manifest-commit'" "$native_case/impact.diagnostic"
}

impact_include() {
    # The baseline predates conversion. XML includes remain readable while
    # the current composition has exactly one native source.
    cp "$central/reqvire-workspace.toml" "$native_case/current-native.toml"
    git -C "$central" rm -q reqvire-workspace.toml
    mkdir -p "$central/parts"
    printf '<manifest><include name="parts/projects.xml"/></manifest>\n' > "$central/default.xml"
    cat > "$central/parts/projects.xml" <<EOF
<manifest>
  <remote name="local" fetch="file://$native_case/remotes/"/>
  <project name="component" path="repositories/component" remote="local" revision="$alpha"/>
  <project name="docs" path="repositories/docs" remote="local" revision="$docs_commit"/>
</manifest>
EOF
    impact_base=$(impact_commit "$central" 'Version XML baseline with included source')
    git -C "$central" rm -q default.xml parts/projects.xml
    cp "$native_case/current-native.toml" "$central/reqvire-workspace.toml"
    impact_current=$(impact_commit "$central" 'Use one native manifest after conversion')
    impact_cli central --json
    impact_graph "$native_case/impact.out"
    jq -e '.comparison |
      [.base.manifest_inputs[].path] == ["default.xml","parts/projects.xml"] and
      [.current.manifest_inputs[].path] == ["reqvire-workspace.toml"]' "$native_case/impact.out"
}

impact_bad_base() {
    local kind=$1 revision=0000000000000000000000000000000000000000 code=repo_impact_base_unavailable
    if [[ "$kind" == unlocked ]]; then
        revision=main; code=repo_impact_base_unlocked
    else
        # This commit exists only in the remote. A read must neither fetch it
        # nor mistake a remote repository for retained local Repo object storage.
        local tree
        tree=$(git -C "$native_case/remotes/component" rev-parse "$alpha^{tree}")
        revision=$(git -C "$native_case/remotes/component" commit-tree "$tree" -p "$alpha" -m 'Remote-only historical revision')
        if git -C "$central/repositories/component" cat-file -e "$revision^{commit}" 2>/dev/null; then
            echo 'Remote-only fixture commit is already available locally'; return 1
        fi
    fi
    cp "$central/reqvire-workspace.toml" "$native_case/valid-pins.toml"
    sed -i "s/$omega/$revision/" "$central/reqvire-workspace.toml"
    impact_base=$(impact_commit "$central" 'Historical invalid or unlocked project selection')
    cp "$native_case/valid-pins.toml" "$central/reqvire-workspace.toml"
    impact_commit "$central" 'Restore the valid current composition' > /dev/null
    if impact_cli central --json; then echo 'Invalid historical selection was accepted'; return 1; fi
    local token
    for token in "$code" component repositories/component "$revision"; do
        grep -Fq "$token" "$native_case/impact.diagnostic"
    done
}

impact_project() {
    local kind=$1
    if [[ "$kind" == move ]]; then
        impact_base=$impact_current
        git clone -q "$native_case/remotes/component" "$central/repositories/moved"
        git -C "$central/repositories/moved" checkout -q --detach "$omega"
        sed -i 's@path = "repositories/component"@path = "repositories/moved"@' "$central/reqvire-workspace.toml"
        sed -i 's@repositories/component/@repositories/moved/@g' "$central/Model.md" "$central/repositories/docs/Model.md"
        docs_commit=$(impact_commit "$central/repositories/docs" 'Follow relocated contract')
        # Rewrite only the docs project pin; preserve the component revision.
        sed -i "/name = \\\"docs\\\"/,/commit =/s/commit = \\\"[^\\\"]*\\\"/commit = \\\"$docs_commit\\\"/" "$central/reqvire-workspace.toml"
    elif [[ "$kind" == remove ]]; then
        fixture_remove "$central/reqvire-workspace.toml" docs
        # Preserve local object storage while removing the retired live checkout.
        # This fixture models an existing separate Git directory, as before.
        mkdir -p "$central/.repo/projects/repositories"
        mv "$central/repositories/docs/.git" "$central/.repo/projects/repositories/docs.git"
        rm -rf "$central/repositories/docs"
    else
        # Compare a current composition containing docs with a complete base
        # composition that has no docs project, while keeping current files valid.
        cp "$central/reqvire-workspace.toml" "$native_case/current-pins.toml"
        fixture_remove "$central/reqvire-workspace.toml" docs
        impact_base=$(impact_commit "$central" 'Historical composition without docs')
        cp "$native_case/current-pins.toml" "$central/reqvire-workspace.toml"
    fi
    impact_commit "$central" 'Change project composition' > /dev/null
    (cd "$central" && "$REQVIRE_BIN" --manifest reqvire-workspace.toml validate)
    impact_cli central --json
    case "$kind" in
        add) jq -e '[.added[] | .. | objects | .name? // empty] | index("API Guide Requirement") != null' "$native_case/impact.out" ;;
        remove) jq -e 'any(.removed[]; .element_id | endswith("repositories/docs/Model.md#api-guide-requirement"))' "$native_case/impact.out" ;;
        move)
            jq -e '.added == [] and .removed == [] and .changed == []' "$native_case/impact.out"
            jq -S '[.relocated[] | select(.name | startswith("Component"))] | sort_by(.name)' \
                "$native_case/impact.out" > "$native_case/relocations.json"
            diff -u "$TEST_SCRIPT_DIR/expected/impact-relocations.json" "$native_case/relocations.json"
            ;;
    esac
}

impact_pending() {
    impact_start_server mcp central
    local args
    args=$(jq -n --rawfile content "$TEST_SCRIPT_DIR/fixtures/addition.md.txt" \
        '{branch:"main",file:"repositories/component/Added.md",content:$content}')
    impact_rpc reqvire.add_element "$args" "$native_case/add-result.json"
    cp "$central/repositories/component/Added.md" "$native_case/accepted-added.txt"
    cp "$central/reqvire-workspace.toml" "$native_case/accepted-manifest.txt"
    printf 'External invalid model must not be imported.\n' > "$central/repositories/component/Added.md"
    printf 'External invalid manifest must not be imported.\n' > "$central/reqvire-workspace.toml"
    # Capture availability after independent external drift. The impact request
    # must preserve that state rather than hide source-guard diagnostics.
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-before.json"
    impact_state > "$native_case/before-state.txt"
    # A failed historical read must preserve accepted data and write admission.
    if impact_rpc reqvire.change_impact '{"branch":"main","git_commit":"0000000000000000000000000000000000000000"}' "$native_case/failed-impact-result.json"; then
        echo 'Unavailable historical commit unexpectedly succeeded'; return 1
    fi
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-failed.json"
    jq -S '.result.structuredContent.context | {model_revision,accepted_head,pending_changes,writes_available}' \
        "$native_case/status-before.json" > "$native_case/accepted-before.json"
    jq -S '.result.structuredContent.context | {model_revision,accepted_head,pending_changes,writes_available}' \
        "$native_case/status-failed.json" > "$native_case/accepted-failed.json"
    diff -u "$native_case/accepted-before.json" "$native_case/accepted-failed.json"
    args=$(jq -n --arg base "$impact_base" '{branch:"main",git_commit:$base}')
    local result=0
    impact_rpc reqvire.change_impact "$args" "$native_case/impact-result.json" || result=1
    impact_state > "$native_case/after-state.txt"
    diff -u "$native_case/before-state.txt" "$native_case/after-state.txt"
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-after.json"
    local phase
    for phase in before after; do
        jq -S '.result.structuredContent.context | {model_revision,accepted_head,pending_changes,writes_available}' \
            "$native_case/status-$phase.json" > "$native_case/accepted-$phase.json"
    done
    diff -u "$native_case/accepted-before.json" "$native_case/accepted-after.json"
    [[ "$result" == 0 ]] || { cat "$native_case/impact-result.json"; return 1; }
    jq -e 'any(.result.structuredContent.added[]; .element_id | endswith("repositories/component/Added.md#added-capability"))' \
        "$native_case/impact-result.json"
    cp "$native_case/accepted-added.txt" "$central/repositories/component/Added.md"
    cp "$native_case/accepted-manifest.txt" "$central/reqvire-workspace.toml"
    impact_rpc reqvire.git.commit '{"branch":"main","message":"Save accepted Repo impact fixture"}' "$native_case/commit-result.json"
    # Explicit HEAD follows the newly accepted root checkpoint, including
    # the current set of selecting inputs after Central manifest flattening.
    impact_state > "$native_case/before-saved-impact.txt"
    impact_rpc reqvire.change_impact '{"branch":"main","git_commit":"HEAD"}' "$native_case/saved-impact-result.json"
    impact_state > "$native_case/after-saved-impact.txt"
    diff -u "$native_case/before-saved-impact.txt" "$native_case/after-saved-impact.txt"
    jq -e '.result.structuredContent |
      .added == [] and .removed == [] and .changed == [] and .relocated == [] and
      .comparison.base == .comparison.current' "$native_case/saved-impact-result.json"
}

run_impact_cases() {
    local key label
    : > "$TEST_DIR/output/impact-checks.txt"
    while read -r key label; do
        if bash "$TEST_SCRIPT_DIR/check_change_impact.sh" "$key" > "$TEST_DIR/output/impact-$key.log" 2>&1; then
            printf 'PASS %s\n' "$label" >> "$TEST_DIR/output/impact-checks.txt"
        else
            printf 'FAIL %s\n' "$label" >> "$TEST_DIR/output/impact-checks.txt"
            cat "$TEST_DIR/output/impact-$key.log" >&2
            status=1
        fi
    done <<'CASES'
complete cli/repo-impact-complete-graphs
provenance cli/repo-impact-provenance-and-links
text cli/repo-impact-text
mcp mcp/repo-impact-parity-and-isolation
serve serve/repo-impact-parity-and-isolation
foreign_manifest cli/repo-impact-root-source-without-association
foreign_mcp mcp/repo-impact-root-source-without-association
foreign_serve serve/repo-impact-root-source-without-association
selector cli/repo-impact-single-baseline-selector
include cli/repo-impact-historical-includes
missing cli/repo-impact-missing-base-commit
unlocked cli/repo-impact-unlocked-base-revision
add cli/repo-impact-added-project
remove cli/repo-impact-removed-project
move cli/repo-impact-project-path-relocation
pending mcp/repo-impact-accepted-pending-snapshot
CASES
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    set -euo pipefail
    impact_prepare "$1"
    case "$1" in
        complete) impact_complete ;;
        provenance) impact_provenance_case ;;
        text) impact_text ;;
        mcp|serve) impact_mcp "$1" ;;
        foreign_manifest) impact_native ;;
        foreign_mcp) impact_mcp mcp native ;;
        foreign_serve) impact_mcp serve native ;;
        selector) impact_single_selector ;;
        include) impact_include ;;
        missing|unlocked) impact_bad_base "$1" ;;
        add|remove|move) impact_project "$1" ;;
        pending) impact_pending ;;
    esac
fi
