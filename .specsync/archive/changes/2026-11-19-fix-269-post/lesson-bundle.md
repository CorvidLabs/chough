# Lesson bundle: fix-269-post

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 269 post
- **Kind**: BugFix
- **Specs**: records, history
- **Paths**: src/journal.rs, src/sessions.rs, tests/journal.rs, docs/cli.md, .chough/cases/records.objective-at-closeout.cases.toml, changelog.d/2026-11-18-fix-269-post.md
- **Acceptance**: A derived journal post's body opens with the session's current objective, not the Goal's first paragraph, so after a second checkpoint --objective its title and body agree. At closeout, when the story's Goal paragraph is not the current objective, the archived story gets one line 'Objective at closeout: <current objective>' after the Goal paragraph; when it is, no line is added; no title, Goal or other text is rewritten.

## Evidence

- Verification commit: `933aad92dfef6546612f7306dcaf229a6e1bc19d`
- Base commit: `8f3270abaedf56575c3afcc9999c43e6cf6cbd9b`
- Verified by: `specsync check --spec history --spec records`

## From the change's context.md

# Context

A follow-up to fix-269 (#344), from the tester's check. A session's placeholder title and Goal are filled once, from the first objective. After a second `checkpoint --objective`, the closeout's journal post took its title from the current objective but its body from the Goal, which kept the first one, so the two disagreed. Leif's #269 decision said the journal post and the archived story use the current objective.

Leif's answers were relayed by hc t4c on 2026-11-18, within his #269 scope:
- "Fix first: post uses current": a derived post's body describes the current objective, while the session's Goal keeps the filled-once text.
- "Add one line if changed": at closeout, if the objective changed after the Goal was filled, the story gets one added line, `Objective at closeout: <current objective>`. Nothing is rewritten.

The post is REQ-records-011 (`src/journal.rs`), and the story is REQ-history-006 (`src/sessions.rs`).

## From the change's design.md

# Design

- `journal::derived_body` takes the story's front matter `objective` as its first part, where it took the Goal's first paragraph. The weights and cutting are unchanged.
- `sessions::archive` compares the record's Goal first paragraph with `one_line(objective)` before rendering the story. When they differ, it appends one paragraph, `Objective at closeout: <objective>`, to the end of the Goal section. The title, the Goal paragraph and every other byte stay as they were. A Goal that is the objective gets no line.

## From the change's testing.md

# Testing

Each test fails on fix-269 before this change:
- `tests/journal.rs::a_changed_objective_is_in_the_post_and_one_story_line`: a placeholder session given an objective twice, then moved into done. The post's body opens with the second objective and not the first. The story keeps the first as its title and Goal, and adds exactly one `Objective at closeout: <second>` line.
- `tests/journal.rs::the_journal_post_and_story_use_the_objective_set_after_session_new` (from fix-269): a single objective change adds no closeout line.
- `tests/journal.rs::without_a_title_or_summary_the_post_derives_both_from_the_story`, updated: the body opens with the objective, and its other parts are unchanged.

Case `TC-records-a-changed-objective-is-in-the-post-and-one-story-line` binds the first.

## Where these lessons go

- `specs/records/context.md`
- `specs/history/context.md`
