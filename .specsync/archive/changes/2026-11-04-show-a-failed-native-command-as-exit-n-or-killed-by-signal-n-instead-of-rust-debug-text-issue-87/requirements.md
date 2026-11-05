---
change: show-a-failed-native-command-as-exit-n-or-killed-by-signal-n-instead-of-rust-debug-text-issue-87
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-003 (see `deltas/cli.md`): a failed native command is shown with its
result in plain words, `exit <code>` or `killed by signal <number>`. Trace unchanged
plus LIFE-7.
