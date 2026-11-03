# Lesson bundle: identity-acceptance-evidence-for-issue-30-a-pinned-clock-same-second-run-in-two-git-roots-and-a-real-git-integration

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Identity acceptance evidence for issue 30: a pinned-clock same-second run in two Git roots and a real Git integration fixture for generated records
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/records.rs, src/main.rs, tests/cases.rs, specs/cli/, CHANGELOG.md, .chough/cases/
- **Acceptance**: A deterministic test pins the clock (a cfg(test)-only hook, no runtime switch) and runs the same bound case in two independent Git roots on separate branches at the same second; after merging, the generated observation IDs and artifact paths are distinct, both records keep the pinned time, both artifacts keep their recorded bytes and fingerprints, and both observations read as current
- **Acceptance**: A real Git fixture adds generated records independently on two branches in two roots (cases run observations and artifacts, evidence add with an artifact, bug add); the merge has no conflicts, every record from both sides is byte-identical to its branch version under a distinct ID, and cases check and status validate the merged tree with every observation current
- **Acceptance**: No product behavior changes; fledge lanes run verify passes; hosted CI is reported only as observed

## Evidence

- Verification commit: `600851caae7cdc191ddd3cebd0bdd90240679d3e`
- Base commit: `a788869b32b8442cbc8d7be2bf19c19a922954a8`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #30 still has unchecked acceptance evidence after PR #33 (slug case IDs and random OBS/VAL/SNAP IDs) merged. The two items picked up here are:

- the deterministic same-second two-root test;
- the real Git fixture.

`records::now()` cannot be injected (`tests/common` says so), and the Product CI jobs do not install `hi`. So the same-second test runs inside the crate, with a test-only clock pin and a test-only `hi` export stand-in. Neither exists outside `cfg(test)`, so no runtime switch can forge observation times.

Plan comment: https://github.com/CorvidLabs/chough/issues/30#issuecomment-6593220605. Criteria 2, 4, 5 and 6 of the issue are out of scope here.

## From the change's design.md

# Design

- `src/records.rs`: `#[cfg(test)]` thread-local hooks, built like `identity::with_random_results`:
  - `with_pinned_clock(seconds, run)`, which `now()` honors;
  - `with_native_stdout(program, stdout, run)`, which `native()` honors, so case-link validation can read a canned `hi export`.
  Release builds are unchanged.
- `src/main.rs` tests: a same-second test. It initializes one root, clones it into a second root, runs the same `command`-bound case (`git --version`) on branch `a` and branch `b` under one pinned second, merges `b` into `a`, and checks IDs, paths, times, bytes and freshness.
- `tests/cases.rs`: a real Git fixture test using the built binary.

## From the change's testing.md

# Testing

- `same_second_runs_in_two_git_roots_keep_distinct_identities_after_merge` (unit, bin target) runs on every CI host and needs only `git`.
- `independently_added_records_merge_without_conflict_or_replacement` (integration, `tests/cases.rs`) uses the built binary, a `hi` stand-in and real `git`.
- Cases `TC-cli-same-second-runs-in-two-git-roots-stay-distinct` and `TC-cli-independently-added-records-merge-cleanly` link REQ-cli-023.
- These are acceptance tests for behavior already on main. Each assertion is checked against a deliberately broken variant: the test must fail if the IDs or paths collide, or if a record is replaced.

## Where these lessons go

- `specs/cli/context.md`
