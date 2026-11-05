---
id: show-a-failed-native-command-as-exit-n-or-killed-by-signal-n-instead-of-rust-debug-text-issue-87
state: archived
type: bug_fix
base_commit: 91a1fdd82a4e314e5037c6256f17199ef0d604f0
---

# Show a failed native command as exit N or killed by signal N instead of Rust debug text (issue 87)

## Intent

Show a failed native command as exit N or killed by signal N instead of Rust debug text (issue 87)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A native command that exits 2 is shown as exit 2 in status and next and never as Some. A native command ended by SIGTERM is shown as killed by signal 15. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
