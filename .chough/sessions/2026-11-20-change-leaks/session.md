---
kind: session
id: 2026-11-20-change-leaks
aliases: [change-leaks]
objective: "A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367)"
intent_scope: ""
modules: [cli, records]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: change-leaks
pr: "430"
criteria: [LIFE-26]
requirements: [REQ-cli-057, REQ-records-006]
cases: []
design: []
files: []
observations:
  - 2026-11-20-tc-cli-a-leaking-description-rationale-or-answer-is-refused
  - 2026-11-20-tc-records-doctor-scans-the-open-changes
  - 2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds
  - 2026-11-20-tc-records-no-command-echoes-a-given-value
  - 2026-11-20-tc-cli-a-leaking-description-rationale-or-answer-is-refused-52fc05
  - 2026-11-20-tc-records-doctor-scans-the-open-changes-64304c
  - 2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds-3d2da5
  - 2026-11-20-tc-records-a-close-gate-value-is-named-by-what-it-holds
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-f0d727
  - 2026-11-20-tc-cli-a-leaking-description-rationale-or-answer-is-refused-425c1f
  - 2026-11-20-tc-records-doctor-scans-the-open-changes-f7ed25
  - 2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds-a244fe
  - 2026-11-20-tc-records-a-close-gate-value-is-named-by-what-it-holds-1a8c61
  - 2026-11-20-tc-records-no-command-echoes-a-given-value-576667
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367)

## Goal

A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367)

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of change-leaks": "A: refuse, --redact (Recommended); 1: open changes (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 15:08Z, relayed by hc t4c; the questions as shown: 'Chough #412: a change's description, rationale and answers aren't leak-checked today ... When one holds a leak, what should happen?' and 'Chough #412: what should doctor --leaks also scan?' (.specsync/changes/ but not each change's deltas/, and --fix --redact skips those findings); on change-leaks at 01b7952d with choices (A) and (1)); recorded by chough-fixer-t4n; a relayed claim, not authenticated; ran specsync change approve change-leaks as 0xLeif
- 0xLeif decided "Implementation review of change-leaks": "Pass (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 22:43Z, relayed by hc t4c: his implementation review of Chough #412 (PR #430) at 7ab2ca20 (the leak check on description, --rationale and answers with --redact as the way out; doctor --leaks scanning open changes; the tester's PASS: 178/178, 2 new real-repo findings, --redact with SpecSync 6.3.0 and 6.4.0)); recorded by chough-fixer-t4n; a relayed claim, not authenticated; ran specsync change review change-leaks as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test change -- --exact a_leaking_description_rationale_or_answer_is_refused_before_specsync_runs for TC-cli-a-leaking-description-rationale-or-answer-is-refused: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-leaking-description-rationale-or-answer-is-refused.md)
- [Ran cargo test --locked --test leaks -- --exact doctor_scans_the_open_changes_but_not_their_deltas_or_the_archive for TC-records-doctor-scans-the-open-changes: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-doctor-scans-the-open-changes.md)
- [Ran cargo test --locked --test output_safety -- --exact an_explicit_evidence_id_is_named_by_what_it_holds for TC-records-an-explicit-evidence-id-is-named-by-what-it-holds: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-echoes-a-given-value.md)
- [Ran cargo test --locked --test change -- --exact a_leaking_description_rationale_or_answer_is_refused_before_specsync_runs for TC-cli-a-leaking-description-rationale-or-answer-is-refused: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-leaking-description-rationale-or-answer-is-refused-52fc05.md)
- [Ran cargo test --locked --test leaks -- --exact doctor_scans_the_open_changes_but_not_their_deltas_or_the_archive for TC-records-doctor-scans-the-open-changes: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-doctor-scans-the-open-changes-64304c.md)
- [Ran cargo test --locked --test output_safety -- --exact an_explicit_evidence_id_is_named_by_what_it_holds for TC-records-an-explicit-evidence-id-is-named-by-what-it-holds: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds-3d2da5.md)
- [Ran cargo test --locked --test change -- --exact change_ship_names_a_close_gate_value_by_what_it_holds for TC-records-a-close-gate-value-is-named-by-what-it-holds: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-close-gate-value-is-named-by-what-it-holds.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-echoes-a-given-value-f0d727.md)
- [Ran cargo test --locked --test change -- --exact a_leaking_description_rationale_or_answer_is_refused_before_specsync_runs for TC-cli-a-leaking-description-rationale-or-answer-is-refused: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-leaking-description-rationale-or-answer-is-refused-425c1f.md)
- [Ran cargo test --locked --test leaks -- --exact doctor_scans_the_open_changes_but_not_their_deltas_or_the_archive for TC-records-doctor-scans-the-open-changes: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-doctor-scans-the-open-changes-f7ed25.md)
- [Ran cargo test --locked --test output_safety -- --exact an_explicit_evidence_id_is_named_by_what_it_holds for TC-records-an-explicit-evidence-id-is-named-by-what-it-holds: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds-a244fe.md)
- [Ran cargo test --locked --test change -- --exact change_ship_names_a_close_gate_value_by_what_it_holds for TC-records-a-close-gate-value-is-named-by-what-it-holds: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-a-close-gate-value-is-named-by-what-it-holds-1a8c61.md)
- [Ran cargo test --locked --test output_safety -- --exact no_command_echoes_a_value_given_on_the_command_line for TC-records-no-command-echoes-a-given-value: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-records-no-command-echoes-a-given-value-576667.md)

## Open threads

## Timeline

- 2026-11-20T14:54:25Z · start · intent → intent · A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367) ^t-295bc8
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:54:26Z · advance · intent → design ·  ^t-72e0e1
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:54:26Z · skip · design → plan · Nothing visual changes: refusals before SpecSync runs, a wider doctor scan and plain error wording. ^t-072fbe
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:09:16Z · gate-opened · plan → plan · Approve the scope of change-leaks ^t-1d3ce9
  - native_change: change-leaks
  - revision: 01b7952d1fe5f422a610eb9a957aa62a686fd4ae
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:09:16Z · gate-closed · plan → plan · Approve the scope of change-leaks ^t-38d949
  - native_change: change-leaks
  - revision: 01b7952d1fe5f422a610eb9a957aa62a686fd4ae
  - relayed_by:
    - decided_by: 0xLeif
    - words: A: refuse, --redact (Recommended); 1: open changes (Recommended)
    - where: Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 15:08Z, relayed by hc t4c; the questions as shown: 'Chough #412: a change's description, rationale and answers aren't leak-checked today ... When one holds a leak, what should happen?' and 'Chough #412: what should doctor --leaks also scan?' (.specsync/changes/ but not each change's deltas/, and --fix --redact skips those findings); on change-leaks at 01b7952d with choices (A) and (1)
    - recorded_by: chough-fixer-t4n
    - native: specsync change approve change-leaks
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T16:50:32Z · advance · plan → build · native:approved ^t-1ea9cc
  - native_change: change-leaks
  - gate: native:approved
  - revision: f6536ca4bd92c7fd83642f0028b4f954a641254c
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T22:43:47Z · advance · build → test · native:verifying ^t-f74f97
  - native_change: change-leaks
  - gate: native:verifying
  - revision: 7ab2ca20f460d5c9187e3dd8ffab3d64e0809901
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T22:43:49Z · advance · test → accept · native:verifying ^t-d4400b
  - native_change: change-leaks
  - gate: native:verifying
  - revision: 7ab2ca20f460d5c9187e3dd8ffab3d64e0809901
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T22:44:04Z · gate-opened · accept → accept · Implementation review of change-leaks ^t-dfbd03
  - native_change: change-leaks
  - revision: 7ab2ca20f460d5c9187e3dd8ffab3d64e0809901
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T22:44:53Z · gate-closed · accept → accept · Implementation review of change-leaks ^t-91620e
  - native_change: change-leaks
  - revision: aef38332e3abfe8dfca6ce35837f16fccf0ff619
  - relayed_by:
    - decided_by: 0xLeif
    - words: Pass (Recommended)
    - where: Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 22:43Z, relayed by hc t4c: his implementation review of Chough #412 (PR #430) at 7ab2ca20 (the leak check on description, --rationale and answers with --redact as the way out; doctor --leaks scanning open changes; the tester's PASS: 178/178, 2 new real-repo findings, --redact with SpecSync 6.3.0 and 6.4.0)
    - recorded_by: chough-fixer-t4n
    - native: specsync change review change-leaks
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T22:44:54Z · advance · accept → done · native:archived ^t-9a5701
  - native_change: change-leaks
  - gate: native:archived
  - revision: aef38332e3abfe8dfca6ce35837f16fccf0ff619
  - by: chough-fixer (claude-opus-5-5, hc t4n)
