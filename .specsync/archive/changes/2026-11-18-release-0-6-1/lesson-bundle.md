# Lesson bundle: release-0-6-1

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Release 0 6 1
- **Kind**: Operations
- **Paths**: Cargo.toml, Cargo.lock, CHANGELOG.md, changelog.d/, knowledge/, tests/changelog.rs, tests/fixtures/changelog/
- **Acceptance**: Cargo.toml and Cargo.lock say 0.6.1; CHANGELOG.md has a 0.6.1 section holding every fragment of a change merged to main and changelog.d holds none of them; fledge run release-check passes; cargo publish --dry-run --locked succeeds; the two changelog tests that read live fragments pass with changelog.d empty, from fixture copies and the released section. Nothing is tagged or published.

## Evidence

- Verification commit: `81f03326619f3859f1eefd79416cbd5505c15f33`
- Base commit: `975af07cc89912f9ea02d27c81fe87f046685eaa`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

0.6.1 is a fixes-only patch release, decided by Leif on 2026-11-18 and relayed
by his planning session (hc t4c): "focus on getting 0.6.1 ready and up and
released and all so we can unblock Kyn". Jackdaw PR 184 needs the blocking-bug
fix (#304) in a released Chough.

`chough change ready` (#311) and its fix (#310) are not in it; they go to 0.7.0.
#306 was reverted (#323), so #260 is not fixed in 0.6.1.

This change is the release preparation of `docs/releasing.md`: the version, the
change log and the rebuilt knowledge index, on a branch, merged before the tag.
Tagging, publishing and the GitHub release are Leif's separate decisions and are
not part of this change.

## From the change's design.md

# Design

Nothing visual changes. `chough --version` prints 0.6.1.

## From the change's testing.md

# Testing

No behavior changes, so no new test. Two existing tests in `tests/changelog.rs`
are changed so they hold after a release removes the fragments. The verify lane runs on the release
branch. `cargo package --list --locked` and `cargo publish --dry-run --locked`
are recorded as dry runs, and `fledge run release-check` as the index check.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| none changed | the verify lane and the recorded dry runs |

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
