---
kind: session
id: 2026-11-15-fix-290-changelog-fragments
aliases: [fix-290-changelog-fragments]
objective: "Fix #290: each change adds a changelog fragment, assembled into CHANGELOG.md at release, so open PRs stop conflicting on the changelog"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
pr: ""
criteria: [LIFE-16]
requirements: []
cases: []
design: []
files: []
observations: [2026-11-15-cargo-test-test-changelog-13-passed-including-the-round-trip]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue accept
---
# Describe the current objective before beginning work.

## Goal

Describe the current objective before beginning work.

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290": "yes" (Claude Code chat (hc t28), 2026-11-15, reply to the #290 scope summary); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290 as 0xLeif

## What was verified

- [cargo test --test changelog: 13 passed, including the round-trip that rebuilds cf2ff72's CHANGELOG.md byte for byte and the refusal of a repeated version heading; fledge run fmt and lint clean](evidence/2026-11-15-cargo-test-test-changelog-13-passed-including-the-round-trip.md)

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
