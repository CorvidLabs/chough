---
kind: bug
id: 2026-11-01-checkpoint-parallel-merge-conflicts-in-real-git
aliases: [BUG-checkpoint-parallel-merge-conflicts-in-real-git-f8779cb8507b621927a0d9ee]
title: Checkpoint parallel merge conflicts in real Git
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
cases: []
evidence:
  - checkpoint-merge-real-git-pre-fix
  - checkpoint-merge-real-git-post-fix
  - checkpoint-merge-real-git-final
receipts:
  - id: 2026-11-01-real-git-cannot-yet-merge-compatible-parallel-schema-2
    aliases: checkpoint-merge-real-git-pre-fix
    claim: Real Git cannot yet merge compatible parallel schema 2 checkpoint changes
    result: failed
    revision: 5bab395af745e328e918a5f358c5149d673e47ca
    environment: local-macos-arm64
    content: 00172e42eed0b28fec247932c8fe1cc2532740bbcc47d6fb7f351258e0320bf3
    fingerprint: sha256:322890afc0f9ea2a374a5a21df22b9150b26194596c4cb91beef6d456cc8fe6f
    observed: 2026-11-01T23:22:37Z
  - id: 2026-11-01-the-configured-driver-merges-compatible-parallel-schema-2
    aliases: checkpoint-merge-real-git-post-fix
    claim: The configured driver merges compatible parallel schema 2 checkpoint changes in real Git
    result: passed
    revision: 5bab395af745e328e918a5f358c5149d673e47ca
    environment: local-macos-arm64
    content: 80c3c8b4a8eef70d159d562aa2caca92f24c40f7866abb61b8287ebd9960d08f
    fingerprint: sha256:e7418990c391030b8ad4782989dcd54f024a9739167c8fba937087d993ed1a58
    observed: 2026-11-01T23:39:07Z
  - id: checkpoint-merge-real-git-final
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
environment: local-macos-arm64
revision: 5bab395af745e328e918a5f358c5149d673e47ca
found_by: manual report
related: []
created: 2026-11-01T23:22:52Z
updated: 2026-11-01T23:47:22Z
---
# Checkpoint parallel merge conflicts in real Git

**What happened:** Compatible additive schema 2 checkpoint edits remain unresolved in a real Git merge
Actual: Ordinary Git reports a content conflict with conflict markers; the configured driver exits on unexpected argument --merge and leaves the path unresolved

**Steps:** Merge two branches from one schema 2 base with one-side removals and an identical addition, first without a Chough driver and then with the documented driver

**Expected:** Git cleanly merges compatible collections in base, ours, theirs order and preserves one-side removals

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T23:22:52Z · Reported (manual report); evidence: checkpoint-merge-real-git-pre-fix
- 2026-11-01T23:39:09Z · status: open -> verified. The unchanged real Git regression passed after the merge implementation and review fixes; evidence: checkpoint-merge-real-git-post-fix
- 2026-11-01T23:47:22Z · Final-content real Git regression passed after all implementation, documentation and native task edits; evidence: checkpoint-merge-real-git-final
