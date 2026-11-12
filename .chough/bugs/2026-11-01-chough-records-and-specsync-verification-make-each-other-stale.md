---
kind: bug
id: 2026-11-01-chough-records-and-specsync-verification-make-each-other-stale
aliases: [BUG-0016]
title: Chough records and SpecSync verification make each other stale
status: verified
classification: product-defect
scope: in-scope
impact: Repeated verification and review rebinding
criteria: [LIFE-6, LIFE-7]
requirements: [REQ-cli-004]
cases: []
evidence:
  - bug-0016-repro-1
  - bug-0016-fixed-1
  - tc-cli-0070-1793498638
  - tc-cli-0071-1793498642
  - tc-cli-0072-1793498646
  - OBS-tc-records-doctor-names-missing-exclusions-7348439622f474caf85af184
  - OBS-tc-records-doctor-names-missing-exclusions-4844a80f3a0c79319da89e3a
  - OBS-tc-records-untracked-receipt-keeps-verification-c662b9bae64eac4b34277ee5
  - OBS-tc-records-write-after-check-keeps-verification-915a49bf836ccd7d105306c6
  - OBS-tc-records-case-definitions-stay-verified-42c391cf9182f2f8ef22b328
receipts:
  - id: 2026-11-01-before-the-fix-a-specsync-bookkeeping-write-in-a-change
    aliases: bug-0016-repro-1
    claim: Before the fix, a SpecSync bookkeeping write in a change directory (state.json first) and moving a change into the archive make a new observation stale-revision-or-inputs
    result: failed
    revision: a386c132598916c6572a633da455b169b825d3e9
    environment: local-macos-arm64
    fingerprint: sha256:46a4e5b895879bc4971571ee5d0003e7d0045a1cbfabe6e9ec1c3d7ab4aa9604
    observed: 2026-11-01T01:31:33Z
  - id: 2026-11-01-tc-cli-0070
    aliases: bug-0016-fixed-1
    claim: After the fix, every SpecSync bookkeeping write, the change.md front-matter state change, their commit and the archive move keep a new observation current; an archived artifact edit or an unlisted file stales it
    result: passed
    revision: e806f783b34792c76961edfb24d42aa73764e6fa
    environment: local-macos-arm64
    content: 7eb66be2577f9e78895e0a93951cf202ab0bfe83611034719cfce6dfd4190e27
    fingerprint: sha256:ebdac59589f4db3932d75d054df88165b9bc3122f21f5aab2e9486e37ee68849
    case: TC-cli-0070
    case_digest: ee73ffc1f6e8e74e66ad008bf152fa4d3915eba2162bcda3225ca83008d1e449
    observed: 2026-11-01T01:40:32Z
  - id: 2026-11-01-tc-cli-0070-f1fbd0
    aliases: tc-cli-0070-1793498638
    claim: "Ran cargo test --locked --test cli -- --exact specsync_bookkeeping_writes_keep_an_observation_current for TC-cli-0070: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact specsync_bookkeeping_writes_keep_an_observation_current
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:076ca83cf28f9ef648da0133c0878b2d7ce031f0de5abe1b6af8238b4fdc2cdd
    case: TC-cli-0070
    case_digest: ee73ffc1f6e8e74e66ad008bf152fa4d3915eba2162bcda3225ca83008d1e449
    observed: 2026-11-01T02:04:01Z
  - id: 2026-11-01-tc-cli-0071
    aliases: tc-cli-0071-1793498642
    claim: "Ran cargo test --locked --test cli -- --exact edits_to_a_native_change_scope_stale_an_observation for TC-cli-0071: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact edits_to_a_native_change_scope_stale_an_observation
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:744ed23cf05d6a8c852b8795b48dfb44b90b2a27ab9e8c8972d66370693de18f
    case: TC-cli-0071
    case_digest: 4962cc13c0e60f9435898d5e0f9a0f0007f7117d63ed9999e1763b5c085eb99c
    observed: 2026-11-01T02:04:06Z
  - id: 2026-11-01-tc-cli-0072
    aliases: tc-cli-0072-1793498646
    claim: "Ran cargo test --locked --test cli -- --exact archiving_a_change_keeps_an_observation_current_until_an_artifact_changes for TC-cli-0072: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact archiving_a_change_keeps_an_observation_current_until_an_artifact_changes
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:daea9d32f088e930d0fd32ed792c8c77d4afa39811c04674922056b674c3bd0a
    case: TC-cli-0072
    case_digest: c2adbaa8ca6e791306a83469076acdcd4f307544303cec8738c3535ca47076af
    observed: 2026-11-01T02:04:09Z
  - id: 2026-11-11-tc-records-doctor-names-missing-exclusions
    aliases: OBS-tc-records-doctor-names-missing-exclusions-7348439622f474caf85af184
    claim: "Ran cargo test --locked --test doctor -- doctor_names_the_missing_verification_exclusions next_says_when_chough_writes_are_safe_and_init_gives_the_advice for TC-records-doctor-names-missing-exclusions: failed (exit 101)."
    result: failed
    command: cargo test --locked --test doctor -- doctor_names_the_missing_verification_exclusions next_says_when_chough_writes_are_safe_and_init_gives_the_advice
    revision: c5c3bc148768ae4f049e4d175002fff4cfdaf3ac
    environment: local-macos-arm64
    content: 85140163ef66d8112a6bf1e3f70b6c49701dacf728346a37c323b07f1516bd08
    fingerprint: sha256:e1418577d642ad719a597e9cbde4e5ff10ba418910d80838aff763f015ea70b7
    case: TC-records-doctor-names-missing-exclusions
    case_digest: 8da898555c33d8eb5419b201439307ba531b7cbbe86883bc9bc3c662dd2985b4
    observed: 2026-11-11T19:52:42Z
    duration_seconds: 0
  - id: 2026-11-11-tc-records-doctor-names-missing-exclusions-5a64e2
    aliases: OBS-tc-records-doctor-names-missing-exclusions-4844a80f3a0c79319da89e3a
    claim: "Ran cargo test --locked --test doctor -- doctor_names_the_missing_verification_exclusions next_says_when_chough_writes_are_safe_and_init_gives_the_advice for TC-records-doctor-names-missing-exclusions: passed (exit 0)."
    result: passed
    command: cargo test --locked --test doctor -- doctor_names_the_missing_verification_exclusions next_says_when_chough_writes_are_safe_and_init_gives_the_advice
    revision: 504cecba6393f6b629f83a36ec3847842055da83
    environment: local-macos-arm64
    content: 879f7097ade0f326c1c45b5faa86603a11ce283581e3856ab68bb56a08f1e93a
    fingerprint: sha256:8f8bc6857f9d448a4936c6c3b0fdd1e12b30d56b2e8940d6484f1367b56a6a15
    case: TC-records-doctor-names-missing-exclusions
    case_digest: 8da898555c33d8eb5419b201439307ba531b7cbbe86883bc9bc3c662dd2985b4
    observed: 2026-11-11T20:16:05Z
    duration_seconds: 2
  - id: 2026-11-11-tc-records-untracked-receipt-keeps-verification
    aliases: OBS-tc-records-untracked-receipt-keeps-verification-c662b9bae64eac4b34277ee5
    claim: "Ran cargo test --locked --test native -- --exact an_untracked_receipt_at_check_commit_matches_a_clean_clone for TC-records-untracked-receipt-keeps-verification: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact an_untracked_receipt_at_check_commit_matches_a_clean_clone
    revision: 504cecba6393f6b629f83a36ec3847842055da83
    environment: local-macos-arm64
    content: 879f7097ade0f326c1c45b5faa86603a11ce283581e3856ab68bb56a08f1e93a
    fingerprint: sha256:5df17580edff723dc6f2cfd9e3fadf3feebb0559452b21320e1f84ab32191371
    case: TC-records-untracked-receipt-keeps-verification
    case_digest: 1f991a07ad9efa6c826a8a4ce497660a8815b609f4b1a7a7e1d3b67479319963
    observed: 2026-11-11T20:14:53Z
    duration_seconds: 46
  - id: 2026-11-11-tc-records-write-after-check-keeps-verification
    aliases: OBS-tc-records-write-after-check-keeps-verification-915a49bf836ccd7d105306c6
    claim: "Ran cargo test --locked --test native -- --exact chough_writes_after_check_commit_leave_verification_current for TC-records-write-after-check-keeps-verification: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact chough_writes_after_check_commit_leave_verification_current
    revision: 504cecba6393f6b629f83a36ec3847842055da83
    environment: local-macos-arm64
    content: 879f7097ade0f326c1c45b5faa86603a11ce283581e3856ab68bb56a08f1e93a
    fingerprint: sha256:f7c43d54678effb66be985fdd867dd9276c3939dde97455df08d34cb247a039d
    case: TC-records-write-after-check-keeps-verification
    case_digest: 118692c895fabcf23398dcba0d2470de21b3042d2d508b4e46d2f88a1997561d
    observed: 2026-11-11T20:15:43Z
    duration_seconds: 48
  - id: 2026-11-11-tc-records-case-definitions-stay-verified
    aliases: OBS-tc-records-case-definitions-stay-verified-42c391cf9182f2f8ef22b328
    claim: "Ran cargo test --locked --test native -- --exact a_case_definition_change_still_stales_verification for TC-records-case-definitions-stay-verified: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact a_case_definition_change_still_stales_verification
    revision: 504cecba6393f6b629f83a36ec3847842055da83
    environment: local-macos-arm64
    content: 879f7097ade0f326c1c45b5faa86603a11ce283581e3856ab68bb56a08f1e93a
    fingerprint: sha256:d771ce64cf3c68410bba1c2e49ed6cd24cf7de869ebb67727ba174e320441ba2
    case: TC-records-case-definitions-stay-verified
    case_digest: 1d1d3bd516c24e712907225f7fcb4aa0163f662605eec5e4f7262f6cf3662741
    observed: 2026-11-11T20:16:02Z
    duration_seconds: 18
environment: local-macos-arm64
revision: 2756dc24a821bbb92021455c704ef7a177123ffa
found_by: CorvidLabs/chough#12 (Raven run); reproduced on main 2756dc2
related: []
created: 2026-11-01T00:57:53Z
updated: 2026-11-11T20:17:14Z
---
# Chough records and SpecSync verification make each other stale

**What happened:** Uncommitted .chough/ stales SpecSync verification; committing .chough/ forces another change check; lifecycle commits stale Chough observations
Actual: Three re-verifications and a stash were needed in the Raven run

**Steps:** Normal cases run, commit, change check, review, finalize sequence
1. chough cases run; specsync change review
2. commit .chough/; specsync change status

**Expected:** Each tool ignores the other's bookkeeping, and the documented order needs one verification

**Why it matters:** Repeated verification and review rebinding

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T00:57:53Z · Reported (CorvidLabs/chough#12 (Raven run); reproduced on main 2756dc2)
- 2026-11-01T00:57:53Z · Added despite similar reports: SpecSync bookkeeping and .chough interplay, beyond the revision salt in #10
- 2026-11-01T01:31:34Z · Reproduced before the fix by the new tests; see .chough/artifacts/bug-0016-reproduction.log.; evidence: bug-0016-repro-1
- 2026-11-01T01:40:33Z · status: open -> verified. Fixed in 4bf46da; the reproduction tests pass at e806f78 (.chough/artifacts/bug-0016-fixed-reproduction.log).; evidence: bug-0016-fixed-1
- 2026-11-01T02:05:06Z · The reproduction was a test-log observation recorded with evidence add before the fix, not a case-run observation: the regression case definitions TC-cli-0070 to TC-cli-0072 arrived with the fix commit 4bf46da, so they could not run at the unfixed tip. This note links their current passing case-run observations at the review-fix head.; evidence: tc-cli-0070-1793498638, tc-cli-0071-1793498642, tc-cli-0072-1793498646
- 2026-11-11T19:53:15Z · status: verified -> open. Reopened as GitHub #12 recurred: an untracked evidence receipt at check --commit staled a clean clone (spec-sync #1045, 2026-11-11), and checkpoints written after check --commit staled verification on #212. Red on 0.5.0: TC-records-doctor-names-missing-exclusions failed on leif/0.6.0-12-red at c5c3bc14 (doctor names no missing exclusion; next never says when writes are safe). Fixed in the 0.6.0 change (PR #238, REQ-records-014): the three record folders go in SpecSync's verification_ignored_paths, doctor and next say when they are missing, with real-SpecSync tests for both repros.; evidence: OBS-tc-records-doctor-names-missing-exclusions-7348439622f474caf85af184
- 2026-11-11T20:17:14Z · status: open -> verified. Fixed again in the 0.6.0 change (PR #238, REQ-records-014): the four #12 cases passed after the 0.5.0 failure, the two repros with real SpecSync 6.2.0 and their controls.; evidence: OBS-tc-records-doctor-names-missing-exclusions-4844a80f3a0c79319da89e3a, OBS-tc-records-untracked-receipt-keeps-verification-c662b9bae64eac4b34277ee5, OBS-tc-records-write-after-check-keeps-verification-915a49bf836ccd7d105306c6, OBS-tc-records-case-definitions-stay-verified-42c391cf9182f2f8ef22b328
