---
kind: bug
id: 2026-11-06-a-mistaken-duplicate-cannot-be-reopened-a-bug-can-relate-to
aliases:
  - BUG-a-mistaken-duplicate-cannot-be-reopened-a-bug-can-relate-to-itself-and-2823f5504985675b64530948
title: A mistaken duplicate cannot be reopened, a bug can relate to itself, and link refusals name no recovery
status: verified
classification: product-defect
scope: in-scope
impact: a bug marked duplicate by mistake stays a duplicate for good; a bug's related list can name itself; an agent refused here is not told how to recover
criteria: [FLOW-8, LIFE-5]
requirements: []
cases:
  - TC-cli-bug-self-or-unknown-links-refused-with-recovery
  - TC-cli-bug-mistaken-duplicate-reopened
evidence:
  - OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-7bb4a5f534267588823b6c95
  - OBS-tc-cli-bug-mistaken-duplicate-reopened-fb28dada965816100e5a4ccb
  - OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-4e8ec21f18075198237f2355
  - OBS-tc-cli-bug-mistaken-duplicate-reopened-5b25cda941f9982b4a833f00
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
environment: local-macos-arm64
revision: fd52ed635432bb049b171c9f4dc03ffff39d79f9
found_by: "#153, found writing the audit Bugs row's tests"
related: [BUG-0019]
created: 2026-11-06T13:30:12Z
updated: 2026-11-06T13:34:19Z
---
# A mistaken duplicate cannot be reopened, a bug can relate to itself, and link refusals name no recovery

**What happened:** chough bug update <id> --status open on a duplicate bug fails with 'duplicate_of is only set for duplicate bugs'; --related <its own id> is accepted; the self-duplicate and unknown-bug refusals name no command to recover with
Actual: the reopen is refused, the self link is recorded, and the refusals say only what is wrong

**Steps:** mark a bug duplicate by mistake and try to reopen it; link a bug to itself with --related; name an unknown bug
1. chough bug update <id> --duplicate-of <other>, then chough bug update <id> --status open
2. chough bug update <id> --related <id>

**Expected:** reopening clears duplicate_of and notes it; a self-related link is refused; each refusal names chough bug list --all (and verified without a linked failure names chough bug update <id> --observation <id>)

**Why it matters:** a bug marked duplicate by mistake stays a duplicate for good; a bug's related list can name itself; an agent refused here is not told how to recover

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T13:30:12Z · Reported (#153, found writing the audit Bugs row's tests); evidence: OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-7bb4a5f534267588823b6c95, OBS-tc-cli-bug-mistaken-duplicate-reopened-fb28dada965816100e5a4ccb
- 2026-11-06T13:30:26Z · BUG-0019 (verified) made validation errors name their fix; these link refusals were not covered
- 2026-11-06T13:34:19Z · status: open -> verified. Fixed in 4edae055: both failing-first cases pass after the fix; evidence: OBS-tc-cli-bug-self-or-unknown-links-refused-with-recovery-4e8ec21f18075198237f2355, OBS-tc-cli-bug-mistaken-duplicate-reopened-5b25cda941f9982b4a833f00
