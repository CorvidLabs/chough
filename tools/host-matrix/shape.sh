#!/bin/sh
# The host-independent shape of a finished host-matrix sample (REQ-cli-066).
#
# Reads only the sample's committed files: the archived session story, SpecSync's
# archived change and Git. Leaves out everything that differs between hosts by
# design (IDs, times, digests, agent names, wording), so the same scenario gives the
# same shape on every host. run.sh compares it with expected-shape.json.
#
# Usage: tools/host-matrix/shape.sh <sample-dir> <native-change-id> <seed-commit>
set -eu

[ $# -eq 3 ] || { echo "usage: shape.sh <sample-dir> <native-change-id> <seed-commit>" >&2; exit 64; }
cd "$1"
id=$2
seed=$3

story_count=$(find .chough/archive/sessions -maxdepth 1 -name '*.md' 2>/dev/null | wc -l | tr -d ' ')
open_count=$(find .chough/sessions -mindepth 1 -maxdepth 1 -type d 2>/dev/null | wc -l | tr -d ' ')
story=$(find .chough/archive/sessions -maxdepth 1 -name '*.md' 2>/dev/null | sort | tail -n 1)
[ -n "$story" ] || { echo "no archived session story" >&2; exit 1; }
archive=$(find .specsync/archive/changes -mindepth 1 -maxdepth 1 -type d -name "*$id" | sort | tail -n 1)
[ -n "$archive" ] || { echo "no archived native change for $id" >&2; exit 1; }

# Timeline entries: "- <time> · <kind> · <from> → <to> · <text> ^t-…", with detail
# lines such as "  - holds: plan" below each.
timeline=$(awk '/^## Timeline/{p=1; next} /^## /{p=0} p' "$story")

stage_path=$(printf '%s\n' "$timeline" | awk -F ' · ' '
    /^- / {
        split($3, move, " → ")
        if (!seen[move[1]]++) print move[1]
        if (!seen[move[2]]++) print move[2]
    }' | jq -R . | jq -s .)

gates=$(printf '%s\n' "$timeline" | awk -F ' · ' '
    /^- / { kind = $2; title = $4; sub(/ \^t-[0-9a-f]+$/, "", title); holds = ""; drives = "" }
    /^- / && kind == "gate-closed" { closed[title] = 1 }
    /^  - holds: / && kind == "gate-opened" { holds = substr($0, 12) }
    /^  - drives: / && kind == "gate-opened" { opened[title] = holds ":" substr($0, 13) }
    END { for (t in opened) print opened[t] ":" (closed[t] ? "closed" : "open") }' |
    sort -u | jq -R 'split(":") | {holds: .[0], drives: .[1], state: .[2]}' | jq -s .)

in_a_persons_name=$(grep -c 'decided_by:' "$story" || true)

cases_passed=$(awk '/^## Receipts/{p=1; next} /^## /{p=0} p' "$story" | awk '
    /^  - id: / { result = ""; case_id = "" }
    /^    result: / { result = $2 }
    /^    case: / { case_id = $2; if (result == "passed") print case_id }' |
    sort -u | jq -R . | jq -s .)

approved_by=$(jq '[.approvals[]? | select(.gate == "definition") | .actor] | unique' "$archive/approvals.json")
reviewed_by=$(jq '[.reviewer] | unique' "$archive/review.json")
state=$(jq -r .state "$archive/state.json")

product_files=$(git diff --name-only "$seed" HEAD -- . ':(exclude).chough' ':(exclude).specsync' \
    ':(exclude)specs' ':(exclude)hi' ':(exclude)knowledge' ':(exclude).agents' ':(exclude).gitattributes' |
    sort | jq -R . | jq -s .)

jq -n \
    --argjson open "$open_count" --argjson archived "$story_count" \
    --argjson stage_path "$stage_path" --argjson gates "$gates" \
    --argjson in_a_persons_name "$in_a_persons_name" \
    --arg state "$state" --argjson approved_by "$approved_by" --argjson reviewed_by "$reviewed_by" \
    --argjson cases_passed "$cases_passed" --argjson product_files "$product_files" \
    '{sessions: {open: $open, archived: $archived},
      stage_path: $stage_path,
      gates: $gates,
      decisions_in_a_persons_name: $in_a_persons_name,
      native: {state: $state, approved_by: $approved_by, reviewed_by: $reviewed_by},
      cases_passed: $cases_passed,
      product_files: $product_files}'
