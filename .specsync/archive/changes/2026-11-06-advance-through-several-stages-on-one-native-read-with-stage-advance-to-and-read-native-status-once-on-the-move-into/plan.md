---
change: advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into
artifact: plan
---

# Plan

## Approach

Port #69's `advance_to` onto today's main, with checks shared with `move_forward`. Make the move into done reuse the gate's read for its own manifest. Write the failing tests first, under per-change sessions.

## Out of scope

- Any cache that outlives one command. #32 owns that, with REQ-cli-053.
- `next`, `status` and `timing` reads.
- `stage skip`, `back`, `reset` and `return` batching.
- A batch into release.
- The skill's transition table. A follow-up can add `--to` there.

## Steps

1. Leif approves the scope, through the orc.
2. After chough#141 merges, merge origin/main.
3. Commit `tests/advance_to.rs`, then run it against main to show it fails first.
4. Implement `--to`, the shared step checks and the reused read into done.
5. Update `docs/cli.md`, `docs/FRD.md` and `CHANGELOG.md`, and add a per-change case file.
6. Run `fledge lanes run verify`, then `specsync change check --commit`.
7. Open the PR, then close #69 once #32's PR is also open, pointing to both.

## Risks

See design.

## Constraints consulted

- DRIVE-9, DRIVE-2.
- Leif's "Split" (21:51 MST) and Q2 "Yes" (21:52 MST).
- REQ-cli-008, REQ-cli-030, REQ-cli-041.
- REQ-history-008.

## Estimate

Small. History is too thin for a range of this size.
- Uncertainty: the merge with #141 and with whatever else lands in `stage.rs`.
