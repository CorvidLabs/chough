---
change: integrate-the-approved-open-human-gate-follow-up-and-examples
artifact: context
---

# Context

The user finished the remaining Grok PRs while PR44 integration was underway.
PR45 is now merged as 9d02fbf173f8a7471661a556676603123f70482f, the final pinned
main input for this task. The previous 3d00e44 merge is preserved as an
intermediate merge commit, including both source behaviors and a reproduced/fixed
current-main predecessor upgrade defect. No history is rewritten.

This disjoint scope owns only examples/feature-bug-loop.sh and docs/walkthrough.md.
The other integration scopes already own src/stage.rs, tests/stages.rs, CLI docs,
case/spec metadata and bundle/test support. Preserve PR45's approved guard refusing
advance/skip while awaiting_human is open, its tests and executable sample ordering.
Preserve PR44's acting-agent account and separate actual human authorization in the
walkthrough. No new product or requirement meaning is introduced.

Actual authority is 0xLeif's 2026-11-02 standing repository-changes-through-PRs
instruction, relayed at
https://github.com/CorvidLabs/chough/issues/11#issuecomment-6581702299 , and his
later "Can you finish up all these?" request after the PR status/closeout gates.
The acting project account is corvid-agent. Covered scope is this bounded final
main integration, qualification and normal branch push. Root alone reviews/ships.
PR merge, release, deployment, cleanup and fabricated new as-is acceptance remain
excluded. All existing archives, observations, artifacts and approvals are frozen.
