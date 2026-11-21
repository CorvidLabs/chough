---
kind: session
id: 2026-11-21-fork-symlink-label-refresh
aliases: [fork-symlink-label-refresh]
objective: A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.
intent_scope: ""
modules: []
stage: test
stage_status: active
iteration: 1
callers: []
native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
pr: "https://github.com/CorvidLabs/chough/pull/443"
criteria: [LIFE-31]
requirements: [REQ-cli-070]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Commit the follow-up, record verification, and push the pull request branch. Do not merge.
---
# A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.

## Goal

A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated.

## Where it stands

- Fork pull requests skip the label job, symlinked session files are not read, and a matching stage label is not recreated.

## Decisions and why

- The review of pull request 443 authorizes three fixes only: skip the label job for a fork pull request, do not read a symlinked session file, and create a stage label only when it is missing or its color or description differs. Reopening the archived change is out of scope. Merging is not authorized.

## What was verified

## Open threads

## Timeline

- 2026-11-21T17:09:54Z · start · intent → intent · A fork pull request does not fail the label job, a symlinked session is not read, and a matching stage label is not recreated. ^t-83738b
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:17:22Z · advance · intent → design ·  ^t-c7fe91
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:17:22Z · skip · design → plan · This is a workflow and script safety fix. Nothing visual changes. ^t-331716
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:17:45Z · direct-native-write · plan → plan · a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-b60064
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - direct_write:
    - change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: unknown agent (hc t10j)
- 2026-11-21T17:17:45Z · advance · plan → build · native:approved ^t-cf128b
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - gate: native:approved
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
- 2026-11-21T17:30:01Z · advance · build → test · native:approved ^t-134ef0
  - native_change: a-fork-pull-request-does-not-fail-the-label-job-a-symlinked-session-is-not-read-and-a-matching-stage-label-is-not
  - gate: native:approved
  - revision: 97df042ea89d865cc08b36d1f96c3901103e366d
  - by: unknown agent (hc t10j)
