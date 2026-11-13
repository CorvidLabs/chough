#!/bin/sh
# Host-matrix end-to-end harness (REQ-cli-066; LIFE-21 item 4, LIFE-22, LIFE-24, LIFE-27).
#
# Drives one fixed Chough cycle (init, intent, plan, build, test, accept, done, then a
# cold resume) in a throwaway copy of examples/shipping through one agent host, one
# fresh host run per turn. Between turns it stands in for every human gate as
# SIMULATED-host-matrix-not-a-human, labelled SIMULATED; it never records a decision
# in a person's name. The verdict comes only from the sample's files and native
# status, never from what the agent says it did.
#
# Usage: tools/host-matrix/run.sh <host> [--adapter <file>] [--work <empty-dir>] [--logs <dir>]
#   <host>       a folder under integrations/ holding adapter.sh, or `stand-in`, the
#                scripted host with no model that the self-test uses
#   --adapter    use this adapter file instead (any CLI agent; see README.md)
#   --work       run in this empty directory (default: a new temporary one)
#   --logs       keep the raw logs here (default: a new temporary directory); they are
#                never committed
# Environment: CHOUGH, the chough under test (default: chough on PATH);
#              HOST_MATRIX_FAULT, passed to the host (the stand-in's known-bad runs).
#
# Prints one JSON result on stdout and exits 0 for PASS, 1 for FAIL and 2 for
# UNAVAILABLE (the host cannot run here; it still counts as a missing leg). Exit 3 is
# a harness error: a missing tool or a bad argument.
set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
simulated=SIMULATED-host-matrix-not-a-human
environment=exercise-local
allowed_product_files="src/lib.rs tests/threshold.rs"

die() {
    printf 'host-matrix: %s\n' "$*" >&2
    exit 3
}

[ $# -ge 1 ] || die "usage: run.sh <host> [--adapter <file>] [--work <empty-dir>] [--logs <dir>]"
host=$1
shift
case "$host" in
*[!a-z0-9-]* | "") die "host must be lowercase letters, digits and hyphens: $host" ;;
esac
adapter="" work="" logs=""
while [ $# -gt 0 ]; do
    [ $# -ge 2 ] || die "$1 needs a value"
    case "$1" in
    --adapter) adapter=$2 ;;
    --work) work=$2 ;;
    --logs) logs=$2 ;;
    *) die "unknown option: $1" ;;
    esac
    shift 2
done
if [ -z "$adapter" ]; then
    if [ "$host" = stand-in ]; then
        adapter=$here/stand-in/adapter.sh
    else
        adapter=$root/integrations/$host/adapter.sh
    fi
fi
[ -f "$adapter" ] || die "no adapter for $host: $adapter"
adapter=$(cd "$(dirname "$adapter")" && pwd)/$(basename "$adapter")

for tool in git jq cargo hi specsync; do
    command -v "$tool" >/dev/null 2>&1 || die "missing required tool: $tool"
done
chough_bin=${CHOUGH:-$(command -v chough || true)}
[ -n "$chough_bin" ] && [ -x "$chough_bin" ] || die "no chough under test: set CHOUGH or put chough on PATH"
chough_bin=$(cd "$(dirname "$chough_bin")" && pwd)/$(basename "$chough_bin")

if [ -z "$logs" ]; then
    logs=$(mktemp -d "${TMPDIR:-/tmp}/chough-host-matrix-logs.XXXXXX")
fi
mkdir -p "$logs"
logs=$(cd "$logs" && pwd)
if [ -z "$work" ]; then
    work=$(mktemp -d "${TMPDIR:-/tmp}/chough-host-matrix.XXXXXX")
fi
mkdir -p "$work"
work=$(cd "$work" && pwd)
[ -z "$(ls -A "$work")" ] || die "$work is not empty"

# The chough under test comes first on PATH for the host and for the harness.
mkdir -p "$logs/bin"
ln -sf "$chough_bin" "$logs/bin/chough"
PATH="$logs/bin:$PATH"
HM_ROOT=$root
export PATH HM_ROOT

# shellcheck source=/dev/null
. "$adapter"
: "${HOST_LABEL:=$host}"
: "${HOST_INIT_ARGS:=--host openai}"
: "${HOST_SKILL:=.agents/skills/chough-workflow/SKILL.md}"
: "${HOST_SKILL_READ:=chough-workflow/SKILL\\.md}"
: "${HOST_TURN_TIMEOUT:=1800}"

# Each host turn is a fresh, unattended session of its own, not part of the agent or
# terminal that started the harness, and Chough records it as that host (REQ-records-007).
for variable in $(env | sed -n 's/^\(HC_[A-Z0-9_]*\)=.*/\1/p'); do unset "$variable"; done
unset CLAUDECODE CLAUDE_CODE_ENTRYPOINT CLAUDE_CODE_SESSION_ID CLAUDE_CODE_CHILD_SESSION \
    CLAUDE_CODE_SESSION_ATTENDED CLAUDE_CODE_MESSAGING_SOCKET CLAUDE_CODE_MESSAGING_TOKEN \
    CLAUDE_CODE_EXECPATH CLAUDE_CODE_PLUGIN_DIRS CLAUDE_PID CLAUDE_EFFORT \
    CHOUGH_HC_TERMINAL CHOUGH_ACTOR_MODEL
as_harness() {
    CHOUGH_ACTOR="host-matrix-harness"
    export CHOUGH_ACTOR
}
as_host() {
    CHOUGH_ACTOR=$HOST_LABEL
    export CHOUGH_ACTOR
}
as_harness

failures=$logs/failures.txt
: >"$failures"
turns_done=0
failed_log=""

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then sha256sum | cut -d' ' -f1; else shasum -a 256 | cut -d' ' -f1; fi
}

finish() {
    verdict=$1
    status=0
    [ "$verdict" = FAIL ] && status=1
    [ "$verdict" = UNAVAILABLE ] && status=2
    digest=$(cat "$logs"/turn-*.log 2>/dev/null | sha256)
    key_lines=""
    if [ -n "$failed_log" ] && [ -f "$failed_log" ]; then
        key_lines=$(tail -n 20 "$failed_log")
    fi
    jq -n \
        --arg host "$host" --arg label "$HOST_LABEL" --arg version "${host_version_line:-}" \
        --arg platform "$(uname -s)-$(uname -m)" --arg verdict "$verdict" \
        --arg chough "$("$chough_bin" --version 2>/dev/null || echo unknown)" \
        --arg commit "$(git -C "$root" rev-parse HEAD 2>/dev/null || echo unknown)" \
        --arg command "tools/host-matrix/run.sh $host" --arg log_sha256 "$digest" \
        --arg key_lines "$key_lines" --argjson turns "$turns_done" \
        --rawfile failed "$failures" \
        '{verdict: $verdict, host: $host, host_label: $label, host_version: $version,
          platform: $platform, chough: $chough, chough_commit: $commit, command: $command,
          turns_completed: $turns, failed: ($failed | split("\n") | map(select(length > 0))),
          log_sha256: $log_sha256, key_failing_lines: $key_lines}' |
        tee "$logs/result.json"
    exit "$status"
}

# check <description> <command...>: record a failed check without stopping.
check() {
    description=$1
    shift
    if ! "$@" >/dev/null 2>&1; then
        printf '%s\n' "$description" >>"$failures"
    fi
}

stop_if_failed() {
    if [ -s "$failures" ]; then
        [ -n "$failed_log" ] || failed_log=$logs/turn-$1.log
        finish FAIL
    fi
}

# with_deadline <seconds> <command...>: run it in its own process group and end the
# whole group when time runs out.
with_deadline() {
    seconds=$1
    shift
    set -m
    "$@" &
    job=$!
    (sleep "$seconds" && kill -TERM -"$job" 2>/dev/null && sleep 10 && kill -KILL -"$job" 2>/dev/null) &
    watcher=$!
    set +m
    set +e
    wait "$job"
    job_status=$?
    set -e
    kill -TERM -"$watcher" 2>/dev/null || true
    wait "$watcher" 2>/dev/null || true
    return "$job_status"
}

next_json() {
    chough next --environment "$environment" --json 2>/dev/null
}
reported() {
    grep -E "^$1:" "$logs/turn-$2.reply" 2>/dev/null | tail -n 1 | sed "s/^$1:[[:space:]]*//; s/[[:space:]]*\$//" |
        tr '[:upper:]' '[:lower:]'
}
tree_clean() {
    [ -z "$(git status --porcelain)" ]
}
jq_true() {
    printf '%s' "$1" | jq -e "$2"
}

run_turn() {
    n=$1
    prompt=$logs/turn-$n.prompt
    {
        cat "$here/prompts/common.md"
        cat "$here/prompts/turn-$n.md"
    } | sed "s|{{ENVIRONMENT}}|$environment|g; s|{{INIT_ARGS}}|$HOST_INIT_ARGS|g" >"$prompt"
    : >"$logs/turn-$n.reply"
    as_host
    if ! with_deadline "$HOST_TURN_TIMEOUT" host_turn "$prompt" "$logs/turn-$n.log" "$logs/turn-$n.reply"; then
        printf 'turn %s: the host run ended without an error within %ss\n' "$n" "$HOST_TURN_TIMEOUT" >>"$failures"
    fi
    as_harness
    turns_done=$n
    check "turn $n: the tree is committed" tree_clean
    check "turn $n: the reply ends with STAGE: and NEXT: lines" \
        sh -c '[ -n "$1" ] && [ -n "$2" ]' _ "$(reported STAGE "$n")" "$(reported NEXT "$n")"
}

commit_stand_in() {
    git add -A
    git commit -qm "SIMULATED stand-in for the human: $1"
}

# --- The host ---------------------------------------------------------------
if ! reason=$(host_check 2>&1); then
    printf 'the host cannot run here: %s\n' "$reason" >>"$failures"
    finish UNAVAILABLE
fi
host_version_line=$(host_version 2>/dev/null | head -n 1 || true)

# --- Seed -------------------------------------------------------------------
cp -R "$root/examples/shipping/." "$work/"
cd "$work"
# The sample's patches hold the solution; a host must write its own.
rm -rf patches target .specsync/changes/.keep .specsync/archive/changes/.keep
git -c init.defaultBranch=main init -q
git config user.name "Chough host matrix"
git config user.email host-matrix@example.invalid
git config commit.gpgsign false
specsync change adopt --json >/dev/null 2>&1 || die "specsync change adopt failed in the sample"
# As in a project that adopted Chough: its records stay out of SpecSync verification.
# The sample's inert `specs/` ignore entry only makes SpecSync print a note on every command.
jq '.verification_ignored_paths = [".chough/sessions/", ".chough/archive/", ".chough/bugs/"]
    | .ignored_paths -= ["specs/"]' \
    .specsync/sdd.json >"$logs/sdd.json" && cp "$logs/sdd.json" .specsync/sdd.json
git add -A
git commit -qm "Seed the shipping sample for the host matrix"
seed=$(git rev-parse HEAD)

# --- Turn 1: init and intent -------------------------------------------------
run_turn 1
check "turn 1: the skill is installed at $HOST_SKILL" test -f "$HOST_SKILL"
check "turn 1: the host read the skill" grep -Eq "$HOST_SKILL_READ" "$logs/turn-1.log"
n1=$(next_json || echo '{}')
check "turn 1: an open session is at intent" jq_true "$n1" '.stage.current == "intent"'
check "turn 1: an intent gate is open" jq_true "$n1" '[.gates[]? | select(.holds == "intent")] | length >= 1'
check "turn 1: the reply states stage intent" test "$(reported STAGE 1)" = intent
stop_if_failed 1
chough checkpoint --clear-awaiting-human \
    --decision "SIMULATED: the host-matrix harness stands in for the intent approval; no human decided" >/dev/null
commit_stand_in "intent approved (SIMULATED)"

# --- Turn 2: plan -------------------------------------------------------------
run_turn 2
n2=$(next_json || echo '{}')
id=$(printf '%s' "$n2" | jq -r '.native.change // empty')
check "turn 2: the session is at plan" jq_true "$n2" '.stage.current == "plan"'
check "turn 2: a native change is recorded" test -n "$id"
check "turn 2: the change waits in draft for the scope decision" jq_true "$n2" '.native.state == "draft"'
check "turn 2: the scope gate is open and drives approval" \
    jq_true "$n2" '[.gates[]? | select(.holds == "plan" and .drives == "approve")] | length >= 1'
check "turn 2: the reply states stage plan" test "$(reported STAGE 2)" = plan
stop_if_failed 2
if ! specsync change approve "$id" --actor "$simulated" \
    --note "SIMULATED: the host-matrix harness stands in for the scope approval; no human decided" \
    >"$logs/approve.log" 2>&1; then
    printf 'turn 2: the drafted change is ready to approve\n' >>"$failures"
    failed_log=$logs/approve.log
    finish FAIL
fi
chough checkpoint --clear-awaiting-human \
    --decision "SIMULATED: scope of $id approved by $simulated; no human decided" >/dev/null
commit_stand_in "scope approved (SIMULATED)"

# --- Turn 3: build and test ---------------------------------------------------
run_turn 3
n3=$(next_json || echo '{}')
check "turn 3: the session is at accept" jq_true "$n3" '.stage.current == "accept"'
check "turn 3: the change is verifying under a valid approval" \
    jq_true "$n3" '.native.state == "verifying" and .native.approval_valid == true'
check "turn 3: the review gate is open and drives review" \
    jq_true "$n3" '[.gates[]? | select(.holds == "accept" and .drives == "review")] | length >= 1'
check "turn 3: the reply states stage accept" test "$(reported STAGE 3)" = accept
product=$(git diff --name-only "$seed" HEAD -- . ':(exclude).chough' ':(exclude).specsync' ':(exclude)specs' \
    ':(exclude)hi' ':(exclude)knowledge' ':(exclude).agents' ':(exclude).gitattributes' | sort | tr '\n' ' ')
printf '%s\n' "$product" >"$logs/product-files.txt"
check "turn 3: only the allowed product files changed ($allowed_product_files)" \
    test "$product" = "$allowed_product_files "
rm -rf "$logs/acceptance"
git clone -q "$work" "$logs/acceptance"
mkdir -p "$logs/acceptance/tests"
cp "$here/acceptance.rs" "$logs/acceptance/tests/host_matrix_acceptance.rs"
check "turn 3: the harness's own acceptance test passes" \
    sh -c 'cd "$1" && cargo test --quiet --test host_matrix_acceptance >"$2" 2>&1' _ \
    "$logs/acceptance" "$logs/acceptance.log"
stop_if_failed 3
review_and_ship() {
    specsync change review "$id" --reviewer "$simulated" --verdict pass &&
        specsync change ship "$id"
}
if ! review_and_ship >"$logs/review.log" 2>&1; then
    printf 'turn 3: the change passes review and ships\n' >>"$failures"
    failed_log=$logs/review.log
    finish FAIL
fi
chough checkpoint --clear-awaiting-human \
    --decision "SIMULATED: implementation of $id reviewed by $simulated and archived; no human decided" >/dev/null
commit_stand_in "implementation reviewed and archived (SIMULATED)"

# --- Turn 4: done -------------------------------------------------------------
run_turn 4
check "turn 4: no session is left open" sh -c '! chough next --environment "$1" --json' _ "$environment"
check "turn 4: no session folder is left" sh -c '[ -z "$(ls -A .chough/sessions 2>/dev/null)" ]'
check "turn 4: one archived session story" \
    sh -c '[ "$(ls .chough/archive/sessions/*.md 2>/dev/null | wc -l | tr -d " ")" = 1 ]'
check "turn 4: the native change is archived" \
    sh -c 'specsync change status "$1" --json | jq -e ".summary.state == \"archived\""' _ "$id"
check "turn 4: the reply states stage done" test "$(reported STAGE 4)" = "done"
stop_if_failed 4

# --- Turn 5: cold resume ------------------------------------------------------
before=$(git rev-parse HEAD)
run_turn 5
check "turn 5: the cold resume changed nothing" sh -c '[ "$(git rev-parse HEAD)" = "$1" ]' _ "$before"
check "turn 5: the host read the skill" grep -Eq "$HOST_SKILL_READ" "$logs/turn-5.log"
check "turn 5: the cold resume states stage done" test "$(reported STAGE 5)" = "done"
check "turn 5: the cold resume names chough session new" \
    sh -c 'printf "%s" "$1" | grep -q "session new"' _ "$(reported NEXT 5)"
stop_if_failed 5

# --- The records' shape -------------------------------------------------------
sh "$here/shape.sh" "$work" "$id" "$seed" >"$logs/shape.json" 2>"$logs/shape.err" ||
    printf "the records' shape can be read\n" >>"$failures"
if ! jq -S . "$here/expected-shape.json" >"$logs/expected.json" ||
    ! jq -S . "$logs/shape.json" >"$logs/actual.json" ||
    ! diff -u "$logs/expected.json" "$logs/actual.json" >"$logs/shape.diff"; then
    printf 'the records match expected-shape.json\n' >>"$failures"
    failed_log=$logs/shape.diff
fi
check "chough doctor --leaks is clean" chough doctor --leaks
stop_if_failed 5
finish PASS
