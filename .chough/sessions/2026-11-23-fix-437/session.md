---
kind: session
id: 2026-11-23-fix-437
aliases: [fix-437]
objective: "Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text"
intent_scope: ""
modules: []
stage: intent
stage_status: active
iteration: 1
callers: []
native_change: ""
pr: ""
criteria: [FLOW-2]
requirements: []
cases: []
design: []
files: []
observations: [2026-11-23-tc-records-inbox-add-without-hard-links-8d250e]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Read scoped intent and native status, then choose the next action within the authorized scope.
---
# Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text

## Goal

Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text

## Where it stands

## Decisions and why

## What was verified

- [Ran cargo test --locked --test no_hard_links -- --ignored --exact new_records_are_written_on_a_volume_without_hard_links for TC-records-inbox-add-without-hard-links: passed (exit 0).](evidence/2026-11-23-tc-records-inbox-add-without-hard-links-8d250e.md)

## Open threads

## Timeline

- 2026-11-23T14:44:33Z · start · intent → intent · fix-437 ^t-87f09a
  - revision: 35828a1df3d308677a38b138e7aebe0e7b6770ef
  - by: unknown agent (hc t14z)
