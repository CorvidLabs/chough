---
id: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
state: archived
type: bug_fix
base_commit: 97df042ea89d865cc08b36d1f96c3901103e366d
---

# A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.

## Intent

A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A fork pull request skips the label job. A symlinked session.md is not read. An existing label is created only when it is missing, and --force is used only when its color or description differs. implement and verify are deleted only when they exist.

## No-spec Rationale

Not applicable
