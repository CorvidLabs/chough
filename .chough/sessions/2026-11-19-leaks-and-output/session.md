---
kind: session
id: 2026-11-19-leaks-and-output
aliases: [leaks-and-output]
objective: "Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380)"
intent_scope: ""
modules: []
stage: build
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
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue build
---
# Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380)

## Goal

Leak and output batch: --answer is leak-checked and bounded (#364), conflict findings name only the marker (#365), errors show a value only when it holds no leak and no control characters (#367), the leak guard adds the listed forms (#368), the user name comes from the account (#379), and gate record says Error once (#380)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of leaks-and-output": "Yes, as each issue states … approved. Errors name the field and the kind, never the offending text … Show it only if clean … approved. The leak guard adds all the listed forms" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: the leaks-and-output batch of the bug round, one native change for #364, #365, #367, #368, #379 and #380. For #364, #365, #379 and #380 he approved each issue's stated expected behaviour ('Yes, as each issue states'). For #367 his decision comment on the issue, with 'Show it only if clean': the value is shown only when it holds no leak and no control character, otherwise the error names the field and what it holds. For #368 his decision comment on the issue: all the listed forms, the guard on session new --module, and the evidence add --redact count); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve leaks-and-output as 0xLeif

## What was verified

- [Ran cargo test --locked --test leaks -- --exact an_answer_is_leak_checked_redacted_and_bounded for TC-records-an-answer-is-leak-checked-and-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-an-answer-is-leak-checked-and-bounded.md)
- [Ran cargo test --locked --test leaks -- --exact the_listed_leak_forms_are_refused_and_counted for TC-records-the-listed-leak-forms-are-refused: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-listed-leak-forms-are-refused.md)
- [Ran cargo test --locked --test leaks -- --exact the_user_name_comes_from_the_account for TC-records-the-user-name-comes-from-the-account: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-the-user-name-comes-from-the-account.md)
- [Ran cargo test --locked --test doctor -- --exact a_conflict_marker_is_named_by_its_seven_characters for TC-records-a-conflict-marker-is-named-by-its-seven-characters: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-conflict-marker-is-named-by-its-seven-characters.md)
- [Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-session-name-only-when-clean.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean.md)
- [Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean.md)

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
