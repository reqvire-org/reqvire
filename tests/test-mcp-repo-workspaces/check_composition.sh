#!/bin/bash
# Canonical composition: one root history owns source XML and optional models.
# Real Repo/Git fixtures; Bash/curl/jq assertions. Every golden outcome is PASS.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$TEST_SCRIPT_DIR/check_change_impact.sh"

composition_prepare() {
    local key=$1 mode=empty
    [[ "$key" != *_model ]] || mode=model
    native_fixture "canonical-$key" "$mode" nested
    root_model=$mode
    mkdir -p "$native_case/docs"
    cp "$TEST_SCRIPT_DIR/fixtures/impact-docs.md.txt" "$native_case/docs/Model.md"
    printf 'Guide for the public component behavior.\n' > "$native_case/docs/Guide.txt"
    git -C "$native_case/docs" init -qb main
    docs_commit=$(impact_commit "$native_case/docs" 'Documentation model and page')
    git clone -q --bare "$native_case/docs" "$native_case/remotes/docs"
    source_projects="$source_manifest"
    { fixture_manifest; fixture_project component "$alpha"; fixture_project docs "$docs_commit"; } > "$source_manifest"
    impact_base=$(impact_commit "$central" 'Record authoritative root source and component pins')
    git -C "$central" push -q "file://$native_case/remotes/central" HEAD:refs/heads/main
    (cd "$central" && "$REQVIRE_BIN" workspace sync)
    git -C "$central/repositories/component" checkout -q -B main "$alpha"
    git -C "$central/repositories/docs" checkout -q -B main "$docs_commit"
    git -C "$central" branch release/1.0 "$impact_base"
    test ! -e "$central/.repo/manifests/.git"
    test -z "$(git -C "$central" status --porcelain)"
    test -z "$(git -C "$central" ls-files -- .repo repositories)"
    test ! -e "$central/.reqvire-composition.json"
    mkdir -p "$central/.repo/manifests/manifests"
    cp "$source_projects" "$central/.repo/manifests/reqvire-workspace.toml"
    printf '%s\n' "$impact_base" > "$native_case/cache-head"
    (cd "$central" && "$REQVIRE_BIN" validate)
}

composition_stop() {
    kill "$impact_pid"
    wait "$impact_pid" 2>/dev/null || true
    trap - EXIT
}

composition_add() {
    local args
    if [[ "$root_model" == model ]]; then
        # One mutation changes the contract and its root/docs consumers. This
        # remains one transaction even when automatic commits are enabled.
        impact_rpc reqvire.rename_element \
          '{"branch":"main","element_name":"Component Specification","new_name":"Reviewed Component Specification","dry_run":false}' \
          "$native_case/add.json"
        for owner in "$central" "$central/repositories/docs"; do
            grep -Fq 'Reviewed Component Specification' "$owner/Model.md"
        done
    else
        args=$(jq -n --rawfile content "$TEST_SCRIPT_DIR/fixtures/addition.md.txt" \
            '{branch:"main",file:"repositories/component/Added.md",content:$content}')
        impact_rpc reqvire.add_element "$args" "$native_case/add.json"
        diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$central/repositories/component/Added.md"
    fi
}

composition_commit() {
    impact_rpc reqvire.git.commit '{"branch":"main","message":"Checkpoint root source composition"}' "$native_case/commit.json"
}

composition_assert_checkpoint() {
    local component root_head cached
    component=$(git -C "$central/repositories/component" rev-parse HEAD)
    root_head=$(git -C "$central" rev-parse HEAD)
    cached=$(cat "$native_case/cache-head")
    test "$component" != "$alpha"
    test "$(git -C "$central/repositories/component" rev-parse HEAD^)" = "$alpha"
    test "$root_head" != "$impact_base"
    test "$(git -C "$central" rev-parse HEAD^)" = "$impact_base"
    grep -Fq "commit = \"$component\"" "$source_projects"
    git -C "$central" show HEAD:reqvire-workspace.toml | grep -Fq "commit = \"$component\""
    if [[ "$root_model" == model ]]; then
        local docs_head
        docs_head=$(git -C "$central/repositories/docs" rev-parse HEAD)
        test "$docs_head" != "$docs_commit"
        test "$(git -C "$central/repositories/docs" rev-parse HEAD^)" = "$docs_commit"
        grep -Fq "commit = \"$docs_head\"" "$source_projects"
    else
        test "$(git -C "$central/repositories/docs" rev-parse HEAD)" = "$docs_commit"
    fi
    # A cache may retain the baseline or reflect the actual root commit. It
    # cannot have an independently authored manifest commit.
    [[ "$cached" == "$impact_base" || "$cached" == "$root_head" ]]
    test ! -e "$central/.reqvire-composition.json"
    test -z "$(git -C "$central" ls-files -- .repo repositories)"
    test -z "$(git -C "$central" for-each-ref --format='%(refname)' refs/heads/reqvire/manifests/)"
    git -C "$central" diff-tree --no-commit-id --name-only -r HEAD | sort > "$native_case/root-paths.txt"
    if [[ "$root_model" == model ]]; then
        printf '%s\n' Model.md reqvire-workspace.toml > "$native_case/expected-paths.txt"
    else
        printf '%s\n' reqvire-workspace.toml > "$native_case/expected-paths.txt"
    fi
    diff -u "$native_case/expected-paths.txt" "$native_case/root-paths.txt"
    (cd "$central" && "$REQVIRE_BIN" validate)
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status.json"
    jq -e '.result.structuredContent.context |
      .pending_changes == [] and ((.manifest_owners // []) | length == 0)' "$native_case/status.json"
}

composition_load() {
    (cd "$central" && "$REQVIRE_BIN" search --json) > "$native_case/cli-model.json"
    jq -S '[.files[].elements[].name] | sort' "$native_case/cli-model.json" > "$native_case/cli-names.json"
    jq -n --arg mode "$root_model" '
      ["Component Capability","Component Requirement","Component Specification",
       "Documentation Capability","API Guide Requirement","Documentation Review Objective","Documentation Inspection"]
      + (if $mode == "model" then ["Central Capability","Central Consumer"] else [] end) | sort' \
      > "$native_case/expected-names.json"
    diff -u "$native_case/expected-names.json" "$native_case/cli-names.json"
    impact_start_server mcp native
    impact_rpc reqvire.search '{"branch":"main"}' "$native_case/mcp-model.json"
    jq -S '[.result.structuredContent.files[].elements[].name] | sort' "$native_case/mcp-model.json" > "$native_case/mcp-names.json"
    diff -u "$native_case/expected-names.json" "$native_case/mcp-names.json"
}

composition_source_owner() {
    impact_start_server mcp native
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status.json"
    jq -e '.result.structuredContent.context | ((.manifest_owners // []) | length == 0)' "$native_case/status.json"
    composition_commit
    test "$(git -C "$central" rev-parse HEAD)" = "$impact_base"
    test ! -e "$central/.reqvire-composition.json"
}

composition_omega() {
    git -C "$central/repositories/component" merge -q --ff-only "$omega"
    sed -i "s/$alpha/$omega/" "$source_projects"
    impact_current=$(impact_commit "$central" 'Advance pins in authoritative root source')
}

composition_publish_source() {
    git -C "$central" push -q "file://$native_case/remotes/central" HEAD:refs/heads/main
    (cd "$central" && "$REQVIRE_BIN" workspace sync)
}

composition_stale_cache() {
    composition_omega
    # The root source and component checkout agree. Repo's cache is still alpha.
    grep -Fq "$alpha" "$central/.repo/manifests/reqvire-workspace.toml"
    impact_state > "$native_case/before.txt"
    (cd "$central" && "$REQVIRE_BIN" search --json) > "$native_case/search.json"
    grep -Fq 'behavior is omega' "$native_case/search.json"
    impact_state > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
}

composition_branches() {
    composition_omega
    composition_publish_source
    impact_start_server mcp native
    impact_state | sed '/^refs\/heads\/feature\/topic /d' > "$native_case/before.txt"
    impact_rpc reqvire.branch.create \
      '{"branch":"feature/topic","base_ref":"release/1.0","from_branch":"main"}' "$native_case/create.json"
    local child
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/create.json")
    test "$(git -C "$child" rev-parse HEAD)" = "$impact_base"
    test "$(git -C "$child/repositories/component" rev-parse HEAD)" = "$alpha"
    test "$(git -C "$child/repositories/docs" rev-parse HEAD)" = "$docs_commit"
    for owner in "$child" "$child/repositories/component" "$child/repositories/docs"; do
        test "$(git -C "$owner" branch --show-current)" = feature/topic
    done
    grep -Fq "$alpha" "$child/reqvire-workspace.toml"
    test ! -e "$child/.reqvire-composition.json"
    jq -e '(.result.structuredContent.context // .result.structuredContent) |
      ((.manifest_owners // []) | length == 0)' "$native_case/create.json"
    impact_state | sed '/^refs\/heads\/feature\/topic /d' > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
    impact_rpc reqvire.branch.close '{"branch":"feature/topic"}' "$native_case/close.json"
    impact_rpc reqvire.branch.open '{"branch":"feature/topic"}' "$native_case/reopen.json"
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/reopen.json")
    test "$(git -C "$child/repositories/component" rev-parse HEAD)" = "$alpha"
    grep -Fq "$alpha" "$child/reqvire-workspace.toml"
}

composition_checkpoint() {
    [[ "$1" != automatic_* ]] || composition_server_options=(--enable-commits)
    impact_start_server mcp native
    composition_add
    [[ "$1" == automatic_* ]] || composition_commit
    composition_assert_checkpoint
    local saved
    saved=$(git -C "$central" rev-parse HEAD)
    composition_commit
    test "$(git -C "$central" rev-parse HEAD)" = "$saved"
    composition_stop
    impact_start_server mcp native
    impact_rpc reqvire.search '{"branch":"main"}' "$native_case/restarted.json"
    local expected='Added Capability'
    [[ "$root_model" != model ]] || expected='Reviewed Component Specification'
    jq -e --arg name "$expected" 'any(.result.structuredContent.files[].elements[]; .name == $name)' "$native_case/restarted.json"
}

composition_external() {
    impact_start_server mcp native
    composition_omega
    impact_rpc reqvire.read_element '{"branch":"main","name":"Component Specification"}' "$native_case/before-open.json"
    grep -Fq 'behavior is alpha' "$native_case/before-open.json"
    impact_rpc reqvire.branch.refresh '{"branch":"main"}' "$native_case/refresh.json" || true
    grep -Fq manifest_or_project_changed "$native_case/refresh.json"
    impact_rpc reqvire.branch.open '{"branch":"main"}' "$native_case/open.json"
    impact_rpc reqvire.read_element '{"branch":"main","name":"Component Specification"}' "$native_case/after-open.json"
    grep -Fq 'behavior is omega' "$native_case/after-open.json"
    test ! -e "$central/.reqvire-composition.json"
}

composition_rejected_open() {
    local kind=$1
    impact_start_server mcp native
    if [[ "$kind" == pending ]]; then composition_add; fi
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-before.json"
    if [[ "$kind" == invalid ]]; then
        cp "$central/repositories/component/Model.md" "$central/Broken.md"
        impact_commit "$central" 'External invalid duplicate model' > /dev/null
    else
        git -C "$central" -c commit.gpgSign=false commit --allow-empty -qm 'External root commit'
    fi
    if impact_rpc reqvire.branch.open '{"branch":"main"}' "$native_case/rejected.json"; then
        echo "Explicit open admitted $kind work" >&2; return 1
    fi
    impact_rpc reqvire.search '{"branch":"main"}' "$native_case/retained.json"
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/status-after.json"
    for phase in before after; do
        jq -S '.result.structuredContent.context | {accepted_head,model_revision,pending_changes}' \
          "$native_case/status-$phase.json" > "$native_case/$phase.json"
    done
    diff -u "$native_case/before.json" "$native_case/after.json"
    if [[ "$kind" == pending ]]; then
        jq -e 'any(.result.structuredContent.files[].elements[]; .name == "Added Capability")' "$native_case/retained.json"
    fi
}

composition_impact() {
    local mode=$1
    composition_omega
    composition_publish_source
    impact_cli native --json
    impact_graph "$native_case/impact.out"
    jq -e --arg base "$impact_base" --arg current "$impact_current" '
      .comparison | .base.central_commit == $base and .current.central_commit == $current and
      all([.base,.current][]; .central_commit as $root |
        ([.manifest_inputs[].path] == ["reqvire-workspace.toml"]) and
        all(.manifest_inputs[]; .commit == $root and ((.owner // "") == "")))' "$native_case/impact.out"
    if [[ "$mode" != cli ]]; then
        impact_start_server "$mode" native
        local args
        args=$(jq -n --arg base "$impact_base" '{branch:"main",git_commit:$base}')
        impact_rpc reqvire.change_impact "$args" "$native_case/mcp-impact.json"
        jq -S '.result.structuredContent | del(.context)' "$native_case/mcp-impact.json" > "$native_case/mcp-normalized.json"
        jq -S . "$native_case/impact.out" > "$native_case/cli-normalized.json"
        diff -u "$native_case/cli-normalized.json" "$native_case/mcp-normalized.json"
    fi
    test ! -e "$central/.reqvire-composition.json"
}

composition_review() {
    composition_omega
    composition_publish_source
    (cd "$central" && GITHUB_BASE_REF=release/1.0 "$REQVIRE_BIN" change-impact --json) > "$native_case/review.json"
    impact_graph "$native_case/review.json"
    (cd "$central" && "$REQVIRE_BIN" change-impact --git-commit HEAD --json) > "$native_case/head.json"
    jq -e '.added == [] and .removed == [] and .changed == [] and .relocated == []' "$native_case/head.json"
    if (cd "$central" && env -u GITHUB_BASE_REF "$REQVIRE_BIN" change-impact --json) > "$native_case/no-base.txt" 2>&1; then
        echo 'Comparison guessed a review baseline' >&2; return 1
    fi
    grep -Fq review_baseline_required "$native_case/no-base.txt"
}

composition_root_failure() {
    local hook saved
    hook=$(git -C "$central" rev-parse --path-format=absolute --git-path hooks/reference-transaction)
    printf '#!/bin/bash\n[[ "$1" != prepared ]]\n' > "$hook"
    chmod +x "$hook"
    impact_start_server mcp native
    composition_add
    if composition_commit; then echo 'Root commit denial was ignored' >&2; return 1; fi
    saved=$(git -C "$central/repositories/component" rev-parse HEAD)
    test "$saved" != "$alpha"
    test "$(git -C "$central" rev-parse HEAD)" = "$impact_base"
    test "$(cat "$native_case/cache-head")" = "$impact_base"
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/failed-status.json"
    jq -e '.result.structuredContent.context.pending_changes | index("reqvire-workspace.toml") != null' "$native_case/failed-status.json"
    rm "$hook"
    composition_commit
    test "$(git -C "$central/repositories/component" rev-parse HEAD)" = "$saved"
    composition_assert_checkpoint
}

composition_collision() {
    git -C "$central/repositories/component" branch feature/topic "$omega"
    impact_start_server mcp native
    impact_state > "$native_case/before.txt"
    if impact_rpc reqvire.branch.create '{"branch":"feature/topic","base_ref":"release/1.0","from_branch":"main"}' "$native_case/collision.json"; then
        echo 'Component branch collision was ignored' >&2; return 1
    fi
    grep -Fq repositories/component "$native_case/collision.json"
    test "$(git -C "$central/repositories/component" rev-parse feature/topic)" = "$omega"
    impact_state > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
    ! git -C "$central" show-ref --verify --quiet refs/heads/feature/topic
}

composition_publication_prepare() {
    # Both the review base and feature use the same logical repository
    # identities. The fixture's Git wrapper maps those identities to real
    # local remotes without changing which repository an old pin belongs to.
    sed -i "s@file://$native_case/remotes/@https://github.com/test/@" "$source_projects"
    impact_base=$(impact_commit "$central" 'Register stable offline publication identities')
    git -C "$central" push -q "file://$native_case/remotes/central" HEAD:refs/heads/main
    git -C "$central" switch -qc feature/topic
    git -C "$central/repositories/component" switch -qc feature/topic
    cp "$TEST_SCRIPT_DIR/expected/added.md.txt" "$central/repositories/component/Added.md"
    published_component=$(impact_commit "$central/repositories/component" 'Feature component change')
    git -C "$central/repositories/component" push -q "file://$native_case/remotes/component" HEAD:refs/heads/feature/topic
    sed -i "s@$alpha@$published_component@" "$source_projects"
    published_root=$(impact_commit "$central" 'Feature model and pins on one root branch')
    git -C "$central" push -q "file://$native_case/remotes/central" HEAD:refs/heads/feature/topic
    git -C "$central" remote set-url origin https://github.com/test/project.git
    git -C "$central/repositories/component" remote set-url origin https://github.com/test/component
    git -C "$central/repositories/docs" remote set-url origin https://github.com/test/docs
    git -C "$central" config branch.feature/topic.reqvireCompositionBase "$impact_base"
    mkdir -p "$native_case/bin" "$native_case/github-state"
    cp "$TEST_SCRIPT_DIR/fixtures/git-double.sh" "$native_case/bin/git"
    cp "$TEST_SCRIPT_DIR/fixtures/gh-composition-double.sh" "$native_case/bin/gh"
    chmod +x "$native_case/bin/git" "$native_case/bin/gh"
    export REQVIRE_GH_TEST_STATE="$native_case/github-state"
    export REQVIRE_REPO_TEST_GIT_BINARY
    REQVIRE_REPO_TEST_GIT_BINARY=$(command -v git)
    export REQVIRE_REPO_TEST_REMOTES
    REQVIRE_REPO_TEST_REMOTES=$(jq -cn --arg root "$native_case/remotes" '{
      "https://github.com/test/project.git":($root+"/central"),
      "https://github.com/test/component":($root+"/component"),
      "https://github.com/test/docs":($root+"/docs")}')
    export REQVIRE_REPO_TEST_PUSHES="$native_case/pushes.jsonl"
    : > "$REQVIRE_REPO_TEST_PUSHES"
    export REQVIRE_REPO_TEST_FAIL_COMPONENT="$native_case/deny-component"
    export PATH="$native_case/bin:$PATH"
    composition_server_options=(--enable-github)
    impact_start_server mcp native
}

composition_prs() {
    local kind=$1 args
    composition_publication_prepare
    args='{"branch":"feature/topic","base":"main","title":"Coordinated change","body":"Review the product change.","repository_bases":{"repositories/component":"main"}}'
    if [[ "$kind" == partial_prs ]]; then
        touch "$REQVIRE_GH_TEST_STATE/deny-root"
        if impact_rpc reqvire.github.pr.create "$args" "$native_case/partial.json"; then return 1; fi
        test -f "$REQVIRE_GH_TEST_STATE/test_component.json"
        test ! -e "$REQVIRE_GH_TEST_STATE/test_project.json"
        rm "$REQVIRE_GH_TEST_STATE/deny-root"
    fi
    impact_rpc reqvire.github.pr.create "$args" "$native_case/prs.json"
    jq -e '.result.structuredContent.pull_requests | map(.path) | sort == ["","repositories/component"]' "$native_case/prs.json"
    impact_rpc reqvire.github.pr.create "$args" "$native_case/repeat.json"
    local root_attempts=1
    [[ "$kind" != partial_prs ]] || root_attempts=2
    jq -e -s --argjson roots "$root_attempts" '
      ([.[] | select(.operation == "pr create" and .repository == "test/component")] | length == 1) and
      ([.[] | select(.operation == "pr create" and .repository == "test/project")] | length == $roots) and
      all(.[] | select(.operation == "pr create"); .repository == "test/component" or .repository == "test/project")' \
      "$REQVIRE_GH_TEST_STATE/calls.jsonl"
    test ! -e "$REQVIRE_GH_TEST_STATE/test_docs.json"
}

composition_push_retry() {
    composition_publication_prepare
    # Remove only the fixture's already-published feature refs, so the push
    # operation must perform the dependency publications itself.
    "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$native_case/remotes/component" update-ref -d refs/heads/feature/topic
    "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$native_case/remotes/central" update-ref -d refs/heads/feature/topic
    touch "$native_case/deny-component"
    if impact_rpc reqvire.git.push '{"branch":"feature/topic"}' "$native_case/denied.json"; then return 1; fi
    jq -e -s 'length > 0 and all(.[]; all(.targets[]; contains("project.git") | not))' "$REQVIRE_REPO_TEST_PUSHES"
    rm "$native_case/deny-component"
    impact_rpc reqvire.git.push '{"branch":"feature/topic"}' "$native_case/pushed.json"
    jq -e -s 'all(.[]; .cwd | contains("/.repo/") | not)' "$REQVIRE_REPO_TEST_PUSHES"
    test "$("$REQVIRE_REPO_TEST_GIT_BINARY" -C "$native_case/remotes/central" rev-parse refs/heads/feature/topic)" = "$published_root"
    test "$("$REQVIRE_REPO_TEST_GIT_BINARY" -C "$native_case/remotes/component" rev-parse refs/heads/feature/topic)" = "$published_component"
    ! "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$native_case/remotes/docs" show-ref --verify --quiet refs/heads/feature/topic
}

composition_fresh_consumer() {
    impact_start_server mcp native
    composition_add
    composition_commit
    composition_assert_checkpoint
    composition_stop
    git -C "$central/repositories/component" push -q "file://$native_case/remotes/component" HEAD:refs/heads/main
    git -C "$central" push -q "file://$native_case/remotes/central" HEAD:refs/heads/main
    local consumer="$native_case/consumer"
    git clone -q --branch main "file://$native_case/remotes/central" "$consumer"
    (cd "$consumer" && "$REQVIRE_BIN" workspace sync && "$REQVIRE_BIN" validate &&
      "$REQVIRE_BIN" change-impact --git-commit "$impact_base" --json > "$native_case/consumer-impact.json")
    jq -e 'any(.added[]; .element_id | endswith("#added-capability"))' "$native_case/consumer-impact.json"
    test ! -e "$consumer/.reqvire-composition.json"
}

composition_missing_source() {
    git -C "$central" rm -q reqvire-workspace.toml
    impact_commit "$central" 'Remove explicitly selected source while old cache retains it' > /dev/null
    if (cd "$central" && "$REQVIRE_BIN" --manifest reqvire-workspace.toml validate) > "$native_case/missing.txt" 2>&1; then
        echo 'Cached XML substituted for missing authoritative source' >&2; return 1
    fi
    grep -Fq reqvire-workspace.toml "$native_case/missing.txt"
}

composition_unknown_component() {
    export REQVIRE_REF_TEST_GIT REQVIRE_REF_TEST_OWNER
    REQVIRE_REF_TEST_GIT=$(command -v git)
    REQVIRE_REF_TEST_OWNER="$central/repositories/component"
    export REQVIRE_REF_TEST_LOST="$native_case/lost-observation"
    export REQVIRE_REF_TEST_ATTEMPTS="$native_case/ref-attempts.txt"
    mkdir -p "$native_case/bin"
    cp "$TEST_SCRIPT_DIR/fixtures/git-observation-double.sh" "$native_case/bin/git"
    chmod +x "$native_case/bin/git"
    export PATH="$native_case/bin:$PATH"
    impact_start_server mcp native
    composition_add
    if composition_commit; then echo 'Unknown component publication reported success' >&2; return 1; fi
    test "$(wc -l < "$REQVIRE_REF_TEST_ATTEMPTS")" = 1
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/recovery.json"
    jq -e '.result.structuredContent.context | .recovery_required == true and .writes_available == false' "$native_case/recovery.json"
    rm "$REQVIRE_REF_TEST_LOST"
    if impact_rpc reqvire.branch.open '{"branch":"main"}' "$native_case/blocked-open.json"; then
        echo 'Open cleared an unresolved component outcome' >&2; return 1
    fi
    impact_rpc reqvire.search '{"branch":"main"}' "$native_case/recovery-read.json"
    jq -e 'any(.result.structuredContent.files[].elements[]; .name == "Added Capability")' "$native_case/recovery-read.json"
}

composition_overlay() {
    local kind=$1 overlay="$central/manifests/revisions.xml" saved
    git -C "$central" rm -q reqvire-workspace.toml
    mkdir -p "$central/manifests"
    cat > "$central/manifests/default.xml" <<EOF
<manifest>
  <remote name="local" fetch="file://$native_case/remotes/"/>
  <project name="component" path="repositories/component" remote="local" revision="$alpha"/>
  <project name="docs" path="repositories/docs" remote="local" revision="$docs_commit"/>
  <include name="manifests/revisions.xml"/>
</manifest>
EOF
    printf '<manifest><extend-project name="component" revision="%s"/></manifest>\n' "$alpha" > "$overlay"
    impact_commit "$central" 'Version legacy XML selection' > /dev/null
    if [[ "$kind" == unversioned_overlay ]]; then
        mkdir -p "$central/.repo/local_manifests"
        printf '<manifest><extend-project name="component" revision="%s"/></manifest>\n' "$omega" > "$central/.repo/local_manifests/review.xml"
        impact_state > "$native_case/before.txt"
        if (cd "$central" && "$REQVIRE_BIN" workspace migrate --from repo --source manifests/default.xml) > "$native_case/rejected.txt" 2>&1; then
            echo 'Unsupported local XML overlay was silently omitted' >&2; return 1
        fi
        grep -Fq '.repo/local_manifests/review.xml' "$native_case/rejected.txt"
        test ! -e "$source_manifest"
        impact_state > "$native_case/after.txt"
        diff -u "$native_case/before.txt" "$native_case/after.txt"
        return
    fi
    (cd "$central" && "$REQVIRE_BIN" workspace migrate --from repo --source manifests/default.xml)
    ! grep -Fq include "$source_manifest"
    impact_base=$(impact_commit "$central" 'Record one native composition')
    cp "$overlay" "$native_case/unchanged-legacy.xml"
    impact_start_server mcp native
    composition_add
    composition_commit
    saved=$(git -C "$central/repositories/component" rev-parse HEAD)
    grep -Fq "commit = \"$saved\"" "$source_manifest"
    diff -u "$native_case/unchanged-legacy.xml" "$overlay"
    test "$(git -C "$central" rev-parse HEAD^)" = "$impact_base"
    (cd "$central" && "$REQVIRE_BIN" validate)
    (cd "$central" && "$REQVIRE_BIN" change-impact --git-commit "$impact_base" --json) > "$native_case/impact.json"
    jq -e 'any(.added[]; .element_id == "repositories/component/Added.md#added-capability") and
      all([.comparison.base,.comparison.current][];
        .central_commit as $root | [.manifest_inputs[].path] == ["reqvire-workspace.toml"] and
        all(.manifest_inputs[]; .commit == $root))' "$native_case/impact.json"
    composition_stop
    impact_start_server mcp native
    impact_rpc reqvire.rename_element '{"branch":"main","element_name":"Added Capability","new_name":"Renamed Added Capability","dry_run":false}' "$native_case/restarted-rename.json"
    composition_commit
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/restarted-status.json"
    jq -e '.result.structuredContent.context | .writes_available == true and .pending_changes == []' "$native_case/restarted-status.json"
    saved=$(git -C "$central/repositories/component" rev-parse HEAD)
    grep -Fq "commit = \"$saved\"" "$source_manifest"
    impact_rpc reqvire.branch.create '{"branch":"feature/converted","base_ref":"HEAD","from_branch":"main"}' "$native_case/child.json"
    local child
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/child.json")
    grep -Fq "commit = \"$saved\"" "$child/reqvire-workspace.toml"
    test "$(git -C "$child/repositories/component" rev-parse HEAD)" = "$saved"
}

run_composition_cases() {
    local key label
    : > "$TEST_DIR/output/composition-checks.txt"
    while read -r key label; do
        if bash "$TEST_SCRIPT_DIR/check_composition.sh" "$key" > "$TEST_DIR/output/composition-$key.log" 2>&1; then
            printf 'PASS %s\n' "$label" >> "$TEST_DIR/output/composition-checks.txt"
        else
            printf 'FAIL %s\n' "$label" >> "$TEST_DIR/output/composition-checks.txt"
            cat "$TEST_DIR/output/composition-$key.log" >&2
            status=1
        fi
    done <<'CASES'
load_model cli-mcp/root-source-with-root-model
load_empty cli-mcp/manifest-only-root-with-complete-pinned-model
owner mcp/cache-is-not-an-authoring-owner-or-initial-checkpoint
stale cli/root-source-over-stale-cache
branches_model mcp/root-source-release-base-and-reopen-with-root-model
branches_empty mcp/root-source-release-base-and-reopen-without-root-model
explicit_model mcp/component-first-one-root-commit-with-model
explicit_empty mcp/component-first-one-root-commit-without-model
automatic_model mcp/automatic-one-root-source-checkpoint-with-model
automatic_empty mcp/automatic-one-root-source-checkpoint-without-model
external mcp/external-root-pins-explicit-admission
invalid mcp/rejected-invalid-admission-retains-accepted-model
pending mcp/rejected-pending-admission-retains-accepted-edits
impact_cli cli/root-source-impact-and-provenance
impact_mcp mcp/root-source-impact-parity
impact_serve serve/root-source-impact-parity
review cli/pr-merge-base-and-explicit-head
collision mcp/component-branch-collision-preserves-source
root_failure mcp/root-failure-retains-components-without-manifest-stage
component_unknown mcp/unknown-component-ref-retains-recovery
prs mcp/linked-component-root-prs-without-manifest-pr
partial_prs mcp/partial-component-root-pr-set-reused
push_retry mcp/component-before-root-push-without-manifest-ref
fresh_consumer cli/published-root-source-composition-fresh-consumer
missing_source cli/missing-authoritative-source-not-replaced-by-cache
versioned_overlay mcp/migrated-xml-overlay-single-manifest-checkpoint-and-impact
unversioned_overlay cli/unsupported-local-xml-overlay-rejected
CASES
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    set -Eeuo pipefail
    case_name=${1:-unknown}
    trap 'printf "FAIL canonical composition case %s at line %s: %s\n" "$case_name" "$LINENO" "$BASH_COMMAND" >&2' ERR
    composition_prepare "$1"
    case "$1" in
        load_*) composition_load ;;
        owner) composition_source_owner ;;
        stale) composition_stale_cache ;;
        branches_*) composition_branches ;;
        explicit_*|automatic_*) composition_checkpoint "$1" ;;
        external) composition_external ;;
        invalid|pending) composition_rejected_open "$1" ;;
        impact_*) composition_impact "${1#impact_}" ;;
        review) composition_review ;;
        collision) composition_collision ;;
        root_failure) composition_root_failure ;;
        component_unknown) composition_unknown_component ;;
        prs|partial_prs) composition_prs "$1" ;;
        push_retry) composition_push_retry ;;
        fresh_consumer) composition_fresh_consumer ;;
        missing_source) composition_missing_source ;;
        versioned_overlay|unversioned_overlay) composition_overlay "$1" ;;
        *) printf 'Unknown canonical composition case: %s\n' "$1" >&2; exit 2 ;;
    esac
fi
