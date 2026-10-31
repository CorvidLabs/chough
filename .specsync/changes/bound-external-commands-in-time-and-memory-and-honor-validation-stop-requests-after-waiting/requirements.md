---
change: bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting
artifact: requirements
---

# Requirements

REQ-cli-011 and REQ-cli-003 are modified (see `deltas/cli.md`) to state the execution
deadline, descendant handling, capture bounds and stop timing explicitly. Invariant 5
already requires captured output to be bounded. Trace: FLOW-7, FLOW-9, LIFE-7. No HI
criterion is added or reworded.
