---
kind: session
id: 2026-11-20-406-rebase-own-work
aliases: [406-rebase-own-work]
objective: chough change ready counts only the change's own work after a rebase onto main or a squash merge of main, so main's files are not named as outside the declared paths (#406, item 2)
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
pr: "419"
criteria: []
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase
  - 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge
  - 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files
  - 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly
  - 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase-306018
  - 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-c155e2
  - 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-e4c078
  - 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-1a346c
  - 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase-20e93e
  - 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-ebe4e2
  - 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-368cdf
  - 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-e3d8cf
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
title: change ready counts only a change's own work after a rebase
summary:
  - "chough change ready no longer names the default branch's files as outside the declared paths after a rebase onto it or a squash merge of it: a committed file whose content at HEAD is the default branch's, at its tip or at the merge base, is not the change's own, never on the default branch itself; a missing base commit is said plainly (#406)."
  - "Verified at df9914cd: eleven verify steps and four cases on macOS; the tester's PASS (rebased branch 690 and 684 findings down to 9 and 3, every dropped one main's; 66 changes, none dropped); tester2's Linux PASS; corvid-agent's approval; fable-lead's read; main 643420a9 merged in at 0b50b7aa, own lines unchanged, lane green."
---
# chough change ready counts only the change's own work after a rebase onto main or a squash merge of main, so main's files are not named as outside the declared paths (#406, item 2)

## Goal

chough change ready counts only the change's own work after a rebase onto main or a squash merge of main, so main's files are not named as outside the declared paths (#406, item 2)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main (native approval)": "Approve (Recommended)" (Leif's own answer, chosen in a Claude Code question dialog in his planning session (hc t4c) on 2026-11-20 at about 04:16Z, to the question: "Chough #406 (rebase part): ... Approve its scope with the four recommended choices (A: decide by content; B: default branch with #1081's protections; C: a missing starting commit is named; D: several changes on one branch left for later)?" Recorded by t4c itself, as his relayed answer, at the draft 5d492341); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main as 0xLeif
- 0xLeif decided "Implementation review of fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main": "Go" (Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. At df9914cd: the tester's PASS on the rebased branch (690 and 684 findings down to 9 and 3, every dropped one main's) and on the 66 changes (none dropped), tester2's Linux PASS with Git 2.39, corvid-agent's approval, fable-lead's read; no approved sentence changed by the build. Then main 643420a9 (#418) merged in at 0b50b7aa: t4b's script shows its own 469 changed lines byte-identical before and after, the cli spec only renumbered to 127, and the lane green again. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change review fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase.md)
- [Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge.md)
- [Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files.md)
- [Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly.md)
- [Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase-306018.md)
- [Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-c155e2.md)
- [Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-e4c078.md)
- [Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-1a346c.md)
- [Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase-20e93e.md)
- [Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-ebe4e2.md)
- [Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-368cdf.md)
- [Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-e3d8cf.md)

## Open threads

## Timeline

- 2026-11-20T04:06:05Z · start · intent → intent · chough change ready counts only the change's own work after a rebase onto main or a squash merge of main, so main's files are not named as outside the declared paths (#406, item 2) ^t-520ac2
  - revision: 1fb67c29295cc2472285bbd03b6a10cf8c5f6c40
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T04:19:29Z · gate-opened · intent → intent · Scope approval of fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main (native approval) ^t-fb9f0d
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - revision: 5d492341e2540bb73687bef8aeb644a01095d2b6
  - holds: intent
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T04:19:29Z · gate-closed · intent → intent · Scope approval of fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main (native approval) ^t-ddcf8c
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - revision: 5d492341e2540bb73687bef8aeb644a01095d2b6
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve (Recommended)
    - where: Leif's own answer, chosen in a Claude Code question dialog in his planning session (hc t4c) on 2026-11-20 at about 04:16Z, to the question: "Chough #406 (rebase part): ... Approve its scope with the four recommended choices (A: decide by content; B: default branch with #1081's protections; C: a missing starting commit is named; D: several changes on one branch left for later)?" Recorded by t4c itself, as his relayed answer, at the draft 5d492341
    - recorded_by: t4c
    - native: specsync change approve fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T04:20:55Z · advance · intent → design ·  ^t-3d853e
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - revision: e2f3380e758001e8ffd117e6c669ecf8dfc922ea
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T04:24:02Z · advance · design → plan ·  ^t-12cfae
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - revision: e2f3380e758001e8ffd117e6c669ecf8dfc922ea
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T04:24:09Z · advance · plan → build · native:approved ^t-e7409a
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - gate: native:approved
  - revision: e2f3380e758001e8ffd117e6c669ecf8dfc922ea
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T14:16:31Z · advance · build → test · native:verifying ^t-e9096b
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - gate: native:verifying
  - revision: 0b50b7aaf73452f835f072d8419ee85b08173ae5
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T14:16:31Z · advance · test → accept · native:verifying ^t-2073a5
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - gate: native:verifying
  - revision: 0b50b7aaf73452f835f072d8419ee85b08173ae5
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T14:16:31Z · gate-opened · accept → accept · Implementation review of fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main ^t-e00ea6
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - revision: 0b50b7aaf73452f835f072d8419ee85b08173ae5
  - holds: accept
  - drives: review
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T14:17:22Z · gate-closed · accept → accept · Implementation review of fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main ^t-a3192d
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - revision: cbae0be5e38b674a83bce1669ae8536efaa67f8a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. At df9914cd: the tester's PASS on the rebased branch (690 and 684 findings down to 9 and 3, every dropped one main's) and on the 66 changes (none dropped), tester2's Linux PASS with Git 2.39, corvid-agent's approval, fable-lead's read; no approved sentence changed by the build. Then main 643420a9 (#418) merged in at 0b50b7aa: t4b's script shows its own 469 changed lines byte-identical before and after, the cli spec only renumbered to 127, and the lane green again. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change review fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T14:17:23Z · advance · accept → done · native:archived ^t-86ca9d
  - native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
  - gate: native:archived
  - revision: cbae0be5e38b674a83bce1669ae8536efaa67f8a
  - by: t4c (claude-opus-5-5, hc t4c)
