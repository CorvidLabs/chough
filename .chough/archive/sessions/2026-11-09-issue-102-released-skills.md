---
kind: session
id: 2026-11-09-issue-102-released-skills
aliases: [issue-102-released-skills]
objective: Recognize every pristine skill file from pinned released bundles while preserving all user edits
intent_scope: FLOW
modules: [cli]
stage: intent
stage_status: active
iteration: 1
callers: []
native_change: recognize-pristine-skills-from-pinned-released-bundles-without-replacing-edits
pr: ""
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
cases:
  - TC-cli-release-corpus-identity
  - TC-cli-release-pristine-status
  - TC-cli-release-pristine-update
  - TC-cli-release-edited-preservation
  - TC-cli-release-name-byte-matching
  - TC-cli-release-host-current-missing
  - TC-cli-release-legacy-policy
design: []
files: [src/skill.rs]
observations:
  - OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b
  - OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e
  - OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56
  - OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7
  - OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619
  - OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23
  - OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3
  - OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62
  - OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314
bugs:
  - BUG-pinned-released-skill-files-are-misclassified-as-edits-because-six-pred-2448f259d21efce039cd3e9b
blockers: []
waiting_on_human:
  - gate: Independent scoped review and human implementation acceptance remain pending; no review/ship authorization is recorded.
    holds: intent
    drives: none
    opened: 2026-11-09T05:14:35Z
run_consents: []
next: Finish allocated INDEX-only generation/native-current freeze then different-author scoped review; human implementation acceptance pending
closed: 2026-11-09
outcome: closed-without-done
receipts:
  - id: 2026-11-09-tc-cli-release-pristine-status
    aliases: OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b
    claim: Retained actual t2ns 2026-11-09T04:46:29.660928+00:00; source6bb513b66e488359a7995c4f1eb764d6496a555e; /usr/bin/nice -n10 Fledge1.8 run test -- --locked --test skill_releases ran7,5pass/2fail. every_pinned_release_file_is_recognized_before_update FAILED at54 pristine host/file recognition/update assertions (six missing predecessor pairs). Recorder t2nr pre-Cargo setup failure is separate. Original evidence/chough102-compiled-red/MANIFEST.json and actual-red.log retained; no re-execution now.
    result: failed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-pristine-status
    case_digest: 23399f86eb9d87156f73ceab34cc0643dceca757bb00a5bc27d456d96a13fa8d
    observed: 2026-11-09T05:14:12Z
  - id: 2026-11-09-tc-cli-release-pristine-update
    aliases: OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e
    claim: Retained actual t2ns 2026-11-09T04:46:29.660928+00:00; source6bb513b66e488359a7995c4f1eb764d6496a555e; /usr/bin/nice -n10 Fledge1.8 run test -- --locked --test skill_releases ran7,5pass/2fail. every_pristine_release_updates_instead_of_staying_a_preserved_edit FAILED at54 pristine host/file recognition/update assertions (six missing predecessor pairs). Recorder t2nr pre-Cargo setup failure is separate. Original evidence/chough102-compiled-red/MANIFEST.json and actual-red.log retained; no re-execution now.
    result: failed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-pristine-update
    case_digest: 8e6776b6769ec0f675efcef3544a458350f8a86a9032189a678d49744cb79370
    observed: 2026-11-09T05:14:14Z
  - id: 2026-11-09-tc-cli-release-corpus-identity
    aliases: OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, released_corpus_identity_rejects_tampering_before_cli_invocation passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-corpus-identity
    case_digest: 0907fe020f9f87844fe327342567e5ef5bae5704d5a37ba59aac39e619a41bcb
    observed: 2026-11-09T05:14:16Z
  - id: 2026-11-09-tc-cli-release-pristine-status-4d921c
    aliases: OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, every_pinned_release_file_is_recognized_before_update passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-pristine-status
    case_digest: 23399f86eb9d87156f73ceab34cc0643dceca757bb00a5bc27d456d96a13fa8d
    observed: 2026-11-09T05:14:18Z
  - id: 2026-11-09-tc-cli-release-pristine-update-55048e
    aliases: OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, every_pristine_release_updates_instead_of_staying_a_preserved_edit passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-pristine-update
    case_digest: 8e6776b6769ec0f675efcef3544a458350f8a86a9032189a678d49744cb79370
    observed: 2026-11-09T05:14:20Z
  - id: 2026-11-09-tc-cli-release-edited-preservation
    aliases: OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, user_edited_release_bytes_and_modes_survive_every_host_update passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-edited-preservation
    case_digest: dc997d6bf201c933d7781dbc968ed4de7305698db1c811e3c26a15f34dc5c4a0
    observed: 2026-11-09T05:14:23Z
  - id: 2026-11-09-tc-cli-release-name-byte-matching
    aliases: OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, recognition_requires_the_filename_and_exact_unnormalized_bytes passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-name-byte-matching
    case_digest: a5acedd9a4928ad9ee4b772ed63a878118e614703a574b6996c4d19960f89387
    observed: 2026-11-09T05:14:25Z
  - id: 2026-11-09-tc-cli-release-host-current-missing
    aliases: OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, current_missing_and_shared_host_routing_stay_unchanged passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-host-current-missing
    case_digest: 6c1065735bc06c8ab7ab71e7f5ffe8b2930b051353eae17648205e397d5b0188
    observed: 2026-11-09T05:14:27Z
  - id: 2026-11-09-tc-cli-release-legacy-policy
    aliases: OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314
    claim: Retained actual t2p7 at2026-11-09T05:04:53.416163+00:00; sourced9a9d92459a611eeca6a587fb83e130b95c91dee; SAME Fledge1.8 run test -- --locked --test skill_releases:7/7 PASS, legacy_status_update_and_explicit_removal_keep_the_existing_policy passed. Exact Rust1.95/source/test/corpus identities in evidence/chough102-focused-green;720 edited rows/modes and all filename/host/current/missing/legacy boundaries retain assertions. Additional existing4+3+16, fmt/lint passed; no fresh execution while recording.
    result: passed
    revision: d9a9d92459a611eeca6a587fb83e130b95c91dee
    environment: local-macos-arm64
    content: 06c26c73395526350b28c0d3d3b313c73e0e82af8ebd55c21c007c5f065bea8f
    case: TC-cli-release-legacy-policy
    case_digest: d386e03ca8cccb1eb2e99ff760f89061c9578aa1b2ccd387aa1335f50c674f3a
    observed: 2026-11-09T05:14:29Z
---
# Recognize every pristine skill file from pinned released bundles while preserving all user edits

## Goal

Recognize every pristine skill file from pinned released bundles while preserving all user edits

## Where it stands

- Retained t2ns actual compiled RED then t2p7 SAME7/7 plus23 unchanged controls/fmtlint; no fresh run while recording receipts

## Decisions and why

## What was verified

- OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b
- OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e
- OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56
- OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7
- OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619
- OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23
- OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3
- OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62
- OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314

## Open threads

## Timeline

- 2026-11-09T05:14:35Z · gate-opened · intent → intent · Independent scoped review and human implementation acceptance remain pending; no review/ship authorization is recorded. ^t-25127f
  - native_change: recognize-pristine-skills-from-pinned-released-bundles-without-replacing-edits
  - revision: d9a9d92459a611eeca6a587fb83e130b95c91dee


Closed 2026-11-09: closed-without-done.
