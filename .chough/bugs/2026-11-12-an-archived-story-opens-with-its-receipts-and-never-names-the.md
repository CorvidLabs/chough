---
kind: bug
id: 2026-11-12-an-archived-story-opens-with-its-receipts-and-never-names-the
aliases: []
title: An archived story opens with its receipts and never names the bug it found
status: verified
classification: product-defect
scope: in-scope
impact: A person reading the record meets YAML before the story and misses its bug (t38m F10a, F10b)
criteria: []
requirements: [REQ-history-006]
cases: [TC-history-story-receipts-last-and-bugs-named]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-last-and-bugs-named
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-last-and-bugs-named-9a5af0
receipts: []
environment: local-macos-arm64
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:05:20Z
updated: 2026-11-12T23:06:39Z
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

**Fix:** 5b2b5a58 (t38b's a43a34fb): the story's front matter holds no receipts; a Bugs section names each bug with its status; the receipts follow the closing line under ## Receipts; older stories still read (REQ-history-006)

## Log

- 2026-11-12T23:05:20Z · Reported (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-last-and-bugs-named · by claude-code (hc t382)
- 2026-11-12T23:05:20Z · Added despite similar reports: The verified story bugs fixed its verified list and its stale stage_status and next; this one is the story's layout: receipts before the body, and no bug named in it · by claude-code (hc t382)
- 2026-11-12T23:06:28Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-last-and-bugs-named-9a5af0; fix: t38b's a43a34fb: the story's front matter holds no receipts; a Bugs section names each bug with its status; the receipts follow the closing line under ## Receipts; older stories still read (REQ-history-006). Red at 4e245263, green at 421a7034 · by claude-code (hc t382)
- 2026-11-12T23:06:39Z · fix: 5b2b5a58 (t38b's a43a34fb): the story's front matter holds no receipts; a Bugs section names each bug with its status; the receipts follow the closing line under ## Receipts; older stories still read (REQ-history-006). Fix line names the commits on leif/human-first-records · by claude-code (hc t382)
