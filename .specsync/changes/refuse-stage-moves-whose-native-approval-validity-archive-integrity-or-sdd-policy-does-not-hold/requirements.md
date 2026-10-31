---
change: refuse-stage-moves-whose-native-approval-validity-archive-integrity-or-sdd-policy-does-not-hold
artifact: requirements
---

# Requirements

REQ-cli-008 is modified (see `deltas/cli.md`): stage prerequisites apply to the
destination of every move, approval validity and archive integrity are part of the
native gate, and an invalid SDD policy refuses gated moves. REQ-cli-003 already
requires unknown native state to stay unknown; this change applies it to the policy.
Trace: FLOW-2, FLOW-5, FLOW-10, LIFE-7. No HI criterion is added or reworded.
