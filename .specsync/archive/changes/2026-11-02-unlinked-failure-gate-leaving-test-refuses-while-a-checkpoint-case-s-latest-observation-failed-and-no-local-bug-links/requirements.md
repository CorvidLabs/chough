---
change: unlinked-failure-gate-leaving-test-refuses-while-a-checkpoint-case-s-latest-observation-failed-and-no-local-bug-links
artifact: requirements
---

# Requirements

One requirement is modified; none is added or removed. See `deltas/cli.md`.

- REQ-cli-008 (stage routing bounded by native gates) gains the unlinked-failure
  prerequisite on leaving test for accept and one acceptance criterion. Trace adds
  FLOW-7 ("a failing case stays failed even when others pass") and LIFE-5 ("connect
  them to criteria and verification evidence").

Unchanged and relied on: REQ-cli-021 (the next view and its 16384-byte bound and
one native read; the existing next-action rules), REQ-cli-010 and REQ-cli-011
(observations and the bug backlog), and the existing `refused:blocking-bugs` gate.
No HI criterion is added or reworded.
