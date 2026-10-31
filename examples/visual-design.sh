#!/bin/sh
# Visual design exercise in a disposable copy of examples/checkout-ui.
#
# Enters the design stage, records the pending human design approval, and runs the
# deterministic rendered checks through `chough cases run`. The visual judgment
# (TC-checkout-ui-0002) is recorded afterwards by a person or a vision-capable agent
# who actually looked at the screenshots; the human copy review (TC-checkout-ui-0003)
# and the design approval are never scripted.
#
# Usage: PLAYWRIGHT_MODULE=/path/to/playwright/index.mjs [AXE_SCRIPT=/path/to/axe.min.js] \
#        examples/visual-design.sh [empty-workdir]
set -eu

here=$(cd "$(dirname "$0")" && pwd)
work=${1:-$(mktemp -d "${TMPDIR:-/tmp}/chough-visual.XXXXXX")}
chough=${CHOUGH:-chough}
fail() { printf 'EXERCISE FAILED: %s\n' "$*" >&2; exit 1; }

for tool in "${chough%% *}" hi bun git jq; do
    command -v "$tool" >/dev/null 2>&1 || fail "missing required tool: $tool"
done
[ -n "${PLAYWRIGHT_MODULE:-}" ] || fail "set PLAYWRIGHT_MODULE to an installed playwright/index.mjs"
[ -z "$(ls -A "$work" 2>/dev/null)" ] || fail "$work is not empty"
mkdir -p "$work"
cp -R "$here/checkout-ui/." "$work/"
cd "$work"
git -c init.defaultBranch=main init -q

$chough init --objective "Design the checkout summary for free shipping" \
    --intent-scope CHECKOUT --module checkout-ui --host claude >/dev/null
git add -A
git -c user.name="Chough exercise" -c user.email=exercise@example.invalid \
    -c commit.gpgsign=false commit -qm "Seed the checkout-ui sample"
$chough stage advance --note "SIMULATED: the exercise approves the sample intent; no human decided" >/dev/null
$chough checkpoint --design design/prototype/index.html --design design/prototype/COMPANION.md \
    --awaiting-human "Design approval of design/prototype (rendered prototype and companion together)" \
    --next-action "Record the visual verdicts for TC-checkout-ui-0002, then wait for the human design approval." >/dev/null

printf '== Selection for a visual change\n'
$chough cases select --requirement REQ-checkout-ui-001 --track design-qa |
    jq -r '(.selected[] | "selected: \(.id) (\(.method))"), (.omitted[] | "omitted: \(.id) (\(.reason))")'

printf '\n== Deterministic rendered checks\n'
set +e
result=$($chough cases run TC-checkout-ui-0001 --environment chromium-headless)
set -e
outcome=$(printf '%s' "$result" | jq -r .outcome)
printf 'TC-checkout-ui-0001: %s, observation %s\n' "$outcome" "$(printf '%s' "$result" | jq -r .observation)"
[ "$outcome" = passed ] || fail "rendered checks did not pass: $result"

printf '\n== Design stage stays open until a human approves\n'
$chough next --environment chromium-headless | jq -r .next_valid_action
printf '\nScreenshots: %s/.chough/artifacts/visual-review/\n' "$work"
printf 'Next: inspect them, then record TC-checkout-ui-0002 with `chough cases record`.\n'
printf 'VISUAL SETUP PASSED in %s\n' "$work"
