---
change: fix-269-post
artifact: testing
---

# Testing

Each test fails on fix-269 before this change:
- `tests/journal.rs::a_changed_objective_is_in_the_post_and_one_story_line`: a placeholder session given an objective twice, then moved into done. The post's body opens with the second objective and not the first. The story keeps the first as its title and Goal, and adds exactly one `Objective at closeout: <second>` line.
- `tests/journal.rs::the_journal_post_and_story_use_the_objective_set_after_session_new` (from fix-269): a single objective change adds no closeout line.
- `tests/journal.rs::without_a_title_or_summary_the_post_derives_both_from_the_story`, updated: the body opens with the objective, and its other parts are unchanged.

Case `TC-records-a-changed-objective-is-in-the-post-and-one-story-line` binds the first.
