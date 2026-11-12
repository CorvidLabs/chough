---
kind: session
id: 2026-11-06-assert-the-eight-reader-ceiling-for-archive-validity-reads-on
aliases: [assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11]
objective: Assert REQ-cli-033's eight-reader ceiling for archive validity reads on any host (audit CH-M11, plan C5)
intent_scope: KNOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
pr: ""
criteria: [KNOW-10]
requirements: [REQ-cli-033]
cases: [TC-cli-validity-readers-on-any-host, TC-cli-at-most-eight-validity-reads]
design: []
files: []
observations:
  - OBS-tc-cli-validity-readers-on-any-host-1fccb1a5923652ade8e26238
  - OBS-tc-cli-at-most-eight-validity-reads-5b7afc441c01deea7818e53b
  - OBS-product-test-56d46b6484dae509ee8e848c
bugs:
  - BUG-an-empty-change-directory-left-by-a-branch-switch-counts-as-an-open-nat-505311f29b37d46cccc197e6
blockers: []
waiting_on_human: []
run_consents: []
next: Push the archive, observe hosted CI at the tip, and merge on green plus corvid-agent approval of the exact head, pinned with --match-head-commit
closed: 2026-11-06
outcome: done
receipts:
  - id: 2026-11-06-tc-cli-validity-readers-on-any-host
    aliases: OBS-tc-cli-validity-readers-on-any-host-1fccb1a5923652ade8e26238
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::at_most_eight_validity_readers_on_any_host for TC-cli-validity-readers-on-any-host: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact evidence::tests::at_most_eight_validity_readers_on_any_host
    revision: 3a87c7c9ffade09ac48da294b54257c27719e245
    environment: local-macos-arm64
    content: 17c3002ba5085b9bfec8eb07f833866a2c39b24f813e8818554d051deb46f210
    fingerprint: sha256:d0039ff83a236e2781f5cc5687587f832663b4cf477ebfe031918b83eecbb15d
    case: TC-cli-validity-readers-on-any-host
    case_digest: dc7366dff35fb72879adcb5374d44206e715e25abf793cf99a7681c61ba85e1e
    observed: 2026-11-06T04:50:40Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-at-most-eight-validity-reads
    aliases: OBS-tc-cli-at-most-eight-validity-reads-5b7afc441c01deea7818e53b
    claim: "Ran cargo test --locked --test reduce -- --exact at_most_eight_validity_reads_run_at_once for TC-cli-at-most-eight-validity-reads: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact at_most_eight_validity_reads_run_at_once
    revision: 3a87c7c9ffade09ac48da294b54257c27719e245
    environment: local-macos-arm64
    content: 17c3002ba5085b9bfec8eb07f833866a2c39b24f813e8818554d051deb46f210
    fingerprint: sha256:a85d07c314f12b71e24d9ab966e5f77b0ae2196d1f27bbc1def4485565777ccf
    case: TC-cli-at-most-eight-validity-reads
    case_digest: 8e9254fac3c4364ed53c413ca3556ecd9bfcd04d00cff543136cb78e88808142
    observed: 2026-11-06T04:50:51Z
    duration_seconds: 8
  - id: OBS-product-test-56d46b6484dae509ee8e848c
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
---
# Assert REQ-cli-033's eight-reader ceiling for archive validity reads on any host (audit CH-M11, plan C5)

## Goal

Assert REQ-cli-033's eight-reader ceiling for archive validity reads on any host (audit CH-M11, plan C5)

## Where it stands

- Change package drafted; at_most_eight_validity_reads_run_at_once fails against the CH-M11 mutant (.clamp(2, 128): 12 reads at once on this 20-core <redacted:host-name>) and passes restored; native_validity_reads_run_in_parallel passes both ways
- validity_readers with its unit test; REQ-cli-033 criterion materialized (cli version 62 after merging main); both cases pass on macOS; mutation check recorded as OBS-product-test-56d46b6484dae509ee8e848c

## Decisions and why

- Agent's reading, not a human decision: an end-to-end test alone cannot kill the mutant on hosts reporting 8 or fewer cores, so the plan adds a pure validity_readers function with a unit test; the ceiling stays 8 and the floor 2.
- Leif (0xLeif), 2026-11-05 21:23 MST, through the orc (t18): approves C5 (his 21:22 'Hold' was a misclick, the orc said); the orc recorded the native approval on 87d4e76e. GitHub issue #137 filed at his request.
- decided: stage moves resumed after chough#141's fix (#142, 90e5de6f) merged into this branch; build -> test -> accept. Commit a7bfba06's message said these moves and the review gate happened; the KNOW-29 regression refused them then, and they happen here.
- failed: fd0345df's knowledge index listed the C6 change as active, from an empty .specsync/changes/<C6>/deltas/ directory the branch switch left behind; the Linux knowledge step rejected it. Removed the empty directory (no files in it), rebuilt the index, recorded BUG-an-empty-change-directory-left-by-a-branch-switch-counts-as-an-open-nat-505311f29b37d46cccc197e6 (out of scope)
- PR #146 (C5, #137) review: the orc (t18) decided for Leif under his authorization (2026-11-05 23:3x MST: decide as him overnight where 95% confident) and recorded the native review (pass) as 0xLeif at 69a9cec0; Leif did not review it himself.
- Leif (0xLeif), 2026-11-05 22:43 MST, through the orc (standing rule): when a PR he approved only gets origin/main merged in (bookkeeping, changelog or version conflicts, no change to reviewed content) and checks re-run green, merge at the new head without asking again and say so; any change to reviewed content comes back for review.
- Leif (0xLeif), 2026-11-05 23:24 MST, through the orc: agent commits keep the worktree's git identity (0xLeif) as author; existing commits are not rewritten; native SpecSync approvals and reviews in his name come only through the orc.

## What was verified

- OBS-tc-cli-validity-readers-on-any-host-1fccb1a5923652ade8e26238
- OBS-tc-cli-at-most-eight-validity-reads-5b7afc441c01deea7818e53b
- OBS-product-test-56d46b6484dae509ee8e848c

## Open threads

## Timeline

- 2026-11-06T04:07:23Z · gate-opened · intent → intent · Scope decision of assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11 ^t-b14fa0
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - revision: 73a7da5653312e907621c0165d0b5f903c22ff53
- 2026-11-06T04:44:35Z · gate-closed · intent → intent · Scope decision of assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11 ^t-e4d134
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - revision: 87d4e76e02f46d3a6f1e35b94395fe26fdd18ba5
- 2026-11-06T04:44:35Z · advance · intent → design ·  ^t-0f2a75
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - revision: 87d4e76e02f46d3a6f1e35b94395fe26fdd18ba5
- 2026-11-06T04:44:35Z · skip · design → plan · Design not applicable: a test and a no-behavior-change refactor with no visuals or interactions ^t-b7b908
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - revision: 87d4e76e02f46d3a6f1e35b94395fe26fdd18ba5
- 2026-11-06T04:44:38Z · advance · plan → build · native:approved ^t-5d8273
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - gate: native:approved
  - revision: 87d4e76e02f46d3a6f1e35b94395fe26fdd18ba5
- 2026-11-06T05:27:54Z · advance · build → test · native:verifying ^t-09a2ba
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - gate: native:verifying
  - revision: 5b4555c75c9fa946cf3fbcfc591fc76e2f5976dd
- 2026-11-06T05:28:08Z · advance · test → accept · native:verifying ^t-12f0c3
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - gate: native:verifying
  - revision: 5b4555c75c9fa946cf3fbcfc591fc76e2f5976dd
- 2026-11-06T05:28:09Z · gate-opened · accept → accept · Implementation review of assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11 ^t-151c9a
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - revision: 5b4555c75c9fa946cf3fbcfc591fc76e2f5976dd
- 2026-11-06T06:36:55Z · gate-closed · accept → accept · Implementation review of assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11 ^t-c4503e
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - revision: 5701b277e040a6c29433300e86aa54b037f3f0cf
- 2026-11-06T06:38:18Z · advance · accept → done · native:archived ^t-675cb3
  - native_change: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
  - gate: native:archived
  - revision: 5701b277e040a6c29433300e86aa54b037f3f0cf


Closed 2026-11-06: done.
