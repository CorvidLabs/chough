---
change: fix-269
artifact: testing
---

# Testing

Each test fails on main:
- `tests/records.rs::a_default_goal_is_filled_once_by_the_objective`: `session new` without an objective, then `checkpoint --objective`. The title and Goal now carry the objective, and a later objective change rewrites neither.
- `tests/journal.rs::the_journal_post_and_story_use_the_objective_set_after_session_new`: a session holding the placeholder title and Goal is given an objective and moved into done. The post and the story carry the objective and never the placeholder.

`tests/records.rs::a_goal_a_person_wrote_is_never_filled` checks that a person's Goal is kept byte for byte while the placeholder title is filled.

Cases `TC-records-the-default-goal-is-filled-once` and `TC-records-the-journal-post-uses-the-filled-goal` each bind one test.
