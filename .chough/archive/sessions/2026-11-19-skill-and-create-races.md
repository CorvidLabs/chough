---
kind: session
id: 2026-11-19-skill-and-create-races
aliases: [skill-and-create-races]
objective: "Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408)"
stage: done
stage_status: archived
iteration: 1
native_change: skill-and-create-races
pr: "420"
criteria: [LIFE-4, LIFE-12, DRIVE-17]
requirements: [REQ-cli-016, REQ-cli-001, REQ-history-016]
observations:
  - 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state
  - 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated
  - 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly
  - 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected
  - 2026-11-20-tc-cli-parallel-skill-updates-each-succeed
  - 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly
  - 2026-11-20-tc-history-doctor-reads-uncommitted-records
  - 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it
  - 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once
  - 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-a47f17
  - 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-18ecd1
  - 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-400c1d
  - 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-83476b
  - 2026-11-20-tc-cli-parallel-skill-updates-each-succeed-3ce8d4
  - 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-aec866
  - 2026-11-20-tc-history-doctor-reads-uncommitted-records-4895a8
  - 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-37de89
  - 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-baf142
  - 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-c3d8a2
  - 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-f546f9
  - 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-3f8156
  - 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-59cb89
  - 2026-11-20-tc-cli-parallel-skill-updates-each-succeed-1e9c8d
  - 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-94ae64
  - 2026-11-20-tc-history-doctor-reads-uncommitted-records-2c7a6f
  - 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-c1514f
  - 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-2fd9c3
  - 2026-11-20-tc-cli-release-name-byte-matching
  - 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-88f07d
  - 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-16ac1e
  - 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-1145ae
  - 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-b7793f
  - 2026-11-20-tc-cli-parallel-skill-updates-each-succeed-54b621
  - 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-a11c2d
  - 2026-11-20-tc-history-doctor-reads-uncommitted-records-375f68
  - 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-65a905
  - 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-2126cf
  - 2026-11-20-tc-cli-release-name-byte-matching-6874f7
closed: 2026-11-20
outcome: done
---
# Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408)

## Goal

Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408)

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of skill-and-create-races": "Approve" (Claude Code chat answer, relayed by his planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-19 about 14:12Z, on D2 skill-and-create-races at 2bf6318 with the four recommended choices; recorded at the batch start on that text, the draft carried onto main 056cbb80 by a merge with no delta word changed); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve skill-and-create-races as 0xLeif
- 0xLeif decided "Add tests/skill_releases.rs to skill-and-create-races's scope by one send-back, run by the fixer": "Yes (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough D2: ... May D2's scope add tests/skill_releases.rs so the test can follow your rule? No wording changes.'; add tests/skill_releases.rs to D2's scope by one send-back the fixer runs, no sentence change); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of skill-and-create-races with tests/skill_releases.rs added": "Yes (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough D2: ... May D2's scope add tests/skill_releases.rs so the test can follow your rule? No wording changes.'; on this text: tests/skill_releases.rs joins the declared paths (11 to 12) with no sentence change, the deltas byte-identical to those approved at f37fc73); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve skill-and-create-races as 0xLeif
- 0xLeif decided "Implementation review of skill-and-create-races": "Go" (Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #420 at 3b5890ae: the testers' PASS (the tester on macOS at 802537b6 with the 22 real repositories identical to main, and at 0dde9150, 90 of 90 rounds; tester2 on Linux at 0dde9150, 0 of 200 runs failing) and the tester's harness rerun by the fixer at 3b5890ae (38 of 38: 11 D2 rows fixed against main 7bf110dc, the 22 real repositories unchanged); corvid-agent APPROVED 3b5890ae (2026-11-20 about 15:10Z); fable-lead's read of the source at 802537b6, its finding D2-1 fixed. Every sentence change in this batch carries Leif's own words: the scope approval, and his 'Yes (Recommended)' for tests/skill_releases.rs, which changed no sentence. fable-lead is paused until about 2026-11-23, so t4c is told after.); recorded by chough-fixer-t4n; a relayed claim, not authenticated; ran specsync change review skill-and-create-races as 0xLeif with verdict pass

## What was verified

- 28 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1). - 54d33ada0904; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-88f07d
- Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1). - 58331d588ac8; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-16ac1e
- Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1). - e728d7adc842; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-1145ae
- Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1). - 96cb6beeed59; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-b7793f
- Ran cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed for TC-cli-parallel-skill-updates-each-succeed: passed (exit 0, tests run 1). - 13380f161c27; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-parallel-skill-updates-each-succeed-54b621
- Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1). - a97898fe82e5; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-a11c2d
- Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1). - feb14ec33729; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-doctor-reads-uncommitted-records-375f68
- Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1). - 1675df3da581; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-65a905
- Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1). - e08f27364046; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-2126cf
- Ran cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes for TC-cli-release-name-byte-matching: passed (exit 0, tests run 1). - 091f020a3dca; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-release-name-byte-matching-6874f7

## Bugs

None.

## Open threads

## Timeline

- 2026-11-19T07:43:53Z · start · intent → intent · Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408) ^t-2aa121
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T07:43:53Z · advance · intent → design ·  ^t-3ffc10
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T07:43:53Z · advance · design → plan ·  ^t-d15454
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T09:09:15Z · gate-opened · plan → plan · Approve the scope of skill-and-create-races ^t-7d30e5
  - native_change: skill-and-create-races
  - revision: ff3c80cae8022b05bb8ac41b13e53d4feab91a2c
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T09:09:15Z · gate-closed · plan → plan · Approve the scope of skill-and-create-races ^t-afa4bb
  - native_change: skill-and-create-races
  - revision: ff3c80cae8022b05bb8ac41b13e53d4feab91a2c
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve
    - where: Claude Code chat answer, relayed by his planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-19 about 14:12Z, on D2 skill-and-create-races at 2bf6318 with the four recommended choices; recorded at the batch start on that text, the draft carried onto main 056cbb80 by a merge with no delta word changed
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve skill-and-create-races
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T09:10:10Z · advance · plan → build · native:approved ^t-bca690
  - native_change: skill-and-create-races
  - gate: native:approved
  - revision: f37fc73fb8f76e7c872a50ce6cdda4f69b3a1aa8
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:22:37Z · gate-opened · build → build · Add tests/skill_releases.rs to skill-and-create-races's scope by one send-back, run by the fixer ^t-9f01db
  - native_change: skill-and-create-races
  - revision: 02b0b254c7ad5eb80758d42067091208dace8c36
  - holds: build
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:22:37Z · gate-closed · build → build · Add tests/skill_releases.rs to skill-and-create-races's scope by one send-back, run by the fixer ^t-7a3c1c
  - native_change: skill-and-create-races
  - revision: 02b0b254c7ad5eb80758d42067091208dace8c36
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes (Recommended)
    - where: Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough D2: ... May D2's scope add tests/skill_releases.rs so the test can follow your rule? No wording changes.'; add tests/skill_releases.rs to D2's scope by one send-back the fixer runs, no sentence change
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:23:06Z · gate-opened · build → build · Re-approval of skill-and-create-races with tests/skill_releases.rs added ^t-2c9e96
  - native_change: skill-and-create-races
  - revision: 02b0b254c7ad5eb80758d42067091208dace8c36
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:23:06Z · direct-native-write · build → build · skill-and-create-races's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-d1c273
  - native_change: skill-and-create-races
  - revision: 02b0b254c7ad5eb80758d42067091208dace8c36
  - direct_write:
    - change: skill-and-create-races
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:23:06Z · gate-closed · build → build · Re-approval of skill-and-create-races with tests/skill_releases.rs added ^t-f1eb3a
  - native_change: skill-and-create-races
  - revision: 02b0b254c7ad5eb80758d42067091208dace8c36
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes (Recommended)
    - where: Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 14:13Z, relayed by hc t4c through fable-lead (hc t53); the question as shown: 'Chough D2: ... May D2's scope add tests/skill_releases.rs so the test can follow your rule? No wording changes.'; on this text: tests/skill_releases.rs joins the declared paths (11 to 12) with no sentence change, the deltas byte-identical to those approved at f37fc73
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve skill-and-create-races
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:30:02Z · advance · build → test · native:verifying ^t-f3161b
  - native_change: skill-and-create-races
  - gate: native:verifying
  - revision: 3b5890ae3edd848815a9271832beaaa2d846512a
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:30:21Z · advance · test → accept · native:verifying ^t-ec0f5e
  - native_change: skill-and-create-races
  - gate: native:verifying
  - revision: 3b5890ae3edd848815a9271832beaaa2d846512a
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:30:49Z · gate-opened · accept → accept · Implementation review of skill-and-create-races ^t-3a7fdd
  - native_change: skill-and-create-races
  - revision: 3b5890ae3edd848815a9271832beaaa2d846512a
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:33:27Z · gate-closed · accept → accept · Implementation review of skill-and-create-races ^t-3d4ebb
  - native_change: skill-and-create-races
  - revision: 7a45e876c456ad4b4171c952c97c0f894710979a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #420 at 3b5890ae: the testers' PASS (the tester on macOS at 802537b6 with the 22 real repositories identical to main, and at 0dde9150, 90 of 90 rounds; tester2 on Linux at 0dde9150, 0 of 200 runs failing) and the tester's harness rerun by the fixer at 3b5890ae (38 of 38: 11 D2 rows fixed against main 7bf110dc, the 22 real repositories unchanged); corvid-agent APPROVED 3b5890ae (2026-11-20 about 15:10Z); fable-lead's read of the source at 802537b6, its finding D2-1 fixed. Every sentence change in this batch carries Leif's own words: the scope approval, and his 'Yes (Recommended)' for tests/skill_releases.rs, which changed no sentence. fable-lead is paused until about 2026-11-23, so t4c is told after.
    - recorded_by: chough-fixer-t4n
    - native: specsync change review skill-and-create-races
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:33:30Z · advance · accept → done · native:archived ^t-1d9419
  - native_change: skill-and-create-races
  - gate: native:archived
  - revision: 7a45e876c456ad4b4171c952c97c0f894710979a
  - by: chough-fixer (claude-opus-5-5, hc t4n)

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state
    claim: "Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:a231ed7cf6ae285943dac65cc32d1207703001c8336f18896de95be9b97d6136
    case: TC-cli-a-crlf-skill-copy-reads-as-its-lf-state
    case_digest: 84db714666013ed423ef083c32b53a0f123a47d64ec214ec4baacc0c69799225
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:32Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated
    claim: "Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:58331d588ac8a9111bff475363ca19a1abda4af616f49a71360d54b0603e1643
    case: TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated
    case_digest: f5dafab564def17561e196a24027c02c604deab6484670dc79ef812d6efc49a5
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:34Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly
    claim: "Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:619c24eb0d0eb517c08a36414db933f505d02a6726c4782c2716d26b0f3ede56
    case: TC-cli-a-skill-file-it-cannot-write-is-refused-plainly
    case_digest: f4ac5530827e43d00ea38628952c7a73e90c99a30c9c9ecde6ca2d77b62a7b39
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:36Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected
    claim: "Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:5376fdd1603c85631016e0046a4862ebe8c073f19de844338806000b6bb5e262
    case: TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected
    case_digest: 086bcec2745fd2bf5bacaac4374db24720b2860179bb63f9fb10168a7c418e49
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:37Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-parallel-skill-updates-each-succeed
    claim: "Ran cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed for TC-cli-parallel-skill-updates-each-succeed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:81c023d3ad7c5846dcd54c1c7ddd2b4f29e969a26f0cf6f094148b3437894aa8
    case: TC-cli-parallel-skill-updates-each-succeed
    case_digest: 3cc2d4766c37270036460ba50fde9a16f8675611607656fa0395837dfa97e7b5
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:40Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly
    claim: "Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:00f52a16a89f9e48103a158b0b3416d8793e44a87d0dbd0b2756db1d15aaac32
    case: TC-cli-parallel-inits-each-succeed-or-refuse-plainly
    case_digest: 59a8124b5e60791583eb75d311612c77d169ec54345a474522cd6a70b531d0e3
    criteria: LIFE-4, LIFE-6
    requirements: REQ-cli-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:43Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-reads-uncommitted-records
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:b01bdad875b83ebecab9403b3a6966cb98aa7151e7e65aa618858029aa4d830f
    case: TC-history-doctor-reads-uncommitted-records
    case_digest: ffa17dc175342e495e3f67aee544085353a8ffc9b06e24f88e5838113f777372
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:46Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:078ec0ae12ac48b8b317f9c9d114cddac019ab71e83494f0d32a7bab118c5c9d
    case: TC-history-doctor-names-a-special-file-without-opening-it
    case_digest: bae764610596635f70c93a50d78adfde336d28e78d96e756b798d93373b58e92
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:48Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once
    revision: 63057710d3223425b0db8d988717f2c657e5035b
    environment: local-macos-arm64
    content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
    fingerprint: sha256:161d34f7d7b676b52736142598875000896c715df1e223ad55569f25fecb4ffc
    case: TC-history-doctor-skips-ignored-and-reports-once
    case_digest: bca77d93afd1d748cd865bf4ee61a60f234c7d0465b9b3bbd91da7f9a523f336
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T09:20:50Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-a47f17
    claim: "Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:89538eeb371bffdc239c27e385d72e4798602692c5d496b45c916aa66b61aa87
    case: TC-cli-a-crlf-skill-copy-reads-as-its-lf-state
    case_digest: 84db714666013ed423ef083c32b53a0f123a47d64ec214ec4baacc0c69799225
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:21Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-18ecd1
    claim: "Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:a181a5cd38de31072bf5ddb78dd5963992509380aee429c5edfe3f0a930612d5
    case: TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated
    case_digest: f5dafab564def17561e196a24027c02c604deab6484670dc79ef812d6efc49a5
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:23Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-400c1d
    claim: "Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:c47cc9e805ec8447d87b6c85b6018ab70b0b9cf025f5189d26d1164a5f7d8f39
    case: TC-cli-a-skill-file-it-cannot-write-is-refused-plainly
    case_digest: f4ac5530827e43d00ea38628952c7a73e90c99a30c9c9ecde6ca2d77b62a7b39
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:25Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-83476b
    claim: "Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:b5ae604f6583229f6f28593e6b9a228502d4b99b21e95d9f7d7c970daf161288
    case: TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected
    case_digest: 086bcec2745fd2bf5bacaac4374db24720b2860179bb63f9fb10168a7c418e49
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:27Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-parallel-skill-updates-each-succeed-3ce8d4
    claim: "Ran cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed for TC-cli-parallel-skill-updates-each-succeed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:e3f271f11cbf0f70242c7784756c18ab43f55833f0a8180f6c7542bb7cc62db3
    case: TC-cli-parallel-skill-updates-each-succeed
    case_digest: 3cc2d4766c37270036460ba50fde9a16f8675611607656fa0395837dfa97e7b5
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:30Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-aec866
    claim: "Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:07ad4746ad5276845264e943270df8ed383a18bc87f27d287d875c41ff8dbc8d
    case: TC-cli-parallel-inits-each-succeed-or-refuse-plainly
    case_digest: 59a8124b5e60791583eb75d311612c77d169ec54345a474522cd6a70b531d0e3
    criteria: LIFE-4, LIFE-6
    requirements: REQ-cli-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:40Z
    duration_seconds: 8
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-reads-uncommitted-records-4895a8
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:22701bfdeb5707836304ece0c35d4842b48bb6a43330e975d7e8edafa7f0e4dc
    case: TC-history-doctor-reads-uncommitted-records
    case_digest: ffa17dc175342e495e3f67aee544085353a8ffc9b06e24f88e5838113f777372
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:42Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-37de89
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:617698c61a8f67d8188d259ef00f113854411c404094c1a4eaddfb5eddea9be9
    case: TC-history-doctor-names-a-special-file-without-opening-it
    case_digest: bae764610596635f70c93a50d78adfde336d28e78d96e756b798d93373b58e92
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:45Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-baf142
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once
    revision: 29fb39131c6435612182f52929b94e69ab0dcba3
    environment: local-macos-arm64
    content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
    fingerprint: sha256:f41dc12f1d89ff99e3027ab59524cf3a0421c01637111104c6c728a7e623273a
    case: TC-history-doctor-skips-ignored-and-reports-once
    case_digest: bca77d93afd1d748cd865bf4ee61a60f234c7d0465b9b3bbd91da7f9a523f336
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:19:47Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-c3d8a2
    claim: "Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:d54aa0aa0bf3293354e9cfffb83b80b7bc362ed54096350b2a17f5b864438a5f
    case: TC-cli-a-crlf-skill-copy-reads-as-its-lf-state
    case_digest: 84db714666013ed423ef083c32b53a0f123a47d64ec214ec4baacc0c69799225
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:35:42Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-f546f9
    claim: "Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:deefcac33bdb9c7d1814f287df7a51fcb8e67716e8f05e089c8369086b250146
    case: TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated
    case_digest: f5dafab564def17561e196a24027c02c604deab6484670dc79ef812d6efc49a5
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:35:44Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-3f8156
    claim: "Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:619c24eb0d0eb517c08a36414db933f505d02a6726c4782c2716d26b0f3ede56
    case: TC-cli-a-skill-file-it-cannot-write-is-refused-plainly
    case_digest: f4ac5530827e43d00ea38628952c7a73e90c99a30c9c9ecde6ca2d77b62a7b39
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:35:45Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-59cb89
    claim: "Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:331ad919a2aeb157c5c2785b24967fdd2c693939d16898a66d410439274b9082
    case: TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected
    case_digest: 086bcec2745fd2bf5bacaac4374db24720b2860179bb63f9fb10168a7c418e49
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:35:47Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-parallel-skill-updates-each-succeed-1e9c8d
    claim: "Ran cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed for TC-cli-parallel-skill-updates-each-succeed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:f904b0ac6b49adbf2681bca9f051f80e004cae26145cc6b2b83135d72b6bf3b7
    case: TC-cli-parallel-skill-updates-each-succeed
    case_digest: 3cc2d4766c37270036460ba50fde9a16f8675611607656fa0395837dfa97e7b5
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:35:50Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-94ae64
    claim: "Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:1c1b80b7c9df97a5de30f679cb6bb0a3772d7df9d4115cc7790e936cef3313e6
    case: TC-cli-parallel-inits-each-succeed-or-refuse-plainly
    case_digest: 59a8124b5e60791583eb75d311612c77d169ec54345a474522cd6a70b531d0e3
    criteria: LIFE-4, LIFE-6
    requirements: REQ-cli-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:35:59Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-reads-uncommitted-records-2c7a6f
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:5a462589e6107c144a15a18e949a33497585229a283f8beba18d4be7efd10879
    case: TC-history-doctor-reads-uncommitted-records
    case_digest: ffa17dc175342e495e3f67aee544085353a8ffc9b06e24f88e5838113f777372
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:36:01Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-c1514f
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:1675df3da581591f5def468ace43ab298102b9a4b4bf7cd2ad542bcd135f05e4
    case: TC-history-doctor-names-a-special-file-without-opening-it
    case_digest: bae764610596635f70c93a50d78adfde336d28e78d96e756b798d93373b58e92
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:36:03Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-2fd9c3
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:161d34f7d7b676b52736142598875000896c715df1e223ad55569f25fecb4ffc
    case: TC-history-doctor-skips-ignored-and-reports-once
    case_digest: bca77d93afd1d748cd865bf4ee61a60f234c7d0465b9b3bbd91da7f9a523f336
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:36:05Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-release-name-byte-matching
    claim: "Ran cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes for TC-cli-release-name-byte-matching: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes
    revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
    environment: local-macos-arm64
    content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
    fingerprint: sha256:24811f306aeaa7cd1a94e2bd3c4b91204653feb87e2e50611ad4e0cc36f58aef
    case: TC-cli-release-name-byte-matching
    case_digest: a5acedd9a4928ad9ee4b772ed63a878118e614703a574b6996c4d19960f89387
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:36:07Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-88f07d
    claim: "Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:54d33ada09042ffcc22b4a40234abbc397da88eb62ca6f9a991f7250be4c5943
    case: TC-cli-a-crlf-skill-copy-reads-as-its-lf-state
    case_digest: 84db714666013ed423ef083c32b53a0f123a47d64ec214ec4baacc0c69799225
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:59:47Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated-16ac1e
    claim: "Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:58331d588ac8a9111bff475363ca19a1abda4af616f49a71360d54b0603e1643
    case: TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated
    case_digest: f5dafab564def17561e196a24027c02c604deab6484670dc79ef812d6efc49a5
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:59:49Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly-1145ae
    claim: "Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:e728d7adc8421585b173e2966aa6b00ed0530db3d90afc0dbb7aa27333a95b2d
    case: TC-cli-a-skill-file-it-cannot-write-is-refused-plainly
    case_digest: f4ac5530827e43d00ea38628952c7a73e90c99a30c9c9ecde6ca2d77b62a7b39
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:59:53Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-b7793f
    claim: "Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:96cb6beeed59130fc6241076a18542677b436445661a4c0259154c631d64a89c
    case: TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected
    case_digest: 086bcec2745fd2bf5bacaac4374db24720b2860179bb63f9fb10168a7c418e49
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:59:55Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-parallel-skill-updates-each-succeed-54b621
    claim: "Ran cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed for TC-cli-parallel-skill-updates-each-succeed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:13380f161c27c88082116c335321c39c623d12b8600733244ba2e664e23d737a
    case: TC-cli-parallel-skill-updates-each-succeed
    case_digest: 3cc2d4766c37270036460ba50fde9a16f8675611607656fa0395837dfa97e7b5
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T14:59:58Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-a11c2d
    claim: "Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:a97898fe82e526e44251a06fd5e0b4be0b9f1f5f2cd92750dbfa7f70497bfda3
    case: TC-cli-parallel-inits-each-succeed-or-refuse-plainly
    case_digest: 59a8124b5e60791583eb75d311612c77d169ec54345a474522cd6a70b531d0e3
    criteria: LIFE-4, LIFE-6
    requirements: REQ-cli-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:00:08Z
    duration_seconds: 8
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-reads-uncommitted-records-375f68
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:feb14ec337291f3cc6006037887654700376b3cc660894acba4a3fd88f18a07f
    case: TC-history-doctor-reads-uncommitted-records
    case_digest: ffa17dc175342e495e3f67aee544085353a8ffc9b06e24f88e5838113f777372
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:00:10Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it-65a905
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:1675df3da581591f5def468ace43ab298102b9a4b4bf7cd2ad542bcd135f05e4
    case: TC-history-doctor-names-a-special-file-without-opening-it
    case_digest: bae764610596635f70c93a50d78adfde336d28e78d96e756b798d93373b58e92
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:00:12Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-2126cf
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:e08f27364046e845374ac3ff8d74377f419b86e87105df256e09ff1d95530d55
    case: TC-history-doctor-skips-ignored-and-reports-once
    case_digest: bca77d93afd1d748cd865bf4ee61a60f234c7d0465b9b3bbd91da7f9a523f336
    criteria: DRIVE-17
    requirements: REQ-history-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:00:14Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-release-name-byte-matching-6874f7
    claim: "Ran cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes for TC-cli-release-name-byte-matching: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test skill_releases -- --exact recognition_requires_the_filename_and_exact_unnormalized_bytes
    revision: f38abb556cee50e587a4c2446a91c7f2fa140997
    environment: local-macos-arm64
    content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
    fingerprint: sha256:091f020a3dca7e2c91f3c3324b25cdca30987e29611fe206ead07fa2c1a86b79
    case: TC-cli-release-name-byte-matching
    case_digest: a5acedd9a4928ad9ee4b772ed63a878118e614703a574b6996c4d19960f89387
    criteria: FLOW-1, LIFE-4
    requirements: REQ-cli-016
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T15:00:16Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
```
