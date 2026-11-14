---
kind: session
id: 2026-11-14-275-scp-host-path
aliases: [275-scp-host-path]
objective: The leak check reads an scp-style host path, host:/path, like the path it carries (#275)
intent_scope: LIFE
modules: [records]
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
pr: ""
criteria: [LIFE-26]
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
    holds: plan
    drives: approve
    opened: 2026-11-14T17:16:07Z
run_consents: []
next: Read scoped intent and native status, then choose the next action within the authorized scope.
---
# The leak check reads an scp-style host path, host:/path, like the path it carries (#275)

## Goal

The leak check reads an scp-style host path, host:/path, like the path it carries (#275)

## Where it stands

## Decisions and why

## What was verified

## Open threads

## Timeline

- 2026-11-14T17:14:51Z · start · intent → intent · The leak check reads an scp-style host path, host:/path, like the path it carries (#275) ^t-ebba83
  - revision: 9b729e908adf30b6511531776f6976b1afd01395
  - by: claude-code
- 2026-11-14T17:14:51Z · advance · intent → design · Intent is LIFE-26 with REQ-records-006 (no leaks in committed records); issue #275 names the missing scp-style form. ^t-0e7dce
  - revision: 9b729e908adf30b6511531776f6976b1afd01395
  - by: claude-code
- 2026-11-14T17:14:51Z · skip · design → plan · Leak-check logic and its records only; no visual or interaction change. ^t-2ea5b4
  - revision: 9b729e908adf30b6511531776f6976b1afd01395
  - by: claude-code
- 2026-11-14T17:16:07Z · gate-opened · plan → plan · Scope approval of the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275 ^t-3b63d8
  - native_change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
  - revision: b0a62e7abdf886aac7c7d47409a20f6dd54e0c18
  - holds: plan
  - drives: approve
  - by: claude-code
