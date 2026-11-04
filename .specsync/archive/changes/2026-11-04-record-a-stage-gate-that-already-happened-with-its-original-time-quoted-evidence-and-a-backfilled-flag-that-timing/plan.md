---
change: record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing
artifact: plan
---

# Plan

## Approach

Add a `backfill` subcommand to `chough stage` that reuses `move_forward` with an
explicit past time and a `backfill` object on the transition, and teach `timing` to
report backfilled moves separately.

## Out of scope

Backfilling human awaiting-human gates (their waiting time and decisions are not
Chough's to assert after the fact), backward or side-stage moves, release, and any
change to existing gates or native reads.

## Steps

1. Write failing tests in tests/stages.rs.
2. Add the optional `backfill` field to `Transition`, the command in src/stage.rs, and
   timing's separate report in src/timing.rs.
3. Update docs/cli.md, the lifecycle skill reference and its installed copy, and
   CHANGELOG.md.
4. Run `specsync change check --commit` and `fledge lanes run verify`.

## Risks

A backfilled time could reorder history. The command refuses a time before the
latest recorded transition or after now, so the log stays in time order. Older Chough
binaries reject the new field (the transition record denies unknown fields), so the
CHANGELOG says to upgrade before reading a backfilled log.

## Constraints consulted

Native SpecSync state decides every gate; Chough records no human approval or review;
no new dependency; records without a backfill keep their exact bytes.
