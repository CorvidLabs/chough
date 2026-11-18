---
kind: session
id: 2026-11-18-docs-0-6-1-release
aliases: [docs-0-6-1-release]
objective: The roadmap's delivered table has the 0.6.1 release row, and the README no longer says 0.6.0 is not released yet
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: docs-0-6-1-release
pr: ""
criteria: [LIFE-25]
requirements: []
cases: []
design: []
files: [README.md, docs/roadmap-1.0.md]
observations: [2026-11-18-after-cargo-publish-locked-from-tag-v0-6-1-cef86e2-ended-with]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of docs-0-6-1-release
    holds: accept
    drives: review
    opened: 2026-11-18T18:00:18Z
run_consents: []
next: Wait for Leif's implementation review of docs-0-6-1-release
---
# The roadmap's delivered table has the 0.6.1 release row, and the README no longer says 0.6.0 is not released yet

## Goal

The roadmap's delivered table has the 0.6.1 release row, and the README no longer says 0.6.0 is not released yet

## Where it stands

- The 0.6.1 release row and the README sentence are written; the docs tests pass

## Decisions and why

- 0xLeif decided "Scope approval of docs-0-6-1-release": "Approve" (Relayed by Leif's planning session (hc t4c) on 2026-11-18, which asked him: 'Chough docs PR #327: add the 0.6.1 row to the roadmap (released, crates.io, the clean install check passed) and fix README line 258 (the stale documents 0.6.0, which is not released yet). Docs only. Approve the scope?'); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve docs-0-6-1-release as 0xLeif

## What was verified

- [After cargo publish --locked from tag v0.6.1 (cef86e2) ended with 'Published chough v0.6.1 at registry crates-io', cargo install chough --version 0.6.1 --locked into an empty root with cargo 1.100.0 on macOS arm64 exited 0; chough --version printed 'chough 0.6.1'; chough init, next and status exited 0 in a new Git repository.](evidence/2026-11-18-after-cargo-publish-locked-from-tag-v0-6-1-cef86e2-ended-with.md)

## Open threads

## Timeline

- 2026-11-18T17:50:20Z · start · intent → intent · The roadmap's delivered table has the 0.6.1 release row, and the README no longer says 0.6.0 is not released yet ^t-0e0c3c
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T17:50:20Z · advance · intent → design ·  ^t-eece86
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T17:50:20Z · skip · design → plan · Docs change; nothing visual or interactive changes ^t-2c26cf
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T17:50:52Z · gate-opened · plan → plan · Scope approval of docs-0-6-1-release ^t-1f9051
  - native_change: docs-0-6-1-release
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T17:56:31Z · gate-closed · plan → plan · Scope approval of docs-0-6-1-release ^t-8e5079
  - native_change: docs-0-6-1-release
  - revision: 28b55ce1555d7bf9dc6a336143c0bf3e25534640
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve
    - where: Relayed by Leif's planning session (hc t4c) on 2026-11-18, which asked him: 'Chough docs PR #327: add the 0.6.1 row to the roadmap (released, crates.io, the clean install check passed) and fix README line 258 (the stale documents 0.6.0, which is not released yet). Docs only. Approve the scope?'
    - recorded_by: fable-t4b
    - native: specsync change approve docs-0-6-1-release
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T17:56:32Z · advance · plan → build · native:approved ^t-22f4a6
  - native_change: docs-0-6-1-release
  - gate: native:approved
  - revision: 28b55ce1555d7bf9dc6a336143c0bf3e25534640
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T17:56:32Z · advance · build → test · native:approved ^t-6d2aa0
  - native_change: docs-0-6-1-release
  - gate: native:approved
  - revision: 28b55ce1555d7bf9dc6a336143c0bf3e25534640
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T18:00:18Z · advance · test → accept · native:verifying ^t-665f6d
  - native_change: docs-0-6-1-release
  - gate: native:verifying
  - revision: 73d3c62bfbf14a4f5da6e299ed3c15469e4693b3
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T18:00:18Z · gate-opened · accept → accept · Implementation review of docs-0-6-1-release ^t-e7b628
  - native_change: docs-0-6-1-release
  - revision: 73d3c62bfbf14a4f5da6e299ed3c15469e4693b3
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
