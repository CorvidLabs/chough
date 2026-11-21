# Lesson bundle: release-0-6-2

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Release 0.6.2
- **Kind**: Operations
- **Paths**: Cargo.toml, Cargo.lock, CHANGELOG.md, changelog.d/, knowledge/, tests/changelog.rs, tests/fixtures/changelog/
- **Acceptance**: Cargo.toml and Cargo.lock say 0.6.2; CHANGELOG.md has a 0.6.2 section with every merged fragment from changelog.d, which is then empty, and its release link; fledge run release-check passes; cargo package --list and cargo publish --dry-run pass; the verify lane is green.

## Evidence

- Verification commit: `6e1fd6823933738f230927b55fca1dff2031d524`
- Base commit: `e272a6f3a77338404fe2448dcfeb9d68f84998dc`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

0.6.2 is a patch release, decided by Leif on 2026-11-21 (about 23:59Z, relayed by his
planning session, hc t4c): "Yes, release 0.6.2 (Recommended)", to releasing Chough 0.6.2
once the last regression check passes, with the remainder moved to 0.6.3. #404 stays open
for its second item (accepting a delta SpecSync rebased, spec-sync #1089) and moves to
0.6.3.

This change is the release preparation of `docs/releasing.md`: the version, the change
log assembled from the 43 merged fragments in `changelog.d/`, and the rebuilt knowledge
index, on a branch, merged before the tag. The tag waits for the tester's regression
line on main e272a6f3. Tagging, publishing, the GitHub release and the installed
binaries on this Mac are Leif's separate steps and are not part of this change.

0.6.2 needs SpecSync 6.4.0 (#404, #429); its changelog says so.

## From the change's design.md

# Design

Nothing visual changes. `chough --version` prints 0.6.2.

## From the change's testing.md

# Testing

No behavior changes, so no new test. The verify lane runs on the release branch.
`cargo package --list --locked` and `cargo publish --dry-run --locked` are recorded as
dry runs, and `fledge run release-check` as the index check.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| none changed | the verify lane and the recorded dry runs |

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
