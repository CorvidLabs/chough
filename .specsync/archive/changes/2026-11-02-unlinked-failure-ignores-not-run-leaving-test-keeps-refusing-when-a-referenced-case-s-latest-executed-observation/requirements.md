---
change: unlinked-failure-ignores-not-run-leaving-test-keeps-refusing-when-a-referenced-case-s-latest-executed-observation
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-008: the unlinked-failure rule uses the latest executed (passed or failed) observation of each referenced case, by recorded time and then ID; a later `not-run` observation does not replace it. A not-run-only case does not refuse, and a later pass still lifts the refusal. Everything else in REQ-cli-008 is unchanged. Trace unchanged (FLOW-2, FLOW-5, FLOW-10, LIFE-1, FLOW-7, LIFE-5).
