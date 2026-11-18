# Lesson bundle: docs-0-6-1-release

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Docs 0 6 1 release
- **Kind**: Documentation
- **Paths**: docs/roadmap-1.0.md, README.md, changelog.d/2026-11-18-docs-0-6-1-release.md
- **Acceptance**: docs/roadmap-1.0.md's delivered table has a 0.6.1 release row that states only observed facts (tag, commit, release preparation PR, GitHub release, and the crates.io version and clean install once they are observed); README.md no longer says 0.6.0 is not released yet; the docs tests pass.

## Evidence

- Verification commit: `8d58a297bf8631450557f3bb6cd6792913639ffc`
- Base commit: `cef86e2622a2a908856b0c2cf11b29d3c1d3e472`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

Chough 0.6.1 was tagged on 2026-11-18: tag `v0.6.1` at cef86e26 (release
preparation PR #326), with its GitHub release. `docs/releasing.md` step 9 asks
for the release's row in the roadmap's delivered table, marked published with
the date.

`README.md` still says "This table documents 0.6.0, which is not released yet."
0.6.0 was published on 2026-11-12. Leif chose to leave the 0.6.1 tag where it is
and fix the sentence afterwards, relayed by his planning session (hc t4c).

0.6.1 was published to crates.io on 2026-11-18 from the tagged commit, and a
clean install into an empty root was observed on macOS arm64. The row states
those observations.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
