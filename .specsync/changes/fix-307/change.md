---
id: fix-307
state: approved
type: bug_fix
base_commit: ea159cfef2259a81d1855371d351d543a0d833a4
---

# Fix 307

## Intent

fix 307

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A change with an affected path that is an existing file outside test and fixture folders, listed in a spec's files: while no declared module lists it, makes chough change ready exit 1, naming the module, the paths it owns and the affected_specs answer that declares it; a folder, a test file, a file that does not exist yet and a path no spec lists are not named; with the module declared the report has no finding and exits 0.

## No-spec Rationale

Not applicable
