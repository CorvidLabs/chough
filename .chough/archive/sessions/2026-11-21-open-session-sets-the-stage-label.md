---
kind: session
id: 2026-11-21-open-session-sets-the-stage-label
aliases: [open-session-sets-the-stage-label]
objective: An open session sets the stage label when archived sessions are also on the pull request.
stage: done
stage_status: archived
iteration: 1
native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
pr: "443"
criteria: [LIFE-32]
requirements: [REQ-cli-070]
cases: [TC-cli-open-session-sets-the-stage-label]
observations:
  - 2026-11-21-tc-cli-open-session-sets-the-stage-label
  - 2026-11-21-tc-cli-open-session-sets-the-stage-label-5cb5f5
title: An open session sets the stage label
summary:
  - When a pull request archives a session and starts a new one, the stage label shows the new open session. An archived story sets the label only when no open session remains.
  - "Verified on this machine: the new test failed first on archived done plus open intent, then TC-cli-open-session-sets-the-stage-label passed. cargo test --test github_labels passed 7, and the verify lane completed 11 steps."
  - "Next: push both archives on pull request 443 and watch Archive. Merging was not requested."
closed: 2026-11-21
outcome: done
---
# An open session sets the stage label when archived sessions are also on the pull request.

## Goal

An open session sets the stage label when archived sessions are also on the pull request.

## Where it stands

- An open session sets the stage label. Before the fix the test reported multiple distinct stages done intent. After the fix TC-cli-open-session-sets-the-stage-label passed on local-macos-arm64.

## Decisions and why

- The user said "we need to make sure this change works for multiple sessions.. when they are archived.. they start a new change it should show the new change stages". Scope is that an open session sets the stage label and an archived story is used only when no open session remains. Merging is not authorized. The keep-label review stays open.
- 0xLeif decided "Implementation review of an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request": "Yes, record the review and archive both." (Grok chat on 2026-11-21, relayed by grok-build); recorded by grok-build-t10j; a relayed claim, not authenticated; ran specsync change review an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request as 0xLeif with verdict pass

## What was verified

- 1 earlier run of the cases below is kept in this story's receipts, not listed here.
- Ran cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one for TC-cli-open-session-sets-the-stage-label: passed (exit 0, tests run 1). - 95310f0bfe8a; limits: inherited environment variables were not recorded ^2026-11-21-tc-cli-open-session-sets-the-stage-label-5cb5f5

## Bugs

None.

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
- 2026-11-21T18:39:18Z · gate-closed · accept → accept · Implementation review of an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request ^t-08cad0
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - revision: 55244f9c89746e17b23bdfdec927f67562720751
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, record the review and archive both.
    - where: Grok chat on 2026-11-21, relayed by grok-build
    - recorded_by: grok-build-t10j
    - native: specsync change review an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
    - actor: 0xLeif
  - by: unknown agent (hc t10j)
- 2026-11-21T18:39:20Z · advance · accept → done · native:archived ^t-fd883d
  - native_change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
  - gate: native:archived
  - revision: 55244f9c89746e17b23bdfdec927f67562720751
  - by: unknown agent (hc t10j)

Closed 2026-11-21: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-21-tc-cli-open-session-sets-the-stage-label
    claim: "Ran cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one for TC-cli-open-session-sets-the-stage-label: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one
    revision: 6bc60913b028761b20832a6906dc8e31d17e70df
    environment: local-macos-arm64
    content: d30d4d763d1167a47980a64dd0a82060d63b7a49fa34428e37150132597e159f
    fingerprint: sha256:1670c9bfaf7c9dfa18ecd23756028b0edcb743c39684203abe2663f09bc0c67b
    case: TC-cli-open-session-sets-the-stage-label
    case_digest: ff3a404cb5b38745504b41f403911d9aa4f5e422cd5795552a4b4148841f11fb
    criteria: LIFE-32, LIFE-31
    requirements: REQ-cli-070
    limitations: inherited environment variables were not recorded
    observed: 2026-11-21T18:14:10Z
    duration_seconds: 0
    actor: unknown agent (hc t10j)
  - id: 2026-11-21-tc-cli-open-session-sets-the-stage-label-5cb5f5
    claim: "Ran cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one for TC-cli-open-session-sets-the-stage-label: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test github_labels -- --exact the_script_follows_an_open_session_ahead_of_an_archived_one
    revision: 6bc60913b028761b20832a6906dc8e31d17e70df
    environment: local-macos-arm64
    content: 540e9aa15ae2641e45f5b3e026b5135a4c7132e33a6c799de14ba1a5ee8d8f21
    fingerprint: sha256:95310f0bfe8aaf867e3933b4951adb106e216b8eb82e3b60ed66924f88a7353b
    case: TC-cli-open-session-sets-the-stage-label
    case_digest: ff3a404cb5b38745504b41f403911d9aa4f5e422cd5795552a4b4148841f11fb
    criteria: LIFE-32, LIFE-31
    requirements: REQ-cli-070
    limitations: inherited environment variables were not recorded
    observed: 2026-11-21T18:15:02Z
    duration_seconds: 0
    actor: unknown agent (hc t10j)
```
