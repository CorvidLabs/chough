---
kind: bug
id: 2026-11-12-at-plan-chough-next-routes-to-specsync-change-approve-with-an
aliases: []
title: At plan, chough next routes to specsync change approve with an actor and never names the scope gate, so an agent following it approves its own scope
status: verified
classification: requirement-gap
scope: blocking
impact: An agent that follows next literally self-approves the scope
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-plan-names-the-scope-gate
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-plan-names-the-scope-gate-a75cc8
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:39Z
updated: 2026-11-12T17:20:18Z
---
# At plan, chough next routes to specsync change approve with an actor and never names the scope gate, so an agent following it approves its own scope

**What happened:** chough next at plan with a complete draft says Run: specsync change status <id> and the native action run specsync change approve <id> --actor <name>
Actual: The raw native approve is the only route given

**Steps:** chough next --environment local at plan with a complete draft

**Expected:** It names the scope gate (chough checkpoint --awaiting-human, then --close-gate with the human's decision), as accept already names the review gate, and no output at plan names specsync change approve with an agent as actor

**Why it matters:** An agent that follows next literally self-approves the scope

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:39Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
- 2026-11-12T16:48:39Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-plan-names-the-scope-gate. Reproduced: red at da98f439, TC-cli-next-at-plan-names-the-scope-gate failed; at plan with a draft whose SpecSync next action is specsync change approve --actor, next said Gate not open (refused:native-draft) and named no scope gate. In scope (Leif, 16:26Z, relayed by t370) · by claude-code (hc t382)
- 2026-11-12T17:20:18Z · status: investigating -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-next-at-plan-names-the-scope-gate-a75cc8. Green at 56c3f4d3: TC-cli-next-at-plan-names-the-scope-gate passed; next at plan opens the scope gate and names --close-gate, and next, its text view and status show SpecSync's approve as the human's scope approval, never as a command · by claude-code (hc t382)
