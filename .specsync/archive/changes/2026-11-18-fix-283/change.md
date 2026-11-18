---
id: fix-283
state: archived
type: bug_fix
base_commit: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
---

# Fix 283

## Intent

fix 283

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- chough cases check passes a retired case whose fledge-task binding names a task no longer in fledge.toml, with the binding kept as written, while an active case naming a missing task is still refused naming the task; cases run still refuses the retired case.

## No-spec Rationale

No requirement says cases check validates a retired case's fledge-task binding; the code checked it before its retired-case early return. Leif decided the check is skipped for retired cases so the original binding stays as history; a retired case was already never run (find_checked), so only the check order, a test and the docs change.
