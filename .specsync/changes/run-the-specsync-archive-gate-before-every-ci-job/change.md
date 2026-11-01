---
id: run-the-specsync-archive-gate-before-every-ci-job
state: draft
type: operations
base_commit: 3ddbcca8ac4fb912c73d22248335d55c4e8aa8c0
---

# Run the SpecSync archive gate before every CI job

## Intent

Run the SpecSync archive gate before every CI job

## Affected Canonical Specs

- None

## Acceptance Criteria

- CI's Archive job runs first via CorvidLabs/spec-sync archive-gate-only at the #832 merge commit; product, native-gates, the existing SpecSync Archive job and ci-fixtures all need it, so a pull request with an open change under .specsync/changes/ runs nothing else.

## No-spec Rationale

CI workflow ordering only; no canonical spec owns .github/workflows
