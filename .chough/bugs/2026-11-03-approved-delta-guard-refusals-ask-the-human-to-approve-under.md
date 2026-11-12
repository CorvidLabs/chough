---
kind: bug
id: 2026-11-03-approved-delta-guard-refusals-ask-the-human-to-approve-under
aliases:
  - BUG-approved-delta-guard-refusals-ask-the-human-to-approve-under-their-own-2ebc91196f2ee5ff4d68f39f
title: Approved-delta guard refusals ask the human to approve under their own name
status: verified
classification: requirement-gap
scope: in-scope
impact: Agents following the refusal either stop to ask the human to run a native command already covered by standing authority, or record a human name as the actor
criteria: [FLOW-5, LIFE-1]
requirements: [REQ-cli-022]
cases: [TC-cli-delta-guard-refusals-name-the-agent-actor]
evidence:
  - OBS-tc-cli-delta-guard-refusals-name-the-agent-actor-a56263c10f646c87c5ed5ea0
  - OBS-tc-cli-delta-guard-refusals-name-the-agent-actor-81c735f25cb340ccfb103c4a
receipts:
  - id: 2026-11-03-tc-cli-delta-guard-refusals-name-the-agent-actor
    aliases: OBS-tc-cli-delta-guard-refusals-name-the-agent-actor-a56263c10f646c87c5ed5ea0
    claim: "Ran cargo test --locked --test stages -- --exact delta_guard_refusals_name_the_agent_actor_and_the_human_authority_note for TC-cli-delta-guard-refusals-name-the-agent-actor: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test stages -- --exact delta_guard_refusals_name_the_agent_actor_and_the_human_authority_note
    revision: 81f8948ca2cd3619e801d4631b6645f9c45d0a14
    environment: local-linux-x86_64
    content: 6225f7182d2bef072cc344f0fcf8b3b87231c1cf36d715a9cd540563e6009f86
    fingerprint: sha256:d52d4da8e173dedf7da02463c55048a130285bf322e256b3d4a14d1043658b47
    case: TC-cli-delta-guard-refusals-name-the-agent-actor
    case_digest: 6341f9cbae9a52d34e19f91d571c9446e32deb9d7958aa7c328169c399133934
    observed: 2026-11-03T14:54:19Z
    duration_seconds: 10
  - id: 2026-11-03-tc-cli-delta-guard-refusals-name-the-agent-actor-886f89
    aliases: OBS-tc-cli-delta-guard-refusals-name-the-agent-actor-81c735f25cb340ccfb103c4a
    claim: "Ran cargo test --locked --test stages -- --exact delta_guard_refusals_name_the_agent_actor_and_the_human_authority_note for TC-cli-delta-guard-refusals-name-the-agent-actor: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact delta_guard_refusals_name_the_agent_actor_and_the_human_authority_note
    revision: 5810ca01ba05fbc5c822920217000049f84f981b
    environment: local-linux-x86_64
    content: 341f203ff08d03c3b980d95ca685ec6f6e8ed304260d17a2c8b7fe8f738a4a9c
    fingerprint: sha256:d65a03a35623a99b68b00f21d34cd11205e35a7d037725affca14fdb4f625961
    case: TC-cli-delta-guard-refusals-name-the-agent-actor
    case_digest: 6341f9cbae9a52d34e19f91d571c9446e32deb9d7958aa7c328169c399133934
    observed: 2026-11-03T14:54:50Z
    duration_seconds: 0
environment: local-linux-x86_64
revision: 81f8948ca2cd3619e801d4631b6645f9c45d0a14
found_by: "Leif 2026-11-03 7:49 AM MT backlog item: contradictory recovery guidance"
related: []
created: 2026-11-03T14:54:26Z
updated: 2026-11-03T14:55:12Z
---
# Approved-delta guard refusals ask the human to approve under their own name

**What happened:** stage moves refused as native-delta-changed or as native-delta-unverifiable for a missing digest say: ask the human for a new scope approval (specsync change approve <id> --actor "<their name>")
Actual: The remedy asks the human to approve and uses their name as the native actor

**Steps:** An approved change whose delta is edited after approval, or whose definition approval has no approved_delta_digests, then chough stage advance

**Expected:** The remedy names specsync change approve <id> --actor "<agent-account>" --note "Human authority: <source>; covered scope: <scope>; limits: <limits>", as lifecycle.md, plan.md and docs/cli.md say

**Why it matters:** Agents following the refusal either stop to ask the human to run a native command already covered by standing authority, or record a human name as the actor

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-03T14:54:26Z · Reported (Leif 2026-11-03 7:49 AM MT backlog item: contradictory recovery guidance); evidence: OBS-tc-cli-delta-guard-refusals-name-the-agent-actor-a56263c10f646c87c5ed5ea0
- 2026-11-03T14:55:12Z · status: open -> verified. Fixed in src/stage.rs: both remedies name the agent-account actor and the human authority note; the failure-first case passed after the fix; evidence: OBS-tc-cli-delta-guard-refusals-name-the-agent-actor-81c735f25cb340ccfb103c4a
