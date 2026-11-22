# Lesson bundle: release-0-6-3

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Release 0.6.3
- **Kind**: Operations
- **Paths**: Cargo.toml, Cargo.lock, CHANGELOG.md, changelog.d, knowledge/index.md, knowledge/journal/feed.xml
- **Acceptance**: Cargo.toml and Cargo.lock say 0.6.3; CHANGELOG.md has a 0.6.3 section dated 2026-11-22 with every merged fragment from changelog.d, which is then empty, and its release link; fledge run release-check passes; cargo package --list --locked lists only the package files; cargo publish --dry-run --locked passes. Nothing is tagged or published.

## Evidence

- Verification commit: `f474ed76a1c6283c91ae16b4c906061b1a5be777`
- Base commit: `61479feb1e88a57bc45716ff4e42ce84fb2b463b`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

0.6.3 is a patch release. Leif said on 2026-11-22, after merging the fork pull
request, that the release which introduces it comes next. This change is the
preparation in `docs/releasing.md`: the version, the change log assembled from the
six merged fragments in `changelog.d/`, and the rebuilt knowledge index.

The tag, `cargo publish`, the GitHub release, the clean-install observation and the
roadmap row are separate steps and are not part of this change.

## From the change's design.md

# Design

Nothing visual changes. `chough --version` prints 0.6.3 after the version bump.

## From the change's testing.md

# Testing

No behavior changes, so no new test. `cargo package --list --locked` and
`cargo publish --dry-run --locked` are recorded as dry runs, and
`fledge run release-check` as the index check.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| none changed | the recorded dry runs and the release check |

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
