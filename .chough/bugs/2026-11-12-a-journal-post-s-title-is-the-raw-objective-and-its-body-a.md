---
kind: bug
id: 2026-11-12-a-journal-post-s-title-is-the-raw-objective-and-its-body-a
aliases: []
title: A journal post's title is the raw objective and its body a stale first bullet, and the stories archived before the journal have no post
status: open
classification: requirement-gap
scope: blocking
impact: The journal, the timeline of archived changes, reads badly and covers one change of 69
criteria: [KNOW-33]
requirements: [REQ-records-011]
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-journal-post-uses-the-title-and-summary
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-journal-post-derives-a-title-and-body
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-journal-add-record-backfills-a-story
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-journal-and-feed
receipts: []
environment: local-macos-arm64
revision: 423169dee4924d188777c611349ce0e0318820a8
found_by: claude-code (hc t382) (Leif, reviewing knowledge/journal in the coordinator chat (hc t370), 2026-11-12T15:35Z, relayed by t370; red at 423169de by t382)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:04:18Z
updated: 2026-11-12T16:04:18Z
---
# A journal post's title is the raw objective and its body a stale first bullet, and the stories archived before the journal have no post

**What happened:** This repository's 0.6.0 post at the move into done has the three-line objective as its title, cut mid-word in the index, and the body 'Plan step 3 (sessions) ... 561 passed', the first Where it stands bullet; the 68 earlier archived stories have no post
Actual: REQ-records-011 makes the title the objective and the body the first Outcome or Where it stands paragraph, and journal add cannot date a post by its story

**Steps:** The move into done of a session with a long objective and an old first Where it stands entry; a project whose stories were archived before the journal began

**Expected:** A short human title and a real summary (what changed and why, what was verified with the final numbers, what is next), which the agent sets before the move into done, with a derived title and body as the fallback; a way to backfill an archived story's post

**Why it matters:** The journal, the timeline of archived changes, reads badly and covers one change of 69

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:04:18Z · Reported (Leif, reviewing knowledge/journal in the coordinator chat (hc t370), 2026-11-12T15:35Z, relayed by t370; red at 423169de by t382); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-journal-post-uses-the-title-and-summary, 2026-11-12-release-0-6-0/2026-11-12-tc-records-journal-post-derives-a-title-and-body, 2026-11-12-release-0-6-0/2026-11-12-tc-records-journal-add-record-backfills-a-story, 2026-11-12-release-0-6-0/2026-11-12-tc-records-journal-and-feed · by claude-code (hc t382)
