---
kind: session
id: 2026-11-19-leaks-and-output
aliases: [leaks-and-output]
objective: "Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380)"
stage: done
stage_status: archived
iteration: 1
native_change: leaks-and-output
pr: https://github.com/CorvidLabs/chough/pull/407
criteria: [LIFE-26]
requirements: [REQ-records-006, REQ-records-001, REQ-cli-055, REQ-history-006]
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
  - 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-f01a17
  - 2026-11-19-tc-records-the-listed-leak-forms-are-refused-11e401
  - 2026-11-19-tc-records-the-user-name-comes-from-the-account-d63728
  - 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-3ac443
  - 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-7fa583
  - 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-abd587
  - 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-4c7ef7
  - 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-c07f09
  - 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-b053bc
  - 2026-11-19-tc-records-gate-record-redact-redacts-the-close-be202e
  - 2026-11-19-tc-records-every-user-variable-counts-52f91b
  - 2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced-de0b2b
  - 2026-11-19-tc-records-closing-a-session-with-a-stored-leak-redacts-the-31c071
  - 2026-11-19-tc-cli-doctor-names-a-conflict-marker-by-its-seven-characters
  - 2026-11-19-tc-history-session-use-names-a-session-only-when-clean
closed: 2026-11-19
outcome: done
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
- 0xLeif decided "Implementation review of leaks-and-output": "Yes to both" (Claude Code chat answer, 2026-11-19 about 14:08Z, relayed by hc t4c through fable-lead (hc t53), to: send leaks-and-output back once for the cli and history case files, and approve the merge once that is pushed, tester2 has rechecked it and CI is green; at 0871080 tester2's recheck passed and fable-lead's read passed, both checked by fable-lead, with CI to be green before the merge); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change review leaks-and-output as 0xLeif with verdict pass

## What was verified

- 86 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1). - 2a99ea362433; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-f01a17
- Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1). - 3f4f18057d1b; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-the-listed-leak-forms-are-refused-11e401
- Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1). - 5e9cae6e3dc2; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-the-user-name-comes-from-the-account-d63728
- Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1). - 17a0d287a9a1; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-3ac443
- Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1). - 61abfb8907f5; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-7fa583
- Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1). - f94cd2be4db2; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-abd587
- Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1). - b12e8bc69bfd; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-4c7ef7
- Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1). - 081f4a5a7930; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-c07f09
- Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1). - 3dd65eb8dadb; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-b053bc
- Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1). - 70d6b782f968; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-gate-record-redact-redacts-the-close-be202e
- Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1). - 589c49f93f88; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-every-user-variable-counts-52f91b
- Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1). - 457e04749074; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced-de0b2b
- Ran cargo test --locked --test leaks -- --exact closing_a_session_with_a_stored_leak_redacts_the_story_and_says_so for TC-records-closing-a-session-with-a-stored-leak-redacts-the-story: passed (exit 0, tests run 1). - effaf03e15d0; limits: inherited environment variables were not recorded ^2026-11-19-tc-records-closing-a-session-with-a-stored-leak-redacts-the-31c071
- Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-cli-doctor-names-a-conflict-marker-by-its-seven-characters: passed (exit 0, tests run 1). - 0d28c6491078; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-doctor-names-a-conflict-marker-by-its-seven-characters
- Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-history-session-use-names-a-session-only-when-clean: passed (exit 0, tests run 1). - 7f368bc11f41; limits: inherited environment variables were not recorded ^2026-11-19-tc-history-session-use-names-a-session-only-when-clean

## Bugs

None.

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
- 2026-11-19T14:44:38Z · gate-closed · accept → accept · Implementation review of leaks-and-output ^t-f0e75d
  - native_change: leaks-and-output
  - revision: 8594051275394625fcc5f6d009b88d6c662e6bc2
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes to both
    - where: Claude Code chat answer, 2026-11-19 about 14:08Z, relayed by hc t4c through fable-lead (hc t53), to: send leaks-and-output back once for the cli and history case files, and approve the merge once that is pushed, tester2 has rechecked it and CI is green; at 0871080 tester2's recheck passed and fable-lead's read passed, both checked by fable-lead, with CI to be green before the merge
    - recorded_by: chough-fixer-t4g
    - native: specsync change review leaks-and-output
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:44:42Z · advance · accept → done · native:archived ^t-1d24b4
  - native_change: leaks-and-output
  - gate: native:archived
  - revision: 8594051275394625fcc5f6d009b88d6c662e6bc2
  - by: chough-fixer (claude-opus-5-5, hc t4n)

Closed 2026-11-19: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
    environment: local-macos-arm64
    content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
    fingerprint: sha256:2a89d34c93998b107bd6a2c1a16efb39cb2248fa20f3212cc19c1bffc2cac5c3
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:43:14Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
    environment: local-macos-arm64
    content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
    fingerprint: sha256:06c6dc1bbb2226877eb148a9ad3cbf9a44eeee67f696bf87c3d08c21abf176a2
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:43:18Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
    environment: local-macos-arm64
    content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
    fingerprint: sha256:da3ffd08f0fe017505daf821ec7a1bd626954535099661509ecfff8c32e0d5e0
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:43:21Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
    environment: local-macos-arm64
    content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
    fingerprint: sha256:fa313fecde4f7d74f1a242c1db2895e8f1b8cb31c5c0403a895d9f0f640fed86
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:43:25Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
    environment: local-macos-arm64
    content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
    fingerprint: sha256:b869cb4020f15ecec8f26828fc3b60922f41bd4daed7b48d721fa4ca290965bc
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:43:28Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
    environment: local-macos-arm64
    content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
    fingerprint: sha256:ed8c5aa2a6ae2d06cd5fbaa69a149ea6f3d5594fa53dcd0cdef778d0748ce387
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:43:32Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
    environment: local-macos-arm64
    content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
    fingerprint: sha256:59af0af0c5636c65e1e6f1473aebd752fab324bb30ed64eb647362eba209c797
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:43:36Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-2b3212
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
    environment: local-macos-arm64
    content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
    fingerprint: sha256:1fb929a513cf5b0cbacfca53f30fc7886085a0a62cb611b5e0e2198c55046b8b
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:44:23Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-fcdb89
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
    environment: local-macos-arm64
    content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
    fingerprint: sha256:e0f53f084e9dab20103fb332857ace9c1a65518ad3565c2dcfc39522bbff270a
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:44:25Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-42e32d
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
    environment: local-macos-arm64
    content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
    fingerprint: sha256:4eee4a4decb2481e90c0fbace1b062c5b44285af361492d3a257c1a2fca9003a
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:44:27Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-ad0790
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
    environment: local-macos-arm64
    content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
    fingerprint: sha256:21bbac3fea453ead7927d6b7c3e4622b6894d2c806373d1552bb2a3f5515e3f1
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:44:29Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-956b58
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
    environment: local-macos-arm64
    content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
    fingerprint: sha256:52d571122392d36f11261424f941fe7459cea8a52cf6583d321914d520593af4
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:44:31Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-a69367
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
    environment: local-macos-arm64
    content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
    fingerprint: sha256:9f1e3c564443b618b460b911051367eb860437953fb53d9eb761e670324e62b4
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:44:34Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-663602
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
    environment: local-macos-arm64
    content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
    fingerprint: sha256:29e2637120830975f6ea334f9b49db52c9b8aa70f01d065eb3d4169ae6da8083
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T02:44:36Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-41951b
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
    environment: local-macos-arm64
    content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
    fingerprint: sha256:925231ad736b4054ce739fbcb197e7c3c96c00708ec82e48632008518f4f5556
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:04:10Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-b02b4e
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
    environment: local-macos-arm64
    content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
    fingerprint: sha256:1e92929dc3f24dedfa2adee498427c4a86cd129508a9340c124c167fc9e08171
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:04:13Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-6ffb61
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
    environment: local-macos-arm64
    content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
    fingerprint: sha256:4ed84268deb32eb5e195c108038f26d0ee54bad8a3de08a2a60b88ea62932a74
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:04:15Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-338f9e
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
    environment: local-macos-arm64
    content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
    fingerprint: sha256:d1f66481b6973e53cc1aada2dbc62389bcc00ddae8d8b8a6a1f66665c16d66fc
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:04:17Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-b3a4d6
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
    environment: local-macos-arm64
    content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
    fingerprint: sha256:0dac13c68005aa163d354909073555bf37e256de633df920734573353177e55d
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:04:20Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-4ef830
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
    environment: local-macos-arm64
    content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
    fingerprint: sha256:068a93c4509400d2a6c457003460938382f6d6bd367b785ad2f10c8ed6cc700f
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:04:23Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-21dc1c
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
    environment: local-macos-arm64
    content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
    fingerprint: sha256:c3a52f770507f6dd2759b0f5613bc54cb2edb731cb9792732e40d811d411893c
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:04:25Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean
    claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean
    revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
    environment: local-macos-arm64
    content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
    fingerprint: sha256:003d81db53f589893553aeca8ef709da14da3204275cf2a1dd35a1fef6879f56
    case: TC-records-errors-show-a-given-value-only-when-clean
    case_digest: 1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:04:28Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-9cf90e
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: ccb57abff508d187593c07849872a17c37036014
    environment: local-macos-arm64
    content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
    fingerprint: sha256:f3a3f5627ed39263d8445706219891dd7d293e6f96a924b46f7beeb1036580d3
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:38:44Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-14cad5
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: ccb57abff508d187593c07849872a17c37036014
    environment: local-macos-arm64
    content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
    fingerprint: sha256:45c4b675a8c597753d4dc1c01ea76cb533b2969f07ed6f48122653327a8241e2
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:38:47Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-1d690a
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: ccb57abff508d187593c07849872a17c37036014
    environment: local-macos-arm64
    content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
    fingerprint: sha256:bbbca574df0a5b7bc27f415ff1ba7125116c1054d38ba0d7262b046e42398521
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:38:49Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-9ac275
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: ccb57abff508d187593c07849872a17c37036014
    environment: local-macos-arm64
    content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
    fingerprint: sha256:4e75dc2a3d14ba1477731249c3bef26b4c7972ac02138989c5af01d4de2e76a3
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:38:51Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-00f0cf
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: ccb57abff508d187593c07849872a17c37036014
    environment: local-macos-arm64
    content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
    fingerprint: sha256:b3abe21bf5740fa33391a5b96926fa3a5479a65cb0caf92b12338b712bf4b394
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:38:54Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-cc18d8
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: ccb57abff508d187593c07849872a17c37036014
    environment: local-macos-arm64
    content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
    fingerprint: sha256:59487c7e7a7015bf725ebb609c40133f59b7c77dca05bf06f77e0496da6a88d5
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:38:57Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-4da2d0
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: ccb57abff508d187593c07849872a17c37036014
    environment: local-macos-arm64
    content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
    fingerprint: sha256:2b018f6aa686755dae58ed6610b1cfcbe5ca40ce13d645ec974fde6759df8aaf
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:39:00Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-97dada
    claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean
    revision: ccb57abff508d187593c07849872a17c37036014
    environment: local-macos-arm64
    content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
    fingerprint: sha256:b3367108614fe34ee5ebbb2a1163516ac641da9b587385d59be77dabe169f83b
    case: TC-records-errors-show-a-given-value-only-when-clean
    case_digest: 1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T03:39:02Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-8ade06
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:608f2c6b115dc27dd4ad77597408526792030b199090af76653aea457c8d4792
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:15:56Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-750e65
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:ede5fb32f3b36819330e770c2716aca477e347af91388f79a8210c0180b6d2ee
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:15:59Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-af64ff
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:04e4d5a9f4307b7ecf0788fd33ca8239a88c9a6ee513b0987f46e809f6f350e7
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:16:01Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-dab931
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:110c69671dd2eb84adee5f9dcb069c6a0f4bb6fcf15b42add8dba4ba2a6db2b6
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:16:04Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-134730
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:e33471e1cc8954933b904b42b35e649a0d53b9494d07cca1bed0a89ba1417627
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:16:06Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-83abb7
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:003d4a13a4fd3c7bd705a57d7da87554c01fbf04ca47e5e80e30ca93dbc5f81d
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:16:08Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-f05049
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:716567b226fedf140347ad1a48a3f4ab19145a44c2e4e67ea86a8f7843c9d1a5
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:16:11Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-09351b
    claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:3c86fa71a293fe4f379d37cd0cf295d5e3926b4f54cb0b4099d85c3581c73532
    case: TC-records-errors-show-a-given-value-only-when-clean
    case_digest: 1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:16:14Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak
    claim: "Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:ea9a3e82badc070829009b02464158adf827e2b7e20233a03e8e9489a5ed5ec8
    case: TC-records-a-redacting-save-replaces-a-stored-leak
    case_digest: 44004ef165e948a2f6f1a5c6eeae7ffffd09c4656ed449a43d7454aebe4eb288
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:16:16Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close
    claim: "Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs
    revision: b84728117b64a7ed3ac049540fde93c0228c58cb
    environment: local-macos-arm64
    content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
    fingerprint: sha256:130c896e6d6eecefe18c38e5bf19d4cbb37f81d69041d1cfde5ae51d1018b19d
    case: TC-records-gate-record-redact-redacts-the-close
    case_digest: 8d7158e0728006a4500fd90d93739207c92f10d0a7b4237b003814986f37dae2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:16:19Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-ae33e9
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:eabfcd4357f15516aa5727bfd0d21a5cc0840b553e23d27c332cdf68969e88dc
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:22Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-b4b3df
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:ede5fb32f3b36819330e770c2716aca477e347af91388f79a8210c0180b6d2ee
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:25Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-9fdd5c
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:8ca4ba0b5ba5a47e46cdd14d42319a34a9300436fded6f02ba969d5b623c0fb3
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:27Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-ecf999
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:f07645f2cf84676aeb976fd7f5c8aa35a737d2bf9c0cf25a4e356cf588904dce
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:30Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-1cc08b
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:504d6f5fe1d94b2f1214831e1cb0c336c5803fdf05f99f5d5385e780b9151525
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:33Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-96cdb0
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:246cce0b2e3a09c7f41e9826ef647fa146cdeaa20b441720a918680b20c9f742
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:35Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-07c3c7
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:cff3d6d9975c5b3f0e3579adaed515565ac69a2eaf552794b9a63fc367aafb41
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:38Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-ba13e7
    claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:cd20e1a63e620df3a5c71b1d2e748bea44b8ed993a87006c35b283ef59e28723
    case: TC-records-errors-show-a-given-value-only-when-clean
    case_digest: 1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:40Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-e4bc49
    claim: "Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:c4e2b838c31970252de65dd26a77cc0f764bfd2cbf825cdde788e75458d585b1
    case: TC-records-a-redacting-save-replaces-a-stored-leak
    case_digest: 44004ef165e948a2f6f1a5c6eeae7ffffd09c4656ed449a43d7454aebe4eb288
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:43Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close-f5cf61
    claim: "Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs
    revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
    environment: local-macos-arm64
    content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
    fingerprint: sha256:11046964b4867cf6ce7612c6d1a1e9b9106616e19e292cd334f35b67640e00de
    case: TC-records-gate-record-redact-redacts-the-close
    case_digest: 8d7158e0728006a4500fd90d93739207c92f10d0a7b4237b003814986f37dae2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:31:45Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-bdc8f4
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:4728acc3f822cb40d5cdf6313bb18d044c0ff8a32e279251621b06bde8a311e2
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:11Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-a05a13
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:bb1aa4251f9143251376e254f108adb9b6afa58911dac0f85066abcdd1861efb
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:16Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-ee5caf
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:73a52a2865b176896b15352db591a422cb4ab122930b1f3b98c7df087d3d4bbb
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:20Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-f33dcf
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:c41d6a13aff1696ba2da50464a8863125370073c93973d7c4f099c61447757a9
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:24Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-9790cd
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:be99c9b37b90cd33ead2412435af86974dbaffe95bde28dfddb92a8ed5382907
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:28Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-507693
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:386267b4d443bdd97ed0f684c43f90bbe9a5c10e291fbf459013ca87a3f8df9c
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:33Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-331ecd
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:d280142d6d64f78596520ec6c381b6a6c2dd383841214e8ce7aa7ce26b3d5e78
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:39Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-460113
    claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:a3173fe28f5ad795e039a319a51c596fa1b057c84a3928a8c5c17277588a5d37
    case: TC-records-errors-show-a-given-value-only-when-clean
    case_digest: 1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:43Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-ebff3b
    claim: "Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:a50ed3532d5099bee1c28b8088f4388635641e80fb746b28cbfc443e260c1f29
    case: TC-records-a-redacting-save-replaces-a-stored-leak
    case_digest: 44004ef165e948a2f6f1a5c6eeae7ffffd09c4656ed449a43d7454aebe4eb288
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:47Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close-27d0e1
    claim: "Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:3b7bb031c5bfd1e9097ac41dbfa23b0cb9b40c5ab5ae5b254c7719609686ffff
    case: TC-records-gate-record-redact-redacts-the-close
    case_digest: 8d7158e0728006a4500fd90d93739207c92f10d0a7b4237b003814986f37dae2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:53Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-every-user-variable-counts
    claim: "Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts
    revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
    environment: local-macos-arm64
    content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
    fingerprint: sha256:2d18a3ec3f0a1e749bd59b19a29147b90e46525824d13208000edb3d1feca402
    case: TC-records-every-user-variable-counts
    case_digest: 01512ff4c0f2dbaa33be4cbaaddd3968f785c41c406aaf70e1db62e130091dc2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:39:57Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-88fa27
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:996afb13f456f8c888e5d90fee5d1b20ca57a6831b6c5744e8f5bf652ae8330e
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:32Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-8617d8
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:0a2595f48dd6b114185b36d9d1112bdbbaef1a7ad803af202f19ddeafb02c041
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:36Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-f76edf
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:26b9b92df7cc57280d1b3b1f0d1b13356adf44a74a20102ce3f58c0c78150283
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:39Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-0ceeb1
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:091117cbfc52c429f5087af84318d38e1b8e4822bbd8dc568854b5d3d44f86cb
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:43Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-38847b
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:422d0c227543587d1e2f05a2e5365f7a9982d78bc8652fb0ca8a1e09f0df1c95
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:47Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-c9b1be
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:498a43f056db52ced776d69c7fcbeca600ce623d3a046582259a983b45546e6b
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:52Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-8fb099
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:66d46d639b08c4b79bab8cf459f122389d6691a75867e8f6fd4db9a1fea7559e
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:53:56Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-6992a9
    claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:35978fe020dea06e2d300b1bbf72a0dd7afc63f28b002eb14671f071e4b80d85
    case: TC-records-errors-show-a-given-value-only-when-clean
    case_digest: 1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:54:00Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-3ad8cc
    claim: "Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:8bc87b03bb3d2cb2529c2abbe359ee11bf643d4ac6aa6dc9be3a4ff81dc29a50
    case: TC-records-a-redacting-save-replaces-a-stored-leak
    case_digest: 44004ef165e948a2f6f1a5c6eeae7ffffd09c4656ed449a43d7454aebe4eb288
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:54:09Z
    duration_seconds: 5
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close-fbe4ff
    claim: "Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:4ae6dc65a3f0b4fc0ff220b24dca803e18f9cda1fa8a73a37a63e20468d25747
    case: TC-records-gate-record-redact-redacts-the-close
    case_digest: 8d7158e0728006a4500fd90d93739207c92f10d0a7b4237b003814986f37dae2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:54:14Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-every-user-variable-counts-f7e32d
    claim: "Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:3aa5b12bd6054f2b8094be29daffe2cbd26a4037f57986dbc6ec711784a681d3
    case: TC-records-every-user-variable-counts
    case_digest: 01512ff4c0f2dbaa33be4cbaaddd3968f785c41c406aaf70e1db62e130091dc2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:54:18Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced
    claim: "Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced
    revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
    environment: local-macos-arm64
    content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
    fingerprint: sha256:ed46ca7f1aa2c8b484d24bcd22e4307387927e9f87e0608be890529aa106e877
    case: TC-records-the-redaction-count-is-what-the-save-replaced
    case_digest: 64420d75b9474d13ef66f57850ee92362ed4092a98b5fd39f02439bb2ed6110d
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T04:54:23Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-26d52c
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:8be5f7897ecc4dcdf16f4c980e8445089decb26d16053f5c54b8c6d66b5fb326
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:22Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-723c21
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:c1faeb0317875664f13bd0bb020f7ac085bb038ff4275d542021bc59dd12e778
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:27Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-d07c05
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:e320605e92f0514f9cbe3185694ec7422601c8765b2a855f1dcf7b8a1c4c0af4
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:30Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-f4c131
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:64095815ffd1d69cce5d68e5abc320450829b188644b78aef976a3278006c694
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:35Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-8d2ae0
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:af21d6493dd0feb1f2faa5a94032248c5d62913dbce9469cdd96d0ef7d1e543f
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:39Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-522a18
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:a320c02a3a0df1e8383752c7ab87825cf2e242472b39c5f190be1f2d0682b875
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:43Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-baf262
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:92eca1becfbaf5f01c1b9aa6932a1341310b7e554bd9ad7a217fd117b18adf63
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:48Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-e0e21c
    claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:78febcf645a6179ffc8b0c43db86fc76d888f1129162deabb1c4ba1ae02cac35
    case: TC-records-errors-show-a-given-value-only-when-clean
    case_digest: 1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:52Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-65583d
    claim: "Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:1e217bb70553d0565fd60af60a454587201856a609bb8267cda3f9226db8a7ed
    case: TC-records-a-redacting-save-replaces-a-stored-leak
    case_digest: 44004ef165e948a2f6f1a5c6eeae7ffffd09c4656ed449a43d7454aebe4eb288
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:11:57Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close-46334d
    claim: "Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:a95d2158dbac381abf6910d2f4f012980eafd955e87ad4494f673b7e2f8f556e
    case: TC-records-gate-record-redact-redacts-the-close
    case_digest: 8d7158e0728006a4500fd90d93739207c92f10d0a7b4237b003814986f37dae2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:12:01Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-every-user-variable-counts-7db994
    claim: "Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:e445e7d5cd4f424ac69f09471a71a478b27559507a74face12840b67cbf89d34
    case: TC-records-every-user-variable-counts
    case_digest: 01512ff4c0f2dbaa33be4cbaaddd3968f785c41c406aaf70e1db62e130091dc2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:12:05Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced-1c5e27
    claim: "Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:1b0c44136b75f71fcd540bb621e14020faa7dc16e898588a2e8c581d15450e52
    case: TC-records-the-redaction-count-is-what-the-save-replaced
    case_digest: 64420d75b9474d13ef66f57850ee92362ed4092a98b5fd39f02439bb2ed6110d
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:12:10Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-closing-a-session-with-a-stored-leak-redacts-the
    claim: "Ran cargo test --locked --test leaks -- --exact closing_a_session_with_a_stored_leak_redacts_the_story_and_says_so for TC-records-closing-a-session-with-a-stored-leak-redacts-the-story: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact closing_a_session_with_a_stored_leak_redacts_the_story_and_says_so
    revision: 908d3ace4545d270125d13706f6b012cdc276e21
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:584db3e0a6df0fa017dfd1ca018672c303bd73821ffdde5cd6c9237524491e54
    case: TC-records-closing-a-session-with-a-stored-leak-redacts-the-story
    case_digest: 7e8959d58ffeab6ab8d90498d401a853c4f71612fb5e21b0ade9e43e1c9f9f1e
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:12:13Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded-f01a17
    claim: "Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:2a99ea3624336b1e39d31b3ea9cf69e3f44755d00d9fedabdb1de37a9525438f
    case: TC-records-an-answer-is-leak-checked-and-bounded
    case_digest: 7f54a2cb18baf841f1457f7db329ea7243c63c29f554ef812b876b89cfd2852b
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:08Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-listed-leak-forms-are-refused-11e401
    claim: "Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:3f4f18057d1b4a77b707161c64337a26df4f1ca9ca810fe6c7375b107eb20a75
    case: TC-records-the-listed-leak-forms-are-refused
    case_digest: e3434ea14829075b3529ca6a1192d4ab045c58637f1109d22566bab2931531b1
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:11Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-user-name-comes-from-the-account-d63728
    claim: "Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:5e9cae6e3dc2d8f5a3c042177b508f34af84daf4a98bc45728337df3571e5bfa
    case: TC-records-the-user-name-comes-from-the-account
    case_digest: 8973a6a08e273e42441c965a9310563cfcf78774ec6d7521d01cf833c4ea58f3
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:13Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters-3ac443
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:17a0d287a9a1aea52debdd3ebd98db2367e39f9f3cbe56d0391e3e13f117729d
    case: TC-records-a-conflict-marker-is-named-by-its-seven-characters
    case_digest: 49d014c7b709f412cfe74999bc57a0bf696e21a599920c2f180164aa1a859ad0
    criteria: LIFE-26
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:15Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-7fa583
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:61abfb8907f5a12d4e0612d0768233fa662b6d0fbcbd1dc60eda53d5fcb622c7
    case: TC-records-errors-show-a-session-name-only-when-clean
    case_digest: 5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:18Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-abd587
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:f94cd2be4db21d3c661d2639ad0baadc69fc646a071ae158592490e6741b9cec
    case: TC-records-errors-show-a-bug-reference-only-when-clean
    case_digest: b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:20Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-4c7ef7
    claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:b12e8bc69bfd66dbea4caa33b04a048610717e29e9fcf5905921fdcfced8932c
    case: TC-records-errors-show-a-decided-by-value-only-when-clean
    case_digest: aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:23Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-c07f09
    claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:081f4a5a7930c5d9ea49f694121ec3d762ef04ed3f67c1bb7bc66350c38943b7
    case: TC-records-errors-show-a-given-value-only-when-clean
    case_digest: 1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:26Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-b053bc
    claim: "Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:3dd65eb8dadbdb534620fc3b511de29ddda62f2682170be20798909e3b0a150a
    case: TC-records-a-redacting-save-replaces-a-stored-leak
    case_digest: 44004ef165e948a2f6f1a5c6eeae7ffffd09c4656ed449a43d7454aebe4eb288
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:28Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-gate-record-redact-redacts-the-close-be202e
    claim: "Ran cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs for TC-records-gate-record-redact-redacts-the-close: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test humans -- --exact gate_record_redact_redacts_in_the_close_it_runs
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:70d6b782f968119f0eae2cdc5797e8439979ae16db29bd685138054adab3b517
    case: TC-records-gate-record-redact-redacts-the-close
    case_digest: 8d7158e0728006a4500fd90d93739207c92f10d0a7b4237b003814986f37dae2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:31Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-every-user-variable-counts-52f91b
    claim: "Ran cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts for TC-records-every-user-variable-counts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact every_user_variable_that_is_set_counts
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:589c49f93f8897575ee3fc361094ff2b95ffa14dc3dc3bb8b52244a66eb28840
    case: TC-records-every-user-variable-counts
    case_digest: 01512ff4c0f2dbaa33be4cbaaddd3968f785c41c406aaf70e1db62e130091dc2
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:33Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced-de0b2b
    claim: "Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:457e04749074e576c5fd335e26be09b5bc5a8ede248c44cb90156d54a3e3f3c9
    case: TC-records-the-redaction-count-is-what-the-save-replaced
    case_digest: 64420d75b9474d13ef66f57850ee92362ed4092a98b5fd39f02439bb2ed6110d
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:36Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-closing-a-session-with-a-stored-leak-redacts-the-31c071
    claim: "Ran cargo test --locked --test leaks -- --exact closing_a_session_with_a_stored_leak_redacts_the_story_and_says_so for TC-records-closing-a-session-with-a-stored-leak-redacts-the-story: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact closing_a_session_with_a_stored_leak_redacts_the_story_and_says_so
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:effaf03e15d0d9acd8223baa911fe2c7b0d944f285448320ba4fdacb019a4bbf
    case: TC-records-closing-a-session-with-a-stored-leak-redacts-the-story
    case_digest: 7e8959d58ffeab6ab8d90498d401a853c4f71612fb5e21b0ade9e43e1c9f9f1e
    criteria: LIFE-26
    requirements: REQ-records-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:38Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-doctor-names-a-conflict-marker-by-its-seven-characters
    claim: "Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-cli-doctor-names-a-conflict-marker-by-its-seven-characters: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:0d28c64910782132b88720c1d90e9a3bb1b47c633073bd30c14db40db28fb312
    case: TC-cli-doctor-names-a-conflict-marker-by-its-seven-characters
    case_digest: ef855cbfb79fbe9906fd35608afd6f1cabadf96b774b05c74d1f2847c54dfdd6
    criteria: LIFE-26
    requirements: REQ-cli-055
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:41Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-session-use-names-a-session-only-when-clean
    claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-history-session-use-names-a-session-only-when-clean: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean
    revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
    environment: local-macos-arm64
    content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
    fingerprint: sha256:7f368bc11f4122d410f2499e81797a72661f113d07fa1bb553910475f601d6d3
    case: TC-history-session-use-names-a-session-only-when-clean
    case_digest: a547df812106cb5ac65af3b5c0d2597f6d7e57607acf4932f6a01cbc9f843f80
    criteria: LIFE-26
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:23:44Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
```
