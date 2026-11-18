---
id: fix-298
state: archived
type: bug_fix
base_commit: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
---

# Fix 298

## Intent

fix 298

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- When a commit between the change's base commit and HEAD holds a change both open under .specsync/changes/<id>/ and archived under .specsync/archive/changes/<date>-<id>/, chough change ship refuses before it runs SpecSync's review or ship, naming the commit, the duplicated change and how to recover, with every file unchanged and no native step run; without such a commit the ship runs as before. The docs say the check is interim until SpecSync's preflight (CorvidLabs/spec-sync#1082).

## No-spec Rationale

Not applicable
