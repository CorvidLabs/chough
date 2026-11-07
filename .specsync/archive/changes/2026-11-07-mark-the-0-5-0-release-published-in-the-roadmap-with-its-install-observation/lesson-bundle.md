# Lesson bundle: mark-the-0-5-0-release-published-in-the-roadmap-with-its-install-observation

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Mark the 0.5.0 release published in the roadmap with its install observation
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: docs/ROADMAP.md
- **Acceptance**: docs/ROADMAP.md has a 0.5.0 release row naming the v0.5.0 tag at 455cb643, release prep PR #185, crates.io chough 0.5.0, the GitHub release and the clean install result, and the clean install is recorded as an installation observation; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `b6d9ba0c14d99ccb7de74fd27fd418d3d946b172`
- Base commit: `86ae8dda94aba981094e92d5395dc085ebe65dbd`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif approved all Chough 0.5.0 release steps through the orc and explicitly assigned release step 9 to this worker. The release prep merged in #185 as 455cb643, and the orc tagged `v0.5.0` there, published `chough` 0.5.0 to crates.io, created the GitHub release and checked a clean install. This change independently captures a clean install on this Mac as an installation observation, with its command log, and marks the release published in the roadmap.

The scope is `docs/ROADMAP.md`, the observation and native lifecycle records only. No HI criterion, requirement, contract, tag, publication, GitHub release or deployment is part of this change.

## Where these lessons go

- `specs/cli/context.md`
