---
kind: bug
id: 2026-11-12-an-archived-story-opens-with-its-receipts-and-never-names-the
aliases: []
title: An archived story opens with its receipts and never names the bug it found
status: open
classification: product-defect
scope: in-scope
impact: A person reading the record meets YAML before the story and misses its bug (t38m F10a, F10b)
criteria: []
requirements: [REQ-history-006]
cases: [TC-history-story-receipts-last-and-bugs-named]
evidence: [2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-last-and-bugs-named]
receipts: []
environment: local-macos-arm64
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:05:20Z
updated: 2026-11-12T23:05:20Z
---
# An archived story opens with its receipts and never names the bug it found

**What happened:** The story Chough writes at done or session close opens with about 100 lines of receipts YAML in its front matter before '# <objective>', and its body never names the bug the session found and fixed; only the front matter lists it
Actual: Receipts first; no bug named in the body

**Steps:** Trigger: Archive a session that ran cases and found a bug, then read its story
1. Find and fix a bug in a session, with failing then passing runs
2. Move the session into done
3. Read .chough/archive/sessions/<id>.md

**Expected:** The story first, a Bugs section naming each bug with its status, and the receipts last

**Why it matters:** A person reading the record meets YAML before the story and misses its bug (t38m F10a, F10b)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T23:05:20Z · Reported (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-last-and-bugs-named · by claude-code (hc t382)
- 2026-11-12T23:05:20Z · Added despite similar reports: The verified story bugs fixed its verified list and its stale stage_status and next; this one is the story's layout: receipts before the body, and no bug named in it · by claude-code (hc t382)
