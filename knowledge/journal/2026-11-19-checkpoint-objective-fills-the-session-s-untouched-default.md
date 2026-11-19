---
id: 2026-11-19-checkpoint-objective-fills-the-session-s-untouched-default
title: checkpoint --objective fills the session's untouched default Goal once
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T00:07:29Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-269.md
  - .specsync/archive/changes/2026-11-19-fix-269/
record: .chough/archive/sessions/2026-11-18-fix-269.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# checkpoint --objective fills the session's untouched default Goal once

checkpoint --objective fills the session's untouched default Goal once, so the story and journal post use the current objective; a Goal a person wrote is never rewritten (REQ-records-004, #269)

Why: 0xLeif decided "Scope approval of fix-269": "fill the placeholder once. When the session's Goal section still holds the untouched default text, checkpoint --objective replaces it. A Goal a person has written is never rewritten. The journal post and the archived story use the current objective. This adds that one exception to REQ-records-004.

Verified: Ran cargo test --locked --test journal -- --exact the_journal_post_and_story_use_the_objective_set_after_session_new for TC-records-the-journal-post-uses-the-filled-goal: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-269.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-269/)
