---
kind: session
id: 2026-11-23-fix-435
aliases: [fix-435]
objective: "Fix chough#435: inbox add writes its entry without a hard link, whole or not at all, like records::write_new_bytes"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-435
pr: ""
criteria: [FLOW-8]
requirements: []
cases: [TC-records-inbox-add-without-hard-links]
design: []
files: []
observations: [2026-11-23-tc-records-inbox-add-without-hard-links]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-435
    holds: accept
    drives: review
    opened: 2026-11-23T14:42:18Z
run_consents: []
next: Continue accept
---
# Fix chough#435: inbox add writes its entry without a hard link, whole or not at all, like records::write_new_bytes

## Goal

Fix chough#435: inbox add writes its entry without a hard link, whole or not at all, like records::write_new_bytes

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of fix-435": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-435 as 0xLeif

## What was verified

- [Ran cargo test --locked --test no_hard_links -- --ignored --exact new_records_are_written_on_a_volume_without_hard_links for TC-records-inbox-add-without-hard-links: passed (exit 0).](evidence/2026-11-23-tc-records-inbox-add-without-hard-links.md)

## Open threads

## Timeline

- 2026-11-23T14:20:29Z · start · intent → intent · fix-435 ^t-b6d8e9
  - revision: 4a46668220663eb00524cb6990a325b0713eb099
  - by: unknown agent (hc t14z)
- 2026-11-23T14:22:55Z · gate-opened · intent → intent · Approve the scope of fix-435 ^t-65d239
  - native_change: fix-435
  - revision: 4a46668220663eb00524cb6990a325b0713eb099
  - holds: intent
  - drives: approve
  - by: unknown agent (hc t14z)
- 2026-11-23T14:23:02Z · direct-native-write · intent → intent · fix-435's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-c7dfe8
  - native_change: fix-435
  - revision: 4a46668220663eb00524cb6990a325b0713eb099
  - direct_write:
    - change: fix-435
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: unknown agent (hc t14z)
- 2026-11-23T14:23:02Z · gate-closed · intent → intent · Approve the scope of fix-435 ^t-4bb00d
  - native_change: fix-435
  - revision: 4a46668220663eb00524cb6990a325b0713eb099
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve
    - where: pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch
    - recorded_by: pi-t14z
    - summary_digest: 67c1a5b7e0262297545cf16767b9c5913e98ac6d0537ca9a8f22eae0d6e1daef
    - list: 28c0511ccf55
    - native: specsync change approve fix-435
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-23T14:23:15Z · advance · intent → design ·  ^t-bc917f
  - native_change: fix-435
  - revision: 4a46668220663eb00524cb6990a325b0713eb099
  - by: unknown agent (hc t14z)
- 2026-11-23T14:23:15Z · advance · design → plan ·  ^t-14e7d3
  - native_change: fix-435
  - revision: 4a46668220663eb00524cb6990a325b0713eb099
  - by: unknown agent (hc t14z)
- 2026-11-23T14:42:13Z · advance · plan → build · native:approved ^t-16c1ef
  - native_change: fix-435
  - gate: native:approved
  - revision: 91a38bf5fe28054bb1d65cf15634816fe14c50bd
  - by: unknown agent (hc t14z)
- 2026-11-23T14:42:13Z · advance · build → test · native:approved ^t-cfc0e5
  - native_change: fix-435
  - gate: native:approved
  - revision: 91a38bf5fe28054bb1d65cf15634816fe14c50bd
  - by: unknown agent (hc t14z)
- 2026-11-23T14:42:14Z · advance · test → accept · native:approved ^t-cd0696
  - native_change: fix-435
  - gate: native:approved
  - revision: 91a38bf5fe28054bb1d65cf15634816fe14c50bd
  - by: unknown agent (hc t14z)
- 2026-11-23T14:42:18Z · gate-opened · accept → accept · Implementation review of fix-435 ^t-14d201
  - native_change: fix-435
  - revision: 91a38bf5fe28054bb1d65cf15634816fe14c50bd
  - holds: accept
  - drives: review
  - by: unknown agent (hc t14z)
