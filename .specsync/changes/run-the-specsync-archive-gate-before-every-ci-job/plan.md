---
change: run-the-specsync-archive-gate-before-every-ci-job
artifact: plan
---

# Plan

## Approach

Add an `archive-gate` job named `Archive` at the top of `ci.yml`, using
`CorvidLabs/spec-sync@106aaa43d0bcf049f19b173ddb63815a25c3f931` with `archive-gate-only: 'true'`,
and make every other job need it.

## Out of scope

Product, Fledge, HI and native lifecycle checks themselves; the existing archive closure script.
Branch protection: the org ruleset requiring a check named `Archive` is configured separately.

## Steps

1. Add the `archive-gate` job.
2. Add `needs: archive-gate` to `product`, `native-gates`, `archive` and `ci-fixtures`.

## Risks

`@v6.1.1` ignores the gate inputs, so the step pins the #832 merge commit until a release ships.

## Constraints consulted

AGENTS.md: meaningful files need an active native change, and approval comes from the human (0xLeif), not an agent label.
