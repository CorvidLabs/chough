---
change: fix-269-post
artifact: context
---

# Context

A follow-up to fix-269 (#344), from the tester's check. A session's placeholder title and Goal are filled once, from the first objective. After a second `checkpoint --objective`, the closeout's journal post took its title from the current objective but its body from the Goal, which kept the first one, so the two disagreed. Leif's #269 decision said the journal post and the archived story use the current objective.

Leif's answers were relayed by hc t4c on 2026-11-18, within his #269 scope:
- "Fix first: post uses current": a derived post's body describes the current objective, while the session's Goal keeps the filled-once text.
- "Add one line if changed": at closeout, if the objective changed after the Goal was filled, the story gets one added line, `Objective at closeout: <current objective>`. Nothing is rewritten.

The post is REQ-records-011 (`src/journal.rs`), and the story is REQ-history-006 (`src/sessions.rs`).
