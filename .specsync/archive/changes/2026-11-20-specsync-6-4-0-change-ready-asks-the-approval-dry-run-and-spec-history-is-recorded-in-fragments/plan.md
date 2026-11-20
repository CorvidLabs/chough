---
change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
artifact: plan
---

# Plan

## Approach

Draft on SpecSync v6.4.0-rc.1 with failing tests first; build when approved; swap the
Action commit for 6.4.0's when it publishes; land with no other pull request open.

## Out of scope

- #404's item 2, accepting a delta SpecSync rebased (spec-sync #1089, not in 6.4.0).
  #404 stays open for it.
- Freezing the existing Change Log tables: 6.4.0 keeps them as they are by itself.
- Any SpecSync version floor in Chough's own code.

## Steps

1. Red tests in `tests/ready.rs` and `tests/package.rs`, committed failing.
2. `src/ready.rs`: the dry run, the paths exceptions, the delta-set rule and the
   hash-file code removed; `src/change.rs`'s install hint.
3. The pin in CI, the archive scripts, the config header, `AGENTS.md`, the README and
   docs; `.gitattributes` and `.specsync/sdd.json`.
4. Cases, a changelog fragment, the full lane with the candidate, then the tester's
   real-repository run of ready (66 changes, as for #406) before review.

## Risks

- CI cannot install 6.4.0 until it publishes, so the pull request waits for that.
- A machine still on 6.3.0 that runs `change check` here after this lands adds a Change
  Log row 6.4.0 refuses; every machine, and the SpecSync merge driver each clone
  registers, moves to 6.4.0 when this merges.
- A pull request open when this lands, whose change 6.3.0 recorded, keeps its row; 6.4.0
  still reads it. Landing with none open avoids the mix.

## Constraints consulted

Leif's rule 5 (prefer fixes that remove rules); his "Ask SpecSync itself" (#406); rule 3,
the tester's real-repository run for a changed refusal; every new free-text value
through the leak guard (none here: SpecSync's messages are shown, not written).
