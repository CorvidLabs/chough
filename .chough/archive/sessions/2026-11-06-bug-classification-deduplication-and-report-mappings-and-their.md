---
kind: session
id: 2026-11-06-bug-classification-deduplication-and-report-mappings-and-their
aliases: [bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153]
objective: Bug classification, deduplication and report mappings, and their recovery branches (#153, audit Bugs row)
intent_scope: FLOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
pr: ""
criteria: [FLOW-8, LIFE-5]
requirements: [REQ-cli-012]
cases:
  - TC-cli-bug-values-accepted-others-refused
  - TC-cli-bug-only-open-blocking-blocks
  - TC-cli-bug-search-scores-and-refuses-likely-duplicates
  - TC-cli-bug-self-or-unknown-links-refused-with-recovery
  - TC-cli-bug-mistaken-duplicate-reopened
  - TC-cli-bug-report-shows-every-field
design: []
files: []
observations:
  - OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-7bb4a5f534267588823b6c95
  - OBS-tc-cli-bug-mistaken-duplicate-reopened-fb28dada965816100e5a4ccb
  - OBS-product-test-dff775cc31b7e3161bf2186d
  - OBS-tc-cli-bug-mistaken-duplicate-reopened-5b25cda941f9982b4a833f00
  - OBS-tc-cli-bug-only-open-blocking-blocks-adcc669269da23a9ea1d46e2
  - OBS-tc-cli-bug-report-shows-every-field-548b2ac9b3d7b795e9aefe63
  - OBS-tc-cli-bug-search-scores-and-refuses-likely-duplicates-6289660cafeab00be8740628
  - OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-4e8ec21f18075198237f2355
  - OBS-tc-cli-bug-values-accepted-others-refused-be3710283ea7041554b6a30d
bugs:
  - BUG-a-mistaken-duplicate-cannot-be-reopened-a-bug-can-relate-to-itself-and-2823f5504985675b64530948
blockers: []
waiting_on_human: []
run_consents: []
next: Wait for Leif's GitHub review through the orc; the orc records the native review, then this agent ships, finalizes, moves to done, pushes and merges on green plus corvid-agent
closed: 2026-11-06
outcome: done
receipts:
  - id: 2026-11-06-tc-cli-bug-self-or-unknown-links-refused-with-recovery
    aliases: OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-7bb4a5f534267588823b6c95
    claim: "Ran cargo test --locked --test bugs -- --exact links_to_the_bug_itself_or_to_an_unknown_bug_are_refused_with_their_recovery for TC-cli-bug-self-or-unknown-links-refused-with-recovery: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test bugs -- --exact links_to_the_bug_itself_or_to_an_unknown_bug_are_refused_with_their_recovery
    revision: fd52ed635432bb049b171c9f4dc03ffff39d79f9
    environment: local-macos-arm64
    content: 5a03cb0adb51d3a42ce993d39f59a745a243a3f1942f3a36b2a93455f00c0068
    fingerprint: sha256:e8f5f26f397862036b5cab1e5e8f74230175b2298d12a9869d729f416410a687
    case: TC-cli-bug-self-or-unknown-links-refused-with-recovery
    case_digest: 869a6cd7b00dcabca0e463bbbf52900640ef62323a33dad9dc635ef8a94b29fb
    observed: 2026-11-06T13:29:44Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-bug-mistaken-duplicate-reopened
    aliases: OBS-tc-cli-bug-mistaken-duplicate-reopened-fb28dada965816100e5a4ccb
    claim: "Ran cargo test --locked --test bugs -- --exact a_mistaken_duplicate_is_reopened_and_an_unverifiable_fix_names_its_recovery for TC-cli-bug-mistaken-duplicate-reopened: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test bugs -- --exact a_mistaken_duplicate_is_reopened_and_an_unverifiable_fix_names_its_recovery
    revision: fd52ed635432bb049b171c9f4dc03ffff39d79f9
    environment: local-macos-arm64
    content: 5a03cb0adb51d3a42ce993d39f59a745a243a3f1942f3a36b2a93455f00c0068
    fingerprint: sha256:847ece42bed8da12743a5e484c2c0f7533be2afa9e395d307016be786cb4504e
    case: TC-cli-bug-mistaken-duplicate-reopened
    case_digest: 55deda4ba0c5ee623250e1fd10b111b10c023f2c64794091a57f1efdacc6ad50
    observed: 2026-11-06T13:29:53Z
    duration_seconds: 2
  - id: OBS-product-test-dff775cc31b7e3161bf2186d
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-06-tc-cli-bug-mistaken-duplicate-reopened-75b416
    aliases: OBS-tc-cli-bug-mistaken-duplicate-reopened-5b25cda941f9982b4a833f00
    claim: "Ran cargo test --locked --test bugs -- --exact a_mistaken_duplicate_is_reopened_and_an_unverifiable_fix_names_its_recovery for TC-cli-bug-mistaken-duplicate-reopened: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_mistaken_duplicate_is_reopened_and_an_unverifiable_fix_names_its_recovery
    revision: 4edae0554e42cc3c4ead3424431388c251c557e3
    environment: local-macos-arm64
    content: d5b087fb4d8a7069f0af53f6bd3c649da8801d2b4a71c683fab6d9d72f5cce05
    fingerprint: sha256:1f8ff287e6cf826ea3270e90b93116ae3ddd8645f1e4872ca79056d22a8575c7
    case: TC-cli-bug-mistaken-duplicate-reopened
    case_digest: 55deda4ba0c5ee623250e1fd10b111b10c023f2c64794091a57f1efdacc6ad50
    observed: 2026-11-06T13:34:02Z
    duration_seconds: 4
  - id: 2026-11-06-tc-cli-bug-only-open-blocking-blocks
    aliases: OBS-tc-cli-bug-only-open-blocking-blocks-adcc669269da23a9ea1d46e2
    claim: "Ran cargo test --locked --test bugs -- --exact only_an_open_blocking_bug_blocks_leaving_test for TC-cli-bug-only-open-blocking-blocks: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact only_an_open_blocking_bug_blocks_leaving_test
    revision: 4edae0554e42cc3c4ead3424431388c251c557e3
    environment: local-macos-arm64
    content: d5b087fb4d8a7069f0af53f6bd3c649da8801d2b4a71c683fab6d9d72f5cce05
    fingerprint: sha256:111f9765549e01265ed4a2a3a0d4e846615d7e18fc6f9b60a6a138ce8bcc22fb
    case: TC-cli-bug-only-open-blocking-blocks
    case_digest: c9dafba98df618115f41b036fccb2b66d1057cbaf9d0ea49346aac76953a0423
    observed: 2026-11-06T13:33:47Z
    duration_seconds: 5
  - id: 2026-11-06-tc-cli-bug-report-shows-every-field
    aliases: OBS-tc-cli-bug-report-shows-every-field-548b2ac9b3d7b795e9aefe63
    claim: "Ran cargo test --locked --test bugs -- --exact the_report_shows_every_stored_field_and_none_recorded_for_empty_ones for TC-cli-bug-report-shows-every-field: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact the_report_shows_every_stored_field_and_none_recorded_for_empty_ones
    revision: 4edae0554e42cc3c4ead3424431388c251c557e3
    environment: local-macos-arm64
    content: d5b087fb4d8a7069f0af53f6bd3c649da8801d2b4a71c683fab6d9d72f5cce05
    fingerprint: sha256:6df50876f5f630d976f8f1d53ba166de97d18d604fcf1c16983a5bbfc4218c49
    case: TC-cli-bug-report-shows-every-field
    case_digest: c1620d7bad3c8558fb45b38ff806bc0cba3c90b3d0999cc67aaec3917b6089e4
    observed: 2026-11-06T13:34:11Z
    duration_seconds: 6
  - id: 2026-11-06-tc-cli-bug-search-scores-and-refuses-likely-duplicates
    aliases: OBS-tc-cli-bug-search-scores-and-refuses-likely-duplicates-6289660cafeab00be8740628
    claim: "Ran cargo test --locked --test bugs -- --exact search_scores_words_and_shared_ids_and_refuses_only_likely_duplicates for TC-cli-bug-search-scores-and-refuses-likely-duplicates: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact search_scores_words_and_shared_ids_and_refuses_only_likely_duplicates
    revision: 4edae0554e42cc3c4ead3424431388c251c557e3
    environment: local-macos-arm64
    content: d5b087fb4d8a7069f0af53f6bd3c649da8801d2b4a71c683fab6d9d72f5cce05
    fingerprint: sha256:f7a69329b49dd3bd738f56f7461700fbcc44067fdc3cc36c1a91a748220b0f62
    case: TC-cli-bug-search-scores-and-refuses-likely-duplicates
    case_digest: 2dc18f29cad6b21b7b2282f6bf27ca1e935cd83289f456200804f6b9c4df2e09
    observed: 2026-11-06T13:33:51Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-bug-self-or-unknown-links-refused-with-recovery-a31f75
    aliases: OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-4e8ec21f18075198237f2355
    claim: "Ran cargo test --locked --test bugs -- --exact links_to_the_bug_itself_or_to_an_unknown_bug_are_refused_with_their_recovery for TC-cli-bug-self-or-unknown-links-refused-with-recovery: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact links_to_the_bug_itself_or_to_an_unknown_bug_are_refused_with_their_recovery
    revision: 4edae0554e42cc3c4ead3424431388c251c557e3
    environment: local-macos-arm64
    content: d5b087fb4d8a7069f0af53f6bd3c649da8801d2b4a71c683fab6d9d72f5cce05
    fingerprint: sha256:6d38ce6d382538a6a274229babc7589539ddd7c4b8bc577ae31baa3010385fbb
    case: TC-cli-bug-self-or-unknown-links-refused-with-recovery
    case_digest: 869a6cd7b00dcabca0e463bbbf52900640ef62323a33dad9dc635ef8a94b29fb
    observed: 2026-11-06T13:33:54Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-bug-values-accepted-others-refused
    aliases: OBS-tc-cli-bug-values-accepted-others-refused-be3710283ea7041554b6a30d
    claim: "Ran cargo test --locked --test bugs -- --exact every_classification_scope_and_status_is_accepted_and_others_are_refused for TC-cli-bug-values-accepted-others-refused: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact every_classification_scope_and_status_is_accepted_and_others_are_refused
    revision: 4edae0554e42cc3c4ead3424431388c251c557e3
    environment: local-macos-arm64
    content: d5b087fb4d8a7069f0af53f6bd3c649da8801d2b4a71c683fab6d9d72f5cce05
    fingerprint: sha256:5467be78180ff31bacd2bf592c4c4038c760cc11135a10880f690261ce2a8669
    case: TC-cli-bug-values-accepted-others-refused
    case_digest: eae08a0db8a09adb178230fa46c9ee0451cdd862306dc57f9730fe9cb0b70755
    observed: 2026-11-06T13:33:39Z
    duration_seconds: 2
---
# Bug classification, deduplication and report mappings, and their recovery branches (#153, audit Bugs row)

## Goal

Bug classification, deduplication and report mappings, and their recovery branches (#153, audit Bugs row)

## Where it stands

- Change package drafted with REQ-cli-054; six tests, of which links_to_the_bug_itself_or_to_an_unknown_bug_are_refused_with_their_recovery and a_mistaken_duplicate_is_reopened_and_an_unverifiable_fix_names_its_recovery fail first on the defects; a prototype of the fix (reverted, kept as a patch) passes all 19 bug tests; 5 of 5 mapping mutations caught while the existing tests let them pass
- Six cases defined; the two failing-first cases recorded failing on fd52ed63 and linked to BUG-a-mistaken-duplicate-cannot-be-reopened-a-bug-can-relate-to-itself-and-2823f5504985675b64530948
- Fix applied (4edae055); all six cases pass on macOS; the bug is verified (reproduced, then passed); mutation round 5 of 5 caught (OBS-product-test-dff775cc31b7e3161bf2186d)

## Decisions and why

- Assignment from the orc (t18), deciding overnight for Leif (2026-11-06): the audit's Bugs row, with its own issue (#153), a worktree from origin/main, failing tests first, a mutation check, 'ready for approval' queued for Leif; REQ-cli-054 from the registry.
- Agent's recommendation, not a human decision: fix the two defects and the missing recovery hints in this change (small src/bugs.rs change), because the row's recovery branches cannot be exercised otherwise; the alternative is tests for the mappings only, with the defects filed apart.
- Leif (0xLeif) decided "Scope decision of bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153": "Ok yes I approve all" (orc tab, 2026-11-06 06:1x MST, in reply to the orc's morning brief, which put the A/B choice; relayed by the orc (hc terminal t18) as option A, with the src/bugs.rs fix; the orc recorded the native approval as 0xLeif at 1f822bc4); recorded by ch-c456 (Claude Opus 5.5); a relayed claim, not authenticated
- decided: option A, fix the two defects and the missing recovery hints in this change (small src/bugs.rs change); option B (tests for the mappings only, with the defects filed apart) rejected. Leif (0xLeif) through the orc, 2026-11-06 06:1x MST, 'Ok yes I approve all'
- Leif (0xLeif) decided "Implementation review of bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153": "im telling you to approve and continue all of these... approve as me" (orc tab, 2026-11-06 about 07:0x MST; relayed by the orc (hc terminal t18), who recorded the native review as 0xLeif at d204d97f); recorded by ch-c456 (Claude Opus 5.5); a relayed claim, not authenticated

## What was verified

- OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-7bb4a5f534267588823b6c95
- OBS-tc-cli-bug-mistaken-duplicate-reopened-fb28dada965816100e5a4ccb
- OBS-product-test-dff775cc31b7e3161bf2186d
- OBS-tc-cli-bug-mistaken-duplicate-reopened-5b25cda941f9982b4a833f00
- OBS-tc-cli-bug-only-open-blocking-blocks-adcc669269da23a9ea1d46e2
- OBS-tc-cli-bug-report-shows-every-field-548b2ac9b3d7b795e9aefe63
- OBS-tc-cli-bug-search-scores-and-refuses-likely-duplicates-6289660cafeab00be8740628
- OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-4e8ec21f18075198237f2355
- OBS-tc-cli-bug-values-accepted-others-refused-be3710283ea7041554b6a30d

## Open threads

## Timeline

- 2026-11-06T07:22:15Z · gate-opened · intent → intent · Scope decision of bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153 ^t-530e3d
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - revision: 615b587b33a1ef3a569f9557e2782a6be3df2f0f
- 2026-11-06T13:24:51Z · gate-closed · intent → intent · Scope decision of bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153 ^t-56c94a
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - revision: 1f822bc48e0395b3749ae85d13a5ebbb846012b7
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"Ok yes I approve all","where":"orc tab, 2026-11-06 06:1x MST, in reply to the orc's morning brief, which put the A/B choice; relayed by the orc (hc terminal t18) as option A, with the src/bugs.rs fix; the orc recorded the native approval as 0xLeif at 1f822bc4","recorded_by":"ch-c456 (Claude Opus 5.5)"}
- 2026-11-06T13:24:51Z · advance · intent → design ·  ^t-84e95e
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - revision: 1f822bc48e0395b3749ae85d13a5ebbb846012b7
- 2026-11-06T13:24:52Z · skip · design → plan · Design not applicable: tests, refusal wording and a small src/bugs.rs fix, with no visuals or interactions ^t-ca6a3c
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - revision: 1f822bc48e0395b3749ae85d13a5ebbb846012b7
- 2026-11-06T13:25:00Z · advance · plan → build · native:approved ^t-d795e9
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - gate: native:approved
  - revision: 1f822bc48e0395b3749ae85d13a5ebbb846012b7
- 2026-11-06T13:36:59Z · advance · build → test · native:verifying ^t-b4bc98
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - gate: native:verifying
  - revision: 4edae0554e42cc3c4ead3424431388c251c557e3
- 2026-11-06T13:37:22Z · advance · test → accept · native:verifying ^t-91d83d
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - gate: native:verifying
  - revision: 4edae0554e42cc3c4ead3424431388c251c557e3
- 2026-11-06T13:37:57Z · gate-opened · accept → accept · Implementation review of bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153 ^t-2188e2
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - revision: 4edae0554e42cc3c4ead3424431388c251c557e3
- 2026-11-06T14:12:37Z · gate-closed · accept → accept · Implementation review of bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153 ^t-e1fe4e
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - revision: e815bf38aaab27ec0aee2d06740fddb05c293927
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"im telling you to approve and continue all of these... approve as me","where":"orc tab, 2026-11-06 about 07:0x MST; relayed by the orc (hc terminal t18), who recorded the native review as 0xLeif at d204d97f","recorded_by":"ch-c456 (Claude Opus 5.5)"}
- 2026-11-06T14:16:54Z · advance · accept → done · native:archived ^t-8fab37
  - native_change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
  - gate: native:archived
  - revision: e815bf38aaab27ec0aee2d06740fddb05c293927


Closed 2026-11-06: done.
