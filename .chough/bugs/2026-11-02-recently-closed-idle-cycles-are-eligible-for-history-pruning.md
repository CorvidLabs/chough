---
kind: bug
id: 2026-11-02-recently-closed-idle-cycles-are-eligible-for-history-pruning
aliases: [BUG-recently-closed-idle-cycles-are-eligible-for-history-pruning-be8b45601a1249bee88c19a4]
title: Recently closed idle cycles are eligible for history pruning
status: verified
classification: product-defect
scope: blocking
impact: Acceptance blocker for proof-preserving history and retention.
criteria: [LIFE-8]
requirements: [REQ-cli-027]
cases: []
evidence:
  - OBS-product-test-4d5f09a757e2e50d651abef4
  - OBS-tc-cli-history-adoption-99c96d4d9cfbb7322ae81763
receipts:
  - id: 2026-11-02-recently-closed-idle-cycles-are-eligible-for-history-pruning
    aliases: OBS-product-test-4d5f09a757e2e50d651abef4
    claim: Recently closed idle cycles are eligible for history pruning reproduced at clean 489ecf50 before source edits. Preview makes the newly closed cycle eligible and authorized disposable apply removes its event.
    result: failed
    revision: 489ecf50fc5ddb113a9a1053a2efa26c8a07f677
    environment: local-macos-arm64
    content: fdd264c30beb9a54d5fdc114f4882d3af7f578b2cfaec9be46f6c55296c81e90
    observed: 2026-11-02T17:57:09Z
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
fixture: .chough/artifacts/history-idle-closure-before
environment: local-macos-arm64
revision: 489ecf50fc5ddb113a9a1053a2efa26c8a07f677
found_by: corvid-agent PR37 review5740236776, independently reproduced by Codex
related: []
created: 2026-11-02T17:57:23Z
updated: 2026-11-02T18:15:08Z
---
# Recently closed idle cycles are eligible for history pruning

**What happened:** Preview makes the newly closed cycle eligible and authorized disposable apply removes its event.

**Steps:** An idle thirty-day-old cycle closes now and four newer resets shift the newest-three window.

**Expected:** The fourteen-day protection starts at the closing successor reset.

**Why it matters:** Acceptance blocker for proof-preserving history and retention.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T17:57:23Z · Reported (corvid-agent PR37 review5740236776, independently reproduced by Codex); evidence: OBS-product-test-4d5f09a757e2e50d651abef4
- 2026-11-02T18:15:08Z · status: open -> verified. Both independently reproduced failures now have failure-first regressions, original-reproducer confirmation, ten passed bound cases, all225tests on currentRust/Rust1.88 and all nine Fledge steps. Failed proof remains unchanged.; evidence: OBS-tc-cli-history-adoption-99c96d4d9cfbb7322ae81763
