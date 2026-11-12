---
kind: bug
id: 2026-11-12-chough-context-in-markdown-prints-the-session-its-timeline-and
aliases: []
title: chough context in Markdown prints the session, its Timeline and its open bugs as JSON
status: verified
classification: product-defect
scope: in-scope
impact: The agent-facing summary reads as a data dump (t38m F10d)
criteria: []
requirements: [REQ-cli-002]
cases:
  - TC-cli-context-gives-the-session-as-markdown
  - TC-cli-context-gives-the-timeline-and-bugs-as-markdown
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-gives-the-session-as-markdown
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-gives-the-timeline-and-bugs-as-markdown
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-gives-the-session-as-markdown-ae263f
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-gives-the-timeline-and-bugs-as-markdown-4a8787
receipts: []
environment: local-macos-arm64
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:05:05Z
updated: 2026-11-12T23:06:39Z
---
# chough context in Markdown prints the session, its Timeline and its open bugs as JSON

**What happened:** chough context, Markdown by default, prints the Session section, the recent Timeline and the open-bug backlog as raw JSON blocks
Actual: Three JSON blocks in the Markdown view

**Steps:** Trigger: Run chough context in a project with a session
1. chough context

**Expected:** Each section as Markdown, the JSON form kept under --json

**Why it matters:** The agent-facing summary reads as a data dump (t38m F10d)

**Fix:** bcac715b and b322c651 (t38b's 68dd590b and e2a12ed7): context's Markdown gives the session, its recent Timeline and the open bugs as Markdown; --json is unchanged

## Log

- 2026-11-12T23:05:05Z · Reported (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-gives-the-session-as-markdown, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-gives-the-timeline-and-bugs-as-markdown · by claude-code (hc t382)
- 2026-11-12T23:06:28Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-gives-the-session-as-markdown-ae263f, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-context-gives-the-timeline-and-bugs-as-markdown-4a8787; fix: t38b's 68dd590b and e2a12ed7: context's Markdown gives the session, its recent Timeline and the open bugs as Markdown; --json is unchanged. Red at 4e245263, green at 421a7034 · by claude-code (hc t382)
- 2026-11-12T23:06:39Z · fix: bcac715b and b322c651 (t38b's 68dd590b and e2a12ed7): context's Markdown gives the session, its recent Timeline and the open bugs as Markdown; --json is unchanged. Fix line names the commits on leif/human-first-records · by claude-code (hc t382)
