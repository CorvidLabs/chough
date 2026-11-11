---
change: capture-the-2026-11-10-mvp-interview-as-hi-criteria-and-add-the-1-0-roadmap-test-plan-and-draft-issue-mapping
artifact: tasks
---

# Tasks

Done before this PR opened:

- Capture the 13 new hi criteria and regenerate the INTENT.md index.
- Write docs/roadmap-1.0.md and docs/test-plan-1.0.md; link both from the
  README and docs/ROADMAP.md.
- Verify: `hi check` clean; `fledge lanes run verify` green locally.

Human gates and post-approval steps, as plain text (not checkboxes, per the
tasks lesson of 2026-10-31):

- Leif approves the intent as a whole (the 13 criteria) and this change's
  scope, recorded natively.
- Leif rules on the flagged conflicts: FLOW-17 vs FLOW-21 (Windows via WSL
  versus best-effort) and KNOW-29 vs KNOW-30 (roll-up move versus copy).
- After approval: `specsync change check`, Leif's implementation review,
  native archive, merge.

Done since this PR opened (per Leif's direction relayed by the coordinator):

- The issues are created, not drafted: milestone "Chough 1.0", epic #232, new
  issues #213 to #231, and the 14 existing open issues mapped onto the
  milestone. #187 and #196 have since closed on main.
