# Lesson bundle: fix-269

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 269
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/session_md.rs, src/markdown.rs, tests/records.rs, tests/journal.rs, docs/cli.md, .chough/cases/records.default-goal.cases.toml, changelog.d/2026-11-18-fix-269.md
- **Acceptance**: A session started with chough session new and no objective holds the placeholder as its '# ...' title and its Goal; the first chough checkpoint --objective replaces each that still holds the untouched placeholder with the objective, so the archived story and its journal post carry the objective and never the placeholder. A later objective change rewrites neither, and a Goal or title a person wrote is kept byte for byte.

## Evidence

- Verification commit: `e0e3658d4903746e7f112dcb7a7673ef53faa92f`
- Base commit: `387d8314aa6dbd9836c7935eaa34530b59fd8c82`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

GitHub #269, found by a fresh-eyes UX run on 0.6.0. `chough session new` without an objective writes the placeholder "Describe the current objective before beginning work." as the story's `# <title>` and its Goal. `chough checkpoint --objective` then updates the front matter only. The session record, its archived story, and the journal post built from the Goal all keep the stale placeholder, text that was never the objective.

Leif's decision (2026-11-18, planning interview, recorded on #269 and relayed by hc t4c as the scope approval) is to fill the placeholder once. When the session's Goal section still holds the untouched default text, `checkpoint --objective` replaces it. A Goal a person has written is never rewritten. The journal post and the archived story use the current objective. This adds one exception to REQ-records-004.

Asked whether the `# <title>` line, which holds the same placeholder, is filled too, he answered "Yes, fill both" (relayed by hc t4c, 2026-11-18).

## From the change's design.md

# Design

- `Record::fill_placeholder(heading, untouched, with)` in `src/markdown.rs` makes two replacements, each only when the text is untouched:
  - the `## <heading>` section's text becomes `with` when, trimmed, it is exactly `untouched`, with the blank lines around it kept;
  - the body's first line becomes `# <with>` when it is exactly `# <untouched>`.
- `session_md::render_with`, through which every save of a session passes, calls it with the Goal, the placeholder and `one_line(objective)` whenever the objective is not the placeholder.
  - The first save after `checkpoint --objective` fills both.
  - From then on neither matches, so nothing is rewritten again, and a person's text never matches.
- The journal post takes its Goal paragraph from the record at the move into done, so it carries the filled objective.
- The placeholder is named once in `src/session_md.rs`, matching `sessions::DEFAULT_OBJECTIVE`. The tests use the literal, so a change to the default fails them.

## From the change's testing.md

# Testing

Each test fails on main:
- `tests/records.rs::a_default_goal_is_filled_once_by_the_objective`: `session new` without an objective, then `checkpoint --objective`. The title and Goal now carry the objective, and a later objective change rewrites neither.
- `tests/journal.rs::the_journal_post_and_story_use_the_objective_set_after_session_new`: a session holding the placeholder title and Goal is given an objective and moved into done. The post and the story carry the objective and never the placeholder.

`tests/records.rs::a_goal_a_person_wrote_is_never_filled` checks that a person's Goal is kept byte for byte while the placeholder title is filled.

Cases `TC-records-the-default-goal-is-filled-once` and `TC-records-the-journal-post-uses-the-filled-goal` each bind one test.

## Where these lessons go

- `specs/records/context.md`
