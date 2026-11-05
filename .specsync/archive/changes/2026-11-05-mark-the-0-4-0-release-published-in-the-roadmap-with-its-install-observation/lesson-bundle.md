# Lesson bundle: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Mark the 0.4.0 release published in the roadmap with its install observation
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: docs/ROADMAP.md
- **Acceptance**: docs/ROADMAP.md has a 0.4.0 release row naming the v0.4.0 tag at 69950c7, release prep PR #116, crates.io chough 0.4.0, the GitHub release and the clean install result, and the clean install is recorded as an installation observation; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `844f026681dfa9349d2115882763ab04c2111456`
- Base commit: `69950c7a9253230aeeee3bac0d0d6b580e291aad`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif approved all Chough 0.4.0 release steps (2026-11-05 11:26 MST, and "Yes, run them now", through the orc). The release prep merged in #116 as 69950c7a, with main CI green there, MSRV included. The orc tagged `v0.4.0` at 69950c7a, published `chough` 0.4.0 to crates.io from a fresh clone of the tag, and created the GitHub release. A clean `cargo install chough --version 0.4.0 --locked` into an empty root on this Mac (cargo 1.95.0, macOS 26.5.2 arm64) exited 0, and `chough --version` printed `chough 0.4.0`. Step 6 of docs/releasing.md marks the release published in the roadmap.

## Where these lessons go

- `specs/cli/context.md`
