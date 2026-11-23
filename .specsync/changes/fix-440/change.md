---
id: fix-440
state: approved
type: feature
base_commit: 0d5988f804155e7b3c285cace6cc0c52f540e035
---

# Fix 440

## Intent

fix 440

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- When stage advance --to names a stage that is not ahead, the refusal's go-back hint names --reason.

## No-spec Rationale

A refusal hint string only; no contract text changes (docs fix named by the 0.6.2 audit, docs/exercises/cold-start.md already requires --reason in hints)
