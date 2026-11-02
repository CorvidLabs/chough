---
change: human-gate-hold-stage-advance-and-skip-refuse-while-the-checkpoint-has-an-open-awaiting-human-gate-issue-38
artifact: requirements
---

# Requirements

One requirement is added; none is modified or removed. See `deltas/cli.md`.

- REQ-cli-030 (open human gates hold forward moves): advance and skip are refused
  with `refused:awaiting-human` while `awaiting_human` is not empty; other moves are
  not; next and status say so. Trace: FLOW-3, FLOW-5, FLOW-13, FLOW-2.

Unchanged and relied on: REQ-cli-007 (the checkpoint and its `awaiting_human` list),
REQ-cli-008 (stage routing and native gates), REQ-cli-020 (gate-opened and
gate-closed records) and REQ-cli-021 (the next view and its bounds). No HI criterion
is added or reworded.
