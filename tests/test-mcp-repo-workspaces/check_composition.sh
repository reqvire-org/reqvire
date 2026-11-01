#!/bin/bash
# Coordinated branch lifecycle over Repo's native manifest and real local Git.
# Every listed case must pass; missing implementation is an ordinary failure.
TEST_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$TEST_SCRIPT_DIR/check_change_impact.sh"

composition_record() {
    local commit repository owner source
    : > "$native_case/record-sources.jsonl"
    for owner in .repo/manifests .repo/local_manifests; do
        [[ -e "$central/$owner/.git" ]] || continue
        commit=$(git -C "$central/$owner" rev-parse HEAD)
        repository=$(git -C "$central/$owner" remote get-url origin)
        while IFS= read -r -d '' source; do
            jq -cn --arg commit "$commit" --arg repository "$repository" --arg owner "$owner" --arg source "$source" \
              '{path:($owner+"/"+$source),owner:$owner,source:$source,repository:$repository,commit:$commit}' \
              >> "$native_case/record-sources.jsonl"
        done < <(git -C "$central/$owner" ls-files -z -- '*.xml')
    done
    jq -s '{schema_version:1,selection:".repo/manifest.xml",manifest:".repo/manifests/default.xml",sources:sort_by(.path)}' \
        "$native_case/record-sources.jsonl" > "$central/.reqvire-composition.json"
}

composition_prepare() {
    native_fixture "composition-$1"
    git clone -q --bare "$central/.repo/manifests" "$native_case/remotes/manifests"
    git -C "$central/.repo/manifests" remote set-url origin "file://$native_case/remotes/manifests"
    git -C "$central/.repo/manifests" checkout -q -B main
    git -C "$central/.repo/manifests" config branch.main.remote origin
    git -C "$central/.repo/manifests" config branch.main.merge refs/heads/main
    mkdir -p "$native_case/docs"
    cp "$TEST_SCRIPT_DIR/fixtures/impact-docs.md.txt" "$native_case/docs/Model.md"
    printf 'Guide for the selected public contract.\n' > "$native_case/docs/Guide.txt"
    git -C "$native_case/docs" init -qb main
    docs_commit=$(impact_commit "$native_case/docs" 'Documentation requirements and page')
    git clone -q --bare "$native_case/docs" "$native_case/remotes/docs"
    sed -i "/<\/manifest>/i\  <project name=\"docs\" path=\"repositories/docs\" remote=\"local\" revision=\"$docs_commit\"/>" "$native"
    impact_commit "$central/.repo/manifests" 'Declare pinned documentation component' > /dev/null
    git -C "$central/.repo/manifests" push -q "file://$native_case/remotes/manifests" HEAD:refs/heads/main
    (cd "$central" && repo sync -q --no-clone-bundle --no-tags)
    git -C "$central/repositories/component" checkout -q -B main "$alpha"
    git -C "$central/repositories/docs" checkout -q -B main "$docs_commit"
    composition_record
    impact_base=$(impact_commit "$central" 'Record native alpha composition')
    manifest_base=$(git -C "$central/.repo/manifests" rev-parse HEAD)
    git -C "$central" branch release/1.0 "$impact_base"
    test ! -e "$central/default.xml"
    test -z "$(git -C "$central" ls-files .repo default.xml)"
    (cd "$central" && "$REQVIRE_BIN" validate)
}

composition_omega() {
    git -C "$central/repositories/component" checkout -q main
    git -C "$central/repositories/component" merge -q --ff-only "$omega"
    sed -i "s/$alpha/$omega/" "$native"
    impact_commit "$central/.repo/manifests" 'Select omega component pin' > /dev/null
    composition_record
    impact_current=$(impact_commit "$central" 'Record native omega composition')
}

composition_stop() {
    kill "$impact_pid"
    wait "$impact_pid" 2>/dev/null || true
    trap - EXIT
}

composition_source_state() {
    local owner
    for owner in "$central" "$central/.repo/manifests" "$central"/repositories/*; do
        printf 'OWNER %s\n' "$owner"
        git -C "$owner" rev-parse HEAD
        git -C "$owner" symbolic-ref --short HEAD
        git -C "$owner" ls-files --stage
        git -C "$owner" status --porcelain
    done
}

composition_native_history() {
    composition_omega
    impact_cli native --json
    impact_graph "$native_case/impact.out"
    jq -e --arg base "$impact_base" --arg owner "$manifest_base" '
      .comparison.base.central_commit == $base and
      any(.comparison.base.manifest_inputs[];
        .path == ".repo/manifests/default.xml" and .commit == $owner)
    ' "$native_case/impact.out"
    test ! -e "$central/default.xml"
    test -z "$(git -C "$central" ls-files .repo default.xml)"
}

composition_native_input_history() {
    mv "$native" "$central/.repo/manifests/projects.xml"
    printf '<manifest><include name="projects.xml"/></manifest>\n' > "$native"
    mkdir -p "$central/.repo/local_manifests"
    printf '<manifest><extend-project name="component" revision="%s"/></manifest>\n' "$alpha" \
        > "$central/.repo/local_manifests/pins.xml"
    git -C "$central/.repo/local_manifests" init -qb main
    git -C "$central/.repo/local_manifests" remote add origin "file://$native_case/remotes/overlays"
    impact_commit "$central/.repo/manifests" 'Select native included projects' > /dev/null
    impact_commit "$central/.repo/local_manifests" 'Version native local overlay' > /dev/null
    composition_record
    impact_base=$(impact_commit "$central" 'Associate native included composition')
    sed -i "s@$alpha@$omega@" "$central/.repo/local_manifests/pins.xml"
    git -C "$central/repositories/component" checkout -q -B main "$omega"
    impact_commit "$central/.repo/local_manifests" 'Advance native overlay pin' > /dev/null
    composition_record
    impact_commit "$central" 'Associate changed overlay composition' > /dev/null
    impact_state > "$native_case/before.txt"
    impact_cli native --json
    impact_graph "$native_case/impact.out"
    impact_state > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
}

composition_branch_set() {
    composition_omega
    composition_source_state > "$native_case/source-before.txt"
    impact_start_server mcp native
    impact_rpc reqvire.branch.create \
        '{"branch":"feature/topic","from_branch":"main","base_ref":"release/1.0"}' \
        "$native_case/create.json"
    local child owner
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/create.json")
    test "$(git -C "$child" rev-parse HEAD)" = "$impact_base"
    for owner in "$child" "$child/repositories/component" "$child/repositories/docs" "$child/.repo/manifests"; do
        test "$(git -C "$owner" symbolic-ref --short HEAD)" = feature/topic
    done
    test "$(git -C "$child/repositories/component" rev-parse HEAD)" = "$alpha"
    test "$(git -C "$child/repositories/docs" rev-parse HEAD)" = "$docs_commit"
    grep -Fq "revision=\"$alpha\"" "$child/.repo/manifests/default.xml"
    test ! -e "$child/default.xml"
    composition_source_state > "$native_case/source-after.txt"
    diff -u "$native_case/source-before.txt" "$native_case/source-after.txt"
    impact_rpc reqvire.branch.close '{"branch":"feature/topic"}' "$native_case/close.json"
    impact_rpc reqvire.branch.open '{"branch":"feature/topic"}' "$native_case/reopen.json"
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/reopen.json")
    test "$(git -C "$child/repositories/component" symbolic-ref --short HEAD)" = feature/topic
    test "$(git -C "$child/repositories/component" rev-parse HEAD)" = "$alpha"
}

composition_add() {
    local arguments
    arguments=$(jq -n --rawfile content "$TEST_SCRIPT_DIR/fixtures/addition.md.txt" \
        '{branch:"main",file:"repositories/component/Added.md",content:$content}')
    impact_rpc reqvire.add_element "$arguments" "$native_case/add-result.json"
    diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$central/repositories/component/Added.md"
}

composition_commit() {
    impact_rpc reqvire.git.commit '{"branch":"main","message":"Checkpoint native component change"}' \
        "$native_case/commit-result.json"
}

composition_bootstrap() {
    git -C "$central" rm -q .reqvire-composition.json
    before=$(impact_commit "$central" 'Unrecorded native starting composition')
    impact_start_server mcp native
    composition_commit
    test "$(git -C "$central" rev-parse HEAD)" != "$before"
    git -C "$central" show HEAD:.reqvire-composition.json > "$native_case/bootstrap.json"
    jq -e --arg commit "$manifest_base" '.sources[0].commit == $commit' "$native_case/bootstrap.json"
    test "$(git -C "$central/repositories/component" rev-parse HEAD)" = "$alpha"
    test "$(git -C "$central/.repo/manifests" rev-parse HEAD)" = "$manifest_base"
    after=$(git -C "$central" rev-parse HEAD)
    composition_commit
    test "$(git -C "$central" rev-parse HEAD)" = "$after"
    impact_rpc reqvire.branch.create '{"branch":"feature/bootstrap","from_branch":"main","base_ref":"HEAD"}' "$native_case/bootstrap-branch.json"
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/bootstrap-branch.json")
    test "$(git -C "$child/repositories/component" symbolic-ref --short HEAD)" = feature/bootstrap
}

composition_shared_history() {
    git -C "$central/.repo/manifests" remote set-url origin "file://$native_case/remotes/central"
    composition_record
    impact_base=$(impact_commit "$central" 'Record native manifest in shared repository history')
    git -C "$central" branch -f release/1.0 "$impact_base"
    impact_start_server mcp native
    impact_rpc reqvire.branch.create '{"branch":"feature/shared","from_branch":"main","base_ref":"release/1.0"}' "$native_case/shared-branch.json"
    local child saved arguments
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/shared-branch.json")
    test "$(git -C "$child/.repo/manifests" symbolic-ref --short HEAD)" = reqvire/manifests/feature/shared
    arguments=$(jq -n --rawfile content "$TEST_SCRIPT_DIR/fixtures/addition.md.txt" \
        '{branch:"feature/shared",file:"repositories/component/Added.md",content:$content}')
    impact_rpc reqvire.add_element "$arguments" "$native_case/shared-add.json"
    impact_rpc reqvire.git.commit '{"branch":"feature/shared","message":"Checkpoint shared-history manifest"}' "$native_case/shared-commit.json"
    saved=$(git -C "$child/.repo/manifests" rev-parse HEAD)
    impact_rpc reqvire.branch.close '{"branch":"feature/shared"}' "$native_case/shared-close.json"
    impact_rpc reqvire.branch.open '{"branch":"feature/shared"}' "$native_case/shared-open.json"
    child=$(jq -er '.result.structuredContent.workspace_root' "$native_case/shared-open.json")
    test "$(git -C "$child/.repo/manifests" rev-parse HEAD)" = "$saved"
    test "$(git -C "$child/.repo/manifests" symbolic-ref --short HEAD)" = reqvire/manifests/feature/shared
    diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$child/repositories/component/Added.md"
    test ! -e "$child/default.xml"
    test "$(git -C "$central" rev-parse HEAD)" = "$impact_base"
    test ! -e "$central/repositories/component/Added.md"
}

composition_checkpoint() {
    local mode=$1 saved manifest_saved root_saved
    [[ "$mode" != automatic ]] || composition_server_options=(--enable-commits)
    impact_start_server mcp native
    composition_add
    [[ "$mode" == automatic ]] || composition_commit
    saved=$(git -C "$central/repositories/component" rev-parse HEAD)
    manifest_saved=$(git -C "$central/.repo/manifests" rev-parse HEAD)
    root_saved=$(git -C "$central" rev-parse HEAD)
    test "$saved" != "$alpha"
    test "$manifest_saved" != "$manifest_base"
    test "$root_saved" != "$impact_base"
    test "$(git -C "$central/repositories/component" symbolic-ref --short HEAD)" = main
    git -C "$central/.repo/manifests" show HEAD:default.xml > "$native_case/saved-manifest.xml"
    grep -Fq "revision=\"$saved\"" "$native_case/saved-manifest.xml"
    git -C "$central" show HEAD:.reqvire-composition.json > "$native_case/saved-record.json"
    jq -e --arg saved "$manifest_saved" '.sources[] |
      select(.owner == ".repo/manifests") | .commit == $saved' "$native_case/saved-record.json"
    test "$(git -C "$central/repositories/docs" rev-parse HEAD)" = "$docs_commit"
    test -z "$(git -C "$central" ls-files .repo default.xml repositories)"
    local owner
    for owner in "$central" "$central/.repo/manifests" "$central"/repositories/*; do
        test -z "$(git -C "$owner" status --porcelain)"
    done
    composition_commit
    test "$(git -C "$central" rev-parse HEAD)" = "$root_saved"
    test "$(git -C "$central/.repo/manifests" rev-parse HEAD)" = "$manifest_saved"
    test "$(git -C "$central/repositories/component" rev-parse HEAD)" = "$saved"
    composition_stop
    impact_start_server mcp native
    impact_rpc reqvire.search '{"branch":"main"}' "$native_case/restarted.json"
    jq -e 'any(.result.structuredContent.files[].elements[]; .name == "Added Capability")' "$native_case/restarted.json"
    impact_rpc reqvire.change_impact "$(jq -n --arg base "$impact_base" '{branch:"main",git_commit:$base}')" \
        "$native_case/restarted-impact.json"
    jq -e 'any(.result.structuredContent.added[]; .element_id | endswith("#added-capability"))' \
        "$native_case/restarted-impact.json"
}

composition_external_open() {
    impact_start_server mcp native
    composition_omega
    impact_rpc reqvire.read_element '{"branch":"main","name":"Component Specification"}' "$native_case/before-open.json"
    grep -Fq 'behavior is alpha' "$native_case/before-open.json"
    impact_rpc reqvire.branch.refresh '{"branch":"main"}' "$native_case/guarded-refresh.json" || true
    grep -Fq manifest_or_project_changed "$native_case/guarded-refresh.json"
    impact_rpc reqvire.branch.open '{"branch":"main"}' "$native_case/open.json"
    impact_rpc reqvire.read_element '{"branch":"main","name":"Component Specification"}' "$native_case/after-open.json"
    grep -Fq 'behavior is omega' "$native_case/after-open.json"
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/opened-status.json"
    jq -e --arg head "$impact_current" '.result.structuredContent.context |
      .accepted_head == $head and .writes_available == true' "$native_case/opened-status.json"
    impact_rpc reqvire.branch.open '{"branch":"main"}' "$native_case/open-again.json"
    impact_rpc reqvire.read_element '{"branch":"main","name":"Component Specification"}' "$native_case/after-again.json"
    diff -u "$native_case/after-open.json" "$native_case/after-again.json"
}

composition_review_baseline() {
    composition_omega
    impact_state > "$native_case/before-state.txt"
    (cd "$central" && GITHUB_BASE_REF=release/1.0 "$REQVIRE_BIN" change-impact --json) > "$native_case/review.json"
    impact_graph "$native_case/review.json"
    (cd "$central" && "$REQVIRE_BIN" change-impact --git-commit HEAD --json) > "$native_case/head.json"
    jq -e '.added == [] and .removed == [] and .changed == [] and .relocated == []' "$native_case/head.json"
    if (cd "$central" && env -u GITHUB_BASE_REF "$REQVIRE_BIN" change-impact --json) > "$native_case/no-base.json" 2>&1; then
        echo 'Missing review baseline silently defaulted instead of requesting an explicit selection' >&2
        return 1
    fi
    grep -Fq review_baseline_required "$native_case/no-base.json"
    impact_state > "$native_case/after-state.txt"
    diff -u "$native_case/before-state.txt" "$native_case/after-state.txt"
}

composition_invalid_open() {
    impact_start_server mcp native
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/accepted-before.json"
    printf '# Elements\n\n### Central Capability\n\nDuplicate invalid model.\n\n#### Metadata\n  * type: capability\n' \
        > "$central/Broken.md"
    impact_commit "$central" 'External invalid candidate' > /dev/null
    if impact_rpc reqvire.branch.open '{"branch":"main"}' "$native_case/invalid-open.json"; then
        echo 'Invalid external composition was admitted' >&2
        return 1
    fi
    impact_rpc reqvire.read_element '{"branch":"main","name":"Component Specification"}' "$native_case/retained-read.json"
    grep -Fq 'behavior is alpha' "$native_case/retained-read.json"
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/accepted-after.json"
    jq -S '.result.structuredContent.context | {accepted_head,model_revision,pending_changes}' \
        "$native_case/accepted-before.json" > "$native_case/before.json"
    jq -S '.result.structuredContent.context | {accepted_head,model_revision,pending_changes}' \
        "$native_case/accepted-after.json" > "$native_case/after.json"
    diff -u "$native_case/before.json" "$native_case/after.json"
}

composition_pending_open() {
    impact_start_server mcp native
    composition_add
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/accepted-before.json"
    git -C "$central" -c commit.gpgSign=false commit --allow-empty -qm 'External committed tip'
    if impact_rpc reqvire.branch.open '{"branch":"main"}' "$native_case/pending-open.json"; then
        echo 'Explicit re-admission bypassed accepted pending edits' >&2
        return 1
    fi
    impact_rpc reqvire.search '{"branch":"main"}' "$native_case/pending-read.json"
    jq -e 'any(.result.structuredContent.files[].elements[]; .name == "Added Capability")' "$native_case/pending-read.json"
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/accepted-after.json"
    jq -S '.result.structuredContent.context | {accepted_head,model_revision,pending_changes}' \
        "$native_case/accepted-before.json" > "$native_case/before.json"
    jq -S '.result.structuredContent.context | {accepted_head,model_revision,pending_changes}' \
        "$native_case/accepted-after.json" > "$native_case/after.json"
    diff -u "$native_case/before.json" "$native_case/after.json"
}

composition_contract() {
    impact_start_server mcp native
    impact_rpc reqvire.tool_contract '{}' "$native_case/contract.json"
    jq -e '.result.structuredContent |
      .tool_contract_version == "5" and .capabilities.composition_management == true and
      any(.tools[]; .name == "reqvire.branch.create")' "$native_case/contract.json"
}

composition_collision() {
    git -C "$central/repositories/component" branch feature/topic "$omega"
    composition_source_state > "$native_case/before.txt"
    impact_start_server mcp native
    if impact_rpc reqvire.branch.create '{"branch":"feature/topic","base_ref":"release/1.0","from_branch":"main"}' \
        "$native_case/collision.json"; then
        echo 'Existing component feature branch was not protected' >&2
        return 1
    fi
    grep -Fq repositories/component "$native_case/collision.json"
    test "$(git -C "$central/repositories/component" rev-parse feature/topic)" = "$omega"
    composition_source_state > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
    if git -C "$central" show-ref --verify --quiet refs/heads/feature/topic; then
        echo 'Rejected preparation left a root branch behind' >&2
        return 1
    fi
}

composition_unversioned_overlay() {
    mkdir -p "$central/.repo/local_manifests"
    printf '<manifest><extend-project name="component" revision="%s"/></manifest>\n' "$alpha" \
        > "$central/.repo/local_manifests/pins.xml"
    impact_start_server mcp native
    composition_add
    if composition_commit; then
        echo 'An unversioned selecting overlay was reported as a committed composition' >&2
        return 1
    fi
    grep -Fq .repo/local_manifests/pins.xml "$native_case/commit-result.json"
    test "$(git -C "$central" rev-parse HEAD)" = "$impact_base"
}

composition_conflicting_owner() {
    git -C "$central/.repo/manifests" remote set-url origin "file://$native_case/remotes/central"
    git -C "$central/.repo/manifests" checkout -q -B main
    composition_record
    impact_base=$(impact_commit "$central" 'Record conflicting authoring destinations')
    impact_start_server mcp native
    composition_add
    composition_source_state > "$native_case/before.txt"
    if composition_commit; then
        echo 'Competing authoring branches were checkpointed' >&2
        return 1
    fi
    grep -Fq 'share authoring branch' "$native_case/commit-result.json"
    composition_source_state > "$native_case/after.txt"
    diff -u "$native_case/before.txt" "$native_case/after.txt"
    diff -u "$TEST_SCRIPT_DIR/expected/added.md.txt" "$central/repositories/component/Added.md"
}

composition_checkpoint_failure() {
    local stage=$1 owner hook saved
    owner=$central
    [[ "$stage" != manifest ]] || owner="$central/.repo/manifests"
    hook=$(git -C "$owner" rev-parse --path-format=absolute --git-path hooks/reference-transaction)
    mkdir -p "$(dirname "$hook")"
    printf '#!/bin/bash\n[[ "$1" != prepared ]]\n' > "$hook"
    chmod +x "$hook"
    impact_start_server mcp native
    composition_add
    if composition_commit; then
        echo "Injected $stage checkpoint failure was ignored" >&2
        return 1
    fi
    saved=$(git -C "$central/repositories/component" rev-parse HEAD)
    test "$saved" != "$alpha"
    test "$(git -C "$central" rev-parse HEAD)" = "$impact_base"
    if [[ "$stage" == manifest ]]; then
        test "$(git -C "$central/.repo/manifests" rev-parse HEAD)" = "$manifest_base"
    else
        test "$(git -C "$central/.repo/manifests" rev-parse HEAD)" != "$manifest_base"
    fi
    impact_rpc reqvire.workspace_status '{"branch":"main"}' "$native_case/partial-status.json"
    jq -e '.result.structuredContent.context.pending_changes | length > 0' "$native_case/partial-status.json"
    impact_rpc reqvire.search '{"branch":"main"}' "$native_case/partial-search.json"
    jq -e 'any(.result.structuredContent.files[].elements[]; .name == "Added Capability")' "$native_case/partial-search.json"
    rm "$hook"
    composition_commit
    test "$(git -C "$central/repositories/component" rev-parse HEAD)" = "$saved"
    test "$(git -C "$central" rev-parse HEAD)" != "$impact_base"
}

composition_unknown_ref() {
    local kind=$1
    export REQVIRE_REF_TEST_GIT REQVIRE_REF_TEST_OWNER
    REQVIRE_REF_TEST_GIT=$(command -v git)
    REQVIRE_REF_TEST_OWNER="$central/repositories/component"
    [[ "$kind" != manifest_unknown ]] || REQVIRE_REF_TEST_OWNER="$central/.repo/manifests"
    export REQVIRE_REF_TEST_LOST="$native_case/lost-observation"
    export REQVIRE_REF_TEST_ATTEMPTS="$native_case/ref-attempts.txt"
    mkdir "$native_case/bin"
    cp "$TEST_SCRIPT_DIR/fixtures/git-observation-double.sh" "$native_case/bin/git"
    chmod +x "$native_case/bin/git"
    export PATH="$native_case/bin:$PATH"
    impact_start_server mcp native
    composition_add
    if composition_commit; then echo 'Unobserved ref publication was accepted' >&2; return 1; fi
    local attempted
    attempted=$("$REQVIRE_REF_TEST_GIT" -C "$REQVIRE_REF_TEST_OWNER" rev-parse HEAD)
    jq -e --arg attempted "$attempted" '.result.structuredContent.context |
      .recovery_required == true and .writes_available == false and
      any(.project_outcomes[]; .commit == $attempted and .outcome == "unknown")' "$native_case/commit-result.json"
    rm "$REQVIRE_REF_TEST_LOST"
    if composition_commit; then echo 'Unknown ref outcome was blindly retried' >&2; return 1; fi
    test "$(wc -l < "$REQVIRE_REF_TEST_ATTEMPTS")" = 1
    impact_rpc reqvire.search '{"branch":"main"}' "$native_case/unknown-retained-read.json"
    jq -e 'any(.result.structuredContent.files[].elements[]; .name == "Added Capability")' \
        "$native_case/unknown-retained-read.json"
    if impact_rpc reqvire.branch.open '{"branch":"main"}' "$native_case/unknown-reopen.json"; then
        echo 'Explicit open bypassed unknown ref recovery' >&2; return 1
    fi
    test "$("$REQVIRE_REF_TEST_GIT" -C "$REQVIRE_REF_TEST_OWNER" rev-parse HEAD)" = "$attempted"
    test "$(git -C "$central" rev-parse HEAD)" = "$impact_base"
}

composition_publication_prepare() {
    local owner
    git -C "$central" remote set-url origin https://github.com/test/project.git
    git -C "$central/repositories/component" remote set-url local https://github.com/test/component
    git -C "$central/repositories/docs" remote set-url local https://github.com/test/docs
    git -C "$central/.repo/manifests" remote set-url origin https://github.com/test/manifests
    sed -i "s@file://$native_case/remotes/@https://github.com/test/@" "$native"
    impact_commit "$central/.repo/manifests" 'Register baseline publication identities' > /dev/null
    composition_record
    impact_base=$(impact_commit "$central" 'Publishable baseline composition')
    git -C "$central" push -q "$native_case/remotes/central" HEAD:refs/heads/main
    git -C "$central/.repo/manifests" push -q "$native_case/remotes/manifests" HEAD:refs/heads/main
    for owner in "$central" "$central/.repo/manifests" "$central"/repositories/*; do
        git -C "$owner" switch -qc feature/topic
    done
    git -C "$central" config branch.feature/topic.reqvireCompositionBase "$impact_base"
    cp "$TEST_SCRIPT_DIR/expected/added.md.txt" "$central/repositories/component/Added.md"
    published_component=$(impact_commit "$central/repositories/component" 'Saved feature component')
    sed -i "s@$alpha@$published_component@" "$native"
    impact_commit "$central/.repo/manifests" 'Saved feature pins' > /dev/null
    composition_record
    published_root=$(impact_commit "$central" 'Saved coordinated feature checkpoint')
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
      "https://github.com/test/docs":($root+"/docs"),
      "https://github.com/test/manifests":($root+"/manifests")}')
    export REQVIRE_REPO_TEST_PUSHES="$native_case/pushes.jsonl"
    export REQVIRE_REPO_TEST_FAIL_COMPONENT="$native_case/deny-component"
    export PATH="$native_case/bin:$PATH"
    composition_server_options=(--enable-github)
    impact_start_server mcp native
}

composition_coordinated_prs() {
    composition_publication_prepare
    # PR acceptance is independent of the separate push implementation: start
    # with all intended branches already published using real local Git.
    "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$central/repositories/component" push -q \
        "$native_case/remotes/component" HEAD:refs/heads/feature/topic
    "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$central/.repo/manifests" push -q \
        "$native_case/remotes/manifests" HEAD:refs/heads/feature/topic
    "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$central" push -q \
        "$native_case/remotes/central" HEAD:refs/heads/feature/topic
    local arguments
    arguments='{"branch":"feature/topic","base":"main","title":"Coordinated change","body":"Review the product change.","repository_bases":{"repositories/component":"main",".repo/manifests":"main"}}'
    impact_rpc reqvire.github.pr.create "$arguments" "$native_case/pr-result.json"
    jq -e '.result.structuredContent.pull_requests | map(.path) | sort ==
      ["", ".repo/manifests", "repositories/component"]' "$native_case/pr-result.json"
    jq -e -s '[.[] | select(.operation == "pr create") | .repository] ==
      ["test/component","test/manifests","test/project"]' "$REQVIRE_GH_TEST_STATE/calls.jsonl"
    jq -e -s 'any(.[]; .operation == "pr create" and .repository == "test/project" and
      (.body | contains("https://github.com/test/component/pull/7")) and
      (.body | contains("https://github.com/test/manifests/pull/7")))' "$REQVIRE_GH_TEST_STATE/calls.jsonl"
    impact_rpc reqvire.github.pr.create "$arguments" "$native_case/pr-repeat.json"
    jq -e -s '[.[] | select(.operation == "pr create")] | length == 3' "$REQVIRE_GH_TEST_STATE/calls.jsonl"
    test ! -e "$REQVIRE_GH_TEST_STATE/test_docs.json"
    local comment='{"branch":"feature/topic","repository_path":"repositories/component","pr_number":7,"body":"Review the component.\nKeep this exact text."}'
    impact_rpc reqvire.github.pr.comment "$comment" "$native_case/component-comment.json"
    jq -e '.result.structuredContent | .path == "repositories/component" and .comment_id == 11' \
        "$native_case/component-comment.json"
    jq -e -s '[.[] | select(.operation == "pr comment") | .repository] == ["test/component"]' \
        "$REQVIRE_GH_TEST_STATE/calls.jsonl"
}

composition_partial_prs() {
    composition_publication_prepare
    "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$central/repositories/component" push -q "$native_case/remotes/component" HEAD:refs/heads/feature/topic
    "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$central/.repo/manifests" push -q "$native_case/remotes/manifests" HEAD:refs/heads/feature/topic
    "$REQVIRE_REPO_TEST_GIT_BINARY" -C "$central" push -q "$native_case/remotes/central" HEAD:refs/heads/feature/topic
    local arguments='{"branch":"feature/topic","base":"main","title":"Coordinated change","body":"Review the product change.","repository_bases":{"repositories/component":"main",".repo/manifests":"main"}}'
    touch "$REQVIRE_GH_TEST_STATE/deny-root"
    if impact_rpc reqvire.github.pr.create "$arguments" "$native_case/partial-prs.json"; then
        echo 'Root PR denial was ignored' >&2
        return 1
    fi
    jq -e '.result.structuredContent | .outcome == "failed" and
      ([.pull_requests[].path] == ["repositories/component",".repo/manifests"])' "$native_case/partial-prs.json"
    test -f "$REQVIRE_GH_TEST_STATE/test_component.json"
    test -f "$REQVIRE_GH_TEST_STATE/test_manifests.json"
    test ! -e "$REQVIRE_GH_TEST_STATE/test_project.json"
    rm "$REQVIRE_GH_TEST_STATE/deny-root"
    impact_rpc reqvire.github.pr.create "$arguments" "$native_case/retry-prs.json"
    jq -e '.result.structuredContent.pull_requests | length == 3' "$native_case/retry-prs.json"
    jq -e -s '[.[] | select(.operation == "pr create" and .repository != "test/project")] | length == 2' \
        "$REQVIRE_GH_TEST_STATE/calls.jsonl"
}

composition_failed_push() {
    composition_publication_prepare
    touch "$native_case/deny-component"
    if impact_rpc reqvire.git.push '{"branch":"feature/topic"}' "$native_case/denied-push.json"; then
        echo 'Dependency push denial was ignored' >&2
        return 1
    fi
    jq -e -s 'all(.[]; all(.targets[]; contains("project.git") | not))' "$native_case/pushes.jsonl"
    test "$(git -C "$central" rev-parse HEAD)" = "$published_root"
    rm "$native_case/deny-component"
    impact_rpc reqvire.git.push '{"branch":"feature/topic"}' "$native_case/retry-push.json"
    test "$(git -C "$native_case/remotes/component" rev-parse refs/heads/feature/topic)" = "$published_component"
    if git -C "$native_case/remotes/docs" show-ref --verify --quiet refs/heads/feature/topic; then
        echo 'Untouched documentation branch was unnecessarily published' >&2
        return 1
    fi
}

composition_fresh_consumer() {
    composition_publication_prepare
    impact_rpc reqvire.git.push '{"branch":"feature/topic"}' "$native_case/published.json"
    (cd "$central" && "$REQVIRE_BIN" search --json) > "$native_case/published-model.json"
    jq '[.files[].elements[].name] | sort' "$native_case/published-model.json" > "$native_case/published-names.json"
    composition_stop
    local consumer="$native_case/fresh-consumer"
    "$REQVIRE_REPO_TEST_GIT_BINARY" clone -q --branch feature/topic "$native_case/remotes/central" "$consumer"
    (
        cd "$consumer"
        # Preparation alone rewrites fixture GitHub transport URLs to local
        # bare remotes. Stored manifest/config URLs retain their real identities.
        export GIT_CONFIG_COUNT=1
        export GIT_CONFIG_KEY_0="url.file://$native_case/remotes/.insteadOf"
        export GIT_CONFIG_VALUE_0=https://github.com/test/
        repo init -q -u https://github.com/test/manifests -b feature/topic -m default.xml \
            --repo-url "file://$native_source" --repo-rev "$(git -C "$native_source" rev-parse HEAD)" \
            --no-repo-verify --no-clone-bundle
        repo sync -q --no-clone-bundle --no-tags
    )
    test ! -e "$consumer/default.xml"
    test "$(git -C "$consumer/repositories/component" rev-parse HEAD)" = "$published_component"
    (cd "$consumer" && "$REQVIRE_BIN" validate)
    (cd "$consumer" && "$REQVIRE_BIN" search --json) > "$native_case/consumer-model.json"
    jq '[.files[].elements[].name] | sort' "$native_case/consumer-model.json" > "$native_case/consumer-names.json"
    diff -u "$native_case/published-names.json" "$native_case/consumer-names.json"
    (
        central=$consumer
        native="$consumer/.repo/manifests/default.xml"
        impact_state > "$native_case/consumer-before.txt"
        cd "$consumer"
        "$REQVIRE_BIN" change-impact --git-commit "$impact_base" --json > "$native_case/consumer-impact.json"
        impact_state > "$native_case/consumer-after.txt"
        diff -u "$native_case/consumer-before.txt" "$native_case/consumer-after.txt"
    )
    jq -e 'any(.added[]; .element_id | endswith("#added-capability"))' "$native_case/consumer-impact.json"
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
history cli/native-composition-history
input_history cli/native-includes-and-versioned-overlay-history
bootstrap mcp/initial-native-composition-checkpoint
branches mcp/native-coordinated-branch-base
shared_history mcp/shared-root-manifest-history-and-reopen
explicit mcp/native-committed-checkpoint-and-restart
automatic mcp/native-automatic-checkpoint-and-restart
external mcp/prepared-composition-explicit-open
invalid_open mcp/rejected-admission-retains-accepted-reads
pending_open mcp/pending-admission-retains-accepted-edits
review cli/pr-baseline-and-explicit-head
contract mcp/composition-contract-discovery
collision mcp/component-branch-collision-preserves-source
conflicting_owner mcp/conflicting-owner-branches-preserve-refs
overlay mcp/unversioned-overlay-cannot-checkpoint
manifest_failure mcp/manifest-checkpoint-failure-retains-components
root_failure mcp/root-checkpoint-failure-retains-manifest
component_unknown mcp/unknown-component-ref-retains-recovery
manifest_unknown mcp/unknown-manifest-ref-retains-recovery
prs mcp/coordinated-linked-pr-set
partial_prs mcp/partial-pr-set-retained-and-reused
push_failure mcp/dependency-push-failure-and-retry
fresh_consumer cli/published-native-composition-fresh-consumer
CASES
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    set -Eeuo pipefail
    trap 'printf "FAIL composition case %s at line %s: %s\n" "${1:-unknown}" "$LINENO" "$BASH_COMMAND" >&2' ERR
    composition_prepare "$1"
    case "$1" in
        history) composition_native_history ;;
        input_history) composition_native_input_history ;;
        bootstrap) composition_bootstrap ;;
        branches) composition_branch_set ;;
        shared_history) composition_shared_history ;;
        explicit|automatic) composition_checkpoint "$1" ;;
        external) composition_external_open ;;
        invalid_open) composition_invalid_open ;;
        pending_open) composition_pending_open ;;
        review) composition_review_baseline ;;
        contract) composition_contract ;;
        collision) composition_collision ;;
        conflicting_owner) composition_conflicting_owner ;;
        overlay) composition_unversioned_overlay ;;
        manifest_failure) composition_checkpoint_failure manifest ;;
        root_failure) composition_checkpoint_failure root ;;
        component_unknown|manifest_unknown) composition_unknown_ref "$1" ;;
        prs) composition_coordinated_prs ;;
        partial_prs) composition_partial_prs ;;
        push_failure) composition_failed_push ;;
        fresh_consumer) composition_fresh_consumer ;;
    esac
fi
