---
change: know-29-roll-up-keeps-or-flags-tracked-session-records-when-the-librarian-vault-is-untracked-165
artifact: plan
---

# Plan

## Approach

The roll-up hard-links each session beyond the five newest into the vault and leaves the source in `.chough/sessions/archive/` — a copy, never a move, per Leif's decision on #165. The report key `moved` becomes `copied`. REQ-history-011 is amended by the drafted delta; existing roll-up tests are updated to the copy semantics.

## Out of scope

A restore/unroll command, changes to the keep-five count, and any rewording of existing hi criteria.

## Steps

1. Amend REQ-history-011 via the drafted delta (done).
2. Copy semantics in `roll_up` with RED-updated regressions (done).
3. Verify with `change check --commit`, refresh knowledge, open the PR for acceptance.

## Risks

A rolled session stays listed and stays a retention reference source (its observations are no longer pruned through roll-up); flagged in the PR for acceptance. Hard links require one filesystem, as before.

## Constraints consulted

specs/history/context.md lessons; KNOW-29 anchor-holder rule kept unchanged.
