#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: submodel_collect_downstream_compare.sh [options]

Options:
  --base-ref REF             Git ref/commit to compare against (default: HEAD~1)
  --base-worktree PATH       Optional existing worktree path for base ref
  --out-root PATH            Output root directory (default: /tmp/submodel-collect-compare)
  --roots-file PATH          Precomputed root names file (one name per line)
  --direction DIR            reqvire collect direction: DOWNSTREAM|UPSTREAM (default: DOWNSTREAM)
  --reqvire-cmd CMD          reqvire executable/path (default: reqvire)
  -h, --help                Show this help
EOF
}

REQVIRE_CMD="reqvire"
if git -C "$(pwd)" rev-parse --show-toplevel >/dev/null 2>&1; then
  HEAD_REPO="$(git -C "$(pwd)" rev-parse --show-toplevel)"
else
  HEAD_REPO="$(pwd)"
fi
BASE_REF="HEAD~1"
BASE_WT=""
OUT_ROOT="/tmp/submodel-collect-compare"
DIRECTION="DOWNSTREAM"
ROOTS_FILE=""
AUTO_BASE_WT=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-ref)
      [[ $# -ge 2 ]] || { echo "Missing value for --base-ref"; exit 1; }
      BASE_REF="$2"
      shift 2
      ;;
    --base-worktree)
      [[ $# -ge 2 ]] || { echo "Missing value for --base-worktree"; exit 1; }
      BASE_WT="$2"
      shift 2
      ;;
    --out-root)
      [[ $# -ge 2 ]] || { echo "Missing value for --out-root"; exit 1; }
      OUT_ROOT="$2"
      shift 2
      ;;
    --roots-file)
      [[ $# -ge 2 ]] || { echo "Missing value for --roots-file"; exit 1; }
      ROOTS_FILE="$2"
      shift 2
      ;;
    --direction)
      [[ $# -ge 2 ]] || { echo "Missing value for --direction"; exit 1; }
      DIRECTION="$2"
      shift 2
      ;;
    --reqvire-cmd)
      [[ $# -ge 2 ]] || { echo "Missing value for --reqvire-cmd"; exit 1; }
      REQVIRE_CMD="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1"
      usage
      exit 1
      ;;
  esac
done

if [[ "${DIRECTION^^}" != "DOWNSTREAM" && "${DIRECTION^^}" != "UPSTREAM" ]]; then
  echo "Invalid direction: ${DIRECTION}. Use DOWNSTREAM or UPSTREAM."
  exit 1
fi
DIRECTION="${DIRECTION^^}"

if [[ -z "${ROOTS_FILE}" ]]; then
  ROOTS_FILE="$OUT_ROOT/root-names.txt"
else
  if [[ ! -f "${ROOTS_FILE}" ]]; then
    echo "Roots file not found: ${ROOTS_FILE}"
    exit 1
  fi
fi

HEAD_DIR="$OUT_ROOT/head"
BASE_DIR="$OUT_ROOT/base"
DIFF_DIR="$OUT_ROOT/diff"
RUN_LOG="$OUT_ROOT/run.log"
SUMMARY="$OUT_ROOT/summary.txt"
RUN_SUMMARY="$OUT_ROOT/run-summary.txt"
mkdir -p "$OUT_ROOT" "$HEAD_DIR" "$BASE_DIR" "$DIFF_DIR"
: > "$RUN_LOG"
: > "$SUMMARY"

if [[ -z "${BASE_WT}" ]]; then
  BASE_WT="$(mktemp -d -t submodel-collect-base-wt-XXXXXX)"
  AUTO_BASE_WT=1
fi
AUTO_CLEANUP=1

run_collect() {
  local repo_dir="$1"
  local out_dir="$2"
  local mode="$3"
  local roots_file="$4"

  local total=0
  local fail=0

  while IFS= read -r root || [ -n "$root" ]; do
    [[ -z "$root" ]] && continue
    local safe_name
    safe_name=$(printf '%s' "$root" | tr ' /' '__')
    local out_json="$out_dir/${safe_name}.json"
    local err_file="$out_dir/${safe_name}.err"

    echo "[${mode}] collect: $root" >> "$RUN_LOG"
    total=$((total+1))

    if (cd "$repo_dir" && "$REQVIRE_CMD" collect "$root" --direction "$DIRECTION" --json > "$out_json" 2> "$err_file"); then
      :
    else
      fail=$((fail+1))
      echo "[${mode}] failed: $root" >> "$RUN_LOG"
    fi
  done < "$roots_file"

  echo "$total" > "$out_dir/_total_count.txt"
  echo "$fail" > "$out_dir/_failed_count.txt"
}

trap '[[ "$AUTO_BASE_WT" == "1" ]] && git worktree remove -f "$BASE_WT" >/dev/null 2>&1 || true' EXIT

if [[ ! -f "$ROOTS_FILE" ]]; then
  mkdir -p "$OUT_ROOT"
  : > "$RUN_LOG"
  : > "$SUMMARY"
  (cd "$HEAD_REPO" && "$REQVIRE_CMD" submodels --json | jq -r '.submodels[]?.root_name' > "$ROOTS_FILE")
fi

ROOT_COUNT=$(wc -l < "$ROOTS_FILE")

run_collect "$HEAD_REPO" "$HEAD_DIR" "HEAD" "$ROOTS_FILE"
HEAD_TOTAL=$(cat "$HEAD_DIR/_total_count.txt")
HEAD_FAIL=$(cat "$HEAD_DIR/_failed_count.txt")

if [[ -d "$BASE_WT" ]] && [[ "$AUTO_BASE_WT" == "0" ]]; then
  git worktree remove -f "$BASE_WT" >/dev/null 2>&1 || true
fi

git worktree add --detach "$BASE_WT" "$BASE_REF" >/dev/null
run_collect "$BASE_WT" "$BASE_DIR" "BASE" "$ROOTS_FILE"
BASE_TOTAL=$(cat "$BASE_DIR/_total_count.txt")
BASE_FAIL=$(cat "$BASE_DIR/_failed_count.txt")

echo "Started diff across roots" >> "$RUN_LOG"

CHANGED_ROOTS=0
CHANGED_LINES=0
ROOTS_PROCESSED=0
DIFF_FILES=0

while IFS= read -r root || [ -n "$root" ]; do
  [[ -z "$root" ]] && continue
  ROOTS_PROCESSED=$((ROOTS_PROCESSED+1))

  safe_name=$(printf '%s' "$root" | tr ' /' '__')
  head_file="$HEAD_DIR/${safe_name}.json"
  base_file="$BASE_DIR/${safe_name}.json"
  diff_file="$DIFF_DIR/${safe_name}.diff"

  if [ ! -f "$head_file" ] || [ ! -f "$base_file" ]; then
    echo "[missing] $root" >> "$SUMMARY"
    : > "$diff_file"
    echo "[missing-diff] $root" >> "$SUMMARY"
    continue
  fi

  if diff -u "$base_file" "$head_file" > "$diff_file"; then
    DIFF_FILES=$((DIFF_FILES+1))
    echo "[same] $root" >> "$SUMMARY"
    continue
  else
    diff_status=$?
    if [ "$diff_status" -gt 1 ]; then
      echo "[diff-failed] $root status=$diff_status" >> "$SUMMARY"
      : > "$diff_file"
      continue
    fi

    adds=$(grep -c '^\+[^+]' "$diff_file" || true)
    dels=$(grep -c '^-[^-]' "$diff_file" || true)
    CHANGED_ROOTS=$((CHANGED_ROOTS+1))
    CHANGED_LINES=$((CHANGED_LINES + adds + dels))
    DIFF_FILES=$((DIFF_FILES+1))
    echo "[changed] $root (+$adds -$dels)" >> "$SUMMARY"
  fi

done < "$ROOTS_FILE"

{
  echo "SUBMODEL ${DIRECTION} COLLECT COMPARISON"
  echo "Created: $(date -Iseconds)"
  echo "Base ref: $BASE_REF"
  echo "Head repo: $HEAD_REPO"
  echo "Roots discovered: $ROOT_COUNT"
  echo "Roots processed: $ROOTS_PROCESSED"
  echo "Head total collected: $HEAD_TOTAL"
  echo "Head failed: $HEAD_FAIL"
  echo "Base total collected: $BASE_TOTAL"
  echo "Base failed: $BASE_FAIL"
  echo "Changed roots: $CHANGED_ROOTS"
  echo "Diff files written: $DIFF_FILES"
  echo "Approx diff line changes: $CHANGED_LINES"
  echo "Output root: $OUT_ROOT"
} > "$RUN_SUMMARY"

echo "run-summary: $RUN_SUMMARY"
echo "per-root summary: $SUMMARY"
echo "diff files: $DIFF_DIR"
cat "$RUN_SUMMARY"
