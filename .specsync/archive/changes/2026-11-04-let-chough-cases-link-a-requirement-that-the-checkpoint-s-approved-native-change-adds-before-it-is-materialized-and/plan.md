---
change: let-chough-cases-link-a-requirement-that-the-checkpoint-s-approved-native-change-adds-before-it-is-materialized-and
artifact: plan
---

# Plan

## Approach

Add an approved-delta fallback to case requirement validation, reusing the approved-delta guard, and document failing-first evidence in the test stage.

## Out of scope

Folding the test stage into build; a new stage gate; changing `evidence add`; materializing deltas in Chough.

## Steps

1. Write failing tests in tests/cases.rs.
2. Add the resolver in src/cases.rs, with the delta parser beside the approved-delta guard in src/stage.rs, and the hint in src/evidence.rs.
3. Update the skill references, docs/cli.md, docs/lifecycle.md and CHANGELOG.md.
4. Run `specsync change check --commit` and `fledge lanes run verify`.

## Risks

A delta may name a requirement whose final text changes before materialization; the approved-delta digest check refuses an edited delta, so only approved text is linked.

## Constraints consulted

Native SpecSync state decides what is approved; Chough refuses to record human approval or review; requirements.md stays within its bound.
