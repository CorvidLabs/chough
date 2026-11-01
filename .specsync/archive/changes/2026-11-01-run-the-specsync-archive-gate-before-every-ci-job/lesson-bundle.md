# Lesson bundle — run-the-specsync-archive-gate-before-every-ci-job

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Run the SpecSync archive gate before every CI job
- **Kind**: Operations
- **Paths**: .github/workflows/ci.yml
- **Acceptance**: CI's Archive job runs first via CorvidLabs/spec-sync archive-gate-only at the #832 merge commit; product, native-gates, the existing SpecSync Archive job and ci-fixtures all need it, so a pull request with an open change under .specsync/changes/ runs nothing else.

## Evidence

- Verification commit: `1fd26f3dd7c7c9fc8db35fb13f6eddd3237643b4`
- Base commit: `3ddbcca8ac4fb912c73d22248335d55c4e8aa8c0`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

CorvidLabs requires every SpecSync change to be archived before a pull request merges, and CI
must not run anything until then. SpecSync's GitHub Action gained an archive gate in
CorvidLabs/spec-sync#832 (`archive-gate-only`), merged as `106aaa43d0bcf049f19b173ddb63815a25c3f931`.
This repository's existing `SpecSync Archive` job ran in parallel with the product jobs, so
product CI ran while a change was still open. Approved by 0xLeif as part of the org-wide rollout.

## From the change's testing.md

# Testing

- `ci.yml` parses and every job other than `archive-gate` needs it.
- `python3 .github/scripts/test-specsync-archive.py` passes.
- On the pull request, `Archive` runs first and passes on the archived tip; every other job runs after it.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
