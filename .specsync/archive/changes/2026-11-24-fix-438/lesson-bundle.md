# Lesson bundle: fix-438

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 438
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/gates.rs, src/checkpoint.rs, tests, changelog.d
- **Acceptance**: One chough gate record reply naming several gates closes all of them or none: when any named gate's close would be refused (for example a later save over the session record bound), the whole record is refused with nothing written and no earlier gate left closed.

## Evidence

- Verification commit: `6ffcf5ac00fc43e82385390c8c35c747f950ce56`
- Base commit: `2053788676035ddbe4658b6cc07b18d817c597e6`
- Verified by: `specsync check --spec cli --spec history --spec records --spec reporting`

## From the change's context.md

# Context

Found by the 0.6.2 documentation audit and filed as chough#438.
REQ-cli-061 says `chough gate record` "SHALL close all the named gates or
none". Once its pre-checks pass, it closes them one at a time, each as
its own `chough checkpoint --close-gate` subprocess (src/gates.rs about
lines 1084-1127). If a later close is refused — the reachable case is a
later save hitting the session record or Timeline bound — the earlier
gates stay closed and the error asks to record the remaining gates
again. The pre-checks already cover relay completeness, digest changes,
verifying changes and the native-step count; the bound hit is what
slips through.

## From the change's testing.md

# Testing

The regression: a session record padded so gate 1's close fits but gate
2's cumulative close exceeds SESSION_LIMIT; `gate record --gates 1,2`
refuses with nothing written and both gates still open. Also: the dry
run writes nothing and runs no native step; a normal two-gate record
still closes both. Gate and checkpoint test files, plus the change's
check lane, fmt and lint.

## Where these lessons go

- `specs/cli/cli.3md`
