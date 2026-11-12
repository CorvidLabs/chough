---
kind: bug
id: 2026-11-11-next-does-not-guide-a-fresh-agent-through-the-stages-76
aliases: [BUG-next-does-not-guide-a-fresh-agent-through-the-stages-76-666a81db074a4b84a2a93788]
title: next does not guide a fresh agent through the stages (#76)
status: verified
classification: product-defect
scope: in-scope
impact: "A fresh agent cannot get from intent to archive on its own (GitHub #76, items 1, 3 and 4)"
criteria: [LIFE-1, LIFE-3, FLOW-13]
requirements: [REQ-cli-021]
cases: [TC-cli-76-next-guides-a-fresh-agent]
evidence:
  - OBS-tc-cli-76-next-guides-a-fresh-agent-a9e1c71dbf2104d904c41589
  - OBS-tc-cli-76-next-guides-a-fresh-agent-c0aa2f39f9ffa0401eaeab92
receipts:
  - id: 2026-11-11-tc-cli-76-next-guides-a-fresh-agent
    aliases: OBS-tc-cli-76-next-guides-a-fresh-agent-a9e1c71dbf2104d904c41589
    claim: "Ran cargo test --locked --test next -- --exact next_guides_a_fresh_agent_from_init_to_accept for TC-cli-76-next-guides-a-fresh-agent: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test next -- --exact next_guides_a_fresh_agent_from_init_to_accept
    revision: dbd3d23b78ad4f001968530f6cd336105f4bbf58
    environment: local-macos-arm64
    content: 4e698547b6c905aa2792cff66eccfd248ecdb5b9d891fb6997a50f038a38e75a
    fingerprint: sha256:d17af72ca922ea653109e47f7b21681bd4a8e1ad036ecff8d47fa678daa4052c
    case: TC-cli-76-next-guides-a-fresh-agent
    case_digest: 4f9cb078a8262a1c9fcc1ec23d78a5cbaaf2ee6440053e573b6a2022bb7113cb
    observed: 2026-11-11T19:39:10Z
    duration_seconds: 2
  - id: 2026-11-11-tc-cli-76-next-guides-a-fresh-agent-d47be1
    aliases: OBS-tc-cli-76-next-guides-a-fresh-agent-c0aa2f39f9ffa0401eaeab92
    claim: "Ran cargo test --locked --test next -- --exact next_guides_a_fresh_agent_from_init_to_accept for TC-cli-76-next-guides-a-fresh-agent: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_guides_a_fresh_agent_from_init_to_accept
    revision: f9c0ae674c89f3a4c4a6c5b9d29057f22f7b9bf0
    environment: local-macos-arm64
    content: 4adc6e184b41d042b73284db8f143439ad1bd2bdc6fc389e3a5266068b430ba2
    fingerprint: sha256:41f1ee2be24774af6c0dd31fc4d2756575250923b5d5a83d58a396b48fb96e7d
    case: TC-cli-76-next-guides-a-fresh-agent
    case_digest: 4f9cb078a8262a1c9fcc1ec23d78a5cbaaf2ee6440053e573b6a2022bb7113cb
    observed: 2026-11-11T19:42:53Z
    duration_seconds: 1
environment: local-macos-arm64
revision: d507cf28c2e8a208aff21a4e5f808fea06101d90
found_by: "GitHub issue #76; red on 0.5.0 at leif/0.6.0-76-0023-red dbd3d23b"
related: []
created: 2026-11-11T19:39:30Z
updated: 2026-11-11T19:43:31Z
---
# next does not guide a fresh agent through the stages (#76)

**What happened:** right after init the next action says only 'Continue intent'; at test and at accept while verifying it names no case selection, no bug for a failed run and no human review
Actual: Continue intent: Read scoped intent and native status, then choose the next action within the authorized scope.

**Steps:** chough init, then chough next; advance to test; reach accept with the native change verifying
1. chough init --objective <outcome> --intent-scope <family> --module <module>
2. chough next --json

**Expected:** after init: hi check, then chough context; at test: chough cases select, chough cases run and chough bug add; at accept while verifying: the human reviews through --close-gate with --verdict, or the review gate is opened, never an agent review

**Why it matters:** A fresh agent cannot get from intent to archive on its own (GitHub #76, items 1, 3 and 4)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T19:39:30Z · Reported (GitHub issue #76; red on 0.5.0 at leif/0.6.0-76-0023-red dbd3d23b); evidence: OBS-tc-cli-76-next-guides-a-fresh-agent-a9e1c71dbf2104d904c41589
- 2026-11-11T19:39:30Z · Added despite similar reports: The near matches are fixture and history bugs, not next's guidance
- 2026-11-11T19:43:31Z · status: open -> verified. Fixed in plan step 12 (PR #238, f9c0ae67); the case passed after its 0.5.0 failure.; evidence: OBS-tc-cli-76-next-guides-a-fresh-agent-c0aa2f39f9ffa0401eaeab92
