---
kind: session
id: 2026-11-20-bug-add-leaves-the-session
aliases: [bug-add-leaves-the-session]
objective: "bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)"
modules: [cli, history]
stage: done
stage_status: archived
iteration: 1
native_change: bug-add-leaves-the-session
pr: "422"
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012, REQ-cli-054, REQ-history-006]
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
closed: 2026-11-20
outcome: done
---
# bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

## Goal

bug add records the session on the new bug and never writes the session, so parallel filing merges and filing costs the same at any size (#382, #395)

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of bug-add-leaves-the-session": "A: on the bug (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough R2a (#382, #395, two regressions from #343). Where should a bug's "filed in this session" link live?'; on R2a bug-add-leaves-the-session at 51a848c6 with choice A); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve bug-add-leaves-the-session as 0xLeif
- 0xLeif decided "Implementation review of bug-add-leaves-the-session": "Go" (Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #422 at 97c4c1ed: the tester's harness, run by the fixer at 9ea50230, with R2A-395 fixed (0.03 s per bug add at 3 and at 203 observations) and the 22 real repositories unchanged against main; R2A-382 shows a clean merge with no doctor finding, and fails only its one pre-approval check, which reads session.md's text, while the approved criterion (the merged checkpoint lists both bugs) passes in tests/bugs.rs and on a copy of jackdaw; copies of jackdaw, pico-8 and spec-sync pass the run on the real repositories; corvid-agent APPROVED 97c4c1ed (2026-11-20 about 16:42Z). No approved sentence was changed by the build; the scope carries Leif's 'A: on the bug (Recommended)'. fable-lead is paused until about 2026-11-23, so t4c is told after, and can hold the merge until a tester aligns R2A-382.); recorded by chough-fixer-t4n; a relayed claim, not authenticated; ran specsync change review bug-add-leaves-the-session as 0xLeif with verdict pass

## What was verified

- 7 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1). - 4cfbf534fcd9; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-bug-add-leaves-the-session-record-503b6b
- Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1). - e40d3ffc1025; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly-eca047
- Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1). - f8dc9741aecc; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint-04e584
- Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1). - 4bde595b4e35; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined-7a1ed6
- Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1). - a58801672895; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones-b63f9e
- Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1). - 0eeaadca9626; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session-e3fdb8
- Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1). - 961fb56f62a2; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-bug-add-joins-the-active-session-0df608

## Bugs

None.

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

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-20-tc-cli-bug-add-leaves-the-session-record
    claim: "Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record
    revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
    environment: local-macos-arm64
    content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
    fingerprint: sha256:802fade65057e811b36ff54dd97b44484d33b0ee05cdc7d56fd79c6652f8dcd6
    case: TC-cli-bug-add-leaves-the-session-record
    case_digest: 5865d20fc1880780d79a943f924712df017375d0e2d58e01bcfe04507f94bb22
    criteria: LIFE-5, DRIVE-12
    requirements: REQ-cli-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:48:24Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly
    claim: "Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly
    revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
    environment: local-macos-arm64
    content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
    fingerprint: sha256:731c6f2548394ab08961bb7e9fc991155b9aad1b2061a9b64f32a0a481b76586
    case: TC-cli-bug-adds-on-two-branches-merge-cleanly
    case_digest: 9a381267c6fa4636c79c97b4c76bf43849335d876d2b49bcac4fa41ab826696b
    criteria: LIFE-5, DRIVE-12
    requirements: REQ-cli-012, REQ-cli-054
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:48:26Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint
    claim: "Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs
    revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
    environment: local-macos-arm64
    content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
    fingerprint: sha256:1a5c070da9b8a67bfb064629edc9ad6fb56e34b9446e0987cc195b2dcca6df48
    case: TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint
    case_digest: 4191fe071fb4b202a3f94f260e5368f80bb5a7dfa484076b83de7def3e83fcfd
    criteria: LIFE-5
    requirements: REQ-cli-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:48:28Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined
    claim: "Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined
    revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
    environment: local-macos-arm64
    content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
    fingerprint: sha256:12ebc4fcd0d7b751d96c4d66b74a382c25ca4b51812d1642ee225b0bbdffba85
    case: TC-cli-an-update-leaves-a-filed-bug-unjoined
    case_digest: f4e6963593f982c31173b059805ac0f02375862d71524cccbaa96bfaa479d11c
    criteria: LIFE-5, DRIVE-12
    requirements: REQ-cli-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:48:31Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing
    revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
    environment: local-macos-arm64
    content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
    fingerprint: sha256:b9c74961c48ac9af1c0331fd668e56152002c191ccacd63af33f88a7afba3cb6
    case: TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones
    case_digest: 2b866350a4c2ce5a8a9a45933c8c00a6a2e5b76512703347e027e335c6cae80e
    criteria: LIFE-5
    requirements: REQ-cli-012, REQ-cli-054
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:48:33Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session
    claim: "Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session
    revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
    environment: local-macos-arm64
    content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
    fingerprint: sha256:2a38035de23e1b8c3988781548595a766f90ef74906b8da1f1a51f3d345a5770
    case: TC-history-the-story-names-a-bug-filed-in-the-session
    case_digest: cbcf83f40b37912bb08aa22baa458ea469ac3453dd3169aa8d67f039fcfc00ab
    criteria: LIFE-5
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:48:36Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-bug-add-joins-the-active-session
    claim: "Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it
    revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
    environment: local-macos-arm64
    content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
    fingerprint: sha256:1748b149a496169a3a072098c8dde7f7e0c9d5d0e5c87ae52d66d20297608993
    case: TC-cli-bug-add-joins-the-active-session
    case_digest: e57e15da79b42470a8595b74e0f3a4a9b5cab2537380d39af458cb8765d9c386
    criteria: LIFE-5
    requirements: REQ-cli-012, REQ-cli-054
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:48:38Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-bug-add-leaves-the-session-record-503b6b
    claim: "Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record
    revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
    environment: local-macos-arm64
    content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
    fingerprint: sha256:4cfbf534fcd93e24c40e6541ecd5097bea316ea200a5fd4da62ee532a3c6ed27
    case: TC-cli-bug-add-leaves-the-session-record
    case_digest: 5865d20fc1880780d79a943f924712df017375d0e2d58e01bcfe04507f94bb22
    criteria: LIFE-5, DRIVE-12
    requirements: REQ-cli-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:34:39Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly-eca047
    claim: "Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly
    revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
    environment: local-macos-arm64
    content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
    fingerprint: sha256:e40d3ffc1025d23648c0ee7a1dc1d048d49c8c4b9e7e650761586302c45872ff
    case: TC-cli-bug-adds-on-two-branches-merge-cleanly
    case_digest: 9a381267c6fa4636c79c97b4c76bf43849335d876d2b49bcac4fa41ab826696b
    criteria: LIFE-5, DRIVE-12
    requirements: REQ-cli-012, REQ-cli-054
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:34:41Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-bug-add-files-a-bug-beside-a-full-checkpoint-04e584
    claim: "Ran cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs for TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact bug_add_files_the_bug_in_a_session_whose_checkpoint_holds_200_bugs
    revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
    environment: local-macos-arm64
    content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
    fingerprint: sha256:f8dc9741aecc18843914b0994aa6dbdae882fabbc81a6a506c548ff9b33ccbb6
    case: TC-cli-bug-add-files-a-bug-beside-a-full-checkpoint
    case_digest: 4191fe071fb4b202a3f94f260e5368f80bb5a7dfa484076b83de7def3e83fcfd
    criteria: LIFE-5
    requirements: REQ-cli-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:34:43Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-an-update-leaves-a-filed-bug-unjoined-7a1ed6
    claim: "Ran cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined for TC-cli-an-update-leaves-a-filed-bug-unjoined: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact an_update_naming_an_active_entry_leaves_a_filed_bug_unjoined
    revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
    environment: local-macos-arm64
    content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
    fingerprint: sha256:4bde595b4e3506af4501013bb899a4e47f44cb98f8f0026d712053126e793716
    case: TC-cli-an-update-leaves-a-filed-bug-unjoined
    case_digest: f4e6963593f982c31173b059805ac0f02375862d71524cccbaa96bfaa479d11c
    criteria: LIFE-5, DRIVE-12
    requirements: REQ-cli-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:34:45Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones-b63f9e
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing
    revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
    environment: local-macos-arm64
    content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
    fingerprint: sha256:a5880167289511fc9bc52070e195cf8c12c122455c198d14feaea92433fbd589
    case: TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones
    case_digest: 2b866350a4c2ce5a8a9a45933c8c00a6a2e5b76512703347e027e335c6cae80e
    criteria: LIFE-5
    requirements: REQ-cli-012, REQ-cli-054
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:34:47Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-the-story-names-a-bug-filed-in-the-session-e3fdb8
    claim: "Ran cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session for TC-history-the-story-names-a-bug-filed-in-the-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact the_story_names_a_bug_filed_in_the_session
    revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
    environment: local-macos-arm64
    content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
    fingerprint: sha256:0eeaadca9626c38254f9779bc3e0cbd3a52061f28c63f26749cd3307b3830001
    case: TC-history-the-story-names-a-bug-filed-in-the-session
    case_digest: cbcf83f40b37912bb08aa22baa458ea469ac3453dd3169aa8d67f039fcfc00ab
    criteria: LIFE-5
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:34:49Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-bug-add-joins-the-active-session-0df608
    claim: "Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it
    revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
    environment: local-macos-arm64
    content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
    fingerprint: sha256:961fb56f62a29bcdf3ab4ad0a3b5477d544941596af4b651a521aca8049e6844
    case: TC-cli-bug-add-joins-the-active-session
    case_digest: e57e15da79b42470a8595b74e0f3a4a9b5cab2537380d39af458cb8765d9c386
    criteria: LIFE-5
    requirements: REQ-cli-012, REQ-cli-054
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T16:34:51Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
```
