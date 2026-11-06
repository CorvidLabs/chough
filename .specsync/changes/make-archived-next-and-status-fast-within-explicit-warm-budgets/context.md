---
change: make-archived-next-and-status-fast-within-explicit-warm-budgets
artifact: context
---

# Context

Issue #32 / plan C8 reports 3-6 s `next` and `status` on Chough's own repository. DRIVE-2 already asks for archived changes to use committed files or a digest-keyed cache, at most one native status read per command, and a `next` response within a few seconds. This package proposes the missing numeric budget and a narrow implementation; it is not yet human-approved.

Profiling at `e99dc7961323e5c4e2aff1a2cf13333b0c47037e` on macOS arm64 with SpecSync 6.2.0 and 121 archives found:

- Direct archived status: 4.36-4.84 s warm, 261 Git children in an instrumented run.
- Direct active-draft status: 1.80-2.25 s, 18 Git children.
- No-active-change status: 0.006-0.020 s; Chough adds four Git calls and substantial local hashing.
- Fresh release `next`/`status` without an active change: 1.39-4.20 s across warm runs, dominated by Git enumeration and two snapshot fingerprint passes.

The full measurements and limitations are in `.chough/artifacts/issue-32-next-status-latency/PROFILE.md`.

PR #69 previously proposed authenticated archive reuse. It is stale/conflicting, reviewer evidence showed that shallow-history changes were absent from its key, and it reused verdicts for stage mutations. The coordinated split assigns its archive cache to this #32 change and batched `stage advance --to` to a separate small PR; #69 closes pointing to both replacements. This proposal retains SpecSync as the only authenticator, adds the missing history inputs, and limits reuse to read-only `next`/`status`. Every stage transition, timing report, retention decision and native verification stays live. No cached active-change status is allowed.

The ID registry audit covered origin/main `260ecc949ea01214f4bec43b30ea4226ce8fae59` and every remote branch. REQ-cli-051, REQ-history-011, FLOW-17 and DRIVE-14 were the highest respective claims at that point. ch-pr69 then claimed REQ-cli-052 for the split stage-advance PR, so this change takes REQ-cli-053 and proposes DRIVE-15; it claims no history or FLOW ID.

The checkpoint-cap owner retains `src/checkpoint.rs`, `src/sessions.rs`, `tests/cli.rs`, `tests/sessions.rs`, and session/capacity records. PR #133 also touches `src/evidence.rs`, `tests/next.rs`, `tests/common/mod.rs`, `docs/cli.md`, and `hi/drive.md`; merge main after it lands and preserve those edits. No rebase or force push is permitted.

Rejected: parsing archive files and deciding terminal validity in Chough, because that creates a second native authority. Rejected: weakening a gate, dropping an invalidation input, increasing a timeout, or caching active native state to meet the number. Cold archived status and active-change native latency are measured but are not given a new latency promise in this change.
