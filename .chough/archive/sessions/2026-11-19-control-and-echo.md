---
kind: session
id: 2026-11-19-control-and-echo
aliases: [control-and-echo]
objective: "Control and echo batch: control characters refused at write and escaped on print (#366), --pr takes a number or an http(s) URL and is shown in next, status, context and the checkpoint view (#353), the remaining sites echo no value (part of #367), and a file URL holds the user name"
stage: done
stage_status: archived
iteration: 1
native_change: control-and-echo
pr: "418"
criteria: [LIFE-26, DRIVE-26, LIFE-3]
requirements: [REQ-records-006, REQ-cli-007, REQ-history-015, REQ-records-016, REQ-reporting-001]
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
  - 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-5785af
  - 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-10a1cb
  - 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-21e264
  - 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-2ffaad
  - 2026-11-20-tc-records-a-control-character-is-refused-at-write-a4049e
  - 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-2cc44e
  - 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-62cb20
  - 2026-11-20-tc-records-generated-files-hold-text-as-printed-c67389
  - 2026-11-20-tc-records-reserved-words-are-written-quoted-10e700
  - 2026-11-20-tc-records-flags-stay-booleans-5fee50
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-b80628
  - 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-f5bb39
  - 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-6d845e
  - 2026-11-20-tc-records-tabs-in-values-read-back-31f266
  - 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-1b762e
  - 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-88607b
  - 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-a61c25
  - 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-43c234
  - 2026-11-20-tc-records-a-control-character-is-refused-at-write-1c7fa0
  - 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-4e814f
  - 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-322f44
  - 2026-11-20-tc-records-generated-files-hold-text-as-printed-77c25f
  - 2026-11-20-tc-records-reserved-words-are-written-quoted-4db936
  - 2026-11-20-tc-records-flags-stay-booleans-90ec8b
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-608a6b
  - 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-5ae0f6
  - 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable
  - 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path
  - 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-6c0449
  - 2026-11-20-tc-records-tabs-in-values-read-back-2ec9d6
  - 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-52888f
  - 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-13622e
  - 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-4fb643
  - 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-8c2fd9
  - 2026-11-20-tc-records-a-control-character-is-refused-at-write-966aab
  - 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b8e56d
  - 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-26365a
  - 2026-11-20-tc-records-generated-files-hold-text-as-printed-7c42bf
  - 2026-11-20-tc-records-reserved-words-are-written-quoted-5e953a
  - 2026-11-20-tc-records-flags-stay-booleans-deeb28
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-741e03
  - 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-6e6698
  - 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-ac3bde
  - 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-fc9781
  - 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-192171
  - 2026-11-20-tc-records-tabs-in-values-read-back-de8a98
  - 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-bed804
  - 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-69ab28
  - 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-d489f3
  - 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-04b819
  - 2026-11-20-tc-records-a-control-character-is-refused-at-write-f8ba89
  - 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-d557ef
  - 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-345b8b
  - 2026-11-20-tc-records-generated-files-hold-text-as-printed-c80aa1
  - 2026-11-20-tc-records-reserved-words-are-written-quoted-ef0699
  - 2026-11-20-tc-records-flags-stay-booleans-c8cc8a
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-98eaf9
  - 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-aeff0f
  - 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-446851
  - 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-6fa760
  - 2026-11-20-tc-records-a-fragment-of-a-stored-control-line-is-refused
  - 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-593542
  - 2026-11-20-tc-records-tabs-in-values-read-back-936d51
  - 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-363716
  - 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-84c2f2
  - 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-094d32
  - 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-24f590
  - 2026-11-20-tc-records-a-control-character-is-refused-at-write-322676
  - 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b60c5d
  - 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-2c65bb
  - 2026-11-20-tc-records-generated-files-hold-text-as-printed-32f51c
  - 2026-11-20-tc-records-reserved-words-are-written-quoted-ce0898
  - 2026-11-20-tc-records-flags-stay-booleans-4026e1
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-d1cdc9
  - 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-730dc1
  - 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-d6591f
  - 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-2fcaa8
  - 2026-11-20-tc-records-a-fragment-of-a-stored-control-line-is-refused-a21996
  - 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-31d612
  - 2026-11-20-tc-records-tabs-in-values-read-back-dfcbc2
closed: 2026-11-20
outcome: done
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
- 0xLeif decided "Implementation review of control-and-echo": "Go" (Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #418 at 7c1b9f03: the tester's PASS at 7c1b9f03 (only #412's rows still failing), corvid-agent APPROVED 7c1b9f03 (2026-11-20 about 07:07Z), and fable-lead's read of every head; every sentence change in this batch carries Leif's own words or a standing OK (the scope approval, standing OK (d) for REQ-cli-007, his 'Ok' for four paths with standing OK 6(a) for REQ-reporting-001's example, and 'continue with 95% confidence', read by t4c, for tests/spec_split.rs); relayed by fable-lead (hc t53), 2026-11-20); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change review control-and-echo as 0xLeif with verdict pass

## What was verified

- 105 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1). - 1c42bd82cb58; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-the-pull-request-shows-in-every-view-363716
- Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1). - 77e5574c0712; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-84c2f2
- Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1). - 01e6fca273ee; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-094d32
- Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1). - aafdac08970c; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-correct-names-what-a-given-entry-holds-24f590
- Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1). - 2c9d76bc6599; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-a-control-character-is-refused-at-write-322676
- Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1). - b9d44f9a7394; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b60c5d
- Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1). - b6a29b0b76db; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-2c65bb
- Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1). - 389390150f06; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-generated-files-hold-text-as-printed-32f51c
- Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1). - ceab4deb9157; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-reserved-words-are-written-quoted-ce0898
- Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1). - b3c28d2abcf2; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-flags-stay-booleans-4026e1
- Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1). - 362e1d6e99bd; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-no-command-echoes-a-given-value-d1cdc9
- Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1). - cd689ae962cd; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-a-file-url-is-checked-like-its-path-730dc1
- Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1). - b81218fba640; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-d6591f
- Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1). - 60f6d429afe6; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-2fcaa8
- Ran cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused for TC-records-a-fragment-of-a-stored-control-line-is-refused: passed (exit 0, tests run 1). - 43f6bb48e1b3; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-a-fragment-of-a-stored-control-line-is-refused-a21996
- Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1). - 7f73730557ec; limits: inherited environment variables were not recorded ^2026-11-20-tc-reporting-timing-prints-stored-text-escaped-31d612
- Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1). - ecb7f2b9f53c; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-tabs-in-values-read-back-dfcbc2

## Bugs

None.

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
- 2026-11-20T07:32:26Z · advance · build → test · native:verifying ^t-443c44
  - native_change: control-and-echo
  - gate: native:verifying
  - revision: 7c1b9f03cb0f8bc52d3402732cf11095ad9810cb
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T07:32:28Z · advance · test → accept · native:verifying ^t-af1f08
  - native_change: control-and-echo
  - gate: native:verifying
  - revision: 7c1b9f03cb0f8bc52d3402732cf11095ad9810cb
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T07:32:32Z · gate-opened · accept → accept · Implementation review of control-and-echo ^t-d3fe66
  - native_change: control-and-echo
  - revision: 7c1b9f03cb0f8bc52d3402732cf11095ad9810cb
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T07:33:30Z · gate-closed · accept → accept · Implementation review of control-and-echo ^t-55a9bd
  - native_change: control-and-echo
  - revision: 084d4d1c57cbd5386cee412660f6243dfe1f626e
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #418 at 7c1b9f03: the tester's PASS at 7c1b9f03 (only #412's rows still failing), corvid-agent APPROVED 7c1b9f03 (2026-11-20 about 07:07Z), and fable-lead's read of every head; every sentence change in this batch carries Leif's own words or a standing OK (the scope approval, standing OK (d) for REQ-cli-007, his 'Ok' for four paths with standing OK 6(a) for REQ-reporting-001's example, and 'continue with 95% confidence', read by t4c, for tests/spec_split.rs); relayed by fable-lead (hc t53), 2026-11-20
    - recorded_by: chough-fixer-t4g
    - native: specsync change review control-and-echo
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T07:33:33Z · advance · accept → done · native:archived ^t-dd36d2
  - native_change: control-and-echo
  - gate: native:archived
  - revision: 084d4d1c57cbd5386cee412660f6243dfe1f626e
  - by: chough-fixer (claude-opus-5-5, hc t4n)

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view
    claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:f6348d8637f3fe4363e07302d0f1313e05b72158fcfb19d049291b2fbf7d89b5
    case: TC-cli-the-pull-request-shows-in-every-view
    case_digest: 7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:20:56Z
    duration_seconds: 8
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form
    claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:acb0e610c5a2d263612134baad7399cc2cbae6c62177cf69f9009343fe27c3c7
    case: TC-cli-every-session-writer-keeps-the-pull-request-form
    case_digest: 504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:20:59Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form
    claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:472aec1a60e68e72974406c43e5a9a18dfeef26f68d69d888c017a5b2bb951f3
    case: TC-cli-the-post-links-only-a-pull-request-form
    case_digest: 6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:03Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds
    claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:7fde0017927b7217104d7e61d68756e185d02b609e552d4e8fe5e30c8cff46cc
    case: TC-history-correct-names-what-a-given-entry-holds
    case_digest: 2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc
    criteria: LIFE-18, LIFE-7
    requirements: REQ-history-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:06Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-control-character-is-refused-at-write
    claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:330f1d125115e3d0579e93d3e9c7e3a3d192818be8a37285cfca320faf8c36e6
    case: TC-records-a-control-character-is-refused-at-write
    case_digest: 0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:13Z
    duration_seconds: 5
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one
    claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:8f34ef89224a6577d6ad2df384ac99e9600d0c4293056686092b427a6d1b0c09
    case: TC-records-a-line-feed-is-kept-where-a-field-takes-one
    case_digest: 45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:16Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:be7f5eda96b8911e7cc9d21a799ed5023eab156ba311d8937afa6df6cba8c577
    case: TC-records-no-command-prints-a-stored-control-character-raw
    case_digest: c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:25Z
    duration_seconds: 7
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-generated-files-hold-text-as-printed
    claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:5be39eefaa39dc1fdec32d703738fc05e1601c542e0c1ceaa6b1826a38d706c1
    case: TC-records-generated-files-hold-text-as-printed
    case_digest: cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:27Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-reserved-words-are-written-quoted
    claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:6dc42844d959add1ae6716378c04dc2260fd1ce8301412321b6420e79620c15a
    case: TC-records-reserved-words-are-written-quoted
    case_digest: 6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:30Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-flags-stay-booleans
    claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:72e386f0e7398fc25807919c855be2b2ad2771a2e8a5e7ed92811e173803d1c3
    case: TC-records-flags-stay-booleans
    case_digest: c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:32Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-echoes-a-given-value
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:61ab5149d050363bf7c656182693a8c88ee2b654c12cd6d70234ffd49272c87d
    case: TC-records-no-command-echoes-a-given-value
    case_digest: 76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:43Z
    duration_seconds: 9
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path
    claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:d2159ba8d0e981033cab27ccfdb5f874f6b3905f35da6a98278127bfd8c13618
    case: TC-records-a-file-url-is-checked-like-its-path
    case_digest: b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:45Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped
    claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:3cf94e5436e76a6269d9e10d0465a29d0692ba16772976edbb192f486c10abf4
    case: TC-reporting-timing-prints-stored-text-escaped
    case_digest: 9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5
    criteria: LIFE-10, LIFE-3
    requirements: REQ-reporting-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:48Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-tabs-in-values-read-back
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
    revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
    environment: local-macos-arm64
    content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
    fingerprint: sha256:157512629af7fa82c9110b0ee54aeef14d6deee95558737bf29aa0a8ae69f696
    case: TC-records-tabs-in-values-read-back
    case_digest: 40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22
    criteria: DRIVE-25, DRIVE-26, KNOW-32, LIFE-3
    requirements: REQ-records-001, REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:21:52Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-c10aa7
    claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:7a4bec3f3c7135aef9449c4cbc0af15c207fdb42952478efa5cddf834c8fd94e
    case: TC-cli-the-pull-request-shows-in-every-view
    case_digest: 7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:11Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-ac089a
    claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:51af22f880de2bdaf113275b6a3c8288ff1ed592234708237faca39b987cb31f
    case: TC-cli-every-session-writer-keeps-the-pull-request-form
    case_digest: 504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:15Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-27ad78
    claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:a6673e8136a2ddf17a4db235ce1af48763d3704decc0a16be4a20a3d872d9599
    case: TC-cli-the-post-links-only-a-pull-request-form
    case_digest: 6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:18Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-7f3465
    claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:eb626bbb221e3910801c406ca6207728659004af6f5e2a5b3dd74ad522f0f764
    case: TC-history-correct-names-what-a-given-entry-holds
    case_digest: 2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc
    criteria: LIFE-18, LIFE-7
    requirements: REQ-history-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:21Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-5ac074
    claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:20d3a7f17dbe13bf38650c9b1760f348021d53360f32a69e67a3f661ef188ef0
    case: TC-records-a-control-character-is-refused-at-write
    case_digest: 0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:31Z
    duration_seconds: 9
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b3814b
    claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:45e888f6dea132359d74feb81c1df1017cdab7d9a1f41795ed09616fd059ff72
    case: TC-records-a-line-feed-is-kept-where-a-field-takes-one
    case_digest: 45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:34Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-55f86d
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:02058d7c2958a362d0f5b9c4eaa99c10a776f4cf45eb39f92ef692431e972b0c
    case: TC-records-no-command-prints-a-stored-control-character-raw
    case_digest: c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:46Z
    duration_seconds: 10
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-0897b5
    claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:539aa3b4440aeb92185a6a8dd3d5840b308f7322f1ee94143dcb707e60579b0e
    case: TC-records-generated-files-hold-text-as-printed
    case_digest: cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:48Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-reserved-words-are-written-quoted-c69322
    claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:e7906d8d31a1adf4db269e6b8ed8dac93fd5b25dd56b21d50077b23d4240ef8c
    case: TC-records-reserved-words-are-written-quoted
    case_digest: 6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:51Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-flags-stay-booleans-5184c5
    claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:76ea2fd5fdfa4878e5c417afd18c2db9e1d475a45fdb70b715ae683ccfddea81
    case: TC-records-flags-stay-booleans
    case_digest: c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:23:53Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-echoes-a-given-value-073e44
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:ee0bef1890202c20b1ca9c3e7cc7bb35f87c1d749ed88f14663410785db0ed1b
    case: TC-records-no-command-echoes-a-given-value
    case_digest: 76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:24:07Z
    duration_seconds: 11
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-72c270
    claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:f0fae125ff8961a9565cc4be2c06019561fe877384d6c3c445dfb4b031a3bf11
    case: TC-records-a-file-url-is-checked-like-its-path
    case_digest: b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:24:10Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-551f25
    claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:13df1b2f5411e824ae0fa62767049e04419534982f0b343f50b32878f9f1f42d
    case: TC-reporting-timing-prints-stored-text-escaped
    case_digest: 9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5
    criteria: LIFE-10, LIFE-3
    requirements: REQ-reporting-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:24:12Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-tabs-in-values-read-back-867742
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
    revision: 92b89b970299939dd38635eab1c13fb1952b539a
    environment: local-macos-arm64
    content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
    fingerprint: sha256:ca05ea93fa44a9046c143213e20e36da866f635a85a7d758176c4390d0226121
    case: TC-records-tabs-in-values-read-back
    case_digest: 40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22
    criteria: DRIVE-25, DRIVE-26, KNOW-32, LIFE-3
    requirements: REQ-records-001, REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T03:24:18Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-18092a
    claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:3e01a1f30d5f4c7c70796580d3773d4b254b9cf1ae57e9ecb009fc441d625f84
    case: TC-cli-the-pull-request-shows-in-every-view
    case_digest: 7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:11:44Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-79cb99
    claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:af851aa711b00af0649b0d8fecf6889928591aa4d2c93375008eaa459265ad93
    case: TC-cli-every-session-writer-keeps-the-pull-request-form
    case_digest: 504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:11:46Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-fa2c8f
    claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:c6516337aae3fd186bf7f1d61f666bae1722046bca82eea2e094ee2345d17219
    case: TC-cli-the-post-links-only-a-pull-request-form
    case_digest: 6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:11:50Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-a77687
    claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:aafdac08970c43f9546a74df4a023ed459bd0fab9c4dbf6e60e40b2fd3fb0e7b
    case: TC-history-correct-names-what-a-given-entry-holds
    case_digest: 2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc
    criteria: LIFE-18, LIFE-7
    requirements: REQ-history-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:11:52Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-5847df
    claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:e08d11d3f1cbc199484c9bc2ac382a419d9a42b71dcdc7c475262b611c3304e8
    case: TC-records-a-control-character-is-refused-at-write
    case_digest: 0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:01Z
    duration_seconds: 7
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-c7d3e9
    claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:ccd4edb66094a2cdf925eda72b92ff78abfecc5282678ebcc2377f0f7db9ac57
    case: TC-records-a-line-feed-is-kept-where-a-field-takes-one
    case_digest: 45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:04Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-f7278f
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:67a95946b639017e6e6dd5d3592ab86bd5a1ac2baad723d86072dd0e4b9a7894
    case: TC-records-no-command-prints-a-stored-control-character-raw
    case_digest: c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:15Z
    duration_seconds: 9
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-35d318
    claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:bcf138f69519bbab98b23831d1527fb0ec4326dad8f786c57e48743d435833ca
    case: TC-records-generated-files-hold-text-as-printed
    case_digest: cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:18Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-reserved-words-are-written-quoted-fd1c42
    claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:ad171f2b026fb9fbc78193f0b4d8a5f86856fd289953f4ded7b1137d6565fdd0
    case: TC-records-reserved-words-are-written-quoted
    case_digest: 6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:21Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-flags-stay-booleans-0fd3cd
    claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:cf8188ba886057bbd9dc636ff90ce33938eb56a6417d1cb95180a075a08f553c
    case: TC-records-flags-stay-booleans
    case_digest: c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:23Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-echoes-a-given-value-73f48a
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:20def220fcb865125adcb7878a8bd8923d69d5b6175e6dd94bcaed4c0a8dad33
    case: TC-records-no-command-echoes-a-given-value
    case_digest: 76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:38Z
    duration_seconds: 13
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-68b196
    claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:7132da844e67fc75c87d0469f6e49bdfa716ee723cca91af6c4af02206924911
    case: TC-records-a-file-url-is-checked-like-its-path
    case_digest: b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:41Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-622b00
    claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:902432363527d7f153fce32f3e71fcbce53674a4b7f7c79067c8009b783e5ebe
    case: TC-reporting-timing-prints-stored-text-escaped
    case_digest: 9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5
    criteria: LIFE-10, LIFE-3
    requirements: REQ-reporting-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:43Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-tabs-in-values-read-back-64ca6f
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
    revision: 26a42a15d428cc7db4805069cc58c0d23330f390
    environment: local-macos-arm64
    content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
    fingerprint: sha256:2fb5d059bbf416903ec5a47b9e58e4259fef8a21fb7e650f7e99ebeaab596a0e
    case: TC-records-tabs-in-values-read-back
    case_digest: 40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22
    criteria: DRIVE-25, DRIVE-26, KNOW-32, LIFE-3
    requirements: REQ-records-001, REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:12:48Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-5785af
    claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:1f64a26b86fede592d02ceb2e6e987ad7aa3759b9761a8a6938e274e317170a8
    case: TC-cli-the-pull-request-shows-in-every-view
    case_digest: 7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:33:11Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-10a1cb
    claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:93d95d4fcc294680c5a475888185ebb2d4e5858dad2c48932bd3adeab5a50230
    case: TC-cli-every-session-writer-keeps-the-pull-request-form
    case_digest: 504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:33:17Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-21e264
    claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:82437c435460e226dfa10ccf6e6cec0ff7152257cbfe701c452f6fdd1507a9cb
    case: TC-cli-the-post-links-only-a-pull-request-form
    case_digest: 6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:33:24Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-2ffaad
    claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:37527535669c8541fa65622fdc06e651f03c71ccfb3d22bb4ed0f25924804a6b
    case: TC-history-correct-names-what-a-given-entry-holds
    case_digest: 2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc
    criteria: LIFE-18, LIFE-7
    requirements: REQ-history-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:33:28Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-a4049e
    claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:d008952dd4fa1fc4ea1dabb35f6ae814bd13d1d3036dde26c3945531539c920c
    case: TC-records-a-control-character-is-refused-at-write
    case_digest: 0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:33:50Z
    duration_seconds: 20
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-2cc44e
    claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:a45e4619a3ef458dcc662ebdc36a81cddc064959a9f7ac518b4df13a53a7c0de
    case: TC-records-a-line-feed-is-kept-where-a-field-takes-one
    case_digest: 45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:33:53Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-62cb20
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:6d21bd27df716e249e6e91dab6c0e9667663aeb35f3b6d3602c2d045cffaa11d
    case: TC-records-no-command-prints-a-stored-control-character-raw
    case_digest: c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:34:07Z
    duration_seconds: 11
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-c67389
    claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:dd83ecee7bb4149dc2c5f95e89e1bc3b3fafc63c4c57d5eba46cf1dab7c97f2a
    case: TC-records-generated-files-hold-text-as-printed
    case_digest: cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:34:09Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-reserved-words-are-written-quoted-10e700
    claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:47ccc0d5ce32c34011312655396acf51f2eaf7e0b97cf69d5090e72fb288c011
    case: TC-records-reserved-words-are-written-quoted
    case_digest: 6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:34:12Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-flags-stay-booleans-5fee50
    claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:44d33439157bfb05c31da9d6174e039d7926db7713ab20cee1f1897d1e50c81d
    case: TC-records-flags-stay-booleans
    case_digest: c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:34:14Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-echoes-a-given-value-b80628
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:f5c4ab0fddd2034904a17d39c4bc5cbf48c3bf2bce4d504947a8f907d3bd31a1
    case: TC-records-no-command-echoes-a-given-value
    case_digest: 76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:34:28Z
    duration_seconds: 12
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-f5bb39
    claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:4603d02700cfce87f818a3c8d2ef7b5af751d256f446b75befb54df74f4f3a1e
    case: TC-records-a-file-url-is-checked-like-its-path
    case_digest: b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:34:31Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-6d845e
    claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:7f73730557ec6f3f9a67c080e7ac22b1c9dc865b6713a1937af00c41e8b32a0f
    case: TC-reporting-timing-prints-stored-text-escaped
    case_digest: 9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5
    criteria: LIFE-10, LIFE-3
    requirements: REQ-reporting-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:34:33Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-tabs-in-values-read-back-31f266
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
    revision: cdc0d2e2f85a6d09567b8780662656ccec551526
    environment: local-macos-arm64
    content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
    fingerprint: sha256:094e507bd807edf17bcd541e2c9f791e14af2123784cc34f18a1a957d3f8c4d9
    case: TC-records-tabs-in-values-read-back
    case_digest: 40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22
    criteria: DRIVE-25, DRIVE-26, KNOW-32, LIFE-3
    requirements: REQ-records-001, REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:34:39Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-1b762e
    claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:55d0c72e4345963ff068a23cee62187f9e6b50a9ffec44722fb97e1898523cd7
    case: TC-cli-the-pull-request-shows-in-every-view
    case_digest: 7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:41:42Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-88607b
    claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:3a88dcf2a57ce081ce51f5599477bdd01d93c64cc192aa91e207523bbd1e7c5d
    case: TC-cli-every-session-writer-keeps-the-pull-request-form
    case_digest: 504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:41:45Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-a61c25
    claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:2e17c7a3adaa84b9b3a97be29cf11ee89dfef1b5bfdeb5f5ac15eac78837ad63
    case: TC-cli-the-post-links-only-a-pull-request-form
    case_digest: 6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:41:49Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-43c234
    claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:b89a9c32aae4d4dbc642004cfa77d4f14be1fa016074b547e7fc94c04a5e4b16
    case: TC-history-correct-names-what-a-given-entry-holds
    case_digest: 2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc
    criteria: LIFE-18, LIFE-7
    requirements: REQ-history-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:41:51Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-1c7fa0
    claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:189fbf16244f4daea024763d63de04170a6deb2a0c9d207891e9b97764454129
    case: TC-records-a-control-character-is-refused-at-write
    case_digest: 0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:41:59Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-4e814f
    claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:9636125d79d042b66073a29868bed99d0bb5511f2a926c4acc40398f5481159b
    case: TC-records-a-line-feed-is-kept-where-a-field-takes-one
    case_digest: 45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:01Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-322f44
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:8615d4c5ae0c0c3429befd37ec494a1618c97881eabba37c89d88c7800a0e3f3
    case: TC-records-no-command-prints-a-stored-control-character-raw
    case_digest: c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:10Z
    duration_seconds: 7
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-77c25f
    claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:383df9f90997867fe68aa102967f705755aca156e9e9ceeb41057001b9632129
    case: TC-records-generated-files-hold-text-as-printed
    case_digest: cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:13Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-reserved-words-are-written-quoted-4db936
    claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:5472835afffd44e8bc20641475f594a4d761bc84c089db887b72c0bc5ad0464e
    case: TC-records-reserved-words-are-written-quoted
    case_digest: 6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:15Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-flags-stay-booleans-90ec8b
    claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:b3c28d2abcf2dc2c1bcb8351a542cc223e377496e2b32f0cb053ff1924c5ea69
    case: TC-records-flags-stay-booleans
    case_digest: c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:17Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-echoes-a-given-value-608a6b
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:f0b5571e90257d7094ebe6e499e77b5793726665bc497ada42e4aeb30ee91481
    case: TC-records-no-command-echoes-a-given-value
    case_digest: 76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:29Z
    duration_seconds: 9
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-5ae0f6
    claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:b11737f55faf8e52ac02f4902830e7772834117af93fa62ebfcaa5f345f0cd70
    case: TC-records-a-file-url-is-checked-like-its-path
    case_digest: b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:31Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable
    claim: "Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:fe845b5f3a630910fa4cabe7d49ece327a73f220b08212913e3bd21923ca5c5b
    case: TC-records-a-stored-inbox-entry-folds-printable
    case_digest: 3481f41774e6a2e6e61190d52263a109b722dc7f01f03e6e88a0a0dbcc5b17ab
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:34Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path
    claim: "Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:7aa42caf17afd80cda1e946d249ba017265aae934514d4715e6c4c75d0b34cde
    case: TC-records-bug-merge-never-shows-a-leaking-path
    case_digest: 774b3c328cf1ef1475cde9340b9edbf6fa619022f5f2b87d85cbc0e0b0afe416
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:36Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-6c0449
    claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:82c320fd5d772915121e52ee240c6b3e6ba024e1cddb8c67bc07a9e4341b4a1f
    case: TC-reporting-timing-prints-stored-text-escaped
    case_digest: 9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5
    criteria: LIFE-10, LIFE-3
    requirements: REQ-reporting-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:38Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-tabs-in-values-read-back-2ec9d6
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
    revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
    environment: local-macos-arm64
    content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
    fingerprint: sha256:0b3f67e20fa1dfa81248bc9c34d9f351d48ee86d048d60b3ed73f373840b9098
    case: TC-records-tabs-in-values-read-back
    case_digest: 40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22
    criteria: DRIVE-25, DRIVE-26, KNOW-32, LIFE-3
    requirements: REQ-records-001, REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T05:42:42Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-52888f
    claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:0cc3fb5974d5d8cb3d2048b5603bca04f7d5515253ec661d16c0217de5f98eea
    case: TC-cli-the-pull-request-shows-in-every-view
    case_digest: 7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:11:07Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-13622e
    claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:06ff64b427728d6bbd123e856e1eb9c69483c0c2b74bbfad947fa8d8c2c8804f
    case: TC-cli-every-session-writer-keeps-the-pull-request-form
    case_digest: 504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:11:14Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-4fb643
    claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:fa1c4961b39c82cb4e04d9e662addfe5953e3020b81615fae833330965883c3c
    case: TC-cli-the-post-links-only-a-pull-request-form
    case_digest: 6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:11:23Z
    duration_seconds: 5
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-8c2fd9
    claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:dd6a311053dea7fe72b7721588ecd861bae1535d6c5d4a4e767d14936a1395b8
    case: TC-history-correct-names-what-a-given-entry-holds
    case_digest: 2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc
    criteria: LIFE-18, LIFE-7
    requirements: REQ-history-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:11:28Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-966aab
    claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:47b4dba3423981298ecab0c9f774e61001de1b33e9615b19f9e28a3008cb2c36
    case: TC-records-a-control-character-is-refused-at-write
    case_digest: 0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:11:56Z
    duration_seconds: 25
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b8e56d
    claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:559a78dd4c4198517645117a9fe9eb26b313d0518333de9181d9952c4e538cc8
    case: TC-records-a-line-feed-is-kept-where-a-field-takes-one
    case_digest: 45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:01Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-26365a
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:45dce0201661bac9ebeb2eebf951f4b528ebe1c6019bd327a09f8bc3a7cfbbd2
    case: TC-records-no-command-prints-a-stored-control-character-raw
    case_digest: c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:14Z
    duration_seconds: 10
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-7c42bf
    claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:d1788d72145b5462799c4f1e250c8b05022bd316344d41c2e03deb27381cbaa9
    case: TC-records-generated-files-hold-text-as-printed
    case_digest: cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:17Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-reserved-words-are-written-quoted-5e953a
    claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:31ae7923b501c98b59fb1c41e2b4e41bebe4b3f7c3fde56629b7302509187a47
    case: TC-records-reserved-words-are-written-quoted
    case_digest: 6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:20Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-flags-stay-booleans-deeb28
    claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:72e386f0e7398fc25807919c855be2b2ad2771a2e8a5e7ed92811e173803d1c3
    case: TC-records-flags-stay-booleans
    case_digest: c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:22Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-echoes-a-given-value-741e03
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:912a8f5c63dd62c230f87f03ee616297ae2576ac87c8b7feb9c95b7205150caa
    case: TC-records-no-command-echoes-a-given-value
    case_digest: 76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:35Z
    duration_seconds: 11
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-6e6698
    claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:d5d980f8aca412d5a1bdfc556fb95caedc9e4f4974fb0218b85578f754e586f2
    case: TC-records-a-file-url-is-checked-like-its-path
    case_digest: b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:38Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-ac3bde
    claim: "Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:e96af6d3d134082d068b4136291278a86d6ac3c638c65c8651c1923f30e9eaa4
    case: TC-records-a-stored-inbox-entry-folds-printable
    case_digest: 3481f41774e6a2e6e61190d52263a109b722dc7f01f03e6e88a0a0dbcc5b17ab
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:40Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-fc9781
    claim: "Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:2c7bdbd699e31159b5cc3ea6e3960868f3fcccebf1ecfe3b51f468b418a394cd
    case: TC-records-bug-merge-never-shows-a-leaking-path
    case_digest: 774b3c328cf1ef1475cde9340b9edbf6fa619022f5f2b87d85cbc0e0b0afe416
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:43Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-192171
    claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:76940c2358314e338e172cf10744f798b98d3dc148e89ffe7324cd93b07e4ef3
    case: TC-reporting-timing-prints-stored-text-escaped
    case_digest: 9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5
    criteria: LIFE-10, LIFE-3
    requirements: REQ-reporting-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:45Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-tabs-in-values-read-back-de8a98
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
    revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
    environment: local-macos-arm64
    content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
    fingerprint: sha256:8e6548d37ce731009587b2ef4f97670feffb090421539d5b4e008f07ad4ccc30
    case: TC-records-tabs-in-values-read-back
    case_digest: 40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22
    criteria: DRIVE-25, DRIVE-26, KNOW-32, LIFE-3
    requirements: REQ-records-001, REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:12:51Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-bed804
    claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:c6944e2d07ba3d960f23c4a37d7632009fb8979c0f67e57c864555bfc380a172
    case: TC-cli-the-pull-request-shows-in-every-view
    case_digest: 7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:25:44Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-69ab28
    claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:bcbf3dc6318efd55ec4d48fb3f62ec3a16651fd45bf5eb621a507043f70a07cd
    case: TC-cli-every-session-writer-keeps-the-pull-request-form
    case_digest: 504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:25:48Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-d489f3
    claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:0fc7d4bf3bbb4f568e7ba3ec3bbd62d23a28d84017351d99aa9185f765653e2d
    case: TC-cli-the-post-links-only-a-pull-request-form
    case_digest: 6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:25:53Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-04b819
    claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:79af7f62340babf794ad338fca13b53251042634ec1bd7720991f6418186d98c
    case: TC-history-correct-names-what-a-given-entry-holds
    case_digest: 2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc
    criteria: LIFE-18, LIFE-7
    requirements: REQ-history-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:25:56Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-f8ba89
    claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:94ffd852a3778ea73dc29a6dd02dace2c7b66c76064e6636a9df80f8b6b5c9f6
    case: TC-records-a-control-character-is-refused-at-write
    case_digest: 0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:06Z
    duration_seconds: 8
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-d557ef
    claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:ac605931bca2431db3d0623c101289ba2de2fb4166d24b3afb38fb30f9237aa7
    case: TC-records-a-line-feed-is-kept-where-a-field-takes-one
    case_digest: 45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:10Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-345b8b
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:964ee6d0e203c6390c26a3ce1c3c3161e828b4e8fb729a83f21b53a0ad2eede9
    case: TC-records-no-command-prints-a-stored-control-character-raw
    case_digest: c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:22Z
    duration_seconds: 10
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-c80aa1
    claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:81ce17988d4854833015c15ea269dc299c021d70d69429fdef3e4434d61951f9
    case: TC-records-generated-files-hold-text-as-printed
    case_digest: cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:25Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-reserved-words-are-written-quoted-ef0699
    claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:4c9f3150f5231a234e4e0bd5674e6286ee722d0bd3c0be52a09f0febbbbd632c
    case: TC-records-reserved-words-are-written-quoted
    case_digest: 6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:29Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-flags-stay-booleans-c8cc8a
    claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:44d33439157bfb05c31da9d6174e039d7926db7713ab20cee1f1897d1e50c81d
    case: TC-records-flags-stay-booleans
    case_digest: c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:31Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-echoes-a-given-value-98eaf9
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:4e655c5300fa65d0cc3641fa9a99469337f80c4c57fbd500415ced5ad3efd27e
    case: TC-records-no-command-echoes-a-given-value
    case_digest: 76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:47Z
    duration_seconds: 13
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-aeff0f
    claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:d983fd18af0b0a47d3a3bfb16f3387c35797534f9a5e9b2fae02c782972b5e3c
    case: TC-records-a-file-url-is-checked-like-its-path
    case_digest: b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:50Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-446851
    claim: "Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:bd24a404626c59b0440c7a9ee852ee89f7c864c545b282cc12c065b5680a8336
    case: TC-records-a-stored-inbox-entry-folds-printable
    case_digest: 3481f41774e6a2e6e61190d52263a109b722dc7f01f03e6e88a0a0dbcc5b17ab
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:53Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-6fa760
    claim: "Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:1f47c2b5c6332a42ad55bd433db467e33012cca3291c3e86af59c36cea720c89
    case: TC-records-bug-merge-never-shows-a-leaking-path
    case_digest: 774b3c328cf1ef1475cde9340b9edbf6fa619022f5f2b87d85cbc0e0b0afe416
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:26:56Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-fragment-of-a-stored-control-line-is-refused
    claim: "Ran cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused for TC-records-a-fragment-of-a-stored-control-line-is-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:5337c160ae7746ace5935d31bcae91e2a3f3ab19e81e3b0b7bd7050845d5a50e
    case: TC-records-a-fragment-of-a-stored-control-line-is-refused
    case_digest: 4e563d17db1240c7916c6bdaedeb23d6066dd601b8d7be50c3774d89b75f7cf1
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:27:00Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-593542
    claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:a85bfd36c983d773b2bb8621b2eaad654cf7a118ce6a04d43910ec837a6ab3c2
    case: TC-reporting-timing-prints-stored-text-escaped
    case_digest: 9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5
    criteria: LIFE-10, LIFE-3
    requirements: REQ-reporting-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:27:03Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-tabs-in-values-read-back-936d51
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
    revision: 17f214884b36a9edd288d49c998da9712d7c4117
    environment: local-macos-arm64
    content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
    fingerprint: sha256:078a30edd49a46731d8f4d199ba6c8d3951621e8048455a23d33664b20f09785
    case: TC-records-tabs-in-values-read-back
    case_digest: 40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22
    criteria: DRIVE-25, DRIVE-26, KNOW-32, LIFE-3
    requirements: REQ-records-001, REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:27:09Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-363716
    claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:1c42bd82cb58905c3481e0a9e9f3604acc3828e94168d28a17ea2250450d623c
    case: TC-cli-the-pull-request-shows-in-every-view
    case_digest: 7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:06Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-84c2f2
    claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:77e5574c07127780939f42640bf9cfbfca57ad7b9cc2b0a919b1acc261d08271
    case: TC-cli-every-session-writer-keeps-the-pull-request-form
    case_digest: 504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:09Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-094d32
    claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:01e6fca273eebacf740125c644afa3805d145ad0634de06294b6e71c799792f9
    case: TC-cli-the-post-links-only-a-pull-request-form
    case_digest: 6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11
    criteria: FLOW-13, LIFE-3
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:13Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-correct-names-what-a-given-entry-holds-24f590
    claim: "Ran cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it for TC-history-correct-names-what-a-given-entry-holds: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test history -- --exact correct_names_what_a_given_entry_holds_and_never_echoes_it
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:aafdac08970c43f9546a74df4a023ed459bd0fab9c4dbf6e60e40b2fd3fb0e7b
    case: TC-history-correct-names-what-a-given-entry-holds
    case_digest: 2aec6d6f86db7893a2a401f6cd8f00e56e132fdd023a582f7807df86a5992adc
    criteria: LIFE-18, LIFE-7
    requirements: REQ-history-015
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:16Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-control-character-is-refused-at-write-322676
    claim: "Ran cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field for TC-records-a-control-character-is-refused-at-write: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_control_character_is_refused_at_write_in_every_field
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:2c9d76bc6599df4d96c1824acb47c01a69ccd4c8175719073b12fdee9bffe6d8
    case: TC-records-a-control-character-is-refused-at-write
    case_digest: 0d4b3f0897a1f6b85befdccd570871a59f03bbe3f14a853a018f3cc972d3b706
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:28Z
    duration_seconds: 8
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-line-feed-is-kept-where-a-field-takes-one-b60c5d
    claim: "Ran cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused for TC-records-a-line-feed-is-kept-where-a-field-takes-one: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:b9d44f9a73943893d4df8fd976e8f8f5e16713da388a32e6e685faa5ca4f0f3b
    case: TC-records-a-line-feed-is-kept-where-a-field-takes-one
    case_digest: 45ed1184127f3ca3620c7d537003d97b60481a512e736cf5f63c7964aa4a70b4
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:34Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-prints-a-stored-control-character-raw-2c65bb
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw for TC-records-no-command-prints-a-stored-control-character-raw: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_prints_a_stored_control_character_raw
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:b6a29b0b76db33a2fa33af9264d0cec2cf7175c3e0c5ca3028babf598f9c10b5
    case: TC-records-no-command-prints-a-stored-control-character-raw
    case_digest: c5dfef7da7946fd01503a23be76a57c5d0071ae7bd7254b48a46511090800a10
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:47Z
    duration_seconds: 10
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-32f51c
    claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:389390150f06603cffe4e0d776d09f0f028407e2a5e62e5079c5b59561edd843
    case: TC-records-generated-files-hold-text-as-printed
    case_digest: cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:49Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-reserved-words-are-written-quoted-ce0898
    claim: "Ran cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text for TC-records-reserved-words-are-written-quoted: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact yaml_reserved_words_are_written_quoted_and_read_back_as_text
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:ceab4deb91573d52320819298968dbe022d04add7113aa549bb100e3aa608f46
    case: TC-records-reserved-words-are-written-quoted
    case_digest: 6c17edd3b466edea721bb9e625c11af9fb75c251abb8c617c7eeaf5ae517ea97
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:52Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-flags-stay-booleans-4026e1
    claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:b3c28d2abcf2dc2c1bcb8351a542cc223e377496e2b32f0cb053ff1924c5ea69
    case: TC-records-flags-stay-booleans
    case_digest: c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:38:55Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-no-command-echoes-a-given-value-d1cdc9
    claim: "Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:362e1d6e99bd6d31a9f4c67837269663457558e9a676939a68baf95cee454026
    case: TC-records-no-command-echoes-a-given-value
    case_digest: 76db36b53812aaa597a0496af0ccda5bd97b3a4831a688a83010c85202b2a4a9
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:39:07Z
    duration_seconds: 9
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-file-url-is-checked-like-its-path-730dc1
    claim: "Ran cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts for TC-records-a-file-url-is-checked-like-its-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_file_url_is_checked_like_its_path_and_username_counts
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:cd689ae962cd05e5f6c2a79a96a6033077625d2e16a5ed31fbeffb824c9905a5
    case: TC-records-a-file-url-is-checked-like-its-path
    case_digest: b4d420b5d02270473e1994c3655c45471590e9e5c734a417b40c84829ce62e04
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:39:09Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-d6591f
    claim: "Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:b81218fba6404c14e7e4e4a94ba047d6d3653479e86d11aa041148c36eea69aa
    case: TC-records-a-stored-inbox-entry-folds-printable
    case_digest: 3481f41774e6a2e6e61190d52263a109b722dc7f01f03e6e88a0a0dbcc5b17ab
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:39:12Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-bug-merge-never-shows-a-leaking-path-2fcaa8
    claim: "Ran cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token for TC-records-bug-merge-never-shows-a-leaking-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact bug_merge_never_shows_a_path_holding_a_token
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:60f6d429afe602781716fbfa562358a4b580c8c1f21e7b03870267adb02807f6
    case: TC-records-bug-merge-never-shows-a-leaking-path
    case_digest: 774b3c328cf1ef1475cde9340b9edbf6fa619022f5f2b87d85cbc0e0b0afe416
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:39:15Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-fragment-of-a-stored-control-line-is-refused-a21996
    claim: "Ran cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused for TC-records-a-fragment-of-a-stored-control-line-is-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:43f6bb48e1b356f4e665b9de040fc1e651aa0c4f18c89d048a1972c4a09d0caa
    case: TC-records-a-fragment-of-a-stored-control-line-is-refused
    case_digest: 4e563d17db1240c7916c6bdaedeb23d6066dd601b8d7be50c3774d89b75f7cf1
    criteria: LIFE-26, LIFE-3
    requirements: REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:39:18Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-reporting-timing-prints-stored-text-escaped-31d612
    claim: "Ran cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped for TC-reporting-timing-prints-stored-text-escaped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact timing_prints_a_stored_control_character_escaped
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:7f73730557ec6f3f9a67c080e7ac22b1c9dc865b6713a1937af00c41e8b32a0f
    case: TC-reporting-timing-prints-stored-text-escaped
    case_digest: 9de3b7455166fd781909361c4e210c6815865b378eaa4a6ca4fb4f0250ee3ae5
    criteria: LIFE-10, LIFE-3
    requirements: REQ-reporting-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:39:21Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-tabs-in-values-read-back-dfcbc2
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_given_value_is_refused_and_the_records_stay_readable
    revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
    environment: local-macos-arm64
    content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
    fingerprint: sha256:ecb7f2b9f53c0c9555db9a2f2fad55741b2738629351af63dfac7ca549bc4785
    case: TC-records-tabs-in-values-read-back
    case_digest: 40343f67bca7e9c6c4a8e645d7362d1e3a4196492a31339d2db7fa1afb9d3d22
    criteria: DRIVE-25, DRIVE-26, KNOW-32, LIFE-3
    requirements: REQ-records-001, REQ-records-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:39:27Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
```
