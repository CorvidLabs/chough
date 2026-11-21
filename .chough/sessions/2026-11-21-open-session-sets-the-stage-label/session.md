---
kind: session
id: 2026-11-21-open-session-sets-the-stage-label
aliases: [open-session-sets-the-stage-label]
objective: An open session sets the stage label when archived sessions are also on the pull request.
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
pr: "443"
criteria: [LIFE-32]
requirements: [REQ-cli-070]
cases: [TC-cli-open-session-sets-the-stage-label]
design: []
files: []
observations:
  - 2026-11-21-tc-cli-open-session-sets-the-stage-label
  - 2026-11-21-tc-cli-open-session-sets-the-stage-label-5cb5f5
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
    holds: accept
    drives: review
    opened: 2026-11-21T18:15:19Z
run_consents: []
next: Open the implementation review and push. Do not merge.
title: An open session sets the stage label
summary:
  - When a pull request archives a session and starts a new one, the stage label shows the new open session. An archived story sets the label only when no open session remains.
  - "Verified on this machine: the new test failed first on archived done plus open intent, then TC-cli-open-session-sets-the-stage-label passed. cargo test --test github_labels passed 7, and the verify lane completed 11 steps."
  - "Next: push both archives on pull request 443 and watch Archive. Merging was not requested."
---
# An open session sets the stage label when archived sessions are also on the pull request.

## Goal

An open session sets the stage label when archived sessions are also on the pull request.

## Where it stands

- An open session sets the stage label. Before the fix the test reported multiple distinct stages done intent. After the fix TC-cli-open-session-sets-the-stage-label passed on local-macos-arm64.

## Decisions and why

- The user said "we need to make sure this change works for multiple sessions.. when they are archived.. they start a new change it should show the new change stages". Scope is that an open session sets the stage label and an archived story is used only when no open session remains. Merging is not authorized. The keep-label review stays open.

## What was verified

- [Ran cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one for TC-cli-open-session-sets-the-stage-label: passed (exit 0, tests run 1).](evidence/2026-11-21-tc-cli-open-session-sets-the-stage-label.md)
- [Ran cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one for TC-cli-open-session-sets-the-stage-label: passed (exit 0, tests run 1).](evidence/2026-11-21-tc-cli-open-session-sets-the-stage-label-5cb5f5.md)

## Open threads

## Timeline

- 2026-11-21T18:05:42Z · start · intent → intent · An open session sets the stage label when archived sessions are also on the pull request. ^t-350787
  - revision: 6bc60913b028761b20832a6906dc8e31d17e70df
  - by: unknown agent (hc t10j)
- 2026-11-21T18:12:25Z · advance · intent → design ·  ^t-d97245
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - revision: 6bc60913b028761b20832a6906dc8e31d17e70df
  - by: unknown agent (hc t10j)
- 2026-11-21T18:12:27Z · skip · design → plan · The label is a GitHub label name. Nothing visual in Chough changes. ^t-df62cb
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - revision: 6bc60913b028761b20832a6906dc8e31d17e70df
  - by: unknown agent (hc t10j)
- 2026-11-21T18:12:56Z · direct-native-write · plan → plan · an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-bbacd2
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - revision: 6bc60913b028761b20832a6906dc8e31d17e70df
  - direct_write:
    - change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: unknown agent (hc t10j)
- 2026-11-21T18:12:56Z · advance · plan → build · native:approved ^t-de9fff
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - gate: native:approved
  - revision: 6bc60913b028761b20832a6906dc8e31d17e70df
  - by: unknown agent (hc t10j)
- 2026-11-21T18:15:18Z · advance · build → test · native:verifying ^t-712048
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - gate: native:verifying
  - revision: 6bc60913b028761b20832a6906dc8e31d17e70df
  - by: unknown agent (hc t10j)
- 2026-11-21T18:15:19Z · advance · test → accept · native:verifying ^t-436776
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - gate: native:verifying
  - revision: 6bc60913b028761b20832a6906dc8e31d17e70df
  - by: unknown agent (hc t10j)
- 2026-11-21T18:15:19Z · gate-opened · accept → accept · Implementation review of an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request ^t-3000a3
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - revision: 6bc60913b028761b20832a6906dc8e31d17e70df
  - holds: accept
  - drives: review
  - by: unknown agent (hc t10j)
