---
change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
artifact: context
---

# Context

Issue #126, audit finding D10. The audit was of main `700dad96`; REQ-cli-039 is not in the published 0.3.0.

REQ-cli-039's body says a `verifying` change uses the same status read's `handoff.readiness`. Readiness `safe` means the last check matches the tree. Any other readiness is reported as not passed for this tree, and no readiness is reported as unknown, "so a bare `verifying` state never clears the statement". Its last criterion says the same.

AC2 ends "when verifying, it does not". Read alone, that says any verifying change shows no note, which contradicts the body and the last criterion. The runtime (`src/native_check.rs`) and the tests follow the body:
- `a_verifying_change_whose_tree_moved_has_not_passed_for_this_tree` covers readiness `conditional` (a note with SpecSync's reason) and `safe` (no note).
- `a_verifying_change_without_readiness_reports_freshness_unknown` covers no readiness (an unknown note).

No runtime failure is claimed.

## Leif's plan

The 95% plan, item C4 (2026-11-05): a wording repair, recorded as one issue, one change and one PR. It does not authorize a behavior change. Keep the note for unsafe and missing readiness, and do not weaken any gate.
