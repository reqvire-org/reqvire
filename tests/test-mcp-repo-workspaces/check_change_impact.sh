#!/bin/bash
# Verifies Manifest-Based Change Impact Baselines and Cross-Repository Change
# Impact Reporting. Require real reports; missing support is never an expected
# failure. Reuse the suite's actual Repo clients, local remotes and fixture setup.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$TEST_SCRIPT_DIR/check_native_repo.sh"

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
    mkdir -p "$central/parts" "$(dirname "$native")/parts"
    printf '<manifest><include name="parts/projects.xml"/></manifest>\n' > "$central/default.xml"
    cat > "$central/parts/projects.xml" <<EOF
<manifest>
  <remote name="local" fetch="file://$native_case/remotes/"/>
  <project name="component" path="repositories/component" remote="local" revision="$alpha"/>
  <project name="docs" path="repositories/docs" remote="local" revision="$docs_commit"/>
</manifest>
EOF
    impact_base=$(impact_commit "$central" 'Historical combined model')
    cp "$central/default.xml" "$native"
    cp "$central/parts/projects.xml" "$(dirname "$native")/parts/projects.xml"
    manifest_base=$(impact_commit "$(dirname "$native")" 'Historical manifest inputs')
    sed -i "s/$alpha/$omega/" "$central/parts/projects.xml"
    impact_current=$(impact_commit "$central" 'Current combined model')
    cp "$central/parts/projects.xml" "$(dirname "$native")/parts/projects.xml"
    manifest_current=$(impact_commit "$(dirname "$native")" 'Current manifest inputs')
    (cd "$central" && repo sync -q --no-clone-bundle --no-tags)
    # Keep Central's web identity distinct from component file remotes. Reports
    # must not invent Central blob URLs for component-owned paths.
    git -C "$central" remote set-url origin https://github.com/reqvire-test/central.git
    (cd "$central" && "$REQVIRE_BIN" --manifest default.xml validate)
    (cd "$central" && "$REQVIRE_BIN" validate)
}

impact_state() {
    local owner path
    for owner in "$central" "$central"/repositories/* "$(dirname "$native")"; do
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
    # Include source-selection Repo administration, not only model inputs.
    # Repo's own command trace is observational logging, not source state.
    (cd "$central/.repo" && find . -type f ! -path './repo/*' ! -name index.lock \
        ! -path '*/logs/*' ! -name 'TRACE_FILE*' ! -name FETCH_HEAD ! -name COMMIT_EDITMSG -print0 | sort -z | xargs -0 sha256sum)
}

impact_cli() {
    local selection=$1 result
    shift
    local options=()
    [[ "$selection" == native ]] || options+=(--manifest default.xml)
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
    local prefix="${1:-}" base_manifest=$impact_base current_manifest=$impact_current
    if [[ -n "$prefix" ]]; then base_manifest=$manifest_base; current_manifest=$manifest_current; fi
    jq -S --arg base "$impact_base" --arg current "$impact_current" \
        --arg alpha "$alpha" --arg omega "$omega" --arg docs "$docs_commit" \
        --arg prefix "$prefix" --arg mb "$base_manifest" --arg mc "$current_manifest" '
      .comparison | select(.workspace_mode == "repo") |
      [.base, .current] | to_entries | map(
        .key as $side | .value |
        {central: (.central_commit == (if $side == 0 then $base else $current end)),
         inputs: (.manifest_inputs == [
           {path: ($prefix + "default.xml"), commit: (if $side == 0 then $mb else $mc end)},
           {path: ($prefix + "parts/projects.xml"), commit: (if $side == 0 then $mb else $mc end)}]),
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
    local mode=$1 selection=$2 attempt options=()
    [[ "$selection" == native ]] || options+=(--manifest default.xml)
    [[ "$mode" != serve ]] || options+=(--enable-mcp)
    (cd "$central" && exec "$REQVIRE_BIN" "${options[@]}" "$mode" \
        --host 127.0.0.1 --port 0 --enable-mutations) > "$native_case/server.log" 2>&1 &
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
    if [[ "$selection" == native ]]; then
        impact_rpc reqvire.tool_contract '{}' "$native_case/tool-contract.json"
        jq -e '.result.structuredContent.tools[] | select(.name == "reqvire.change_impact") |
          .inputSchema.properties | has("base_manifest_commit")' "$native_case/tool-contract.json"
    fi
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-before.json"
    args=$(jq -n --arg commit "$impact_base" --arg manifest "$manifest_base" --arg selection "$selection" \
        '{branch:"main",git_commit:$commit} + if $selection == "native" then {base_manifest_commit:$manifest} else {} end')
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
    local extra=()
    [[ "$selection" != native ]] || extra+=(--base-manifest-commit "$manifest_base")
    impact_cli "$selection" "${extra[@]}" --json
    jq -S . "$native_case/impact.out" > "$native_case/cli-normalized.json"
    jq -S . "$native_case/mcp-impact.json" > "$native_case/mcp-normalized.json"
    diff -u "$native_case/cli-normalized.json" "$native_case/mcp-normalized.json"
}

impact_native() {
    impact_cli native --base-manifest-commit "$manifest_base" --json
    impact_graph "$native_case/impact.out"
    impact_provenance '.repo/manifests/'
}

impact_native_required() {
    if impact_cli native --json; then echo 'Native manifest history was silently guessed'; return 1; fi
    grep -Fq base_manifest_commit_required "$native_case/impact.diagnostic"
    grep -Fq .repo/manifests/ "$native_case/impact.diagnostic"
}

impact_include() {
    local owner
    for owner in "$central" "$(dirname "$native")"; do
        mv "$owner/parts/projects.xml" "$owner/parts/current.xml"
        sed -i 's@parts/projects.xml@parts/current.xml@' "$owner/default.xml"
        impact_commit "$owner" 'Remove the historical include from the live tree' > /dev/null
    done
    impact_cli central --json
    impact_graph "$native_case/impact.out"
}

impact_bad_base() {
    local kind=$1 revision=0000000000000000000000000000000000000000 code=repo_impact_base_unavailable
    if [[ "$kind" == unlocked ]]; then revision=main; code=repo_impact_base_unlocked; fi
    cp "$central/parts/projects.xml" "$native_case/valid-pins.xml"
    sed -i "s/$omega/$revision/" "$central/parts/projects.xml"
    impact_base=$(impact_commit "$central" 'Historical invalid or unlocked project selection')
    cp "$native_case/valid-pins.xml" "$central/parts/projects.xml"
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
        sed -i 's@path="repositories/component"@path="repositories/moved"@' "$central/parts/projects.xml"
        sed -i 's@repositories/component/@repositories/moved/@g' "$central/Model.md" "$central/repositories/docs/Model.md"
        docs_commit=$(impact_commit "$central/repositories/docs" 'Follow relocated contract')
        # Rewrite only the docs project pin; preserve the component revision.
        sed -i "/name=\"docs\"/s/revision=\"[^\"]*\"/revision=\"$docs_commit\"/" "$central/parts/projects.xml"
    elif [[ "$kind" == remove ]]; then
        sed -i '/name="docs"/d' "$central/parts/projects.xml"
        # Keep Repo's object database, while removing the retired live checkout.
        rm -rf "$central/repositories/docs"
    else
        # Compare a current composition containing docs with a complete base
        # composition that has no docs project, while keeping current files valid.
        cp "$central/parts/projects.xml" "$native_case/current-pins.xml"
        sed -i '/name="docs"/d' "$central/parts/projects.xml"
        impact_base=$(impact_commit "$central" 'Historical composition without docs')
        cp "$native_case/current-pins.xml" "$central/parts/projects.xml"
    fi
    impact_commit "$central" 'Change project composition' > /dev/null
    (cd "$central" && "$REQVIRE_BIN" --manifest default.xml validate)
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
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-before.json"
    printf 'External invalid model must not be imported.\n' > "$central/repositories/component/Added.md"
    impact_state > "$native_case/before-state.txt"
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
native cli/repo-impact-native-history
native_mcp mcp/repo-impact-native-history
native_required cli/repo-impact-native-history-required
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
        native) impact_native ;;
        native_mcp) impact_mcp mcp native ;;
        native_required) impact_native_required ;;
        include) impact_include ;;
        missing|unlocked) impact_bad_base "$1" ;;
        add|remove|move) impact_project "$1" ;;
        pending) impact_pending ;;
    esac
fi
