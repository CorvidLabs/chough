#!/bin/sh
# Feature-and-bug exercise in a disposable copy of examples/shipping.
#
# Runs the real chough, hi, specsync, cargo and git: a nonvisual design skip, a
# native scope gate, a deliberately failing assertion, a local blocking bug, a fix
# and a focused retest, stopping at acceptance where only a human may review.
# Human gates in this sample are SIMULATED and labelled; nothing is pushed or posted.
#
# Usage: examples/feature-bug-loop.sh [empty-workdir] [full|prepare-cold-start]
# Environment: CHOUGH (default: chough), CHOUGH_ENVIRONMENT (default: exercise-local).
set -eu

here=$(cd "$(dirname "$0")" && pwd)
work=${1:-$(mktemp -d "${TMPDIR:-/tmp}/chough-exercise.XXXXXX")}
mode=${2:-full}
chough=${CHOUGH:-chough}
env_name=${CHOUGH_ENVIRONMENT:-exercise-local}
last_observation=""

step() { printf '\n== %s\n' "$*"; }
fail() { printf 'EXERCISE FAILED: %s\n' "$*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing required tool: $1"; }
expect_refusal() {
    label=$1 pattern=$2
    shift 2
    if out=$("$@" 2>&1); then fail "$label unexpectedly succeeded"; fi
    printf '%s\n' "$out" | grep -q -- "$pattern" || fail "$label: expected '$pattern' in: $out"
    printf 'refused as expected: %s (%s)\n' "$label" "$pattern"
}
run_case() {
    set +e
    out=$($chough cases run "$1" --environment "$env_name")
    set -e
    outcome=$(printf '%s' "$out" | jq -r .outcome)
    last_observation=$(printf '%s' "$out" | jq -r .observation)
    [ "$outcome" = "$2" ] || fail "$1 expected $2 but was $outcome: $out"
    printf '%s: %s, observation %s\n' "$1" "$outcome" "$last_observation"
}
commit() {
    git add -A
    git -c user.name="Chough exercise" -c user.email=exercise@example.invalid \
        -c commit.gpgsign=false commit -qm "$1"
}
mark_tasks_done() {
    sed 's/- \[ \]/- [x]/' "$1" >"$1.tmp" && mv "$1.tmp" "$1"
}
artifact() {
    printf -- '---\nchange: %s\nartifact: %s\n---\n\n' "$id" "$1" >".specsync/changes/$id/$1.md"
    cat >>".specsync/changes/$id/$1.md"
}

for tool in "${chough%% *}" hi specsync cargo git jq; do need "$tool"; done
[ -z "$(ls -A "$work" 2>/dev/null)" ] || fail "$work is not empty"
mkdir -p "$work"
cp -R "$here/shipping/." "$work/"
cd "$work"
rm -f .specsync/changes/.keep .specsync/archive/changes/.keep
git -c init.defaultBranch=main init -q
# Native tools also commit (for example `specsync change check --commit`), so the
# disposable repository carries its own identity instead of relying on global config.
git config user.name "Chough exercise"
git config user.email exercise@example.invalid
git config commit.gpgsign false

step "Intent: sample criteria CHECKOUT-1 and CHECKOUT-2 (sample input, already in hi/)"
hi check --json | jq -e '.problems == []' >/dev/null
$chough init --objective "Free shipping from fifty dollars" --intent-scope CHECKOUT \
    --module shipping --host openai --host claude >/dev/null
specsync change adopt --json >/dev/null
$chough pref set repeat-validation off \
    --source "Sample project setting: no repeated validation unless someone asks" >/dev/null
commit "Seed the shipping sample"
$chough stage advance --note "SIMULATED: the exercise approves the sample intent; no human decided" >/dev/null

step "Design: nothing visual changes, so the stage is skipped with a reason"
$chough stage skip --reason "Fee calculation only; no visuals or interactions change" |
    jq -r '.applied | "\(.from) -> \(.to) (\(.kind))"'

step "Plan: a native SpecSync change, and the scope gate before build"
id=$(specsync change new "Free shipping at the threshold" --spec shipping \
    --path src/lib.rs --path tests/ --json | jq -r .change.id)
specsync change answer "$id" acceptance_criteria \
    '["An order of exactly 5000 cents ships free", "An order under 5000 cents pays the standard fee"]' --json >/dev/null
specsync change answer "$id" public_contract yes --json >/dev/null
remaining=$(specsync change answer "$id" architecture_risk no --json)
# The constraints question appears when the module already has lessons; the answer
# output lists it, while `change status` reports it through next_action.
if printf '%s' "$remaining" | jq -e '.questions | map(.id) | index("constraints")' >/dev/null; then
    specsync change answer "$id" constraints none --json >/dev/null
fi
for selected in $(specsync change status "$id" --json | jq -r '.change.selected_artifacts[]'); do
    case "$selected" in
    plan) artifact plan <<'EOF'
# Plan

## Approach

Add FREE_SHIPPING_FROM_CENTS and return a zero fee from that total upward.

## Out of scope

Discount codes, regional fees and checkout wording.

## Steps

1. Add the constant and the threshold rule.
2. Add boundary tests at, below and above the threshold.

## Risks

An off-by-one at the boundary; covered by TC-shipping-0001.

## Constraints consulted

None beyond the sample module context.
EOF
    ;;
    tasks) artifact tasks <<'EOF'
# Tasks

- [ ] Add the free-shipping threshold rule.
- [ ] Add boundary tests in tests/threshold.rs.
EOF
    ;;
    testing) artifact testing <<'EOF'
# Testing

REQ-shipping-002 is verified by TC-shipping-0001 (exactly the threshold) and
TC-shipping-0003 (above it), bound to tests/threshold.rs. REQ-shipping-001 keeps
TC-shipping-0002 as a regression check.
EOF
    ;;
    *) artifact "$selected" <<EOF
# ${selected}

Sample ${selected} notes for the free shipping threshold: orders of 5000 cents or
more ship free (CHECKOUT-1, REQ-shipping-002); others pay the standard fee.
EOF
    ;;
    esac
done
mkdir -p ".specsync/changes/$id/deltas"
cat >".specsync/changes/$id/deltas/shipping.md" <<'EOF'
## ADDED

### REQUIREMENT REQ-shipping-002

The module SHALL charge no shipping fee for an order total at or above the free-shipping threshold. Trace: CHECKOUT-1.

Acceptance Criteria
- tests/threshold.rs asserts a zero fee at exactly the threshold and above it.

## MODIFIED

### SPEC SECTION Public API

| Name | Description |
|------|-------------|
| `FREE_SHIPPING_FROM_CENTS` | Order total in cents from which shipping is free |
| `STANDARD_FEE_CENTS` | Standard shipping fee in cents |
| `shipping_fee` | Shipping fee in cents for an order total in cents |
EOF
commit "Draft the native change $id"
$chough checkpoint --native-change "$id" --awaiting-human "Scope approval of $id" >/dev/null
expect_refusal "entering build before scope approval" "refused:native-draft" $chough stage advance
specsync change approve "$id" --actor "SIMULATED-sample-approver-not-a-human" >/dev/null
$chough checkpoint --clear-awaiting-human \
    --decision "Scope of $id approved by a SIMULATED actor in this sample; not a human decision" >/dev/null
$chough stage advance | jq -r '"plan -> build, gate \(.applied.gate)"'

step "Build: implement the feature with its boundary tests"
cp patches/lib.v1.rs src/lib.rs
mkdir -p tests
cp patches/threshold.rs tests/threshold.rs
mark_tasks_done ".specsync/changes/$id/tasks.md"
commit "Add the free shipping threshold"
specsync change check "$id" --commit >/dev/null
$chough stage advance >/dev/null

step "Test: focused selection for the affected requirement and path"
selection=$($chough cases select --requirement REQ-shipping-002 --path src/lib.rs)
printf '%s' "$selection" | jq -r '"selected: \([.selected[].id] | join(", "))", (.omitted[] | "omitted: \(.id) (\(.reason))")'
[ "$(printf '%s' "$selection" | jq -r '[.selected[].id] | join(",")')" = \
    "TC-shipping-0001,TC-shipping-0002,TC-shipping-0003" ] || fail "unexpected selection"
run_case TC-shipping-0001 failed
failed_observation=$last_observation
run_case TC-shipping-0002 passed
run_case TC-shipping-0003 passed

step "Bug: reproduce, search, record locally, and refuse a duplicate"
$chough bug search "order of exactly fifty dollars is charged shipping" --case TC-shipping-0001 |
    jq -r '"existing matches: \(.matches | length)"'
bug=$($chough bug add --title "Shipping charged at exactly the free threshold" \
    --symptom "An order of exactly 5000 cents is charged the standard fee" \
    --trigger "Order total equal to FREE_SHIPPING_FROM_CENTS" \
    --expected "Fee of 0 (CHECKOUT-1)" --actual "Fee of 499" \
    --environment "$env_name" --criterion CHECKOUT-1 --requirement REQ-shipping-002 \
    --case TC-shipping-0001 --observation "$failed_observation" \
    --step "chough cases run TC-shipping-0001 --environment $env_name" \
    --classification product-defect --scope blocking | jq -r .id)
printf 'recorded %s: %s\n' "$bug" "$($chough bug show "$bug" | jq -r .verification)"
expect_refusal "a duplicate report" "similar bugs exist" $chough bug add \
    --title "Shipping charged at exactly the free threshold" \
    --symptom "An order of exactly 5000 cents is charged the standard fee" \
    --trigger "Order total equal to FREE_SHIPPING_FROM_CENTS" --expected "0" --actual "499" \
    --environment "$env_name" --case TC-shipping-0001
expect_refusal "accepting with an open blocking bug" "refused:blocking-bugs" $chough stage advance
expect_refusal "repeated validation without consent" "not authorized" \
    $chough validate run TC-shipping-0001 --times 3 --environment "$env_name"
$chough checkpoint --bug "$bug" --observation "$failed_observation" --case TC-shipping-0001 \
    --next-action "Fix $bug at the threshold boundary, then rerun the selected cases." >/dev/null

if [ "$mode" = "prepare-cold-start" ]; then
    commit "Record the failing case and $bug"
    printf '\nPrepared for a cold start at %s (stage test, %s open).\n' "$work" "$bug"
    exit 0
fi

step "Back to build: fix the boundary and retest the focused cases"
$chough stage back --to build --reason "$bug blocks acceptance" | jq -r '.applied.refresh[]'
cp patches/lib.v2.rs src/lib.rs
commit "Ship free at exactly the threshold ($bug)"
specsync change check "$id" --commit >/dev/null
$chough stage advance >/dev/null
run_case TC-shipping-0001 passed
fixed_observation=$last_observation
run_case TC-shipping-0002 passed
run_case TC-shipping-0003 passed
$chough bug update "$bug" --status verified --observation "$fixed_observation" \
    --note "Boundary fixed; TC-shipping-0001 passes" | jq -r '"\(.bug.id): \(.bug.status), \(.verification)"'

step "Accept: only a human review and native archive can close this"
$chough stage advance >/dev/null
expect_refusal "leaving accept before review and archive" "refused:native-verifying" $chough stage advance
$chough next --environment "$env_name" | jq -r '.next_valid_action'
printf '\nEXERCISE PASSED in %s\n' "$work"
