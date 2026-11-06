---
change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
artifact: testing
---

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
