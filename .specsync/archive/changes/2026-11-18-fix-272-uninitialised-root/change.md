---
id: fix-272-uninitialised-root
state: archived
type: bug_fix
base_commit: 7aed03f6993eef37826de783c84ccb4d9ab52648
---

# Fix 272 uninitialised root

## Intent

fix 272 uninitialised root

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- In a project with no .chough directory, chough journal add takes the writer lock .chough/local/checkpoint.lock before it writes, and twelve journal add commands run at once leave a feed that lists all twelve posts, with chough library index --check passing.

## No-spec Rationale

REQ-cli-040 already names journal add among the commands that hold the writer lock, with no exception for a project that has no .chough directory yet; only the code and its tests change.
