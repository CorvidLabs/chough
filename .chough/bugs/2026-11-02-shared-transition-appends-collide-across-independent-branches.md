---
kind: bug
id: 2026-11-02-shared-transition-appends-collide-across-independent-branches
aliases:
  - BUG-shared-transition-appends-collide-across-independent-branches-8bcc3168f918377ac434cd77
title: Shared transition appends collide across independent branches
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: [LIFE-3]
requirements: [REQ-cli-008]
cases: []
evidence:
  - OBS-product-test-41e7829d3ff0fdecd63a45ee
  - OBS-tc-cli-history-branches-af8a1be077b88571650a8232
receipts:
  - id: 2026-11-02-before-implementation-cargo-test-locked-test-history-failed
    aliases: OBS-product-test-41e7829d3ff0fdecd63a45ee
    claim: Before implementation, cargo test --locked --test history failed all three regressions. The real Git fixture produced a shared transitions.jsonl merge conflict; new-project cursor and explicit adoption are absent.
    result: failed
    revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
    environment: local-macos-arm64
    content: 54273b17f7ff83319eddbed2ec49b6f405207b24d15e7ca7406a7d743cfd0194
    fingerprint: sha256:c2e7607dc569b1f87843a7143e5d322fb5dd2ff7c923a227f5087c32e18652f9
    observed: 2026-11-02T13:11:23Z
  - id: 2026-11-02-tc-cli-history-branches
    aliases: OBS-tc-cli-history-branches-af8a1be077b88571650a8232
    claim: "Ran cargo test --locked --test history -- --exact real_git_branch_events_survive_while_checkpoint_selection_remains_explicit for TC-cli-history-branches: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact real_git_branch_events_survive_while_checkpoint_selection_remains_explicit
    revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
    environment: local-macos-arm64
    content: 0706650c13e025a3e3af738b2b3a63ed9e32dcdf22d03c217ee27d913c34e031
    fingerprint: sha256:7a291e202d2832395fa92c9b4fd853019836b70323f2eb66b5928f30887bb8c4
    case: TC-cli-history-branches
    case_digest: ff026a031c9844283eda3e2ffe51b46797ae11a898f12fa638c68a27a13155d9
    observed: 2026-11-02T14:55:10Z
    duration_seconds: 1
fixture: tests/history.rs
environment: local-macos-arm64
revision: 9a8b9b11e16bedac4808c56d48307fa29ad9cfe3
found_by: "Observed failure-first regression for approved issue #31 scope"
related: []
created: 2026-11-02T13:11:37Z
updated: 2026-11-02T15:02:10Z
---
# Shared transition appends collide across independent branches

**What happened:** Two branch-local stage events edit the same transition log and cannot integrate independently
Actual: Git reports .chough/transitions.jsonl as the unmerged path

**Steps:** Run the real_git_branch_events_survive_while_checkpoint_selection_remains_explicit regression before the history implementation

**Expected:** Both immutable branch event files survive while any divergent checkpoint cursor requires explicit selection

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T13:11:37Z · Reported (Observed failure-first regression for approved issue #31 scope); evidence: OBS-product-test-41e7829d3ff0fdecd63a45ee
- 2026-11-02T15:02:10Z · status: open -> verified. The later real Git case passes with preserved event paths and explicit coherent checkpoint selection.; evidence: OBS-tc-cli-history-branches-af8a1be077b88571650a8232
