---
kind: session
id: 2026-11-20-bug-add-leaves-the-session
aliases: [bug-add-leaves-the-session]
objective: "bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)"
intent_scope: ""
modules: [cli, history]
stage: build
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
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue build
---
# bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

## Goal

bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of bug-add-leaves-the-session": "A: on the bug (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough R2a (#382, #395, two regressions from #343). Where should a bug's "filed in this session" link live?'; on R2a bug-add-leaves-the-session at 51a848c6 with choice A); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve bug-add-leaves-the-session as 0xLeif

## What was verified

- [Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-leaves-the-session-record.md)
- [Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly.md)
- [Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint.md)
- [Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones.md)
- [Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session.md)
- [Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-bug-add-joins-the-active-session.md)

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
