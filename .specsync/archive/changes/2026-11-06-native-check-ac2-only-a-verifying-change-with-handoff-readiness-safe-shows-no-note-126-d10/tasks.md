---
change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
artifact: tasks
---

# Tasks

- [x] Add the failing test `req_cli_039_criteria_clear_the_note_only_for_safe_verifying_readiness` in tests/native_check.rs. Make `next_says_when_the_native_check_has_not_passed_for_this_tree` check the exact note for both `approved` and `implementing`.
- [x] Modify REQ-cli-039 AC2: a verifying change shows no note only with handoff readiness `safe`, and any other or missing readiness keeps a note. The body, the other criteria and the runtime are unchanged.
- [x] Add a CHANGELOG line.

After `specsync change check` materializes the delta, not a task box: record the case and run `fledge lanes run verify` on Linux.
