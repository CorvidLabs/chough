---
change: focused-chough-next-a-verification-guard-before-build-and-done-stage-resume
artifact: requirements
---

# Requirements

Two requirements are added; see `deltas/cli.md`.

- REQ-cli-021 (focused next action, including done-stage resume) traces to LIFE-3
  ("resume work with focused context, decisions, open questions, and the next action
  rather than loading the whole history"), FLOW-13, FLOW-2 and LIFE-7.
- REQ-cli-022 (approved-delta guard) traces to FLOW-5 (one approval over a concrete
  definition), LIFE-7 and LIFE-1. It adds to the build, test and accept prerequisites
  of REQ-cli-008 without rewording it, because open PR #22 modifies REQ-cli-008.

REQ-cli-003 (native status is reported, unknown stays unknown) and REQ-cli-016 (host
installation) are unchanged: the next view reports the same native states, and the
host map is not changed, only its wording. No HI criterion is added or reworded.
