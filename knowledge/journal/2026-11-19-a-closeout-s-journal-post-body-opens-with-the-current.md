---
id: 2026-11-19-a-closeout-s-journal-post-body-opens-with-the-current
title: A closeout's journal post body opens with the current objective
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T00:08:38Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-269-post.md
  - .specsync/archive/changes/2026-11-19-fix-269-post/
record: .chough/archive/sessions/2026-11-18-fix-269-post.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# A closeout's journal post body opens with the current objective

A closeout's journal post body opens with the current objective, and the archived story gets one line 'Objective at closeout: <objective>' when the objective changed after the Goal was filled; nothing is rewritten (REQ-records-011, REQ-history-006, #344 review)

Why: 0xLeif decided "Scope approval of fix-269-post": "Fix first: post uses current … Add one line if changed" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his answers on the tester's #344 finding, within his #269 scope: the journal post's body describes the current objective while the session's Goal keeps the filled-once text, as a small follow-up change on #344 declaring src/journal.rs and REQ-records-011; and at closeout, if the…

Verified: Ran cargo test --locked --test journal -- --exact a_changed_objective_is_in_the_post_and_one_story_line for TC-records-a-changed-objective-is-in-the-post-and-one-story-line: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-269-post.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-269-post/)
