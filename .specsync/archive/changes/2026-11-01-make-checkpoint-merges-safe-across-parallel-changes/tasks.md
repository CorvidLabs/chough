---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: tasks
---

# Tasks

- [x] Add focused checkpoint merge and real Git regressions, retain `checkpoint-merge-real-git-pre-fix`, and link the local merge bug before implementation.
- [x] Implement bounded schema 2 `checkpoint --merge` with ordered exact-string collections, preserved removals and atomic current-cycle selection.
- [x] Validate every input and the complete result before writing, replace only `ours`, and preserve every input and surrounding record on refusal.
- [x] Cover schema 1, mixed and malformed inputs, unknown fields, unsafe paths, consent and collection bounds, and closed-stdout reporting after a successful write.
- [x] Add the attributes declaration, per-clone driver documentation and explicit fallback without changing Git configuration during init.
- [x] Apply the CLI, architecture, FRD and checkpoint-skill updates, refresh all installed references, register the previous digest and preserve edited copies.
- [x] Pass 33 stage tests, 26 CLI tests, 11 package tests, 7 optional tests, formatting and warnings-denied Clippy after the review fixes.

Materialization and final evidence: REQ-cli-007 is materialized and `checkpoint-merge-real-git-post-fix` passed after the retained failure, verifying the linked local bug. The full verify lane passed 9 steps in 221.890 seconds, the full Rust suite passed with Rust 1.88.0, and pre-push passed 5 steps in 5.422 seconds. A final current-content real Git observation is pending before acceptance.

Human gates (not tasks): scope approval is recorded for 0xLeif. Human implementation review, native finalization and archive, and the merge decision remain separate human decisions. No checked task, structural pass or agent label approves, finalizes, merges, releases or deploys this change.
