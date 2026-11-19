# Lesson bundle: pin-the-edited-workflow-fast-path-in-the-skill-release-corpus

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Pin the edited workflow fast path in the skill release corpus
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/skill_releases.rs, tests/fixtures/skill-releases/manifest.json
- **Acceptance**: tests/skill_releases.rs passes, and the current bundle pin for references/stages.md is the hash of skills/chough-workflow/references/stages.md.

## Evidence

- Verification commit: `6f89faa4e7761066eba42410d54f2cafe58db6c4`
- Base commit: `613f7bdb02409e6a296ad597b0c8df405de25e58`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Editing the workflow fast path in `skills/chough-workflow/references/stages.md` made `tests/skill_releases.rs` fail in CI. The current bundle pin still named the previous bytes of that file. Product on Ubuntu, MSRV, and the Fledge verify lane all failed on `613f7bdb` with `current bundle changed: references/stages.md`.

The historical release blobs stay as they are. Only the current pin and the manifest digest constant move to the edited file.

## From the change's testing.md

# Testing

`cargo test --locked --test skill_releases` passed locally: 7 passed. That is the check that hashes `skills/chough-workflow/references/stages.md` against `current_bundle_sha256` and hashes the manifest against `MANIFEST_SHA256`.

## Where these lessons go

- `specs/cli/cli.3md`
