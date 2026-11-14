#!/bin/sh
# Self-test of the host-matrix harness (REQ-cli-066), with the scripted stand-in host
# and no model. Passes only when the stand-in's good run is judged PASS and each
# known-bad run is judged FAIL on the check that names its fault.
#
# Usage: tools/host-matrix/selftest.sh
# Environment: CHOUGH, the chough under test (default: chough on PATH).
set -eu

here=$(cd "$(dirname "$0")" && pwd)
scratch=$(mktemp -d "${TMPDIR:-/tmp}/chough-host-matrix-selftest.XXXXXX")
scratch=$(cd "$scratch" && pwd)
keep=""
cleanup() {
    if [ -z "$keep" ]; then rm -rf "$scratch"; else printf 'Kept %s\n' "$scratch" >&2; fi
}
trap cleanup EXIT
# One build of the sample's crate serves every run.
CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$scratch/target}
export CARGO_TARGET_DIR

fail() {
    keep=yes
    printf 'HOST MATRIX SELFTEST FAILED: %s\n' "$*" >&2
    exit 1
}

# expect <name> <verdict> <failed-check fragment, or ""> [fault [adapter]]
expect() {
    name=$1 verdict=$2 fragment=$3 fault=${4:-} adapter=${5:-$here/stand-in/adapter.sh}
    set +e
    result=$(HOST_MATRIX_FAULT=$fault sh "$here/run.sh" stand-in --adapter "$adapter" \
        --work "$scratch/$name/work" --logs "$scratch/$name/logs")
    set -e
    got=$(printf '%s' "$result" | jq -r .verdict)
    [ "$got" = "$verdict" ] || fail "$name: expected $verdict, got ${got:-no result}: $result"
    if [ -n "$fragment" ]; then
        printf '%s' "$result" | jq -e --arg f "$fragment" '.failed | any(contains($f))' >/dev/null ||
            fail "$name: expected a failed check naming '$fragment': $result"
    fi
    printf 'ok: %s -> %s%s\n' "$name" "$got" "${fragment:+ ($fragment)}"
}

expect good PASS ""
expect uncommitted FAIL "turn 1: the tree is committed" uncommitted
expect skip-gate FAIL "turn 2: the scope gate is open" skip-gate
expect approve-itself FAIL "turn 2: the change waits in draft" approve-itself
expect extra-file FAIL "turn 3: only the allowed product files changed" extra-file
expect wrong-resume FAIL "turn 5: the cold resume states stage done" wrong-resume
# The hung turn must be ended at its limit, with its process group, under any sh: dash,
# Ubuntu's sh, has no job control without a terminal (BUG: the turn deadline under dash).
(
    HOST_TURN_TIMEOUT=3
    export HOST_TURN_TIMEOUT
    started=$(date +%s)
    expect hang FAIL "turn 1: the host run ended without an error within 3s" hang
    elapsed=$(($(date +%s) - started))
    [ "$elapsed" -lt 60 ] || fail "hang: the hung turn was not ended at its limit (${elapsed}s)"
)
cat >"$scratch/missing-host.sh" <<'EOF'
host_check() { echo "the host's CLI is not installed"; return 1; }
host_version() { :; }
host_turn() { return 1; }
EOF
expect missing-host UNAVAILABLE "the host cannot run here" "" "$scratch/missing-host.sh"
printf 'HOST MATRIX SELFTEST PASSED\n'
