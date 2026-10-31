---
change: refuse-stage-moves-whose-native-approval-validity-archive-integrity-or-sdd-policy-does-not-hold
artifact: tasks
---

# Tasks

- [ ] Replace the build and accept gates with one destination prerequisite check in src/stage.rs.
- [ ] Read approval_valid, correction_valid and terminal_evidence.validity from native status.
- [ ] Parse .specsync/sdd.json with SpecSync's defaults and types; refuse an invalid policy.
- [ ] Apply the check to advance, skip, back, reset and release -> done.
- [ ] Add regression tests in tests/stages.rs with real SpecSync and synthetic approvals.
- [ ] Add case definitions for BUG-0006 to BUG-0008 and record passing observations.
- [ ] Update docs/cli.md and the skill's stages and lifecycle references in all three bundle copies.

Human gates and post-verification steps (not tasks): scope approval of this change;
push and CI; the human implementation review; native review and ship.
