---
kind: bug
id: 2026-11-01-archived-done-checkpoint-repeats-stale-review-and-ship-action
aliases: [BUG-0024]
title: Archived done checkpoint repeats stale review and ship action
status: verified
classification: product-defect
scope: in-scope
impact: A cold agent can repeat a human lifecycle command or fail to identify the real next action
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-003, REQ-cli-007]
cases: []
evidence:
  - kimi-cold-start-5af3f78
  - kimi-cold-start-5af3f78-scope
  - tc-cli-0105-1793551352
  - tc-cli-0106-kimi-original
  - tc-cli-0105-1793553670
receipts:
  - id: 2026-11-01-original-kimi-code-cli-0-41-0-cold-start-discovered-the-agents
    aliases: kimi-cold-start-5af3f78
    claim: Original Kimi Code CLI 0.41.0 cold start discovered the .agents skill and completed local checks, but safe cold resume failed because next_valid_action repeated stale review and ship guidance and Kimi mapping was undocumented
    result: failed
    revision: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
    environment: local-macos-arm64
    content: b877fa05caa2297084ca7a9beb1828f9a91d42c7c32675846fe9f624b6618d47
    fingerprint: sha256:66de8ea91e3c1f7a8c35f6958f2a4d134782b806b2ff77876ade31a8305b2c97
    observed: 2026-11-01T16:25:22Z
  - id: 2026-11-01-retrospective-review-of-the-original-kimi-code-cli-0-41-0-cold
    aliases: kimi-cold-start-5af3f78-scope
    claim: "Retrospective review of the original Kimi Code CLI 0.41.0 cold start on the completed scope definition: discovery and local checks succeeded, but safe cold resume failed on stale archived guidance and missing Kimi mapping"
    result: failed
    revision: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
    environment: local-macos-arm64
    content: 0c308b91d4f20f4c9840bf918118bc459b38882bac949998052aab238cc35744
    fingerprint: sha256:66de8ea91e3c1f7a8c35f6958f2a4d134782b806b2ff77876ade31a8305b2c97
    observed: 2026-11-01T16:30:45Z
  - id: tc-cli-0105-1793551352
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-01-tc-cli-0106
    aliases: tc-cli-0106-kimi-original
    claim: "Verdicts 1=pass, 2=fail, 3=fail, 4=pass. Original retrospective Kimi cold-start review: the preserved run facts and human-gate limits pass, but Kimi mapping guidance and consistent corrected docs are absent at base"
    result: failed
    revision: 1d9264f71168f9f9cf2a50d45ab118e517b5a3ee
    environment: local-macos-arm64
    content: 7e03f73492d75b516a35048fce831115d2b1785a8651d0cb12a89d4f7b094702
    fingerprint: sha256:66de8ea91e3c1f7a8c35f6958f2a4d134782b806b2ff77876ade31a8305b2c97
    case: TC-cli-0106
    case_digest: 9306885ac0365a73944302c527dae2935465f1f55c932f4de386eb243eb9554f
    observed: 2026-11-01T16:43:04Z
  - id: 2026-11-01-tc-cli-0105
    aliases: tc-cli-0105-1793553670
    claim: "Ran cargo test --locked --test stages -- --exact done_archived_native_guidance_drives_next_and_status for TC-cli-0105: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact done_archived_native_guidance_drives_next_and_status
    revision: 1d9264f71168f9f9cf2a50d45ab118e517b5a3ee
    environment: local-macos-arm64
    content: 5b05fcd19483f75a103491b5bb875250e916191adc423a4c18ae033de6a61365
    fingerprint: sha256:2ca9e159b78b372a84eef52afc5f38a7d95d39d89f46ff240194e454ac9fa0f0
    case: TC-cli-0105
    case_digest: 09794b3e7e55214990356af668130d699189e0169836ed573562ab296344edea
    observed: 2026-11-01T17:21:12Z
    duration_seconds: 2
fixture: The existing checkpoint and its archived bundle hygiene change
environment: local-macos-arm64
revision: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
found_by: manual report
related: []
created: 2026-11-01T15:16:41Z
updated: 2026-11-01T17:21:49Z
---
# Archived done checkpoint repeats stale review and ship action

**What happened:** next_valid_action contradicts archived native state
Actual: It prints Continue done with specsync change review and ship while native_summary.state is archived and native guidance is different

**Steps:** Run chough next with a done checkpoint whose archived native change has an old review and ship next_action
1. Start from origin/main with its existing done checkpoint
2. Run ./target/debug/chough next --environment local-macos-arm64

**Expected:** next_valid_action follows native SpecSync guidance and never recommends repeating completed lifecycle steps

**Why it matters:** A cold agent can repeat a human lifecycle command or fail to identify the real next action

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T15:16:41Z · Reported (manual report)
- 2026-11-01T16:25:22Z · The original Kimi cold-start observation supplies failed evidence for this finding; its recorded scope and open status are unchanged; evidence: kimi-cold-start-5af3f78
- 2026-11-01T16:30:46Z · The retrospective scope-gate observation preserves the same original failure on the completed draft content; scope and open status are unchanged; evidence: kimi-cold-start-5af3f78-scope
- 2026-11-01T16:43:04Z · TC-cli-0105 failed at the unfixed tip with one test run: next_valid_action repeated stale review and ship text instead of archived native guidance; evidence: tc-cli-0105-1793551352
- 2026-11-01T16:43:22Z · The original TC-cli-0106 retrospective observation preserves evidence for this finding before the approved fixes; scope and open status are unchanged; evidence: tc-cli-0106-kimi-original
- 2026-11-01T17:21:49Z · The final unchanged TC-cli-0105 case passed after all content edits, later than every linked failure; evidence: tc-cli-0105-1793553670
- 2026-11-01T17:21:49Z · status: open -> verified. Verified only from the later TC-cli-0105 pass after the retained unfixed failure
