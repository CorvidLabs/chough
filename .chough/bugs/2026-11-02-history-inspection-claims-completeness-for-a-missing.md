---
kind: bug
id: 2026-11-02-history-inspection-claims-completeness-for-a-missing
aliases:
  - BUG-history-inspection-claims-completeness-for-a-missing-referenced-pruned-c15bc08868b100926c733b0b
title: History inspection claims completeness for a missing referenced pruned event
status: verified
classification: product-defect
scope: blocking
impact: Acceptance blocker for proof-preserving history and retention.
criteria: [LIFE-8]
requirements: [REQ-cli-027]
cases: []
evidence:
  - OBS-product-test-f59a8fa906c43f699940f33c
  - OBS-tc-cli-history-adoption-99c96d4d9cfbb7322ae81763
  - OBS-tc-cli-history-post-prune-reference-23f4f23b82735a0a21fc1aab
  - OBS-tc-cli-history-adoption-7e242d1718d20b2678fe6af5
  - OBS-tc-cli-history-adoption-b8147b0b44224077038a22bb
  - OBS-tc-cli-history-post-prune-reference-8e61ca8d5022c39e4631183c
  - OBS-tc-cli-history-post-prune-reference-92a2a07272b35e93c3ece834
  - OBS-tc-cli-history-adoption-6cb6b66e612d1aa174aa9390
  - OBS-tc-cli-history-adoption-9c0a346ae0344141170e9194
receipts:
  - id: 2026-11-02-history-inspection-claims-completeness-for-a-missing
    aliases: OBS-product-test-f59a8fa906c43f699940f33c
    claim: History inspection claims completeness for a missing referenced pruned event reproduced at clean 489ecf50 before source edits. The checkpoint reference remains, its file is absent, and history show reports complete true.
    result: failed
    revision: 489ecf50fc5ddb113a9a1053a2efa26c8a07f677
    environment: local-macos-arm64
    content: fdd264c30beb9a54d5fdc114f4882d3af7f578b2cfaec9be46f6c55296c81e90
    observed: 2026-11-02T17:57:23Z
  - id: 2026-11-02-tc-cli-history-adoption
    aliases: OBS-tc-cli-history-adoption-99c96d4d9cfbb7322ae81763
    claim: All nine Fledge steps and all 225 tests across 11 suites passed; ten exact bound cases passed and original reproductions confirm recent closure protection and explicit missing referenced history.
    result: passed
    revision: 489ecf50fc5ddb113a9a1053a2efa26c8a07f677
    environment: local-macos-arm64
    content: 9fe40085660dc33fbcca4fe27ee091587762e75a578a0aebe9699b2d94a81bef
    fingerprint: sha256:afb41b03ba9506c931dfe9ceb278fb9648e38c6a0bde8296e0dd16740b05496b
    case: TC-cli-history-adoption
    case_digest: b05d1faad38150e1d51f5d216716b57dae8e7c84272563b14978cf669cb7da45
    observed: 2026-11-02T18:15:07Z
  - id: OBS-tc-cli-history-post-prune-reference-23f4f23b82735a0a21fc1aab
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-history-adoption-7e242d1718d20b2678fe6af5
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-history-adoption-b8147b0b44224077038a22bb
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-history-post-prune-reference-8e61ca8d5022c39e4631183c
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-03-tc-cli-history-post-prune-reference
    aliases: OBS-tc-cli-history-post-prune-reference-92a2a07272b35e93c3ece834
    claim: Independent review reproduced a known exact-file path prefix followed by .bak being treated as that file alone; after partial restore history incorrectly reported complete. Source/binary identities and full raw fixture are retained.
    result: failed
    revision: b25731739beba8a6fe9ad0e8c9e0fb126211da30
    environment: local-macos-arm64
    content: 46d88d2af570d0495176f703f84ffb9a4e582b3c4aa630138a909facc3cb3f34
    case: TC-cli-history-post-prune-reference
    case_digest: ae1a5c85c3011b0a4da18c977587b9348fc371c48e34cacd48f9b39e7af4c2d3
    observed: 2026-11-03T00:17:41Z
  - id: OBS-tc-cli-history-adoption-6cb6b66e612d1aa174aa9390
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-history-adoption-9c0a346ae0344141170e9194
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
fixture: .chough/artifacts/history-post-prune-reference-before
environment: local-macos-arm64
revision: 489ecf50fc5ddb113a9a1053a2efa26c8a07f677
found_by: corvid-agent PR37 review5740236776, independently reproduced by Codex
related: []
created: 2026-11-02T17:57:23Z
updated: 2026-11-03T00:31:41Z
---
# History inspection claims completeness for a missing referenced pruned event

**What happened:** The checkpoint reference remains, its file is absent, and history show reports complete true.

**Steps:** Branch A prunes; branch B forked before pruning references the removed event; Git merges A into B cleanly.

**Expected:** History inspection explicitly reports partial missing referenced history and its source.

**Why it matters:** Acceptance blocker for proof-preserving history and retention.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T17:57:23Z · Reported (corvid-agent PR37 review5740236776, independently reproduced by Codex); evidence: OBS-product-test-f59a8fa906c43f699940f33c
- 2026-11-02T18:15:08Z · status: open -> verified. Both independently reproduced failures now have failure-first regressions, original-reproducer confirmation, ten passed bound cases, all225tests on currentRust/Rust1.88 and all nine Fledge steps. Failed proof remains unchanged.; evidence: OBS-tc-cli-history-adoption-99c96d4d9cfbb7322ae81763
- 2026-11-02T22:49:49Z · status: verified -> open. Reopened from incremental PR37 review4446381376: exact-event restoration is fixed but bare and single-quoted cycle-directory references still bypass missing-reference diagnostics/refusal. Original passed and failed observations remain intact.; evidence: OBS-tc-cli-history-post-prune-reference-23f4f23b82735a0a21fc1aab
- 2026-11-02T23:05:42Z · status: open -> verified. Bare and single-quoted directory gap now shares normalized whole-token classification with all other forms. All five real Git reference/restoration fixtures, all22historytests, all225tests on both Rust versions, ten cases and nine Fledge steps pass. Original failures remain intact.; evidence: OBS-tc-cli-history-adoption-7e242d1718d20b2678fe6af5
- 2026-11-02T23:32:44Z · Final integrated main9d02 source retains the five-form directory/exact-file repair and passes all232tests on both Rust versions, ten cases and nine Fledge steps; no failure or old proof was rewritten.; evidence: OBS-tc-cli-history-adoption-b8147b0b44224077038a22bb
- 2026-11-02T23:59:09Z · status: verified -> open. Independent incremental review confirmed protected punctuation forms still bypass missing-reference diagnostics. Repair must classify every recognized cycle mention consistently while preserving exact-file-only restoration. Older failures and passes remain intact.; evidence: OBS-tc-cli-history-post-prune-reference-8e61ca8d5022c39e4631183c
- 2026-11-03T00:17:52Z · Independent review found a known event path followed by .bak still classified as the exact file. The bug remains open and blocking until complete path-boundary classification, red/green regressions and final combined-source qualification pass. Original raw failures remain intact.; evidence: OBS-tc-cli-history-post-prune-reference-92a2a07272b35e93c3ece834
- 2026-11-03T00:31:41Z · status: open -> verified. Occurrence and both-edge path classification now cover17 real Git variants and24 frozen alias/punctuation fixtures. Both strengthened regressions failed before repair and pass afterward; all 233 tests on Rust 1.95/1.88, ten cases, all nine Fledge steps and both original reproducers pass. Independent scoped rereview has no remaining finding. All earlier failures and protected proof remain unchanged.; evidence: OBS-tc-cli-history-adoption-6cb6b66e612d1aa174aa9390, OBS-tc-cli-history-adoption-9c0a346ae0344141170e9194
