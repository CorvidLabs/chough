# Lesson bundle: docs-0-6-3-release

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Docs 0.6.3 release
- **Kind**: Documentation
- **Paths**: docs/roadmap-1.0.md, changelog.d
- **Acceptance**: The roadmap delivered table has a 0.6.3 release row marked published on 2026-11-22, naming tag v0.6.3 at dadea1c4, pull request 446, crates.io chough 0.6.3, and the clean install result. A changelog fragment names that row.

## Evidence

- Verification commit: `4e37e0c5164f0e2d41b834844ce6af33642f71d1`
- Base commit: `0267cfebfdc1d0df638d0fa7a12c771354144238`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

Chough 0.6.3 is published. This change records that on the roadmap, with the
clean install that was observed on macOS arm64: cargo 1.100.0, an empty root,
a fresh target directory, `chough --version` printing `chough 0.6.3`, and
`chough init`, `chough next`, and `chough status` each exiting 0 in a new Git
repository. A changelog fragment names the row.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
