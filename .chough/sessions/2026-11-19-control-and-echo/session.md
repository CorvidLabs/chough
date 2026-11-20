---
kind: session
id: 2026-11-19-control-and-echo
aliases: [control-and-echo]
objective: "Control and echo batch: control characters refused at write and escaped on print (#366), --pr takes a number or an http(s) URL and is shown in next, status, context and the checkpoint view (#353), the remaining sites echo no value (part of #367), and a file URL holds the user name"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: control-and-echo
pr: "418"
criteria: [LIFE-26, DRIVE-26, LIFE-3]
requirements: [REQ-records-006, REQ-cli-007, REQ-history-015]
cases: []
design: []
files: []
observations:
  - 2026-11-20-tc-cli-the-pull-request-shows-in-every-view
  - 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form
  - 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form
  - 2026-11-20-tc-history-correct-names-what-a-given-entry-holds
  - 2026-11-20-tc-records-a-control-character-is-refused-at-write
  - 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one
  - 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw
  - 2026-11-20-tc-records-generated-files-hold-text-as-printed
  - 2026-11-20-tc-records-reserved-words-are-written-quoted
  - 2026-11-20-tc-records-flags-stay-booleans
  - 2026-11-20-tc-records-no-command-echoes-a-given-value
  - 2026-11-20-tc-records-a-file-url-is-checked-like-its-path
  - 2026-11-20-tc-reporting-timing-prints-stored-text-escaped
  - 2026-11-20-tc-records-tabs-in-values-read-back
  - 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-c10aa7
  - 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-ac089a
  - 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-27ad78
  - 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-7f3465
  - 2026-11-20-tc-records-a-control-character-is-refused-at-write-5ac074
  - 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b3814b
  - 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-55f86d
  - 2026-11-20-tc-records-generated-files-hold-text-as-printed-0897b5
  - 2026-11-20-tc-records-reserved-words-are-written-quoted-c69322
  - 2026-11-20-tc-records-flags-stay-booleans-5184c5
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-073e44
  - 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-72c270
  - 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-551f25
  - 2026-11-20-tc-records-tabs-in-values-read-back-867742
  - 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-18092a
  - 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-79cb99
  - 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-fa2c8f
  - 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-a77687
  - 2026-11-20-tc-records-a-control-character-is-refused-at-write-5847df
  - 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-c7d3e9
  - 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-f7278f
  - 2026-11-20-tc-records-generated-files-hold-text-as-printed-35d318
  - 2026-11-20-tc-records-reserved-words-are-written-quoted-fd1c42
  - 2026-11-20-tc-records-flags-stay-booleans-0fd3cd
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-73f48a
  - 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-68b196
  - 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-622b00
  - 2026-11-20-tc-records-tabs-in-values-read-back-64ca6f
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Re-approval of control-and-echo with tests/spec_split.rs added\" closed (approved); run `chough stage advance` to move on."
---
# Control and echo batch: control characters refused at write and escaped on print (#366), --pr takes a number or an http(s) URL and is shown in next, status, context and the checkpoint view (#353), the remaining sites echo no value (part of #367), and a file URL holds the user name

## Goal

Control and echo batch: control characters refused at write and escaped on print (#366), --pr takes a number or an http(s) URL and is shown in next, status, context and the checkpoint view (#353), the remaining sites echo no value (part of #367), and a file URL holds the user name

## Where it stands

## Decisions and why

- 0xLeif decided "Standing OK (d): rebase control-and-echo's REQ-cli-007 onto main after #413": "yes to both" (Standing OK (d), Leif's Claude Code chat message relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:02Z; #413 changed REQ-cli-007 on main, so the delta was rebased onto main's text at 2968f061 with exactly the approved words (six body edits, one --pr criterion replaced and two added) and #413's (one sentence, one criterion) kept, the criteria placed as main's three unchanged, L2's three, then #413's; fable-lead verified by script on 2026-11-20 that nothing else changed); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Approve the scope of control-and-echo": "Approve the recommended set" (Claude Code chat answer, relayed by his planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-19 about 14:12Z, on L2 control-and-echo at 1068f67 with the seven recommended choices; recorded at the batch start on that text, with REQ-cli-007 rebased onto main after #413 under standing OK (d) (his 'yes to both', about 15:02Z, relayed by t4c; fable-lead's script check of 2026-11-20)); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve control-and-echo as 0xLeif
- 0xLeif decided "Add four files to control-and-echo's scope by one send-back, run by the fixer": "Ok" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 03:00Z, answering t4c's six-item list, put to him with 'all yes' as the way to accept all; item 2: add src/evidence.rs, src/gates.rs, tests/front_matter_writes.rs and its case file .chough/cases/records.test-stage.cases.toml to L2's scope by one send-back the fixer runs, no sentence change); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of control-and-echo with four more paths and REQ-reporting-001's example": "Ok" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 03:00Z, item 2 of t4c's six-item list, on this text: src/evidence.rs, src/gates.rs, tests/front_matter_writes.rs and .chough/cases/records.test-stage.cases.toml join the declared paths, with no sentence change; and REQ-reporting-001's example and criterion name a gate's name in place of a stage move's reason, under standing OK 6(a), a wording fix that only makes an approved sentence match tested code, relayed via t4c and checked by fable-lead's script; the earlier approvals stand for the rest: 'Approve the recommended set' (2026-11-19 about 14:12Z) and standing OK (d) for REQ-cli-007's rebase after #413); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve control-and-echo as 0xLeif
- 0xLeif decided "Add tests/spec_split.rs to control-and-echo's scope by one send-back, run by the fixer": "continue with 95% confidence" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 04:09Z, answering t4c's five-item list, each item carrying t4c's recommendation; he named no item. The reading is t4c's, recorded with his words at t4c's request: go ahead with the recommendations t4c is at least 95% sure of, items 1 to 4. Item 3, as t4c put it to him: 'Chough L2: add the one test file (tests/spec_split.rs) and widen the standing OK to cover tests that only pin a requirement's wording?'); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of control-and-echo with tests/spec_split.rs added": "continue with 95% confidence" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 04:09Z, on this text: tests/spec_split.rs, which pins REQ-reporting-001's wording, joins the declared paths, with no delta word. The reading is t4c's, recorded with his words at t4c's request: go ahead with t4c's recommendations it is at least 95% sure of, items 1 to 4, item 3 being 'Chough L2: add the one test file (tests/spec_split.rs) and widen the standing OK to cover tests that only pin a requirement's wording?'. The earlier approvals stand for the rest: 'Approve the recommended set', standing OK (d) for REQ-cli-007, 'Ok' (item 2) for the four paths, and standing OK 6(a) for REQ-reporting-001's example); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve control-and-echo as 0xLeif

## What was verified

- [Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-the-pull-request-shows-in-every-view.md)
- [Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form.md)
- [Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-the-post-links-only-a-pull-request-form.md)
- [Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-correct-names-what-a-given-entry-holds.md)
- [Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-control-character-is-refused-at-write.md)
- [Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw.md)
- [Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-generated-files-hold-text-as-printed.md)
- [Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-reserved-words-are-written-quoted.md)
- [Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-flags-stay-booleans.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-echoes-a-given-value.md)
- [Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-file-url-is-checked-like-its-path.md)
- [Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-reporting-timing-prints-stored-text-escaped.md)
- [Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-tabs-in-values-read-back.md)
- [Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-the-pull-request-shows-in-every-view-c10aa7.md)
- [Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-ac089a.md)
- [Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-27ad78.md)
- [Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-correct-names-what-a-given-entry-holds-7f3465.md)
- [Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-control-character-is-refused-at-write-5ac074.md)
- [Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b3814b.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-55f86d.md)
- [Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-generated-files-hold-text-as-printed-0897b5.md)
- [Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-reserved-words-are-written-quoted-c69322.md)
- [Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-flags-stay-booleans-5184c5.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-echoes-a-given-value-073e44.md)
- [Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-file-url-is-checked-like-its-path-72c270.md)
- [Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-reporting-timing-prints-stored-text-escaped-551f25.md)
- [Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-tabs-in-values-read-back-867742.md)
- [Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-the-pull-request-shows-in-every-view-18092a.md)
- [Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-79cb99.md)
- [Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-fa2c8f.md)
- [Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-correct-names-what-a-given-entry-holds-a77687.md)
- [Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-control-character-is-refused-at-write-5847df.md)
- [Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-c7d3e9.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-f7278f.md)
- [Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-generated-files-hold-text-as-printed-35d318.md)
- [Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-reserved-words-are-written-quoted-fd1c42.md)
- [Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-flags-stay-booleans-0fd3cd.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-echoes-a-given-value-73f48a.md)
- [Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-file-url-is-checked-like-its-path-68b196.md)
- [Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-reporting-timing-prints-stored-text-escaped-622b00.md)
- [Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-tabs-in-values-read-back-64ca6f.md)

## Open threads

## Timeline

- 2026-11-19T06:46:15Z · start · intent → intent · Control and echo batch: control characters refused at write and escaped on print (#366), --pr takes a number or an http(s) URL and is shown in next, status, context and the checkpoint view (#353), the remaining sites echo no value (part of #367), and a file URL holds the user name ^t-4da47c
  - revision: 9a81c859ae449f9da541cab3538d318de411a1ee
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T06:47:18Z · advance · intent → design ·  ^t-d9ff7a
  - revision: 9a81c859ae449f9da541cab3538d318de411a1ee
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T06:47:18Z · advance · design → plan ·  ^t-ab1ddc
  - revision: 9a81c859ae449f9da541cab3538d318de411a1ee
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:41:50Z · gate-opened · plan → plan · Standing OK (d): rebase control-and-echo's REQ-cli-007 onto main after #413 ^t-aa2996
  - native_change: control-and-echo
  - revision: 248d6984a5000cceba8cfff149c01d6b22c26e50
  - holds: plan
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:41:50Z · gate-closed · plan → plan · Standing OK (d): rebase control-and-echo's REQ-cli-007 onto main after #413 ^t-43f02e
  - native_change: control-and-echo
  - revision: 248d6984a5000cceba8cfff149c01d6b22c26e50
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes to both
    - where: Standing OK (d), Leif's Claude Code chat message relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:02Z; #413 changed REQ-cli-007 on main, so the delta was rebased onto main's text at 2968f061 with exactly the approved words (six body edits, one --pr criterion replaced and two added) and #413's (one sentence, one criterion) kept, the criteria placed as main's three unchanged, L2's three, then #413's; fable-lead verified by script on 2026-11-20 that nothing else changed
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:41:50Z · gate-opened · plan → plan · Approve the scope of control-and-echo ^t-821609
  - native_change: control-and-echo
  - revision: 248d6984a5000cceba8cfff149c01d6b22c26e50
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:41:50Z · gate-closed · plan → plan · Approve the scope of control-and-echo ^t-f0a5ad
  - native_change: control-and-echo
  - revision: 248d6984a5000cceba8cfff149c01d6b22c26e50
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve the recommended set
    - where: Claude Code chat answer, relayed by his planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-19 about 14:12Z, on L2 control-and-echo at 1068f67 with the seven recommended choices; recorded at the batch start on that text, with REQ-cli-007 rebased onto main after #413 under standing OK (d) (his 'yes to both', about 15:02Z, relayed by t4c; fable-lead's script check of 2026-11-20)
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve control-and-echo
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:42:57Z · advance · plan → build · native:approved ^t-206354
  - native_change: control-and-echo
  - gate: native:approved
  - revision: 0e29a28a355707d0b1403a4059b9a564f7a84b6f
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T03:03:14Z · gate-opened · build → build · Add four files to control-and-echo's scope by one send-back, run by the fixer ^t-3e92f4
  - native_change: control-and-echo
  - revision: df70adb2274a78df8c073be78c35b77dd80b2ac2
  - holds: build
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T03:03:14Z · gate-closed · build → build · Add four files to control-and-echo's scope by one send-back, run by the fixer ^t-fca033
  - native_change: control-and-echo
  - revision: df70adb2274a78df8c073be78c35b77dd80b2ac2
  - relayed_by:
    - decided_by: 0xLeif
    - words: Ok
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 03:00Z, answering t4c's six-item list, put to him with 'all yes' as the way to accept all; item 2: add src/evidence.rs, src/gates.rs, tests/front_matter_writes.rs and its case file .chough/cases/records.test-stage.cases.toml to L2's scope by one send-back the fixer runs, no sentence change
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T03:03:48Z · gate-opened · build → build · Re-approval of control-and-echo with four more paths and REQ-reporting-001's example ^t-f9e2ca
  - native_change: control-and-echo
  - revision: df70adb2274a78df8c073be78c35b77dd80b2ac2
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T03:03:48Z · direct-native-write · build → build · control-and-echo's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-845a37
  - native_change: control-and-echo
  - revision: df70adb2274a78df8c073be78c35b77dd80b2ac2
  - direct_write:
    - change: control-and-echo
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T03:03:48Z · gate-closed · build → build · Re-approval of control-and-echo with four more paths and REQ-reporting-001's example ^t-0383ba
  - native_change: control-and-echo
  - revision: df70adb2274a78df8c073be78c35b77dd80b2ac2
  - relayed_by:
    - decided_by: 0xLeif
    - words: Ok
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 03:00Z, item 2 of t4c's six-item list, on this text: src/evidence.rs, src/gates.rs, tests/front_matter_writes.rs and .chough/cases/records.test-stage.cases.toml join the declared paths, with no sentence change; and REQ-reporting-001's example and criterion name a gate's name in place of a stage move's reason, under standing OK 6(a), a wording fix that only makes an approved sentence match tested code, relayed via t4c and checked by fable-lead's script; the earlier approvals stand for the rest: 'Approve the recommended set' (2026-11-19 about 14:12Z) and standing OK (d) for REQ-cli-007's rebase after #413
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve control-and-echo
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T04:10:21Z · gate-opened · build → build · Add tests/spec_split.rs to control-and-echo's scope by one send-back, run by the fixer ^t-9b6ba5
  - native_change: control-and-echo
  - revision: 4edfaac69645eed7244af72c97c4864272eff5c3
  - holds: build
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T04:10:21Z · gate-closed · build → build · Add tests/spec_split.rs to control-and-echo's scope by one send-back, run by the fixer ^t-d236b0
  - native_change: control-and-echo
  - revision: 4edfaac69645eed7244af72c97c4864272eff5c3
  - relayed_by:
    - decided_by: 0xLeif
    - words: continue with 95% confidence
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 04:09Z, answering t4c's five-item list, each item carrying t4c's recommendation; he named no item. The reading is t4c's, recorded with his words at t4c's request: go ahead with the recommendations t4c is at least 95% sure of, items 1 to 4. Item 3, as t4c put it to him: 'Chough L2: add the one test file (tests/spec_split.rs) and widen the standing OK to cover tests that only pin a requirement's wording?'
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T04:10:46Z · gate-opened · build → build · Re-approval of control-and-echo with tests/spec_split.rs added ^t-6b5392
  - native_change: control-and-echo
  - revision: 4edfaac69645eed7244af72c97c4864272eff5c3
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T04:10:46Z · direct-native-write · build → build · control-and-echo's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-f1de82
  - native_change: control-and-echo
  - revision: 4edfaac69645eed7244af72c97c4864272eff5c3
  - direct_write:
    - change: control-and-echo
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T04:10:46Z · gate-closed · build → build · Re-approval of control-and-echo with tests/spec_split.rs added ^t-e59a99
  - native_change: control-and-echo
  - revision: 4edfaac69645eed7244af72c97c4864272eff5c3
  - relayed_by:
    - decided_by: 0xLeif
    - words: continue with 95% confidence
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 04:09Z, on this text: tests/spec_split.rs, which pins REQ-reporting-001's wording, joins the declared paths, with no delta word. The reading is t4c's, recorded with his words at t4c's request: go ahead with t4c's recommendations it is at least 95% sure of, items 1 to 4, item 3 being 'Chough L2: add the one test file (tests/spec_split.rs) and widen the standing OK to cover tests that only pin a requirement's wording?'. The earlier approvals stand for the rest: 'Approve the recommended set', standing OK (d) for REQ-cli-007, 'Ok' (item 2) for the four paths, and standing OK 6(a) for REQ-reporting-001's example
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve control-and-echo
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
