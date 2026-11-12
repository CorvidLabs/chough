---
kind: session
id: 2026-11-07-issue-65-evidence-prune-race
aliases: [issue-65-evidence-prune-race]
objective: Make evidence prune --apply fail closed and restore partial removals (#65)
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
pr: ""
criteria: [LIFE-8, LIFE-2, LIFE-7]
requirements: [REQ-history-004]
cases:
  - TC-history-prune-protection-race
  - TC-history-prune-restores-partial-removal
  - TC-history-prune-restores-earlier-sets
  - TC-history-prune-restore-preserves-recreated-file
design: []
files: []
observations:
  - OBS-tc-history-prune-protection-race-16793e9e4c84b7f00004251a
  - OBS-tc-history-prune-restores-earlier-sets-bba515e3dbd42cea1399e660
  - OBS-tc-history-prune-restore-preserves-recreated-file-fe5568108c710f47994da0c7
  - OBS-tc-history-prune-restores-partial-removal-9d8c4669e6b99ad1f7b3504d
  - OBS-tc-history-prune-protection-race-831610f271a785164c912a6e
  - OBS-tc-history-prune-restores-earlier-sets-42efe7ab63bba4851815235f
  - OBS-tc-history-prune-restore-preserves-recreated-file-ec625ca74058487a167adf05
  - OBS-tc-history-prune-restores-partial-removal-c5ff0e975581686f00f9f078
  - OBS-product-test-972af92fbd4cb64f5dd3094a
  - OBS-product-test-51a832506d132b756faaf82a
  - OBS-product-test-f87bec1f3802297bc62bebe7
  - OBS-product-test-cf146b8281569e3b55420c53
  - OBS-product-test-8596f4c916b301c4ec8853cd
  - OBS-structural-b66cb4a28fd68f089b2a01b3
  - OBS-product-test-1a8d3cd9e2fff7f17314bd35
  - OBS-tc-history-prune-protection-race-d117515c4e78a708145abbbb
  - OBS-tc-history-prune-restores-partial-removal-281599d68d18010c1b54cdd3
  - OBS-tc-history-prune-restores-earlier-sets-5869e7a3e14302d5d2bac1da
  - OBS-tc-history-prune-restore-preserves-recreated-file-485fc2b3eb590a24b9ba8820
  - OBS-structural-0569129fb63cffe3aa2b53bf
  - OBS-structural-1b635c037853e3ce2f70d10b
  - OBS-structural-4bb245473772e0d89afd2868
  - OBS-structural-01fdbe567f85ef177eeea144
  - OBS-structural-ec5e908e555a15be59cc7e27
  - OBS-product-test-177ef9b1ad9bc144d0b26aa4
  - OBS-product-test-766be385e3010d3b004a21f6
bugs:
  - BUG-evidence-prune-loses-removals-when-protections-change-f0aabb199383ff63877394c2
  - BUG-evidence-prune-does-not-restore-partial-removal-14338a2c0a36ae54472c1b07
  - BUG-approved-prune-requirement-delta-leaves-split-digest-stale-51af3f652531624b8e771636
  - BUG-reapproved-prune-inputs-lack-current-native-verification-b5e5ca5c85f4d7672d2d43f1
blockers: []
waiting_on_human: []
run_consents: []
next: "Leif implementation review through orc t19f of the one #65 PR at the final native-checked head; no worker approval/review/ship/merge. All evidence and gate writes are complete before the final change check."
closed: 2026-11-07
outcome: done
receipts:
  - id: 2026-11-07-tc-history-prune-protection-race
    aliases: OBS-tc-history-prune-protection-race-16793e9e4c84b7f00004251a
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes for TC-history-prune-protection-race: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes
    revision: 3ad9df48933b880836c812c06369d00b9369e918
    environment: local-macos-arm64
    content: db3db4c3f6021d81312953b0bc5cf8fd5d1d5ca5e682fa02bc8db977c78e3a96
    fingerprint: sha256:8349a7581aba8bab3d75a6a99b023fb9eb5063e7e42cdd653d6ffedb8f959217
    case: TC-history-prune-protection-race
    case_digest: 763949fbada75217dc6f170254ae8a0313bff28996820116e8b9097159f1df00
    observed: 2026-11-07T03:24:16Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-restores-earlier-sets
    aliases: OBS-tc-history-prune-restores-earlier-sets-bba515e3dbd42cea1399e660
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_restores_earlier_sets_when_a_later_set_becomes_protected for TC-history-prune-restores-earlier-sets: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_restores_earlier_sets_when_a_later_set_becomes_protected
    revision: 3ad9df48933b880836c812c06369d00b9369e918
    environment: local-macos-arm64
    content: db3db4c3f6021d81312953b0bc5cf8fd5d1d5ca5e682fa02bc8db977c78e3a96
    fingerprint: sha256:62270aeb93736aad7dbb8655d352090c33750c2b5d6bc9b2f992d3a7af63f621
    case: TC-history-prune-restores-earlier-sets
    case_digest: 171ed7d2d36db96fd78c5d34b7ffa93e16bf5c0403fb267924b90bd1d0ab61fd
    observed: 2026-11-07T03:24:22Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-restore-preserves-recreated-file
    aliases: OBS-tc-history-prune-restore-preserves-recreated-file-fe5568108c710f47994da0c7
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_restore_preserves_recreated_file_and_uncommitted_original for TC-history-prune-restore-preserves-recreated-file: failed (exit 101)."
    result: failed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_restore_preserves_recreated_file_and_uncommitted_original
    revision: 3ad9df48933b880836c812c06369d00b9369e918
    environment: local-macos-arm64
    content: db3db4c3f6021d81312953b0bc5cf8fd5d1d5ca5e682fa02bc8db977c78e3a96
    fingerprint: sha256:2cc8cf88624c42b6d20d99d110ac9aa6d5bfea814af51250bd0a19b34ef5dc26
    case: TC-history-prune-restore-preserves-recreated-file
    case_digest: bb57cfda6d4595d4797054fe06a429bfa00f41b10593bfd196bb9cc5e278847c
    observed: 2026-11-07T03:24:31Z
    duration_seconds: 0
  - id: 2026-11-07-tc-history-prune-restores-partial-removal
    aliases: OBS-tc-history-prune-restores-partial-removal-9d8c4669e6b99ad1f7b3504d
    claim: "Ran cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal for TC-history-prune-restores-partial-removal: failed (exit 101)."
    result: failed
    command: cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal
    revision: 3ad9df48933b880836c812c06369d00b9369e918
    environment: local-macos-arm64
    content: db3db4c3f6021d81312953b0bc5cf8fd5d1d5ca5e682fa02bc8db977c78e3a96
    fingerprint: sha256:68002027fad3d48796db36407ca4274a39c8200540898326746d902600222929
    case: TC-history-prune-restores-partial-removal
    case_digest: 7293ab7ec230fc592298ec9915eaab8b78054de3265729abcff9834a7d4e2af2
    observed: 2026-11-07T03:24:47Z
    duration_seconds: 9
  - id: 2026-11-07-tc-history-prune-protection-race-d33d41
    aliases: OBS-tc-history-prune-protection-race-831610f271a785164c912a6e
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes for TC-history-prune-protection-race: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact evidence::tests::prune_protection_race_restores_removed_bytes
    revision: 55cb7fa5662fd6bc67b32757d67242df8ed121dd
    environment: local-macos-arm64
    content: da0c63e939b46491710343634b2abce74b5a5a3e68a747d6470db21efda19748
    fingerprint: sha256:769b457288a4fca8f7d8b4d24a6f631dbcb392005a50f8f741d31b8371b151ab
    case: TC-history-prune-protection-race
    case_digest: 763949fbada75217dc6f170254ae8a0313bff28996820116e8b9097159f1df00
    observed: 2026-11-07T03:35:06Z
    duration_seconds: 1
  - id: OBS-tc-history-prune-restores-earlier-sets-42efe7ab63bba4851815235f
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-history-prune-restore-preserves-recreated-file-ec625ca74058487a167adf05
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-07-tc-history-prune-restores-partial-removal-5788f1
    aliases: OBS-tc-history-prune-restores-partial-removal-c5ff0e975581686f00f9f078
    claim: "Ran cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal for TC-history-prune-restores-partial-removal: passed (exit 0)."
    result: passed
    command: cargo test --locked --test evidence -- --exact evidence_prune_restores_partial_removal
    revision: 55cb7fa5662fd6bc67b32757d67242df8ed121dd
    environment: local-macos-arm64
    content: da0c63e939b46491710343634b2abce74b5a5a3e68a747d6470db21efda19748
    fingerprint: sha256:83cfea1d15344d944e8dc295dcce846032fa6a2eba3f66e6b8f148f4140e6825
    case: TC-history-prune-restores-partial-removal
    case_digest: 7293ab7ec230fc592298ec9915eaab8b78054de3265729abcff9834a7d4e2af2
    observed: 2026-11-07T03:35:59Z
    duration_seconds: 3
  - id: OBS-product-test-972af92fbd4cb64f5dd3094a
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-product-test-51a832506d132b756faaf82a
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-07-fledge-run-linux-verify-at
    aliases: OBS-product-test-f87bec1f3802297bc62bebe7
    claim: "fledge run linux-verify at 7febd53a86d1a007565ff38638b41abd760c123f failed test step 3: moved_requirements_keep_their_wording_under_their_new_ids still pins old REQ-history-004 body; exact old/new SHA256 in log. No assertion changed."
    result: failed
    revision: 7febd53a86d1a007565ff38638b41abd760c123f
    environment: local-linux-aarch64
    content: 4c668870ced52a9c7a062d91efe5eab149c877b1b996c41748c2cf558353ab87
    fingerprint: sha256:a69c9b00e67effaeb197f7158f3ccf5b1505acad866afb29fbdce654ca6754d8
    observed: 2026-11-07T06:50:48Z
  - id: OBS-product-test-cf146b8281569e3b55420c53
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-product-test-8596f4c916b301c4ec8853cd
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-structural-b66cb4a28fd68f089b2a01b3
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-07-pinned-focused-validation-fledge-1-8-0-hi-0-8-0-specsync-6-2-0
    aliases: OBS-product-test-1a8d3cd9e2fff7f17314bd35
    claim: "Pinned focused validation: Fledge 1.8.0, HI 0.8.0, SpecSync 6.2.0, Rust 1.95.0. Build, all 4 spec_split tests, 11 evidence unit tests, 30 evidence integration tests, 25 history tests and global case-definition validation passed. The formerly failing strict REQ-history-004 assertion now passes with the exact approved digest."
    result: passed
    revision: 4ef6b8e29913077a6f557d1371a930540e02884b
    environment: local-macos-arm64
    content: 7357063ec647a2c8493d700dc99e2ebaa99d0f01043e3bd400bdacef867bc44e
    fingerprint: sha256:9dc22d60f02451b332ba4c856bd41f10b7e2a0211707384932c7bf882c123009
    observed: 2026-11-07T14:54:44Z
  - id: OBS-tc-history-prune-protection-race-d117515c4e78a708145abbbb
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-history-prune-restores-partial-removal-281599d68d18010c1b54cdd3
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-history-prune-restores-earlier-sets-5869e7a3e14302d5d2bac1da
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-history-prune-restore-preserves-recreated-file-485fc2b3eb590a24b9ba8820
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-07-pinned-linux-verify-at
    aliases: OBS-structural-0569129fb63cffe3aa2b53bf
    claim: "Pinned Linux verify at b4b4dae7208570d63b613a0d5e638e42aa975e3a passed the complete product suite and strict spec check, then failed lifecycle audit step 6: src/evidence.rs and tests/evidence.rs reported uncovered despite valid approved scope. Native next action remains change check since reapproval."
    result: failed
    revision: b4b4dae7208570d63b613a0d5e638e42aa975e3a
    environment: local-linux-aarch64
    content: 324e9aaa8a63ba86fb6cb31608a5c5e08962a9011615ba239e32bba746b7718d
    fingerprint: sha256:c661c7de6e18bb98b1b9db257e1c3acaa3f6aa3315f25e11587f9888061a352e
    observed: 2026-11-07T15:00:05Z
  - id: OBS-structural-1b635c037853e3ce2f70d10b
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-07-preparatory-chough-change-check-passed-and-committed-current
    aliases: OBS-structural-4bb245473772e0d89afd2868
    claim: Preparatory Chough change check passed and committed current native verification at 3e1198b4cea01fdbbbb6904a58dce94529dfc29e. specsync change audit --strict, all five pre-push steps and knowledge index check then passed. Product source/spec/docs/test files are byte-identical to b4b4dae7208570d63b613a0d5e638e42aa975e3a.
    result: passed
    revision: 3e1198b4cea01fdbbbb6904a58dce94529dfc29e
    environment: local-macos-arm64
    content: 63720fce57a8bb4623f7ed805347e8968256c838e8889ab57c883f2ccdc48b45
    fingerprint: sha256:81d3a5cee25bbb7f02215f723414ee2474fdb7429b7bbd8f472ba811ad72be7a
    observed: 2026-11-07T15:02:43Z
  - id: 2026-11-07-linux-full-rerun-at-9ebacabe170eee2cf7009c2d181d3093da73ae6d
    aliases: OBS-structural-01fdbe567f85ef177eeea144
    claim: Linux full rerun at 9ebacabe170eee2cf7009c2d181d3093da73ae6d passed product tests and strict specs, then audit rejected stale native verification. The recovery evidence/index commit after the preparatory check invalidated the verification digest; no product source changed. Retain all receipts and freeze all inputs after the next check.
    result: failed
    revision: 9ebacabe170eee2cf7009c2d181d3093da73ae6d
    environment: local-linux-aarch64
    content: 9c3435ba717cb1dfc6540a9c1cd6ab23a4582fcbf14128ac8fa1dbfb6dc3e083
    fingerprint: sha256:b141efcb8d9172b114baa30852d976ce57207c41e6572069ac39e79a60e8a590
    observed: 2026-11-07T15:07:56Z
  - id: 2026-11-07-macos-full-rerun-at-9ebacabe170eee2cf7009c2d181d3093da73ae6d
    aliases: OBS-structural-ec5e908e555a15be59cc7e27
    claim: macOS full rerun at 9ebacabe170eee2cf7009c2d181d3093da73ae6d passed product tests and strict specs, then audit rejected stale native verification. The recovery evidence/index commit after the preparatory check invalidated the verification digest; no product source changed. Retain all receipts and freeze all inputs after the next check.
    result: failed
    revision: 9ebacabe170eee2cf7009c2d181d3093da73ae6d
    environment: local-macos-arm64
    content: 9c3435ba717cb1dfc6540a9c1cd6ab23a4582fcbf14128ac8fa1dbfb6dc3e083
    fingerprint: sha256:b5a7fb10187ea7a999447d4f25e4860ada8e9da5546f0cf3eef34cbe33419e67
    observed: 2026-11-07T15:07:58Z
  - id: OBS-product-test-177ef9b1ad9bc144d0b26aa4
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-07-complete-pinned-linux-verify-lane-passed-all-10-steps-at
    aliases: OBS-product-test-766be385e3010d3b004a21f6
    claim: "Complete pinned linux verify lane passed all 10 steps at frozen commit 8d5bcf430921bf14c8557214354f1928f2358cc3: 597 product tests; formatting/lint, intent, strict specs with 100% coverage, native audit, case definitions, knowledge, real CLI exercise and build. No workspace writes occurred during either platform run."
    result: passed
    revision: 8d5bcf430921bf14c8557214354f1928f2358cc3
    environment: local-linux-aarch64
    content: e7ee35372197946f77f0ab0be149f6769b5c14768ed2cf881715441f85004e62
    fingerprint: sha256:237bb6f9034df4db9f9d76ddb985cb72727af9c4099f220ddc12524c68ce5cbb
    observed: 2026-11-07T15:14:17Z
---
# Make evidence prune --apply fail closed and restore partial removals (#65)

## Goal

Make evidence prune --apply fail closed and restore partial removals (#65)

## Where it stands

- Confirmed on origin/main 455cb643 that evidence prune still deletes directly without rollback, while library archive supplies remove_verified and restored ledger events; existing LIFE-8 covers the requested behavior
- Completed the #65 native draft with four deterministic failure-first regressions, byte-checked removal, create-new rollback and uncommitted-byte recovery holds; SpecSync reports artifacts_complete true
- Added cli ownership with a minimal Purpose delta for src/evidence.rs; a disposable worktree probe accepted the cli and history deltas with approval_valid true, and the probe and its non-human approval record were removed
- Failing tests: 3ad9df48933b880836c812c06369d00b9369e918; all four direct regressions failed for the intended behavior and their failed cases are linked to local bugs
- All four prune regressions passed after implementation; 10 evidence unit, 30 evidence integration and 25 history tests passed, and both local bugs are verified
- Failure first: prune_restore_permissions_preserve_replacement_identity ran 1 test, failed mode assertion 0640 versus 0600 before the restore publication fix; receipt restore-identity-before.log
- Failing tests: bf57eb259b17a2ed5634b5ef62f4b1520f8bdd93; rollback replacement chmod changes 0600 to 0640, linked to existing partial-removal bug
- Focused rollback tests pass after private publication; baseline and all three mutation checks verified. Linux full verify failed the omitted strict split pin; macOS full verify interrupted for scope amendment. No full platform lane passed.
- Pinned focused checks passed: build; 4 strict split regressions; 11 evidence unit, 30 evidence integration and 25 history tests; all case definitions valid; the four approved #65 cases passed including replacement inode/mode/bytes.
- Pinned product suites passed: macOS 593 tests and Linux 597 tests, both with strict spec checks at 100% coverage. Both complete lanes failed lifecycle audit step 6 before the reapproved native check; failure receipts preserved.
- Native verification after reapproval resolved the audit coverage gap; strict audit, pre-push and knowledge passed with source/test/spec/docs unchanged from b4b4dae7208570d63b613a0d5e638e42aa975e3a. Rerun both full lanes before implementation review.
- Both reruns at 9ebacabe170eee2cf7009c2d181d3093da73ae6d passed product/spec but failed stale native verification because recovery bookkeeping followed the preparatory check. All failures retained. Next check must follow all metadata/index writes; then freeze all inputs through full lanes.
- Full pinned macOS verify (593 product tests, all 10 steps) and Linux verify (597 product tests, all 10 steps) passed at 8d5bcf430921bf14c8557214354f1928f2358cc3. No source/spec/docs/test changes after b4b4dae7208570d63b613a0d5e638e42aa975e3a. All prior failures preserved; Trust unavailable and optional scope probe not run as directed.

## Decisions and why

- agent: existing LIFE-8, LIFE-2 and LIFE-7 already cover #65 because they require proof protection and honest removed-versus-passed states, so no new HI criterion is needed
- rejected: changing retention eligibility, protections, bounds, output schemas or authorization because #65 is a bounded race and rollback bug fix under REQ-history-004
- rejected: refactoring the proven library archive path because the small fix can reuse remove_verified and the existing restored event shape without changing archive behavior
- Leif decided "Scope decision of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals": "Yes to all 3" (Moshi question from orc t18, 2026-11-06; existing approval relayed by orc t19f); recorded by sol-ch65; a relayed claim, not authenticated
- Leif decided "Scope reapproval of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals: add tests/spec_split.rs only": "Yes" (Direct chat reply to orc t19f three-gate batch, 2026-11-07); recorded by sol-ch65; a relayed claim, not authenticated
- Agent clarification: the frozen platform runs had no edits to tracked or non-ignored verification inputs, and Git stayed clean; builds and tools still wrote their ignored output/cache files.
- Leif decided "Implementation review of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals": "yes" (Direct Leif reply to orc t19f ONE-item #65/#197 implementation acceptance gate, 2026-11-07); recorded by sol-ch65; a relayed claim, not authenticated

## What was verified

- OBS-tc-history-prune-protection-race-16793e9e4c84b7f00004251a
- OBS-tc-history-prune-restores-earlier-sets-bba515e3dbd42cea1399e660
- OBS-tc-history-prune-restore-preserves-recreated-file-fe5568108c710f47994da0c7
- OBS-tc-history-prune-restores-partial-removal-9d8c4669e6b99ad1f7b3504d
- OBS-tc-history-prune-protection-race-831610f271a785164c912a6e
- OBS-tc-history-prune-restores-earlier-sets-42efe7ab63bba4851815235f
- OBS-tc-history-prune-restore-preserves-recreated-file-ec625ca74058487a167adf05
- OBS-tc-history-prune-restores-partial-removal-c5ff0e975581686f00f9f078
- OBS-product-test-972af92fbd4cb64f5dd3094a
- OBS-product-test-51a832506d132b756faaf82a
- OBS-product-test-f87bec1f3802297bc62bebe7
- OBS-product-test-cf146b8281569e3b55420c53
- OBS-product-test-8596f4c916b301c4ec8853cd
- OBS-structural-b66cb4a28fd68f089b2a01b3
- OBS-product-test-1a8d3cd9e2fff7f17314bd35
- OBS-tc-history-prune-protection-race-d117515c4e78a708145abbbb
- OBS-tc-history-prune-restores-partial-removal-281599d68d18010c1b54cdd3
- OBS-tc-history-prune-restores-earlier-sets-5869e7a3e14302d5d2bac1da
- OBS-tc-history-prune-restore-preserves-recreated-file-485fc2b3eb590a24b9ba8820
- OBS-structural-0569129fb63cffe3aa2b53bf
- OBS-structural-1b635c037853e3ce2f70d10b
- OBS-structural-4bb245473772e0d89afd2868
- OBS-structural-01fdbe567f85ef177eeea144
- OBS-structural-ec5e908e555a15be59cc7e27
- OBS-product-test-177ef9b1ad9bc144d0b26aa4
- OBS-product-test-766be385e3010d3b004a21f6

## Open threads

## Timeline

- 2026-11-07T02:50:26Z · advance · intent → design ·  ^t-792115
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 455cb643cd91a9c804afdf5566e3889abf1bde49
- 2026-11-07T02:50:26Z · skip · design → plan · No visual or interaction design changes: this is an internal evidence-retention failure-handling fix with unchanged command shapes ^t-24e9b2
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 455cb643cd91a9c804afdf5566e3889abf1bde49
- 2026-11-07T02:56:48Z · gate-opened · plan → plan · Scope decision of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals ^t-190e1c
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 455cb643cd91a9c804afdf5566e3889abf1bde49
- 2026-11-07T06:41:58Z · gate-closed · plan → plan · Scope decision of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals ^t-f0589b
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 716a966fc4c22c5853f29a60a2f0c179cb0dc9a0
  - relayed_by: {"decided_by":"Leif","words":"Yes to all 3","where":"Moshi question from orc t18, 2026-11-06; existing approval relayed by orc t19f","recorded_by":"sol-ch65"}
- 2026-11-07T06:42:52Z · direct-native-write · plan → plan ·  ^t-2a7726
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: d9d7b12695b52622efe29932fd98ddada6eb0086
  - direct_write: {"change":"make-evidence-prune-apply-recheck-protections-and-restore-partial-removals","files":[{"path":"state.json","change":"changed"}],"before_state":"draft","after_state":"draft"}
- 2026-11-07T06:42:52Z · direct-native-write · plan → plan ·  ^t-6f36f8
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: d9d7b12695b52622efe29932fd98ddada6eb0086
  - direct_write: {"change":"make-evidence-prune-apply-recheck-protections-and-restore-partial-removals","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"draft","after_state":"approved"}
- 2026-11-07T06:42:52Z · advance · plan → build · native:verifying ^t-f0739d
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - gate: native:verifying
  - revision: d9d7b12695b52622efe29932fd98ddada6eb0086
- 2026-11-07T06:51:20Z · back · build → plan · Orc t19f directed scope amendment for the required tests/spec_split.rs digest pin after the declared Linux lane demonstrated its stale assertion ^t-3b7f4c
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 7febd53a86d1a007565ff38638b41abd760c123f
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, checkpoint references: 4 case(s) and 9 observation(s) to re-check with `chough status`, native: the definition is verifying; changing intent, contract, acceptance criteria or scope needs native reapproval. Follow `specsync change status make-evidence-prune-apply-recheck-protections-and-restore-partial-removals`; Chough does not change native state.
- 2026-11-07T06:52:18Z · gate-opened · plan → plan · Scope reapproval of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals: add tests/spec_split.rs only ^t-ca0fa2
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 7febd53a86d1a007565ff38638b41abd760c123f
- 2026-11-07T06:52:18Z · direct-native-write · plan → plan ·  ^t-55d6d1
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 7febd53a86d1a007565ff38638b41abd760c123f
  - direct_write: {"change":"make-evidence-prune-apply-recheck-protections-and-restore-partial-removals","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"verifying","after_state":"draft"}
- 2026-11-07T06:52:18Z · iterate · plan → plan · Orc t19f directed one-path scope amendment after the declared Linux lane failed the unrefreshed requirement pin; no implementation pin edit or approval/review ^t-9e76c5
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 7febd53a86d1a007565ff38638b41abd760c123f
- 2026-11-07T14:52:40Z · gate-closed · plan → plan · Scope reapproval of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals: add tests/spec_split.rs only ^t-4f8c65
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 4ef6b8e29913077a6f557d1371a930540e02884b
  - relayed_by: {"decided_by":"Leif","words":"Yes","where":"Direct chat reply to orc t19f three-gate batch, 2026-11-07","recorded_by":"sol-ch65"}
- 2026-11-07T14:52:41Z · direct-native-write · plan → plan ·  ^t-bf71f8
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 4ef6b8e29913077a6f557d1371a930540e02884b
  - direct_write: {"change":"make-evidence-prune-apply-recheck-protections-and-restore-partial-removals","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"draft","after_state":"approved"}
- 2026-11-07T14:52:41Z · advance · plan → build · native:approved ^t-4efb7c
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - gate: native:approved
  - revision: 4ef6b8e29913077a6f557d1371a930540e02884b
- 2026-11-07T14:55:26Z · advance · build → test · native:approved ^t-24a7c1
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - gate: native:approved
  - revision: 4ef6b8e29913077a6f557d1371a930540e02884b
- 2026-11-07T15:14:23Z · advance · test → accept · native:verifying ^t-206e70
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - gate: native:verifying
  - revision: 8d5bcf430921bf14c8557214354f1928f2358cc3
- 2026-11-07T15:14:23Z · gate-opened · accept → accept · Implementation review of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals ^t-c1a153
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: 8d5bcf430921bf14c8557214354f1928f2358cc3
- 2026-11-07T15:51:24Z · gate-closed · accept → accept · Implementation review of make-evidence-prune-apply-recheck-protections-and-restore-partial-removals ^t-a537df
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: a3261f8a82760a7f1b490142fc831b7f33b7fa79
  - relayed_by: {"decided_by":"Leif","words":"yes","where":"Direct Leif reply to orc t19f ONE-item #65/#197 implementation acceptance gate, 2026-11-07","recorded_by":"sol-ch65"}
- 2026-11-07T15:51:32Z · direct-native-write · accept → accept ·  ^t-d5ddab
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - revision: a3261f8a82760a7f1b490142fc831b7f33b7fa79
  - direct_write: {"change":"make-evidence-prune-apply-recheck-protections-and-restore-partial-removals","files":[{"path":"review.json","change":"added"},{"path":"review-attempts.json","change":"added"}],"before_state":"verifying","after_state":"verifying"}
- 2026-11-07T15:51:32Z · advance · accept → done · native:archived ^t-599195
  - native_change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
  - gate: native:archived
  - revision: a3261f8a82760a7f1b490142fc831b7f33b7fa79


Closed 2026-11-07: done.
