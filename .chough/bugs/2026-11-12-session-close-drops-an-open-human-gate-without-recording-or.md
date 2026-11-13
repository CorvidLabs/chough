---
kind: bug
id: 2026-11-12-session-close-drops-an-open-human-gate-without-recording-or
aliases: []
title: session close drops an open human gate without recording or reporting it
status: verified
classification: product-defect
scope: in-scope
impact: A human's pending decision disappears silently (t38m F2)
criteria: []
requirements: [REQ-history-006]
cases: [TC-history-session-close-records-withdrawn-gates]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-session-close-records-withdrawn-gates
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-session-close-records-withdrawn-gates-9c73cd
receipts: []
environment: local-macos-arm64
revision: 95ba89c6bd2551a68f061cf7b821ffef07797de8
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report, F2 (relayed by t370))
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T22:06:08Z
updated: 2026-11-12T22:13:51Z
---
# session close drops an open human gate without recording or reporting it

**What happened:** chough session close <id> --reason archives a session that holds an open awaiting-human gate: exit 0, no gate-closed entry, the output does not name the gate, the story still lists it as awaiting, and gate list then shows no open gates
Actual: The gate vanishes with no entry and no report

**Steps:** Trigger: Open a human gate in a session, then close that session with session close
1. chough checkpoint --awaiting-human <gate>
2. chough session close <id> --reason <text>
3. Read the output, the story and chough gate list

**Expected:** The gate withdrawn as --clear-awaiting-human does, with one gate-closed entry and no decision, named under withdrawn_gates, and no gate awaiting in the story

**Why it matters:** A human's pending decision disappears silently (t38m F2)

**Fix:** d2bfc0cb: session close withdraws each open human gate as --clear-awaiting-human does, one gate-closed entry and no decision, and names them under withdrawn_gates (REQ-history-006)

## Log

- 2026-11-12T22:06:08Z · Reported (t38m's third fresh-eyes round report, F2 (relayed by t370)); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-session-close-records-withdrawn-gates · by claude-code (hc t382)
- 2026-11-12T22:06:08Z · Added despite similar reports: #181 kept a finished session's open gate open at session new and use; this is session close, which archives the session and drops the gate unrecorded · by claude-code (hc t382)
- 2026-11-12T22:13:51Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-session-close-records-withdrawn-gates-9c73cd; fix: d2bfc0cb: session close withdraws each open human gate as --clear-awaiting-human does, one gate-closed entry and no decision, and names them under withdrawn_gates (REQ-history-006). Red at 95ba89c6, green at d2bfc0cb · by claude-code (hc t382)
