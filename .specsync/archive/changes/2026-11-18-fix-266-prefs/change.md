---
id: fix-266-prefs
state: archived
type: bug_fix
base_commit: 968172b73473b5f9149355b47bb9a59a7ea48d17
---

# Fix 266 prefs

## Intent

fix 266 prefs

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- While a path under .chough/ is unmerged, chough pref set and chough pref unset refuse as refused:unmerged, naming the paths, with .chough/preferences.json and every other file unchanged; chough pref show still runs. With nothing unmerged they write as before, under the writer lock.

## No-spec Rationale

REQ-cli-069 already says every command that writes a Chough record refuses while a path under .chough/ is unmerged; pref set and pref unset wrote .chough/preferences.json without the writer lock, so only their lock and a test change.
