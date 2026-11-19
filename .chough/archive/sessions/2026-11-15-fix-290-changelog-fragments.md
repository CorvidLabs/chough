---
kind: session
id: 2026-11-15-fix-290-changelog-fragments
aliases: [fix-290-changelog-fragments]
objective: "Fix #290: each change adds a changelog fragment, assembled into CHANGELOG.md at release, so open PRs stop conflicting on the changelog"
stage: done
stage_status: archived
iteration: 1
native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
criteria: [LIFE-16]
observations:
  - 2026-11-15-cargo-test-test-changelog-13-passed-including-the-round-trip
  - 2026-11-15-on-d081c91-cargo-test-test-changelog-18-passed-after-grok-s-8
closed: 2026-11-15
outcome: done
---
# Describe the current objective before beginning work.

## Goal

Describe the current objective before beginning work.

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290": "yes" (Claude Code chat (hc t28), 2026-11-15, reply to the #290 scope summary); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290 as 0xLeif
- 0xLeif decided "Review the implementation of each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290": "yes i approve" (Claude Code chat (hc t28), 2026-11-15, reply to the #290 implementation review (PR #296)); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change review each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290 as 0xLeif with verdict pass

## What was verified

- cargo test --test changelog: 13 passed, including the round-trip that rebuilds cf2ff72's CHANGELOG.md byte for byte and the refusal of a repeated version heading; fledge run fmt and lint clean — bb85aa0975bd ^2026-11-15-cargo-test-test-changelog-13-passed-including-the-round-trip
- On d081c91, cargo test --test changelog 18 passed after Grok's 8 findings were fixed; fmt and lint clean; repro of same-day #99/#100 now assembles newest first — 26af66458d37 ^2026-11-15-on-d081c91-cargo-test-test-changelog-18-passed-after-grok-s-8

## Bugs

None.

## Open threads

## Timeline

- 2026-11-15T07:15:23Z · start · intent → intent · fix-290-changelog-fragments ^t-04e003
  - revision: 49b1a252ae47a9e7a5fd8fe257c993d29351a90a
  - by: claude-code (hc t28)
- 2026-11-15T07:15:23Z · advance · intent → design ·  ^t-b0fffd
  - revision: 49b1a252ae47a9e7a5fd8fe257c993d29351a90a
  - by: claude-code (hc t28)
- 2026-11-15T07:15:23Z · skip · design → plan · Nothing visual changes: a changelog file layout and a release task ^t-b5d1d0
  - revision: 49b1a252ae47a9e7a5fd8fe257c993d29351a90a
  - by: claude-code (hc t28)
- 2026-11-15T07:17:15Z · gate-opened · plan → plan · Approve the scope of each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290 ^t-64971e
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
  - holds: plan
  - drives: approve
  - by: claude-code (hc t28)
- 2026-11-15T15:12:10Z · direct-native-write · plan → plan · each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-79c660
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
  - direct_write:
    - change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: claude-code (hc t28)
- 2026-11-15T15:12:10Z · direct-native-write · plan → plan · each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-65acdd
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
  - direct_write:
    - change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: claude-code (hc t28)
- 2026-11-15T15:12:10Z · gate-closed · plan → plan · Approve the scope of each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290 ^t-61e9e6
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes
    - where: Claude Code chat (hc t28), 2026-11-15, reply to the #290 scope summary
    - recorded_by: claude-code (hc t28)
    - summary_digest: e315a14a366f7b8627cd400f8dad6499184ea7e19d628812606d3b530a5e346b
    - list: 05e9d46ea2bd
    - native: specsync change approve each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
    - actor: 0xLeif
  - by: claude-code (hc t28)
- 2026-11-15T15:12:20Z · advance · plan → build · native:approved ^t-7c08dd
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - gate: native:approved
  - revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
  - by: claude-code (hc t28)
- 2026-11-15T15:12:33Z · advance · build → test · native:approved ^t-199523
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - gate: native:approved
  - revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
  - by: claude-code (hc t28)
- 2026-11-15T15:12:42Z · advance · test → accept · native:verifying ^t-7e634d
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - gate: native:verifying
  - revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
  - by: claude-code (hc t28)
- 2026-11-15T16:28:42Z · gate-opened · accept → accept · Review the implementation of each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290 ^t-71887d
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - revision: 957afb4d8b1833f12c868e3535085455f6540130
  - holds: accept
  - drives: review
  - by: claude-code (hc t28)
- 2026-11-15T16:41:21Z · gate-closed · accept → accept · Review the implementation of each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290 ^t-db8e3a
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - revision: 713621c4d1c4963a17b9bcf79eb379fe9d05d790
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes i approve
    - where: Claude Code chat (hc t28), 2026-11-15, reply to the #290 implementation review (PR #296)
    - recorded_by: claude-code (hc t28)
    - native: specsync change review each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
    - actor: 0xLeif
  - by: claude-code (hc t28)
- 2026-11-15T16:41:22Z · advance · accept → done · native:archived ^t-cea6ff
  - native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
  - gate: native:archived
  - revision: 713621c4d1c4963a17b9bcf79eb379fe9d05d790
  - by: claude-code (hc t28)

Closed 2026-11-15: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-15-cargo-test-test-changelog-13-passed-including-the-round-trip
    claim: "cargo test --test changelog: 13 passed, including the round-trip that rebuilds cf2ff72's CHANGELOG.md byte for byte and the refusal of a repeated version heading; fledge run fmt and lint clean"
    result: passed
    revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
    environment: local-macos-arm64
    content: c68b423c4f71081b2b6285e61ebccbe42cfc93161157a41c05cb8f1c14cc8f9d
    fingerprint: sha256:bb85aa0975bd3eb377c4c0b3c5ad2840147b82d3af68c7e6bab5f214cb8d7458
    observed: 2026-11-15T15:12:33Z
    actor: claude-code (hc t28)
  - id: 2026-11-15-on-d081c91-cargo-test-test-changelog-18-passed-after-grok-s-8
    claim: "On d081c91, cargo test --test changelog 18 passed after Grok's 8 findings were fixed; fmt and lint clean; repro of same-day #99/#100 now assembles newest first"
    result: passed
    revision: d081c910526761fb393fceb3633daa73ce3368e5
    environment: local-macos-arm64
    content: 18a9c39b9c3cc35e79f7c22e431d783d1766b839726faea6e4b278174ad72043
    fingerprint: sha256:26af66458d3770822f183157d8549f7117efe210e87510c3d9492ff8183c21df
    observed: 2026-11-15T15:55:59Z
    actor: claude-code (hc t28)
```
