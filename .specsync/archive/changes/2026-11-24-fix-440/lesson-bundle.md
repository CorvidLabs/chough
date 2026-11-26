# Lesson bundle: fix-440

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 440
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/stage.rs, tests, changelog.d, .gitattributes
- **Acceptance**: When stage advance --to names a stage that is not ahead, the refusal's go-back hint names --reason.

## Evidence

- Verification commit: `d47cb17be46c3e71e992d6dbbce88d790d893f5f`
- Base commit: `0d5988f804155e7b3c285cace6cc0c52f540e035`
- Verified by: `specsync check --spec cli --spec history --spec records --spec reporting`

## From the change's context.md

# Context

Found by the 0.6.2 documentation audit and filed as chough#440. When
`stage advance --to` names a stage that is not ahead, its refusal says
"(use `chough stage back` or `chough stage reset` to go back)"
(src/stage.rs, the `ensure!` in the advance-to handler). Both of those
commands require `--reason`, and every other back or reset hint in Chough
gives `--reason "<why>"` (src/stage.rs around lines 1117, 1120, 1901 and
3013). docs/exercises/cold-start.md says Chough's guidance gives every
reset with `--reason`, so this one hint is off-contract.

Constraints: refusal wording is behavior other tests match on; the fix is
one message string plus its test. No contract text changes (rationale
recorded at change new).

## From the change's testing.md

# Testing

The refusal from `chough stage advance --to <not-ahead>` must contain
`--reason`. The test that asserts on this message is updated to expect the
new wording; the rest of the stage test suite guards the other hints.
Verification commands: the change's own lane (`specsync check --spec cli
--spec history --spec records --spec reporting`) plus the touched cargo
tests.

## Where these lessons go

- `specs/cli/cli.3md`
