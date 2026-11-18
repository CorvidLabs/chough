---
id: fix-272
state: archived
type: bug_fix
base_commit: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
---

# Fix 272

## Intent

fix 272

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- While another process holds .chough/local/checkpoint.lock, chough journal add waits and writes no post; once the lock is released it writes its post and a feed that lists it. chough library index --check passes after concurrent journal adds.

## No-spec Rationale

REQ-cli-040 already names journal add among the commands that hold the writer lock; only the code and its test change.
