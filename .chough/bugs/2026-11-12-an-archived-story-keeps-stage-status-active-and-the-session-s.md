---
kind: bug
id: 2026-11-12-an-archived-story-keeps-stage-status-active-and-the-session-s
aliases: []
title: An archived story keeps stage_status active and the session's stale next
status: verified
classification: product-defect
scope: in-scope
impact: An archived story reads as live work with a next step (t38h 6, t38k F8)
criteria: []
requirements: [REQ-history-006]
cases: [TC-history-archived-story-says-archived]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-archived-story-says-archived
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-archived-story-says-archived-64087b
receipts: []
environment: local-macos-arm64
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
found_by: claude-code (hc t382) (Fresh-eyes round 2 (t38k, and t38h for Kimi 6; relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T21:06:23Z
updated: 2026-11-12T21:07:59Z
---
# An archived story keeps stage_status active and the session's stale next

**What happened:** The story Chough writes at the move into done or session close keeps the session's front matter stage_status: active and its next action
Actual: stage_status: active and the pre-accept next

**Steps:** Trigger: Archive a session and read the story's front matter
1. Move a session into done, or close it
2. Read .chough/archive/sessions/<id>.md

**Expected:** stage_status: archived and no next

**Why it matters:** An archived story reads as live work with a next step (t38h 6, t38k F8)

**Fix:** d57b20f3 (t38b's 5a344e46): an archived story gives stage_status: archived and no next (REQ-history-006)

## Log

- 2026-11-12T21:06:23Z · Reported (Fresh-eyes round 2 (t38k, and t38h for Kimi 6; relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-archived-story-says-archived · by claude-code (hc t382)
- 2026-11-12T21:07:59Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-archived-story-says-archived-64087b; fix: d57b20f3 (t38b's 5a344e46): an archived story gives stage_status: archived and no next (REQ-history-006). Red at 804bdb50, green at 91c3824f · by claude-code (hc t382)
