---
kind: session
id: 2026-11-20-bug-add-leaves-the-session
aliases: [bug-add-leaves-the-session]
objective: "bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)"
intent_scope: ""
modules: [cli, history]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: bug-add-leaves-the-session
pr: "422"
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012, REQ-cli-054, REQ-history-006]
cases: []
design: []
files: []
observations:
  - 2026-11-20-tc-cli-bug-add-leaves-the-session-record
  - 2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly
  - 2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint
  - 2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined
  - 2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones
  - 2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session
  - 2026-11-20-tc-cli-bug-add-joins-the-active-session
  - 2026-11-20-tc-cli-bug-add-leaves-the-session-record-503b6b
  - 2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly-eca047
  - 2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint-04e584
  - 2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined-7a1ed6
  - 2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones-b63f9e
  - 2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session-e3fdb8
  - 2026-11-20-tc-cli-bug-add-joins-the-active-session-0df608
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

## Goal

bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of bug-add-leaves-the-session": "A: on the bug (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough R2a (#382, #395, two regressions from #343). Where should a bug's "filed in this session" link live?'; on R2a bug-add-leaves-the-session at 51a848c6 with choice A); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve bug-add-leaves-the-session as 0xLeif
- 0xLeif decided "Implementation review of bug-add-leaves-the-session": "Go" (Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #422 at 97c4c1ed: the tester's harness, run by the fixer at 9ea50230, with R2A-395 fixed (0.03 s per bug add at 3 and at 203 observations) and the 22 real repositories unchanged against main; R2A-382 shows a clean merge with no doctor finding, and fails only its one pre-approval check, which reads session.md's text, while the approved criterion (the merged checkpoint lists both bugs) passes in tests/bugs.rs and on a copy of jackdaw; copies of jackdaw, pico-8 and spec-sync pass the run on the real repositories; corvid-agent APPROVED 97c4c1ed (2026-11-20 about 16:42Z). No approved sentence was changed by the build; the scope carries Leif's 'A: on the bug (Recommended)'. fable-lead is paused until about 2026-11-23, so t4c is told after, and can hold the merge until a tester aligns R2A-382.); recorded by chough-fixer-t4n; a relayed claim, not authenticated; ran specsync change review bug-add-leaves-the-session as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-leaves-the-session-record.md)
- [Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly.md)
- [Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint.md)
- [Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones.md)
- [Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session.md)
- [Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-joins-the-active-session.md)
- [Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-leaves-the-session-record-503b6b.md)
- [Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly-eca047.md)
- [Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint-04e584.md)
- [Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined-7a1ed6.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones-b63f9e.md)
- [Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session-e3fdb8.md)
- [Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-joins-the-active-session-0df608.md)

## Open threads

## Timeline

- 2026-11-20T11:18:44Z · start · intent → intent · bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395) ^t-199ddf
  - revision: 056cbb8029f6535ff74f866a0523893916f682db
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T11:19:10Z · advance · intent → design ·  ^t-e6d040
  - revision: 056cbb8029f6535ff74f866a0523893916f682db
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T11:19:15Z · skip · design → plan · Nothing visual changes: bug add stops writing the session and a bug records the session it was filed in. ^t-a49bfd
  - revision: 056cbb8029f6535ff74f866a0523893916f682db
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:25:36Z · gate-opened · plan → plan · Approve the scope of bug-add-leaves-the-session ^t-e41443
  - native_change: bug-add-leaves-the-session
  - revision: 51a848c6029a17796df8b75c4a3cd9e051ca7774
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:25:36Z · gate-closed · plan → plan · Approve the scope of bug-add-leaves-the-session ^t-b2943a
  - native_change: bug-add-leaves-the-session
  - revision: 51a848c6029a17796df8b75c4a3cd9e051ca7774
  - relayed_by:
    - decided_by: 0xLeif
    - words: A: on the bug (Recommended)
    - where: Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough R2a (#382, #395, two regressions from #343). Where should a bug's "filed in this session" link live?'; on R2a bug-add-leaves-the-session at 51a848c6 with choice A
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve bug-add-leaves-the-session
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:18:26Z · advance · plan → build · native:approved ^t-41ac22
  - native_change: bug-add-leaves-the-session
  - gate: native:approved
  - revision: 5db3b73de910f88edf637f2a58ca014405030f5c
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:03:12Z · advance · build → test · native:verifying ^t-a60cb1
  - native_change: bug-add-leaves-the-session
  - gate: native:verifying
  - revision: 97c4c1ed4f6d16c11f45b84b24b17fd5e16daf5f
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:03:13Z · advance · test → accept · native:verifying ^t-1fd996
  - native_change: bug-add-leaves-the-session
  - gate: native:verifying
  - revision: 97c4c1ed4f6d16c11f45b84b24b17fd5e16daf5f
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:03:34Z · gate-opened · accept → accept · Implementation review of bug-add-leaves-the-session ^t-ba5c14
  - native_change: bug-add-leaves-the-session
  - revision: 97c4c1ed4f6d16c11f45b84b24b17fd5e16daf5f
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:04:25Z · gate-closed · accept → accept · Implementation review of bug-add-leaves-the-session ^t-c51674
  - native_change: bug-add-leaves-the-session
  - revision: af24022963564c3661872b9f8ec1c03e4cb40de3
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #422 at 97c4c1ed: the tester's harness, run by the fixer at 9ea50230, with R2A-395 fixed (0.03 s per bug add at 3 and at 203 observations) and the 22 real repositories unchanged against main; R2A-382 shows a clean merge with no doctor finding, and fails only its one pre-approval check, which reads session.md's text, while the approved criterion (the merged checkpoint lists both bugs) passes in tests/bugs.rs and on a copy of jackdaw; copies of jackdaw, pico-8 and spec-sync pass the run on the real repositories; corvid-agent APPROVED 97c4c1ed (2026-11-20 about 16:42Z). No approved sentence was changed by the build; the scope carries Leif's 'A: on the bug (Recommended)'. fable-lead is paused until about 2026-11-23, so t4c is told after, and can hold the merge until a tester aligns R2A-382.
    - recorded_by: chough-fixer-t4n
    - native: specsync change review bug-add-leaves-the-session
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:04:26Z · advance · accept → done · native:archived ^t-e49a14
  - native_change: bug-add-leaves-the-session
  - gate: native:archived
  - revision: af24022963564c3661872b9f8ec1c03e4cb40de3
  - by: chough-fixer (claude-opus-5-5, hc t4n)
