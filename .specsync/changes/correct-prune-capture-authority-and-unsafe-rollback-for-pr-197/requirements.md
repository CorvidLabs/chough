---
change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
artifact: requirements
---

# Requirements and traceability

The existing requirement bodies are unchanged; this artifact defines corrective acceptance under them.

| Existing source | Required corrective outcome | Oracle |
|---|---|---|
| LIFE-8, REQ-history-004 | Captured artifact bytes must match the eligible fingerprint before they become removal authority; replacements survive. Earlier removals roll back on a later capture mismatch. | Deterministic capture hook; exact writer bytes, record retention and earlier-set recovery assertions |
| LIFE-2, LIFE-7, LIFE-8, REQ-history-004 | A recreated symlink at a removed file or its ancestor stays untouched, original bytes survive in a safe named hold, other held files are still attempted, and every ledgered set gets honest restored accounting. | Two deterministic rollback variants; symlink inode/target, hold bytes, earlier artifact/record/mode and ledger event assertions |
| LIFE-2, LIFE-7 | Keep historical acceptance immutable; native scope approval and a fresh implementation review are separate. | Exact predecessor obligations; archived-byte manifests; real human gates |

CLI Purpose and history Error Cases receive the aligned section deltas. No requirement body, ID, strict split digest, policy, output/ledger schema or command interface is changed.
