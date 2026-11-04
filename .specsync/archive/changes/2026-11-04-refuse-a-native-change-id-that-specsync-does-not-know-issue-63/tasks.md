---
change: refuse-a-native-change-id-that-specsync-does-not-know-issue-63
artifact: tasks
---

# Tasks

- [x] Failing-first tests for the refusal and the unchanged non-native path
- [x] `ensure_native_change_known` in src/checkpoint.rs, called by `checkpoint --native-change` before any change is applied
- [x] docs/cli.md and CHANGELOG.md
- [x] MODIFIED REQ-cli-007 delta for the refusal, replacing the no-spec rationale (review of #71)
- [x] Refuse a reply that names a different change ID (`refused:native-change-mismatch`, review of #71)
