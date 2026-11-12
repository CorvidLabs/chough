---
kind: bug
id: 2026-11-01-cold-start-next-and-status-output-is-difficult-to-scan
aliases: [BUG-0023]
title: Cold-start next and status output is difficult to scan
status: verified
classification: requirement-gap
scope: in-scope
impact: Cold resume is slow and the important contradiction is easy to miss
criteria: [LIFE-3]
requirements: [REQ-cli-007]
cases: []
evidence:
  - kimi-cold-start-5af3f78
  - kimi-cold-start-5af3f78-scope
  - tc-cli-0106-kimi-original
  - OBS-tc-cli-bug-0023-status-leads-with-summary-756b63467d9055a1e6ef254a
  - OBS-tc-cli-bug-0023-status-leads-with-summary-93588cb44d8c09012f035462
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
  - id: 2026-11-11-tc-cli-bug-0023-status-leads-with-summary
    aliases: OBS-tc-cli-bug-0023-status-leads-with-summary-756b63467d9055a1e6ef254a
    claim: "Ran cargo test --locked --test next -- --exact status_leads_with_the_same_summary_as_next for TC-cli-bug-0023-status-leads-with-summary: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test next -- --exact status_leads_with_the_same_summary_as_next
    revision: dbd3d23b78ad4f001968530f6cd336105f4bbf58
    environment: local-macos-arm64
    content: 4e698547b6c905aa2792cff66eccfd248ecdb5b9d891fb6997a50f038a38e75a
    fingerprint: sha256:b678ef4859df6b6e03eaf4140df8f86e6fb1b06c27aef298050b8175dedf13aa
    case: TC-cli-bug-0023-status-leads-with-summary
    case_digest: c8bb64427e302bda89dff9ddffacb53aecf9787c51498ea4d5c2111ac99715b7
    observed: 2026-11-11T19:39:13Z
    duration_seconds: 1
  - id: 2026-11-11-tc-cli-bug-0023-status-leads-with-summary-1df337
    aliases: OBS-tc-cli-bug-0023-status-leads-with-summary-93588cb44d8c09012f035462
    claim: "Ran cargo test --locked --test next -- --exact status_leads_with_the_same_summary_as_next for TC-cli-bug-0023-status-leads-with-summary: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact status_leads_with_the_same_summary_as_next
    revision: f9c0ae674c89f3a4c4a6c5b9d29057f22f7b9bf0
    environment: local-macos-arm64
    content: 4adc6e184b41d042b73284db8f143439ad1bd2bdc6fc389e3a5266068b430ba2
    fingerprint: sha256:f42bcb56a687e60cb83e0f7016aaca4a46af6b7d215db254a14e1225e3e270f2
    case: TC-cli-bug-0023-status-leads-with-summary
    case_digest: c8bb64427e302bda89dff9ddffacb53aecf9787c51498ea4d5c2111ac99715b7
    observed: 2026-11-11T19:42:54Z
    duration_seconds: 0
environment: local-macos-arm64
revision: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
found_by: manual report
related: []
created: 2026-11-01T15:16:42Z
updated: 2026-11-11T19:43:31Z
---
# Cold-start next and status output is difficult to scan

**What happened:** The next action is buried in very large JSON output
Actual: next emitted 11567 JSON lines in 16 seconds and status emitted 11567 JSON lines in 36 seconds, including full native content and observation records

**Steps:** Run chough next and chough status --json on the repository checkpoint with its existing evidence history
1. Capture command output and count the lines

**Expected:** A fresh agent can identify the native and session actions without manually inspecting thousands of lines

**Why it matters:** Cold resume is slow and the important contradiction is easy to miss

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T15:16:42Z · Reported (manual report)
- 2026-11-01T16:06:27Z · The read-only chough timing command also took about 233 seconds and emitted 51507 bytes; plan, test and human-gate histories had too few samples. This is additional output-size and performance friction, not work included in this change
- 2026-11-01T16:24:40Z · Correction to the original actual field: both next and status emitted 11567 JSON lines, not 11568 for status. The reviewer counted the JSON payload boundaries in the preserved scratch capture; the timing-output note is unchanged
- 2026-11-01T16:25:22Z · The original Kimi cold-start observation supplies failed evidence for this finding; its recorded scope and open status are unchanged; evidence: kimi-cold-start-5af3f78
- 2026-11-01T16:30:46Z · The retrospective scope-gate observation preserves the same original failure on the completed draft content; scope and open status are unchanged; evidence: kimi-cold-start-5af3f78-scope
- 2026-11-01T16:43:22Z · The original TC-cli-0106 retrospective observation preserves evidence for this finding before the approved fixes; scope and open status are unchanged; evidence: tc-cli-0106-kimi-original
- 2026-11-11T19:39:22Z · status: open -> fix-pending-verification; scope: out-of-scope -> in-scope. Red on 0.5.0: TC-cli-bug-0023-status-leads-with-summary failed on leif/0.6.0-76-0023-red at dbd3d23b. Fixed in plan step 12 (PR #238): status JSON leads with a summary (stage, the same next action and command as next, open gates, blockers).; evidence: OBS-tc-cli-bug-0023-status-leads-with-summary-756b63467d9055a1e6ef254a
- 2026-11-11T19:43:31Z · status: fix-pending-verification -> verified. Fixed in plan step 12 (PR #238, f9c0ae67); the case passed after its 0.5.0 failure.; evidence: OBS-tc-cli-bug-0023-status-leads-with-summary-93588cb44d8c09012f035462
