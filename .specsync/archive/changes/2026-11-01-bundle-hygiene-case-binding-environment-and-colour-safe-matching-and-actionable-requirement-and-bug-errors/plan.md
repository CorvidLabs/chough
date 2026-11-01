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
| Red runs: case definitions and failing tests at the unfixed tip | small | none |
| Bundle dashes, KNOWN_PREVIOUS, package test, host refresh | small | none |
| Requirement refusal messages and hints | small to medium | wording per caller |
| Bug flag hints and help text | small | none |
| Binding `env` | medium | completeness of the refused cargo-test names |
| Escape removal and ESC refusal | medium | malformed sequences at the 1 MiB cut |

## Out of scope

Package 1, the evidence-freshness change (#10, #12, BUG-0014 and the documented
accept order, including when to run cases for new requirement IDs), resolving links against
deltas, external artifact paths, a NO_COLOR default, clearing the inherited
environment, Git LFS guidance, a repository-wide dash rule, SpecSync's own em dashes
(upstream report), and any change to native records.

## Steps

0. Red runs, before any fix: add the regression tests and the case definitions in
   .chough/cases/cli.cases.toml for BUG-0017 (error half), BUG-0018 and BUG-0019. They
   link only existing requirement IDs (this delta modifies, adds none), so they validate
   before `change check`. Run each with `chough cases run` at the unfixed tip, confirm
   it records failed, and link that observation with `chough bug update <BUG>
   --observation <id>` before writing the fix. BUG-0014 (open; fixed by package 1)
   means Chough does not yet check that order itself, so the order is kept by this
   procedure and the later pass must be observed in a later second.
1. Bundle: replace the dashes; add KNOWN_PREVIOUS digests; add the package test.
2. Requirement refusals: typed miss, module derivation, caller wording; tests in
   tests/cases.rs and tests/cli.rs for cross-module, typo and unmaterialized IDs.
3. Bug flags: hints and help text; tests in tests/bugs.rs.
4. Binding env: schema, bounds, runner argument, limitation; tests for bounds,
   variable delivery, unchanged digests of existing cases and changed digest on edit.
5. Colour matching: cleaner with unit tests (CSI, OSC with BEL and ESC backslash,
   two-byte, truncated sequence), ESC refusal, limitation; integration test with
   vitest-style output.
6. references/testing.md, docs/cli.md and docs/architecture.md; `chough skill update`
   for the host copies.
7. After verification, rerun the unchanged cases (passing), link each pass to its
   bug, set BUG-0018 and BUG-0019 verified, and add a note to BUG-0017 that leaves it
   open for package 1.

## Risks

- Existing case files with ESC in a fragment start failing `cases check` (none
  known). There is no change log, so docs/cli.md marks it as a change from earlier
  releases and the pull request description calls it out (criterion 6).
- Users may put secrets in `env`; mitigated by docs and by recording names only.
- Windows treats variable names case-insensitively; this draft refuses two names
  equal ignoring ASCII case on every platform. The approver can ask to drop it.
- A declared env could redirect what `cargo test` runs; the cargo-test refusal list
  covers the known names, and the inherited environment stays a stated limitation.
- Every executed case gains one more limitation string; a script that compares
  limitations exactly would see it (none in this repository; the exercise is checked).
- The two packages share source and docs files (context.md); whichever merges second
  rebases. They edit different bundle files, so their KNOWN_PREVIOUS lines do not
  overlap.

## Constraints consulted

specs/cli/context.md lessons (plain-text gates and post-verification steps;
new requirement IDs validate only after materialization; not-run results; pub(super);
disposable Git identity), Invariants 1, 4, 5, 6 and 7, REQ-cli-006 and REQ-cli-016,
references/bugs.md (a bug without a linked failed observation is unverified), open
BUG-0014 and CorvidLabs/spec-sync#789.
