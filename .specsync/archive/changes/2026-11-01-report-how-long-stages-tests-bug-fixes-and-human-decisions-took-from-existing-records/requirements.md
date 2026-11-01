---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: requirements
---

# Requirements

REQ-cli-020 (timing report) is added; see `deltas/cli.md`. It traces to LIFE-10,
the criterion the human agreed and that this change captured in `hi/life.md`, and
to the existing LIFE-3, LIFE-6 and LIFE-7. REQ-cli-007 and REQ-cli-008 are not
reworded: the checkpoint still holds the awaiting-human list, and the gate records
are new transition kinds in the existing append-only history. Invariant 8 (append-
only transitions) and Invariants 2, 3, 5 and 6 apply unchanged. No other HI criterion
is added or reworded.
