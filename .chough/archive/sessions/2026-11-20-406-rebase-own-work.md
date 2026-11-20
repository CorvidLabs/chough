---
kind: session
id: 2026-11-20-406-rebase-own-work
aliases: [406-rebase-own-work]
objective: chough change ready counts only the change's own work after a rebase onto main or a squash merge of main, so main's files are not named as outside the declared paths (#406, item 2)
stage: done
stage_status: archived
iteration: 1
native_change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
pr: "419"
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
title: change ready counts only a change's own work after a rebase
summary:
  - "chough change ready no longer names the default branch's files as outside the declared paths after a rebase onto it or a squash merge of it: a committed file whose content at HEAD is the default branch's, at its tip or at the merge base, is not the change's own, never on the default branch itself; a missing base commit is said plainly (#406)."
  - "Verified at df9914cd: eleven verify steps and four cases on macOS; the tester's PASS (rebased branch 690 and 684 findings down to 9 and 3, every dropped one main's; 66 changes, none dropped); tester2's Linux PASS; corvid-agent's approval; fable-lead's read; main 643420a9 merged in at 0b50b7aa, own lines unchanged, lane green."
closed: 2026-11-20
outcome: done
---
# chough change ready counts only the change's own work after a rebase onto main or a squash merge of main, so main's files are not named as outside the declared paths (#406, item 2)

## Goal

chough change ready counts only the change's own work after a rebase onto main or a squash merge of main, so main's files are not named as outside the declared paths (#406, item 2)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main (native approval)": "Approve (Recommended)" (Leif's own answer, chosen in a Claude Code question dialog in his planning session (hc t4c) on 2026-11-20 at about 04:16Z, to the question: "Chough #406 (rebase part): ... Approve its scope with the four recommended choices (A: decide by content; B: default branch with #1081's protections; C: a missing starting commit is named; D: several changes on one branch left for later)?" Recorded by t4c itself, as his relayed answer, at the draft 5d492341); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main as 0xLeif
- 0xLeif decided "Implementation review of fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main": "Go" (Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. At df9914cd: the tester's PASS on the rebased branch (690 and 684 findings down to 9 and 3, every dropped one main's) and on the 66 changes (none dropped), tester2's Linux PASS with Git 2.39, corvid-agent's approval, fable-lead's read; no approved sentence changed by the build. Then main 643420a9 (#418) merged in at 0b50b7aa: t4b's script shows its own 469 changed lines byte-identical before and after, the cli spec only renumbered to 127, and the lane green again. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change review fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main as 0xLeif with verdict pass

## What was verified

- 8 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1). - 01ea6d174a24; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase-20e93e
- Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1). - 8a34995c5814; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-ebe4e2
- Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1). - e7abfb9ed95c; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-368cdf
- Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1). - 013aed0570a4; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-e3d8cf

## Bugs

None.

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

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase
    claim: "Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named
    revision: 874d3a4e8cb923c923381debbcc7e45603fc0f7b
    environment: local-macos-arm64
    content: 360b47091367fe398daf5ee42a7b0f7e64c534cf2947b0f34445012bdb69d358
    fingerprint: sha256:4fe4b367a97ca4928652a424120ff2dfe04cb4459a523d5a999e9c91f5fc2603
    case: TC-cli-change-ready-counts-only-own-work-after-a-rebase
    case_digest: e001c17066461d24c97a6896e5db031cfcf315baae14097591a43c893633e734
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:32:36Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge
    claim: "Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named
    revision: 874d3a4e8cb923c923381debbcc7e45603fc0f7b
    environment: local-macos-arm64
    content: 360b47091367fe398daf5ee42a7b0f7e64c534cf2947b0f34445012bdb69d358
    fingerprint: sha256:d54be2eba13980c57856726b4cea3b43a44236166d6028e00f1f30b27cdd7bd8
    case: TC-cli-change-ready-counts-only-own-work-after-a-squash-merge
    case_digest: 6c4edb31439a53fd6a2068ef21b1b47fa6479329c6ed274e245c3f9170e6d8c8
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:32:41Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files
    claim: "Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named
    revision: 874d3a4e8cb923c923381debbcc7e45603fc0f7b
    environment: local-macos-arm64
    content: 360b47091367fe398daf5ee42a7b0f7e64c534cf2947b0f34445012bdb69d358
    fingerprint: sha256:0176451202936e5de908baf71868ba0a2f30f8dde7707d8637b889b73fd35f6f
    case: TC-cli-change-ready-on-the-default-branch-names-own-files
    case_digest: 0cb9bc7a1551cc2ca552005b982baf6dcb01712bb51e8fbfd1c12d8a3b0a2f12
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:32:49Z
    duration_seconds: 5
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly
    claim: "Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly
    revision: 874d3a4e8cb923c923381debbcc7e45603fc0f7b
    environment: local-macos-arm64
    content: 360b47091367fe398daf5ee42a7b0f7e64c534cf2947b0f34445012bdb69d358
    fingerprint: sha256:e91cc8d6752be72849bfe228c99d8b68397b306980a8ae6caea11ba03698e15f
    case: TC-cli-change-ready-names-a-missing-base-plainly
    case_digest: d6851ef47f5c60f0fba435275e008a048d4eba48aa7aaef9def0095aff34494f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T04:32:55Z
    duration_seconds: 4
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase-306018
    claim: "Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named
    revision: 9c79d7694223aa3acc2027e8f95d03edb888ab16
    environment: local-macos-arm64
    content: a5023893b74f5a383e037635a88a59ffabfdf2c3a6e06852324b3460eec94e03
    fingerprint: sha256:6e2c5bbee2997e262c2a1f0374023cb7a466fc2a691336e47be31fba30a4fe13
    case: TC-cli-change-ready-counts-only-own-work-after-a-rebase
    case_digest: e001c17066461d24c97a6896e5db031cfcf315baae14097591a43c893633e734
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:41:36Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-c155e2
    claim: "Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named
    revision: 9c79d7694223aa3acc2027e8f95d03edb888ab16
    environment: local-macos-arm64
    content: a5023893b74f5a383e037635a88a59ffabfdf2c3a6e06852324b3460eec94e03
    fingerprint: sha256:9132ab4629055d040be9a2808759f230caf1dd6a8640ece0d9e9af35ae3d93bf
    case: TC-cli-change-ready-counts-only-own-work-after-a-squash-merge
    case_digest: 6c4edb31439a53fd6a2068ef21b1b47fa6479329c6ed274e245c3f9170e6d8c8
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:41:38Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-e4c078
    claim: "Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named
    revision: 9c79d7694223aa3acc2027e8f95d03edb888ab16
    environment: local-macos-arm64
    content: a5023893b74f5a383e037635a88a59ffabfdf2c3a6e06852324b3460eec94e03
    fingerprint: sha256:35541848f4be7ec19356ef1f040cc2fbf636a78ec9cc17624ba06bf667359a10
    case: TC-cli-change-ready-on-the-default-branch-names-own-files
    case_digest: 0cb9bc7a1551cc2ca552005b982baf6dcb01712bb51e8fbfd1c12d8a3b0a2f12
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:41:41Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-1a346c
    claim: "Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly
    revision: 9c79d7694223aa3acc2027e8f95d03edb888ab16
    environment: local-macos-arm64
    content: a5023893b74f5a383e037635a88a59ffabfdf2c3a6e06852324b3460eec94e03
    fingerprint: sha256:91d16610945ef55d192ecadeb1f5c30b6bc623291ca9011fc9495b093f8e5a0d
    case: TC-cli-change-ready-names-a-missing-base-plainly
    case_digest: d6851ef47f5c60f0fba435275e008a048d4eba48aa7aaef9def0095aff34494f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T06:41:43Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase-20e93e
    claim: "Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named
    revision: ff13be893391d1866f1f881d014ec5d181c7e60f
    environment: local-macos-arm64
    content: 79d69754fcc4e759ac7714c82d28e0cf90a2841d03adf3a6e37f92b923396d4d
    fingerprint: sha256:01ea6d174a24fb44a479ac7d0050919e04b42abcee67791da4002be6605fe8c5
    case: TC-cli-change-ready-counts-only-own-work-after-a-rebase
    case_digest: e001c17066461d24c97a6896e5db031cfcf315baae14097591a43c893633e734
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:14:24Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-ebe4e2
    claim: "Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named
    revision: ff13be893391d1866f1f881d014ec5d181c7e60f
    environment: local-macos-arm64
    content: 79d69754fcc4e759ac7714c82d28e0cf90a2841d03adf3a6e37f92b923396d4d
    fingerprint: sha256:8a34995c581488a334fb0ea23f78152132fd867b91b92d4138c4abe58facb27c
    case: TC-cli-change-ready-counts-only-own-work-after-a-squash-merge
    case_digest: 6c4edb31439a53fd6a2068ef21b1b47fa6479329c6ed274e245c3f9170e6d8c8
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:14:27Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-368cdf
    claim: "Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named
    revision: ff13be893391d1866f1f881d014ec5d181c7e60f
    environment: local-macos-arm64
    content: 79d69754fcc4e759ac7714c82d28e0cf90a2841d03adf3a6e37f92b923396d4d
    fingerprint: sha256:e7abfb9ed95ce9c00b272cd9abecad9103e0081bb6bfcfe622e2be6da0a12868
    case: TC-cli-change-ready-on-the-default-branch-names-own-files
    case_digest: 0cb9bc7a1551cc2ca552005b982baf6dcb01712bb51e8fbfd1c12d8a3b0a2f12
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:14:31Z
    duration_seconds: 2
    actor: fable (claude-opus-5-5, hc t4b)
  - id: 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly-e3d8cf
    claim: "Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly
    revision: ff13be893391d1866f1f881d014ec5d181c7e60f
    environment: local-macos-arm64
    content: 79d69754fcc4e759ac7714c82d28e0cf90a2841d03adf3a6e37f92b923396d4d
    fingerprint: sha256:013aed0570a4e8338fb9fd4fedd939cfade1c059771c88d93ad6c63f6f2baf7e
    case: TC-cli-change-ready-names-a-missing-base-plainly
    case_digest: d6851ef47f5c60f0fba435275e008a048d4eba48aa7aaef9def0095aff34494f
    criteria: DRIVE-29
    requirements: REQ-cli-067
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:14:34Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t4b)
```
