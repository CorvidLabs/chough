#!/bin/sh
# The scripted stand-in host for the host-matrix self-test (REQ-cli-066). No model.
#
# Each run is one fresh "agent turn" in the sample project. Like a fresh agent, it
# decides what to do from the project's own records (`chough next`), takes the work
# to the next human gate, commits, and ends with a STAGE:/NEXT: reply. It never
# approves or reviews anything: the harness stands in for those gates.
#
# HOST_MATRIX_FAULT makes it misbehave in one known way, so the self-test can show the
# judge catching it: uncommitted, skip-gate, approve-itself, extra-file, wrong-resume,
# and hang (never finishes its turn).
set -eu

fault=${HOST_MATRIX_FAULT:-}
patches="$HM_ROOT/examples/shipping/patches"
env_name=exercise-local
skill=.agents/skills/chough-workflow/SKILL.md

run() {
    printf '+ %s\n' "$*"
    "$@"
}
quiet() {
    printf '+ %s\n' "$*"
    "$@" >/dev/null
}
commit() {
    git add -A
    git commit -qm "$1"
}
reply() {
    printf -- '--- reply ---\n%s\nSTAGE: %s\nNEXT: %s\n' "$1" "$2" "$3"
}
read_skill() {
    printf '+ cat %s\n' "$skill"
    cat "$skill" >/dev/null
}
artifact() {
    printf -- '---\nchange: %s\nartifact: %s\n---\n\n' "$id" "$1" >".specsync/changes/$id/$1.md"
    cat >>".specsync/changes/$id/$1.md"
}

turn_intent() {
    printf '+ cat AGENTS.md\n'
    # shellcheck disable=SC2086 # HOST_INIT_ARGS is a word list by design
    quiet chough init --objective "Free shipping from fifty dollars" --intent-scope CHECKOUT \
        --module shipping $HOST_INIT_ARGS
    read_skill
    quiet hi check --json
    quiet chough checkpoint --criterion CHECKOUT-1 --criterion CHECKOUT-2 \
        --awaiting-human "Approve the intent: free shipping from fifty dollars"
    if [ "$fault" != uncommitted ]; then commit "Start the free shipping work"; fi
    reply "Intent captured from hi (CHECKOUT-1, CHECKOUT-2); waiting for its approval." \
        intent "Wait for the human to approve the intent."
}

turn_plan() {
    read_skill
    quiet chough stage advance
    quiet chough stage skip --reason "Fee calculation only; no visuals or interactions change"
    id=$(specsync change new "Free shipping at the threshold" --spec shipping \
        --path src/lib.rs --path tests/threshold.rs --json | jq -r .change.id)
    printf '+ specsync change new -> %s\n' "$id"
    quiet specsync change answer "$id" acceptance_criteria \
        '["An order of exactly 5000 cents ships free", "An order under 5000 cents pays the standard fee"]' --json
    quiet specsync change answer "$id" public_contract yes --json
    remaining=$(specsync change answer "$id" architecture_risk no --json)
    if printf '%s' "$remaining" | jq -e '.questions | map(.id) | index("constraints")' >/dev/null; then
        quiet specsync change answer "$id" constraints none --json
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
        *) artifact "$selected" <<EOF
# ${selected}

Orders of 5000 cents or more ship free (CHECKOUT-1, REQ-shipping-002); others pay the
standard fee (CHECKOUT-2). TC-shipping-0001 to TC-shipping-0003 verify the boundary.
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
    quiet chough checkpoint --native-change "$id"
    case "$fault" in
    skip-gate) ;;
    approve-itself)
        quiet specsync change approve "$id" --actor "0xLeif"
        ;;
    *)
        quiet chough checkpoint --awaiting-human "Scope approval of $id"
        ;;
    esac
    commit "Plan the free shipping change"
    reply "Native change $id is drafted; waiting for the scope approval." \
        plan "Wait for the human scope approval of $id."
}

turn_build() {
    read_skill
    id=$(chough next --environment "$env_name" --json | jq -r .native.change)
    quiet chough stage advance
    cp "$patches/lib.v2.rs" src/lib.rs
    mkdir -p tests
    cp "$patches/threshold.rs" tests/threshold.rs
    sed 's/- \[ \]/- [x]/' ".specsync/changes/$id/tasks.md" >".specsync/changes/$id/tasks.md.tmp"
    mv ".specsync/changes/$id/tasks.md.tmp" ".specsync/changes/$id/tasks.md"
    commit "Ship free from the threshold"
    quiet specsync change check "$id" --commit
    quiet chough stage advance
    for case_id in TC-shipping-0001 TC-shipping-0002 TC-shipping-0003; do
        outcome=$(chough cases run "$case_id" --environment "$env_name" | jq -r .outcome)
        printf '+ chough cases run %s -> %s\n' "$case_id" "$outcome"
        [ "$outcome" = passed ] || { echo "$case_id did not pass" >&2; exit 1; }
    done
    quiet chough stage advance
    quiet chough checkpoint --awaiting-human "Implementation review of $id"
    if [ "$fault" = extra-file ]; then
        printf 'Notes the change did not ask for.\n' >NOTES.md
    fi
    commit "Record the cases and the review gate"
    reply "Built and tested; TC-shipping-0001 to 0003 passed. Waiting for the implementation review." \
        accept "Wait for the human implementation review of $id."
}

turn_done() {
    read_skill
    quiet chough stage advance
    commit "Archive the free shipping session"
    quiet chough doctor --fix
    commit "Remove the archived session folder"
    reply "The change is archived and the session is finished." \
        "done" "Start new work with chough session new <key>."
}

turn_resume() {
    read_skill
    printf '+ chough next --environment %s\n' "$env_name"
    chough next --environment "$env_name" 2>&1 || true
    stage="done"
    if [ "$fault" = wrong-resume ]; then stage=accept; fi
    reply "Nothing is open: the last session is archived." \
        "$stage" "Start new work with chough session new <key>."
}

if [ "$fault" = hang ]; then
    sleep 300
fi
# The sample ships .chough/cases/ only; `chough init` writes .chough/README.md.
if [ ! -f .chough/README.md ]; then
    turn_intent
    exit 0
fi
# With no open session, `chough next` gives a null stage (0.6.1) or fails (earlier).
if ! next=$(chough next --environment "$env_name" --json 2>/dev/null) ||
    printf '%s' "$next" | jq -e '.stage == null' >/dev/null; then
    turn_resume
    exit 0
fi
stage=$(printf '%s' "$next" | jq -r .stage.current)
open_gates=$(printf '%s' "$next" | jq '.gates | length')
if [ "$open_gates" -ne 0 ]; then
    reply "A human gate is still open." "$stage" "Wait for the human decision."
    exit 0
fi
case "$stage" in
intent) turn_plan ;;
plan) turn_build ;;
accept) turn_done ;;
*)
    reply "No scripted step for this stage." "$stage" "Stop."
    exit 1
    ;;
esac
