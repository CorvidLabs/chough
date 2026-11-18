---
id: fix-307
state: draft
type: bug_fix
base_commit: ea159cfef2259a81d1855371d351d543a0d833a4
---

# Fix 307

## Intent

fix 307

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A change whose affected path is listed in a spec's files: while that module is not in affected_specs makes chough change ready exit 1, naming the module, the paths it owns and the affected_specs answer that declares it; a path no spec lists is not named; with the module declared the report has no finding and exits 0.

## No-spec Rationale

Not applicable
