---
change: show-open-questions-in-next-and-status-report-an-archived-session-as-archived-and-warn-when-an-inbox-observation-has-no
artifact: testing
---

# Testing

- tests/next.rs `open_questions_appear_in_next_and_status`: two questions from
  `checkpoint --question` appear in `next` text and JSON and in `status`; 12 questions
  show 10 with total 12; `--resolve-question` removes one from both.
- tests/sessions.rs `a_folded_inbox_question_appears_in_next`: the issue's repro.
- tests/sessions.rs `session_list_reports_an_archived_session_as_archived`.
- tests/sessions.rs `an_unrecorded_inbox_observation_is_added_with_a_warning`:
  `inbox add` with an unknown ID succeeds, warns on stderr naming it, and `inbox list`
  marks it `recorded: false`; a recorded one gets no warning and `recorded: true`.
- Each new test fails at 290cce3; `fledge lanes run verify` passes on Linux.
