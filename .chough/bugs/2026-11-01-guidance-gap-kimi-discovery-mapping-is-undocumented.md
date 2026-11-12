---
kind: bug
id: 2026-11-01-guidance-gap-kimi-discovery-mapping-is-undocumented
aliases: [BUG-0021]
title: "Guidance gap: Kimi discovery mapping is undocumented"
status: verified
classification: requirement-gap
scope: in-scope
impact: Cold discovery works, but a Kimi user cannot reproduce the installation from the docs without guessing
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
cases: []
evidence:
  - kimi-cold-start-5af3f78
  - kimi-cold-start-5af3f78-scope
  - tc-cli-0106-kimi-original
  - tc-cli-0037-1793553673
  - tc-cli-0106-kimi-final
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
  - id: 2026-11-01-tc-cli-0037
    aliases: tc-cli-0037-1793553673
    claim: "Ran cargo test --locked --test optional -- --exact skills_install_per_host_and_update_only_unmodified_files for TC-cli-0037: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test optional -- --exact skills_install_per_host_and_update_only_unmodified_files
    revision: 1d9264f71168f9f9cf2a50d45ab118e517b5a3ee
    environment: local-macos-arm64
    content: 5b05fcd19483f75a103491b5bb875250e916191adc423a4c18ae033de6a61365
    fingerprint: sha256:04909105c24575fbcdb93c65717960e85f647db17d7b4254ae138b82170b987f
    case: TC-cli-0037
    case_digest: bc4a585cc7f16f841b4b86815e785e3b9ac99c9a5ce4b698d34b708278b0d94f
    observed: 2026-11-01T17:21:13Z
    duration_seconds: 0
  - id: 2026-11-01-tc-cli-0106-13e391
    aliases: tc-cli-0106-kimi-final
    claim: "Verdicts 1=pass, 2=pass, 3=pass, 4=pass. Later retrospective documentation review: the original log is accurate, Kimi mapping guidance is present without a new host value, affected docs are consistent, and no fresh Kimi run or human decision is claimed"
    result: passed
    revision: 1d9264f71168f9f9cf2a50d45ab118e517b5a3ee
    environment: repository-text
    content: 5b05fcd19483f75a103491b5bb875250e916191adc423a4c18ae033de6a61365
    fingerprint: sha256:66de8ea91e3c1f7a8c35f6958f2a4d134782b806b2ff77876ade31a8305b2c97
    case: TC-cli-0106
    case_digest: 9306885ac0365a73944302c527dae2935465f1f55c932f4de386eb243eb9554f
    observed: 2026-11-01T17:21:14Z
environment: local-macos-arm64
revision: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
found_by: manual report
related: []
created: 2026-11-01T15:16:42Z
updated: 2026-11-01T17:21:49Z
---
# Guidance gap: Kimi discovery mapping is undocumented

**What happened:** A Kimi user is not told which installer host mapping installs the discovered skill
Actual: Kimi discovered .agents automatically, but the docs list other host values and never explain that mapping

**Steps:** Read the Host use section and walkthrough as a Kimi Code CLI user
1. Inspect skills/chough-workflow/SKILL.md, docs/walkthrough.md and docs/integrations.md

**Expected:** Guidance states that Kimi Code CLI discovers the .agents skill installed by the openai mapping

**Why it matters:** Cold discovery works, but a Kimi user cannot reproduce the installation from the docs without guessing

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T15:16:42Z · Reported (manual report)
- 2026-11-01T16:25:22Z · The original Kimi cold-start observation supplies failed evidence for this finding; its recorded scope and open status are unchanged; evidence: kimi-cold-start-5af3f78
- 2026-11-01T16:30:46Z · The retrospective scope-gate observation preserves the same original failure on the completed draft content; scope and open status are unchanged; evidence: kimi-cold-start-5af3f78-scope
- 2026-11-01T16:43:22Z · The original TC-cli-0106 retrospective observation preserves evidence for this finding before the approved fixes; scope and open status are unchanged; evidence: tc-cli-0106-kimi-original
- 2026-11-01T17:21:49Z · Safe-update TC-cli-0037 and the later four-outcome TC-cli-0106 documentation review passed after the original guidance failure; evidence: tc-cli-0037-1793553673, tc-cli-0106-kimi-final
- 2026-11-01T17:21:49Z · status: open -> verified. Verified from the later TC-cli-0106 documentation pass, with TC-cli-0037 supporting safe updates; no fresh Kimi run is claimed
