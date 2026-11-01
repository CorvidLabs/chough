---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: plan
---

# Plan

## Approach

Five small, independent increments in the CLI, each with its regression test, then
skill and docs updates and a bundle refresh. One agent builds; the increments touch
different functions, so they can also be split by file (skill and package test;
evidence and cases errors; bugs; binding env and matching in cases and records).

| Increment | Relative size | Main uncertainty |
|---|---|---|
| Bundle dashes, KNOWN_PREVIOUS, package test, host refresh | small | none |
| Requirement refusal messages and hints | small to medium | wording per caller |
| Bug flag hints and help text | small | none |
| Binding `env` | medium | Windows variable name case-insensitivity |
| Escape removal and ESC refusal | medium | malformed sequences at the 1 MiB cut |

## Out of scope

Package 1 (#10, #12: freshness and SpecSync staleness), resolving links against
deltas, external artifact paths, a NO_COLOR default, clearing the inherited
environment, a repository-wide dash rule, SpecSync's own em dashes (upstream report),
and any change to native records.

## Steps

1. Bundle: replace the dashes; add KNOWN_PREVIOUS digests; add the package test.
2. Requirement refusals: typed miss, module derivation, caller wording; tests in
   tests/cases.rs and tests/cli.rs for cross-module, typo and unmaterialized IDs.
3. Bug flags: hints and help text; tests in tests/bugs.rs.
4. Binding env: schema, bounds, runner argument, limitation; tests for bounds,
   variable delivery, unchanged digests of existing cases and changed digest on edit.
5. Colour matching: cleaner with unit tests (CSI, OSC with BEL and ESC backslash,
   two-byte, truncated sequence), ESC refusal, limitation; integration test with
   vitest-style output.
6. Skill references (build.md, plan.md, testing.md), docs/cli.md and
   docs/architecture.md; `chough skill update` for the host copies.
7. Case definitions for the new behavior and BUG-0017 to BUG-0019 links; run them
   after `change check` materializes the delta.

## Risks

- Existing case files with ESC in a fragment start failing `cases check` (none
  known; called out in the change log).
- Users may put secrets in `env`; mitigated by docs and by recording names only.
- Windows treats variable names case-insensitively, so two names differing only in
  case behave differently across platforms; whether to refuse them is an open
  question for the scope approval.
- The two packages edit the same skill reference and `src/evidence.rs`; whichever
  merges second rebases, and KNOWN_PREVIOUS must list digests from both.

## Constraints consulted

specs/cli/context.md lessons (plain-text gates and post-verification steps;
new requirement IDs validate only after materialization; not-run results; pub(super);
disposable Git identity), Invariants 1, 5, 6 and 7, REQ-cli-006 and REQ-cli-016.
