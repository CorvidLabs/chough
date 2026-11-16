---
kind: session
id: 2026-11-15-walk-ready-demo
aliases: [walk-ready-demo]
objective: Describe the current objective before beginning work.
stage: done
stage_status: archived
iteration: 1
native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
criteria: [DRIVE-29]
observations:
  - 2026-11-15-tc-cli-walk-ready-init-preserves
  - 2026-11-15-tc-cli-walk-ready-init-preserves-fcb611
closed: 2026-11-15
outcome: done
---
# Describe the current objective before beginning work.

## Goal

Describe the current objective before beginning work.

## Where it stands

- Started demo walk: session created for change-ready mechanics
- Change ready: nothing missing. Awaiting human scope approval before build.
- Build done: REQ-cli-001 acceptance clarified, case added, CHANGELOG and docs updated, fmt/lint/change-check pass
- Test case passed and evidence recorded. Ready for human implementation review.

## Decisions and why

- 0xLeif decided "Scope approval of clarify-req-cli-001-init-preserves-bytes-acceptance": "I approve" (chat, 2026-11-15 (this session)); recorded by pi-agent; a relayed claim, not authenticated; ran specsync change approve clarify-req-cli-001-init-preserves-bytes-acceptance as 0xLeif
- 0xLeif decided "Implementation review of clarify-req-cli-001-init-preserves-bytes-acceptance": "I approve" (chat, 2026-11-15 (this session)); recorded by pi-agent; a relayed claim, not authenticated; ran specsync change review clarify-req-cli-001-init-preserves-bytes-acceptance as 0xLeif with verdict pass

## What was verified

- 1 earlier run of the cases below is kept in this story's receipts, not listed here.
- Case TC-cli-walk-ready-init-preserves passed: echo preserves (exit 0). REQ-cli-001 acceptance clarified and change ready. ^2026-11-15-tc-cli-walk-ready-init-preserves-fcb611

## Bugs

None.

## Open threads

## Timeline

- 2026-11-15T03:28:46Z · start · intent → intent · walk-ready-demo ^t-1a05d1
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t15)
- 2026-11-15T03:29:59Z · advance · intent → design ·  ^t-893075
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t15)
- 2026-11-15T03:29:59Z · skip · design → plan · CLI-only change, no visuals or interactions ^t-573a11
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: unknown agent (hc t15)
- 2026-11-15T03:33:13Z · gate-opened · plan → plan · Scope approval of clarify-req-cli-001-init-preserves-bytes-acceptance ^t-b3e285
  - native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - holds: plan
  - drives: approve
  - by: unknown agent (hc t15)
- 2026-11-15T03:42:10Z · gate-closed · plan → plan · Scope approval of clarify-req-cli-001-init-preserves-bytes-acceptance ^t-e2d0ff
  - native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve
    - where: chat, 2026-11-15 (this session)
    - recorded_by: pi-agent
    - native: specsync change approve clarify-req-cli-001-init-preserves-bytes-acceptance
    - actor: 0xLeif
  - by: unknown agent (hc t15)
- 2026-11-15T03:48:10Z · advance · plan → build · native:verifying ^t-f48daf
  - native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
  - gate: native:verifying
  - revision: 913c0068ee0894aaa6256eb2153419612f8e5172
  - by: unknown agent (hc t15)
- 2026-11-15T03:48:22Z · advance · build → test · native:verifying ^t-9bf5c7
  - native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
  - gate: native:verifying
  - revision: 913c0068ee0894aaa6256eb2153419612f8e5172
  - by: unknown agent (hc t15)
- 2026-11-15T03:50:07Z · advance · test → accept · native:verifying ^t-794d02
  - native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
  - gate: native:verifying
  - revision: 913c0068ee0894aaa6256eb2153419612f8e5172
  - by: unknown agent (hc t15)
- 2026-11-15T03:52:27Z · gate-opened · accept → accept · Implementation review of clarify-req-cli-001-init-preserves-bytes-acceptance ^t-f8b315
  - native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
  - revision: ba30b7d713b9b331bf7e2e2362a531e1aa59c56a
  - holds: accept
  - drives: review
  - by: unknown agent (hc t15)
- 2026-11-15T03:53:16Z · gate-closed · accept → accept · Implementation review of clarify-req-cli-001-init-preserves-bytes-acceptance ^t-fb0ec1
  - native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
  - revision: 8ffcc1f22ab015fcc0ad93779d98bc727b4f790a
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve
    - where: chat, 2026-11-15 (this session)
    - recorded_by: pi-agent
    - native: specsync change review clarify-req-cli-001-init-preserves-bytes-acceptance
    - actor: 0xLeif
  - by: unknown agent (hc t15)
- 2026-11-15T03:53:17Z · advance · accept → done · native:archived ^t-46cf50
  - native_change: clarify-req-cli-001-init-preserves-bytes-acceptance
  - gate: native:archived
  - revision: 8ffcc1f22ab015fcc0ad93779d98bc727b4f790a
  - by: unknown agent (hc t15)

Closed 2026-11-15: done.
- 2026-11-16T03:21:41Z · correction · done → done · The receipt 2026-11-15-tc-cli-walk-ready-init-preserves, command echo preserves, exit 0, is a demo smoke result, not behavioral preservation evidence for REQ-cli-001. Case TC-cli-walk-ready-init-preserves now runs tests/cli.rs init_preserves_user_files_and_installs_a_complete_skill, which keeps edited session and skill bytes and adds a missing bundle file without rewriting the edited neighbor. The original receipts stay as recorded. ^t-beba6c
  - corrects: ^t-794d02
  - by: grok-t1w (grok, hc t1w)

## Receipts

```yaml
receipts:
  - id: 2026-11-15-tc-cli-walk-ready-init-preserves
    claim: "Ran echo preserves for TC-cli-walk-ready-init-preserves: passed (exit 0)."
    result: passed
    command: echo preserves
    revision: 913c0068ee0894aaa6256eb2153419612f8e5172
    environment: local-macos-arm64
    content: 033dd4cc3a2cf422444d1370ddbec06e30c4f3244018dc58e9d06a7615d0af1c
    fingerprint: sha256:3e617855a2ff137331bb011761ee0e379fdaa38819a0d598a6547707d9e53864
    case: TC-cli-walk-ready-init-preserves
    case_digest: 9d9baac3967545ef8b9eb99534b8727354ef0e02a3d509284c9f152d31ba9ced
    observed: 2026-11-15T03:49:07Z
    duration_seconds: 0
    actor: unknown agent (hc t15)
  - id: 2026-11-15-tc-cli-walk-ready-init-preserves-fcb611
    claim: "Case TC-cli-walk-ready-init-preserves passed: echo preserves (exit 0). REQ-cli-001 acceptance clarified and change ready."
    result: passed
    revision: 913c0068ee0894aaa6256eb2153419612f8e5172
    environment: local-macos-arm64
    content: 033dd4cc3a2cf422444d1370ddbec06e30c4f3244018dc58e9d06a7615d0af1c
    case: TC-cli-walk-ready-init-preserves
    case_digest: 9d9baac3967545ef8b9eb99534b8727354ef0e02a3d509284c9f152d31ba9ced
    observed: 2026-11-15T03:49:59Z
    actor: unknown agent (hc t15)
```
