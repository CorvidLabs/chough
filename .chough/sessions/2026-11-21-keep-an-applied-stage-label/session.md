---
kind: session
id: 2026-11-21-keep-an-applied-stage-label
aliases: [keep-an-applied-stage-label]
objective: A pull request that already has its stage label keeps that label.
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
pr: "https://github.com/CorvidLabs/chough/pull/443"
criteria: [LIFE-31]
requirements: [REQ-cli-070]
cases: [TC-cli-keep-applied-stage-label]
design: []
files: []
observations: [2026-11-21-tc-cli-keep-applied-stage-label]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of a-pull-request-that-already-has-its-stage-label-keeps-that-label
    holds: accept
    drives: review
    opened: 2026-11-21T18:01:55Z
run_consents: []
next: Open the implementation review and push. Do not merge.
title: Keep an applied stage label
summary:
  - A pull request that already has its stage label keeps that label. A different stage label is still removed, and a missing one is still added.
  - "Verified on this machine: the keep-label case passed, cargo test --test github_labels passed, and the verify lane completed 11 steps with SpecSync 6.4.0 and Fledge 1.8.0."
  - "Next: archive the open-session change on pull request 443. Merging was not requested."
---
# A pull request that already has its stage label keeps that label.

## Goal

A pull request that already has its stage label keeps that label.

## Where it stands

- A pull request that already has its stage label keeps it. A different stage label is still removed.

## Decisions and why

- corvid-agent requested that a pull request keep a stage label it already has. The user said "corvid-agent requesting changing". Scope is that script behavior, its test, and a refreshed pull request body. Merging is not authorized.

## What was verified

- [Ran cargo test --locked --test github_labels -- --exact the_script_keeps_a_label_it_already_applied for TC-cli-keep-applied-stage-label: passed (exit 0, tests run 1).](evidence/2026-11-21-tc-cli-keep-applied-stage-label.md)

## Open threads

## Timeline

- 2026-11-21T17:53:46Z · start · intent → intent · A pull request that already has its stage label keeps that label. ^t-581668
  - revision: 727d3bfdeef47b5a5df912d542565f8ba321a377
  - by: unknown agent (hc t10j)
- 2026-11-21T17:55:27Z · advance · intent → design ·  ^t-77d103
  - native_change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
  - revision: 727d3bfdeef47b5a5df912d542565f8ba321a377
  - by: unknown agent (hc t10j)
- 2026-11-21T17:55:27Z · skip · design → plan · Nothing visual changes. The script leaves a label that is already correct. ^t-7e963d
  - native_change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
  - revision: 727d3bfdeef47b5a5df912d542565f8ba321a377
  - by: unknown agent (hc t10j)
- 2026-11-21T17:55:28Z · direct-native-write · plan → plan · a-pull-request-that-already-has-its-stage-label-keeps-that-label's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-8d59ff
  - native_change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
  - revision: 727d3bfdeef47b5a5df912d542565f8ba321a377
  - direct_write:
    - change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: unknown agent (hc t10j)
- 2026-11-21T17:55:28Z · advance · plan → build · native:approved ^t-cbcc92
  - native_change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
  - gate: native:approved
  - revision: 727d3bfdeef47b5a5df912d542565f8ba321a377
  - by: unknown agent (hc t10j)
- 2026-11-21T18:01:54Z · advance · build → test · native:verifying ^t-dbbead
  - native_change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
  - gate: native:verifying
  - revision: dae4a30df4660b2dcec61bd44d0d8519fda848b7
  - by: unknown agent (hc t10j)
- 2026-11-21T18:01:55Z · advance · test → accept · native:verifying ^t-d9744e
  - native_change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
  - gate: native:verifying
  - revision: dae4a30df4660b2dcec61bd44d0d8519fda848b7
  - by: unknown agent (hc t10j)
- 2026-11-21T18:01:55Z · gate-opened · accept → accept · Implementation review of a-pull-request-that-already-has-its-stage-label-keeps-that-label ^t-4c562c
  - native_change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
  - revision: dae4a30df4660b2dcec61bd44d0d8519fda848b7
  - holds: accept
  - drives: review
  - by: unknown agent (hc t10j)
