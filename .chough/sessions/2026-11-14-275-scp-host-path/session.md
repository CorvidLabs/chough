---
kind: session
id: 2026-11-14-275-scp-host-path
aliases: [275-scp-host-path]
objective: The leak check reads an scp-style host path, host:/path, like the path it carries (#275)
intent_scope: LIFE
modules: [records]
stage: test
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
observations: [2026-11-14-tc-records-an-scp-style-host-path-is-checked-like-the-path-it]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue test
---
# The leak check reads an scp-style host path, host:/path, like the path it carries (#275)

## Goal

The leak check reads an scp-style host path, host:/path, like the path it carries (#275)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275": "0xLeif ran: specsync change approve the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275 --actor 0xLeif" (0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native approval is already recorded); recorded by claude-code; a relayed claim, not authenticated; ran specsync change approve the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275 as 0xLeif

## What was verified

- [Ran cargo test --locked --bin chough -- --exact leaks::tests::an_scp_style_remote_path_is_checked_like_the_path_it_carries for TC-records-an-scp-style-host-path-is-checked-like-the-path-it-carries: passed (exit 0).](evidence/2026-11-14-tc-records-an-scp-style-host-path-is-checked-like-the-path-it.md)

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
- 2026-11-14T17:36:30Z · direct-native-write · plan → plan · the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-c6ed40
  - native_change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
  - revision: 2b617926f0f0d16dab31e83623bba0ad526a4773
  - direct_write:
    - change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: claude-code
- 2026-11-14T17:36:30Z · gate-closed · plan → plan · Scope approval of the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275 ^t-fb27f6
  - native_change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
  - revision: 2b617926f0f0d16dab31e83623bba0ad526a4773
  - relayed_by:
    - decided_by: 0xLeif
    - words: 0xLeif ran: specsync change approve the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275 --actor 0xLeif
    - where: 0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native approval is already recorded
    - recorded_by: claude-code
    - native: specsync change approve the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
    - actor: 0xLeif
  - by: claude-code
- 2026-11-14T17:36:31Z · advance · plan → build · Scope approved by 0xLeif; the fix is in 9b729e9. ^t-9660da
  - native_change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
  - gate: native:approved
  - revision: 2b617926f0f0d16dab31e83623bba0ad526a4773
  - by: claude-code
- 2026-11-14T17:36:31Z · advance · build → test · Build done in 9b729e9; fmt, test and lint passed. ^t-595b85
  - native_change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
  - gate: native:approved
  - revision: 2b617926f0f0d16dab31e83623bba0ad526a4773
  - by: claude-code
