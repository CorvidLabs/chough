---
kind: session
id: 2026-11-19-leaks-and-output
aliases: [leaks-and-output]
objective: "Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: leaks-and-output
pr: ""
criteria: [LIFE-26]
requirements: [REQ-records-006, REQ-records-001, REQ-cli-055, REQ-history-006]
cases: []
design: []
files: []
observations:
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-2b3212
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-fcdb89
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-42e32d
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-ad0790
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-956b58
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-a69367
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-663602
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-41951b
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-b02b4e
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-6ffb61
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-338f9e
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-b3a4d6
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-4ef830
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-21dc1c
  - 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-9cf90e
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-14cad5
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-1d690a
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-9ac275
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-00f0cf
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-cc18d8
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-4da2d0
  - 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-97dada
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-8ade06
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-750e65
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-af64ff
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-dab931
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-134730
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-83abb7
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-f05049
  - 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-09351b
  - 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak
  - 2026-11-19-tc-records-gate-record-redact-redacts-the-close
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-ae33e9
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-b4b3df
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-9fdd5c
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-ecf999
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-1cc08b
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-96cdb0
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-07c3c7
  - 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-ba13e7
  - 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-e4bc49
  - 2026-11-19-tc-records-gate-record-redact-redacts-the-close-f5cf61
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-bdc8f4
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-a05a13
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-ee5caf
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-f33dcf
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-9790cd
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-507693
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-331ecd
  - 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-460113
  - 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-ebff3b
  - 2026-11-19-tc-records-gate-record-redact-redacts-the-close-27d0e1
  - 2026-11-19-tc-records-every-user-variable-counts
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-88fa27
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-8617d8
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-f76edf
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-0ceeb1
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-38847b
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-c9b1be
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-8fb099
  - 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-6992a9
  - 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-3ad8cc
  - 2026-11-19-tc-records-gate-record-redact-redacts-the-close-fbe4ff
  - 2026-11-19-tc-records-every-user-variable-counts-f7e32d
  - 2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-26d52c
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-723c21
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-d07c05
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-f4c131
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-8d2ae0
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-522a18
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-baf262
  - 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-e0e21c
  - 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-65583d
  - 2026-11-19-tc-records-gate-record-redact-redacts-the-close-46334d
  - 2026-11-19-tc-records-every-user-variable-counts-7db994
  - 2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced-1c5e27
  - 2026-11-19-tc-records-closing-a-session-with-a-stored-leak-redacts-the
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of leaks-and-output
    holds: accept
    drives: review
    opened: 2026-11-19T02:44:55Z
run_consents: []
next: Continue accept
---
# Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380)

## Goal

Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of leaks-and-output": "Yes, as each issue states … approved. Errors name the field and the kind, never the offending text … Show it only if clean … approved. The leak guard adds all the listed forms" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: the leaks-and-output batch of the bug round, one native change for #364, #365, #367, #368, #379 and #380. For #364, #365, #379 and #380 he approved each issue's stated expected behaviour ('Yes, as each issue states'). For #367 his decision comment on the issue, with 'Show it only if clean': the value is shown only when it holds no leak and no control character, otherwise the error names the field and what it holds. For #368 his decision comment on the issue: all the listed forms, the guard on session new --module, and the evidence add --redact count); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve leaks-and-output as 0xLeif
- 0xLeif decided "Re-approval of leaks-and-output for the #379 user-name sentence": "Fix the sentence: check both" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, on fable-lead's L1 review finding 1: REQ-records-006 now says the local user name is checked as the account's name and also as USER or LOGNAME when they are set, as the code does; nothing else in the delta changes); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve leaks-and-output as 0xLeif
- 0xLeif decided "Re-approval of leaks-and-output for the narrowed #379 user-name rule": "Narrow it" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, on tester2's finding C: the user name counts as a leak only inside an absolute path, after ~, or as user@host, and never when the name is root; one more sentence in REQ-records-006 beside his 'Fix the sentence: check both', and its acceptance criterion); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve leaks-and-output as 0xLeif
- 0xLeif decided "Send back leaks-and-output for the cli and history case files, run by the fixer": "The fixer runs them, on my word" (Claude Code chat answer, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 14:17Z); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of leaks-and-output for the cli and history case files": "Yes to both" (Claude Code chat answer, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 14:08Z, to: may leaks-and-output be sent back once for the two case files .chough/cases/cli.leaks-and-output.cases.toml and .chough/cases/history.leaks-and-output.cases.toml, no code and no sentence change, and is the merge approved once that is pushed, tester2 has rechecked it and CI is green); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve leaks-and-output as 0xLeif

## What was verified

- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean.md)
- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-2b3212.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused-fcdb89.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account-42e32d.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-ad0790.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-956b58.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-a69367.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-663602.md)
- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-41951b.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused-b02b4e.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account-6ffb61.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-338f9e.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-b3a4d6.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-4ef830.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-21dc1c.md)
- [Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-given-value-only-when-clean.md)
- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-9cf90e.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused-14cad5.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account-1d690a.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-9ac275.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-00f0cf.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-cc18d8.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-4da2d0.md)
- [Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-97dada.md)
- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-8ade06.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused-750e65.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account-af64ff.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-dab931.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-134730.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-83abb7.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-f05049.md)
- [Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-09351b.md)
- [Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak.md)
- [Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-gate-record-redact-redacts-the-close.md)
- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-ae33e9.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused-b4b3df.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account-9fdd5c.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-ecf999.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-1cc08b.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-96cdb0.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-07c3c7.md)
- [Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-ba13e7.md)
- [Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-e4bc49.md)
- [Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-gate-record-redact-redacts-the-close-f5cf61.md)
- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-bdc8f4.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused-a05a13.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account-ee5caf.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-f33dcf.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-9790cd.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-507693.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-331ecd.md)
- [Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-460113.md)
- [Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-ebff3b.md)
- [Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-gate-record-redact-redacts-the-close-27d0e1.md)
- [Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-every-user-variable-counts.md)
- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-88fa27.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused-8617d8.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account-f76edf.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-0ceeb1.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-38847b.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-c9b1be.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-8fb099.md)
- [Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-6992a9.md)
- [Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-3ad8cc.md)
- [Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-gate-record-redact-redacts-the-close-fbe4ff.md)
- [Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-every-user-variable-counts-f7e32d.md)
- [Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced.md)
- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-26d52c.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused-723c21.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account-d07c05.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-f4c131.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-8d2ae0.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-522a18.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-baf262.md)
- [Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-e0e21c.md)
- [Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-65583d.md)
- [Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-gate-record-redact-redacts-the-close-46334d.md)
- [Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-every-user-variable-counts-7db994.md)
- [Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced-1c5e27.md)
- [Ran cargo test --locked --test leaks -- --exact closing_a_session_with_a_stored_leak_redacts_the_story_and_says_so for TC-records-closing-a-session-with-a-stored-leak-redacts-the-story: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-closing-a-session-with-a-stored-leak-redacts-the.md)

## Open threads

## Timeline

- 2026-11-19T01:56:48Z · start · intent → intent · Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380) ^t-c05e30
  - revision: bd77bbd15f0e373776192a7d058a09c70fdbc00e
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T01:56:48Z · advance · intent → design ·  ^t-eb9229
  - revision: bd77bbd15f0e373776192a7d058a09c70fdbc00e
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T01:56:48Z · skip · design → plan · Leak checks and error wording; nothing visual or interactive changes ^t-7eaa07
  - revision: bd77bbd15f0e373776192a7d058a09c70fdbc00e
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:02:33Z · gate-opened · plan → plan · Scope approval of leaks-and-output ^t-284d0f
  - native_change: leaks-and-output
  - revision: bd77bbd15f0e373776192a7d058a09c70fdbc00e
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:02:33Z · gate-closed · plan → plan · Scope approval of leaks-and-output ^t-9a77ac
  - native_change: leaks-and-output
  - revision: bd77bbd15f0e373776192a7d058a09c70fdbc00e
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, as each issue states … approved. Errors name the field and the kind, never the offending text … Show it only if clean … approved. The leak guard adds all the listed forms
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: the leaks-and-output batch of the bug round, one native change for #364, #365, #367, #368, #379 and #380. For #364, #365, #379 and #380 he approved each issue's stated expected behaviour ('Yes, as each issue states'). For #367 his decision comment on the issue, with 'Show it only if clean': the value is shown only when it holds no leak and no control character, otherwise the error names the field and what it holds. For #368 his decision comment on the issue: all the listed forms, the guard on session new --module, and the evidence add --redact count
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve leaks-and-output
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:02:34Z · advance · plan → build · native:approved ^t-5a94a4
  - native_change: leaks-and-output
  - gate: native:approved
  - revision: bd77bbd15f0e373776192a7d058a09c70fdbc00e
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:44:53Z · direct-native-write · build → build · leaks-and-output's lifecycle records changed outside Chough: state.json changed, verification.json removed, verification-attempts.json removed; state verifying → approved ^t-5d1ea4
  - native_change: leaks-and-output
  - revision: dcddb723c6577e567ad2104cf5f391a5ae0a78aa
  - direct_write:
    - change: leaks-and-output
    - files:
      - state.json: changed
      - verification.json: removed
      - verification-attempts.json: removed
    - before_state: verifying
    - after_state: approved
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:44:53Z · direct-native-write · build → build · leaks-and-output's lifecycle records changed outside Chough: state.json changed, verification.json removed, verification-attempts.json removed; state verifying → approved ^t-a78bf7
  - native_change: leaks-and-output
  - revision: dcddb723c6577e567ad2104cf5f391a5ae0a78aa
  - direct_write:
    - change: leaks-and-output
    - files:
      - state.json: changed
      - verification.json: removed
      - verification-attempts.json: removed
    - before_state: verifying
    - after_state: approved
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:44:53Z · advance · build → test · native:verifying ^t-9df829
  - native_change: leaks-and-output
  - gate: native:verifying
  - revision: dcddb723c6577e567ad2104cf5f391a5ae0a78aa
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:44:54Z · advance · test → accept · native:verifying ^t-cce0ce
  - native_change: leaks-and-output
  - gate: native:verifying
  - revision: dcddb723c6577e567ad2104cf5f391a5ae0a78aa
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:44:54Z · gate-opened · accept → accept · Implementation review of leaks-and-output ^t-294c3f
  - native_change: leaks-and-output
  - revision: dcddb723c6577e567ad2104cf5f391a5ae0a78aa
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T03:03:44Z · gate-opened · accept → accept · Re-approval of leaks-and-output for the #379 user-name sentence ^t-7014d4
  - native_change: leaks-and-output
  - revision: 231dbc22982b592f268dd2e579ecde4ca8de529b
  - holds: accept
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T03:03:44Z · gate-closed · accept → accept · Re-approval of leaks-and-output for the #379 user-name sentence ^t-a57227
  - native_change: leaks-and-output
  - revision: 231dbc22982b592f268dd2e579ecde4ca8de529b
  - relayed_by:
    - decided_by: 0xLeif
    - words: Fix the sentence: check both
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, on fable-lead's L1 review finding 1: REQ-records-006 now says the local user name is checked as the account's name and also as USER or LOGNAME when they are set, as the code does; nothing else in the delta changes
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve leaks-and-output
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T03:53:33Z · gate-opened · accept → accept · Re-approval of leaks-and-output for the narrowed #379 user-name rule ^t-1c4f4b
  - native_change: leaks-and-output
  - revision: 932092439117fcd7743d5b1465db6a597c831eb7
  - holds: accept
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T03:53:33Z · gate-closed · accept → accept · Re-approval of leaks-and-output for the narrowed #379 user-name rule ^t-662453
  - native_change: leaks-and-output
  - revision: 932092439117fcd7743d5b1465db6a597c831eb7
  - relayed_by:
    - decided_by: 0xLeif
    - words: Narrow it
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, on tester2's finding C: the user name counts as a leak only inside an absolute path, after ~, or as user@host, and never when the name is root; one more sentence in REQ-records-006 beside his 'Fix the sentence: check both', and its acceptance criterion
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve leaks-and-output
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:22:07Z · gate-opened · accept → accept · Send back leaks-and-output for the cli and history case files, run by the fixer ^t-8d9754
  - native_change: leaks-and-output
  - revision: 9a81c859ae449f9da541cab3538d318de411a1ee
  - holds: accept
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:22:07Z · gate-closed · accept → accept · Send back leaks-and-output for the cli and history case files, run by the fixer ^t-342d7f
  - native_change: leaks-and-output
  - revision: 9a81c859ae449f9da541cab3538d318de411a1ee
  - relayed_by:
    - decided_by: 0xLeif
    - words: The fixer runs them, on my word
    - where: Claude Code chat answer, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 14:17Z
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:22:51Z · gate-opened · accept → accept · Re-approval of leaks-and-output for the cli and history case files ^t-c6d19c
  - native_change: leaks-and-output
  - revision: 9a81c859ae449f9da541cab3538d318de411a1ee
  - holds: accept
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:22:52Z · direct-native-write · accept → accept · leaks-and-output's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-9c1899
  - native_change: leaks-and-output
  - revision: 9a81c859ae449f9da541cab3538d318de411a1ee
  - direct_write:
    - change: leaks-and-output
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:22:52Z · gate-closed · accept → accept · Re-approval of leaks-and-output for the cli and history case files ^t-086d27
  - native_change: leaks-and-output
  - revision: 9a81c859ae449f9da541cab3538d318de411a1ee
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes to both
    - where: Claude Code chat answer, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 14:08Z, to: may leaks-and-output be sent back once for the two case files .chough/cases/cli.leaks-and-output.cases.toml and .chough/cases/history.leaks-and-output.cases.toml, no code and no sentence change, and is the merge approved once that is pushed, tester2 has rechecked it and CI is green
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve leaks-and-output
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
