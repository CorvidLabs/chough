---
kind: session
id: 2026-11-19-g1-gates-and-pins-fail-closed
aliases: [g1-gates-and-pins-fail-closed]
objective: "Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372)"
stage: done
stage_status: archived
iteration: 1
native_change: fix-g1-gates-and-pins-fail-closed
pr: 413
criteria: [DRIVE-3, DRIVE-24, DRIVE-25]
requirements: [REQ-cli-007, REQ-cli-037, REQ-cli-057, REQ-cli-059, REQ-history-009]
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
title: "G1: gates and session pins fail closed"
summary:
  - "A hand-edited no_native_change can no longer allow the ungated move; chough change ship checks its whole history from the base, refusing with a working way out when Git cannot read it and going on where a squash merge removed the base; its gate close and move into done act on the pinned session; inbox list honors the pin; session use and new say when a pin still names another session and shelve resolves aliases; chough change refuses a leaking argument before SpecSync runs (#401, #400, #369, #393, #372, part of #367)."
  - "Verified at 41d486eb: all eleven verify steps and fifteen cases on macOS; the tester's run on 22 real repositories and its regression set (191, 4 fixed, none newly failing); corvid-agent's review and recheck."
closed: 2026-11-19
outcome: done
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

- 48 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1). - 8219dde5fb6b; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-cbf8df
- Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1). - 2b27c07c98bb; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-ship-checks-the-whole-history-fab6e3
- Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1). - c28fd66f1b02; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-ship-acts-on-the-pinned-session-5f558b
- Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1). - d8de037b1ea0; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-c4bc56
- Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1). - 58d25598a8bf; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-session-shelve-resolves-its-key-76b5f2
- Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1). - 363bad3c0a64; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-0c0883
- Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1). - cd1230d716e3; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-450552
- Ran cargo test --locked --test change -- --exact ship_keeps_the_session_it_started_from_when_the_pointer_moves for TC-cli-ship-keeps-the-session-it-started-from: passed (exit 0, tests run 1). - c9412dda04f3; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-ship-keeps-the-session-it-started-from
- Ran cargo test --locked --test change -- --exact ship_refusals_do_not_echo_a_credential_id for TC-cli-ship-refusals-do-not-echo-a-credential-id: passed (exit 0, tests run 1). - a11e7bf89590; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-ship-refusals-do-not-echo-a-credential-id
- Ran cargo test --locked --test change -- --exact ship_with_an_unreadable_commit_names_fetch_refetch for TC-cli-ship-names-an-unreadable-commit: passed (exit 0, tests run 1). - 498453e1659a; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-ship-names-an-unreadable-commit
- Ran cargo test --locked --test change -- --exact ship_with_a_base_commit_that_is_not_an_object_id_goes_on for TC-cli-ship-with-a-base-commit-that-is-not-an-object-id: passed (exit 0, tests run 1). - 306bc6d20a7a; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-ship-with-a-base-commit-that-is-not-an-object-id
- Ran cargo test --locked --test change -- --exact the_refetch_command_names_the_real_remote_and_works for TC-cli-the-refetch-command-names-the-real-remote: passed (exit 0, tests run 1). - 81a32ec5779f; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-the-refetch-command-names-the-real-remote
- Ran cargo test --locked --test change -- --exact several_untracked_remotes_are_named_to_choose_from for TC-cli-several-untracked-remotes-are-named: passed (exit 0, tests run 1). - 1538adf48fca; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-several-untracked-remotes-are-named
- Ran cargo test --locked --test change -- --exact an_unreadable_change_folder_tree_is_never_read_as_absent for TC-cli-an-unreadable-change-folder-tree-refuses: passed (exit 0, tests run 1). - 1692fbe541ea; limits: inherited environment variables were not recorded ^2026-11-19-tc-cli-an-unreadable-change-folder-tree-refuses
- Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1). - 9f4150369494; limits: inherited environment variables were not recorded ^2026-11-19-tc-history-inbox-list-lists-the-pinned-session-a1215c

## Bugs

None.

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

Closed 2026-11-19: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the
    claim: "Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move
    revision: 8482f2025741f35d4b56afae8df77248c26f55f3
    environment: local-macos-arm64
    content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
    fingerprint: sha256:acd96e85ce6664e30b12f002775b622e906b57f947be6de7189f49c2c521c07b
    case: TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move
    case_digest: 5a71b02828eacc6a111ef179e060d318d7c9d6d728f159c720082d78bacc2243
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:12:11Z
    duration_seconds: 3
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-checks-the-whole-history
    claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base
    revision: 8482f2025741f35d4b56afae8df77248c26f55f3
    environment: local-macos-arm64
    content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
    fingerprint: sha256:9cccbfca744e7b5446d0f39d5666114d11c3060275c59ce331ff1202cc8bfd4d
    case: TC-cli-ship-checks-the-whole-history
    case_digest: bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:12:15Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session
    claim: "Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session
    revision: 8482f2025741f35d4b56afae8df77248c26f55f3
    environment: local-macos-arm64
    content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
    fingerprint: sha256:ae2726fc58ee6ad7ba6778818a2621de55969d0a54582c51453f2d0e0ed3fc98
    case: TC-cli-ship-acts-on-the-pinned-session
    case_digest: 416c0ce58c7d0afd7055a2c22b7cab4af4fdc66d1201a4a655ff53b53448b1ef
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:12:19Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another
    claim: "Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session
    revision: 8482f2025741f35d4b56afae8df77248c26f55f3
    environment: local-macos-arm64
    content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
    fingerprint: sha256:f2bbc2358d390a362c5d6ff80bfe370e45dd431fb597b0a944fa1981fec87d2f
    case: TC-cli-session-use-and-new-say-when-a-pin-names-another-session
    case_digest: d9c6fb3e3e8febc59bbef3b3e30266bcc5f4fb4c09b851462740d2c4fe8ba75c
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:12:22Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-shelve-resolves-its-key
    claim: "Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does
    revision: 8482f2025741f35d4b56afae8df77248c26f55f3
    environment: local-macos-arm64
    content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
    fingerprint: sha256:79e8059678043e25d47e49e1558d8e231ee538aba1e45fbb39b7d33da7837755
    case: TC-cli-session-shelve-resolves-its-key
    case_digest: 0f10f9ab4c89450e62496a7acfa8cf76b6891d813a7f900e5425263a75f3dc4d
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:12:25Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow
    claim: "Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow
    revision: 8482f2025741f35d4b56afae8df77248c26f55f3
    environment: local-macos-arm64
    content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
    fingerprint: sha256:63e5120ba99191957d4a40baf46c04620b5eb1f33971e963d86aa416d10d221e
    case: TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow
    case_digest: 4772c1d8f53608ca711c8fa72e8e9f508e532e1cd100aadccb94b588c43850ee
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:12:27Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument
    claim: "Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs
    revision: 8482f2025741f35d4b56afae8df77248c26f55f3
    environment: local-macos-arm64
    content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
    fingerprint: sha256:cf33b6772c173cbf5452a1f734ff96ae966ae87fae194485bd31f2b3ce806727
    case: TC-cli-change-commands-refuse-a-leaking-argument
    case_digest: 5ef68d263d84f4d077ad328bbc007bf2239b4589c68f5d6ee91350d0b1fcbe2b
    criteria: DRIVE-3
    requirements: REQ-cli-057
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:12:30Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session
    claim: "Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session
    revision: 8482f2025741f35d4b56afae8df77248c26f55f3
    environment: local-macos-arm64
    content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
    fingerprint: sha256:f8ee68803b7a13e326bc1a21f256a1826c0d0ae97204031ec518ae772a538e0e
    case: TC-history-inbox-list-lists-the-pinned-session
    case_digest: b29b0dd986237b002d1a95dee4c6fb8871381d4383e9eaf94c1f8636674bc357
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-history-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:12:33Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-afb826
    claim: "Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move
    revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
    environment: local-macos-arm64
    content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
    fingerprint: sha256:29e3ace83c6c7b154ad01838eed2b2ab304ea3c363875d1cb35294cad45adf32
    case: TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move
    case_digest: 5a71b02828eacc6a111ef179e060d318d7c9d6d728f159c720082d78bacc2243
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:25:13Z
    duration_seconds: 8
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-checks-the-whole-history-effff9
    claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base
    revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
    environment: local-macos-arm64
    content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
    fingerprint: sha256:370007a3cc5b30fe505f4140459325ad3a7f171739f7a7bb514600cfdc3c1dde
    case: TC-cli-ship-checks-the-whole-history
    case_digest: bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:25:19Z
    duration_seconds: 4
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-73b6e9
    claim: "Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session
    revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
    environment: local-macos-arm64
    content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
    fingerprint: sha256:178d3c7f2bb8c0537084960a4f1decc2089345cb66bac1fd8d51ba89c547e47b
    case: TC-cli-ship-acts-on-the-pinned-session
    case_digest: 416c0ce58c7d0afd7055a2c22b7cab4af4fdc66d1201a4a655ff53b53448b1ef
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:25:23Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-99c554
    claim: "Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session
    revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
    environment: local-macos-arm64
    content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
    fingerprint: sha256:b9eb6860dcfbf4016eb2efd88d68cdac190ffbee99bb0130ad93361f3b320827
    case: TC-cli-session-use-and-new-say-when-a-pin-names-another-session
    case_digest: d9c6fb3e3e8febc59bbef3b3e30266bcc5f4fb4c09b851462740d2c4fe8ba75c
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:25:27Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-c3e9e9
    claim: "Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does
    revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
    environment: local-macos-arm64
    content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
    fingerprint: sha256:3302b0e1486ea87945659fffd01cc0672fea05a8a485fcb28a446dcb35ec8508
    case: TC-cli-session-shelve-resolves-its-key
    case_digest: 0f10f9ab4c89450e62496a7acfa8cf76b6891d813a7f900e5425263a75f3dc4d
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:25:30Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-165174
    claim: "Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow
    revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
    environment: local-macos-arm64
    content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
    fingerprint: sha256:ad4e51f22d5796936aa49c0c007540405ddcfc42cd23e5b319865bb66404646a
    case: TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow
    case_digest: 4772c1d8f53608ca711c8fa72e8e9f508e532e1cd100aadccb94b588c43850ee
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:25:33Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-4f1612
    claim: "Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs
    revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
    environment: local-macos-arm64
    content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
    fingerprint: sha256:2c938ec607bfe81dcc0ade904319862e17cbb7528814e3e5ecbb50ccfb17912b
    case: TC-cli-change-commands-refuse-a-leaking-argument
    case_digest: 5ef68d263d84f4d077ad328bbc007bf2239b4589c68f5d6ee91350d0b1fcbe2b
    criteria: DRIVE-3
    requirements: REQ-cli-057
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:25:36Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-841e5f
    claim: "Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session
    revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
    environment: local-macos-arm64
    content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
    fingerprint: sha256:fda4fed78f0a77b633c0ff7f7dde2b9f4d3f2cd3ec2afe2e3e8b1ed82a72f77e
    case: TC-history-inbox-list-lists-the-pinned-session
    case_digest: b29b0dd986237b002d1a95dee4c6fb8871381d4383e9eaf94c1f8636674bc357
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-history-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:25:38Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-f72a5c
    claim: "Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move
    revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
    environment: local-macos-arm64
    content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
    fingerprint: sha256:24ac40ba3e7fbfb4e821ec9d834a41cc05cd1c9fb4c0d41fdc321a0f6dc07e43
    case: TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move
    case_digest: 5a71b02828eacc6a111ef179e060d318d7c9d6d728f159c720082d78bacc2243
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:43:48Z
    duration_seconds: 12
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-checks-the-whole-history-ab5769
    claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base
    revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
    environment: local-macos-arm64
    content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
    fingerprint: sha256:e44c11548fade8eca4601403103451fc630e83345bb93e9b68e324620807b36f
    case: TC-cli-ship-checks-the-whole-history
    case_digest: bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:43:56Z
    duration_seconds: 6
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-59e778
    claim: "Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session
    revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
    environment: local-macos-arm64
    content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
    fingerprint: sha256:d2f168d716465d10baf1f4861aaeab51f01633decdd9d8356a3f597e6e9407dc
    case: TC-cli-ship-acts-on-the-pinned-session
    case_digest: 416c0ce58c7d0afd7055a2c22b7cab4af4fdc66d1201a4a655ff53b53448b1ef
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:44:02Z
    duration_seconds: 3
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-eb55fa
    claim: "Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session
    revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
    environment: local-macos-arm64
    content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
    fingerprint: sha256:fd16353c69bfbcd439b039d70bdb7ea8f8489616548a97f80730319d13f73962
    case: TC-cli-session-use-and-new-say-when-a-pin-names-another-session
    case_digest: d9c6fb3e3e8febc59bbef3b3e30266bcc5f4fb4c09b851462740d2c4fe8ba75c
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:44:07Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-1e1e98
    claim: "Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does
    revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
    environment: local-macos-arm64
    content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
    fingerprint: sha256:ef1ef2585f6268ea3bc0454c731dd2221487393523cf5e4820eab87c6c783ee8
    case: TC-cli-session-shelve-resolves-its-key
    case_digest: 0f10f9ab4c89450e62496a7acfa8cf76b6891d813a7f900e5425263a75f3dc4d
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:44:12Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-7964f9
    claim: "Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow
    revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
    environment: local-macos-arm64
    content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
    fingerprint: sha256:83542ed8f43704ab86d08d68f3866368a1b8abe440d3a9080d79662431988df2
    case: TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow
    case_digest: 4772c1d8f53608ca711c8fa72e8e9f508e532e1cd100aadccb94b588c43850ee
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:44:17Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-0f6567
    claim: "Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs
    revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
    environment: local-macos-arm64
    content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
    fingerprint: sha256:f1b4e74c69aed202529f63f9b6cae5453ffafc1485d395effb56a3b90cf16e02
    case: TC-cli-change-commands-refuse-a-leaking-argument
    case_digest: 5ef68d263d84f4d077ad328bbc007bf2239b4589c68f5d6ee91350d0b1fcbe2b
    criteria: DRIVE-3
    requirements: REQ-cli-057
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:44:25Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-716a68
    claim: "Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session
    revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
    environment: local-macos-arm64
    content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
    fingerprint: sha256:e204582c3e3ea28e34e3a208af1758e6378dcf995ee71ae2483ada67160fd727
    case: TC-history-inbox-list-lists-the-pinned-session
    case_digest: b29b0dd986237b002d1a95dee4c6fb8871381d4383e9eaf94c1f8636674bc357
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-history-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:44:30Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-c8196b
    claim: "Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move
    revision: 08399615be5c89eeecf91690cad2bfa741c2685c
    environment: local-macos-arm64
    content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
    fingerprint: sha256:62bb32b6cb589d00754a2e734a295e7f9bcf0cc78168d8b17e29750ae39db584
    case: TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move
    case_digest: 5a71b02828eacc6a111ef179e060d318d7c9d6d728f159c720082d78bacc2243
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:04:09Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-checks-the-whole-history-66e503
    claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base
    revision: 08399615be5c89eeecf91690cad2bfa741c2685c
    environment: local-macos-arm64
    content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
    fingerprint: sha256:4f2ef8c909d513dc508394025b55353d755bde0b4e771d253071eed761052384
    case: TC-cli-ship-checks-the-whole-history
    case_digest: bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:04:12Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-27266b
    claim: "Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session
    revision: 08399615be5c89eeecf91690cad2bfa741c2685c
    environment: local-macos-arm64
    content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
    fingerprint: sha256:c42d2b9052273bb3f1ae34a15e8777de68e68b119441d10de5758eda9ee84630
    case: TC-cli-ship-acts-on-the-pinned-session
    case_digest: 416c0ce58c7d0afd7055a2c22b7cab4af4fdc66d1201a4a655ff53b53448b1ef
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:04:15Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-55c8df
    claim: "Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session
    revision: 08399615be5c89eeecf91690cad2bfa741c2685c
    environment: local-macos-arm64
    content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
    fingerprint: sha256:48a0480025db721f4b953ed6e3ced6404851bd05bb42ee3f0d79d7ad43e006be
    case: TC-cli-session-use-and-new-say-when-a-pin-names-another-session
    case_digest: d9c6fb3e3e8febc59bbef3b3e30266bcc5f4fb4c09b851462740d2c4fe8ba75c
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:04:17Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-dd0760
    claim: "Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does
    revision: 08399615be5c89eeecf91690cad2bfa741c2685c
    environment: local-macos-arm64
    content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
    fingerprint: sha256:82e0fba5aeb59677c952f6e1273b87ee9bf1d63cb88841027fbd69881a4bb32f
    case: TC-cli-session-shelve-resolves-its-key
    case_digest: 0f10f9ab4c89450e62496a7acfa8cf76b6891d813a7f900e5425263a75f3dc4d
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:04:19Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-d2664d
    claim: "Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow
    revision: 08399615be5c89eeecf91690cad2bfa741c2685c
    environment: local-macos-arm64
    content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
    fingerprint: sha256:16ec371b31deb25fe39265fa333e4205d32540d9fea0723c48b8f59f5804e89e
    case: TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow
    case_digest: 4772c1d8f53608ca711c8fa72e8e9f508e532e1cd100aadccb94b588c43850ee
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:04:22Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-5ad259
    claim: "Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs
    revision: 08399615be5c89eeecf91690cad2bfa741c2685c
    environment: local-macos-arm64
    content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
    fingerprint: sha256:c40d7c853035c2d8a511be6097dbf5e1440e8db84f96f9026ffbbbff61aa8df2
    case: TC-cli-change-commands-refuse-a-leaking-argument
    case_digest: 5ef68d263d84f4d077ad328bbc007bf2239b4589c68f5d6ee91350d0b1fcbe2b
    criteria: DRIVE-3
    requirements: REQ-cli-057
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:04:24Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-218f2e
    claim: "Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session
    revision: 08399615be5c89eeecf91690cad2bfa741c2685c
    environment: local-macos-arm64
    content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
    fingerprint: sha256:9f41503694949d9536ab6dd099e6e94f5653fef128b894b97b62bd920b568f14
    case: TC-history-inbox-list-lists-the-pinned-session
    case_digest: b29b0dd986237b002d1a95dee4c6fb8871381d4383e9eaf94c1f8636674bc357
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-history-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:04:26Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-899356
    claim: "Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move
    revision: 753a022557083e0873d048ced9f5be2b40f686fd
    environment: local-macos-arm64
    content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
    fingerprint: sha256:0414ba00a8333672387081a92eaa307f2c6dfa0472fa6d1893c74e9266e0bab5
    case: TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move
    case_digest: 5a71b02828eacc6a111ef179e060d318d7c9d6d728f159c720082d78bacc2243
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:17:28Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-checks-the-whole-history-ca48d3
    claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base
    revision: 753a022557083e0873d048ced9f5be2b40f686fd
    environment: local-macos-arm64
    content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
    fingerprint: sha256:7ef0e1e84756ab7c02d21e4b2798261c20f305774ca42c814e7874eb17aded19
    case: TC-cli-ship-checks-the-whole-history
    case_digest: bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:17:31Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-e3c78a
    claim: "Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session
    revision: 753a022557083e0873d048ced9f5be2b40f686fd
    environment: local-macos-arm64
    content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
    fingerprint: sha256:d1ae0b648047fd83a88e60d0fe239abafbf49bf640dec54e9c1c38c1aca6e3d9
    case: TC-cli-ship-acts-on-the-pinned-session
    case_digest: 416c0ce58c7d0afd7055a2c22b7cab4af4fdc66d1201a4a655ff53b53448b1ef
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:17:33Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-ecb2af
    claim: "Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session
    revision: 753a022557083e0873d048ced9f5be2b40f686fd
    environment: local-macos-arm64
    content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
    fingerprint: sha256:9110b4288917171ac75af0fbd5094392779ea2b9d2349fc0ce8b483031363e92
    case: TC-cli-session-use-and-new-say-when-a-pin-names-another-session
    case_digest: d9c6fb3e3e8febc59bbef3b3e30266bcc5f4fb4c09b851462740d2c4fe8ba75c
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:17:35Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-f29a9f
    claim: "Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does
    revision: 753a022557083e0873d048ced9f5be2b40f686fd
    environment: local-macos-arm64
    content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
    fingerprint: sha256:58d25598a8bf97db4588bc03445cf7ce33ebeceb9ff8f80d679732564418e015
    case: TC-cli-session-shelve-resolves-its-key
    case_digest: 0f10f9ab4c89450e62496a7acfa8cf76b6891d813a7f900e5425263a75f3dc4d
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:17:37Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-a25bd4
    claim: "Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow
    revision: 753a022557083e0873d048ced9f5be2b40f686fd
    environment: local-macos-arm64
    content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
    fingerprint: sha256:3b9ff86354d16ccdf76ae27ddacf68c6117fd09c6d4cedaa4403fd7b02a3d745
    case: TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow
    case_digest: 4772c1d8f53608ca711c8fa72e8e9f508e532e1cd100aadccb94b588c43850ee
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:17:40Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-f9f0f8
    claim: "Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs
    revision: 753a022557083e0873d048ced9f5be2b40f686fd
    environment: local-macos-arm64
    content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
    fingerprint: sha256:672bf5bb6f1298ec78f0427625a7c2c33135882b34616fe46d9aeaf05aab13c3
    case: TC-cli-change-commands-refuse-a-leaking-argument
    case_digest: 5ef68d263d84f4d077ad328bbc007bf2239b4589c68f5d6ee91350d0b1fcbe2b
    criteria: DRIVE-3
    requirements: REQ-cli-057
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:17:42Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-45cb98
    claim: "Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session
    revision: 753a022557083e0873d048ced9f5be2b40f686fd
    environment: local-macos-arm64
    content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
    fingerprint: sha256:f9b9f852d3a29efad7394e0e0d979ed1d77a0a26e89b3160b00ea684e1d7a662
    case: TC-history-inbox-list-lists-the-pinned-session
    case_digest: b29b0dd986237b002d1a95dee4c6fb8871381d4383e9eaf94c1f8636674bc357
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-history-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:17:44Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-12330e
    claim: "Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move
    revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
    environment: local-macos-arm64
    content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
    fingerprint: sha256:fca2f56efe38456ca5e8a190d73d212b515103ee201dcd436f446bcf06b284f5
    case: TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move
    case_digest: 5a71b02828eacc6a111ef179e060d318d7c9d6d728f159c720082d78bacc2243
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:33:29Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-checks-the-whole-history-a7a49d
    claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base
    revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
    environment: local-macos-arm64
    content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
    fingerprint: sha256:7db894284a9f29d4b6865b7e3b463784c139e85d4576c31f56ab171df8d927a1
    case: TC-cli-ship-checks-the-whole-history
    case_digest: bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:33:32Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-a1988a
    claim: "Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session
    revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
    environment: local-macos-arm64
    content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
    fingerprint: sha256:caba923eaf8f748e9b9585ca4a46f6fde52937a1fccdd2455d852e2d5ac51841
    case: TC-cli-ship-acts-on-the-pinned-session
    case_digest: 416c0ce58c7d0afd7055a2c22b7cab4af4fdc66d1201a4a655ff53b53448b1ef
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:33:35Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-61a58e
    claim: "Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session
    revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
    environment: local-macos-arm64
    content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
    fingerprint: sha256:0b0695893a483e6043a890db5460ce52008bae9f4901d4d3b772a534a54e7428
    case: TC-cli-session-use-and-new-say-when-a-pin-names-another-session
    case_digest: d9c6fb3e3e8febc59bbef3b3e30266bcc5f4fb4c09b851462740d2c4fe8ba75c
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:33:37Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-505a93
    claim: "Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does
    revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
    environment: local-macos-arm64
    content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
    fingerprint: sha256:0fea8b0972aa564302a65ef05110de736037d3578f3bcd1f89141c003677a7cf
    case: TC-cli-session-shelve-resolves-its-key
    case_digest: 0f10f9ab4c89450e62496a7acfa8cf76b6891d813a7f900e5425263a75f3dc4d
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:33:40Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-6c8f1e
    claim: "Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow
    revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
    environment: local-macos-arm64
    content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
    fingerprint: sha256:a53f513ff4781b61b7ddb8857c128cab9250dac9f5cb22abd8130cecfc6638a1
    case: TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow
    case_digest: 4772c1d8f53608ca711c8fa72e8e9f508e532e1cd100aadccb94b588c43850ee
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:33:42Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-3a0423
    claim: "Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs
    revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
    environment: local-macos-arm64
    content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
    fingerprint: sha256:6781b1b8e120e677c2d84be4b56fe7c3dad846f763c40e6349232cfd3708266a
    case: TC-cli-change-commands-refuse-a-leaking-argument
    case_digest: 5ef68d263d84f4d077ad328bbc007bf2239b4589c68f5d6ee91350d0b1fcbe2b
    criteria: DRIVE-3
    requirements: REQ-cli-057
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:33:44Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-43445b
    claim: "Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session
    revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
    environment: local-macos-arm64
    content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
    fingerprint: sha256:2b8a585fc8405baafb23b6013704af58c2dc08011def2b0b712f39ffdedaa854
    case: TC-history-inbox-list-lists-the-pinned-session
    case_digest: b29b0dd986237b002d1a95dee4c6fb8871381d4383e9eaf94c1f8636674bc357
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-history-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T17:33:46Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-cbf8df
    claim: "Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:8219dde5fb6b610a6c134a107b129bf9baf85b4e696be5d9b66ef5ba08651f72
    case: TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move
    case_digest: 5a71b02828eacc6a111ef179e060d318d7c9d6d728f159c720082d78bacc2243
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-007
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:23:46Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-checks-the-whole-history-fab6e3
    claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:2b27c07c98bb144af4254700c369d7c8e83b9d616e83fa0e45d352a5649df830
    case: TC-cli-ship-checks-the-whole-history
    case_digest: bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:23:49Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-5f558b
    claim: "Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:c28fd66f1b023f58315b1e752572f6d074669aab60f46b4995ae46b740a82c02
    case: TC-cli-ship-acts-on-the-pinned-session
    case_digest: 416c0ce58c7d0afd7055a2c22b7cab4af4fdc66d1201a4a655ff53b53448b1ef
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:23:51Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-c4bc56
    claim: "Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:d8de037b1ea0fc25516b519719fa6c4889be26d57cf27002ad63e7ba6a72ac37
    case: TC-cli-session-use-and-new-say-when-a-pin-names-another-session
    case_digest: d9c6fb3e3e8febc59bbef3b3e30266bcc5f4fb4c09b851462740d2c4fe8ba75c
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:23:54Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-session-shelve-resolves-its-key-76b5f2
    claim: "Ran cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does for TC-cli-session-shelve-resolves-its-key: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact session_shelve_resolves_its_key_as_session_use_does
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:58d25598a8bf97db4588bc03445cf7ce33ebeceb9ff8f80d679732564418e015
    case: TC-cli-session-shelve-resolves-its-key
    case_digest: 0f10f9ab4c89450e62496a7acfa8cf76b6891d813a7f900e5425263a75f3dc4d
    criteria: DRIVE-3
    requirements: REQ-cli-037
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:23:56Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-in-a-shallow-clone-names-fetch-unshallow-0c0883
    claim: "Ran cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow for TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_in_a_shallow_clone_without_the_base_names_fetch_unshallow
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:363bad3c0a6460fae2488f0eb4c0fd1ff24dbfddd4861eff5900fe16f2a811d6
    case: TC-cli-ship-in-a-shallow-clone-names-fetch-unshallow
    case_digest: 4772c1d8f53608ca711c8fa72e8e9f508e532e1cd100aadccb94b588c43850ee
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:23:58Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-change-commands-refuse-a-leaking-argument-450552
    claim: "Ran cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs for TC-cli-change-commands-refuse-a-leaking-argument: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact change_commands_refuse_a_leaking_argument_before_specsync_runs
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:cd1230d716e39d190b6c981e9e830c5363d678e258eb5d248696c8372bf0a7fd
    case: TC-cli-change-commands-refuse-a-leaking-argument
    case_digest: 5ef68d263d84f4d077ad328bbc007bf2239b4589c68f5d6ee91350d0b1fcbe2b
    criteria: DRIVE-3
    requirements: REQ-cli-057
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:00Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-keeps-the-session-it-started-from
    claim: "Ran cargo test --locked --test change -- --exact ship_keeps_the_session_it_started_from_when_the_pointer_moves for TC-cli-ship-keeps-the-session-it-started-from: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_keeps_the_session_it_started_from_when_the_pointer_moves
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:c9412dda04f3e9b1c0d53b0f4264be47f9a2d750b1cd143016871b805d4f109e
    case: TC-cli-ship-keeps-the-session-it-started-from
    case_digest: 8be99612cc944962c998cf5e285bab715040c47559fb21ad08eac0c6e0e735ac
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:03Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-refusals-do-not-echo-a-credential-id
    claim: "Ran cargo test --locked --test change -- --exact ship_refusals_do_not_echo_a_credential_id for TC-cli-ship-refusals-do-not-echo-a-credential-id: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_refusals_do_not_echo_a_credential_id
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:a11e7bf8959060136381a0af42e9f64b7315e14ad6df1b41b3ee88e795990b6b
    case: TC-cli-ship-refusals-do-not-echo-a-credential-id
    case_digest: 995f4394b5b03a4b0bc2867e427fc23e3d19cc20aa6af03daf90c30a5beae9e2
    criteria: DRIVE-3
    requirements: REQ-cli-057
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:05Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-names-an-unreadable-commit
    claim: "Ran cargo test --locked --test change -- --exact ship_with_an_unreadable_commit_names_fetch_refetch for TC-cli-ship-names-an-unreadable-commit: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_with_an_unreadable_commit_names_fetch_refetch
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:498453e1659a47938936ecc87536028f3f541c30b4e0c11264aae54a0dd55803
    case: TC-cli-ship-names-an-unreadable-commit
    case_digest: d5691cd7be664d02e1c2dadc748673d3cafc2a6a8af74e5de2cf70cd94ba8f8c
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:08Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-ship-with-a-base-commit-that-is-not-an-object-id
    claim: "Ran cargo test --locked --test change -- --exact ship_with_a_base_commit_that_is_not_an_object_id_goes_on for TC-cli-ship-with-a-base-commit-that-is-not-an-object-id: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact ship_with_a_base_commit_that_is_not_an_object_id_goes_on
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:306bc6d20a7a7b045150716928e2f39e8724582bbf99c046e378f38450929c7a
    case: TC-cli-ship-with-a-base-commit-that-is-not-an-object-id
    case_digest: 59972040bf39a184f70acbc94a4d77422119eed98433a80726c6cd6c0c0a5d67
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:11Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-the-refetch-command-names-the-real-remote
    claim: "Ran cargo test --locked --test change -- --exact the_refetch_command_names_the_real_remote_and_works for TC-cli-the-refetch-command-names-the-real-remote: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact the_refetch_command_names_the_real_remote_and_works
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:81a32ec5779f9b2e3c26e48cbab1578c9483f61e4a98a756bcbdbae2e8cf7a5a
    case: TC-cli-the-refetch-command-names-the-real-remote
    case_digest: 6d88c02567ee7284d165d5993790c58a7d4545bb94089463166ea175c92f4886
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:15Z
    duration_seconds: 3
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-several-untracked-remotes-are-named
    claim: "Ran cargo test --locked --test change -- --exact several_untracked_remotes_are_named_to_choose_from for TC-cli-several-untracked-remotes-are-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact several_untracked_remotes_are_named_to_choose_from
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:1538adf48fcae851c9c3d3c692b647d9eb34abe6bc610b8d805172971de67626
    case: TC-cli-several-untracked-remotes-are-named
    case_digest: f05681de19c72c675616082a6d23120cf968ca7507dd2363d1a206db13f9ee9c
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:18Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-cli-an-unreadable-change-folder-tree-refuses
    claim: "Ran cargo test --locked --test change -- --exact an_unreadable_change_folder_tree_is_never_read_as_absent for TC-cli-an-unreadable-change-folder-tree-refuses: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact an_unreadable_change_folder_tree_is_never_read_as_absent
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:1692fbe541ea1ba465f55420ad192cca8712ae32fe39aa3e363c621a40781daa
    case: TC-cli-an-unreadable-change-folder-tree-refuses
    case_digest: 1e6ffce9c44dd49dce72184a679c7212012cbb8970e65fe4a8d97b9e5b38921e
    criteria: DRIVE-3
    requirements: REQ-cli-059
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:21Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-19-tc-history-inbox-list-lists-the-pinned-session-a1215c
    claim: "Ran cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session for TC-history-inbox-list-lists-the-pinned-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test session_pin -- --exact inbox_list_lists_the_pinned_session
    revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
    environment: local-macos-arm64
    content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
    fingerprint: sha256:9f41503694949d9536ab6dd099e6e94f5653fef128b894b97b62bd920b568f14
    case: TC-history-inbox-list-lists-the-pinned-session
    case_digest: b29b0dd986237b002d1a95dee4c6fb8871381d4383e9eaf94c1f8636674bc357
    criteria: DRIVE-3, DRIVE-24
    requirements: REQ-history-009
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:24:23Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
```
