---
id: keep-a-checkpoint-reference-whose-record-exists-but-cannot-be-read-and-refuse-the-closed-cycle-drop-review-of-108
state: implementing
type: bug_fix
base_commit: 784b1406d9e5b4b3a53176c3772aaeb003483412
---

# Keep a checkpoint reference whose record exists but cannot be read, and refuse the closed-cycle drop (review of 108)

## Intent

Keep a checkpoint reference whose record exists but cannot be read, and refuse the closed-cycle drop (review of 108)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- At done, a current-cycle checkpoint reference whose record exists but cannot be read is kept, the step reports refused naming it, no reference is dropped, and done is still recorded; a missing record is still dropped. The new test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-045: only a missing record may be dropped; an unreadable one is kept and the step refused; no requirement text changes.
