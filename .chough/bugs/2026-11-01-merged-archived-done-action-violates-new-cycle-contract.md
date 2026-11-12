---
kind: bug
id: 2026-11-01-merged-archived-done-action-violates-new-cycle-contract
aliases: [BUG-0025]
title: Merged archived done action violates new-cycle contract
status: verified
classification: product-defect
scope: in-scope
impact: The integrated PR would violate the newer approved contract even though its isolated tests pass
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-021]
cases: [TC-cli-0105]
evidence:
  - tc-cli-0105-bug-0025-pre-fix
  - tc-cli-0105-bug-0025-post-fix
  - tc-cli-0105-integration-final
receipts:
  - id: 2026-11-01-tc-cli-0105-96a4c9
    aliases: tc-cli-0105-bug-0025-pre-fix
    claim: "Ran cargo test --locked --test stages -- --exact done_archived_change_recommends_new_cycle_and_preserves_precedence for TC-cli-0105: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test stages -- --exact done_archived_change_recommends_new_cycle_and_preserves_precedence
    revision: f38e70f58a6dbb65e3be21d066ffa243e2bde234
    environment: local-macos-arm64
    content: 9e1ea29cad4da0623c995ca675dd547b56d293accc3ac9f9a6fc767f4038e2f3
    fingerprint: sha256:a35055f089089896d5a2a0251fc75f1803475b21932a3a5e191d9f5a0a9d857f
    case: TC-cli-0105
    case_digest: 61acdd91c64609ec4bcea9cf12f598d6d2d074f6ff9ce2f061329008fbed8a20
    observed: 2026-11-01T19:36:14Z
    duration_seconds: 2
  - id: 2026-11-01-tc-cli-0105-07e9a3
    aliases: tc-cli-0105-bug-0025-post-fix
    claim: "Ran cargo test --locked --test stages -- --exact done_archived_change_recommends_new_cycle_and_preserves_precedence for TC-cli-0105: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact done_archived_change_recommends_new_cycle_and_preserves_precedence
    revision: f38e70f58a6dbb65e3be21d066ffa243e2bde234
    environment: local-macos-arm64
    content: 71723223eb386b945ffeec41e1d9056ed5c2224ac7f916d3eb56819f38d91234
    fingerprint: sha256:b0020a976e4770f7bef7aeece8a2ed883eb91ce717f318c45ed72c28220d2fe0
    case: TC-cli-0105
    case_digest: 61acdd91c64609ec4bcea9cf12f598d6d2d074f6ff9ce2f061329008fbed8a20
    observed: 2026-11-01T19:39:41Z
    duration_seconds: 1
  - id: 2026-11-01-tc-cli-0105-15b5e2
    aliases: tc-cli-0105-integration-final
    claim: "Ran cargo test --locked --test stages -- --exact done_archived_change_recommends_new_cycle_and_preserves_precedence for TC-cli-0105: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact done_archived_change_recommends_new_cycle_and_preserves_precedence
    revision: f38e70f58a6dbb65e3be21d066ffa243e2bde234
    environment: local-macos-arm64
    content: 2e3b9acb3740022e7f103c41ba8df226191cdbaa4f0788a3171f2d1c91882c9f
    fingerprint: sha256:bea230a97988f49e28cfb8ad3116cb38aad050bda7dde11678495be9a231be17
    case: TC-cli-0105
    case_digest: 61acdd91c64609ec4bcea9cf12f598d6d2d074f6ff9ce2f061329008fbed8a20
    observed: 2026-11-01T20:08:21Z
    duration_seconds: 1
environment: local-macos-arm64
revision: b2e2f19d904aec2a5acd73569c8439f5583c01f9
found_by: manual report
related: []
created: 2026-11-01T19:17:17Z
updated: 2026-11-01T20:08:24Z
---
# Merged archived done action violates new-cycle contract

**What happened:** The integrated branch follows native summary.next_action instead of the REQ-cli-021 new-cycle action
Actual: The archived override recommends the native summary.next_action and bypasses main's new-cycle rule

**Steps:** Run chough next or status for a done checkpoint with an archived authenticated native change at merge b2e2f19
1. Inspect the merged assess override in src/main.rs
2. Run the updated archived-done regression case before removing the override

**Expected:** Recommend a reason-bearing stage reset and a new native change under REQ-cli-021 without repeating stale lifecycle actions

**Why it matters:** The integrated PR would violate the newer approved contract even though its isolated tests pass

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T19:17:17Z · Reported (manual report)
- 2026-11-01T19:17:17Z · Added despite similar reports: BUG-0024 records the earlier stale review and ship defect; this bug is the later merge violation of main's REQ-cli-021
- 2026-11-01T19:36:45Z · Pre-fix TC-cli-0105 reproduced the archived native action at integrated tip f38e70f before the src/main.rs fix.; evidence: tc-cli-0105-bug-0025-pre-fix
- 2026-11-01T19:40:25Z · status: open -> verified. Post-fix TC-cli-0105 passed after the linked pre-fix failure; focused stages and next tests also passed.; evidence: tc-cli-0105-bug-0025-post-fix
- 2026-11-01T20:08:24Z · Final-content TC-cli-0105 passed after the retained pre-fix failure and after all fingerprinted documentation and native task edits; evidence: tc-cli-0105-integration-final
