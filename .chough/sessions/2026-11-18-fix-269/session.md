---
kind: session
id: 2026-11-18-fix-269
aliases: [fix-269]
objective: "checkpoint --objective fills the session's untouched default Goal once, so the story and journal post use the current objective; a Goal a person wrote is never rewritten (REQ-records-004, #269)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-269
pr: ""
criteria: [DRIVE-24]
requirements: [REQ-records-004]
cases:
  - TC-records-the-default-goal-is-filled-once
  - TC-records-the-journal-post-uses-the-filled-goal
design: []
files: []
observations:
  - 2026-11-18-tc-records-the-default-goal-is-filled-once
  - 2026-11-18-tc-records-the-journal-post-uses-the-filled-goal
  - 2026-11-18-tc-records-the-default-goal-is-filled-once-b084da
  - 2026-11-18-tc-records-the-journal-post-uses-the-filled-goal-6cc4d2
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-269
    holds: accept
    drives: review
    opened: 2026-11-18T20:07:07Z
run_consents: []
next: Continue accept
---
# checkpoint --objective fills the session's untouched default Goal once, so the story and journal post use the current objective; a Goal a person wrote is never rewritten (REQ-records-004, #269)

## Goal

checkpoint --objective fills the session's untouched default Goal once, so the story and journal post use the current objective; a Goal a person wrote is never rewritten (REQ-records-004, #269)

## Where it stands

- Failing tests first: a_default_goal_is_filled_once_by_the_objective, a_goal_a_person_wrote_is_never_filled and the_journal_post_and_story_use_the_objective_set_after_session_new failed on main; Record::fill_placeholder fills the untouched placeholder title and Goal once from the objective on save, so the story and journal post carry it; records (10), journal (24), sessions and next pass

## Decisions and why

- 0xLeif decided "Scope approval of fix-269": "fill the placeholder once. When the session's Goal section still holds the untouched default text, checkpoint --objective replaces it. A Goal a person has written is never rewritten. The journal post and the archived story use the current objective. This adds that one exception to REQ-records-004. … Yes, fill both" (Leif's decision comment on CorvidLabs/chough#269 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval; then 'Yes, fill both', relayed by hc t4c on 2026-11-18, to fill the story's '# ...' title as well as the Goal when it still holds the untouched default, under the same never-rewrite-a-person's-text rule); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-269 as 0xLeif

## What was verified

- [Ran cargo test --locked --test records -- --exact a_default_goal_is_filled_once_by_the_objective for TC-records-the-default-goal-is-filled-once: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-the-default-goal-is-filled-once.md)
- [Ran cargo test --locked --test journal -- --exact the_journal_post_and_story_use_the_objective_set_after_session_new for TC-records-the-journal-post-uses-the-filled-goal: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-the-journal-post-uses-the-filled-goal.md)
- [Ran cargo test --locked --test records -- --exact a_default_goal_is_filled_once_by_the_objective for TC-records-the-default-goal-is-filled-once: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-the-default-goal-is-filled-once-b084da.md)
- [Ran cargo test --locked --test journal -- --exact the_journal_post_and_story_use_the_objective_set_after_session_new for TC-records-the-journal-post-uses-the-filled-goal: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-the-journal-post-uses-the-filled-goal-6cc4d2.md)

## Open threads

## Timeline

- 2026-11-18T20:00:59Z · start · intent → intent · checkpoint --objective fills the session's untouched default Goal once, so the story and journal post use the current objective; a Goal a person wrote is never rewritten (REQ-records-004, #269) ^t-900d99
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:00:59Z · advance · intent → design ·  ^t-63677b
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:00:59Z · skip · design → plan · A record body fill; nothing visual or interactive changes ^t-e419da
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:02:36Z · gate-opened · plan → plan · Scope approval of fix-269 ^t-a87c42
  - native_change: fix-269
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:02:36Z · gate-closed · plan → plan · Scope approval of fix-269 ^t-99d7a6
  - native_change: fix-269
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - relayed_by:
    - decided_by: 0xLeif
    - words: fill the placeholder once. When the session's Goal section still holds the untouched default text, checkpoint --objective replaces it. A Goal a person has written is never rewritten. The journal post and the archived story use the current objective. This adds that one exception to REQ-records-004. … Yes, fill both
    - where: Leif's decision comment on CorvidLabs/chough#269 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval; then 'Yes, fill both', relayed by hc t4c on 2026-11-18, to fill the story's '# ...' title as well as the Goal when it still holds the untouched default, under the same never-rewrite-a-person's-text rule
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-269
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:07:01Z · advance · plan → build · native:approved ^t-816b68
  - native_change: fix-269
  - gate: native:approved
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:07:01Z · advance · build → test · native:approved ^t-43638e
  - native_change: fix-269
  - gate: native:approved
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:07:07Z · advance · test → accept · native:verifying ^t-b5a4d8
  - native_change: fix-269
  - gate: native:verifying
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:07:07Z · gate-opened · accept → accept · Implementation review of fix-269 ^t-f93e4c
  - native_change: fix-269
  - revision: 387d8314aa6dbd9836c7935eaa34530b59fd8c82
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
