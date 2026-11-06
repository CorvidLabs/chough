# Lesson bundle: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Native check AC2: only a verifying change with handoff readiness safe shows no note (126, D10)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/native_check.rs, CHANGELOG.md
- **Acceptance**: REQ-cli-039 AC2 says a verifying change shows no note only with handoff readiness safe, and any other or missing readiness keeps a note, matching its body and last criterion; every REQ-cli-039 criterion clause that names the verifying state names its readiness; next text and JSON give approved and implementing changes the same exact not-passed note naming specsync change check <id>; the runtime is unchanged. The new wording test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `1da231f473049cd3780f7673713511b8ac915ee7`
- Base commit: `ec830cfb001cc5f71f4f83809ca3294d685f42a4`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #126, audit finding D10. The audit was of main `700dad96`; REQ-cli-039 is not in the published 0.3.0.

REQ-cli-039's body says a `verifying` change uses the same status read's `handoff.readiness`. Readiness `safe` means the last check matches the tree. Any other readiness is reported as not passed for this tree, and no readiness is reported as unknown, "so a bare `verifying` state never clears the statement". Its last criterion says the same.

AC2 ends "when verifying, it does not". Read alone, that says any verifying change shows no note, which contradicts the body and the last criterion. The runtime (`src/native_check.rs`) and the tests follow the body:
- `a_verifying_change_whose_tree_moved_has_not_passed_for_this_tree` covers readiness `conditional` (a note with SpecSync's reason) and `safe` (no note).
- `a_verifying_change_without_readiness_reports_freshness_unknown` covers no readiness (an unknown note).

No runtime failure is claimed.

## Leif's plan

The 95% plan, item C4 (2026-11-05): a wording repair, recorded as one issue, one change and one PR. It does not authorize a behavior change. Keep the note for unsafe and missing readiness, and do not weaken any gate.

## From the change's testing.md

# Testing

- tests/native_check.rs `req_cli_039_criteria_clear_the_note_only_for_safe_verifying_readiness` reads REQ-cli-039's criteria from specs/cli/requirements.md, as written and as a CRLF copy. Every criterion clause that names the verifying state must name its readiness, and AC2 must say readiness `safe` is what clears the note. It fails first, on AC2's clause "when verifying, it does not."
- `next_says_when_the_native_check_has_not_passed_for_this_tree` checks, for `approved` and for `implementing`, that `next --json` and `next` give the exact not-passed note naming `specsync change check <id>`. A verifying change with readiness `safe` still gets no note.
- Unchanged and still passing: `a_verifying_change_whose_tree_moved_has_not_passed_for_this_tree` (readiness `conditional` gives a note with SpecSync's reason and one native read; `safe` gives none) and `a_verifying_change_without_readiness_reports_freshness_unknown`. The issue also lists two checks of the separate verification-command warning (REQ-cli-038, #92), which this change leaves unchanged: tests/verification.rs `the_warning_changes_no_gate_and_adds_no_native_read` and `the_warning_stays_within_the_next_bound` (a 20,000-character command keeps `next` and `next --json` within 16384 bytes, and 10 commands name 3 with "7 more").

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-039 | tests/native_check.rs `req_cli_039_criteria_clear_the_note_only_for_safe_verifying_readiness`, `next_says_when_the_native_check_has_not_passed_for_this_tree`, `a_verifying_change_whose_tree_moved_has_not_passed_for_this_tree`, `a_verifying_change_without_readiness_reports_freshness_unknown` |

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
