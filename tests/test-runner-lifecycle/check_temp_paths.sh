#!/bin/bash
set -euo pipefail
suite_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
root=$1
mkdir -p "$root/physical/fixtures" "$root/physical/logs" "$root/suites/test-path"
ln -s "$root/physical" "$root/alias"
for file in run_tests.sh reqvire_timing_wrapper.sh stop_test_processes.py; do
    cp "$suite_dir/../$file" "$root/suites/$file"
done
printf '#!/bin/bash\nprintf "stub invocation\\n"\n' > "$root/reqvire"
chmod +x "$root/reqvire"
cat > "$root/suites/test-path/test.sh" <<'TEST'
#!/bin/bash
set -euo pipefail
[[ "$TEST_DIR" == "$(cd "$TEST_DIR" && pwd -P)" ]]
"$REQVIRE_BIN" --version
TEST
TMPDIR="$root/alias/fixtures" REQVIRE_TEST_LOG_DIR="$root/alias/logs" \
    REQVIRE_BIN="$root/reqvire" bash "$root/suites/run_tests.sh" test-path > "$root/result.txt" 2>&1
logs=("$root/physical/logs"/run-*)
[[ ${#logs[@]} == 1 ]]
grep -Fq "Test logs directory: ${logs[0]}" "$root/result.txt"
grep -Fq 'Tests: 1 passed, 0 failed, 1 total' "$root/result.txt"
printf 'PASS canonical-temporary-and-log-paths\n'
