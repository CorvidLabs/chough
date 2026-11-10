---
id: keep-observations-current-when-the-generated-knowledge-index-is-rebuilt-136
state: archived
type: bug_fix
base_commit: 996dadc2d26b200a7f745215ef9c151304107e05
---

# Keep observations current when the generated knowledge index is rebuilt (#136)

## Intent

Keep observations current when the generated knowledge index is rebuilt (#136)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Rebuilding the knowledge index (chough library index) after recording an observation leaves that observation current: the regression records a passed case observation, confirms current, rebuilds knowledge/INDEX.md (which lists the observation) and confirms it is still current. The configured library-dir name is honored; the legacy fingerprint byte stream is unchanged for older records; .chough/, target/ and SpecSync bookkeeping exclusions behave as before.

## No-spec Rationale

Not applicable
