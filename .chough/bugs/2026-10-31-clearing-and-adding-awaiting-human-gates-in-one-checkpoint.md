---
kind: bug
id: 2026-10-31-clearing-and-adding-awaiting-human-gates-in-one-checkpoint
aliases: [BUG-0005]
title: Clearing and adding awaiting-human gates in one checkpoint call drops the new gate
status: verified
classification: product-defect
scope: out-of-scope
impact: "A real gate can silently disappear from the checkpoint; same ordering applies to --clear-blockers. Workaround: clear and add in separate calls. Fix in a new native change."
criteria: [FLOW-13]
requirements: [REQ-cli-007]
cases: []
evidence: [bug-0005-repro-1, tc-cli-0039-1793475513]
receipts:
  - id: 2026-10-31-checkpoint-clear-awaiting-human-with-awaiting-human-in-the
    aliases: bug-0005-repro-1
    claim: checkpoint --clear-awaiting-human with --awaiting-human in the same call left awaiting_human empty; the clear runs after the additions.
    result: failed
    revision: f860c8a1025700a85aae87f94ce3e883b5a09798
    environment: local-macos-arm64
    fingerprint: sha256:7c10ddcb9eb694b5ba55a49b2d8b0b398b66680e9474b52d3f3ac2c884502993
    observed: 2026-10-31T19:24:08Z
  - id: 2026-10-31-tc-cli-0039
    aliases: tc-cli-0039-1793475513
    claim: "Ran cargo test --locked --test stages -- --exact clear_flags_apply_before_new_entries for TC-cli-0039: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact clear_flags_apply_before_new_entries
    revision: 61185115f5e0700eb9f7cc6787de951e6beea9a6
    environment: local-macos-arm64
    fingerprint: sha256:ce90f5b6f6f39f0fae090089cc3e5fef145ab00347f5f8cbc8c086b4006f27c8
    case: TC-cli-0039
    case_digest: 32582463acabe77f95ca4f83df3a72c5899ff86900996d26395ebae48f94adc3
    observed: 2026-10-31T19:38:34Z
environment: local-macos-arm64
revision: f860c8a1025700a85aae87f94ce3e883b5a09798
found_by: Native closeout of evolve-chough-into-a-resumable-stage-workflow
related: []
created: 2026-10-31T19:24:08Z
updated: 2026-10-31T19:38:34Z
---
# Clearing and adding awaiting-human gates in one checkpoint call drops the new gate

**What happened:** chough checkpoint --clear-awaiting-human --awaiting-human X leaves awaiting_human empty
Actual: X is added and then cleared

**Steps:** Combining a clear flag with new entries for the same list
1. chough checkpoint --awaiting-human Old
2. chough checkpoint --clear-awaiting-human --awaiting-human New

**Expected:** Clear first, then add X

**Why it matters:** A real gate can silently disappear from the checkpoint; same ordering applies to --clear-blockers. Workaround: clear and add in separate calls. Fix in a new native change.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T19:24:08Z · Reported (Native closeout of evolve-chough-into-a-resumable-stage-workflow); evidence: bug-0005-repro-1
- 2026-10-31T19:38:34Z · status: open -> verified. Fixed in 6118511 (clears apply before additions); regression test passes.; evidence: tc-cli-0039-1793475513
