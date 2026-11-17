---
id: pin-the-edited-workflow-fast-path-in-the-skill-release-corpus
state: archived
type: bug_fix
base_commit: 613f7bdb02409e6a296ad597b0c8df405de25e58
---

# Pin the edited workflow fast path in the skill release corpus

## Intent

Pin the edited workflow fast path in the skill release corpus

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- tests/skill_releases.rs passes, and the current bundle pin for references/stages.md is the hash of skills/chough-workflow/references/stages.md.

## No-spec Rationale

FLOW-18 already requires the skill to say what current main does and a check to fail when it drifts. Updating the current bundle pin records the edited stages.md. No requirement text changes.
