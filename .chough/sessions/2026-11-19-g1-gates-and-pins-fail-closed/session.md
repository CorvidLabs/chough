---
kind: session
id: 2026-11-19-g1-gates-and-pins-fail-closed
aliases: [g1-gates-and-pins-fail-closed]
objective: "Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372)"
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-g1-gates-and-pins-fail-closed
pr: 413
criteria: [DRIVE-3, DRIVE-24, DRIVE-25]
requirements: [REQ-cli-007, REQ-cli-037, REQ-cli-057, REQ-cli-059, REQ-history-009]
cases: []
design: []
files: []
observations:
  - 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the
  - 2026-11-19-tc-cli-ship-checks-the-whole-history
  - 2026-11-19-tc-cli-ship-acts-on-the-pinned-session
  - 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another
  - 2026-11-19-tc-cli-session-shelve-resolves-its-key
  - 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow
  - 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument
  - 2026-11-19-tc-history-inbox-list-lists-the-pinned-session
  - 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-afb826
  - 2026-11-19-tc-cli-ship-checks-the-whole-history-effff9
  - 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-73b6e9
  - 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-99c554
  - 2026-11-19-tc-cli-session-shelve-resolves-its-key-c3e9e9
  - 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-165174
  - 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-4f1612
  - 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-841e5f
  - 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-f72a5c
  - 2026-11-19-tc-cli-ship-checks-the-whole-history-ab5769
  - 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-59e778
  - 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-eb55fa
  - 2026-11-19-tc-cli-session-shelve-resolves-its-key-1e1e98
  - 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-7964f9
  - 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-0f6567
  - 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-716a68
  - 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-c8196b
  - 2026-11-19-tc-cli-ship-checks-the-whole-history-66e503
  - 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-27266b
  - 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-55c8df
  - 2026-11-19-tc-cli-session-shelve-resolves-its-key-dd0760
  - 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-d2664d
  - 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-5ad259
  - 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-218f2e
  - 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-899356
  - 2026-11-19-tc-cli-ship-checks-the-whole-history-ca48d3
  - 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-e3c78a
  - 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-ecb2af
  - 2026-11-19-tc-cli-session-shelve-resolves-its-key-f29a9f
  - 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-a25bd4
  - 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-f9f0f8
  - 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-45cb98
  - 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-12330e
  - 2026-11-19-tc-cli-ship-checks-the-whole-history-a7a49d
  - 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-a1988a
  - 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-61a58e
  - 2026-11-19-tc-cli-session-shelve-resolves-its-key-505a93
  - 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-6c8f1e
  - 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-3a0423
  - 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-43445b
  - 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-cbf8df
  - 2026-11-19-tc-cli-ship-checks-the-whole-history-fab6e3
  - 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-5f558b
  - 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-c4bc56
  - 2026-11-19-tc-cli-session-shelve-resolves-its-key-76b5f2
  - 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-0c0883
  - 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-450552
  - 2026-11-19-tc-cli-ship-keeps-the-session-it-started-from
  - 2026-11-19-tc-cli-ship-refusals-do-not-echo-a-credential-id
  - 2026-11-19-tc-cli-ship-names-an-unreadable-commit
  - 2026-11-19-tc-cli-ship-with-a-base-commit-that-is-not-an-object-id
  - 2026-11-19-tc-cli-the-refetch-command-names-the-real-remote
  - 2026-11-19-tc-cli-several-untracked-remotes-are-named
  - 2026-11-19-tc-cli-an-unreadable-change-folder-tree-refuses
  - 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-a1215c
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
title: "G1: gates and session pins fail closed"
summary:
  - "A hand-edited no_native_change can no longer allow the ungated move; chough change ship checks its whole history from the base, refusing with a working way out when Git cannot read it and going on where a squash merge removed the base; its gate close and move into done act on the pinned session; inbox list honors the pin; session use and new say when a pin still names another session and shelve resolves aliases; chough change refuses a leaking argument before SpecSync runs (#401, #400, #369, #393, #372, part of #367)."
  - "Verified at 41d486eb: all eleven verify steps and fifteen cases on macOS; the tester's run on 22 real repositories and its regression set (191, 4 fixed, none newly failing); corvid-agent's review and recheck."
---
# Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372)

## Goal

Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-g1-gates-and-pins-fail-closed": "Approve, both as recommended / Approve / Yes, covered" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53). Three answers in order: 'Approve, both as recommended' (Claude Code chat answer, 2026-11-19 about 05:21Z, on G1 at 6a7c7d56 with #372 say-so and #400 whole-range); 'Approve' (about 06:12Z, on declaring history, dropping src/session_md.rs, adding the history spec files and moving the inbox criterion); 'Yes, covered' (2026-11-19 morning MST, on the two case files in place of .chough/cases/ and the re-answered constraints). Scope as checked by fable-lead at e219ab98); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-g1-gates-and-pins-fail-closed as 0xLeif
- 0xLeif decided "Leif's word on the #400 sentence and the G1 send-back": "yes to both" (Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 14:47Z on 2026-11-19 (morning MST): yes to the replacement #400 sentence and criterion in REQ-cli-059, and yes to t4b running G1's one send-back on his word, relayed via t4c); recorded by fable-t4b; a relayed claim, not authenticated
- 0xLeif decided "Scope approval of fix-g1-gates-and-pins-fail-closed with the narrowed #400 sentence (native approval)": "yes to both" (Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 14:47Z on 2026-11-19: yes to the replacement #400 sentence and criterion in REQ-cli-059 as sent (g1-400-wording), and to t4b running this one send-back. The send-back cancelled his three earlier answers on G1, which stand for the rest of the delta: 'Approve, both as recommended' (about 05:21Z), 'Approve' (about 06:12Z), 'Yes, covered' (morning MST)); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-g1-gates-and-pins-fail-closed as 0xLeif
- 0xLeif decided "Implementation review of fix-g1-gates-and-pins-fail-closed": "Yes, record and ship" (Leif's answer in this Claude Code session (hc t4b), 2026-11-19, to recording his implementation review of #413 at 41d486eb and shipping it. Under standing OK 6(b) (his 'Go' to the six rules, relayed by hc t4c): tester PASS at 41d486eb, corvid-agent approved 41d486eb, deltas unchanged since 3438bbca, checked by fable-lead); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change review fix-g1-gates-and-pins-fail-closed as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the.md)
- [Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-checks-the-whole-history.md)
- [Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-acts-on-the-pinned-session.md)
- [Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another.md)
- [Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-shelve-resolves-its-key.md)
- [Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow.md)
- [Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument.md)
- [Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-inbox-list-lists-the-pinned-session.md)
- [Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-afb826.md)
- [Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-checks-the-whole-history-effff9.md)
- [Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-acts-on-the-pinned-session-73b6e9.md)
- [Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-99c554.md)
- [Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-shelve-resolves-its-key-c3e9e9.md)
- [Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-165174.md)
- [Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-4f1612.md)
- [Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-inbox-list-lists-the-pinned-session-841e5f.md)
- [Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-f72a5c.md)
- [Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-checks-the-whole-history-ab5769.md)
- [Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-acts-on-the-pinned-session-59e778.md)
- [Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-eb55fa.md)
- [Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-shelve-resolves-its-key-1e1e98.md)
- [Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-7964f9.md)
- [Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-0f6567.md)
- [Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-inbox-list-lists-the-pinned-session-716a68.md)
- [Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-c8196b.md)
- [Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-checks-the-whole-history-66e503.md)
- [Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-acts-on-the-pinned-session-27266b.md)
- [Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-55c8df.md)
- [Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-shelve-resolves-its-key-dd0760.md)
- [Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-d2664d.md)
- [Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-5ad259.md)
- [Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-inbox-list-lists-the-pinned-session-218f2e.md)
- [Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-899356.md)
- [Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-checks-the-whole-history-ca48d3.md)
- [Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-acts-on-the-pinned-session-e3c78a.md)
- [Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-ecb2af.md)
- [Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-shelve-resolves-its-key-f29a9f.md)
- [Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-a25bd4.md)
- [Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-f9f0f8.md)
- [Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-inbox-list-lists-the-pinned-session-45cb98.md)
- [Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-12330e.md)
- [Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-checks-the-whole-history-a7a49d.md)
- [Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-acts-on-the-pinned-session-a1988a.md)
- [Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-61a58e.md)
- [Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-shelve-resolves-its-key-505a93.md)
- [Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-6c8f1e.md)
- [Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-3a0423.md)
- [Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-inbox-list-lists-the-pinned-session-43445b.md)
- [Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-cbf8df.md)
- [Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-checks-the-whole-history-fab6e3.md)
- [Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-acts-on-the-pinned-session-5f558b.md)
- [Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-c4bc56.md)
- [Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-session-shelve-resolves-its-key-76b5f2.md)
- [Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-0c0883.md)
- [Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-450552.md)
- [Ran cargo test --locked --test change -- --exact ship_keeps_the_session_it_started_from_when_the_pointer_moves for TC-cli-ship-keeps-the-session-it-started-from: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-keeps-the-session-it-started-from.md)
- [Ran cargo test --locked --test change -- --exact ship_refusals_do_not_echo_a_credential_id for TC-cli-ship-refusals-do-not-echo-a-credential-id: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-refusals-do-not-echo-a-credential-id.md)
- [Ran cargo test --locked --test change -- --exact ship_with_an_unreadable_commit_names_fetch_refetch for TC-cli-ship-names-an-unreadable-commit: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-names-an-unreadable-commit.md)
- [Ran cargo test --locked --test change -- --exact ship_with_a_base_commit_that_is_not_an_object_id_goes_on for TC-cli-ship-with-a-base-commit-that-is-not-an-object-id: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-ship-with-a-base-commit-that-is-not-an-object-id.md)
- [Ran cargo test --locked --test change -- --exact the_refetch_command_names_the_real_remote_and_works for TC-cli-the-refetch-command-names-the-real-remote: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-the-refetch-command-names-the-real-remote.md)
- [Ran cargo test --locked --test change -- --exact several_untracked_remotes_are_named_to_choose_from for TC-cli-several-untracked-remotes-are-named: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-several-untracked-remotes-are-named.md)
- [Ran cargo test --locked --test change -- --exact an_unreadable_change_folder_tree_is_never_read_as_absent for TC-cli-an-unreadable-change-folder-tree-refuses: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-an-unreadable-change-folder-tree-refuses.md)
- [Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-inbox-list-lists-the-pinned-session-a1215c.md)

## Open threads

## Timeline

- 2026-11-19T05:13:40Z · start · intent → intent · Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372) ^t-d0790c
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T05:13:50Z · advance · intent → design ·  ^t-b558e1
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T05:13:50Z · skip · design → plan · Bug fixes to validation, pins and refusals; nothing visual or interactive changes ^t-30ad58
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T05:21:53Z · gate-opened · plan → plan · Scope approval of fix-g1-gates-and-pins-fail-closed ^t-feef99
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 6a7c7d569106d0d6f3045470f58790650de1cbfc
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T14:05:52Z · direct-native-write · plan → plan · fix-g1-gates-and-pins-fail-closed's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-66fdb4
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: e219ab986110afa03610666eabf7cbaed212e503
  - direct_write:
    - change: fix-g1-gates-and-pins-fail-closed
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:05:52Z · gate-closed · plan → plan · Scope approval of fix-g1-gates-and-pins-fail-closed ^t-a68821
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: e219ab986110afa03610666eabf7cbaed212e503
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve, both as recommended / Approve / Yes, covered
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53). Three answers in order: 'Approve, both as recommended' (Claude Code chat answer, 2026-11-19 about 05:21Z, on G1 at 6a7c7d56 with #372 say-so and #400 whole-range); 'Approve' (about 06:12Z, on declaring history, dropping src/session_md.rs, adding the history spec files and moving the inbox criterion); 'Yes, covered' (2026-11-19 morning MST, on the two case files in place of .chough/cases/ and the re-answered constraints). Scope as checked by fable-lead at e219ab98
    - recorded_by: fable-t4b
    - native: specsync change approve fix-g1-gates-and-pins-fail-closed
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:18:34Z · advance · plan → build · native:approved ^t-883f9b
  - native_change: fix-g1-gates-and-pins-fail-closed
  - gate: native:approved
  - revision: 966758794f24e6ef21befbb20bfe209b0ea668de
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:47:47Z · gate-opened · build → build · Leif's word on the #400 sentence and the G1 send-back ^t-ca7a35
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - holds: build
  - drives: none
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:47:47Z · gate-closed · build → build · Leif's word on the #400 sentence and the G1 send-back ^t-3d9620
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes to both
    - where: Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 14:47Z on 2026-11-19 (morning MST): yes to the replacement #400 sentence and criterion in REQ-cli-059, and yes to t4b running G1's one send-back on his word, relayed via t4c
    - recorded_by: fable-t4b
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:48:14Z · gate-opened · build → build · Scope approval of fix-g1-gates-and-pins-fail-closed with the narrowed #400 sentence (native approval) ^t-200a2b
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - holds: build
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:48:14Z · direct-native-write · build → build · fix-g1-gates-and-pins-fail-closed's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-dfe690
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - direct_write:
    - change: fix-g1-gates-and-pins-fail-closed
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:48:14Z · gate-closed · build → build · Scope approval of fix-g1-gates-and-pins-fail-closed with the narrowed #400 sentence (native approval) ^t-b6a3e1
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes to both
    - where: Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 14:47Z on 2026-11-19: yes to the replacement #400 sentence and criterion in REQ-cli-059 as sent (g1-400-wording), and to t4b running this one send-back. The send-back cancelled his three earlier answers on G1, which stand for the rest of the delta: 'Approve, both as recommended' (about 05:21Z), 'Approve' (about 06:12Z), 'Yes, covered' (morning MST)
    - recorded_by: fable-t4b
    - native: specsync change approve fix-g1-gates-and-pins-fail-closed
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T16:47:29Z · advance · build → test · native:verifying ^t-c2d9af
  - native_change: fix-g1-gates-and-pins-fail-closed
  - gate: native:verifying
  - revision: d542aa58be33b5402a83330af954978b33bc390a
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T16:47:30Z · advance · test → accept · native:verifying ^t-fa76f0
  - native_change: fix-g1-gates-and-pins-fail-closed
  - gate: native:verifying
  - revision: d542aa58be33b5402a83330af954978b33bc390a
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T16:47:30Z · gate-opened · accept → accept · Implementation review of fix-g1-gates-and-pins-fail-closed ^t-84f19c
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: d542aa58be33b5402a83330af954978b33bc390a
  - holds: accept
  - drives: review
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T19:44:10Z · gate-closed · accept → accept · Implementation review of fix-g1-gates-and-pins-fail-closed ^t-b763eb
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 57d6e38feadc057de23a6037cabac66ff909229f
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, record and ship
    - where: Leif's answer in this Claude Code session (hc t4b), 2026-11-19, to recording his implementation review of #413 at 41d486eb and shipping it. Under standing OK 6(b) (his 'Go' to the six rules, relayed by hc t4c): tester PASS at 41d486eb, corvid-agent approved 41d486eb, deltas unchanged since 3438bbca, checked by fable-lead
    - recorded_by: fable-t4b
    - native: specsync change review fix-g1-gates-and-pins-fail-closed
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T19:44:12Z · advance · accept → done · native:archived ^t-e79cb1
  - native_change: fix-g1-gates-and-pins-fail-closed
  - gate: native:archived
  - revision: 57d6e38feadc057de23a6037cabac66ff909229f
  - by: fable (claude-opus-5-5, hc t4b)
