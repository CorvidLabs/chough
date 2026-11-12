---
kind: bug
id: 2026-11-12-at-plan-chough-next-routes-to-specsync-change-approve-with-an
aliases: []
title: At plan, chough next routes to specsync change approve with an actor and never names the scope gate, so an agent following it approves its own scope
status: open
classification: requirement-gap
scope: in-scope
impact: An agent that follows next literally self-approves the scope
criteria: []
requirements: []
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:39Z
updated: 2026-11-12T16:21:39Z
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
