---
id: fix-269
state: archived
type: bug_fix
base_commit: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
---

# Fix 269

## Intent

fix 269

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- A session started with chough session new and no objective holds the placeholder as its '# ...' title and its Goal; the first chough checkpoint --objective replaces each that still holds the untouched placeholder with the objective, so the archived story and its journal post carry the objective and never the placeholder. A later objective change rewrites neither, and a Goal or title a person wrote is kept byte for byte.

## No-spec Rationale

Not applicable
