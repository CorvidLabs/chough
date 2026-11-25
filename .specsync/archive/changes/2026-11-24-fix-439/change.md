---
id: fix-439
state: archived
type: feature
base_commit: bdda89e40b5ff3086e3d571305f82215a120f8a5
---

# fix-439

## Intent

next and status show the refused-check reason recorded in verification-attempts.json instead of a note that keeps no reason

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- next and status surface the refusal recorded in verification-attempts.json instead of the keeps-no-reason note, with the fallback note unchanged when no refusal is recorded, REQ-cli-039 amended by delta to match, docs/cli.md updated, full suite green

## No-spec Rationale

Not applicable
